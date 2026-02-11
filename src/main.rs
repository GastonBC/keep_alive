use std::{fs, io::{BufReader, Write}, thread, time::Duration};
use std::io::BufRead;
use chrono;
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

fn is_mounted(path_to_check: &str) -> bool {
    fs::metadata(path_to_check).is_ok()
    }

fn get_io_count() -> u64 {
    let file = match fs::File::open("/proc/diskstats") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if line.contains(DRIVE) {
            let fields: Vec<&str> = line.split_whitespace().collect();
            // fields[3] = reads, fields[7] = writes
            let reads = fields.get(3).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let writes = fields.get(7).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            return reads + writes;
        }
    }
    0       
}

fn write_to_dummy(counter: &u8) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(KEEPALIVE_FILE)?;

    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let timestamp: String = now.format("%Y-%m-%d_%H:%M");

    writeln!(file, "keepalive {} {}/4", timestamp, counter)?;
    file.sync_all()?;
    
    println!("Activity triggered: Write successful.");
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Settings:");
    println!("Timer: {}", TIMER);
    println!("Dummy location: {}", KEEPALIVE_FILE);

    if !is_mounted(MOUNT_PATH) {
        println!("No drive mounted");
    }

    let mut last_io = get_io_count();
    let mut counter: u8 = 5;
    

    loop {
        // Check io every 10 minutes
        thread::sleep(Duration::from_secs(TIMER));

        if !is_mounted(MOUNT_PATH) {
            println!("Drive not mounted. Skipping cycle.");
            continue;
        }


        let current_io = get_io_count();
        // there were changes between sleep and check, restart counter
        // write to dummy so it restarts all the loop
        if current_io > last_io + 15 {
            println!("Detected activity in the last 10 minutes");
            counter = 1;
            write_to_dummy(&counter)?;
            last_io = get_io_count();
            println!("Current IO {}", get_io_count());
        }

        // There were no changes, write to keep alive
        else {
            // if less than 4 loops passed (40 minutes), write to keep alive
            // else it will do nothing until the if above this is triggered

            if counter <= 4 {
                println!("No activity detected. Keep alive {counter}/4");
                if let Err(e) = write_to_dummy(&counter) {
                    eprintln!("Write failed: {e}");
                }
                counter += 1;
            }
            else{
                println!("Drive idle and counter exceeded. Waiting for user activity");
            }

            // Update small io increments
            last_io = get_io_count();
            println!("Current IO {}", get_io_count());
        }

    }
}
