#[cfg(test)]
mod tests {
    use crate::input_reader::*;

    #[test]
    fn it_works() {
        let input = read_input_file("test.txt");
        let lines = read_lines(&input);
        assert_eq!(vec!["some text", "here and", "there"], lines);
    }
}