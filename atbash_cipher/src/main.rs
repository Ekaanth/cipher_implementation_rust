use std::{env, process};

fn atbash_cipher_encryption(plaintext: &String) -> String {
    let result = String::new();
    let lookup_table: [i32; 26] = [0..26;26];
    for c in 0..plaintext.len() {
        println!("{}", plaintext.chars().nth(c).unwrap());
        let char = plaintext.chars().nth(c).unwrap();
        let base = if char.is_ascii_uppercase() {
            'A' as u8
        }else if char.is_ascii_lowercase() {
            'a' as u8
        }else{ 
            return char.to_string();
        };
        println!("{}", base);

        let enc = ((char as u8 - base) % 26);
        println!("{:?}", lookup_table)
    }

    result
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please enter plaintext");
        process::exit(1)
    }

    let plaintext = &args[1];

    let encryption = atbash_cipher_encryption(&plaintext);
}
