#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Casing {
    Lower,
    Upper,
    Title,
}

impl Casing {
    pub fn unify_captialization<S: AsRef<str> + Copy, O: AsRef<str>>(
        text: S,
        original_word: O,
    ) -> String {
        let original_casing = Casing::identify(original_word);
        if Casing::identify(text) == original_casing {
            return text.as_ref().to_string();
        }
        original_casing.convert(text)
    }

    ///Identify the casing of a string
    pub fn identify<S: AsRef<str>>(s: S) -> Self {
        if let Some(first) = s.as_ref().chars().next() {
            if first.is_lowercase() {
                return Casing::Lower;
            }

            if s.as_ref().chars().all(|x| x.is_uppercase()) {
                return Casing::Upper;
            }
        }

        Casing::Title
    }

    ///Converts a string to this casing
    pub fn convert<S: AsRef<str>>(self, s: S) -> String {
        match self {
            Casing::Lower => s.as_ref().to_ascii_lowercase(),
            Casing::Title => s
                .as_ref()
                .char_indices()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect(),
            Casing::Upper => s.as_ref().to_ascii_uppercase(),
        }
    }
}
