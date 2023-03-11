use sysinfo::{CpuExt, SystemExt};

struct SystemInfo {
    num_cpus: u32,
    cpu_name: String,
    total_memory: u64,
    os: String,
}

fn get_system_info() -> SystemInfo {
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
    fn print(&self) {
        println!("OS name       \t: {}", self.os);
        println!("CPU name      \t: {}", self.cpu_name);
        println!("Number of CPUs\t: {}", self.num_cpus);
        println!("Total memory  \t: {} MB", self.total_memory / 1024 / 1024);
    }
}

fn main() {
    println!("Build system profiler application");
    // run application specified in process arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No application specified");
        return;
    }

    let sys_info = get_system_info();
    sys_info.print();

    let cmd_name = &args[1];
    let cmd_args = &args[2..];
    if cmd_args.len() > 0 {
        println!("Running command: {} {:?}", cmd_name, cmd_args);
    } else {
        println!("Running command: {}", cmd_name);
    }

    let mut cmd = std::process::Command::new(cmd_name)
        .args(cmd_args)
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
