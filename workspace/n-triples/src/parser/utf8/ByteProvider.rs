// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait ByteProvider where InvalidUtf8ParseError<<Self as ByteProvider>::Error>: From<<Self as ByteProvider>::Error>
{
	type Error: error::Error;
	
	const OneSliceLength: NonZeroUsize;
	
	const TwoSliceLength: NonZeroUsize;
	
	const ThreeSliceLength: NonZeroUsize;
	
	const FourSliceLength: NonZeroUsize;
	
	/// "`remaining_utf8_bytes` is so positioned that `first` is still the first `u8` in it."
	#[inline(always)]
	fn decode_internal_utf8_validity_already_checked(first: u32, remaining_utf8_bytes: &mut &[u8]) -> (char, Utf8CharacterLength)
	{
		let bytes = Self::preamble(first, remaining_utf8_bytes);
		
		let (utf8_character_length, slice_length, raw_unicode_point) = if is_one(first)
		{
			Self::one_(first)
		}
		else if is_two(first)
		{
			let two = Self::two_(first, remaining_utf8_bytes);
			unsafe { two.unwrap_unchecked() }
		}
		else if is_three(first)
		{
			let three = Self::three_(first, remaining_utf8_bytes);
			unsafe { three.unwrap_unchecked() }
		}
		else
		{
			debug_assert!(is_four(first));
			let four = Self::four_(first, remaining_utf8_bytes);
			unsafe { four.unwrap_unchecked() }
		};
		
		let character = unsafe { char::from_u32_unchecked(raw_unicode_point) };
		Self::postamble(remaining_utf8_bytes, bytes, slice_length, character, utf8_character_length)
	}
	
	/// "`remaining_bytes` is so positioned that `first` is still the first `u8` in it."
	#[inline(always)]
	fn decode_internal(first: u32, remaining_bytes: &mut &[u8]) -> Result<(char, Utf8CharacterLength), InvalidUtf8ParseError<Self::Error>>
	{
		use InvalidUtf8ParseError::*;
		
		let bytes = Self::preamble(first, remaining_bytes);
		
		let (utf8_character_length, slice_length, raw_unicode_point) = if is_one(first)
		{
			// There is no need to call `Self::guard_slice_length()` as it must be valid.
			Self::one_(first)
		}
		else if is_two(first)
		{
			const SliceLength: NonZeroUsize = <Self as ByteProvider>::TwoSliceLength;
			Self::guard_slice_length::<SliceLength>(bytes, DidNotExpectEndParsingTwoByteUtf8Character)?;
			Self::two_(first, remaining_bytes)?
		}
		else if is_three(first)
		{
			const SliceLength: NonZeroUsize = <Self as ByteProvider>::ThreeSliceLength;
			Self::guard_slice_length::<SliceLength>(bytes, DidNotExpectEndParsingThreeByteUtf8Character)?;
			Self::three_(first, remaining_bytes)?
		}
		else if is_four(first)
		{
			const SliceLength: NonZeroUsize = <Self as ByteProvider>::FourSliceLength;
			Self::guard_slice_length::<SliceLength>(bytes, DidNotExpectEndParsingFourByteUtf8Character)?;
			Self::four_(first, remaining_bytes)?
		}
		else
		{
			return Err(InvalidStartToUtf8Sequence)
		};
		
		let character = char::try_from(raw_unicode_point).map_err(InvalidUtf8CodePoint)?;
		Ok(Self::postamble(remaining_bytes, bytes, slice_length, character, utf8_character_length))
	}
	
	fn two(bytes: &[u8]) -> Result<u32, Self::Error>;
	
	fn three(bytes: &[u8]) -> Result<(u32, u32), Self::Error>;
	
	fn four(bytes: &[u8]) -> Result<(u32, u32, u32), Self::Error>;
	
	#[inline(always)]
	fn guard_slice_length<const SliceLength: NonZeroUsize>(bytes: &[u8], error: InvalidUtf8ParseError<Self::Error>) -> Result<(), InvalidUtf8ParseError<Self::Error>>
	{
		if bytes.len() < SliceLength.get()
		{
			Err(error)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn one_(first: u32) -> (Utf8CharacterLength, NonZeroUsize, u32)
	{
		(
			One,
			Self::OneSliceLength,
			first,
		)
	}
	
	#[inline(always)]
	fn two_(first: u32, remaining_bytes: &mut &[u8]) -> Result<(Utf8CharacterLength, NonZeroUsize, u32), Self::Error>
	{
		let second = Self::two(remaining_bytes)?;
		Ok
		(
			(
				Two,
				Self::TwoSliceLength,
				(first & x1F) << Shift6 | second,
			)
		)
	}
	
	#[inline(always)]
	fn three_(first: u32, remaining_bytes: &mut &[u8]) -> Result<(Utf8CharacterLength, NonZeroUsize, u32), Self::Error>
	{
		let (second, third) = Self::three(remaining_bytes)?;
		Ok
		(
			(
				Three,
				Self::ThreeSliceLength,
				(first & x0F) << Shift12 | second << Shift6 | third,
			)
		)
	}
	
	#[inline(always)]
	fn four_(first: u32, remaining_bytes: &mut &[u8]) -> Result<(Utf8CharacterLength, NonZeroUsize, u32), Self::Error>
	{
		let (second, third, fourth) = Self::four(remaining_bytes)?;
		Ok
		(
			(
				Four,
				Self::FourSliceLength,
				(first & x07) << Shift18 | second << Shift12 | third << Shift6 | fourth,
			)
		)
	}
	
	#[inline(always)]
	fn preamble<'a>(first: u32, remaining_bytes: &mut &'a [u8]) -> &'a [u8]
	{
		debug_assert_ne!(remaining_bytes.len(), 0, "remaining_utf8_bytes is empty");
		debug_assert_eq!(first, remaining_bytes.get_unchecked_value_safe(0) as u32, "remaining_utf8_bytes does not have first at index 0");
		
		*remaining_bytes
	}
	
	#[inline(always)]
	fn postamble<'a>(remaining_bytes: &mut &'a [u8], bytes: &'a [u8], slice_length: NonZeroUsize, character: char, utf8_character_length: Utf8CharacterLength) -> (char, Utf8CharacterLength)
	{
		*remaining_bytes = bytes.get_unchecked_range_safe(slice_length.get() .. );
		(character, utf8_character_length)
	}
}

const xE0: u32 = 0xE0;

const xF0: u32 = 0xF0;

#[inline(always)]
const fn is_one(first: u32) -> bool
{
	first & 0x80 == 0x00
}

#[inline(always)]
const fn is_two(first: u32) -> bool
{
	first & xE0 == 0xC0
}

#[inline(always)]
const fn is_three(first: u32) -> bool
{
	first & xF0 == xE0
}

#[inline(always)]
const fn is_four(first: u32) -> bool
{
	first & 0xF8 == xF0
}
