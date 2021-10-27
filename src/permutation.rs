//! Choice combinators

use crate::nom::{
    error::{ErrorKind as ParserKind, ParseError},
    Err as OutCome, IResult as ParserResult, Parser,
};

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

macro_rules! permutation_trait(
    (
      $name1:ident $ty1:ident $item1:ident
      $name2:ident $ty2:ident $item2:ident
      $($name3:ident $ty3:ident $item3:ident)*
    ) => (
      permutation_trait!(__impl $name1 $ty1 $item1, $name2 $ty2 $item2; $($name3 $ty3 $item3)*);
    );
    (
      __impl $($name:ident $ty:ident $item:ident),+;
      $name1:ident $ty1:ident $item1:ident $($name2:ident $ty2:ident $item2:ident)*
    ) => (
      permutation_trait_impl!($($name $ty $item),+);
      permutation_trait!(__impl $($name $ty $item),+ , $name1 $ty1 $item1; $($name2 $ty2 $item2)*);
    );
    (__impl $($name:ident $ty:ident $item:ident),+;) => (
      permutation_trait_impl!($($name $ty $item),+);
    );
  );

macro_rules! permutation_trait_impl(
    ($($name:ident $ty:ident $item:ident),+) => (
      impl<
        Input: Clone, $($ty),+ , Error: ParseError<Input>,
        $($name: Parser<Input, $ty, Error>),+
      > Permutation<Input, ( $($ty),+ ), Error> for ( $($name),+ ) {

        fn permutation(&mut self, mut input: Input) -> ParserResult<Input, ( $($ty),+ ), Error> {
          let mut res = ($(Option::<$ty>::None),+);

          loop {
            let mut err: Option<Error> = None;
            permutation_trait_inner!(0, self, input, res, err, $($name)+);

            // If we reach here, every iterator has either been applied before,
            // or errored on the remaining input
            if let Some(err) = err {
              // There are remaining parsers, and all errored on the remaining input
              return Err(OutCome::Error(Error::append(input, ParserKind::Permutation, err)));
            }

            // All parsers were applied
            match res {
              ($(Some($item)),+) => return Ok((input, ($($item),+))),
              _ => unreachable!(),
            }
          }
        }
      }
    );
  );

macro_rules! permutation_trait_inner(
    ($it:tt, $self:expr, $input:ident, $res:expr, $err:expr, $head:ident $($id:ident)*) => (
      if $res.$it.is_none() {
        match $self.$it.parse($input.clone()) {
          Ok((i, o)) => {
            $input = i;
            $res.$it = Some(o);
            continue;
          }
          Err(OutCome::Error(e)) => {
            $err = Some(match $err {
              Some(err) => err.or(e),
              None => e,
            });
          }
          Err(e) => return Err(e),
        };
      }
      succ!($it, permutation_trait_inner!($self, $input, $res, $err, $($id)*));
    );
    ($it:tt, $self:expr, $input:ident, $res:expr, $err:expr,) => ();
  );

permutation_trait!(
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
