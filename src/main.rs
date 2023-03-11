mod profile;

fn main() {
    println!("Build system profiler application");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No application specified");
        return;
    }

    let sys_info = profile::info::get_system_info();
    sys_info.print();

    let cmd_name = &args[1];
    let cmd_args = &args[2..];
    if cmd_args.len() > 0 {
        println!("Running command: {} {:?}", cmd_name, cmd_args);
    } else {
        println!("Running command: {}", cmd_name);
    }

    let report = profile::profile(cmd_name, cmd_args);

    println!("Done, saving report");

    let report_json = serde_json::to_string_pretty(&report).unwrap();
    println!("{}", report_json);
}
