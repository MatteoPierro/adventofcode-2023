#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn it_does_something() {
        let input = indoc! {"
        some input
        here"};

        assert_eq!("foo", input);
    }
}