pub mod parser;
pub mod processor;

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        use super::parser::parse;
        use super::parser::InstructionTypes::*;

        assert_eq!(
            parse("+-><[,.]+"),
            Ok(vec![
                (0, Increment),
                (1, Decrement),
                (2, PointerIncrement),
                (3, PointerDecrement),
                (4, While(vec![(5, GetChar), (6, PutChar)])),
                (8, Increment)
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
