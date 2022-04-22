// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Represents a `BLANK_NODE_LABEL`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BlankNodeLabel<'a>(Cow<'a, str>);

impl<'a> const FromUnchecked<Cow<'a, str>> for BlankNodeLabel<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const FromUnchecked<&'a str> for BlankNodeLabel<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self(Cow::Borrowed(value))
	}
}

impl<'a> const FromUnchecked<String> for BlankNodeLabel<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self(Cow::Owned(value))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for BlankNodeLabel<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for BlankNodeLabel<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8; Count]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a> TryToOwnInPlace for BlankNodeLabel<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for BlankNodeLabel<'a>
{
	type TryToOwned = BlankNodeLabel<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> BlankNodeLabel<'a>
{
	// `(PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?`.
	#[inline(always)]
	fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, BlankNodeLabelParseError>
	{
		use BlankNodeLabelParseError::*;
		if get_0(remaining_bytes).ok_or(DidNotExpectEndParsingColon)? != Colon
		{
			return Err(ExpectedColon)
		}
		
		let mut string = StringSoFar::new_stack(remaining_bytes);
		
		{
			// `(PN_CHARS_U | [0-9])`.
			// `PN_CHARS_U ::= PN_CHARS_BASE | '_' | ':'`.
			// `PN_CHARS_BASE ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6] | [#x00F8-#x02FF] |[#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]`.
			let (character, utf8_character_length) = decode_next_utf8(remaining_bytes)?.ok_or(DidNotExpectEndParsingFirstCharacterOfLabel)?;
			match character
			{
				'0' ..= '9' => string.push_ascii(character)?,
				
				'_' | ':' => string.push_ascii(character)?,
				
				'A' ..= 'Z' | 'a' ..= 'z' => string.push_ascii(character)?,
				
				x00C0..=x00D6 | x00D8..=x00F6 | x00F8..=x02FF | x0370..=x037D | x037F..=x1FFF | x200C..=x200D | x2070..=x218F | x2C00..=x2FEF | x3001..=xD7FF | xF900..=xFDCF | xFDF0..=xFFFD | x10000..=xEFFFF => string.push(character, utf8_character_length)?,
				
				_ => return Err(InvalidCharacter(character)),
			}
		}
		
		loop
		{
			// `((PN_CHARS | '.')* PN_CHARS)?`.
			// `(PN_CHARS | '.')`.
			// `PN_CHARS ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]`.
			// `PN_CHARS_U ::= PN_CHARS_BASE | '_' | ':'`.
			// `PN_CHARS_BASE ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6] | [#x00F8-#x02FF] |[#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]`.
			let (character, utf8_character_length) = decode_next_utf8(remaining_bytes)?.ok_or(DidNotExpectEndParsingSubsequentCharacterOfLabel)?;
			match character
			{
				// Whitespace terminates a blank node label.
				SpaceChar | TabChar =>
				{
					break
				}
				
				'.' => string.push_ascii(character)?,
				
				'-' | '0'..='9' => string.push_ascii(character)?,
				
				x00B7 | x0300..=x036F | x203F..=x2040 => string.push(character, utf8_character_length)?,
				
				'_' | ':' => string.push_ascii(character)?,
				
				'A'..='Z' | 'a'..='z' => string.push_ascii(character)?,
				
				x00C0..=x00D6 | x00D8..=x00F6 | x00F8..=x02FF | x0370..=x037D | x037F..=x1FFF | x200C..=x200D | x2070..=x218F | x2C00..=x2FEF | x3001..=xD7FF | xF900..=xFDCF | xFDF0..=xFFFD | x10000..=xEFFFF => string.push(character, utf8_character_length)?,
				
				_ => return Err(InvalidCharacter(character)),
			}
		}
		
		let cow = string.to_cow();
		
		{
			let string = cow.deref();
			let length = string.len();
			debug_assert_ne!(length, 0, "A label is always at least one byte");
			if length != 1
			{
				if string.as_bytes().get_unchecked_value_safe(length - 1) != Period
				{
					return Err(PeriodIsNotAllowedAsTheFinalCharacterOfABlankNodeLabel)
				}
			}
		}
		
		Ok(constructor(Self(cow)))
	}
}
