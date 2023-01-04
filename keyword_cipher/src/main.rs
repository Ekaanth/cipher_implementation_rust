
fn keyword_cipher(plaintext: &String, keyword: &String) -> String {
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


fn main() {
    let plain_text = String::from("Zombie Here");
    let keyword = String::from("secret");

    let cipher_text = keyword_cipher(&plain_text, &keyword);

    println!("Encrypted message is: {}", cipher_text);
}
