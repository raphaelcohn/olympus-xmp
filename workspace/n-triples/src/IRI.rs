// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Represents an `IRIREF`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IRI<'a>(Cow<'a, str>);

impl<'a> const From<String> for IRI<'a>
{
	#[inline(always)]
	fn from(string: String) -> Self
	{
		Self(Cow::Owned(string))
	}
}

impl<'a> const From<&'a str> for IRI<'a>
{
	#[inline(always)]
	fn from(string: &'a str) -> Self
	{
		Self(Cow::Borrowed(string))
	}
}

impl IRI<'static>
{
	const Simple: Self = Self(Cow::Borrowed("http://www.w3.org/2001/XMLSchema#string"));
}

impl<'a> IRI<'a>
{
	#[inline(always)]
	fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, IRIParseError>
	{
		use IRIParseError::*;
		
		let mut string = StringSoFar::initial(remaining_bytes);
		
		loop
		{
			match decode_next_utf8(remaining_bytes)?.ok_or(DidNotExpectEndParsingBody)?
			{
				'>' => break,
				
				invalid @ (x00 ..= x20 | '<' | '"' | '{' | '}' | '|' | '`') => return Err(InvalidCharacter(invalid)),
				
				'\\' => match get_0(remaining_bytes).ok_or(EndOfFileParsingEscapeSequence)?
				{
					u => string.push_forcing_heap_UCHAR4(remaining_bytes).map_err(InvalidUCHAR4EscapeSequence)?,
					
					U => string.push_forcing_heap_UCHAR8(remaining_bytes).map_err(InvalidUCHAR8EscapeSequence)?,
					
					invalid => return Err(InvalidEscapeSequence(invalid)),
				}
				
				character @ _ => string.push(character)?,
			}
		}
		Ok(constructor(Self(string.to_cow())))
	}
}
