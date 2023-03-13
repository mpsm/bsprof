mod args;
mod cmd;
mod profile;
mod report;

fn main() {
    let args = args::Args::parse_from_cmdline().unwrap();

    args.print();

    let sys_info = profile::info::get_system_info();
    sys_info.print();

    let jobs = args.jobs.unwrap_or(profile::info::get_cpu_count());
    let sequence = if args.sequence {
        (1..=jobs).collect()
    } else {
        vec![jobs]
    };
    println!("Profiling with {:?} jobs sequence", sequence);

    let settings = profile::ProfileSettings::new(args.interval, args.warmup, args.cooldown);
    let mut report = report::Report::new(&sys_info, &settings);
    for j in sequence {
        println!("Profiling with {} jobs", j);
        let mut cmd = cmd::Command::new(&args.command, &args.args);
        let result = profile::profile(&mut cmd, &settings, j);
        report.add_result(result);
    }

    println!("Done, saving report");
    report.save();
}
