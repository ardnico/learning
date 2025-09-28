use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("名前を入力してください");
        return;
    }

    for i in 1..args.len() {
        let name = &args[i];
        println!("Hello, {}!", name);
    }
}
