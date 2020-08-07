//! Implements a number type so we do not have to use floating point arithmetic.

use std::{fmt, ops::Sub};
use tracing::error;

/// Independent Reserve API never returns a fraction with more than 8
/// decimal places. This is precision enough for dollars and dollars.
const NUM_DECIMAL_PLACES: u32 = 8;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct Num {
    // We are only dealing with dollars and bitcoins, I doubt we will overflow.
    inner: isize,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
impl From<f32> for Num {
    fn from(f: f32) -> Self {
        if f.is_nan() {
            error!("cannot convert from NAN");
            return Num { inner: 0 };
        }

        let raw = f * 10_u32.pow(NUM_DECIMAL_PLACES) as f32;

        Num {
            inner: raw.trunc() as isize,
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let float = self.inner as f32 / 10_u32.pow(NUM_DECIMAL_PLACES) as f32;
        write!(f, "{}", float)
    }
}

impl Sub for Num {
    type Output = Num;

    fn sub(self, other: Num) -> Num {
        Num {
            inner: self.inner - other.inner,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use spectral::prelude::*;

    #[test]
    #[allow(clippy::excessive_precision)]
    fn from_float_works_more_decimal_places() {
        let float = 1.123_456_789_123;
        let num = Num::from(float);
        let want = "1.12345678".to_string();
        let got = num.to_string();

        assert_that!(&got).is_equal_to(&want);
    }

    #[test]
    fn from_float_works_less_decimal_places() {
        let float = 1.123_4;
        let num = Num::from(float);
        let want = "1.12340000".to_string();
        let got = num.to_string();

        assert_that!(&got).is_equal_to(&want);
    }

    #[test]
    fn sub_works() {
        let a = Num::from(1.6);
        let b = Num::from(2.2);

        let want = Num::from(0.6);
        let got = b - a;

        assert_that!(&got).is_equal_to(&want);
    }

    proptest! {
        #[test]
        fn from_doesnt_panic(f in any::<f32>()) {
            let _ = Num::from(f);
        }
    }
}
