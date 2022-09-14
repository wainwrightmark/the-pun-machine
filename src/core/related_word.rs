#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct RelatedWord {
    pub word: &'static str,
    pub related_to: &'static str,
    pub reason: &'static str,
    pub meaning: &'static str,
}
