use std::{collections::HashMap, iter::Map};

// fn check_anagram(a: &String, b: &String) -> bool {
//     let mut freq = [0; 26];

//     for (a, b) in a.bytes().zip(b.bytes()) {
//         freq[(a - b'a') as usize] += 1;
//         freq[(b - b'a') as usize] -= 1;
//     }

//     freq.iter().all(|&x| x == 0)
// }


// using this approach for char would be awesome in a utf-8 cenario.

// let mut char_bytes: Vec<i32> = s.bytes().map(|n| n as i32).collect();
// char_bytes.sort();

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    if strs.is_empty() {
        return vec![vec!["".to_string()]];
    }
    
    let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());
    
    for s in strs {
        let mut key = [0u8; 26];

        for b in s.bytes() {
            key[(b - b'a') as usize] += 1;
        }

        map.entry(key).or_default().push(s);
    }
    
    map.into_values().collect()
}

// println!("{:#?}", map);

fn main() {
    let strs: Vec<String> = vec!["act", "pots","tops","cat","stop","hat"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let output: Vec<Vec<String>> = group_anagrams(strs);

    println!("{:#?}", output);
}
