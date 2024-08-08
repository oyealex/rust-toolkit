use nom::error::VerboseError;
use nom::IResult;

mod expression;
mod parser;
mod pretty;
mod errors;

type VResult<I, O> = IResult<I, O, VerboseError<I>>;