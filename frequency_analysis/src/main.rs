use std::{env, process};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut map: HashMap<char, u64> = HashMap::new(); 
    
    if args.len() <= 1 {
            println!("Please enter plaintext");
            process::exit(1)
        }

    let plaintext = &args[1];

    for ch in plaintext.chars().into_iter() {
        if map.get(&ch).is_some() {
            *map.get_mut(&ch).unwrap() += 1;
        }else {
            map.insert(ch, 1);
        }
    }

    println!("Frequency Analysis is : {:?}", map);
}
