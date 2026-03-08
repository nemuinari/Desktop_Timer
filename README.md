# Desktop_Timer

# Concept.
```lua
-- ターミナルから呼び出せるタイマー(GUI)
-- $ rtm

-- ショートカットで操作可能
-- start = [s]
-- stop = [s]
-- reset = [r]
-- close = [ESC]
```

- development: Rust
- Release: wix/GitHub Releases/MyWebsite

## 事前準備 (環境構築)
- Windows用インストーラー (.msi) を作成するためのツールを導入
- WiX Toolset: v3.11.2 等をインストール
- ターミナルで cargo install cargo-wix を実行

## アイコンの用意
- アプリの顔となるアイコンを icons/ フォルダに配置
- 形式: .ico (Windows) / .icns (Mac)
- 512x512 の正方形PNGから変換して作成

## Cargo.toml の設定
- [package] セクションに配布に必要なメタデータを追記

```toml
# Cargo.toml

[package]
name = "desktop_timer"
version = "0.1.0"
authors = ["Your Name <email@example.com>"]
description = "Rust & iced 製の軽量ストップウォッチ"

```

## インストーラーの生成

```bash
// wix/main.wxs が生成
// target/wix/ フォルダ内に .msi ファイル(インストーラー）が生成
$ cargo wix init
$ cargo wix

```
