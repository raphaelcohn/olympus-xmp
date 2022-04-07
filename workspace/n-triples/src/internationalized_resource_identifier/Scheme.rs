// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Scheme<'a>
{
	http,
	
	https,
	
	Unknown(Cow<'a, str>),
}

impl<'a> Scheme<'a>
{
	/// `IRI = scheme ":" ihier-part [ "?" iquery ] [ "#" ifragment ]`.
	/// `scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
	#[inline(always)]
	fn parse(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), SchemeParseError>
	{
		use SchemeParseError::*;
		
		Self::parse_first_character(remaining_bytes)?;
		let index_of_colon = Self::parse_subequent_characters(remaining_bytes)?;
		
		use Scheme::*;
		let slice = bytes.get_unchecked_range_safe(0 .. index_of_colon);
		
		// TODO: We need to match case-insensitive.
		// Uggh..
		xxxx;
		let scheme = match slice
		{
			b"http" => http,
			
			b"https" => https,
			
			_ => Unknown(Cow::Borrowed(unsafe { from_utf8_unchecked(slice) })),
		};
		Ok((scheme, bytes.get_unchecked_range_safe((index_of_colon + 1) .. )))
	}
	
	#[inline(always)]
	fn parse_first_character(bytes: &'a [u8]) -> Result<(), SchemeParseError>
	{
		match Self::next_byte(bytes, 0)?
		{
			A ..= Z | a ..= z => Ok(()),
			
			invalid @ _ => Err(SchemeParseError::InvalidFirstCharacter(invalid))
		}
	}
	
	#[inline(always)]
	fn parse_subequent_characters(bytes: &'a [u8]) -> Result<usize, SchemeParseError>
	{
		let mut slice_length = 1;
		loop
		{
			match Self::next_byte(bytes, slice_length)?
			{
				Colon => break,
				
				_0 ..= _9 | A ..= Z | a ..= z | PlusSign | MinusSign | Period => slice_length += 1,
				
				invalid @ _ => return Err(SchemeParseError::InvalidSubsequentCharacter(invalid))
			}
		}
		Ok(slice_length)
	}
	
	#[inline(always)]
	fn next_byte(bytes: &'a [u8], index: usize) -> Result<u8, SchemeParseError>
	{
		let length = bytes.len();
		debug_assert!(index <= length);
		if length == index
		{
			Err(SchemeParseError::DidNotExpectEndParsingSubsequentCharacter)
		}
		else
		{
			Ok(bytes.get_unchecked_value_safe(index))
		}
	}
}
