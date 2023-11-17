use std::fs;
use std::path::PathBuf;

pub fn read_lines(input: &str) -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(input);
    fs::read_to_string(&path)
        .map(|c|
            c.lines().map(String::from).collect()
        ).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_input_lines() {
        let lines = read_lines("test.txt");
        assert_eq!(vec!["some text", "here and", "there"], lines);
    }
}