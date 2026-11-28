use std::{collections::HashMap, iter::Map};

// fn check_anagram(a: &String, b: &String) -> bool {
//     let mut freq = [0; 26];

//     for (a, b) in a.bytes().zip(b.bytes()) {
//         freq[(a - b'a') as usize] += 1;
//         freq[(b - b'a') as usize] -= 1;
//     }

//     freq.iter().all(|&x| x == 0)
// }

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    if strs.is_empty() {
        return vec![vec!["".to_string()]];
    }
    
    let mut anagrams: Vec<Vec<String>> = Vec::new();
    let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

    
    for s in strs {
        let mut char_bytes: Vec<i32> = s.bytes().map(|n| n as i32).collect();
        char_bytes.sort();

        map.entry(char_bytes).or_insert(Vec::new()).push(s);
    }
    
    for (_k, v) in map {
        anagrams.push(v);
    }
    
    anagrams
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
