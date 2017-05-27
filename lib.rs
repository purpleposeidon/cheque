//! Provides a macro allow natural usage of checked math.
//! 
//! ```
//! # #[macro_use] extern crate cheque;
//! # fn main() {
//! let a = 5u8;
//! let b = 20u8;
//! let z = 0u8;
//! 
//! checked_wrap![a, b, z];
//! 
//! assert_eq!(*(a + b), Some(25));
//! assert_eq!(*(b * b), None);
//! assert_eq!(*(a - b), None);
//! assert_eq!(*(b / z), None);
//! assert_eq!(*(a - 20), None);
//! assert_eq!(*((a - b) + 1), None);
//! # }
//! ```
//! 
//! `checked_wrap!` redeclares each passed in identifier as a checked numeric value.
//! You can then use `+`, etc. on the wrapped variables, and then deref the result to get an
//! `Option<_>`.
//! 
//! You can also use numeric literals/unwrapped values, so long as they are on the right side of
//! the operation.
//! 
//! ```
//! # #[macro_use] extern crate cheque;
//! # fn main() {
//! let c = 20usize;
//! checked_wrap![c];
//! 
//! if let Some(_) = *(c - 100) {
//!     panic!("Ahh!");
//! }
//! # }
//! ```
//! 
//! If you are doing generic programming, you should add the [checked num_traits] to your
//! `where` bounds.
//! [checked num_traits]: http://rust-num.github.io/num/num_traits/ops/checked/index.html

extern crate num_traits;

use num_traits::ops::checked::*;
use std::ops::*;

#[macro_export]
macro_rules! checked_wrap {
    ($($ident:ident),*) => {$(
        let $ident = $crate::Checker(Some($ident));
    )*};
    ($($ident:ident,)*) => {$(
        let $ident = $crate::Checker(Some($ident));
    )*};
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// This implements `Deref<Option<T>>`.
pub struct Checker<T>( #[doc(hidden)] pub /*(macro)*/ Option<T>);

impl<T> Deref for Checker<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for Checker<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

macro_rules! impl_checked {
    ($Vanilla:ident, $vanilla_fn:ident, $Checked:ident, $checked_fn:ident) => {
        impl<T> $Vanilla<Self> for Checker<T>
        where T: $Checked
        {
            type Output = Self;
            #[inline]
            fn $vanilla_fn(self, rhs: Checker<T>) -> Self {
                if let (Some(l), Some(r)) = (self.0, rhs.0) {
                    Checker(l.$checked_fn(&r))
                } else {
                    Checker(None)
                }
            }
        }

        impl<T> $Vanilla<T> for Checker<T>
        where T: $Checked
        {
            type Output = Self;
            #[inline]
            fn $vanilla_fn(self, rhs: T) -> Self {
                Checker(self.0.and_then(|l| l.$checked_fn(&rhs)))
            }
        }
    }
}

impl_checked![Add, add, CheckedAdd, checked_add];
impl_checked![Sub, sub, CheckedSub, checked_sub];
impl_checked![Mul, mul, CheckedMul, checked_mul];
impl_checked![Div, div, CheckedDiv, checked_div];


#[cfg(test)]
mod test {
    #[test]
    fn compiles() {
        let a = 10;
        checked_wrap![a];
        a + a;
        a - a;
        a * a;
        a / a;
        a + 1;
        a - 1;
        a * 1;
        a / 1;
    }
}
