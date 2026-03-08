# ⏱️ RTM (Rust Timer)

ターミナルから即座に起動し、キーボード操作のみで完結する軽量なデスクトップ・ストップウォッチ。

---

## 📋 利用者向けガイド (User Guide)

### インストール方法

- Releases から最新の rtm.msi をダウンロードします。
- インストーラーを実行して完了させると、自動的にシステムパス (Path) が通ります。

### 使い方

任意のターミナル（PowerShell, コマンドプロンプト, Git Bash等）を開き、以下のコマンドを入力するだけで起動します。

$ rtm

### ⌨️ キーボードショートカット

マウスを使わず、すべての操作をキーボードで行えます。

| キー    | アクション                                    |
| :------ | :-------------------------------------------- |
| [S]     | Start / Stop (タイマーの開始と停止を切り替え) |
| [R]     | Reset (時間を 00:00:00:00 にリセット)         |
| [Space] | Minimize (タスクバーへ最小化)                 |
| [Esc]   | Close (アプリを即座に終了)                    |

---

## 🛠️ 開発者向けガイド (Developer Guide)

### 1. 事前準備

- Rust: edition = "2021" 以降。
- WiX Toolset: v3.11.2 等をインストール。bin フォルダを環境変数の Path に追加してください。
- cargo-wix: cargo install cargo-wix で導入。

### 2. プロジェクト設定 (Cargo.toml)

[package]
name = "rtm"
version = "0.1.0"
edition = "2021"

[dependencies]
iced = { version = "0.10", features = ["async-std", "wgpu", "tokio"] }

[[bin]]
name = "rtm"
path = "src/main.rs"

### 3. ビルド実行

- アイコンの準備 (icons/icon.ico)
- WiX設定の初期化

```bash
$ cargo wix init --force
```

- MSI生成 (target/wix/ に出力)

```bash
$ cargo wix -v
```

---

## 📄 技術構成 (Tech Stack)

- Language: Rust
- GUI Framework: iced (v0.10)
- Installer Tool: WiX Toolset (v3)
- Design: Pixel Art Style
