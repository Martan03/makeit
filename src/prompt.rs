use std::io::{stdin, stdout, BufRead, Write};

pub fn replace_prompt(template: &str, auto_yes: bool) -> bool {
    yes_no(
        auto_yes,
        format!(
            "Template '{}' already exists.\nDo you want to replace it?",
            template
        ),
    )
}

pub fn not_empty_prompt(auto_yes: bool) -> bool {
    yes_no(
        auto_yes,
        "Direcotry is not empty.\nDo you want to continue anyway?",
    )
}

/// Creates yes or no prompt with yes as default option
pub fn yes_no<T>(auto_yes: bool, question: T) -> bool
where
    T: AsRef<str>,
{
    if auto_yes {
        return true;
    }

    print!("{} [Y/n]: ", question.as_ref());
    _ = stdout().flush();
    let stdin = stdin();
    let answer = stdin.lock().lines().next().unwrap().unwrap();

    match &*answer.to_lowercase() {
        "n" | "no" => false,
        _ => true,
    }
}
