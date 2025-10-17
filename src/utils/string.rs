/** String utilities */

/** Trim and collapse whitespace to single spaces. */
#[allow(dead_code)]
pub fn normalize_whitespace(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_space = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !prev_space { out.push(' '); prev_space = true; }
        } else {
            out.push(ch);
            prev_space = false;
        }
    }
    out.trim().to_string()
}
