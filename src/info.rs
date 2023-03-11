use serde::Serialize;
use sysinfo::{CpuExt, SystemExt};

#[derive(Serialize)]
pub struct SystemInfo {
    num_cpus: u32,
    cpu_name: String,
    total_memory: u64,
    os: String,
}

pub fn get_system_info() -> SystemInfo {
    let sys = sysinfo::System::new_all();
    let cpu = &sys.cpus()[0];

    let os_name = get_system_name(&sys);

    SystemInfo {
        num_cpus: sys.cpus().len() as u32,
        cpu_name: cpu.vendor_id().to_string() + " / " + cpu.brand(),
        total_memory: sys.total_memory(),
        os: os_name,
    }
}

fn get_system_name(sys: &sysinfo::System) -> String {
    let mut os_name;
    match sys.name() {
        Some(name) => {
            os_name = name;
            match sys.os_version() {
                Some(version) => {
                    os_name += " ";
                    os_name += &version;
                }
                None => {}
            }
            match sys.kernel_version() {
                Some(version) => {
                    os_name += " (";
                    os_name += &version;
                    os_name += ")";
                }
                None => {}
            }
        }
        None => {
            os_name = "Unknown".to_string();
        }
    }
    os_name
}

impl SystemInfo {
    pub fn print(&self) {
        println!("OS name       \t: {}", self.os);
        println!("CPU name      \t: {}", self.cpu_name);
        println!("Number of CPUs\t: {}", self.num_cpus);
        println!("Total memory  \t: {} MB", self.total_memory / 1024 / 1024);
    }
}
