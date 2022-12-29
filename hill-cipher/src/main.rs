use ndarray::{Array2, Array};

fn hill_cipher(message : &String, key: &String) {
    println!("{}", message);
    println!("{}", key);
    let keys_numeric: Vec<i32> = key.chars().map(|c| c as i32 - 'A' as i32).collect();
    let message_numeric: Vec<i32> = message.chars().map(|c| c as i32 - 'A' as i32).collect();
    println!("{:?}", keys_numeric);
    println!("{:?}", message_numeric);

    let mut key_blocks = Vec::new();
    for i in (0..keys_numeric.len()).step_by(message_numeric.len()) {
        key_blocks.push(&keys_numeric[i..i+message_numeric.len()]);
    }

    let mut message_blocks = Vec::new();
    for i in (0..message_numeric.len()).step_by(message_numeric.len()) {
        message_blocks.push(&message_numeric[i..i+message_numeric.len()]);
    }
    
    let mut key_matrix: Array2<i32> = Array::zeros((message_numeric.len(), message_numeric.len()));
    let mut message_matrix: Array2<i32> = Array::zeros((message_numeric.len(), 1));

    for i in 0..key_blocks.len() {
            for j in 0..key_blocks[i].len() {
                // println!("{}", key_blocks[i][j]);
                key_matrix[(i, j)] = key_blocks[i][j];
            }
    }

    println!("{:?}", key_matrix.dot(&key_matrix));

}



fn main() {
    let message = String::from("ACT");
    let key = String::from("GYBNQKURP");
    hill_cipher(&message, &key);
}
