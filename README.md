<div align="center">
  
# blink

<br>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

</div>

A modern, intuitive file manager built in Rust that brings the speed of vim navigation to your file browsing experience. Navigate your filesystem with familiar keybindings, visual mode selections, and powerful file operations.

## ✨ Features

- **⚡ Lightning Fast Navigation** - Vim-like motions (j/k, g/G)
- **👁️ Live Preview** - See file contents and directory listings as you navigate
- **🎯 Visual Mode** - Select multiple files like in vim for batch operations
- **🔖 Bookmarks** - Tag and instantly jump to your favorite directories
- **📝 Editor Integration** - Open files in your $EDITOR (vim, nvim, nano, etc.)
- **🎨 Configurable** - Customize appearance, behavior and keybindings to your liking
- **🚀 Written in Rust** - Fast, safe, and reliable

## 📦 Installation

### Via Cargo (Recommended)

```bash
cargo install blink
```

### From Source

```bash
git clone https://github.com/Varcrow/blink.git
cd blink
cargo build --release
# Binary will be in target/release/blink
```

## 🎮 Usage

Launch blink from your terminal:

```bash
blink         # Opens blink at current directory
```

## ⌨️ Getting Started

### Navigation
| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Go to parent directory |
| `l` / `→` | Enter directory / Open file |
| `g` | Jump to top |
| `G` | Jump to bottom |

### File Operations
| Key | Action |
|-----|--------|
| `m` | Create new file or directory |
| `d` | Delete selected entry/entries |
| `r` | Rename current entry |
| `y` | Yank (copy) selection |
| `x` | Cut selection |
| `p` | Paste yanked/cut items |
| `u` | Undo last operation |

### Visual Mode
| Key | Action |
|-----|--------|
| `v` | Toggle visual mode |
| `j`/`k` | Extend selection up/down |
| `y` | Yank selected items |
| `x` | Cut selected items |
| `d` | Delete selected items |

### Bookmarks
| Key | Action |
|-----|--------|
| `B` | Create bookmark for current directory |
| `b` | Open bookmark list |
| `d` | Delete bookmark (in bookmark menu) |
| `enter` | Jump to bookmark (in bookmark menu) |

### Other
| Key | Action |
|-----|--------|
| `e` | Open file in $EDITOR |
| `o` | Open file in default application |
| `H` | Toggle hidden files |
| `q` | Quit blink |

### Creating Files vs Directories
When using `m` to make a new entry:
- **Without extension** → Creates a directory: `my-folder`
- **With extension** → Creates a file: `script.sh`

### Multi-file Operations
1. Press `v` to enter visual mode
2. Use `j`/`k` to select multiple files
3. Press `y` to copy or `x` to cut
4. Navigate to destination
5. Press `p` to paste

## Editor Integration

Set your preferred editor using environment variables:

**For bash** (~/.bashrc):
```bash
echo 'export EDITOR=nvim' >> ~/.bashrc
source ~/.bashrc
```
**For zsh** (~/.zshrc):
```bash
echo 'export EDITOR=nvim' >> ~/.zshrc
source ~/.zshrc
```

**For PowerShell**
```powershell
# add to PowerShell profile
notepad $PROFILE
# Add this line to the file:
$env:EDITOR = "nvim"
```

Blink automatically detects terminal editors (vim, nvim, nano, emacs, micro, helix) and handles them properly.

## ⚙️ Configuration

Blink stores its configuration and bookmarks in your system's config directory:
- **Linux/macOS**: `~/.config/blink/`
- **Windows**: `%APPDATA%\blink\`

## 🛠️ Requirements

- **Rust** 1.70+ (for building from source)
- **Nerd Font** (recommended for best icon display)

## 🙏 Acknowledgments

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
