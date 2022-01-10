#![feature(hash_raw_entry)]
use std::hash::Hash;

mod ext;

pub use ext::HashMapExt;

/// Marks two types as having compatible `Hash` implementations and
/// allows checking whether two values would have the same hash if
/// hashed by any hasher.
///
/// The existence of a `HashEq<T>` implementation for a type `U`
/// implies that the hashing process for `T` and `U` is analagous;
/// that is, `T` and `U` hash the same sort of data, in the same
/// order, and with the same `Hasher` methods. There is a default
/// implementation which directly compares the values received by the
/// hasher, but it may be slower than a hand-written implementation.
/// 
/// If `T: HashEq<U> + PartialEq<U>` then for any `t: T` and `u: U`
/// where `t == u`, the hash of `t` should be equal to the hash of `u`
/// in any hasher.
///
/// # Implementing
///
/// `HashEq<U>` can always be implemented for `T` if `T: Borrow<U>` or
/// `U: Borrow<T>`; default implementations for this behaviour may be
/// provided when specialization is possible.
pub trait HashEq<Rhs>
where Self: Hash,
      Rhs: Hash
{}

