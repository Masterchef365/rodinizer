use std::fs;
use rodinizer2::*;

fn main() {
    let full_text = fs::read_to_string("./data/yob1999.txt").expect("Failed to read data");
    let mut rng = rand::thread_rng();
    let mut rodinizer = Rodinizer::new();
    for line in full_text.lines() {
        let mut columns = line.split(',');
        let name = columns.next().expect("Column has no name");
        println!("{}", rodinizer.rodinize(name, &mut rng).unwrap());
    }
}
