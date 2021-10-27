pub use nom;

use crate::nom::{error::ParseError, IResult as ParserResult};

/// Helper trait for the [permutation()] combinator.
///
/// This trait is implemented for tuples of up to 21 elements
pub trait Permutation<I, O, E> {
    /// Tries to apply all parsers in the tuple in various orders until all of them succeed
    fn permutation(&mut self, input: I) -> ParserResult<I, O, E>;
}

/// Applies a list of parsers in any order.
///
/// Permutation will succeed if all of the child parsers succeeded.
/// It takes as argument a tuple of parsers, and returns a
/// tuple of the parser results.
///
/// ```rust
/// # use nom_permutation::nom::{Err,error::{Error, ErrorKind}, Needed, IResult};
/// use nom_permutation::nom::character::complete::{alpha1, digit1};
/// use nom_permutation::permutation;
/// # fn main() {
/// fn parser(input: &str) -> IResult<&str, (&str, &str)> {
///   permutation((alpha1, digit1))(input)
/// }
///
/// // permutation recognizes alphabetic characters then digit
/// assert_eq!(parser("abc123"), Ok(("", ("abc", "123"))));
///
/// // but also in inverse order
/// assert_eq!(parser("123abc"), Ok(("", ("abc", "123"))));
///
/// // it will fail if one of the parsers failed
/// assert_eq!(parser("abc;"), Err(Err::Error(Error::new(";", ErrorKind::Digit))));
/// # }
/// ```
///
/// The parsers are applied greedily: if there are multiple unapplied parsers
/// that could parse the next slice of input, the first one is used.
/// ```rust
/// # use nom_permutation::nom::{Err, error::{Error, ErrorKind}, IResult};
/// use nom_permutation::nom::character::complete::{anychar, char};
/// use nom_permutation::permutation;
///
/// fn parser(input: &str) -> IResult<&str, (char, char)> {
///   permutation((anychar, char('a')))(input)
/// }
///
/// // anychar parses 'b', then char('a') parses 'a'
/// assert_eq!(parser("ba"), Ok(("", ('b', 'a'))));
///
/// // anychar parses 'a', then char('a') fails on 'b',
/// // even though char('a') followed by anychar would succeed
/// assert_eq!(parser("ab"), Err(Err::Error(Error::new("b", ErrorKind::Char))));
/// ```
///
pub fn permutation<I: Clone, O, E: ParseError<I>, List: Permutation<I, O, E>>(
    mut l: List,
) -> impl FnMut(I) -> ParserResult<I, O, E> {
    move |i: I| l.permutation(i)
}

impl<Input: Clone, Error: nom::error::ParseError<Input>> Permutation<Input, (), Error> for () {
    fn permutation(&mut self, input: Input) -> nom::IResult<Input, (), Error> {
        Ok((input, ()))
    }
}

include!(concat!(env!("OUT_DIR"), "/permutation.rs"));
