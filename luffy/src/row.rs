pub struct Row{
    string:String
}

impl From<&str> for Row{
    fn from(value: &str) -> Self {
        Self{
            string: String::from(value),
        }
    }
}