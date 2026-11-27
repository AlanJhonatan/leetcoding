use std::collections::HashMap;

// VERSION 1.0
// found a lot of verbose, a lot of problems, but works at least kkkk
// the hashmap internal docs are messy and verbose to understand (im lazy today)

// fn get_frequency(nums: Vec<i32>) -> HashMap<i32, i32> {
//     let mut map: HashMap<i32, i32> = HashMap::new();

//     for n in nums {
//         if !map.contains_key(&n){
//             map.insert(n, 1);
//             continue;
//         }

//         let curr: &i32 = map.get(&n).unwrap_or(&0);
        
//         map.insert(n, curr + 1);
//     }

//     return map;
// }
// ==================================================================


// VERSION 2.0

// took a check on usefull methods from Hashmap

// let valor = map.get(&n).unwrap_or(&0);
// match map.get(&n) {
//     Some(valor) => println!("Achei: {}", valor),
//     None => println!("Não existe!"),
// }
// if let Some(valor) = map.get(&n) {
//     println!("O valor é {}", valor);
// }

// found out that shi is safe as f* to deal with
// this version works well, more semantic/meaningful, 
// but can be improved a lot yet

// 1 - borrowing the values from Vec<>
// fn get_frequency(v: &[i32]) -> HashMap<i32, i32> {
//     let mut map: HashMap<i32, i32> = HashMap::new();

//     for &n in v {
//         if let Some(&value) = map.get(&n) {
//             map.insert(n, value + 1);
//             continue;
//         }
        
//         map.insert(n, 1);
//     }

//     map
// }


// VERSION 3.0 - real, oficial, this time go

// learned more about .get() and .entry() diff and methods
fn get_frequency(v: &[i32]) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for &n in v {
        *map.entry(n).or_insert(0) += 1;
    }

    map
}


fn main() {
    let v: Vec<i32> = vec![2, 2, 4, 3, 6, 1, 2];

    let m: HashMap<i32, i32> = get_frequency(&v);

    println!("result: {:#?}", m);
}