#[cfg(test)]
mod tests {
    use crate::input_reader::read_lines;

    #[test]
    fn it_works() {
        let lines = read_lines("test.txt");
        assert_eq!(vec!["some text", "here and", "there"], lines);
    }
}