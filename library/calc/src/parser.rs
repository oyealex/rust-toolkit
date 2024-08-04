use nom::IResult;

use crate::calculator::Number;

fn number(input: &str) -> IResult<&str, Number> {
    todo!()
}

fn integer(input: &str) -> IResult<&str, Number> {
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_i32() {
        let i: i32 = i32::MAX;
        let s: String = (i + 0).to_string();
        println!("{s}");
        let x = "2147483647".parse::<i32>();
        println!("{:?}", x);
    }
}
