use crate::util::calculate_sha256;
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub struct Downloader<'a> {
    url: &'a str,
    destination: &'a str,
}

impl<'a> Downloader<'a> {
    pub fn new(url: &'a str, destination: &'a str) -> Self {
        Self { url, destination }
    }

    pub fn download_and_update(&self) {
        if !Path::new("bin").exists() {
            fs::create_dir("bin").unwrap();
        }

        let bin_dll_path = Path::new(self.destination);
        let existing_hash = if bin_dll_path.exists() {
            calculate_sha256(&bin_dll_path).ok()
        } else {
            None
        };

        println!("Downloading Boykisser Uncentral...");
        thread::sleep(Duration::from_secs(1));

        let client = Client::new();
        let response = match client.get(self.url).send() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to send request: {}", e);
                thread::sleep(Duration::from_secs(1));
                return;
            }
        };

        let content = match response.bytes() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read response bytes: {}", e);
                thread::sleep(Duration::from_secs(1));
                return;
            }
        };

        let new_hash = format!("{:x}", Sha256::digest(&content));

        if existing_hash.as_deref() != Some(&new_hash) {
            let mut file = match File::create(&bin_dll_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to create file: {}", e);
                    thread::sleep(Duration::from_secs(1));
                    return;
                }
            };

            if let Err(e) = file.write_all(&content) {
                eprintln!("Failed to write to file: {}", e);
                thread::sleep(Duration::from_secs(1));
                return;
            }

            println!("BoyKisser Uncentral is now up to date!");
            thread::sleep(Duration::from_secs(1));
        } else {
            println!("BoyKisser Uncentral is already up to date!");
            thread::sleep(Duration::from_secs(1));
        }
    }
}
