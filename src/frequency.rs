const LETTER_FREQUENCY: [f64; 26] = [
    0.08167, // A
    0.01492, // B
    0.02782, // C
    0.04253, // D
    0.12702, // E
    0.02228, // F
    0.02015, // G
    0.06094, // H
    0.06966, // I
    0.00153, // J
    0.00772, // K
    0.04025, // L
    0.02406, // M
    0.06749, // N
    0.07507, // O
    0.01929, // P
    0.00095, // Q
    0.05987, // R
    0.06327, // S
    0.09056, // T
    0.02758, // U
    0.00978, // V
    0.02360, // W
    0.00150, // X
    0.01974, // Y
    0.00074  // Z
];

pub fn distance_from_english(text: &str) -> f64 {
    let freq = char_frequency(text);
    euclidean_distance(&LETTER_FREQUENCY, &freq)
}

fn char_frequency(text: &str) -> [f64; 26] {
    let mut counts: [u64; 26] = [0; 26];
    text.to_ascii_lowercase()
        .bytes()
        .for_each(|x| {
            let i = (x - 97) as usize;
            counts[i] = counts[i] + 1;
        });

    let mut freq: [f64; 26] = [0.0; 26];
    counts.iter()
        .enumerate()
        .for_each(|(i, &c)| freq[i] = (c as f64) / (text.len() as f64));

    freq
}

fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    assert_eq!(vec1.len(), vec2.len(), "Vectors must be of the same length");

    let sum_of_squares: f64 = vec1.iter()
        .zip(vec2.iter())
        .map(|(&x, &y)| (x - y).powi(2))
        .sum();

    sum_of_squares.sqrt()
}