use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for n in nums {
        *map.entry(n).or_default() += 1;
    }

    let mut order_vec: Vec<(i32, i32)> = Vec::new();

    // here its important the destructuring from js to understand this tuple destructuring
    // and casting, to avoid the '*' dereference usage, could only used the map |&(x,y)| 
    for (k0, v0) in map.iter().map(|(k0,v0)| (*k0 as i32, *v0 as i32)) {
        order_vec.push((k0, v0));
    }

    order_vec.sort_by(|(_k1, v1), (_k2, v2)| v2.cmp(v1));

    let sol: Vec<i32> = order_vec[..k as usize].iter().map(|&(k3, _)| k3).collect();

    sol
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 2, 2, 3, 3, 3, 3];
    let k: i32 = 2;

    let output = top_k_frequent(nums, k);

    println!("{:#?}", output);
}