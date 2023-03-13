mod args;
mod cmd;
mod profile;

use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
struct Report {
    system_info: profile::info::SystemInfo,
    profile_settings: profile::ProfileSettings,
    profile_results: Vec<profile::ProfileResult>,
}

impl Report {
    fn new(system_info: &profile::info::SystemInfo, settings: &profile::ProfileSettings) -> Report {
        Report {
            system_info: (*system_info).clone(),
            profile_settings: *settings,
            profile_results: Vec::new(),
        }
    }

    fn add_result(&mut self, result: profile::ProfileResult) {
        self.profile_results.push(result);
    }
}

fn main() {
    let args = args::Args::parse_from_cmdline().unwrap();

    args.print();

    let sys_info = profile::info::get_system_info();
    sys_info.print();

    let mut cmd = cmd::Command::new(&args.command, &args.args);
    if let Some(jobs) = args.jobs {
        cmd.add_jobs(jobs);
    }

    let settings = profile::ProfileSettings::new(args.interval, args.warmup, args.cooldown);
    let mut report = Report::new(&sys_info, &settings);
    let result = profile::profile(&cmd, &settings);
    report.add_result(result);

    println!("Done, saving report");
    save_report(&report);
}

fn save_report(report: &Report) {
    let file = File::create("report.json").unwrap();
    serde_json::to_writer_pretty(file, &report).unwrap();
}
