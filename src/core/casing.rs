#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Casing {
    Lower,
    Upper,
    Title,
}


impl Casing{

    pub fn unify_captialization(text : &String, original_word : &String )-> String{

        let original_casing = Casing::identify(original_word);
        if Casing::identify(&text) == original_casing{
            return text.clone();
        }
        return original_casing.convert(text);
    }

    ///Identify the casing of a string
    pub fn identify( s: &String )-> Self{

        if let Some(first) = s.chars().next(){
            if first.is_lowercase(){
                return Casing::Lower;
            }

            if s.chars().all(|x|x.is_uppercase()){
                return Casing::Upper;
            }
        }

        
        return Casing::Title;
    }

    ///Converts a string to this casing
    pub fn convert(self, s: &String)-> String{
        match self {
            Casing::Lower => s.to_ascii_lowercase(),
            Casing::Title => {

                s.char_indices().map(|(i,c)| if i == 0{c.to_ascii_uppercase()}else{c.to_ascii_lowercase()}).collect()
            },
            Casing::Upper => s.to_ascii_uppercase(),
        }
    }
 }