
number2name (Rust library)
===============================

This is a simple Rust library to convert an index to a name that is as short as possible.

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
    10 cb
    11 cc
    12 aaa
    13 aab

Note that this is slightly shorter than base3 with leading characters stripped::

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

Character sets
-------------------------------

You can easily use your own character sets using `number2name::Charset`.

Or use one of the built-in ones:

* HEX: `0123456789abcdef`
* HEXLOWERCASE: `0123456789abcdef` (case-sensitive)
* BASE32: `ABCDEFGHIJKLMNOPQRSTUVWXYZ234567`
* BASE32LOWERCASE: `ABCDEFGHIJKLMNOPQRSTUVWXYZ234567` (case-sensitive)
* BASE32HUMAN: `abcdefghjkmnpqrstuvwxyz23456789_`
* BASE32CROCKFORD: `0123456789ABCDEFGHJKMNPQRSTVWXYZ`
* BASE32SCNY: `一二三四五六七八九十鼠牛虎兔龍蛇马羊猴鸡狗猪凤北东南西中左右上下`
* BASE32HEX: `0123456789ABCDEFGHIJKLMNOPQRSTUV`
* BASE64: `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/` (case-sensitive)
* BASE64URL: `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_` (case-sensitive)

Notes
-------------------------------

* It's generally pretty fast, but if performance is critical:

  - Re-use the Charset instances (automatic for built-in ones)
  - Use case-sensitive character sets where possible (_CS postfix)

