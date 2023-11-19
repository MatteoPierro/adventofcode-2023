use std::fs;
use std::path::PathBuf;

pub fn read_lines(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn read_input_file(filename: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(filename);
    fs::read_to_string(&path).unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn it_reads_input_file() {
        let input = read_input_file("test.txt");
        let lines = read_lines(&input);
        assert_eq!(vec!["some text", "here and", "there"], lines);
    }

    #[test]
    fn it_reads_lines() {
        let input = indoc! {"
        foo bar
        fizz
        bazz"};
        let lines = read_lines(&input);
        assert_eq!(vec!["foo bar", "fizz", "bazz"], lines);
    }
}