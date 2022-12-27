use std::io;

fn main() {
 
   let key = 14;

    println!("Please enter the text to be encrypted");
    let mut plain_text = String::new();
    io::stdin().read_line(&mut plain_text).expect("Failed to read line");

    let mut chars:Vec<char> = plain_text.chars().collect();
    chars.pop();
    let mut encrypt = String::new();
    for element in chars.iter() {
        let char = element;
        let base = if char.is_ascii_lowercase() {
            'a' as u8
        } else {
            'A' as u8
        };

        let shifted = ((*char as u8 + key  - base) % 26 + base) as char;
        encrypt.push(shifted);
    }
    
    println!("Plaintext is {}", plain_text);
    println!("Encrypted String is {}", encrypt);
}