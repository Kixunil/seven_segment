Seven segment
=============

Simple Rust driver for 7-segment displays

About
-----

This is a driver (encoder) for 7-segment displays. It's implemented on top of embedded-hal, so you can use it on any platform that has pins with `embedded_hal::OutputPin` implemented.

The driver is very simple, only supports displays that connect directly using seven pins such as [SA52-11EWA](http://www.kingbrightusa.com/images/catalog/SPEC/SA52-11EWA.pdf) and doesn't try to do anything clever like setting all pins at once. It supports both common anode and common cathode displays.

Semver trick
------------

This branch is an imperfect Semver trick bridging versions `0.2.1` and `0.1.1`.
It's imperfect because not all types could have been re-exported due to
inherent merhods.

It allows you to convert the `SevenSegment` struct between versions, of the
crate. It also reduces the code size by removing duplication in case of two
different versions. It's still perfectly compatible with `0.1`. (If not, file a
bug report.)

License
-------
MITNFA
