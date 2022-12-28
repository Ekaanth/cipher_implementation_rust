fn main() {

    let plain_text = String::from("AFFINE CIPHER");

    let a = 17;

    let b = 20;

    // Affine cipher: has two keys a and b 
    // we can represent encryption as a*X+b mod 26
    // we can represent decryption as a(inverse)*(y-b) mod 26

    let mut encryption = String::new();

    let chars = plain_text.chars(); 

    for element in chars {
        println!("{}",element);

        if element.eq_ignore_ascii_case(&' ') {
            encryption.push(element);
        }else {

            let base = if element.is_ascii_lowercase() {
                element as u32
            } else {
                element as u32
            };

            let enc = (((a as u32 * (base - 'A' as u32) as u32 + b as u32) % 26) as u8 + 'A' as u8) as char;
            encryption.push(enc);
        }
    }

    println!("Plaintext is {}", plain_text);
    println!("Encrypted String is {}", encryption);
}
