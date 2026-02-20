use std::{fs, io::{BufReader, Write}, path::PathBuf};
use std::env;

use std::io::BufRead;
use chrono;

pub const MOUNT_PATH: &str = "/mnt/drive1";
pub const DRIVE: &str = "sda";
 
pub const DEFAULT_TIMER_MIN: u32 = 90; // In minutes
pub const LOOP_SECS: u32 = 600; // 10 Minutes. Drive spinsdown at 15 minutes.
pub const KEEPALIVE_FILE: &str = "/mnt/drive1/.keepalive.txt";


/// Resolves the path to 'keep_alive.conf' in the same folder as the binary.
fn get_config_path() -> PathBuf {
    if let Ok(mut exe_path) = env::current_exe() {
        if exe_path.pop() {
            return exe_path.join("keep_alive.conf");
        }
    }

    PathBuf::from("keep_alive.conf")
}


/// Reads the timer value from the local config file.
///
/// Timer is in minutes
pub fn get_timer_duration() -> u32 {
    let config_path = get_config_path();
    
    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(val) = content.trim().parse::<u32>() {
            return val;
        }
    }

    // If reading/parsing fails, create the file with the default value
    // This ensures the file exists for you to edit later
    let _ = fs::write(&config_path, DEFAULT_TIMER_MIN.to_string());
    
    DEFAULT_TIMER_MIN
}

/// The amount of loops calculated to comply with the config
///
/// Total time is loops * 10 min + 5 min to shut down
pub fn calculate_loops() -> u8{
    let timer = get_timer_duration();
    let loops = ((timer-10)*60) / LOOP_SECS;

    loops as u8    
}




pub fn get_io_count(drive: &str) -> u64 {
    let file = match fs::File::open("/proc/diskstats") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if line.contains(drive) {
            let fields: Vec<&str> = line.split_whitespace().collect();
            // fields[3] = reads, fields[7] = writes
            let reads = fields.get(3).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let writes = fields.get(7).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            return reads + writes;
        }
    }
    0       
}

pub fn is_mounted(path_to_check: &str) -> bool {
    fs::metadata(path_to_check).is_ok()
}

pub fn write_to_dummy(dummy_file: &str, counter: &u8) -> std::io::Result<()> {
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let timestamp = now.format("%Y-%m-%d_%H:%M");
    let content = format!("keepalive {} {}/4", timestamp, counter);

    // Determine the base directory of the executable
    let exe_dir = env::current_exe()?
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find parent directory"))?;

    // Create the absolute path for the hidden copy
    let keep_alive_path = exe_dir.join(".keep_alive_copy.txt");

    // List of paths to update (using PathBuf for the joined path)
    // Dummy + local copy for check
    let files_to_update: Vec<PathBuf> = vec![
        PathBuf::from(dummy_file),
        keep_alive_path,
    ];

    for path in files_to_update {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)?;

        writeln!(file, "{}", content)?;
        file.sync_all()?;
    }
    
    println!("Activity triggered: Write successful.");
    Ok(())
}
