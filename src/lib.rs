pub mod starter_hello_world {
    const MESSAGE: &str = "Hello, world!";

    pub fn greet() {
        println!("{}", MESSAGE);
    }

    #[test]
    fn check_message() {
        assert_eq!(MESSAGE, "Hello, world!");
    }
}
