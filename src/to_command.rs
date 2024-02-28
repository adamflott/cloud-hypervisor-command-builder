pub trait ToCommand {
    fn to_command(&self) -> Vec<String>;

    fn to_single_command(&self) -> String {
        self.to_command().join(" ")
    }
}
