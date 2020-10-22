pub mod parser;
pub mod processor;

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        use super::parser::parse;
        use super::parser::Instruction::*;

        assert_eq!(
            parse("+-><[,.]+"),
            Ok(vec![
                Increment,
                Decrement,
                PointerIncrement,
                PointerDecrement,
                While(vec![GetChar, PutChar]),
                Increment,
            ])
        );
        assert!(parse("+++]").is_err());
        assert!(parse("[+++").is_err());
    }

    #[test]
    fn test_processor() {
        use super::parser::parse;
        use super::processor::execute;

        let hello_world =
            &parse("+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.").unwrap();
        assert_eq!(execute(&hello_world, 256, ""), "hello world");
    }
}
