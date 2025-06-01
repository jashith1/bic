use std::fs;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

pub fn build_ignore_matcher() -> Gitignore {
    let mut builder = GitignoreBuilder::new(".");

    // read patterns from .bic_ignore
    if let Ok(contents) = fs::read_to_string(".bic_ignore") {
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                builder.add_line(None, trimmed).unwrap();
            }
        }   
    }

    // add hardcoded patterns
    builder.add_line(None, ".git").unwrap();
    builder.add_line(None, ".bic").unwrap();

    builder.build().unwrap()
}