use std::collections::HashMap;

fn find_first_unique(s: &str) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    
    for c in s.chars() {
        * map.entry(c).or_insert(0) += 1;
    }
    
    for (i, c) in s.chars().enumerate() {
        if map[&c] == 1 {
            return i as i32;
        }
    }
    
    return -1
}

fn main () {
    let s: String = String::from("loveleetcode");
    let idx: i32 = find_first_unique(&s);

    println!("index: {}", idx);
}