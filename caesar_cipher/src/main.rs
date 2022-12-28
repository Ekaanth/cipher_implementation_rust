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

fn main() {
    let original = "Hello, World!";
    let encrypted = caesar_cipher(original, 3);
    let decrypted = caesar_cipher(&encrypted, 23);

    println!("Original message: {}", original);
    println!("Encrypted message: {}", encrypted);
    println!("Decrypted message: {}", decrypted);
}
