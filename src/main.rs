use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let n: i32 = args[1].parse().unwrap();
        for _ in 0..n {
            println!("");
        }
    }
}
