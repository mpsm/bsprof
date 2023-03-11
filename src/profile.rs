use serde::Serialize;
use std::{sync::mpsc::Receiver, sync::mpsc::Sender, time::Duration};
use sysinfo::{CpuExt, SystemExt};

pub mod info;
mod rusage;

#[derive(Serialize)]
pub struct ProfileDatapoint {
    cpu_usage: f32,
    cpus_utilization: Vec<f32>,
    memory_usage: u64,
}

#[derive(Serialize)]
pub struct ProfileReport {
    system_info: info::SystemInfo,
    cmd_name: String,
    cmd_args: Vec<String>,
    rusage: rusage::Rusage,
    datapoints: Vec<ProfileDatapoint>,
}

#[derive(PartialEq)]
enum ThreadCommand {
    Stop,
}

fn get_data_point(sys: &sysinfo::System) -> ProfileDatapoint {
    let cpus_data = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    ProfileDatapoint {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        memory_usage: sys.used_memory(),
        cpus_utilization: cpus_data,
    }
}

fn monitor_thread(rx: Receiver<ThreadCommand>) -> Vec<ProfileDatapoint> {
    let mut sys = sysinfo::System::new_all();
    let mut datapoints = Vec::<ProfileDatapoint>::new();

    loop {
        sys.refresh_all();
        datapoints.push(get_data_point(&sys));

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(cmd) => {
                if cmd == ThreadCommand::Stop {
                    break;
                }
            }
            Err(_) => {}
        }
    }

    datapoints
}

pub fn profile(cmd: &String, args: &[String]) -> ProfileReport {
    let sys_info = info::get_system_info();

    // run monitroing thread and spawn command
    let (tx, rx): (Sender<ThreadCommand>, Receiver<ThreadCommand>) = std::sync::mpsc::channel();
    let monitor = std::thread::spawn(move || monitor_thread(rx));
    let mut cmd_process = std::process::Command::new(cmd).args(args).spawn().unwrap();

    // wait for command to finish and kill monitoring thread
    cmd_process.wait().unwrap();
    tx.send(ThreadCommand::Stop).unwrap();
    let datapoints = monitor.join().unwrap();
    let usage = rusage::get_process_rusage();

    // return report
    ProfileReport {
        system_info: sys_info,
        cmd_name: cmd.clone(),
        cmd_args: args.to_vec(),
        rusage: usage,
        datapoints: datapoints,
    }
}
