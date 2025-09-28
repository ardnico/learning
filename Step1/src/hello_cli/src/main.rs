use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("名前を入力してください");
        return;
    }

    let name = &args[1];
    println!("Hello, {}!", name);
}
