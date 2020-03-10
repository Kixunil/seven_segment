Seven segment
=============

Simple Rust driver for 7-segment displays

About
-----

This is a driver (encoder) for 7-segment displays. It's implemented on top of embedded-hal, so you can use it on any platform that has pins with `embedded_hal::OutputPin` implemented.

The driver is very simple, only supports displays that connect directly using seven pins such as [SA52-11EWA](http://www.kingbrightusa.com/images/catalog/SPEC/SA52-11EWA.pdf) and doesn't try to do anything clever like setting all pins at once. It supports both common anode and common cathode displays.

Notes for upgrading from 0.1 to 0.2
-----------------------------------

While the change in `0.2` is strictly speaking Semver-breaking, the changes you
need to do should be trivial. The breaking difference is use of embedded HAL v2
API with fallible PIN operations. As a result, `set` method now returns
`Result<(), Error>` instead of `()`.

If you feed it pins implementing the old API, it still works thanks to the
blanket impl and the resulting `Error` will be uninhabited, so you can use
`.unwrap_or_else(|e| match e {})` to correctly avoid the unused result warning.

However, in order to use the full power of the new version, you should handle
the errors properly, e.g. using the `?` operator. The implementation requires
that `Error` types of all output pins are the same. If this is limiting to you,
you can use a wrapper for the pins to convert the errors into a single type.
(e.g. an enum)

License
-------
MITNFA
