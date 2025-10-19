use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "hello_cli")]
#[command(version = "0.0.100")]
#[command(about = "CLI tool to greet", long_about = None)]
struct Cli {
    /// greet name(s)
    #[arg(required = true)]
    name: Vec<String>,

    /// print greeting in uppercase
    #[arg(long)]
    shout: bool,
    /// how many times to greet
    #[arg(long,default_value_t = 1)]
    times: u8,
    /// say goodbye instead of hello
    #[arg(long)]
    bye: bool,
    /// Language for greeting (e.g., en, ja, fr)
    #[arg(long, default_value = "en")]
    lang: String,
}

fn greet(name: &str, shout: bool, bye: bool) {
    let mut message = if bye {
        format!("Goodbye, {}!", name)
    } else {
        format!("Hello, {}!", name)
    };

    if shout {
        message = message.to_uppercase();
    }

    println!("{}", message);
}

fn main() {
    let cli = Cli::parse();
    for name in &cli.name {
        for _ in 0..cli.times {
            greet(name, cli.shout, cli.bye);
        }
    }
}