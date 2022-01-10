//! Provides utilities to work with the values being passed by a
//! type's [`Hash`] implementation while preserving the information of
//! which `write_` method was used to write them.

use std::hash::Hasher;

macro_rules! define_hasher_datum {
    ($($method:ident($inttype:ty) -> $var:ident;)+) => {
        #[derive(PartialEq, Eq)]
        /// An enum representing each of the different types of data
        /// that can be written to a [`Hasher`].
        pub enum HasherDatum {
            /// Marks the beginning of a slice from [`Hasher::write`]
            StartSlice,
            /// Byte from [`Hasher::write`]
            Byte(u8),
            $($var($inttype)),+
        }

        impl HasherDatum {
            /// Writes each datum in the iterator into `state` using
            /// the appropriate [`Hasher`] method.
            pub fn hash_data<I, H: Hasher>(data: I, state: &mut H)
                where I: IntoIterator<Item = HasherDatum>
            {
                use HasherDatum::*;
                let mut slice_data = Vec::new();
                let mut iter = data.into_iter().peekable();

                while let Some(datum) = iter.next() {
                    match datum {
                        StartSlice => {
                            slice_data.clear();
                            while let Some(&Byte(v)) = iter.peek() {
                                slice_data.push(v);
                                iter.next();
                            }
                            state.write(&*slice_data);
                        },
                        Byte(_) => {
                            panic!("Byte outside of slice in hasher data");
                        },
                        $($var(v) => {
                            state.$method(v);
                        })+
                    }
                }
            }
        }

        #[derive(Default)]
        /// A hasher which wraps each value it receives in the
        /// appropriate [`HasherDatum`] variant and passes it to a
        /// [`ConsumeHasherDatum`]. It will also call `finish` on the
        /// [`ConsumeHasherDatum`] if [`Hasher::finish`] is called on it.
        pub struct DatumHasher<C> {
            consumer: C
        }
        
        impl<C: ConsumeHasherDatum> DatumHasher<C> {
            pub fn new(consumer: C) -> Self {
                DatumHasher {
                    consumer
                }
            }

            pub fn into_inner(self) -> C {
                self.consumer
            }
        }

        impl<C: ConsumeHasherDatum> Hasher for DatumHasher<C> {
            fn finish(&self) -> u64 {
                self.consumer.finish()
            }
            
            fn write(&mut self, data: &[u8]) {
                self.consumer.consume(HasherDatum::StartSlice);
                for &datum in data {
                    self.consumer.consume(HasherDatum::Byte(datum));
                }
            }
        
            $(fn $method(&mut self, val: $inttype) {
                    self.consumer.consume(HasherDatum::$var(val));
            })+
        }
    }
}

define_hasher_datum! {
    write_usize(usize) -> Usize;
    write_u8(u8) -> U8;
    write_u16(u16) -> U16;
    write_u32(u32) -> U32;
    write_u64(u64) -> U64;
    write_u128(u128) -> U128;
    write_isize(isize) -> Isize;
    write_i8(i8) -> I8;
    write_i16(i16) -> I16;
    write_i32(i32) -> I32;
    write_i64(i64) -> I64;
    write_i128(i128) -> I128;
}

/// `ConsumeHasherDatum` allows handling [`Hasher`] method calls as
/// [`HasherDatum`s passed in from a [`DatumHasher`].
pub trait ConsumeHasherDatum {
    /// Consume a `HasherDatum` from the hasher.
    fn consume(&mut self, datum: HasherDatum);

    /// Called when a [`Hash`] implementation calls
    /// [`Hasher::finish`].
    ///
    /// # Panics
    ///
    /// The default `finish` implementation will panic if it is
    /// called. This is because hashers are expected to have a `u64`
    /// state which can be returned when [`finish`][Hasher::finish] is
    /// called, but in order to track all of the values which have
    /// been hashed, the state would need to be far bigger than a
    /// `u64`; therefore, there is in general no sensible way to
    /// handle a type which calls [`finish`][Hasher::finish] in its
    /// [`Hash`] implementation and passes anything derived from that
    /// value back into the hasher.
    fn finish(&self) -> u64 {
        panic!("Cannot finish() a ConsumeHasherDatum")
    }
}

#[derive(Default)]
/// Consumes `HasherDatum`s and accumulates them into a list for
/// `EqTestCmp`.
pub struct EqTestAcc {
    data: Vec<HasherDatum>
}

impl ConsumeHasherDatum for EqTestAcc {
    fn consume(&mut self, datum: HasherDatum) {
        self.data.push(datum);
    }
}

/// `EqTestCmp` allows comparing a new set of `HasherDatum`s to one
/// previously collected by `EqTestAcc`.
pub struct EqTestCmp {
    cmp: std::vec::IntoIter<HasherDatum>,
    is_eq: bool
}

impl EqTestCmp {
    pub fn result(mut self) -> bool {
        self.is_eq && self.cmp.next().is_none()
    }
}

impl From<EqTestAcc> for EqTestCmp {
    fn from(acc: EqTestAcc) -> Self {
        EqTestCmp {
            cmp: acc.data.into_iter(),
            is_eq: true
        }
    }
}

impl ConsumeHasherDatum for EqTestCmp {
    fn consume(&mut self, datum: HasherDatum) {
        self.is_eq = self.is_eq && self.cmp.next() == Some(datum);
    }
}
