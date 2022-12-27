fn main() {

    let plain_text = String::from("TFFINE CIPHER");

    let a = 17;

    let b = 20;

    // Affine cipher: has two keys a and b 
    // we can represent encryption as a*X+b mod 26
    // we can represent decryption as a(inverse)*(y-b) mod 26

    let mut encryption = String::new();

    let chars:Vec<char> = plain_text.chars().collect(); 

    for element in chars.iter() {
        println!("{}",element);

        if element.eq_ignore_ascii_case(&' ') {
            encryption.push(*element);
        }else {

            let base = if element.is_ascii_lowercase() {
                *element as u8
            } else {
                *element as u8
            };

            let enc = (((a * (base - 'A' as u8) + b) % 26) + 'A' as u8) as char;
            encryption.push(enc);
        }
    }

    println!("Plaintext is {}", plain_text);
    println!("Encrypted String is {}", encryption);
}
