pub trait Command {
    fn run(&self, args: &Vec<&str>, paths: &Vec<&str>);
}
