use std::fs::File;

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
    save_report(&report);
}

fn save_report(report: &profile::ProfileReport) {
    let file = File::create("report.json").unwrap();
    serde_json::to_writer_pretty(file, &report).unwrap();
}
