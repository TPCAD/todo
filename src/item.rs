#[derive(Debug)]
pub struct Todo<'a> {
    content: &'a str,
    date: String,
}

impl<'a> Todo<'a> {
    pub fn new(content: &'a str, date: String) -> Self {
        Todo { content, date }
    }

    pub fn to_record(&self) -> String {
        format!("{} {}", self.date, self.content)
    }
}
