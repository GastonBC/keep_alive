use std::{thread, time::Duration};
mod utils;
use crate::utils::*;

/*
Steps to refresh service
sudo systemctl stop hdd-keepalive
sudo systemctl restart hdd-keepalive

BUILD USING THIS FOR RPI
cross build --release --target aarch64-unknown-linux-gnu
 */

 




fn main() -> std::io::Result<()> {

    if !is_mounted(MOUNT_PATH) {
        println!("No drive mounted");
    }

    let mut loops = calculate_loops();
    let mut last_io = get_io_count(DRIVE);
    let mut counter: u8 = 5;
    
    println!("Settings:");
    println!("Timer: {}", LOOP_SECS);
    print!("Loops: {}", loops);
    println!("Dummy location: {}", KEEPALIVE_FILE);


    loop {
        thread::sleep(Duration::from_secs(LOOP_SECS.into()));
    
        if !is_mounted(MOUNT_PATH) {
            println!("Drive not mounted. Skipping cycle.");
            continue;
        }
    
        let current_io = get_io_count(DRIVE);
    
        if current_io > last_io + 15 {
            println!("Detected activity in the last 10 minutes");
            counter = 1;
            write_to_dummy(KEEPALIVE_FILE, &counter)?;

        } else if counter <= loops {
            println!("No activity detected. Keep alive {counter}/loops");
            if let Err(e) = write_to_dummy(KEEPALIVE_FILE, &counter) {
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
