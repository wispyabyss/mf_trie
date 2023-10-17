use crate::letter::Letter;

#[repr(u8)]
#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
pub enum EnglishLetter {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

impl Letter for EnglishLetter {}

impl EnglishLetter {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'A' | 'a' => Some(EnglishLetter::A),
            'B' | 'b' => Some(EnglishLetter::B),
            'C' | 'c' => Some(EnglishLetter::C),
            'D' | 'd' => Some(EnglishLetter::D),
            'E' | 'e' => Some(EnglishLetter::E),
            'F' | 'f' => Some(EnglishLetter::F),
            'G' | 'g' => Some(EnglishLetter::G),
            'H' | 'h' => Some(EnglishLetter::H),
            'I' | 'i' => Some(EnglishLetter::I),
            'J' | 'j' => Some(EnglishLetter::J),
            'K' | 'k' => Some(EnglishLetter::K),
            'L' | 'l' => Some(EnglishLetter::L),
            'M' | 'm' => Some(EnglishLetter::M),
            'N' | 'n' => Some(EnglishLetter::N),
            'O' | 'o' => Some(EnglishLetter::O),
            'P' | 'p' => Some(EnglishLetter::P),
            'Q' | 'q' => Some(EnglishLetter::Q),
            'R' | 'r' => Some(EnglishLetter::R),
            'S' | 's' => Some(EnglishLetter::S),
            'T' | 't' => Some(EnglishLetter::T),
            'U' | 'u' => Some(EnglishLetter::U),
            'V' | 'v' => Some(EnglishLetter::V),
            'W' | 'w' => Some(EnglishLetter::W),
            'X' | 'x' => Some(EnglishLetter::X),
            'Y' | 'y' => Some(EnglishLetter::Y),
            'Z' | 'z' => Some(EnglishLetter::Z),
            _ => None
        }
    }

    pub fn from_str(s: &str) -> Vec<Self> {
        s.chars()
            .filter_map(Self::from_char)
            .collect()
    }
}

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

    #[test]
    fn test_from_char_letter() {
        let x = EnglishLetter::from_char('X');
        assert_eq!(x, Some(EnglishLetter::X));
    }

    #[test]
    fn test_from_char_non_letter() {
        let x = EnglishLetter::from_char(' ');
        assert_eq!(x, None);
    }

    #[test]
    fn test_from_str_word() {
        let word = "MfTrIe";
        let expected_vec = vec![
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T,
            EnglishLetter::R,
            EnglishLetter::I,
            EnglishLetter::E
        ];
        assert_eq!(expected_vec, EnglishLetter::from_str(word));
    }

    #[test]
    fn test_from_str_sentence() {
        let word = "Mf TrIe";
        let expected_vec = vec![
            EnglishLetter::M,
            EnglishLetter::F,
            EnglishLetter::T,
            EnglishLetter::R,
            EnglishLetter::I,
            EnglishLetter::E
        ];
        assert_eq!(expected_vec, EnglishLetter::from_str(word));
    }
}

