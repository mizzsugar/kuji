use rand::seq::SliceRandom;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let v: Vec<&str> = s.trim().split_whitespace().collect();
    println!("{:?}", v.choose(&mut rand::thread_rng()).unwrap());
}
