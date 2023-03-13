mod args;
mod profile;

use args::parse_args;
use std::fs::File;

fn main() {
    let cmd = clap::Command::new("Build System Profiler")
        .author("Marcin Smoczyński, smoczynski.marcin@gmail.com")
        .about("A simple tool for profiling build systems")
        .arg(
            clap::Arg::new("interval_ms")
                .help("Interval in ms between data points")
                .short('i')
                .long("interval")
                .default_value("1000"),
        )
        .arg(
            clap::Arg::new("warmup_ms")
                .short('w')
                .long("warmup")
                .help("Warmup time in ms")
                .default_value("0"),
        )
        .arg(
            clap::Arg::new("cooldown_ms")
                .short('c')
                .long("cooldown")
                .help("Cooldown time in ms")
                .default_value("0"),
        )
        .arg(
            clap::Arg::new("command")
                .required(true)
                .help("Command to run"),
        )
        .arg(
            clap::Arg::new("args")
                .num_args(0..)
                .help("Command arguments"),
        );

    let args = parse_args(cmd).unwrap();

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
