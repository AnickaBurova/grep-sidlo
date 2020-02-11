use failure::Fallible;
use regex::Regex;
use std::fs;

fn main() -> Fallible<()> {
    let regex = Regex::new(r"(?m)<strong>S&#237;dlo</strong>\s*<span>\s*((?:.|\n)*?)</span>")?;
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let text = fs::read_to_string(path)?;
            let result = regex.captures_iter(&text);
            for mat in result {
                let text = mat[1]
                    .lines()
                    .map(|v| v.trim().to_owned())
                    .collect::<Vec<_>>();
                let text = &text[..].join(" ");
                println!("{}", text);
            }
        }
    }
    Ok(())
}
