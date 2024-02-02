#[doc(no_inline)]
pub use alloc::collections::*;

mod hash;

pub use hash::map::HashMap;

pub mod hash_map {
    //! A hash map implemented with quadratic probing and SIMD lookup.
    pub use super::hash::map::*;
}
