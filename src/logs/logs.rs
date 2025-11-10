
use std::fs::{File, OpenOptions, exists};
use std::io::{self, Write};
use chrono::Local;


pub const FILE_LOG_PATH: &str = "./log.log";


pub struct Logs {
    pub file_log: String
}


impl Logs {
    
    pub fn open_log() -> Result<File, io::Error> {

        if exists(FILE_LOG_PATH).is_err() {
            match (File::create(FILE_LOG_PATH)) {
                Ok(_) => println!("File log created"),
                Err(e) => eprintln!("{:?}", e)
            }
        }
        
        let file_log = File::options()
        .append(true)
        .create(true)
        .open(FILE_LOG_PATH);
        
        file_log
    } 

    pub fn trace(message: &str) -> io::Result<()> {
        let mut file = Logs::open_log()?;
         
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "[TRACE]({}) {}", timestamp, message)?;

        Ok(())
    }

    pub fn error(message: &str) -> io::Result<()> {
        let mut file = Logs::open_log()?; 
         
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "[ERROR]({}) {}", timestamp, message)?;

        Ok(())
    }

    pub fn warn(message: &str) -> io::Result<()> {
        let mut file = Logs::open_log()?;
         
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "[WARN]({}) {}", timestamp, message)?;

        Ok(())
    }


}






