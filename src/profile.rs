use serde::Serialize;

pub mod info;

#[derive(Serialize)]
pub struct ProfileReport {
    system_info: info::SystemInfo,
    cmd_name: String,
    cmd_args: Vec<String>,
}

pub fn profile(cmd: &String, args: &[String]) -> ProfileReport {
    let sys_info = info::get_system_info();

    let mut cmd_process = std::process::Command::new(cmd).args(args).spawn().unwrap();

    cmd_process.wait().unwrap();

    ProfileReport {
        system_info: sys_info,
        cmd_name: cmd.clone(),
        cmd_args: args.to_vec(),
    }
}
