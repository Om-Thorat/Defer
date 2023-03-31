// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextW,ShowWindow};
use tauri::State;
use std::sync::Mutex;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use screenshots::Screen;
use std::thread;
use std::env;
use base64::{Engine as _, engine::general_purpose};

struct Handles(Mutex<Vec<HWND>>);

#[tauri::command]
fn screen_shot()->String{
   let ss = thread::spawn(move || {
        let screen = Screen::all().unwrap()[0];
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        let dir = general_purpose::STANDARD.encode(&buffer);
        return dir
    });
    return ss.join().unwrap();
}

#[tauri::command]
fn get_apps(handles: State<Handles>) -> (String, usize){
    struct App {
        active: bool,
        title: String,
        dim: (i32,i32,u32,u32),
        hwnd: HWND
    }
    let mut n_handles = handles.0.lock().unwrap();
    let mut curr_apps:Vec<App> = Vec::new();
    unsafe { 
        EnumWindows(Some(enum_window), LPARAM(&mut curr_apps as *mut Vec<App> as _)).ok().unwrap();
    };
    let mut tot_apps = String::new();
    let mut index = 0; 
    for i in &curr_apps{
        if i.title != "Defer"{
            n_handles.push(i.hwnd);
            unsafe{
                ShowWindow(i.hwnd, SW_HIDE);
            }
            if index < 3{
                if !i.title.contains("—") {
                    tot_apps = tot_apps + "," + &i.title;
                    index += 1;
                } 
                else {
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
fn restore(handles: State<Handles>){
    let n_handles = handles.0.lock().unwrap().clone().into_iter();
    for i in n_handles{
        unsafe{
            ShowWindow(i, SW_SHOW);
        }
    }
}

extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
    #[allow(dead_code)]
    struct App {
        active: bool,
        title: String,
        dim: (i32,i32,u32,u32),
        hwnd: HWND
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
            println!("{} ({}, {}, {}, {})", text, info.rcWindow.left, info.rcWindow.top,info.rcWindow.right - info.rcWindow.left,info.rcWindow.bottom-info.rcWindow.top);
            if !(info.rcWindow.left == 0) || !(info.rcWindow.left == 0){
                let c_app = App{
                    active: true,
                    title: text,
                    dim: (info.rcWindow.left,info.rcWindow.top,(info.rcWindow.right - info.rcWindow.left).try_into().unwrap(),(info.rcWindow.bottom-info.rcWindow.top).try_into().unwrap()),
                    hwnd: window
                };
                curr_apps.push(c_app)
            }
        }

        true.into()
    }
}

fn main() {
    let tray = SystemTray::new();

    tauri::Builder::default()
        .setup(|app|{
            let main_window = app.get_window("main").unwrap();
            main_window.set_always_on_top(true).expect("Oopsie");
            Ok(())})
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
            SystemTrayEvent::RightClick { .. } =>{
                let window = app.get_window("main").unwrap();
                window.close().expect("won't close");
            }
            _ => {}
        })
        .manage(Handles(Default::default()))
        .invoke_handler(tauri::generate_handler![get_apps,restore,screen_shot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
