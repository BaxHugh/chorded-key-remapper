use std::fmt;

pub fn format_strings(strings: Vec<String>) -> String {
    format!(
        "{}",
        strings
            .into_iter()
            .map(|n| format!("'{n}'"))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
