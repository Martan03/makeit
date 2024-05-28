use std::io::{stdin, stdout, BufRead, Write};

/// Creates yes or no prompt with yes as default option
pub fn yes_no(question: &str) -> bool {
    print!("{question} [Y/n]: ");
    _ = stdout().flush();
    let stdin = stdin();
    let answer = stdin.lock().lines().next().unwrap().unwrap();

    match &*answer.to_lowercase() {
        "n" | "no" => false,
        _ => true,
    }
}
