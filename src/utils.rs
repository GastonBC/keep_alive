use std::{fs, collections::HashMap, io::{BufReader, Write}, path::PathBuf};
use std::env;

use std::io::BufRead;
use chrono;

 
pub struct Config {
    pub uuid: String,
    pub mount_path: String,
    pub timer_mins: u32,
    pub keepalive_file: String,
    pub loop_secs: u32
}

impl Config {
    pub fn default() -> Self {
        Self {
            uuid: "XX-XX".to_string(),
            mount_path: "/XX/XX".to_string(),
            timer_mins: 90,
            keepalive_file: "XX/XX".to_string(),
            loop_secs: 600,
        }
    }

    /// Calcula la cantidad de vueltas para cumplir con el timer.
    /// Total time is loops * 10 min + 5 min to shut down.
    pub fn calculate_loops(&self) -> u8 {
        // Evitamos overflow si timer_min es menor a 10
        if self.timer_mins <= 10 {
            return 1; 
        }

        let loops = ((self.timer_mins - 10) * 60) / self.loop_secs;
        
        // Retornamos como u8, limitando al máximo valor de u8 para evitar pánico
        loops.min(u8::MAX as u32) as u8
    }

}


/// ----------------- READ ACTIONS 

/// Resolves the path to 'keep_alive.conf' in the same folder as the binary.
pub fn get_config_path() -> PathBuf {
    if let Ok(mut exe_path) = env::current_exe() {
        if exe_path.pop() {
            return exe_path.join("keep_alive.conf");
        }
    }

    PathBuf::from("keep_alive.conf")
}

pub fn load_config(config_path: &PathBuf ) -> Config {
    let mut config = Config::default();

    if let Ok(content) = fs::read_to_string(config_path) {
        let mut map = HashMap::new();
        
        // Parseo básico de líneas CLAVE="VALOR"
        for line in content.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let val = parts[1].trim().trim_matches('"'); // Quita espacios y comillas
                map.insert(key, val);
            }
        }

        // Asignar valores si existen en el archivo
        if let Some(v) = map.get("UUID") { config.uuid = v.to_string(); }
        if let Some(v) = map.get("MOUNT_PATH") { config.mount_path = v.to_string(); }
        if let Some(v) = map.get("LOOP_SECS") {
            if let Ok(n) = v.parse::<u32>() { config.loop_secs = n; }
        }
        if let Some(v) = map.get("TIMER_MINS") {
            if let Ok(n) = v.parse::<u32>() { config.timer_mins = n; }
        }
        if let Some(v) = map.get("KEEPALIVE_FILE") { config.keepalive_file = v.to_string(); }
        
    } else {
        // Si no existe, crear un archivo ejemplo con los defaults
        let default_content = format!(
            "UUID=\"{}\"\nMOUNT_PATH=\"{}\"\nTIMER_MIN={}\nKEEPALIVE_FILE=\"{}\"\n",
            config.uuid, config.mount_path, config.timer_mins, config.keepalive_file
        );
        let _ = fs::write(config_path, default_content);
    }

    config
}


/// ----------------- ACTIONS


pub fn get_io_count_by_uuid(uuid: &str) -> u64 {
    let device_name = match resolve_uuid_to_device(uuid) {
        Some(name) => name,
        None => return 0,
    };

    // Si el nombre es "sdb1", limpiamos el número para obtener el disco "sdb"
    let drive = device_name.trim_end_matches(|c: char| c.is_numeric());

    let file = match fs::File::open("/proc/diskstats") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        
        // El nombre del disco está en la columna 2 (índice 2)
        if fields.get(2) == Some(&drive) {
            let reads = fields.get(3).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let writes = fields.get(7).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            return reads + writes;
        }
    }
    0
}


/// ----------------- WRITE ACTIONS


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

/// ------------------------------ PATH, MOUNT ------------------------


pub fn is_mounted(path_to_check: &str) -> bool {
    fs::metadata(path_to_check).is_ok()
}

fn resolve_uuid_to_device(uuid: &str) -> Option<String> {
    let path = format!("/dev/disk/by-uuid/{}", uuid);
    // canonicalize resuelve el link simbólico: 
    // de "/dev/disk/by-uuid/123..." a "/dev/sdX1"
    fs::canonicalize(path).ok().and_then(|p| {
        p.file_name()
            .map(|name| name.to_string_lossy().into_owned())
    })
}



