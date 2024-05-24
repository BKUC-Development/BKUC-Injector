use crate::util::clear_console;
use injrs::inject_windows::*;
use injrs::process_windows::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct ProcessHandler<'a> {
    name: &'a str,
    dll: &'a str,
}

impl<'a> ProcessHandler<'a> {
    pub fn new(name: &'a str, dll: &'a str) -> Self {
        Self { name, dll }
    }

    pub fn monitor_process(&self, infinite: bool) {
        const ANIMATION: [&str; 4] = ["-", "/", "|", "\\"];
        let mut counter = 4;

        loop {
            thread::sleep(Duration::from_secs(1));
            let process = Process::find_first_by_name(self.name);

            match process {
                None => {
                    counter = (counter - 1).max(1);
                    clear_console(0);

                    print!(
                        "{} Waiting for PG3D to start {}",
                        counter,
                        ANIMATION[counter - 1]
                    );
                    io::stdout().flush().unwrap();
                }
                Some(p) => {
                    clear_console(0);
                    println!("Process found!");
                    thread::sleep(Duration::from_secs(1));

                    thread::sleep(Duration::from_secs(4));
                    match p.inject(self.dll) {
                        Err(e) => {
                            println!("Error: {}", e);
                            thread::sleep(Duration::from_secs(1));
                        }
                        Ok(_) => {
                            println!("Succesfully kissed boys, Have fun! PID: {}", p.pid);
                            println!("Discord: https://discord.gg/security-research");
                            thread::sleep(Duration::from_secs(1));
                            loop {
                                thread::sleep(Duration::from_secs(1));
                                match Process::find_first_by_name(self.name) {
                                    Some(_) => {}
                                    None => {
                                        println!("Process closed, exiting...");
                                        thread::sleep(Duration::from_secs(1));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if infinite {
                        clear_console(3);
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
