use std::fs::File;

mod profile;

struct Args {
    interval: std::time::Duration,
    warmup: std::time::Duration,
    cooldown: std::time::Duration,
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
    let warmup = m
        .get_one::<String>("warmup_ms")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let cooldown = m
        .get_one::<String>("cooldown_ms")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let cmdargs: Vec<String> = match m.get_many::<String>("args") {
        Some(args) => args.map(|x| x.to_owned()).collect(),
        None => Vec::new(),
    };
    Ok(Args {
        interval: std::time::Duration::from_millis(interval as u64),
        warmup: std::time::Duration::from_millis(warmup as u64),
        cooldown: std::time::Duration::from_millis(cooldown as u64),
        command: command,
        args: cmdargs,
    })
}
