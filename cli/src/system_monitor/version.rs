use std::{process::Command};
use regex::Regex;
use crate::logs::{Logs};


#[derive(Debug, Clone)]
pub struct Version {
    name: String,
    version: String,
}

fn extract_version(output: &str) -> Option<String> {
    let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    re.captures(output)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

pub fn get_version(software: String) -> Result<Version, String> {
    let output = Command::new(&software)
        .arg("--version")
        .output()
        .map_err(|e| {
            let _ = Logs::error(&format!("Error get version: {}", e));
            format!("Errore nell'esecuzione: {}", e)
        })?;

    if output.status.success() {
        let version_str = String::from_utf8_lossy(&output.stdout).to_string();
        let version = extract_version(&version_str)
            .ok_or_else(|| "Versione non trovata".to_string())?;
        let _ = Logs::trace(&format!("Get Version on {}", software));
        Ok(Version {
            name: software,
            version,
        })
        
    } else {
        let _ = Logs::error(&format!("Error command failed"));
        Err("Comando fallito".to_string())
    }
}