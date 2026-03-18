#![windows_subsystem = "windows"]

mod localization;
mod models;
mod native_interop;
mod poller;
mod theme;
mod updater;
mod window;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(exit_code) = updater::handle_cli_mode(&args) {
        std::process::exit(exit_code);
    }
    window::run();
}
