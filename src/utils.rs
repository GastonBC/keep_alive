use std::{fs, io::{BufReader, Write}, thread, time::Duration};
use std::io::BufRead;
use chrono;

pub fn get_io_count() -> u64 {
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

pub fn is_mounted(path_to_check: &str) -> bool {
    fs::metadata(path_to_check).is_ok()
}

    

pub fn write_to_dummy(path: String, counter: &u8) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;

    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let timestamp: String = now.format("%Y-%m-%d_%H:%M");

    writeln!(file, "keepalive {} {}/4", timestamp, counter)?;
    file.sync_all()?;
    
    println!("Activity triggered: Write successful.");
    Ok(())
}