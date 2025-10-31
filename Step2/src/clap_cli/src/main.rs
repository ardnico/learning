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

fn convert_greeting(greeting: &str, lang: &str) -> String {
    match lang {
        "ja" => match greeting {
            "Hello" => "こんにちは".to_string(),
            "Goodbye" => "さようなら".to_string(),
            _ => greeting.to_string(),
        },
        "fr" => match greeting {
            "Hello" => "Bonjour".to_string(),
            "Goodbye" => "Au revoir".to_string(),
            _ => greeting.to_string(),
        },
        "de" => match greeting {
            "Hello" => "Hallo".to_string(),
            "Goodbye" => "Auf Wiedersehen".to_string(),
            _ => greeting.to_string(),
        },
        "es" => match greeting {
            "Hello" => "Hola".to_string(),
            "Goodbye" => "Adiós".to_string(),
            _ => greeting.to_string(),
        },
        "cn" => match greeting {
            "Hello" => "你好".to_string(),
            "Goodbye" => "再见".to_string(),
            _ => greeting.to_string(),
        },
        "it" => match greeting {
            "Hello" => "Ciao".to_string(),
            "Goodbye" => "Arrivederci".to_string(),
            _ => greeting.to_string(),
        },
        "ru" => match greeting {
            "Hello" => "Здравствуйте".to_string(),
            "Goodbye" => "До свидания".to_string(),
            _ => greeting.to_string(),
        },
        _ => greeting.to_string(), // default to English
    }
}

fn greet(name: &str, shout: bool, bye: bool, lang: &str) {
    let base_message = if bye {
        "Goodbye"
    } else {
        "Hello"
    };
    let mut message = format!("{}, {}!",convert_greeting(base_message, lang), name);
    if shout {
        message = message.to_uppercase();
    }

    println!("{}", message);
}

fn main() {
    let cli = Cli::parse();
    for name in &cli.name {
        for _ in 0..cli.times {
            greet(name, cli.shout, cli.bye, &cli.lang);
        }
    }
}