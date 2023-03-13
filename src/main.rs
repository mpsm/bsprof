mod args;
mod cmd;
mod profile;
mod report;

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
    let mut report = report::Report::new(&sys_info, &settings);
    let result = profile::profile(&cmd, &settings);
    report.add_result(result);

    println!("Done, saving report");
    report.save();
}
