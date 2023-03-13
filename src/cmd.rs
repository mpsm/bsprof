#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(name: &String, args: &Vec<String>) -> Command {
        Command {
            name: name.to_owned(),
            args: args.to_owned(),
        }
    }

    pub fn add_jobs(&mut self, jobs: u32) {
        self.args.push("-j".to_string());
        self.args.push(jobs.to_string());
    }

    pub fn add_arg(&mut self, arg: &String) {
        self.args.push(arg.to_owned());
    }

    pub fn run(&self) -> Result<(), &'static str> {
        let mut cmd_process = std::process::Command::new(&self.name)
            .args(&self.args)
            .spawn()
            .unwrap();
        match cmd_process.wait() {
            Ok(_) => Ok(()),
            Err(_) => Err("Error running command"),
        }
    }
}
