use rand::{distributions::Alphanumeric, Rng};
use std::{env, process};


#[derive(Debug)]
struct Config {
    plaintext: String,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let plaintext = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a plaintext string"),
        };

        Ok(Config{plaintext})
    }
}

fn otp_encryption(plaintext: &String, key: &String) -> String {
    let mut result = String::new();
    let plaintext = plaintext.to_uppercase().rsplit(" ").collect::<String>();
    
    for i in 0..plaintext.len() {
        let plain_char = plaintext.chars().nth(i).unwrap() as u8 - 64;
        let key_char= key.chars().nth(i).unwrap() as u8 - 64;
        let res_char = ((((plain_char + key_char) - 1 as u8) % 26) + 64 ) as char;
        result.push(res_char);
    }
    result
}

fn otp_decryption(encrypted: &String, key: &String) -> String {
    let mut result = String::new();
    
    for i in 0..encrypted.len() {
        let enc_char = encrypted.chars().nth(i).unwrap() as u8 - 64;
        let key_char= key.chars().nth(i).unwrap() as u8 - 64;

        let mut diff = key_char - enc_char ;

        if diff.lt(&0) {
            diff = diff + 26
        }

        let res_char = (((diff + 1 as u8).rem_euclid(26)) + 64 ) as char;
        result.push(res_char);
    }
    result
}


fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let otp_key: String = rand::thread_rng().sample_iter(Alphanumeric)
    .map(char::from)
    .filter(|c| ('A'..='Z').contains(c))
    .take(config.plaintext.len())
    .collect();

    println!("The OTP key is : {:?}", otp_key);

   let encryption = otp_encryption(&config.plaintext, &otp_key);

    println!("Encrypted message is:{}", encryption);

    let decryption = otp_decryption(&encryption, &otp_key);

    println!("Decryption message is:{}", decryption);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_encryption_test() {
        let plaintext = "SECRETMESSAGE".to_owned();
        let key = "CIJTHUUHMLFRU".to_owned();

        assert_eq!("UMLKLNGLEDFXY", otp_encryption(&plaintext, &key));
    }

    #[test]
    fn case_decryption_test() {
        let encrypt = "UMLKLNGLEDFXY".to_owned();
        let key = "CIJTHUUHMLFRU".to_owned();

        assert_eq!("SECRETMESSAGE", otp_decryption(&encrypt, &key));
    }

}