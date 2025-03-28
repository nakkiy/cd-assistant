# cd-assistant

[🇯🇵 日本語版README](docs/ja/README.md)  

> "fzf is great, but sometimes I want to **see the directory tree**."  
> "File managers are powerful, but I really just want to **cd quickly**."

![demo](docs/demo.gif)

---

## 🚀 What is this?

`cda` is a **super lightweight terminal navigator** designed to help you `cd` into directories as quickly and visually as possible.  
It simply prints a `cd` command. **It does NOT execute it.**

- Navigate the directory tree with arrow keys or vim-like keybindings  
- Output a `cd` command for the currently focused directory  
- Designed for speed and simplicity – just `cd`, nothing more

---

## ✨ Features

### ✅ Directory Tree View

- Automatically expands from `/` to your current directory on startup  
- Everything else stays collapsed (`▶` / `▼` indicators)  
- Navigate the hierarchy visually with arrow keys or vim keys  

### ✅ Keybindings (vim-style + arrow keys)

| Key                    | Action                                    |
|------------------------|-------------------------------------------|
| ↑ / `Ctrl + k`         | Move up                                   |
| ↓ / `Ctrl + j`         | Move down                                 |
| → / `Ctrl + l`         | Expand (dynamically load one level)       |
| ← / `Ctrl + h`         | Collapse or move to parent directory      |
| `Ctrl + f`             | Toggle file list popup                    |
| `Enter`                | Output `cd` command and exit              |
| `Esc`                  | Close popup                               |
| `Ctrl + q`             | Quit without output                       |
| Alphanumeric key (e.g. `w`) | Jump to directory matching the starting letter |

### ✅ File List Popup (`Ctrl + f`)

- Shows **files only** in the focused directory  
- Color-coded:
  - White: regular files  
  - Green: executable  
  - Cyan: symlink (valid)  
  - Red: broken symlink  
- Display format:
  ```
  <filename> [-> symlink target] <size> <last modified> <permissions>
  ```

---

## 📦 Installation

### Manual build (requires Rust)
```sh
git clone --depth 1 https://github.com/nakkiy/cd-assistant.git ~/.cda
cd ~/.cda
cargo install --path .
```

### Add to `.bashrc` or `.zshrc`
```sh
# For bash/zsh (launch with Alt+f)
export CDA_BINDKEY='\ef'
source ~/.cda/shell/cda.bash
```

---

## 📦 Uninstallation

### Manual removal
```sh
cd ~/.cda
cargo uninstall --path .
cd ~
rm -rf ~/.cda
```

### Remove from `.bashrc`
```sh
# Remove these lines if you added them:
export CDA_BINDKEY='\ef'
source ~/.cda/shell/cda.bash
```

---

## 📦 Usage

```sh
$ cda
```

Or (if bound to a key):  
👉 `Alt + f`

---

## 📄 License

This project is dual licensed under either:

- [MIT License](LICENSE-MIT) — https://opensource.org/licenses/MIT  
- [Apache License 2.0](LICENSE-APACHE) — https://www.apache.org/licenses/LICENSE-2.0  
