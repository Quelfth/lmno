use std::num::ParseIntError;


#[derive(Debug)]
pub enum Error {
    MixedWhitespace,
    InvalidEscape,
    TrailingContent,
    InvalidLiteral,
    InvalidNumericBase,
    InvalidDigit(ParseIntError),
    MissingValue,
    MismatchedEntry,
    ValueInScalar,
}
