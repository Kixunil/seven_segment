Seven segment
=============

Simple Rust driver for 7-segment displays

About
-----

This is a driver (encoder) for 7-segment displays. It's implemented on top of embedded-hal, so you can use it on any platform that has pins with `embedded_hal::OutputPin` implemented.

The driver is very simple, only supports displays that connect directly using seven pins such as [SA52-11EWA](http://www.kingbrightusa.com/images/catalog/SPEC/SA52-11EWA.pdf) and doesn't try to do anything clever like setting all pins at once. It supports both common anode and common cathode displays.

License
-------
MITNFA
