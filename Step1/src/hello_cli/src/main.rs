use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("名前を入力してください");
        return;
    }

    let mut names: String = "".to_string();
    for i in 1..args.len() {
        if i > 1{
            names += " and "; 
        }
        names += &args[i];
    }
    println!("Hello, {}!", names);
}
