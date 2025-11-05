pub mod cpu;
pub mod memory;
pub mod storage;


pub use cpu::{Cpu, CoreInfo, cpu_check};
pub use memory::{Ram, ram_check};
pub use storage::{Storage, storage_check};