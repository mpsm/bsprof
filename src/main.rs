mod args;
mod profile;

use std::fs::File;

fn main() {
    let args = args::Args::parse_from_cmdline().unwrap();

    let sys_info = profile::info::get_system_info();
    sys_info.print();

    println!("Profiling command:   {}", args.command);
    println!("Profiling args:      {:?}", args.args);
    println!("Profiling warmup:    {} ms", args.warmup.as_millis());
    println!("Profiling cooldown:  {} ms", args.cooldown.as_millis());
    println!("Profiling interval:  {} ms", args.interval.as_millis());

    let report = profile::profile(
        &args.command,
        &args.args,
        args.interval,
        args.warmup,
        args.cooldown,
    );

    println!("Done, saving report");
    save_report(&report);
}

fn save_report(report: &profile::ProfileReport) {
    let file = File::create("report.json").unwrap();
    serde_json::to_writer_pretty(file, &report).unwrap();
}
