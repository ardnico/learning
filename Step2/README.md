# **Step 2: clapで本格CLI**

## ✅ Step 2: clap導入

### 1. Cargo.tomlに依存関係を追加

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

---

### 2. 基本的なCLI定義

`src/main.rs` を以下のようにします：

```rust
use clap::Parser;

/// 簡単な挨拶CLI
#[derive(Parser, Debug)]
#[command(name = "hello_cli")]
#[command(about = "挨拶をするCLIツール", long_about = None)]
struct Cli {
    /// 挨拶する相手の名前
    name: String,

    /// 大文字で出力するかどうか
    #[arg(long)]
    shout: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut message = format!("Hello, {}!", cli.name);

    if cli.shout {
        message = message.to_uppercase();
    }

    println!("{}", message);
}
```

---

### 3. 実行例

```bash
cargo run -- Alice
```

👉 出力:

```
Hello, Alice!
```

```bash
cargo run -- Alice --shout
```

👉 出力:

```
HELLO, ALICE!
```

---

## ✅ 発展課題 (Step2)

1. **複数人対応**

   * `names: Vec<String>` を引数で受け取って `join(" and ")` でつなげる
   * `cargo run -- Alice Bob Charlie` → `Hello, Alice and Bob and Charlie!`

2. **オプションの追加**

   * `--times <N>` を追加して、同じ挨拶をN回繰り返す
   * `cargo run -- Alice --times 3`

     ```
     Hello, Alice!
     Hello, Alice!
     Hello, Alice!
     ```

3. **サブコマンド導入**

   * `greet` と `bye` を作る
   * `cargo run -- greet Alice` → `Hello, Alice!`
   * `cargo run -- bye Alice` → `Goodbye, Alice!`

---

## ✅ Step2で学べること

* `derive` を使った宣言的なCLI引数定義
* 複数引数・フラグ・オプションの扱い
* サブコマンドを使った実用的なCLI構造

---

※本内容はChatGPT出力のものです
