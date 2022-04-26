// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct BytesByteProvider;

impl ByteProvider for BytesByteProvider
{
	type Error = Infallible;
	
	const OneSliceLength: NonZeroUsize = new_non_zero_usize(1);
	
	const TwoSliceLength: NonZeroUsize = new_non_zero_usize(2);
	
	const ThreeSliceLength: NonZeroUsize = new_non_zero_usize(3);
	
	const FourSliceLength: NonZeroUsize = new_non_zero_usize(4);
	
	#[inline(always)]
	fn first(bytes: &[u8]) -> Result<u8, InvalidUtf8ParseError<Self::Error>>
	{
		Ok(Self::get_byte::<{One}>(bytes))
	}
	
	#[inline(always)]
	fn two(bytes: &[u8]) -> Result<u8, Self::Error>
	{
		Ok
		(
			Self::get_byte::<{Two}>(bytes)
		)
	}
	
	#[inline(always)]
	fn three(bytes: &[u8]) -> Result<(u8, u8), Self::Error>
	{
		Ok
		(
			(
				Self::get_byte::<{Two}>(bytes),
				Self::get_byte::<{Three}>(bytes),
			)
		)
	}
	
	#[inline(always)]
	fn four(bytes: &[u8]) -> Result<(u8, u8, u8), Self::Error>
	{
		Ok
		(
			(
				Self::get_byte::<{Two}>(bytes),
				Self::get_byte::<{Three}>(bytes),
				Self::get_byte::<{Four}>(bytes),
			)
		)
	}
}

impl BytesByteProvider
{
	#[inline(always)]
	pub(super) fn decode_next_utf8_validity_already_checked(remaining_utf8_bytes: &mut &[u8]) -> Option<(char, Utf8SequenceEnum)>
	{
		let bytes = *remaining_utf8_bytes;
		
		if bytes.is_empty()
		{
			return None
		}
		
		let (character, utf8_sequence, remaining_bytes) = Self::decode_internal_utf8_validity_already_checked(bytes);
		*remaining_utf8_bytes = remaining_bytes;
		Some((character, utf8_sequence))
	}
	
	#[inline(always)]
	pub(super) fn decode_next_utf8(remaining_utf8_bytes: &mut &[u8]) -> Result<Option<(char, Utf8CharacterLength)>, InvalidUtf8ParseError<Infallible>>
	{
		let bytes = *remaining_utf8_bytes;
		
		if bytes.is_empty()
		{
			return Ok(None)
		}
		
		let (character, utf8_character_length, remaining_bytes) = Self::decode_internal(*remaining_utf8_bytes)?;
		*remaining_utf8_bytes = remaining_bytes;
		Ok(Some((character, utf8_character_length)))
	}
	
	#[inline(always)]
	fn get_byte<const decoded_byte_number: Utf8CharacterLength>(bytes: &[u8]) -> u8
	{
		let index: usize = decoded_byte_number.into() - 1;
		debug_assert!(index < bytes.len());
		
		bytes.get_unchecked_value_safe(index)
	}
}
