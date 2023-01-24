use std::{env, process};


#[derive(Debug)]
struct Config {
    plaintext: String,
    keyword: String
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let plaintext = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a plaintext string"),
        };

        let keyword = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a keyword string"),
        };

        Ok(Config{plaintext, keyword})
    }
}

fn keyword_cipher_encryption(plaintext: &String, keyword: &String) -> String {
    let mut ciphertext = String::new();

    for (i, ch) in plaintext.chars().enumerate() {
        let shift = keyword.chars().nth(i % keyword.len()).unwrap().to_ascii_lowercase() as u8 - 'a' as u8;
        if ch.is_ascii_lowercase() {
            let c = ((ch as u8 - 'a' as u8 + shift) % 26 + 'a' as u8) as char;
            ciphertext.push(c);
        } else {
            ciphertext.push(ch);
        }
    }

    ciphertext
}

fn keyword_cipher_decryption(cipher_test: &String, keyword: String) -> String{
    let result = String::new();



    result
}


fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    println!("{:?}", config);

    let cipher_text = keyword_cipher_encryption(&config.plaintext, &config.keyword);

    println!("Encrypted message is: {}", cipher_text);

    let plain_text = keyword_cipher_decryption(&cipher_text, config.keyword);

    println!("Decrypted message is: {}", plain_text);

}
