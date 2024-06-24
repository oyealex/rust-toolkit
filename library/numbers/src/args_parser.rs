use crate::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;

enum RadixToArg {
    Target(RadixTarget),
    Prefix(bool),
    Upper(bool),
    Fill { fill_char: char, length: u32 },
}

enum RadixTarget {
    Bin,
    Oct,
    Dec,
    Hex,
}

enum WordCase {
    Upper,
    Lower,
}

enum FillDirection {
    Left,
    Right,
}

/// 解析命令行参数
/// 例如：
/// ```bash
/// radix [bin|b|oct|o|dec|d|hex|h] [[prefix|p]|[upper|u|lower|l]] [<fill|f> <length> [left|l|right|r]]
/// ```
fn parse_args(_args: &str) -> Result<Vec<RadixToArg>> {
    todo!()
}

#[cfg(test)]
mod test {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::is_space;
    use nom::character::streaming::one_of;
    use nom::combinator::{opt, success};

    use crate::Result;

    #[test]
    fn test() {
        fn parse(input: &str) -> Result<&str> {
            alt((
                alt((tag("bin"), tag("b"))),
                alt((tag("oct"), tag("o"))),
                alt((tag("dec"), tag("d"))),
                alt((tag("hex"), tag("h"))),
            ))(input)
        }

        println!("{:?}", parse("bin"));
        println!("{:?}", parse("b"));
        println!("{:?}", parse("oct"));
        println!("{:?}", parse("o"));
        println!("{:?}", parse("dec"));
        println!("{:?}", parse("d"));
        println!("{:?}", parse("hex"));
        println!("{:?}", parse("h"));
        println!("{:?}", parse("abc"));
    }
}
