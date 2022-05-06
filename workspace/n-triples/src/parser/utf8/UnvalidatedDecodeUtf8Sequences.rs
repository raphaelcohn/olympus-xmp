// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An unvalidated slice of UTF-8 sequences; typicall as Rust `[u8]`.
pub trait UnvalidatedDecodeUtf8Sequences
{
	/// Decode the next UTF-8 sequence.
	fn decode_next_utf8(&mut self) -> Result<Option<Utf8SequenceAndCharacter>, InvalidUtf8ParseError<Infallible>>;
	
	/// Decode the next percent-encoded UTF-8 sequence.
	///
	/// Assumes that the initial `%` (percent sign) has already been removed from the `self` buffer.
	fn decode_next_percent_encoded_utf8<const to_ascii_lower_case: bool>(&mut self) -> Result<Utf8SequenceAndCharacter, InvalidUtf8ParseError<PercentDecodeError>>;
}

impl<'a> UnvalidatedDecodeUtf8Sequences for &'a str
{
	#[inline(always)]
	fn decode_next_utf8(&mut self) -> Result<Option<Utf8SequenceAndCharacter>, InvalidUtf8ParseError<Infallible>>
	{
		let as_bytes: &'a mut &[u8] = unsafe { transmute(self) };
		as_bytes.decode_next_utf8()
	}
	
	#[inline(always)]
	fn decode_next_percent_encoded_utf8<const to_ascii_lower_case: bool>(&mut self) -> Result<Utf8SequenceAndCharacter, InvalidUtf8ParseError<PercentDecodeError>>
	{
		let as_bytes: &'a mut &[u8] = unsafe { transmute(self) };
		as_bytes.decode_next_percent_encoded_utf8::<to_ascii_lower_case>()
	}
}

impl<'a> UnvalidatedDecodeUtf8Sequences for &'a [u8]
{
	#[inline(always)]
	fn decode_next_utf8(&mut self) -> Result<Option<Utf8SequenceAndCharacter>, InvalidUtf8ParseError<Infallible>>
	{
		let bytes = *remaining_utf8_bytes;
		
		if bytes.is_empty()
		{
			return Ok(None)
		}
		
		let (utf8_sequence_and_character, remaining_bytes) = BytesByteProvider::decode_internal::<false>(*remaining_utf8_bytes)?;
		*remaining_utf8_bytes = remaining_bytes;
		Ok(Some(utf8_sequence_and_character))
	}
	
	#[inline(always)]
	fn decode_next_percent_encoded_utf8<const to_ascii_lower_case: bool>(&mut self) -> Result<Utf8SequenceAndCharacter, InvalidUtf8ParseError<PercentDecodeError>>
	{
		let bytes = *remaining_percent_encoded_bytes;
		
		let (utf8_sequence_and_character, remaining_bytes) = PercentEncodedByteProvider::decode_internal::<to_ascii_lower_case>(bytes)?;
		*remaining_percent_encoded_bytes = remaining_bytes;
		Ok(utf8_sequence_and_character)
	}
}
