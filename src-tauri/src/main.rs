// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64::{engine::general_purpose, Engine as _};
use screenshots::Screen;
use std::io::Error;
use std::sync::Mutex;
use std::thread;
use tauri::api::process::{Command, CommandEvent};
use tauri::Manager;
use tauri::State;
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextW, ShowWindow};
use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

#[derive(Clone, Debug)]
struct Handle {
    hwnd: HWND,
    pid: Option<u32>,
}
struct Handles(Mutex<Vec<Handle>>);

#[tauri::command]
fn screen_shot() -> String {
    let ss = thread::spawn(move || {
        let screen = Screen::all().unwrap()[0];
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        let dir = general_purpose::STANDARD.encode(&buffer);
        return dir;
    });
    return ss.join().unwrap();
}

fn run_pssuspend(pid: u32, suspend: bool) -> Result<(), Error> {
    let args = if suspend {
        vec![pid.to_string()]
    } else {
        vec!["-r".to_string(), pid.to_string()]
    };

    println!("Running pssuspend with args: {:?}", args);

    let (mut rx, _child) = Command::new_sidecar("pssuspend")
        .expect("failed to create `pssuspend` binary command")
        .args(args)
        .spawn()
        .expect("Failed to spawn sidecar");

    while let Ok(event) = rx.try_recv() {
        match event {
            CommandEvent::Terminated(code) => {
                println!("pssuspend exited with code: {}", code.code.unwrap_or(-1));
                break;
            }
            _ => {}
        }
    }

    println!("pssuspend finished");

    Ok(())
}

#[tauri::command]
fn get_apps(handles: State<Handles>) -> (String, usize) {
    struct App {
        title: String,
        hwnd: HWND,
    }

    let whitelist_name = vec![["Defer", "Windows", "Task Manager"]];

    let whitelist_file = vec![["Explorer"]];

    let mut n_handles = handles.0.lock().unwrap();
    let mut curr_apps: Vec<App> = Vec::new();
    unsafe {
        EnumWindows(
            Some(enum_window),
            LPARAM(&mut curr_apps as *mut Vec<App> as _),
        )
        .ok()
        .unwrap();
    };
    let mut tot_apps = String::new();
    let mut index = 0;
    for i in &curr_apps {
        let mut name = [0u8; 512];
        let len = unsafe { GetWindowModuleFileNameA(i.hwnd, &mut name) };
        let name = String::from_utf8_lossy(&name[..len as usize]);
        let exe = name.split("\\").last().unwrap().to_string();

        println!("Cya: {} | {}", i.title, exe);

        if !whitelist_name
            .iter()
            .any(|x| i.title.to_lowercase().contains(&x[0].to_lowercase()))
            && !whitelist_file
                .iter()
                .any(|x| exe.to_lowercase().contains(&x[0].to_lowercase()))
        {
            let mut input = Handle {
                hwnd: i.hwnd,
                pid: None,
            };
            unsafe {
                let mut process_id = 0;
                let _thread_id = GetWindowThreadProcessId(i.hwnd, Some(&mut process_id));

                if process_id != 0 {
                    match run_pssuspend(process_id, true) {
                        Ok(_) => {
                            ShowWindowAsync(i.hwnd, SW_HIDE);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }

                    input.pid = Some(process_id);
                    n_handles.push(input);
                }
            };

            if index < 3 {
                if !i.title.contains("—") {
                    tot_apps = tot_apps + "," + &i.title;
                    index += 1;
                } else {
                    let split_title: Vec<&str> = i.title.split("—").collect();
                    tot_apps = tot_apps + "," + split_title[1];
                    index += 1;
                }
            }
        }
        // println!("{} | {} | {} | {}",i.active,i.title,i.dim.0,i.dim.1);
    }
    tot_apps = tot_apps.chars().skip(1).collect();
    let len = curr_apps.len() - 1;
    return (tot_apps, len);
}

#[tauri::command]
fn restore(handles: State<Handles>) {
    let n_handles = handles.0.lock().unwrap().clone().into_iter();
    for handle in n_handles {
        unsafe {
            println!("Restoring: {:?}", handle);
            // ShowWindow(handle.hwnd, SW_SHOW);
            match handle.pid {
                Some(pid) => match run_pssuspend(pid, false) {
                    Ok(_) => {
                        ShowWindowAsync(handle.hwnd, SW_SHOW);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                },
                _ => {
                    println!("No process to resume");
                }
            }
        }
    }
}

extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
    #[allow(dead_code)]
    struct App {
        title: String,
        hwnd: HWND,
    }
    unsafe {
        let curr_apps = &mut *(lparam.0 as *mut Vec<App>);
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        let mut info = WINDOWINFO {
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        GetWindowInfo(window, &mut info).unwrap();

        if !text.is_empty() && info.dwStyle.contains(WS_VISIBLE) {
            println!(
                "{} ({}, {}, {}, {})",
                text,
                info.rcWindow.left,
                info.rcWindow.top,
                info.rcWindow.right - info.rcWindow.left,
                info.rcWindow.bottom - info.rcWindow.top
            );
            if !(info.rcWindow.left == 0) || !(info.rcWindow.left == 0) {
                let c_app = App {
                    title: text,
                    hwnd: window,
                };
                curr_apps.push(c_app)
            }
        }

        true.into()
    }
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new().add_item(show).add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_always_on_top(true).expect("Oopsie");
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    let window = app.get_window("main").unwrap();
                    let js_code = r#"
                        (async () => {
                        await __TAURI_INVOKE__('restore');
                        window.close();
                        })();
                    "#;
                    window.eval(&js_code).expect("no way");
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .manage(Handles(Default::default()))
        .invoke_handler(tauri::generate_handler![get_apps, restore, screen_shot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
