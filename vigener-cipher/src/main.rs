use std::{env, process};


#[derive(Debug)]
struct Config {
    plaintext: String,
    key: String
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let plaintext = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a plaintext string"),
        };

        let key = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a key string"),
        };

        Ok(Config{plaintext, key})
    }
}

fn vigener_cipher_encrypt(config: &Config) -> String {
    let mut key = config.key.clone();
    let plaintext = config.plaintext.clone();
    let mut encrypt = String::new();

    if key.len() < plaintext.len() {
        key = key.chars().cycle().take(plaintext.len()).collect::<String>();
    }

    for c in 0..plaintext.len() {
        let res = plaintext.as_bytes()[c] + key.as_bytes()[c] % 26;
        encrypt.push(res as char)
    }
   encrypt
}

fn vigener_cipher_decrypt(decrypt: String, key: String) -> String {
    let mut key = key;
    let mut plaintext = String::new();

    if key.len() < decrypt.len() {
        key = key.chars().cycle().take(decrypt.len()).collect::<String>();
    }

    for c in 0..decrypt.len() {
        let res = decrypt.as_bytes()[c] - key.as_bytes()[c] % 26;
        plaintext.push(res as char)
    }
    plaintext
}

fn main() {
     let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    println!("{:?}", config);

    let encrypt = vigener_cipher_encrypt(&config);

    println!("encrypted string is : {}", encrypt);

    let decrypt = vigener_cipher_decrypt(encrypt, config.key);

    println!("Decrypted string is : {}", decrypt);
}
