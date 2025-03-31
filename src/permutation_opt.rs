//! Choice combinators

use crate::nom::{
  Err as OutCome,
  IResult as ParserResult,
  Parser,
  error::ParseError,
};

/// Helper trait for the [permutation_opt()] combinator.
///
/// This trait is implemented for tuples of up to 21 elements
pub trait PermutationOpt<I, O, E> {
  /// Tries to apply all parsers in the tuple in various orders until all of
  /// them succeed or no one succeed anymore
  fn permutation_opt(&mut self, input: I) -> ParserResult<I, O, E>;
}

/// Applies a list of parsers in any order.
///
/// Permutation Optional will always succeed unless a parser return an
/// unrecoverable error It takes as argument a tuple of parsers, and returns a
/// tuple of the parser optional results.
///
/// ```rust
/// # use nom_permutation::nom::{Err,error::{Error, ErrorKind}, Needed, IResult};
/// use nom_permutation::{
///   nom::character::complete::{
///     alpha1,
///     digit1,
///   },
///   permutation_opt,
/// };
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
/// use nom_permutation::{
///   nom::character::complete::{
///     anychar,
///     char,
///   },
///   permutation_opt,
/// };
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
pub fn permutation_opt<I: Clone, O, E: ParseError<I>, List: PermutationOpt<I, O, E>>(
  mut l: List,
) -> impl FnMut(I) -> ParserResult<I, O, E> {
  move |i: I| l.permutation_opt(i)
}

macro_rules! permutation_opt_trait(
    (
      $name1:ident $ty1:ident $item1:ident
      $name2:ident $ty2:ident $item2:ident
      $($name3:ident $ty3:ident $item3:ident)*
    ) => (
      permutation_opt_trait!(__impl $name1 $ty1 $item1, $name2 $ty2 $item2; $($name3 $ty3 $item3)*);
    );
    (
      __impl $($name:ident $ty:ident $item:ident),+;
      $name1:ident $ty1:ident $item1:ident $($name2:ident $ty2:ident $item2:ident)*
    ) => (
      permutation_opt_trait_impl!($($name $ty $item),+);
      permutation_opt_trait!(__impl $($name $ty $item),+ , $name1 $ty1 $item1; $($name2 $ty2 $item2)*);
    );
    (__impl $($name:ident $ty:ident $item:ident),+;) => (
      permutation_opt_trait_impl!($($name $ty $item),+);
    );
  );

macro_rules! permutation_opt_trait_impl(
    ($($name:ident $ty:ident $item:ident),+) => (
      impl<
        Input: Clone, $($ty),+ , Error: ParseError<Input>,
        $($name: Parser<Input, Output=$ty, Error=Error>),+
      > PermutationOpt<Input, ( $(Option<$ty>),+ ), Error> for ( $($name),+ ) {

        fn permutation_opt(&mut self, mut input: Input) -> ParserResult<Input, ( $(Option<$ty>),+ ), Error> {
          let mut res = ($(Option::<$ty>::None),+);

          loop {
            permutation_trait_opt_inner!(0, self, input, res, err, $($name)+);

            // All parsers were applied or failed
            break Ok((input, res));
          }
        }
      }
    );
  );

macro_rules! permutation_trait_opt_inner(
    ($it:tt, $self:expr, $input:ident, $res:expr, $err:expr, $head:ident $($id:ident)*) => (
      if $res.$it.is_none() {
        match $self.$it.parse($input.clone()) {
          Ok((i, o)) => {
            $input = i;
            $res.$it = Some(o);
            continue;
          }
          Err(OutCome::Error(_)) => {}
          Err(e) => return Err(e),
        };
      }
      succ!($it, permutation_trait_opt_inner!($self, $input, $res, $err, $($id)*));
    );
    ($it:tt, $self:expr, $input:ident, $res:expr, $err:expr,) => ();
  );

macro_rules! succ (
    (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
    (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
    (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
    (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
    (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
    (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
    (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
    (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
    (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
    (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
    (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
    (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
    (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
    (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
    (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
    (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
    (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
    (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
    (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
    (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
    (20, $submac:ident ! ($($rest:tt)*)) => ($submac!(21, $($rest)*));
  );

permutation_opt_trait!(
  FnA A a
  FnB B b
  FnC C c
  FnD D d
  FnE E e
  FnF F f
  FnG G g
  FnH H h
  FnI I i
  FnJ J j
  FnK K k
  FnL L l
  FnM M m
  FnN N n
  FnO O o
  FnP P p
  FnQ Q q
  FnR R r
  FnS S s
  FnT T t
  FnU U u
);
