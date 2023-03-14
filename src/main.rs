mod args;
mod cmd;
mod profile;
mod report;

fn main() {
    let args = args::Args::parse_from_cmdline().unwrap();

    args.print();

    let sys_info = profile::info::get_system_info();
    sys_info.print();

    let sequence = args.create_sequence();
    println!("Profiling with {:?} jobs sequence", sequence);

    let settings = profile::ProfileSettings::new(args.interval, args.warmup, args.cooldown);
    let mut report = report::Report::new(&sys_info, &settings);
    let clean_command = cmd::Command::new(&args.command, &vec![args.clean_target.clone()]);
    let mut build_cmd = cmd::Command::new(&args.command, &args.args);
    build_cmd.add_arg(&args.target);

    let mut last_rusage = profile::rusage::get_process_rusage();
    for j in sequence {
        println!("Cleaning up");
        clean_command.run().unwrap();

        println!("Profiling with {} jobs", j);

        let mut result = profile::profile(&build_cmd, &settings, j);
        let corrected_rusage = result.rusage - last_rusage;
        last_rusage = result.rusage;
        result.rusage = corrected_rusage;
        report.add_result(result);
    }

    println!("Done, saving report");
    report.save();
}
