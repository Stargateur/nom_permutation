pub use nom;

use crate::nom::{error::ParseError, IResult as ParserResult};

/// Helper trait for the [permutation_opt()] combinator.
///
/// This trait is implemented for tuples of up to 21 elements
pub trait PermutationOpt<I, O, E> {
    /// Tries to apply all parsers in the tuple in various orders until all of them succeed or no one succeed anymore
    fn permutation_opt(&mut self, input: I) -> ParserResult<I, O, E>;
}

/// Applies a list of parsers in any order.
///
/// Permutation Optional will always succeed unless a parser return an unrecoverable error
/// It takes as argument a tuple of parsers, and returns a
/// tuple of the parser optional results.
///
/// ```rust
/// # use nom_permutation::nom::{Err,error::{Error, ErrorKind}, Needed, IResult};
/// use nom_permutation::nom::character::complete::{alpha1, digit1};
/// use nom_permutation::permutation_opt;
/// # fn main() {
/// fn parser(input: &str) -> IResult<&str, (Option<&str>, Option<&str>)> {
///   permutation_opt((alpha1, digit1))(input)
/// }
///
/// // permutation recognizes alphabetic characters then digit
/// assert_eq!(parser("abc123"), Ok(("", (Some("abc"), Some("123")))));
///
/// // but also in inverse order
/// assert_eq!(parser("123abc"), Ok(("", (Some("abc"), Some("123")))));
///
/// // it will not fail if one of the parsers failed
/// assert_eq!(parser("abc;"), Ok((";", (Some("abc"), None))));
///
/// // in any order
/// assert_eq!(parser("123;"), Ok((";", (None, Some("123")))));
/// # }
/// ```
///
/// The parsers are applied greedily: if there are multiple unapplied parsers
/// that could parse the next slice of input, the first one is used.
/// ```rust
/// # use nom_permutation::nom::{Err, error::{Error, ErrorKind}, IResult};
/// use nom_permutation::nom::character::complete::{anychar, char};
/// use nom_permutation::permutation_opt;
///
/// fn parser(input: &str) -> IResult<&str, (Option<char>, Option<char>)> {
///   permutation_opt((anychar, char('a')))(input)
/// }
///
/// // anychar parses 'b', then char('a') parses 'a'
/// assert_eq!(parser("ba"), Ok(("", (Some('b'), Some('a')))));
///
/// // anychar parses 'a', then char('a') fails on 'b',
/// // even though char('a') followed by anychar would succeed
/// assert_eq!(parser("ab"), Ok(("b", (Some('a'), None))));
/// ```
///
///
pub fn permutation_opt<I: Clone, O, E: ParseError<I>, List: PermutationOpt<I, O, E>>(
    mut l: List,
) -> impl FnMut(I) -> ParserResult<I, O, E> {
    move |i: I| l.permutation_opt(i)
}

// include!(concat!(env!("OUT_DIR"), "/permutation_opt.rs"));
