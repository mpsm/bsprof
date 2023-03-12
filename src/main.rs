use std::fs::File;

mod profile;

struct Args {
    interval: u32,
    warmup: u32,
    command: String,
    args: Vec<String>,
}

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
                .help("Warmup time in ms"),
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

    println!("Profiling command: {}", args.command);
    println!("Profiling args: {:?}", args.args);
    println!("Profiling warmup: {} ms", args.warmup);
    println!("Profiling interval: {} ms", args.interval);

    let interval = std::time::Duration::from_millis(args.interval as u64);
    let report = profile::profile(&args.command, &args.args, interval);

    println!("Done, saving report");
    save_report(&report);
}

fn save_report(report: &profile::ProfileReport) {
    let file = File::create("report.json").unwrap();
    serde_json::to_writer_pretty(file, &report).unwrap();
}

fn parse_args(cmd: clap::Command) -> Result<Args, &'static str> {
    let m = cmd.get_matches();
    let interval = m
        .get_one::<String>("interval_ms")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let command = match m.get_one::<String>("command") {
        Some(cmd) => cmd.to_owned(),
        None => {
            return Err("No command specified");
        }
    };
    let warmup = match m.get_one::<String>("warmup_ms") {
        Some(w) => w.parse::<u32>().unwrap(),
        None => 0,
    };
    let cmdargs: Vec<String> = match m.get_many::<String>("args") {
        Some(args) => args.map(|x| x.to_owned()).collect(),
        None => Vec::new(),
    };
    Ok(Args {
        interval: interval,
        warmup: warmup,
        command: command,
        args: cmdargs,
    })
}
