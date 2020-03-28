
number2name (Rust library)
===============================

This is an extremely simple Rust library to convert an index to an accompanying name that is as short as possible.

It can convert from index to string and back, for different character sets.

Example
-------------------------------

For a character set 'abc', the series is::

    0 a
    1 b
    2 c
    3 aa
    4 ab
    ...
    11 cb
    12 cc
    13 aaa
    14 aab

Note that this is slightly different from base3 with leading characters stripped::

    0 a (or '')
    1 b
    2 c
    3 ba
    4 bb
    ...
    11 bac
    12 bba
    13 bbb
    14 bbc

