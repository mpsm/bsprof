use std::fs::File;

use clap::Parser;

mod profile;

#[derive(Parser, Debug)]
#[command(name = "bsprof")]
#[command(author = "Marcin Smoczyński <smoczynski.marcin@gmail.com>")]
#[command(version)]
#[command(about = "A simple tool for profiling build systems", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1000)]
    interval_ms: u32,
    command: String,
    args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let sys_info = profile::info::get_system_info();
    sys_info.print();

    if args.args.len() > 0 {
        println!("Running command: {} {:?}", args.command, args.args);
    } else {
        println!("Running command: {}", args.command);
    }

    let interval = std::time::Duration::from_millis(args.interval_ms as u64);
    let report = profile::profile(&args.command, &args.args, interval);

    println!("Done, saving report");
    save_report(&report);
}

fn save_report(report: &profile::ProfileReport) {
    let file = File::create("report.json").unwrap();
    serde_json::to_writer_pretty(file, &report).unwrap();
}
