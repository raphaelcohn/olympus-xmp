// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Can be empty; can start with a digit.
///
/// Will have been forced to be lower case.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HostName<'a>(Cow<'a, str>);

impl<'a> const FromUnchecked<Cow<'a, str>> for HostName<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const Into<Cow<'a, str>> for HostName<'a>
{
	#[inline(always)]
	fn into(self) -> Cow<'a, str>
	{
		self.0
	}
}

impl<'a> const Borrow<str> for HostName<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const AsRef<str> for HostName<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const Deref for HostName<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a> HostName<'a>
{
	/// `ireg-name = *( iunreserved / pct-encoded / sub-delims )`.
	#[inline(always)]
	fn parse(mut remaining_utf8_bytes: &[u8]) -> Result<(Self, &[u8]), HostNameParseError>
	{
		use HostNameParseError::*;
		
		let remaining_utf8_bytes = &mut remaining_utf8_bytes;
		let mut string = StringSoFar::new_stack(remaining_utf8_bytes);
		
		let port_bytes_including_colon = loop
		{
			let character = match StringSoFar::decode_next_utf8_validity_already_checked(remaining_utf8_bytes)
			{
				None => break b"",
				
				Some(character) => character,
			};
			
			use Utf8CharacterLength::*;
			
			match character
			{
				ColonChar                                                                         => break unsafe { from_raw_parts((*remaining_utf8_bytes).rewind_buffer(), 1 + remaining_utf8_bytes.len()) },
				
				AChar ..= ZChar                                                                   => string.push_forcing_heap_ascii_to_lower_case(character)?,
				aChar ..= zChar | DIGIT!() | HyphenChar | PeriodChar | UnderscoreChar | TildeChar => string.push(character, One),
				iunreserved_with_ucschar_2!()                                                     => string.push(character, Two),
				iunreserved_with_ucschar_3!()                                                     => string.push(character, Three),
				iunreserved_with_ucschar_4!()                                                     => string.push(character, Four),
				pct_encoded!()                                                                    => string.push_forcing_heap_percent_encoded(remaining_utf8_bytes)?,
				sub_delims!()                                                                     => string.push(character, One),
				
				_ => Err(InvalidCharacterInHostName(character)),
			}
		};
		
		Ok((Self(string.to_cow()), port_bytes_including_colon))
	}
}

impl HostName<'static>
{
	/// `wwww.w3.org`.
	pub const www_w3_org: Self = unsafe { Self::from_unchecked("www.w3.org") };
	
	const Empty: Self = HostName(Cow::Borrowed(""));
}