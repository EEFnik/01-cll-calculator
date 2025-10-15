# 🧮 CLI Calculator with Expression Parser

A powerful command-line calculator built with Rust, featuring advanced expression parsing using the **Shunting Yard Algorithm**. This calculator supports operator precedence, parentheses, and maintains a persistent calculation history.

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)

---

## ✨ Features

### Core Functionality

- ✅ **Basic Operations**: Addition, subtraction, multiplication, division
- ✅ **Advanced Operations**: Power (^), modulo (%), square root (s)
- ✅ **Expression Parsing**: Full mathematical expression support
- ✅ **Operator Precedence**: Respects mathematical order of operations
- ✅ **Parentheses Support**: Handles nested parentheses correctly
- ✅ **Calculation History**: Track all your calculations in memory
- ✅ **File Persistence**: Save and load history from `history.txt`
- ✅ **Error Handling**: Graceful handling of invalid input and edge cases

### User Experience

- 🎨 **Colorful Output**: Color-coded results, errors, and messages
- 🚀 **Interactive Commands**: history, clear, save, last, help
- 💾 **Auto-save**: History automatically saved on exit
- 📝 **Smart Tokenization**: Works with or without spaces

---

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or higher

### Installation

```bash
# Clone the repository
git clone https://github.com/EEFnik/01-cll-calculator.git
cd 01-cll-calculator

# Build the project
cargo build --release

# Run the calculator
cargo run --release
```

---

## 📖 Usage

### Basic Calculations

```bash
> 5 + 3
= 8

> 10 - 4
= 6

> 3 * 7
= 21

> 15 / 3
= 5
```

### Advanced Operations

```bash
> 2 ^ 3
= 8

> 10 % 3
= 1

> 9 s 0
= 3
```

### Operator Precedence

The calculator correctly follows mathematical order of operations:

```bash
> 5 + 3 * 2
= 11          # (3 * 2) is calculated first, then + 5

> 10 / 2 + 3
= 8           # (10 / 2) is calculated first, then + 3

> 2 ^ 3 + 1
= 9           # (2 ^ 3) is calculated first, then + 1
```

### Parentheses

Use parentheses to change the order of operations:

```bash
> (5 + 3) * 2
= 16          # (5 + 3) is calculated first, then * 2

> 2 * (3 + 4)
= 14

> ((2 + 3) * 4) - 1
= 19
```

### Complex Expressions

Mix everything together:

```bash
> 2 ^ 3 * (5 + 3) / 4
= 16

> (10 - 2) / (3 - 1)
= 4

> 5+3*2
= 11          # Works without spaces!
```

---

## 🎮 Commands

| Command             | Description                       | Example       |
| ------------------- | --------------------------------- | ------------- |
| `<expression>`    | Calculate mathematical expression | `5 + 3 * 2` |
| `history`         | Show all calculation history      | `history`   |
| `last`            | Show the last calculation         | `last`      |
| `clear`           | Clear calculation history         | `clear`     |
| `save`            | Save history to file              | `save`      |
| `help`            | Show available commands           | `help`      |
| `exit` / `quit` | Exit calculator                   | `exit`      |

### Operators

| Operator | Operation      | Priority | Example              |
| -------- | -------------- | -------- | -------------------- |
| `+`    | Addition       | 1        | `5 + 3 = 8`        |
| `-`    | Subtraction    | 1        | `10 - 4 = 6`       |
| `*`    | Multiplication | 2        | `3 * 7 = 21`       |
| `/`    | Division       | 2        | `15 / 3 = 5`       |
| `%`    | Modulo         | 2        | `10 % 3 = 1`       |
| `^`    | Power          | 3        | `2 ^ 3 = 8`        |
| `s`    | Square Root    | 4        | `9 s 0 = 3`        |
| `()`   | Parentheses    | -        | `(5 + 3) * 2 = 16` |

---

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_precedence_expression
```

### Test Coverage

The project includes 19 unit tests covering:

- ✅ Basic operations
- ✅ Error cases (division by zero, invalid input)
- ✅ Operator precedence
- ✅ Parentheses handling
- ✅ Complex expressions
- ✅ File I/O operations
- ✅ History management

---

## 🏗️ Project Structure

```
cli-calculator/
├── Cargo.toml              # Dependencies and project config
├── README.md               # This file
├── history.txt             # Calculation history (auto-generated)
└── src/
    └── main.rs             # Main application (~400 lines)
```

### Key Functions

1. **`evaluate_expression()`** - Main parser using Shunting Yard Algorithm
2. **`tokenize()`** - Converts input string into tokens
3. **`apply_operator()`** - Applies operators to operands
4. **`calculate()`** - Performs individual operations
5. **`precedence()`** - Returns operator precedence level
6. **`save_history()` / `load_history()`** - File persistence

---

## 🛠️ Technologies & Concepts

### Rust Concepts

- **Ownership & Borrowing**: Proper memory management without garbage collection
- **Result<T, E>**: Comprehensive error handling
- **Traits**: Custom `Display` implementation for `HistoryEntry`
- **Pattern Matching**: `match` expressions for control flow
- **Iterators**: Functional programming patterns

### Algorithms

- **Shunting Yard Algorithm**: Expression parsing with operator precedence
- **Stack-based Evaluation**: Using two stacks (numbers and operators)
- **Tokenization**: Converting string input to processable tokens

### External Crates

- **colored (2.0)**: Terminal color output

---

## 📚 Algorithm Explanation

### Shunting Yard Algorithm

This calculator uses the **Shunting Yard Algorithm** (invented by Edsger Dijkstra) to parse mathematical expressions:

#### How it works:

1. **Tokenization**: Break input into numbers, operators, and parentheses
2. **Two Stacks**: Maintain separate stacks for numbers and operators
3. **Operator Precedence**: Apply operators based on their priority
4. **Parentheses Handling**: Use `(` and `)` to control evaluation order

#### Example: `5 + 3 * 2`

```
Step 1: Push 5 to numbers stack
Step 2: Push + to operators stack
Step 3: Push 3 to numbers stack
Step 4: * has higher precedence than +, push * to operators
Step 5: Push 2 to numbers stack
Step 6: Apply * first (3 * 2 = 6)
Step 7: Apply + (5 + 6 = 11)
Result: 11
```

---

## 🎓 Learning Outcomes

This project demonstrates:

- ✅ **Rust Fundamentals**: Ownership, borrowing, and lifetimes
- ✅ **Error Handling**: Proper use of `Result` and `Option` types
- ✅ **Data Structures**: Working with `Vec`, `String`, and custom structs
- ✅ **Algorithm Implementation**: Shunting Yard algorithm from scratch
- ✅ **File I/O**: Reading and writing to files
- ✅ **Testing**: Comprehensive unit test coverage
- ✅ **User Interface**: Creating an interactive CLI application
- ✅ **Code Organization**: Clean, modular function design

---

## 🐛 Error Handling

The calculator gracefully handles various error cases:

### Division by Zero

```bash
> 10 / 0
Error: Division by zero
```

### Invalid Input

```bash
> 5 +
Error: Missing operand

> abc + 3
Error: Invalid number: abc
```

### Invalid Characters

```bash
> 5 @ 3
Error: invalid char '@'
```

### Mismatched Parentheses

```bash
> (5 + 3
Error: Missing operand
```

---

## 🚀 Future Enhancements

Potential improvements for future versions:

- [ ] Variables support (`x = 5`, then use `x` in expressions)
- [ ] Function support (`sin`, `cos`, `log`, etc.)
- [ ] Command history with arrow keys
- [ ] Multiple history files
- [ ] Export history to different formats (CSV, JSON)
- [ ] Scientific notation support
- [ ] Undo/redo functionality
- [ ] Configuration file for customization

---

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📝 Development

### Code Style

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run all checks
cargo fmt && cargo clippy && cargo test
```

### Adding New Features

To add a new operator:

1. Add the operator to `precedence()` function
2. Implement logic in `calculate()` function
3. Update `is_operator()` to recognize it
4. Add to `tokenize()` if special handling needed
5. Write tests for the new operator

## 🙏 Acknowledgments

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) by Edsger Dijkstra
- Rust community for excellent documentation and support

---

## 📊 Project Stats

- **Language**: Rust
- **Lines of Code**: ~400
- **Functions**: 12
- **Tests**: 19
- **Dependencies**: 1 (colored)
- **Development Time**: ~10 hours
- **Complexity**: Intermediate

---

Built with 🦀 and ❤️ while learning Rust

**⭐ Star this repo if you find it helpful!**
