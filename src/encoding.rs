//! This module contains traits to convert between your types in memory, and the data sled stores on disk.
//!
//! Unlike serde's Serialize and Deserialize traits, you would not implement these traits on your type `Foo` directly, but on a new type, like a `FooEncoding`.
//! This has two advantages:
//!
//! * You can specify multiple encodings per type by creating `FooJsonEncoding` and `FooMessagePackEncoding`.
//! * You cannot implement these traits on types like i32 or str due to orphan rules, but you can create new types `I32Encoding` or `StringEncoding`.
//!
//! For convenience, this module contains example encoders for integers and strings.
//!
//! TODO: add an example or testcase for serde etc.

#![allow(missing_copy_implementations)]

/// Converts your types into a bunch of bytes.
pub trait Encoder<'a> {
    /// The type you wish to insert into the database.
    type In;
    /// An intermediate type which derefs to a byte slice.
    /// This is usually a `Vec<u8>` or `[u8; n]`.
    /// If [`In`] can deref to a byte slice without conversion, this can be equal to [`In`].
    type Encoded: AsRef<[u8]>;

    /// Encode your data
    /// Encoding cannot fail.
    /// TODO: add an example showing fallible encoding with TryInto.
    fn encode(data: Self::In) -> Self::Encoded;
}

/// Converts a bunch of bytes into your type.
pub trait Decoder<'a> {
    /// The type you get when decoding.
    /// When implementing both [`Encoder`] and [`Decoder`], then `Encoder::In` is often a equivalent to (or a reference to) `Decoder::Out`.
    type Out;
    /// A possible error type when decoding fails.
    type Error;

    /// Decode a byte slice into your type.
    fn decode(bytes: &'a [u8]) -> Result<Self::Out, Self::Error>;
}

/// Encode strings as utf8
pub struct StringEncoding();
impl<'a> Encoder<'a> for StringEncoding {
    type In = &'a str;
    type Encoded = &'a [u8];

    fn encode(data: Self::In) -> Self::Encoded {
        data.as_bytes()
    }
}
impl<'a> Decoder<'a> for StringEncoding {
    type Out = &'a str;
    type Error = std::str::Utf8Error;

    fn decode(bytes: &'a [u8]) -> Result<Self::Out, Self::Error> {
        std::str::from_utf8(bytes)
    }
}

/// Encode an integer T in big endian. This ensures the expected sort order when uses for keys.
pub struct IntegerEncoding<T>(std::marker::PhantomData<T>);
// TODO: .to_be_bytes() isn't part of any trait. We need to write a macro for this.
impl<'a> Encoder<'a> for IntegerEncoding<u128> {
    type In = u128;
    type Encoded = [u8; std::mem::size_of::<Self::In>()];

    fn encode(data: Self::In) -> Self::Encoded {
        data.to_be_bytes()
    }
}
impl<'a> Decoder<'a> for IntegerEncoding<u128> {
    type Out = u128;
    type Error = ();

    fn decode(bytes: &'a [u8]) -> Result<Self::Out, Self::Error> {
        use std::convert::TryInto;
        let array = bytes.try_into().map_err(|_| ())?;
        Ok(u128::from_be_bytes(array))
    }
}
