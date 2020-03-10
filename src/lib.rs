//! Simple driver for 7-segment displays
//!
//! This is a driver (encoder) for 7-segment displays. It's implemented on top of embedded-hal, so you can use it on any platform that has pins with `embedded_hal::OutputPin` implemented.
//! 
//! The driver is very simple, only supports displays that connect directly using seven pins such as [SA52-11EWA](http://www.kingbrightusa.com/images/catalog/SPEC/SA52-11EWA.pdf) and doesn't try to do anything clever like setting all pins at once. It supports both common anode and common cathode displays.
//!
//! In order to use this crate, you have to instantiate `SevenSegmentPins` with your pins (see its
//! documentation for a diagram) and convert it by calling appropriate `with_common_*()` method.
//! Then just call `set(your_digit_here)` on the result whenever you want to change the shown
//! digit.

#![no_std]
#![allow(deprecated)]

pub use embedded_hal::digital::OutputPin;

/// Type erased definitions
pub mod erased {
    /// An alias for SevenSegment which has all pins of the same type.
    pub type SevenSegment<T, Common> = super::SevenSegment<T, T, T, T, T, T, T, Common>;
}

pub use v_0_2::{Polarity, Anode, Cathode};

/// Pins of the 7-sement display
///
/// Pin arrangment:
/// ```text
///  ________
/// |\__a___/|
/// | |    | |
/// |f|    |b|
/// | |____| |
/// |/  g   \|
/// |\______/|
/// | |    | |
/// |e|    |c|
/// | |____| |
/// |/__d___\|
/// ```
///
/// # 0.2 compatibility note
///
/// This is a distinct struct due to inherent methods
pub struct SevenSegmentPins<A, B, C, D, E, F, G> {
    pub a: A,
    pub b: B,
    pub c: C,
    pub d: D,
    pub e: E,
    pub f: F,
    pub g: G,
}

impl<A, B, C, D, E, F, G> SevenSegmentPins<A, B, C, D, E, F, G> {
    /// Constructs `SevenSegment` with specified polarity.
    pub fn with_common<Common: Polarity>(self) -> SevenSegment<A, B, C, D, E, F, G, Common> {
        SevenSegment {
            inner:
                v_0_2::SevenSegmentPins {
                    a: self.a,
                    b: self.b,
                    c: self.c,
                    d: self.d,
                    e: self.e,
                    f: self.f,
                    g: self.g,
                }
                .with_common::<Common>()
        }
    }

    /// Shorthand for `with_common::<Cathode>()`.
    ///
    /// This prevents you from having to import `Cathode` or write
    /// `with_common::<seven_segment::Cathode>()`
    pub fn with_common_cathode(self) -> SevenSegment<A, B, C, D, E, F, G, Cathode> {
        self.with_common::<Cathode>()
    }

    /// Shorthand for `with_common::<Anode>()`.
    ///
    /// This prevents you from having to import `Anode` or write
    /// `with_common::<seven_segment::Anode>()`
    pub fn with_common_anode(self) -> SevenSegment<A, B, C, D, E, F, G, Anode> {
        self.with_common::<Anode>()
    }
}

/// Represents 7-segment display.
///
/// This struct provides you a method to show a value on the 7-segment display by setting the
/// appropriate pins high or low.
///
/// Use `SevenSegmentPins` to construct it.
///
/// # 0.2 compatibility note
///
/// This is a distinct struct due to inherent methods
pub struct SevenSegment<A, B, C, D, E, F, G, Common> {
    inner: v_0_2::SevenSegment<A, B, C, D, E, F, G, Common>,
}

impl<A, B, C, D, E, F, G, Common> SevenSegment<A, B, C, D, E, F, G, Common> where
                                             A: OutputPin,
                                             B: OutputPin,
                                             C: OutputPin,
                                             D: OutputPin,
                                             E: OutputPin,
                                             F: OutputPin,
                                             G: OutputPin,
                                             Common: Polarity
{
    /// Sets the value of the display.
    ///
    /// The valid values are 0-9. In case of invalid value, the display will be blank.
    pub fn set(&mut self, value: u8) {
        // We have to do this to maintain logical backwards-compatibility,
        // since in the old version 10 means blank, but in the new version it's `a`.
        let value = if value > 9 {
            255
        } else {
            value
        };

        self
            .inner
            .set(value)
            // Why this is not `.unwrap_or_else(|e| match e {})`: unfortunately, the authors of
            // embedded-hal used `()` instead of `Infallible` or `void::Void` in the error type,
            // so this must be expect. :(
            .expect("this can't fail")
    }
}

impl<A, B, C, D, E, F, G, Common> From<v_0_2::SevenSegment<A, B, C, D, E, F, G, Common>> for SevenSegment<A, B, C, D, E, F, G, Common> {
    fn from(value: v_0_2::SevenSegment<A, B, C, D, E, F, G, Common>) -> Self {
        SevenSegment {
            inner: value,
        }
    }
}

impl<A, B, C, D, E, F, G, Common> From<SevenSegment<A, B, C, D, E, F, G, Common>> for v_0_2::SevenSegment<A, B, C, D, E, F, G, Common> {
    fn from(value: SevenSegment<A, B, C, D, E, F, G, Common>) -> Self {
        value.inner
    }
}

impl<A, B, C, D, E, F, G, Common> AsRef<v_0_2::SevenSegment<A, B, C, D, E, F, G, Common>> for SevenSegment<A, B, C, D, E, F, G, Common> {
    fn as_ref(&self) -> &v_0_2::SevenSegment<A, B, C, D, E, F, G, Common> {
        &self.inner
    }
}

impl<A, B, C, D, E, F, G, Common> AsMut<v_0_2::SevenSegment<A, B, C, D, E, F, G, Common>> for SevenSegment<A, B, C, D, E, F, G, Common> {
    fn as_mut(&mut self) -> &mut v_0_2::SevenSegment<A, B, C, D, E, F, G, Common> {
        &mut self.inner
    }
}

impl<A, B, C, D, E, F, G, Common> core::borrow::Borrow<v_0_2::SevenSegment<A, B, C, D, E, F, G, Common>> for SevenSegment<A, B, C, D, E, F, G, Common> {
    fn borrow(&self) -> &v_0_2::SevenSegment<A, B, C, D, E, F, G, Common> {
        &self.inner
    }
}

impl<A, B, C, D, E, F, G, Common> core::borrow::BorrowMut<v_0_2::SevenSegment<A, B, C, D, E, F, G, Common>> for SevenSegment<A, B, C, D, E, F, G, Common> {
    fn borrow_mut(&mut self) -> &mut v_0_2::SevenSegment<A, B, C, D, E, F, G, Common> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    fn test_digit(digit: u8, expected: (u8, u8, u8, u8, u8, u8, u8)) {
        struct TestPin(u8);

        impl TestPin {
            fn inv(&self) -> u8 {
                if self.0 == 0 {
                    1
                // Keep values > 1 in order to detect bugs
                } else if self.0 == 1 {
                    0
                } else {
                    self.0
                }
            }
        }

        impl super::OutputPin for &'_ mut TestPin {
            fn set_high(&mut self) {
                (*self).0 = 1;
            }

            fn set_low(&mut self) {
                (*self).0 = 0;
            }
        }

        // We're using 2 to signal uninitialized;
        let mut a = TestPin(2);
        let mut b = TestPin(2);
        let mut c = TestPin(2);
        let mut d = TestPin(2);
        let mut e = TestPin(2);
        let mut f = TestPin(2);
        let mut g = TestPin(2);

        {
            let mut seven_segment = super::SevenSegmentPins {
                a: &mut a,
                b: &mut b,
                c: &mut c,
                d: &mut d,
                e: &mut e,
                f: &mut f,
                g: &mut g,
            }.with_common_anode();

            seven_segment.set(digit);
        }

        assert_eq!((a.inv(), b.inv(), c.inv(), d.inv(), e.inv(), f.inv(), g.inv()), expected);

        let mut a = TestPin(2);
        let mut b = TestPin(2);
        let mut c = TestPin(2);
        let mut d = TestPin(2);
        let mut e = TestPin(2);
        let mut f = TestPin(2);
        let mut g = TestPin(2);

        {
            let mut seven_segment = super::SevenSegmentPins {
                a: &mut a,
                b: &mut b,
                c: &mut c,
                d: &mut d,
                e: &mut e,
                f: &mut f,
                g: &mut g,
            }.with_common_cathode();

            seven_segment.set(digit);
        }

        assert_eq!((a.0, b.0, c.0, d.0, e.0, f.0, g.0), expected);
    }

    #[test]
    fn digit_0() {
        test_digit(0, (1, 1, 1, 1, 1, 1, 0));
    }

    #[test]
    fn digit_1() {
        test_digit(1, (0, 1, 1, 0, 0, 0, 0));
    }

    #[test]
    fn digit_2() {
        test_digit(2, (1, 1, 0, 1, 1, 0, 1));
    }

    #[test]
    fn digit_3() {
        test_digit(3, (1, 1, 1, 1, 0, 0, 1));
    }

    #[test]
    fn digit_4() {
        test_digit(4, (0, 1, 1, 0, 0, 1, 1));
    }

    #[test]
    fn digit_5() {
        test_digit(5, (1, 0, 1, 1, 0, 1, 1));
    }

    #[test]
    fn digit_6() {
        test_digit(6, (1, 0, 1, 1, 1, 1, 1));
    }

    #[test]
    fn digit_7() {
        test_digit(7, (1, 1, 1, 0, 0, 0, 0));
    }

    #[test]
    fn digit_8() {
        test_digit(8, (1, 1, 1, 1, 1, 1, 1));
    }

    #[test]
    fn digit_9() {
        test_digit(9, (1, 1, 1, 1, 0, 1, 1));
    }

    #[test]
    fn digit_invalid() {
        test_digit(10, (0, 0, 0, 0, 0, 0, 0));
    }
}
