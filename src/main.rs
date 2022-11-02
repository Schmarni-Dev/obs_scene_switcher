#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// use anyhow::Result;
// use obws::Client;

// #[tokio::main]
fn main() {
    let password = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            println!("no password given. try help");
            std::process::exit(0);
        }
    };

    if password.to_lowercase() == "help" {
        println!("arg1: password");
        println!("arg2: port (optinal default = 4455)");
        println!("arg3: ip (optinal default = localhost)");
        std::process::exit(0);
    }
    let port = match std::env::args().nth(2) {
        // Some(v) => v.parse::<u16>().unwrap(),
        _ => 4455 as u16,
    }
    .clone();
    let host = match std::env::args().nth(2) {
        Some(v) => v,
        None => "localhost".to_owned(),
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "OBS SCENE SWITCHER",
        options,
        Box::new(move |cc| {
            Box::new(obs_scene_switcher::ObsSwitcher::new(
                cc, host, port, password,
            ))
        }),
    );
}

// fn main() {
//     let options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "Confirm exit",
//         options,
//         Box::new(|_cc| Box::new(obs_scene_switcher::ObsSwitcher::default())),
//     );
// }
