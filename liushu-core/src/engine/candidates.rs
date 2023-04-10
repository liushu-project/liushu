#[derive(Debug, Default)]
pub struct Candidate {
    pub text: String,
    pub code: String,
    pub comment: Option<String>,
    pub weight: u64,
    pub source: CandidateSource,
}

#[derive(Debug)]
pub enum CandidateSource {
    Hmm,
    CodeTable,
}

impl Default for CandidateSource {
    fn default() -> Self {
        Self::CodeTable
    }
}
