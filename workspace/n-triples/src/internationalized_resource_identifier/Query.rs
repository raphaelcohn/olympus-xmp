// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A query.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Query<'a>(Cow<'a, str>);

impl<'a> const FromUnchecked<Cow<'a, str>> for Query<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const Into<Cow<'a, str>> for Query<'a>
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
	fn parse(remaining_utf8_bytes: &'a [u8]) -> Result<(Self, Option<&'a [u8]>), QueryParseError>
	{
		use QueryParseError::InvalidCharacterInQuery;
		
		let remaining_utf8_bytes = &mut remaining_utf8_bytes;
		let mut string = StringSoFar::new_stack(remaining_utf8_bytes);
		
		let hash_fragment_remaining_utf8_bytes = loop
		{
			use Utf8CharacterLength::*;
			match StringSoFar::decode_next_utf8_validity_already_checked(remaining_utf8_bytes)
			{
				None => break None,
				
				Some(character) => match character
				{
					HashChar => break Some(*remaining_utf8_bytes),
					
					ipchar_iunreserved_without_ucschar!() => string.push(character, One)?,
					ipchar_iunreserved_with_ucschar_2!()  => string.push(character, Two)?,
					ipchar_iunreserved_with_ucschar_3!()  => string.push(character, Three)?,
					ipchar_iunreserved_with_ucschar_4!()  => string.push(character, Four)?,
					ipchar_pct_encoded!()                 => string.push_forcing_heap_percent_encoded(remaining_utf8_bytes)?,
					ipchar_sub_delims!()                  => string.push(character, One)?,
					ipchar_other!()                       => string.push(character, One)?,
					iprivate_3!()                         => string.push(character, Three)?,
					iprivate_4!()                         => string.push(character, Four)?,
					SlashChar | QuestionMarkChar          => string.push(character, One)?,
					
					_ => return Err(InvalidCharacterInQuery(character)),
				},
			}
		};
		
		Ok((Self(string.to_cow()), hash_fragment_remaining_utf8_bytes))
	}
}
