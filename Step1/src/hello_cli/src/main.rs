use std::env;

fn main() {
    let mut shout_flag: i32 = 0;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: 名前を入力してください");
        std::process::exit(1);
    }
    if args.contains(&"--shout".to_string()){
        shout_flag = 1;
    }

    let mut names: String = "".to_string();
    for i in 1..args.len() {
        if &args[i] == "--shout" {
            continue;
        }
        if i > 1{
            names += " and "; 
        }
        if shout_flag == 1 {
            names += &args[i].to_uppercase();
        } else {
            names += &args[i];
        }
    }
    println!("Hello, {}!", names);
}
