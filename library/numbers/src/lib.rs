use nom::IResult;

mod args_parser;

/// 通用结果，本crate处理的输入都是字符串
type Result<'a, O> = IResult<&'a str, O>;

/// 进制
mod number_radix {
    use crate::Result;

    pub fn parse_from_bin(input: &str) -> Result<&str> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use nom::character::complete::one_of;
    use nom::combinator::{opt, recognize};
    use nom::multi::many1;
    use nom::sequence::tuple;

    use crate::Result;

    #[test]
    fn test() {
        fn parser(input: &str) -> Result<&str> {
            recognize(tuple((opt(one_of("+-")), many1(one_of("0123456789")))))(input)
        }

        println!("{:?}", parser("1"));
        println!("{:?}", parser("123"));
        println!("{:?}", parser("-123"));
        println!("{:?}", parser("+123"));
        println!("{:?}", parser(""));
        println!("{:?}", parser("+"));
        println!("{:?}", parser("-"));
    }
}
