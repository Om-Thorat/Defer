# Contributing to Defer

Thank you for considering contributing to Defer!

## Development platform

Defer is a desktop tool for windows, Hence windows is the appropriate development platform. 
For technicalities pssuspend won't work on any other platform and the Windows libraries required may not be installed by default elsewhere.

If you are contributing to the frontend, You'll be able to work on that regardless of platform.

## File Structure

* The `src` folder contains the files for the frontend. The strucuture for the same is like a common vite Svelte app.

* The `src-tauri` folder contains the files for the rust backend. The `src` folder in side `src-tauri` contains the `main.rs` housing the backend functions.

* The `bin` folder in `src-tauri` contains `pssuspend` which will be loaded as a sidecar in the application.

## Requirements

Defer is a Tauri App which uses Rust for the backend and Svelte rendered with Webview using Tauri for the frontend.

1. Rust and Cargo.
2. Node and npm.

## Getting Started

To get started with contributing, please follow these steps:

1. Fork and clone the repo.
2. Install all the dependencies with `npm install`.
3. Run the project by doing `npm run tauri dev`

Tauri docs for further reference are available at [Tauri.app](https://tauri.app/v1/guides/).

## Common errors

If you get an error saying `path matching bin/pssuspend-x86_64-pc-windows-gnu.exe not found.` or similar.

Tauri expects a binary compatible with the target you are building to, The name format being `{binary-name}-{architecture}`.

Find your architecture with this command ```rustc -Vv | Select-String "host:" | ForEach-Object {$_.Line.split(" ")[1]}```
and rename `/bin/pssuspend` to the format.

---

Defer also works a little slower in dev mode due to the ScreenShot function running slower, The optimisations by rust and Tauri are further applied during compiling to production. Hence, just be a little patient.


## Reporting Issues

If you encounter any bugs, issues, or have suggestions for improvements, please open an issue on our GitHub repository. When reporting an issue, please provide as much detail as possible, including steps to reproduce the problem and any relevant error messages.

## Contact

If you have any questions or need further assistance, feel free to reach out to me at thoratom1104@gmail.com.

Happy contributing!  

<h3 align="center" >  With 💖 by Om </h3>