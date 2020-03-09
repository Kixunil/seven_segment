//! Simple driver for 7-segment displays
//!
//! This is a driver (encoder) for 7-segment displays. It's implemented on top of embedded-hal, so
//! you can use it on any platform that has pins with `embedded_hal::OutputPin` implemented.
//!
//! The driver is very simple, only supports displays that connect directly using seven pins such
//! as [SA52-11EWA](http://www.kingbrightusa.com/images/catalog/SPEC/SA52-11EWA.pdf) and doesn't
//! try to do anything clever like setting all pins at once. It supports both common anode and
//! common cathode displays.
//!
//! In order to use this crate, you have to instantiate `SevenSegmentPins` with your pins (see its
//! documentation for a diagram) and convert it by calling appropriate `with_common_*()` method.
//! Then just call `set(your_digit_here)` on the result whenever you want to change the shown
//! digit.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use embedded_hal::digital::v2::OutputPin;

/// Type erased definitions
pub mod erased {
    /// An alias for SevenSegment which has all pins of the same type.
    pub type SevenSegment<T, Common> = super::SevenSegment<T, T, T, T, T, T, T, Common>;
}

mod sealed {
    pub trait Polarity {
        fn is_cathode() -> bool;
    }

    impl Polarity for super::Anode {
        fn is_cathode() -> bool {
            false
        }
    }

    impl Polarity for super::Cathode {
        fn is_cathode() -> bool {
            true
        }
    }
}

/// Polarity of the common electrode.
///
/// This trait is sealed and is only implemented for `Anode` and `Cathode` as they're all that's
/// needed.
pub trait Polarity: sealed::Polarity {}

/// Marker type for common anode
pub enum Anode {}

/// Marker type for common cathode
pub enum Cathode {}

impl Polarity for Anode {}
impl Polarity for Cathode {}

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
pub struct SevenSegmentPins<A, B, C, D, E, F, G> {
    /// Upper horizontal bar
    pub a: A,
    /// Upper right vertical bar
    pub b: B,
    /// Lower right vertical bar
    pub c: C,
    /// Lower horizontal bar
    pub d: D,
    /// Lower left vertical bar
    pub e: E,
    /// Upper left vertival bar
    pub f: F,
    /// Middle horizontal bar
    pub g: G,
}

impl<A, B, C, D, E, F, G> SevenSegmentPins<A, B, C, D, E, F, G> {
    /// Constructs `SevenSegment` with specified polarity.
    pub fn with_common<Common: Polarity>(self) -> SevenSegment<A, B, C, D, E, F, G, Common> {
        SevenSegment {
            common: Default::default(),
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            g: self.g,
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
pub struct SevenSegment<A, B, C, D, E, F, G, Common> {
    common: core::marker::PhantomData<Common>,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
}

impl<A, B, C, D, E, F, G, Common> SevenSegment<A, B, C, D, E, F, G, Common>
where
    A: OutputPin,
    B: OutputPin<Error = A::Error>,
    C: OutputPin<Error = A::Error>,
    D: OutputPin<Error = A::Error>,
    E: OutputPin<Error = A::Error>,
    F: OutputPin<Error = A::Error>,
    G: OutputPin<Error = A::Error>,
    Common: Polarity,
{
    /// Sets the value of the display.
    ///
    /// The valid values are 0-9. In case of invalid value, the display will be blank.
    pub fn set(&mut self, value: u8) -> Result<(), A::Error> {
        let mask = match value {
            //      a  b  c  d  e  f  g
            0x0 => (1, 1, 1, 1, 1, 1, 0),
            0x1 => (0, 1, 1, 0, 0, 0, 0),
            0x2 => (1, 1, 0, 1, 1, 0, 1),
            0x3 => (1, 1, 1, 1, 0, 0, 1),
            0x4 => (0, 1, 1, 0, 0, 1, 1),
            0x5 => (1, 0, 1, 1, 0, 1, 1),
            0x6 => (1, 0, 1, 1, 1, 1, 1),
            0x7 => (1, 1, 1, 0, 0, 0, 0),
            0x8 => (1, 1, 1, 1, 1, 1, 1),
            0x9 => (1, 1, 1, 1, 0, 1, 1),
            0xa => (1, 1, 1, 0, 1, 1, 1),
            0xb => (0, 0, 1, 1, 1, 1, 1),
            0xc => (1, 0, 0, 1, 1, 1, 0),
            0xd => (0, 1, 1, 1, 1, 0, 1),
            0xe => (1, 0, 0, 1, 1, 1, 1),
            0xf => (1, 0, 0, 0, 1, 1, 1),
            _ => (0, 0, 0, 0, 0, 0, 0),
        };

        if mask.0 == Common::is_cathode() as u8 {
            self.a.set_high()?;
        } else {
            self.a.set_low()?;
        }

        if mask.1 == Common::is_cathode() as u8 {
            self.b.set_high()?;
        } else {
            self.b.set_low()?;
        }

        if mask.2 == Common::is_cathode() as u8 {
            self.c.set_high()?;
        } else {
            self.c.set_low()?;
        }

        if mask.3 == Common::is_cathode() as u8 {
            self.d.set_high()?;
        } else {
            self.d.set_low()?;
        }

        if mask.4 == Common::is_cathode() as u8 {
            self.e.set_high()?;
        } else {
            self.e.set_low()?;
        }

        if mask.5 == Common::is_cathode() as u8 {
            self.f.set_high()?;
        } else {
            self.f.set_low()?;
        }

        if mask.6 == Common::is_cathode() as u8 {
            self.g.set_high()?;
        } else {
            self.g.set_low()?;
        }
        Ok(())
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
            type Error = core::convert::Infallible;

            fn set_high(&mut self) -> Result<(), Self::Error> {
                (*self).0 = 1;
                Ok(())
            }

            fn set_low(&mut self) -> Result<(), Self::Error> {
                (*self).0 = 0;
                Ok(())
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
            }
            .with_common_anode();

            seven_segment.set(digit).expect("unable to set digit");
        }

        assert_eq!(
            (
                a.inv(),
                b.inv(),
                c.inv(),
                d.inv(),
                e.inv(),
                f.inv(),
                g.inv()
            ),
            expected
        );

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
            }
            .with_common_cathode();

            seven_segment.set(digit).expect("unable to set digit");
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
    fn digit_a() {
        test_digit(0xa, (1, 1, 1, 0, 1, 1, 1));
    }
    #[test]
    fn digit_b() {
        test_digit(0xb, (0, 0, 1, 1, 1, 1, 1));
    }

    #[test]
    fn digit_c() {
        test_digit(0xc, (1, 0, 0, 1, 1, 1, 0));
    }

    #[test]
    fn digit_d() {
        test_digit(0xd, (0, 1, 1, 1, 1, 0, 1));
    }

    #[test]
    fn digit_e() {
        test_digit(0xe, (1, 0, 0, 1, 1, 1, 1));
    }

    #[test]
    fn digit_f() {
        test_digit(0xf, (1, 0, 0, 0, 1, 1, 1));
    }

    #[test]
    fn digit_invalid() {
        test_digit(0x10, (0, 0, 0, 0, 0, 0, 0));
    }
}
