// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn decode_next_utf8(remaining_bytes: &mut &[u8]) -> Result<Option<char>, InvalidUtf8ParseError>
{
	use InvalidUtf8ParseError::*;
	
	#[inline(always)]
	fn get_byte<const index: usize>(bytes: &[u8]) -> u32
	{
		debug_assert_ne!(index, 0);
		debug_assert!(index < 4);
		bytes.get_unchecked_value_safe(index) as u32
	}
	
	let bytes = *remaining_bytes;
	
	let a = match bytes.get(0)
	{
		None => return Ok(None),
		
		Some(pointer) => (*pointer) as u32
	};
	
	let (raw_unicode_point, count) = if a & 0x80 == 0x00
	{
		const Count: usize = 1;
		(
			a,
			Count
		)
	}
	else if a & 0xE0 == 0xC0
	{
		const Count: usize = 2;
		if bytes.len() < Count
		{
			return Err(DidNotExpectEndParsingUtf8Character)
		}
		let b = get_byte::<1>(bytes);
		(
			(a & 0x1F) << 6 | b,
			Count
		)
	}
	else if a & 0xF0 == 0xE0
	{
		const Count: usize = 3;
		if bytes.len() < Count
		{
			return Err(DidNotExpectEndParsingUtf8Character)
		}
		let b = get_byte::<1>(bytes);
		let c = get_byte::<2>(bytes);
		(
			(a & 0x0F) << 12 | b << 6 | c,
			Count
		)
	}
	else if a & 0xF8 == 0xF0
	{
		const Count: usize = 4;
		if bytes.len() < Count
		{
			return Err(DidNotExpectEndParsingUtf8Character)
		}
		let b = get_byte::<1>(bytes);
		let c = get_byte::<2>(bytes);
		let d = get_byte::<3>(bytes);
		(
			(a & 0x07) << 18 | b << 12 | c << 6 | d,
			Count
		)
	}
	else
	{
		return Err(InvalidStartToUtf8Sequence)
	};
	
	let character = char::try_from(raw_unicode_point).map_err(InvalidUtf8CodePoint)?;
	*remaining_bytes = bytes.get_unchecked_range_safe(count .. );
	Ok(Some(character))
}
