use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Container {
    id: String,
    cpu_usage: f64,
    ram_usage: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SysInfo {
    containers: Vec<Container>,
}

fn main() -> std::io::Result<()> {
    // Ruta al archivo JSON
    let file_path = "data/sysinfo_2000.json";
    
    // Abrir el archivo
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Leer y deserializar el JSON
    let sysinfo: SysInfo = serde_json::from_reader(reader)?;

    // Procesar los datos
    for container in sysinfo.containers {
        println!(
            "Container ID: {}, CPU Usage: {}%, RAM Usage: {}MB",
            container.id, container.cpu_usage, container.ram_usage
        );
    }

    Ok(())
}
