use serde::Serialize;

mod info;

#[derive(Serialize)]
struct ProfileReport {
    system_info: info::SystemInfo,
    cmd_name: String,
    cmd_args: Vec<String>,
}

fn main() {
    println!("Build system profiler application");
    // run application specified in process arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No application specified");
        return;
    }

    let sys_info = info::get_system_info();
    sys_info.print();

    let cmd_name = &args[1];
    let cmd_args = &args[2..];
    if cmd_args.len() > 0 {
        println!("Running command: {} {:?}", cmd_name, cmd_args);
    } else {
        println!("Running command: {}", cmd_name);
    }

    let mut cmd = std::process::Command::new(cmd_name)
        .args(cmd_args)
        .spawn()
        .unwrap();

    cmd.wait().unwrap();

    println!("Done, saving report");

    let report = ProfileReport {
        system_info: sys_info,
        cmd_name: cmd_name.to_string(),
        cmd_args: cmd_args.to_vec(),
    };

    let report_json = serde_json::to_string_pretty(&report).unwrap();
    println!("{}", report_json);
}
