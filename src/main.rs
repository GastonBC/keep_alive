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
    
    /*
    if !is_mounted(MOUNT_PATH) {
        println!("No drive mounted");
    }
*/

    let config_path = get_config_path();
    let config = load_config(&config_path);

    let mut loops = config.calculate_loops(); 
    let mut last_io = get_io_count_by_uuid(&config.uuid);
    let mut counter: u8 = loops + 1;
    
    println!("Settings:");
    println!("UUID {}", config.uuid);
    println!("MOUNT PATH{}", config.mount_path);
    println!("ALIVE FILE {}", &config.keepalive_file);
    println!();
    println!("TIMER MIN {}", config.timer_min);                         // Total minutes that you want the disk to stay spinning. Eg 60 minutes
    println!("LOOP SECS {}", config.loop_secs);                         // While loop delay. Disk spins down at 15 mins so we set the loop to check every 10 mins
    println!("TOTAL LOOPS {}", &config.calculate_loops().to_string());  // How many loops will execute before letting it spin down

    loop {
        thread::sleep(Duration::from_secs(config.loop_secs.into()));
    
        if !is_mounted(&config.mount_path) {
            println!("Drive not mounted. Skipping cycle.");
            continue;
        }
    
        let current_io = get_io_count_by_uuid(&config.uuid);
    

        if current_io > last_io + 15 {
            counter = 1;
            println!("{counter}/{loops}: Detected activity in the last 10 minutes");
        }


        else if counter <= loops {
            println!("{counter}/{loops}: No activity detected.");
            write_to_dummy(&config.keepalive_file, &counter)?;
            counter += 1;
        
        } else {
            println!("{counter}/{loops}: Drive idle and counter exceeded. Waiting for user activity");
            counter = loops + 1; // Reset
        }
    
        // Common updates for all mounted states
        last_io = current_io;
        println!("Current IO {last_io}");
        loops = config.calculate_loops();
    }
    
} 
