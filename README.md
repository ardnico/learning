# learning

ChatGPT先生との学習記録です。
以下のロードマップで進めます。

---

## ✅ ステップアップ式ロードマップ（CLI開発しながらRust基礎習得）

### **Step 1: CLIの超基礎**

* **目標:** `cargo run hello` → "Hello, World!" と出す
* 学ぶこと: Cargo, main関数, `std::env::args()`
* ミニ課題: 引数を受け取って `Hello, {name}!` を出す

---

### **Step 2: clapで本格CLI**

* **目標:** 引数やオプションを扱えるツールを作る
* crate: [`clap`](https://crates.io/crates/clap)
* ミニ課題:

  * `mycli greet --name Alice` → "Hello, Alice!"
  * `mycli add 2 3` → "5"

---

### **Step 3: ファイルI/O**

* **目標:** ファイルを読み込んで加工するCLI
* ミニ課題:

  * `mycli upper file.txt` → 中身を大文字化して表示
  * `mycli count file.txt` → 行数や単語数を表示
* 学ぶこと: `std::fs::File`, バッファ読み込み, エラーハンドリング（`Result`）

---

### **Step 4: エラーハンドリング & モジュール分割**

* **目標:** 見やすいコードにする
* ミニ課題:

  * `anyhow`や`thiserror`でエラー処理を整える
  * srcを複数ファイルに分割する（`mod`, `lib.rs`）

---

### **Step 5: 並列処理 / 非同期**

* **目標:** Rustらしい高速処理に挑戦
* ミニ課題:

  * URLを複数叩いてステータスコードを返すツール
  * 並列処理（`rayon`） or 非同期処理（`tokio` + `reqwest`）

---

### **Step 6: ポートフォリオ化**

* Cargoでバイナリとして配布できるようにする
* READMEを書いてGitHubに公開
* 余力があれば `cargo install mycli` で入れられるように crates.io に公開

---
