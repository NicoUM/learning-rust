use std::thread;
use std::time::Duration;
use sysinfo::{Disks, System};

fn main() {
    let mut sys = System::new_all();
    const GB: u64 = 1073741824;

    loop {

        clear_console();

        thread::sleep(Duration::from_secs(2));

        sys.refresh_all();

        // System:
        println!("System name:             {:?}", System::name());
        println!("System kernel version:   {:?}", System::kernel_version());
        println!("System OS version:       {:?}", System::os_version());
        println!("System host name:        {:?}", System::host_name());

        // CPU:
        println!("CPU usage:               {:?}%", sys.global_cpu_usage());

        // RAM:
        println!("Total memory:           {} GB", sys.total_memory() / GB, );
        println!("Used memory :           {} GB {} %", sys.used_memory() / GB, sys.used_memory() as f32 / sys.total_memory() as f32 * 100 as f32);

        // Disk Information
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            let available: u64 = disk.available_space() / GB;
            let total: u64 = disk.total_space() / GB;
            let percent:f32  = (available as f32/ total as f32) * 100 as f32;
            println!("Disk {:?} available space: {:?} of {:?} ({:?}%)", disk.name(), available, total, percent);
        }
    }

    
}


fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}