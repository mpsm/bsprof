mod args;
mod cmd;
mod profile;

use std::fs::File;

fn main() {
    let args = args::Args::parse_from_cmdline().unwrap();

    args.print();

    let sys_info = profile::info::get_system_info();
    sys_info.print();

    let mut cmd = cmd::Command::new(&args.command, &args.args);
    if let Some(jobs) = args.jobs {
        cmd.add_jobs(jobs);
    }

    let report = profile::profile(&cmd, args.interval, args.warmup, args.cooldown);

    println!("Done, saving report");
    save_report(&report);
}

fn save_report(report: &profile::ProfileReport) {
    let file = File::create("report.json").unwrap();
    serde_json::to_writer_pretty(file, &report).unwrap();
}
