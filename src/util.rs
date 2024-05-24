use crossterm::{cursor, execute, terminal};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use log::{error, info};

pub fn clear_console(c_move: u16) {
    if let Err(e) = if c_move == 0 {
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
    } else {
        execute!(
            io::stdout(),
            cursor::MoveUp(c_move),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )
    } {
        error!("Failed to clear console: {}", e);
    }
}

pub fn calculate_sha256<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = File::open(&path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = format!("{:x}", hasher.finalize());
    info!("Calculated SHA-256 for {}: {}", path.as_ref().display(), result);
    Ok(result)
}

pub fn log_and_exit(message: &str) {
    error!("{}", message);
    println!("{}", message);
    std::process::exit(1);
}

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_string_to_file<P: AsRef<Path>>(path: P, data: &str) -> Result<(), io::Error> {
    let mut file = File::create(&path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
