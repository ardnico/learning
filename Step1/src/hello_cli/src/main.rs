use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = arg_chk(&args);
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn greet_some_persons(args: &Vec<String>, shout_flag: i32) -> () {
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
    println!("{}", greet(&names));
}

fn arg_chk(args: &Vec<String>) -> Result<(),()> {
    let mut shout_flag: i32 = 0;
    if args.len() < 2 {
        println!("Error: 名前を入力してください");
        return Err(());
    }
    if args.contains(&"--shout".to_string()){
        shout_flag = 1;
    }
    greet_some_persons(&args,shout_flag);
    Ok(())
}
