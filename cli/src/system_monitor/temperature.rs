use sysinfo::{self, System, Components};
use crate::logs::{Logs};

#[derive(Debug, Clone)]
pub struct Temp {
    name: String,
    max: Option<f32>,
    temperature: Option<f32>,
    critical: Option<f32>
}

pub fn get_temperature() -> Vec<Temp> {
    let mut output: Vec<Temp> = Vec::new();
    
    let components = Components::new_with_refreshed_list();

    for comp in &components {
        
        let name = comp.label().to_string();
        let max = comp.max();
        let temp = comp.temperature();
        let crit = comp.critical();

        output.push(Temp {
            name: name,
            max: max,
            temperature: temp,
            critical: crit
        })
    }
    let _ = Logs::trace("Temperature check");

    output
}
