pub struct Args {
    pub interval: std::time::Duration,
    pub warmup: std::time::Duration,
    pub cooldown: std::time::Duration,
    pub command: String,
    pub args: Vec<String>,
}

pub fn parse_args(cmd: clap::Command) -> Result<Args, &'static str> {
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
