pub use nom;

#[cfg(feature = "permutation")]
mod permutation;
#[cfg(feature = "permutation_opt")]
mod permutation_opt;

#[cfg(feature = "permutation")]
pub use permutation::permutation;

#[cfg(feature = "permutation_opt")]
pub use permutation_opt::permutation_opt;
