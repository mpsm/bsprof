use serde::Serialize;
use std::{sync::mpsc::Receiver, sync::mpsc::Sender, time::Duration};
use sysinfo::{CpuExt, SystemExt};

pub mod info;
mod rusage;

#[derive(Serialize)]
pub struct ProfileDatapoint {
    elapsed: f64,
    cpu_usage: f32,
    cpus_utilization: Vec<f32>,
    memory_usage: u64,
}

#[derive(Serialize)]
pub struct ProfileTimings {
    elapsed: f64,
    warmup: f64,
    cooldown: f64,
    interval: f64,
}

#[derive(Serialize)]
pub struct ProfileReport {
    system_info: info::SystemInfo,
    profile_timings: ProfileTimings,
    cmd_name: String,
    cmd_args: Vec<String>,
    rusage: rusage::Rusage,
    datapoints: Vec<ProfileDatapoint>,
}

#[derive(PartialEq)]
enum ThreadCommand {
    Stop,
}

fn get_data_point(sys: &mut sysinfo::System, start_time: &std::time::Instant) -> ProfileDatapoint {
    let cpus_data = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    let elapsed_time = std::time::Instant::now() - start_time.clone();

    sys.refresh_memory();
    sys.refresh_cpu();

    ProfileDatapoint {
        elapsed: elapsed_time.as_secs_f64(),
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        memory_usage: sys.used_memory(),
        cpus_utilization: cpus_data,
    }
}

fn monitor_thread(rx: Receiver<ThreadCommand>, interval: Duration) -> Vec<ProfileDatapoint> {
    let mut sys = info::create_system_info();
    let mut datapoints = Vec::<ProfileDatapoint>::new();
    let start_time = std::time::Instant::now();

    loop {
        datapoints.push(get_data_point(&mut sys, &start_time));

        match rx.recv_timeout(interval) {
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

pub fn profile(
    cmd: &String,
    args: &[String],
    interval: Duration,
    warmup: Duration,
    cooldown: Duration,
) -> ProfileReport {
    let sys_info = info::get_system_info();

    // run monitroing thread and spawn command
    let (tx, rx): (Sender<ThreadCommand>, Receiver<ThreadCommand>) = std::sync::mpsc::channel();
    let monitor = std::thread::spawn(move || monitor_thread(rx, interval));

    // warmup
    std::thread::sleep(warmup);

    let start_time = std::time::Instant::now();
    let mut cmd_process = std::process::Command::new(cmd).args(args).spawn().unwrap();

    // wait for command to finish and kill monitoring thread
    cmd_process.wait().unwrap();
    let elapsed_time = std::time::Instant::now() - start_time;

    // cooldown
    std::thread::sleep(cooldown);

    tx.send(ThreadCommand::Stop).unwrap();
    let datapoints = monitor.join().unwrap();
    let usage = rusage::get_process_rusage();

    let profile_timings = ProfileTimings {
        elapsed: elapsed_time.as_secs_f64(),
        warmup: warmup.as_secs_f64(),
        cooldown: cooldown.as_secs_f64(),
        interval: interval.as_secs_f64(),
    };

    // return report
    ProfileReport {
        system_info: sys_info,
        profile_timings: profile_timings,
        cmd_name: cmd.clone(),
        cmd_args: args.to_vec(),
        rusage: usage,
        datapoints: datapoints,
    }
}
