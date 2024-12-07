#[derive(Clone, Debug)]
pub struct Task {
    pub name: String,
    pub line_number: usize,
    pub command: Option<String>,
}

