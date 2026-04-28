# 💸 expense-tracker

A CLI tool to log, list, and summarize daily expenses — built in Rust without a database or web server.

Data is persisted locally in a `expenses.json` file. No setup required beyond `cargo build`.

---

## Features

- Add expenses with amount, category, and a note
- List all expenses or filter by category
- Summarize total spending per category
- Delete an expense by ID
- Auto-increments IDs, auto-stamps today's date

---

## Installation

```bash
git clone https://github.com/yourusername/expense-tracker
cd expense-tracker
cargo build --release
```

Optionally move the binary to your PATH:

```bash
cp target/release/expense-tracker /usr/local/bin/expense
```

---

## Usage

> During development, replace `expense` with `cargo run --`

### Add an expense

```bash
expense add --amount 250 --category food --note "lunch"
```

### List all expenses

```bash
expense list
```

### List by category

```bash
expense list travel
```

### Summary by category

```bash
expense summary
```

```
Category    Amount
food:       1250.00
travel:     3400.00
transport:  800.00
```

### Delete an expense

```bash
expense delete 3
```

---

## Project structure

```
expense-tracker/
├── src/
│   ├── main.rs        # CLI setup with Clap, routes subcommands
│   ├── models.rs      # Expense struct (Serialize, Deserialize, Debug)
│   ├── storage.rs     # load_expenses() and save_expenses()
│   └── commands.rs    # add, list, summary, delete — pure logic
├── expenses.json      # auto-created on first add
└── Cargo.toml
```

---

## Tech stack

| Crate        | Purpose                            |
|--------------|------------------------------------|
| `clap`       | CLI argument and subcommand parsing |
| `serde`      | Serialize / deserialize Expense    |
| `serde_json` | Read and write the JSON file       |
| `chrono`     | Auto-stamp today's date on add     |

---

## What I learned building this

- Structuring a Rust project across multiple modules
- Using `HashMap` and iterators (`filter`, `for_each`) on real data
- Persisting state with `serde_json` and `std::fs`
- Parsing typed CLI arguments with Clap (named flags + subcommands)
- Matching on `Option<String>` for optional filters

---

## Next

Building a **Task Manager REST API** using `axum`, `sqlx`, and `tokio` — same ideas, with async and a database layer added on top.
