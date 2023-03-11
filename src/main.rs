fn main() {
    println!("Build system profiler application");
    // run application specified in process arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No application specified");
        return;
    }

    let app = &args[1];
    let mut cmd = std::process::Command::new(app)
        .args(&args[2..])
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
