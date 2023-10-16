use crate::letter::Letter;

#[repr(u8)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum EnglishLetter {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

impl Letter for EnglishLetter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_equality() {
        assert_eq!(EnglishLetter::A, EnglishLetter::A);
        assert_eq!(EnglishLetter::M, EnglishLetter::M);
        assert_eq!(EnglishLetter::Z, EnglishLetter::Z);
    }

    #[test]
    fn test_letter_inequality() {
        assert_ne!(EnglishLetter::A, EnglishLetter::M);
        assert_ne!(EnglishLetter::A, EnglishLetter::Z);
        assert_ne!(EnglishLetter::M, EnglishLetter::Z);
    }
}

