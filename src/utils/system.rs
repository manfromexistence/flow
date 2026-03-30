use sysinfo::System;

/// Get system memory information
pub fn get_memory_info() -> (u64, u64) {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let total = sys.total_memory();
    let available = sys.available_memory();
    
    (total, available)
}

/// Check if system has enough memory for model
pub fn check_memory_requirements(required_mb: u64) -> bool {
    let (_, available) = get_memory_info();
    let available_mb = available / 1024 / 1024;
    available_mb >= required_mb
}
