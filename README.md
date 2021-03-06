
number2name
===============================

This is a simple Rust library to convert an index to a name that is as short as possible.

It can convert from index to string and back, for different character sets.

Example
-------------------------------

From number to text:

    let charset = Charset::case_insensitive("abc");
    let text = charset.encode(13);
    assert_eq!(text, "aab");
    
From text to number:
    
    let nr = charset.decode("aab")?;
    assert_eq!(nr, 13);

Install
-------------------------------

`Cargo.toml`:

    number2name = "^1.0.1"

Encoding
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

Or use one of the built-in ones (make sure to import from this library, e.g. `use ::number2name::BASE64`):

* **HEX** (case-insensitive) / **HEXLOWERCASE** (case-sensitive)

    0123456789abcdef
    
* **BASE32** (case-insensitive) / **BASE32LOWERCASE** (case-sensitive)

    ABCDEFGHIJKLMNOPQRSTUVWXYZ234567
    
* **BASE32HUMAN**

    abcdefghjkmnpqrstuvwxyz23456789
    
* **BASE32CROCKFORD**

    0123456789ABCDEFGHJKMNPQRSTVWXYZ
    
* **BASE32SCNY**

    一二三四五六七八九十鼠牛虎兔龍蛇马羊猴鸡狗猪凤北东南西中左右上下
    
* **BASE32HEX**

    0123456789ABCDEFGHIJKLMNOPQRSTUV
    
* **BASE64** (case-sensitive)

    ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
    
* **BASE64URL** (case-sensitive)

    ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_

Integer types
------------------------------- 

* The default method works on 64 bit unsigned integers.
* Specific unsigned types are available as e.g. `charset.encode_u128(...)`. 
* Signed integer types are available as e.g. `charset.encode_i32(...)`.

Binaries
-------------------------------

While primarily intended as a library, this project also comes with two binaries: `number2name` and `name2number`.

These can be given a series of numbers or strings respectively, and convert them in the matching strings or numbers.

You can specify a charset (with `-c`, default is base32 human readable). You can use signed numbers (with `-s`).

Docker
-------------------------------

Run the CLI utils with Docker:

    docker run --rm -it mverleg/number2name /number2name 8992605869540
    docker run --rm -it mverleg/number2name /name2number "hey_there"

To build the image yourself (instead of downloading from Dockerhub), clone the Github project and run:

    docker build -t mverleg/number2name .

This is also the easiest way to run the tests and lints, to verify that your version is okay.

Mini version
------------------------------- 

If filesize (or compile time) is important, you can build this libary without any dependencies using at all, using `--no-default-features`. This means the built-in charsets are not available, you will need to construct your own (described above). You also can't use the binaries this way, only the library.

Notes
-------------------------------

* It's generally pretty fast, but if performance is critical:

  - Re-use the Charset instances (automatic for built-in ones)
  - Use case-sensitive character sets where possible

