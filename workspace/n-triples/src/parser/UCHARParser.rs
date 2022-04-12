// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct UCHARParser;

impl UCHARParser
{
	#[inline(always)]
	pub(super) fn parse_UCHAR4(remaining_bytes: &mut &[u8]) -> Result<char, UCHARParseError>
	{
		const length: usize = 4;
		Self::parse_UCHAR::<_, length>(remaining_bytes, |bytes| Ok(Self::hex_digit::<0, _>(bytes)? | Self::hex_digit::<1, _>(bytes)? | Self::hex_digit::<2, _>(bytes)? | Self::hex_digit::<3, _>(bytes)?))
	}
	
	#[inline(always)]
	pub(super) fn parse_UCHAR8(remaining_bytes: &mut &[u8]) -> Result<char, UCHARParseError>
	{
		const length: usize = 8;
		Self::parse_UCHAR::<_, length>(remaining_bytes, |bytes| Ok(Self::hex_digit::<0, _>(bytes)? | Self::hex_digit::<1, _>(bytes)? | Self::hex_digit::<2, _>(bytes)? | Self::hex_digit::<3, _>(bytes)? | Self::hex_digit::<4, _>(bytes)? | Self::hex_digit::<5, _>(bytes)? | Self::hex_digit::<6, _>(bytes)? | Self::hex_digit::<7, _>(bytes)?))
	}
	
	#[inline(always)]
	fn parse_UCHAR<PHD: FnOnce(&[u8; length]) -> Result<u32, UCHARParseError>, const length: usize>(remaining_bytes: &mut &[u8], parse_hex_digits: PHD) -> Result<char, UCHARParseError>
	{
		{
			let expected_length = length as usize;
			let actual_length = remaining_bytes.len();
			if actual_length < expected_length
			{
				return Err(UCHARParseError::TooFewBytesRemain { expected_length, actual_length })
			}
		}
		
		let bytes = *remaining_bytes;
		
		let bytes_pointer = bytes.as_ptr().cast::<[u8; length]>();
		let value = parse_hex_digits(unsafe { &*bytes_pointer })?;
		
		let character = char::try_from(value)?;
		*remaining_bytes = bytes.get_unchecked_range_safe(length .. );
		Ok(character)
	}
	
	#[inline(always)]
	fn hex_digit<const index: usize, const length: usize>(bytes: &[u8; length]) -> Result<u32, UCHARParseError>
	{
		debug_assert!(index < length);
		
		let potential_hex_digit = bytes.get_unchecked_value_safe(index);
		let correction = match potential_hex_digit
		{
			_0 ..= _9 => _0,
			
			A ..= F => A,
			
			a ..= f => a,
			
			_ => return Err(UCHARParseError::InvalidHexDigit(potential_hex_digit)),
		};
		let value = potential_hex_digit - correction;
		
		let shift = 4 * (length - index - 1);
		Ok((value as u32) << (shift as u32))
	}
}
