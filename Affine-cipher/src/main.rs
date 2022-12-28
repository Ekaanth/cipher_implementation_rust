    // Affine cipher: has two keys a and b 
    // we can represent encryption as a*X+b mod 26
    // we can represent decryption as a(inverse)*(y-b) mod 26

fn affine_cipher(text: &str, a: u8, b: u8) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() {
                'a' as u8
            } else {
                'A' as u8
            };

            let x = c as u8 - base;
            let y = ((a as u32 * x as u32 + b as u32) % 26) as u8;
            result.push((y + base) as char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let original = "Hello, World!";
    let encrypted = affine_cipher(original, 5, 8);
    let decrypted = affine_cipher(&encrypted, 21, 18);

    println!("Original message: {}", original);
    println!("Encrypted message: {}", encrypted);
    println!("Decrypted message: {}", decrypted);
}
