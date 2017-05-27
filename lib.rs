//! Provides a macro to make it simple to do a long string of checked math.
//! 
//! ```
//! # #[macro_use] extern crate cheque;
//! # fn main() {
//! let a = 5u8;
//! let b = 20u8;
//! let z = 0u8;
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

extern crate num_traits;

use num_traits::ops::checked::*;
use std::ops::*;

#[macro_export]
macro_rules! checked_wrap {
    ($($ident:ident),*) => {$(
        let $ident = $crate::Wrapper(Some($ident));
    )*};
    ($($ident:ident,)*) => {$(
        let $ident = $crate::Wrapper(Some($ident));
    )*};
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// This implements `Deref<Option<T>>`.
pub struct Wrapper<T>( #[doc(hidden)] pub /*(macro)*/ Option<T>);

impl<T> Deref for Wrapper<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

macro_rules! impl_checked {
    ($Vanilla:ident, $vanilla_fn:ident, $Checked:ident, $checked_fn:ident) => {
        impl<T> $Vanilla<Self> for Wrapper<T>
        where T: $Checked
        {
            type Output = Self;
            #[inline]
            fn $vanilla_fn(self, rhs: Wrapper<T>) -> Self {
                if let (Some(l), Some(r)) = (self.0, rhs.0) {
                    Wrapper(l.$checked_fn(&r))
                } else {
                    Wrapper(None)
                }
            }
        }

        impl<T> $Vanilla<T> for Wrapper<T>
        where T: $Checked
        {
            type Output = Self;
            #[inline]
            fn $vanilla_fn(self, rhs: T) -> Self {
                Wrapper(self.0.and_then(|l| l.$checked_fn(&rhs)))
            }
        }
    }
}

impl_checked![Add, add, CheckedAdd, checked_add];
impl_checked![Sub, sub, CheckedSub, checked_sub];
impl_checked![Mul, mul, CheckedMul, checked_mul];
impl_checked![Div, div, CheckedDiv, checked_div];

