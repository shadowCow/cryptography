use std::f64::MAX;

use crate::frequency;

const ALPHABET: [char; 26] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
];

pub fn decipher_shift(text: &str, scorer: fn(&str) -> f64) -> String {
    (0..26).fold((MAX, text.to_string()), |(s, t), n| {
        let d = decipher_shift_n(text, n);
        let score = scorer(&d);
        if score < s {
            (score, d)
        } else {
            (s, t)
        }
    }).1
}

pub fn encipher_shift_n(text: &str, n: u8) -> String {
    text.to_ascii_lowercase()
        .chars()
        .filter(|x| x.is_ascii_lowercase())
        .map(|x| ALPHABET[((x as u8 - 97 + n) % 26) as usize])
        .collect()
}

pub fn decipher_shift_n(text: &str, n: u8) -> String {
    text.to_ascii_lowercase()
        .chars()
        .filter(|x| x.is_ascii_lowercase())
        .map(|x| ALPHABET[to_index(x as i32, n as i32)])
        .collect()
}

fn to_index(x: i32, n: i32) -> usize {
    let i = x - 97 - n;
    let index = if i < 0 {
        26 + i
    } else {
        i
    };
    index as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encipher_1() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected = "bcdefghijklmnopqrstuvwxyza";

        let actual = encipher_shift_n(input, 1);

        assert_eq!(actual, expected.to_string());
    }

    #[test]
    fn test_encipher_25() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected = "zabcdefghijklmnopqrstuvwxy";

        let actual = encipher_shift_n(input, 25);

        assert_eq!(actual, expected.to_string());
    }

    #[test]
    fn test_encipher_0() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected = "abcdefghijklmnopqrstuvwxyz";

        let actual = encipher_shift_n(input, 0);

        assert_eq!(actual, expected.to_string());
    }

    #[test]
    fn test_encipher_26() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected = "abcdefghijklmnopqrstuvwxyz";

        let actual = encipher_shift_n(input, 26);

        assert_eq!(actual, expected.to_string());
    }

    #[test]
    fn test_decipher_5() {
        let input = "fghijklmnopqrstuvwxyzabcde";
        let expected = "abcdefghijklmnopqrstuvwxyz";

        let actual = decipher_shift_n(input, 5);

        assert_eq!(actual, expected.to_string());
    }
}