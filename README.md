# 🦀 Todo CLI

A simple, fast, and persistent command-line todo application built in Rust.

This project was created to learn Rust by building a real-world CLI application with persistent storage, modular architecture, and file-based data management.

## ✨ Features

- Add new tasks
- List all tasks
- Mark tasks as completed
- Delete tasks
- Persistent storage using JSON
- Lightweight and fast
- Simple command-line interface

---

## 📦 Project Structure

```text
src/
├── main.rs      # Entry point
├── cli.rs       # Command parsing
├── app.rs       # Business logic
├── task.rs      # Task model
└── storage.rs   # JSON file storage

Cargo.toml
README.md
```

---

## 🏗 Architecture

```text
CLI Commands
      │
      ▼
Command Parser
      │
      ▼
Application Logic
      │
      ▼
Storage Layer
      │
      ▼
tasks.json
```

---

## 🚀 Installation

### Prerequisites

Make sure Rust is installed:

```bash
rustc --version
cargo --version
```

If Rust is not installed:

https://www.rust-lang.org/tools/install

---

### Clone the Repository

```bash
git clone https://github.com/krishnakasaudhan0/todo-cli.git
cd todo-cli
```

---

### Run in Development Mode

```bash
cargo run -- add "Learn Rust"
cargo run -- list
```

---

### Build Release Version

```bash
cargo build --release
```

Run the binary:

```bash
./target/release/todo add "Learn Rust"
./target/release/todo list
```

---

### Install Globally

Install the CLI on your system:

```bash
cargo install --path .
```

Verify installation:

```bash
which todo
```

Example output:

```text
/Users/your-username/.cargo/bin/todo
```

After installation you can run:

```bash
todo add "Learn Rust"
todo list
```

from any directory.

---

## 📖 Usage

### Add a Task

```bash
todo add "Learn Rust"
```

### List Tasks

```bash
todo list
```

Example output:

```text
1 [✘] Learn Rust
2 [✔] Build Todo CLI
```

### Mark a Task as Done

```bash
todo done 1
```

### Delete a Task

```bash
todo delete 1
```

---

## 💾 Storage

Tasks are stored locally in a JSON file.

Example:

```json
[
  {
    "id": 1,
    "title": "Learn Rust",
    "done": false
  },
  {
    "id": 2,
    "title": "Build Todo CLI",
    "done": true
  }
]
```

This allows tasks to persist between program executions.

---

## 🧠 Concepts Learned

This project helped practice:

- Rust ownership and borrowing
- Structs and enums
- Modules and project organization
- File I/O
- JSON serialization and deserialization
- Error handling
- CLI application architecture
- Persistent data storage

---

## 🔮 Future Improvements

- Colored terminal output
- Task priorities
- Task timestamps
- Search and filtering
- Configuration file support
- Export/import functionality
- Full Clap integration
- Automated tests

---

## 👨‍💻 Author

Krishna Kasaudhan

GitHub: https://github.com/krishnakasaudhan0

---

## 📄 License

This project is licensed under the MIT License.
