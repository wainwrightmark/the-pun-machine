
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Pun{
    new_phrase: String,
    old_phrase: String
}

impl Pun{
    pub fn is_new_different(&self)-> bool{
        self.new_phrase != self.old_phrase
    }
}