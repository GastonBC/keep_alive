use std::{thread, time::Duration};
mod utils;

/*
Steps to refresh service
sudo systemctl stop hdd-keepalive
sudo systemctl restart hdd-keepalive

BUILD USING THIS FOR RPI
cross build --release --target aarch64-unknown-linux-gnu
 */

 
const MOUNT_PATH: &str = "/mnt/drive1";
const DRIVE: &str = "sda";


// Release
const TIMER: u64 = 600;
const KEEPALIVE_FILE: &str = "/mnt/drive1/.keepalive.txt";

// Debug
// const TIMER: u64 = 10;
// const KEEPALIVE_FILE: &str = "/media/gaston/Drive1/keepalive.txt";



fn main() -> std::io::Result<()> {
    println!("Settings:");
    println!("Timer: {}", TIMER);
    println!("Dummy location: {}", KEEPALIVE_FILE);

    if !utils::is_mounted(MOUNT_PATH) {
        println!("No drive mounted");
    }

    let mut last_io = utils::get_io_count(DRIVE);
    let mut counter: u8 = 5;
    

    loop {
        thread::sleep(Duration::from_secs(TIMER));
    
        if !utils::is_mounted(MOUNT_PATH) {
            println!("Drive not mounted. Skipping cycle.");
            continue;
        }
    
        let current_io = utils::get_io_count(DRIVE);
    
        if current_io > last_io + 15 {
            println!("Detected activity in the last 10 minutes");
            counter = 1;
            utils::write_to_dummy(KEEPALIVE_FILE, &counter)?;
        } else if counter <= 4 {
            println!("No activity detected. Keep alive {counter}/4");
            if let Err(e) = utils::write_to_dummy(KEEPALIVE_FILE, &counter) {
                eprintln!("Write failed: {e}");
            }
            counter += 1;
        } else {
            println!("Drive idle and counter exceeded. Waiting for user activity");
        }
    
        // Common updates for all mounted states
        last_io = current_io;
        println!("Current IO {last_io}");
    }
}
