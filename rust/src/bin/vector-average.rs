
fn main() {
    let vec: Vec<i32> = vec![10, 5];
    let mut sum: i32 = 0;

    for x in &vec {
        sum += x;
    }

    let avg: f64 = (sum as f64) / (vec.len() as f64);

    println!("{avg}");
}