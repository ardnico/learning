use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Useage: hello_cli <name>");
        return;
    }

    let name = &args[1];
    println!("Hello, {}!", name);
}
