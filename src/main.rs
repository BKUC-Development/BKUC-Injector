mod download;
mod process;
mod util;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use console::Term;
use download::Downloader;
use process::ProcessHandler;
use util::clear_console;

fn main() {
    let term = Term::stdout();
    let title = "Boykisser Uncentral";
    term.set_title(title);
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();

    let name = "Pixel Gun 3D.exe";
    let dll_path = "bin/PixelGunCheat.dll";
    let infinite = true;

    println!("Welcome to BoyKisser Uncentral!");
    println!("https://github.com/BKUC-Development");
    println!("GET SUPPORT HERE:");
    println!("https://discord.gg/security-research");
    println!("");
    println!("Please make sure your anti-virus is disabled! Download and injecting the dll manually is not recommended.");
    println!("");
    println!("Press ENTER to continue.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    clear_console(0);

    let downloader = Downloader::new("https://cdn.aero.nu/PixelGunCheat.dll", dll_path);
    downloader.download_and_update();

    std::process::Command::new("cmd")
        .args(&["/C", "start", "steam://rungameid/2524890"])
        .output()
        .expect("failed to execute process");

    clear_console(2);

    let process_handler = ProcessHandler::new(name, dll_path);
    process_handler.monitor_process(infinite);

    println!("Press ENTER to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
