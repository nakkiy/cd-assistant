# cd-assistant

> 「fzf は便利だけど、**階層を見ながら cd したい**」  
> 「ファイラもいいけど、**cd さえできればいい**」

![demo](../demo.gif)

---

## 🚀 What is this?

`cda` は、ターミナル上でディレクトリ階層を視覚的にたどりながら、最短操作で `cd` するための **超軽量 cd 支援ツール**です。  
`cd` コマンドを出力するだけです。 **実行はしません。**

- カーソルキー / vim風操作でディレクトリ移動  
- フォーカス中ディレクトリに `cd` するコマンドを出力  
- 機能は `cd` のみ！最速・最短の導線設計  

---

## ✨ Features

### ✅ ディレクトリツリー表示

- `/` から現在ディレクトリまでを自動展開  
- 残りは折りたたみ表示（`▶` / `▼`）  
- vim風 or カーソル操作で階層ナビゲート  

### ✅ キーバインド（vim風 + カーソルキー）

| キー                   | 操作内容                              |
|------------------------|---------------------------------------|
| ↑ / `ctrl + k`        | 上に移動                              |
| ↓ / `ctrl + j`        | 下に移動                              |
| → / `ctrl + l`        | 展開（1階層のみ動的読み込み）         |
| ← / `ctrl + h`        | 折りたたみ or 親ディレクトリへ戻る    |
| `ctrl + f`             | ファイル一覧ポップアップの開閉        |
| `Enter`                | cdコマンドを出力して終了              |
| `Esc`                  | ポップアップを閉じる                  |
| `ctrl + q`             | 終了（何も出力せず終了）              |
| 英数字キー (e.g.`w`)   | 該当するディレクトリ名の先頭一致ジャンプ |

### ✅ ファイル一覧ポップアップ（`ctrl + f`）

- フォーカス中ディレクトリ内の **ファイルのみ** 表示  
- カラーハイライト付き（通常 / 実行可能 / シンボリックリンク / 壊れたリンク）  
- 表示形式：
  ```
  <ファイル名> [-> リンク先] <サイズ> <更新日時> <パーミッション>
  ```

---

## 📦 インストール

### 手動ビルド（Rust必須）
```sh
git clone --depth 1 https://github.com/nakkiy/cd-assistant.git ~/.cda
cd ~/.cda
cargo install --path .
```

### `.bashrc`に追加
```sh
# bash/zsh の場合（Alt+f で起動）
export CDA_BINDKEY='\ef'
source ~/.cda/shell/cda.bash
```

---

## 📦 アンインストール

### 手動削除
```sh
cd ~/.cda
cargo uninstall --path .
cd ~
rm -rf ~/.cda
```

### `.bashrc`から削除
```sh
# インストール時に追加した下記行を削除
export CDA_BINDKEY='\ef'
source ~/.cda/shell/cda.bash
```

---

## 📦 実行

```sh
$ cda
```

または（Alt キーバインド使用時）：  
👉 `Alt + f`

---

## 📄 ライセンス
このプロジェクトは、次のいずれかのライセンスの下で利用できます：

- [MIT License](LICENSE-MIT) または https://opensource.org/licenses/MIT  
- [Apache License 2.0](LICENSE-APACHE) または https://www.apache.org/licenses/LICENSE-2.0  

