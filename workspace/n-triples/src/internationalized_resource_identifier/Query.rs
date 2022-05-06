// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A query.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Query<'a>(Cow<'a, str>);

impl<'a> PercentEncodable<'a> for Query<'a>
{
	#[inline(always)]
	fn as_str(&self) -> &str
	{
		self.0.as_ref()
	}
	
	#[inline(always)]
	fn percent_encode_ascii(ascii_byte: u8) -> bool
	{
		match ascii_byte
		{
			A ..= Z  | a ..= z | _0 ..= _9 | Hyphen | Period | Underscore | Tilde => false,
			
			ExclamationMark | DollarSign | Ampersand | Apostrophe | OpenRoundBracket | CloseRoundBracket | Asterisk | PlusSign | Comma | Semicolon | EqualsSign => false,
			
			Colon | AtSign => false,
			
			Slash | QuestionMark => false,
			
			_ => true,
		}
	}
}

impl<'a> Display for Query<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		self.display_fmt(f)
	}
}

impl<'a> TryToOwnInPlace for Query<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for Query<'a>
{
	type TryToOwned = Query<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> TryToOwn for Option<Query<'a>>
{
	type TryToOwned = Option<Query<'static>>;
	
	#[inline(always)]
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>
	{
		if let Some(value) = self
		{
			Ok(Some(value.try_to_own()?))
		}
		else
		{
			Ok(None)
		}
	}
}

impl<'a> const FromUnchecked<Cow<'a, str>> for Query<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const FromUnchecked<&'a str> for Query<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self(Cow::Borrowed(value))
	}
}

impl<'a> const FromUnchecked<String> for Query<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self(Cow::Owned(value))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for Query<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for Query<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8; Count]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a> Into<Cow<'a, str>> for Query<'a>
{
	#[inline(always)]
	fn into(self) -> Cow<'a, str>
	{
		self.0
	}
}

impl<'a> const Borrow<str> for Query<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const AsRef<str> for Query<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const Deref for Query<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a> Query<'a>
{
	/// `iquery   = *( ipchar / iprivate / "/" / "?" )`.
	/// `iprivate = %xE000-F8FF / %xF0000-FFFFD / %x100000-10FFFD`.
	#[inline(always)]
	fn parse(mut remaining_string: &'a str, scheme_specific_parsing_rule: &SchemeSpecificParsingRule) -> Result<(Self, Option<&'a [u8]>), QueryParseError>
	{
		use QueryParseError::*;
		
		let remaining_string = &mut remaining_string;
		
		if scheme_specific_parsing_rule.query_should_not_be_present(remaining_string)
		{
			return Err(QueryNotAllowedForScheme)
		}
		
		let mut string = StringSoFar::new_stack(remaining_string);
		
		let hash_fragment_remaining_utf8_bytes = loop
		{
			match remaining_string.decode_next_utf8_validity_already_checked()
			{
				None => break None,
				
				Some(Utf8SequenceAndCharacter(utf8_sequence, character)) => match character
				{
					HashChar => break Some(*remaining_string),
					
					ipchar_iunreserved_without_ucschar!() => string.push_ascii_character(character)?,
					ipchar_iunreserved_with_ucschar_2!()  => string.push_utf8_sequence_enum_2(utf8_sequence)?,
					ipchar_iunreserved_with_ucschar_3!()  => string.push_utf8_sequence_enum_3(utf8_sequence)?,
					ipchar_iunreserved_with_ucschar_4!()  => string.push_utf8_sequence_enum_4(utf8_sequence)?,
					ipchar_pct_encoded!()                 => string.push_forcing_heap_percent_encoded::<false>(remaining_string)?,
					ipchar_sub_delims!()                  => string.push_ascii_character(character)?,
					ipchar_other!()                       => string.push_ascii_character(character)?,
					iprivate_3!()                         => string.push_utf8_sequence_enum_3(utf8_sequence)?,
					iprivate_4!()                         => string.push_utf8_sequence_enum_4(utf8_sequence)?,
					SlashChar | QuestionMarkChar          => string.push_ascii_character(character)?,
					
					_ => return Err(InvalidCharacterInQuery(character)),
				},
			}
		};
		
		Ok((Self(string.to_cow()), hash_fragment_remaining_utf8_bytes))
	}
}
