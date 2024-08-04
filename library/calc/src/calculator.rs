pub(crate) enum ExpItem {
    Number(Number),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Number {
    Integer(i64),
    Long(i128),
    Double(f64),
}

pub(crate) enum Operator {
    Add(Option<Number>, Option<Number>),
    Subtract(Option<Number>, Option<Number>),
    Multiply(Option<Number>, Option<Number>),
    Divide(Option<Number>, Option<Number>),
}
