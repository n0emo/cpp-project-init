pub fn trim_newline(s: &mut String) {
    s.truncate(s.trim_end().len());
    s.push('\n');
}
