
// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut left: usize = 0;
//     let mut right: usize = left + 1;

//     while left < nums.len() - 1 {
//         if nums[left] + nums[right] == target {
//             return vec![left as i32, right as i32];
//         }
        
//         if right == nums.len() - 1 {
//             left += 1;
//             right = left + 1;
//         } else {
//             right += 1;
//         }
//     }

//     return vec![0, 0];
// }

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut visited: HashMap<i32, i32> = HashMap::new();

    for (idx, n) in nums.iter().copied().enumerate() {
        let to_find: i32 = target - n;

        let target_idx: i32 = visited.get(&to_find).map(|p| *p as i32).unwrap_or(-1);
        
        if target_idx > -1 {
            return vec![target_idx, idx as i32];
        }

        visited.insert(n, idx as i32);
    }
    
    return vec![0, 0];
}

fn main () {
    let nums: Vec<i32> = vec![4,5,6];
    let target: i32 = 10;

    let solution: Vec<i32> = two_sum(nums, target);
    println!("{:#?}", solution);
}