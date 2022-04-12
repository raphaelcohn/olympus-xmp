// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BytesByteProvider;

impl ByteProvider for BytesByteProvider
{
	type Error = Infallible;
	
	const OneSliceLength: NonZeroUsize = new_non_zero_usize(1);
	
	const TwoSliceLength: NonZeroUsize = new_non_zero_usize(2);
	
	const ThreeSliceLength: NonZeroUsize = new_non_zero_usize(3);
	
	const FourSliceLength: NonZeroUsize = new_non_zero_usize(4);
	
	#[inline(always)]
	fn two(bytes: &[u8]) -> Result<u32, Self::Error>
	{
		Ok
		(
			Self::get_byte::<1>(bytes)
		)
	}
	
	#[inline(always)]
	fn three(bytes: &[u8]) -> Result<(u32, u32), Self::Error>
	{
		Ok
		(
			(
				Self::get_byte::<1>(bytes),
				Self::get_byte::<2>(bytes),
			)
		)
	}
	
	#[inline(always)]
	fn four(bytes: &[u8]) -> Result<(u32, u32, u32), Self::Error>
	{
		Ok
		(
			(
				Self::get_byte::<1>(bytes),
				Self::get_byte::<2>(bytes),
				Self::get_byte::<3>(bytes),
			)
		)
	}
}

impl BytesByteProvider
{
	#[inline(always)]
	fn decode_next_utf8_validity_already_checked(remaining_utf8_bytes: &mut &[u8]) -> Option<(char, Utf8CharacterLength)>
	{
		let first = Self::dereference(Self::first(remaining_utf8_bytes)?);
		
		Some(Self::decode_internal_utf8_validity_already_checked(first, remaining_utf8_bytes))
	}
	
	#[inline(always)]
	fn decode_next_utf8(remaining_bytes: &mut &[u8]) -> Result<Option<(char, Utf8CharacterLength)>, InvalidUtf8ParseError<Infallible>>
	{
		let first = match Self::first(remaining_bytes)
		{
			None => return Ok(None),
			
			Some(pointer) => Self::dereference(pointer)
		};
		
		Ok(Some(Self::decode_internal(first, remaining_bytes)?))
	}
	
	#[inline(always)]
	fn first<'a>(remaining_bytes: &mut &'a [u8]) -> Option<&'a u8>
	{
		(*remaining_bytes).get(0)
	}
	
	#[inline(always)]
	fn dereference(pointer: &u8) -> u32
	{
		(*pointer) as u32
	}
	
	#[inline(always)]
	fn get_byte<const index: usize>(bytes: &[u8]) -> u32
	{
		debug_assert_ne!(index, 0);
		debug_assert!(index < 4);
		bytes.get_unchecked_value_safe(index) as u32
	}
}
