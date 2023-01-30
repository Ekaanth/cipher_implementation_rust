use std::{env, process};
use std::collections::HashMap;

fn atbash_cipher_encryption_decryption(plaintext: &String, map: &HashMap<char, char>) -> String {
    let mut result = String::new();
    for c in 0..plaintext.len() {
        let char = plaintext.to_uppercase().chars().nth(c).unwrap();
        match map.get(&char) {
            Some(value) => result.push(*value),
            None => println!("Key not found"),
        };
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut map = HashMap::new(); 
    map.insert('A', 'Z');
    map.insert('B', 'Y');
    map.insert('C', 'X');
    map.insert('D', 'W');
    map.insert('E', 'V');
    map.insert('F', 'U');
    map.insert('G', 'T');
    map.insert('H', 'S');
    map.insert('I', 'R');
    map.insert('J', 'Q');
    map.insert('K', 'P');
    map.insert('L', 'O');
    map.insert('M', 'N');
    map.insert('N', 'M');
    map.insert('O', 'L');
    map.insert('P', 'K');
    map.insert('Q', 'J');
    map.insert('R', 'I');
    map.insert('S', 'H');
    map.insert('T', 'G');
    map.insert('U', 'F');
    map.insert('V', 'E');
    map.insert('W', 'D');
    map.insert('X', 'C');
    map.insert('Y', 'B');
    map.insert('Z', 'A');

    if args.len() <= 1 {
        println!("Please enter plaintext");
        process::exit(1)
    }

    let plaintext = &args[1];

    println!("The text to be encrypted is :: {}", plaintext);

    let encryption = atbash_cipher_encryption_decryption(&plaintext, &map);

    println!("Plaintext encryption is :{}", encryption);

    let decryption = atbash_cipher_encryption_decryption(&encryption, &map);

    println!("Plaintext decryption is :{}", decryption)
}
