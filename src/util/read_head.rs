use std::fs;

pub fn read_current_head() -> std::io::Result<String> {
    let content = fs::read_to_string(".bic/HEAD")?;
    Ok(content.trim().to_string())
}