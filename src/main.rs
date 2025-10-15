use colored::*;
use core::fmt;
use std::{
    fs,
    io::{self, Write},
};

struct HistoryEntry {
    expression: String,
    result: f64,
}

impl HistoryEntry {
    fn new(expression: String, result: f64) -> Self {
        HistoryEntry { expression, result }
    }
}

impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.expression, self.result)
    }
}

fn save_history(history: &Vec<HistoryEntry>) -> Result<(), String> {
    let mut file =
        fs::File::create("history.txt").map_err(|e| format!("Cannot create file: {}", e))?;

    for entry in history {
        writeln!(file, "{}", entry).map_err(|e| format!("Write Error: {}", e))?;
    }

    Ok(())
}

fn load_history() -> Vec<HistoryEntry> {
    match fs::read_to_string("history.txt") {
        Ok(content) => content
            .lines()
            .filter_map(|line| {
                if let Some(pos) = line.find(" = ") {
                    let expression = line[..pos].to_string();
                    let result = line[pos + 3..].parse::<f64>().ok()?;
                    Some(HistoryEntry::new(expression, result))
                } else {
                    None
                }
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn precedence(op: char) -> u8 {
    match op {
        's' => 4,
        '^' => 3,
        '*' | '/' | '%' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

fn is_operator(token: &str) -> bool {
    token.len() == 1 && "+-*/^%s".contains(token)
}

fn is_number(token: &str) -> bool {
    token.parse::<f64>().is_ok()
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = input.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_ascii_digit() || ch == '.' {
            current.push(ch);
        } else if "+-*/^%()s".contains(ch) {
            if ch == '-' {
                let prev = if i > 0 { chars[i - 1] } else { ' ' };
                if i == 0 || "+-*/^%(s".contains(prev) {
                    current.push(ch);
                    continue;
                }
            }
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(ch.to_string());
        } else if ch.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            return vec![format!("Error: invalid char '{}'", ch)];
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn apply_operator(numbers: &mut Vec<f64>, operators: &mut Vec<char>) -> Result<(), String> {
    let op = operators.pop().ok_or("No operator")?;

    if op == 's' {
        let a = numbers.pop().ok_or("Missing opersand")?;
        let result = calculate(a, op, 0.0)?;
        numbers.push(result);
    } else {
        let b = numbers.pop().ok_or("Missing opersand")?;
        let a = numbers.pop().ok_or("Missing opersand")?;
        let result = calculate(a, op, b)?;
        numbers.push(result);
    }
    Ok(())
}

fn evaluate_expression(input: &str) -> Result<f64, String> {
    let tokens = tokenize(input);
    let mut numbers: Vec<f64> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for token in tokens {
        if is_number(&token) {
            let num = token
                .parse::<f64>()
                .map_err(|_| format!("Invalid number: {}", token))?;
            numbers.push(num);
        } else if is_operator(&token) {
            let op = token.chars().next().unwrap();
            while let Some(&top) = operators.last() {
                if top != '(' && precedence(top) >= precedence(op) {
                    apply_operator(&mut numbers, &mut operators)?;
                } else {
                    break;
                }
            }
            operators.push(op);
        } else if token == "(" {
            operators.push('(');
        } else if token == ")" {
            while let Some(&top) = operators.last() {
                if top == '(' {
                    operators.pop();
                    break;
                }
                apply_operator(&mut numbers, &mut operators)?;
            }
        } else if token.starts_with("Error:") {
            return Err(token);
        } else {
            return Err(format!("Unknown token: {}", token));
        }
    }
    while !operators.is_empty() {
        apply_operator(&mut numbers, &mut operators)?;
    }

    if numbers.len() == 1 {
        Ok(numbers[0])
    } else {
        Err("Error: Incorrect input".to_string())
    }
}

fn calculate(a: f64, op: char, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err(String::from("Error: Division by zero"))
            } else {
                Ok(a / b)
            }
        }
        '^' => Ok(a.powf(b)),
        '%' => Ok(a % b),
        's' => Ok(a.sqrt()),
        _ => Err(format!("Unknown operator: {}", op)),
    }
}

fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn main() {
    println!("{}", "============================".cyan());
    println!("{}", "||   CLI Calculator v1.0  ||".cyan());
    println!("{}", "============================\n".cyan());

    println!("Type {} for available commands\n", "'help'".yellow());

    let mut history: Vec<HistoryEntry> = load_history();

    if !history.is_empty() {
        println!(
            "{} {} {}",
            "Loaded".yellow().italic(),
            history.len().to_string().green(),
            "entries from history.txt\n".yellow().italic()
        );
    }

    loop {
        let input = get_input();

        match input.as_str() {
            "exit" | "quit" => {
                println!("{}", "Goodbye!".green().bold());
                break;
            }

            "history" => {
                if history.is_empty() {
                    println!("{}\n", "History is empty".yellow());
                } else {
                    println!("{}", "Calculation history:".cyan().bold());
                    for (i, entry) in history.iter().enumerate() {
                        println!("{}. {}", i + 1, entry);
                    }
                    println!();
                }
            }

            "clear" => {
                history.clear();
                println!("{}\n", "History cleared".yellow());
            }

            "save" => match save_history(&history) {
                Ok(_) => println!("{}\n", "History saved to 'history.txt'".green()),
                Err(e) => println!("{} {}\n", "Error: ".red(), e.red()),
            },

            "last" => {
                if let Some(entry) = history.last() {
                    println!("{} {}\n", "Last calculation: ".cyan(), entry);
                } else {
                    println!("{}\n", "No calculations yet".yellow());
                }
            }
            "help" => {
                println!("{}", "Available commands:".bold().magenta());
                println!(
                    "{}",
                    "  number op number  - Calculate (e.g., 5 + 3)".magenta()
                );
                println!("{}", "  Operators         - + - * / % ^  s".magenta());
                println!(
                    "{}",
                    "  history           - Show calculation history".magenta()
                );
                println!("{}", "  clear             - Clear history".magenta());
                println!("{}", "  save              - Save history to file".magenta());
                println!(
                    "{}",
                    "  last              - Show last calculation".magenta()
                );
                println!("{}\n", "  exit/quit         - Exit calculator".magenta());
            }
            _ => match evaluate_expression(&input) {
                Ok(result) => {
                    println!("{} {}\n", "=".green(), result.to_string().green());
                    let record = HistoryEntry::new(input, result);
                    history.push(record);
                }
                Err(e) => println!("{} {}\n", "Error:".red(), e.red()),
            },
        }
    }

    match save_history(&history) {
        Ok(_) => {}
        Err(e) => eprintln!("Warning: failed to save history: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(5.0, '+', 3.0).unwrap(), 8.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(10.0, '-', 3.0).unwrap(), 7.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(5.0, '*', 3.0).unwrap(), 15.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(10.0, '/', 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let result = calculate(10.0, '/', 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_operation() {
        let result = calculate(5.0, '@', 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_power() {
        assert_eq!(calculate(2.0, '^', 3.0).unwrap(), 8.0);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(calculate(10.0, '%', 3.0).unwrap(), 1.0);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(calculate(9.0, 's', 0.0).unwrap(), 3.0);
    }

    #[test]
    fn test_parse_valid_input() {
        let result = evaluate_expression("5 + 3").unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = evaluate_expression("5 +");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = evaluate_expression("abc + 3");
        assert!(result.is_err());
    }

    #[test]
    fn test_history_entry_to_string() {
        let entry = HistoryEntry::new("5 + 3".to_string(), 8.0);
        assert_eq!(entry.to_string(), "5 + 3 = 8");
        assert_eq!(entry.result, 8.0)
    }

    #[test]
    fn test_history_entry_new() {
        let entry = HistoryEntry::new("5 + 3".to_string(), 8.0);
        assert_eq!(entry.expression, "5 + 3");
        assert_eq!(entry.result, 8.0);
    }

    #[test]
    fn test_history_entry_with_float_result() {
        let entry = HistoryEntry::new("10 / 3".to_string(), 3.333333);
        assert!(entry.to_string().contains("10 / 3 = "));
    }

    #[test]
    fn test_save_and_load_history() {
        let mut history = Vec::new();
        history.push(HistoryEntry::new("5 + 3".to_string(), 8.0));
        history.push(HistoryEntry::new("10 * 2".to_string(), 20.0));

        save_history(&history).unwrap();
        let loaded = load_history();

        assert_eq!(loaded.len(), 2);

        std::fs::remove_file("history.txt").ok();
    }

    #[test]
    fn test_load_empty_history() {
        std::fs::remove_file("history.txt").ok();

        let history = load_history();
        assert!(history.is_empty());
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(evaluate_expression("(5 + 3) * 2").unwrap(), 16.0);
        assert_eq!(evaluate_expression("2 * (3 + 4)").unwrap(), 14.0);
    }

    #[test]
    fn test_precedence_expression() {
        assert_eq!(evaluate_expression("5 + 3 * 2").unwrap(), 11.0); // NOT 16!
        assert_eq!(evaluate_expression("10 / 2 + 3").unwrap(), 8.0);
    }

    #[test]
    fn test_complex() {
        assert_eq!(evaluate_expression("((2 + 3) * 4) - 1").unwrap(), 19.0);
        assert_eq!(evaluate_expression("2 ^ 3 + 1").unwrap(), 9.0);
    }
}
