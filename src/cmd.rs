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
}
