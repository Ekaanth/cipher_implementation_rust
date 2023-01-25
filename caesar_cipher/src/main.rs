use std::{env, process};


#[derive(Debug)]
struct Config {
    plaintext: String,
    shift: u64
}

impl Config{

    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let plaintext = match args.next() {
            Some(args) => args,
            None => return Err("Please enter plaintext")
        };

        let shift= match args.next() {
            Some(args) => args.parse().expect("Failed to convert string into u64"),
            None => return Err("Please enter the shift number")   
        };  

        Ok(Config { plaintext, shift})
    }
}


fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            let base = if c.is_ascii_lowercase() {
                'a' as u8
            } else if c.is_ascii_uppercase() {
                'A' as u8
            } else {
                return c;
            };

            let shifted = (c as u8 - base + shift) % 26 + base;
            shifted as char
        })
        .collect()
}

fn caesar_cipher_encrypt(text: &str, shift: u8) -> String {
   caesar_cipher(text, shift)
}

fn caesar_cipher_decrypt(text: &str, shift: u8) -> String {
    let inverse_key = 26 - shift % 26 as u8;
    caesar_cipher(text, inverse_key)
}
fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let encrypted = caesar_cipher_encrypt(&config.plaintext, config.shift as u8);
    let decrypted = caesar_cipher_decrypt(&encrypted, config.shift as u8);

    println!("Original message: {}", config.plaintext);
    println!("Encrypted message: {}", encrypted);
    println!("Decrypted message: {}", decrypted);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_encryption_test() {
        let plaintext = "This is a secret message";
        let shift = 27;

        assert_eq!("Uijt jt b tfdsfu nfttbhf", caesar_cipher_encrypt(plaintext, shift as u8));
    }

    #[test]
    fn case_decryption_test() {
        let plaintext = "Uijt jt b tfdsfu nfttbhf";
        let shift = 27;

        assert_eq!("This is a secret message", caesar_cipher_decrypt(plaintext, shift as u8));
    }

}