use ndarray::{Array2, Array};

fn get_key_matrix(key: &String, key_matrix: &mut Array2<i32>){
    let mut k = 0;
    for i in 0..3 {
        for j in 0..3 {
            key_matrix[(i, j)] = (key.as_bytes()[k] as i32) % 65;
            k+=1;
        }
    }
}


fn hill_cipher_encryption(message : &String, key: &String) -> String{

    let mut key_matrix: Array2<i32> = Array::zeros((message.len(), message.len()));
    
    // create an key matrix
    get_key_matrix(key, &mut key_matrix);

    // create an empty message matrix
    let mut message_matrix: Array2<i32> = Array::zeros((message.len(), 1));

    for i in 0..3 {
        message_matrix[(i, 0)] = message.as_bytes()[i] as i32 % 65;
    }

    // Dot vector of 3 matrix 3*3 and 3*1 matrix
    let mut encrypt = key_matrix.dot(&message_matrix) % 26;
    encrypt = encrypt+65;

    let mut enc_string = String::new();

    for i in 0..encrypt.len() {
        enc_string.push(encrypt[(i,0)] as u8 as char);
    };

    enc_string
}

fn hill_cipher_decryption(encrypted: &String, key: &String) -> String {
    let mut plain_text = String::new();

    let mut key_matrix: Array2<i32> = Array::zeros((encrypted.len(), encrypted.len()));
    
    // create an key matrix
    get_key_matrix(key, &mut key_matrix);

    key_matrix

    plain_text
}

fn main() {
    let message = String::from("ACT");
    let key = String::from("GYBNQKURP");

    let encrypted_message = hill_cipher_encryption(&message, &key);

    println!("The encrypted message for the text {} with the key of {} is: {}", message, key, encrypted_message);

    let decrypted_message = hill_cipher_decryption(&encrypted_message, &key);
}
