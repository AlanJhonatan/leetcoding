fn main() {
    let s: String = String::from("hellow world!");

    println!("{} you!", &s[0..5]);
}