use rand::seq::SliceRandom;
use rand::Rng;

const VOWELS: [char; 6] = ['A', 'E', 'I', 'O', 'U', 'Y'];
const CONSONANTS: [char; 20] = [
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Z',
];

pub fn rodinize(s: &str, rng: &mut impl Rng) -> String {
    let mut chars = s.chars();
    let first_letter = chars.next().unwrap();
    let mut name = String::with_capacity(s.len() + 1);
    if VOWELS.contains(&first_letter) {
        name.push(*CONSONANTS.choose(rng).unwrap());
        name.extend(first_letter.to_lowercase());
    } else {
        name.push(loop {
            let &letter = CONSONANTS.choose(rng).unwrap();
            if letter != first_letter {
                break letter;
            }
        });
    }
    name.extend(chars);
    name
}

pub fn read_names(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(path)?;
    Ok(file
        .lines()
        .map(|s| s.split(',').next().unwrap().to_string())
        .collect())
}
