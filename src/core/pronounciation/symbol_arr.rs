use super::prelude::Symbol;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Default,
    serde:: Deserialize,
    serde::Serialize,
)]
pub struct SymbolArr(u64);

impl SymbolArr {
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl FromIterator<Symbol> for SymbolArr {
    fn from_iter<T: IntoIterator<Item = Symbol>>(iter: T) -> Self {
        let mut c: u64 = 0;
        let mut m = 0;
        for x in iter {
            c = c | ((x as u64) << m);
            m = m + 8;
        }
        Self(c)
    }
}

pub struct SymbolArrIter(u64);

impl Iterator for SymbolArrIter {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.0 & 0x000000ff;
        self.0 = self.0 >> 8;

        Symbol::from_repr(n as u8)
    }
}

impl IntoIterator for SymbolArr {
    type Item = Symbol;

    type IntoIter = SymbolArrIter;

    fn into_iter(self) -> Self::IntoIter {
        SymbolArrIter(self.0)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use itertools::*;
    use ntest::test_case;

    use super::*;

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    #[test_case(4)]
    #[test_case(5)]
    #[test_case(6)]
    #[test_case(7)]
    #[test_case(8)]
    fn test_symbol_arr(len: usize) {
        let vec = vec![
            Symbol::AA,
            Symbol::B,
            Symbol::CH,
            Symbol::D,
            Symbol::EH,
            Symbol::F,
            Symbol::G,
            Symbol::HH,
        ];

        let symbol_arr: SymbolArr = vec.iter().take(len).cloned().collect();
        let expected = vec.iter().take(len).cloned().collect_vec();
        let actual = symbol_arr.into_iter().collect_vec();

        assert_eq!(expected, actual)
    }
}
