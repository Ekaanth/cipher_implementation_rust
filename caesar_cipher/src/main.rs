fn main() {
 
    let plain_text = String::from("ATTACKATONCE");

    let key = 4;

    let chars:Vec<char> = plain_text.chars().collect();

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
    
    println!("Plaintest is {}", plain_text);
    println!("Encrypted String is {}", encrypt);
}