use super::cmd::Command;
use serde::Serialize;
use std::{sync::mpsc::Receiver, sync::mpsc::Sender, time::Duration};
use sysinfo::{CpuExt, SystemExt};

pub mod info;
pub mod rusage;

#[derive(Serialize, Clone)]
pub struct ProfileSettings {
    interval: Duration,
    warmup: Duration,
    cooldown: Duration,
}

impl ProfileSettings {
    pub fn new(interval: Duration, warmup: Duration, cooldown: Duration) -> ProfileSettings {
        ProfileSettings {
            interval,
            warmup,
            cooldown,
        }
    }
}

#[derive(Serialize)]
pub struct ProfileDatapoint {
    elapsed: f64,
    cpu_usage: f32,
    cpus_utilization: Vec<f32>,
    memory_usage: u64,
}

#[derive(Serialize)]
pub struct ProfileResult {
    pub elapsed_time: f64,
    pub jobs: u32,
    pub cmd_name: String,
    pub cmd_args: Vec<String>,
    pub rusage: rusage::Rusage,
    pub datapoints: Vec<ProfileDatapoint>,
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

pub fn profile(build_cmd: &Command, settings: &ProfileSettings, jobs: u32) -> ProfileResult {
    // run monitroing thread and spawn command
    let (tx, rx): (Sender<ThreadCommand>, Receiver<ThreadCommand>) = std::sync::mpsc::channel();
    let check_interval = settings.interval.clone();
    let monitor = std::thread::spawn(move || monitor_thread(rx, check_interval));
    let mut cmd = build_cmd.clone();

    // warmup
    std::thread::sleep(settings.warmup);

    // add jobs
    cmd.add_jobs(jobs);

    let start_time = std::time::Instant::now();
    cmd.run().unwrap();
    let elapsed_time = std::time::Instant::now() - start_time;

    // cooldown
    std::thread::sleep(settings.cooldown);

    // stop monitoring thread
    tx.send(ThreadCommand::Stop).unwrap();
    let datapoints = monitor.join().unwrap();
    let usage = rusage::get_process_rusage();

    // return report
    ProfileResult {
        elapsed_time: elapsed_time.as_secs_f64(),
        jobs: jobs,
        cmd_name: cmd.name.clone(),
        cmd_args: cmd.args.to_vec(),
        rusage: usage,
        datapoints: datapoints,
    }
}
