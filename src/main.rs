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

    // this will run only on debug and finish the program
    if cfg!(debug_assertions){

        
    let mut loops = calculate_loops();
    let mut counter: u8 = loops + 1;
    let mut last_io = 5000;

    println!("{}", loops);

    loop {



        thread::sleep(Duration::from_secs(10));
    
        let current_io = 5000;
    
        
        if current_io > last_io + 15 {
            println!("Detected activity in the last 10 minutes. Resetting");
            counter = 1; // Reset counter
            //write_to_dummy(KEEPALIVE_FILE, &counter)?;


        } else if counter <= loops {
            println!("No activity detected. Keep alive {counter}/{loops}");
            //write_to_dummy(KEEPALIVE_FILE, &counter);            
            counter += 1;

        } else {
            println!("Drive idle and counter exceeded. Waiting for user activity");
            counter = loops + 1; 
            // Match the counter so that it will only reset with user activity
            // If I didn't match it, it would write to the dummy just because user updated the config (Number would keep growing)
        }
    
    //     // Common updates for all mounted states
    //     last_io = current_io;
    //     println!("Current IO {last_io}");
           loops = calculate_loops();
        }

    }







    // ------------ RELEASE CODE ---------------
    // ------------ RELEASE CODE ---------------
    // ------------ RELEASE CODE ---------------
    // ------------ RELEASE CODE ---------------



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
