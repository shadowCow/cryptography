mod substitution;
mod frequency;

fn main() {
    let text = "The quick brown fox jumped over the lazy dog";

    let enciphered = substitution::encipher_shift_n(text, 11);

    substitution::decipher_shift(&enciphered, |t| {
        println!("{}", t);
        println!("");
        0.0
    });
}
