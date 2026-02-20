use std::{thread, time::Duration};
mod utils;
use crate::utils::*;

/*
Steps to refresh service

sudo chmod +x keep_alive
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
    let mut counter: u8 = loops + 1;
    
    println!("Settings:");
    println!("Loops: {}. 10 minutes each + 15 minutes for spindown", loops);
    println!("Dummy location: {}", KEEPALIVE_FILE);


    loop {
        thread::sleep(Duration::from_secs(LOOP_SECS.into()));
    
        if !is_mounted(MOUNT_PATH) {
            println!("Drive not mounted. Skipping cycle.");
            continue;
        }
    
        let current_io = get_io_count(DRIVE);
    

        if current_io > last_io + 15 {
            counter = 1;
            println!("{counter}/{loops}: Detected activity in the last 10 minutes");
        }


        if counter <= loops {
            println!("{counter}/{loops}: No activity detected.");
            write_to_dummy(KEEPALIVE_FILE, &counter)?;
            counter += 1;
        
        } else {
            println!("{counter}/{loops}: Drive idle and counter exceeded. Waiting for user activity");
            counter = loops + 1; // Reset
        }
    
        // Common updates for all mounted states
        last_io = current_io;
        println!("Current IO {last_io}");
        loops = calculate_loops();
    }
} 
