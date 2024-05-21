#[derive(Debug, Default, Clone)]
pub struct Candidate {
    pub text: String,
    pub code: String,
    pub comment: Option<String>,
    pub weight: u32,
}
