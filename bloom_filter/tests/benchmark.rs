use sysinfo::{System};
use sysinfo::*;


#[cfg(test)]
#[test]
fn check_usage()
{
        // =============================================================================================
        let mut sys = System::new_all();
        sys.refresh_all();
    
        // Get current process
        if let Some(process) = sys.process(sysinfo::get_current_pid().unwrap()) {
            println!("Memory usage: {} Bytes", process.memory());
            println!("Virtual memory: {} Bytes", process.virtual_memory());
            println!("CPU usage: {} %", process.cpu_usage());
        }
        // =============================================================================================
}