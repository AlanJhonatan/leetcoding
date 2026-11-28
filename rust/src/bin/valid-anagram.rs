
pub fn is_anagram(s: String, mut t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    // racecar
    for s in s.chars() {
        // carrace
        let idx: i32 = t.find(s).map(|p| p as i32).unwrap_or(-1);

        if idx == -1 {
            return false
        }

        t.remove(idx as usize);
    }

    if t.len() > 0 {
        return false
    }
    
    true
}

fn main() {
    let s: String = String::from("x");
    let t: String = String::from("x");

    let output: bool = is_anagram(s, t);

    println!("{}", output);
}