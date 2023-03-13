use serde::Serialize;
use sysinfo::{CpuExt, CpuRefreshKind, SystemExt};

#[derive(Serialize, Clone)]
pub struct SystemInfo {
    pub num_cpus: u32,
    pub cpu_name: String,
    pub total_memory: u64,
    pub os: String,
}

pub fn create_system_info() -> sysinfo::System {
    let refresh_kind = sysinfo::RefreshKind::new()
        .with_cpu(CpuRefreshKind::everything())
        .with_memory();

    sysinfo::System::new_with_specifics(refresh_kind)
}

pub fn get_cpu_count() -> u32 {
    let sys = create_system_info();
    sys.cpus().len() as u32
}

pub fn get_system_info() -> SystemInfo {
    let sys = create_system_info();
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
