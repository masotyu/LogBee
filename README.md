<h1 align="center">LogBee 🐝</h1>

![](screenshot.gif)

README: [日本語](./README_JP.md)

## 🚀 What is LogBee?

LogBee is a TUI (Terminal User Interface) tool designed for viewing JSON log files.
Powered by **DuckDB**, it allows you to instantly filter and sort through tens of thousands of logs using standard SQL syntax (`WHERE` clauses).

## ✨ Key Features

- 🔍 **SQL-like Filtering**: Extract data under complex conditions using standard `WHERE` clause syntax.

## 📦 Installation

### Windows

1. Download the latest `LogBee-setup.exe` from the [Releases](https://github.com/masotyu/LogBee/releases) page.
2. Run the installer and follow the instructions.

### Ubuntu / Debian (Linux)

1. Download the latest `.deb` package from the [Releases](https://github.com/masotyu/LogBee/releases) page.
2. Install it via terminal:

```bash
sudo dpkg -i logbee_*.deb
```

## 📖 Usage

### 1. Launching LogBee

Simply specify the JSON log file you want to view as an argument:

```bash
logbee your-log-file.json
```

## ⌨️ Keybindings

| Key | Action |
|:---:|:---|
| <kbd>i</kbd> | **Search Mode**: Enter a SQL `WHERE` clause (e.g., `level > 30`) |
| <kbd>Enter</kbd> | **Toggle Focus**: Switch between Log List and Detail View (yellow border) |
| <kbd>j</kbd> / <kbd>k</kbd> | **Move / Scroll**: Navigate the list or scroll vertically in the Detail View |
| <kbd>Tab</kbd> | **Cycle Sort Column**: Toggle between timestamp, level, message, etc. |
| <kbd>s</kbd> | **Toggle Sort Order**: Switch between Ascending (▲) and Descending (▼) |
| <kbd>n</kbd> / <kbd>p</kbd> | **Pagination**: Move to Next page / Previous page (loops at ends) |
| <kbd>Esc</kbd> | **Cancel**: Exit search mode or return to list from Detail View |
| <kbd>q</kbd> | **Quit**: Exit the application |

## 🔍 Query Examples (Query Area)

You can use DuckDB syntax directly:

```sql
-- Filter by level (e.g., WARN and above)
level >= 40

-- Search for specific text in messages
msg LIKE '%timeout%'

-- Filter by a specific host
hostname = 'prod-api-01'