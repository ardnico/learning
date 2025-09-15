# **Step 1: CLIの超基礎**

---

## 🔹 Step 1: Hello, World CLI

### 1. 新しいプロジェクトを作る

```bash
cargo new hello_cli
cd hello_cli
```

これで以下の構成になります：

```
hello_cli/
 ├─ Cargo.toml
 └─ src/
     └─ main.rs
```

---

### 2. `main.rs` を編集

まずはシンプルに：

```rust
fn main() {
    println!("Hello, World!");
}
```

実行:

```bash
cargo run
```

👉 出力:

```
Hello, World!
```

---

### 3. 引数を受け取る

次は、コマンドライン引数を扱ってみます。

`src/main.rs` をこう変更：

```rust
use std::env;

fn main() {
    // 引数をベクタに収集
    let args: Vec<String> = env::args().collect();

    // args[0] は実行ファイルの名前
    if args.len() < 2 {
        println!("Usage: hello_cli <name>");
        return;
    }

    let name = &args[1];
    println!("Hello, {}!", name);
}
```

実行例:

```bash
cargo run Alice
```

👉 出力:

```
Hello, Alice!
```

---

### 4. ミニ課題 🎯

* `cargo run Bob` → `Hello, Bob!`
* 引数がなかったら「名前を入力してください」と表示するようにする

---

ここまでで
✅ Cargoプロジェクトの使い方
✅ `main`関数の書き方
✅ CLI引数の扱い方（`std::env::args`）
を学べました。

---

## 🔹 Step 1 発展課題

### 1. 複数の引数を処理する

```bash
cargo run Alice Bob Charlie
```

👉 出力例：

```
Hello, Alice!
Hello, Bob!
Hello, Charlie!
```

ヒント:

* `for arg in &args[1..] { ... }` で2個目以降の引数をまとめて処理できる

---

### 2. 引数の数で挙動を変える

* 引数が1個 → 名前を挨拶する
* 引数が0個 → 「名前を入力してください」と表示する
* 引数が複数 → 「みんなに挨拶」する

👉 実行例:

```bash
cargo run
```

```
名前を入力してください
```

```bash
cargo run Taro
```

```
Hello, Taro!
```

```bash
cargo run Taro Hanako
```

```
Hello, Taro and Hanako!
```

---

### 3. 簡単なオプションを自作する

`--shout` が指定されたら大文字で挨拶する

👉 実行例:

```bash
cargo run Alice
```

```
Hello, Alice!
```

```bash
cargo run Alice --shout
```

```
HELLO, ALICE!
```

ヒント:

* `args.contains(&"--shout".to_string())` で判定できる
* `.to_uppercase()` で大文字に変換できる

---

### 4. 簡単なエラーハンドリング

* 引数が空ならエラーメッセージを出して **終了コードを1にする**

👉 実行例:

```bash
cargo run
```

```
Error: 名前を入力してください
```

（LinuxやmacOSなら `echo $?` で終了コード確認できる）

ヒント:

```rust
std::process::exit(1);
```

---

### 5. 挨拶メッセージを関数に切り出す

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

👉 `main` から呼び出すようにすると、関数・所有権・借用の基礎が自然に学べる

---

## 🎯 ゴール

* 引数を自由に扱えるようになる
* 条件分岐 (`if`, `match`) が使えるようになる
* 文字列操作（大文字化、format!）ができる
* エラーハンドリングの基本 (`exit`) が分かる

---

※本内容はChatGPT出力のものです
