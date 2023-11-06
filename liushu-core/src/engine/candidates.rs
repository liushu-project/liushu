#[derive(Debug, Default)]
pub struct Candidate {
    pub text: String,
    pub code: String,
    pub comment: Option<String>,
    pub weight: u64,
}
