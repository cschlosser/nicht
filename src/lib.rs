// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A more natural way to use `not`
//!
//! # Examples
//!
//! ```
//! use nicht::not;
//!
//! assert_eq!(not(true), false);
//! ```

use std::ops::Not;

pub fn not<T: Not>(x: T) -> <T as Not>::Output {
    Not::not(x)
}

#[cfg(test)]
mod tests {
    use crate::not;

    #[test]
    fn primitive_data_type() {
        assert_eq!(not(true), false);
        assert_eq!(not(false), true);
        assert_ne!(not(true), true);
        assert_ne!(not(false), false);
    }
}
