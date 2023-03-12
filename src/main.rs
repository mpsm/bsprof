use std::fs::File;

mod profile;

struct Args {
    interval_ms: u32,
    command: String,
    args: Vec<String>,
}

fn parse_args(_cmd: &clap::Command) -> Args {
    Args {
        interval_ms: 0,
        command: String::new(),
        args: Vec::<String>::new(),
    }
}

fn main() {
    let mut cmd = clap::Command::new("Build System Profiler")
        .author("Marcin Smoczyński, smoczynski.marcin@gmail.com")
        .about("A simple tool for profiling build systems")
        .arg(
            clap::Arg::new("interval_ms")
                .short('i')
                .long("interval")
                .default_value("1000"),
        )
        .trailing_var_arg(true);

    cmd.print_long_help().unwrap();

    let args = parse_args(&cmd);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let args = Args::parse_from(&["bsprof", "ls"]);
        assert_eq!(args.interval_ms, 1000);
        assert_eq!(args.command, "ls");
        assert_eq!(args.args, Vec::<String>::new());
    }

    #[test]
    fn test_command_argumets() {
        let args = Args::parse_from(&["bsprof", "ls", "-l", "-a"]);
        assert_eq!(args.interval_ms, 1000);
        assert_eq!(args.command, "ls");
        assert_eq!(args.args, vec!["-l".to_string(), "-a".to_string()]);
    }
}
