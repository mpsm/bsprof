pub struct Args {
    pub interval: std::time::Duration,
    pub warmup: std::time::Duration,
    pub cooldown: std::time::Duration,
    pub jobs: Option<u32>,
    pub sequence: bool,
    pub target: String,
    pub clean_target: String,
    pub command: String,
    pub args: Vec<String>,
}

impl Args {
    pub fn parse_from_cmdline() -> Result<Args, &'static str> {
        let cmd_line_args = std::env::args().collect::<Vec<String>>();
        let cmd = clap::Command::new("Build System Profiler")
            .author("Marcin Smoczyński, smoczynski.marcin@gmail.com")
            .about("A simple tool for profiling build systems")
            .version(clap::crate_version!())
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
                    .default_value("0")
                    .help("Cooldown time in ms"),
            )
            .arg(
                clap::Arg::new("jobs")
                    .short('j')
                    .long("jobs")
                    .help("Number of jobs"),
            )
            .arg(
                clap::Arg::new("sequence")
                    .short('s')
                    .long("sequence")
                    .num_args(0)
                    .help("Profile build system with increasing number of jobs"),
            )
            .arg(
                clap::Arg::new("target")
                    .short('t')
                    .long("target")
                    .required(false)
                    .default_value("all")
                    .help("Target to build"),
            )
            .arg(
                clap::Arg::new("clean_target")
                    .short('C')
                    .long("clean-target")
                    .required(false)
                    .default_value("clean")
                    .help("Target to clean"),
            )
            .arg(
                clap::Arg::new("command")
                    .required(true)
                    .help("Build command"),
            )
            .arg(
                clap::Arg::new("args")
                    .num_args(0..)
                    .help("Additional build arguments"),
            );
        parse_args(cmd, &cmd_line_args)
    }

    pub fn print(&self) {
        println!("Profiling command:   {}", self.command);
        println!("Profiling args:      {:?}", self.args);
        println!("Profiling warmup:    {} ms", self.warmup.as_millis());
        println!("Profiling cooldown:  {} ms", self.cooldown.as_millis());
        println!(
            "Profiling sequence:  {}",
            if self.sequence { "yes" } else { "no" }
        );
        if let Some(jobs) = self.jobs {
            println!("Profiling jobs:      {}", jobs);
        }
        println!("Profiling interval:  {} ms", self.interval.as_millis());
    }

    pub fn create_sequence(&self) -> Vec<u32> {
        let jobs = self.jobs.unwrap_or(super::profile::info::get_cpu_count());
        if self.sequence {
            (1..=jobs).collect()
        } else {
            vec![jobs]
        }
    }
}

fn parse_args(cmd: clap::Command, cmd_line_args: &Vec<String>) -> Result<Args, &'static str> {
    let m = cmd.get_matches_from(cmd_line_args);

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

    let mut jobs = None;
    if let Some(jobs_option) = m.get_one::<String>("jobs") {
        let jobs_parsed = jobs_option.parse::<u32>();
        if jobs_parsed.is_err() || jobs_parsed.clone().unwrap() == 0 {
            return Err("Invalid jobs value");
        }
        jobs = Some(jobs_parsed.unwrap());
    }

    let mut sequence = false;
    if let Some(sequnce_option) = m.get_one::<bool>("sequence") {
        sequence = *sequnce_option;
    }

    let cmdargs: Vec<String> = match m.get_many::<String>("args") {
        Some(args) => args.map(|x| x.to_owned()).collect(),
        None => Vec::new(),
    };

    let target = m.get_one::<String>("target").unwrap().to_owned();
    let clean_target = m.get_one::<String>("clean_target").unwrap().to_owned();

    Ok(Args {
        interval: std::time::Duration::from_millis(interval as u64),
        warmup: std::time::Duration::from_millis(warmup as u64),
        cooldown: std::time::Duration::from_millis(cooldown as u64),
        target: target,
        clean_target: clean_target,
        sequence: sequence,
        jobs: jobs,
        command: command,
        args: cmdargs,
    })
}
