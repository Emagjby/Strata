use std::fs;
use strata::parser::parse;
use strata::encode::encode_value;
use strata::hash::hash_value;

fn main() {
    let st = fs::read_to_string("../vectors/v1/01-basic.st").unwrap();
    let val = parse(&st).unwrap();

    let bytes = encode_value(&val);
    let hash = hash_value(&val);

    println!("SCB HEX:");
    for b in &bytes {
        print!("{:02x}", b);
    }
    println!();

    println!("HASH HEX:");
    for b in &hash {
        print!("{:02x}", b);
    }
    println!();
}
