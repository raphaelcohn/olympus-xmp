// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Scheme.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Scheme<'a>
{
	#[allow(missing_docs)]
	http,

	#[allow(missing_docs)]
	https,

	#[allow(missing_docs)]
	file,
	
	/// Will have been forced to be lower case.
	Unknown(Cow<'a, str>),
}

impl<'a> Display for Scheme<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		use Scheme::*;
		match self
		{
			http => write!(f, "http"),
			
			https => write!(f, "https"),
			
			file => write!(f, "file"),
			
			Unknown(unknown) => write!(f, "{}", unknown.as_ref()),
		}
	}
}

impl<'a> TryToOwnInPlace for Scheme<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		if let Scheme::Unknown(cow) = self
		{
			cow.try_to_own_in_place()
		}
		else
		{
			Ok(())
		}
	}
}

impl<'a> TryToOwn for Scheme<'a>
{
	type TryToOwned = Scheme<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> Scheme<'a>
{
	/// Default port for known schemes.
	#[inline(always)]
	pub const fn default_port(&self) -> Option<NonZeroU16>
	{
		use Scheme::*;
		
		match self
		{
			http => Some(new_non_zero_u16(80)),
			
			https => Some(new_non_zero_u16(443)),
			
			file => None,
			
			Unknown(_) => None,
		}
	}
	
	/// `IRI = scheme ":" ihier-part [ "?" iquery ] [ "#" ifragment ]`.
	/// `scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
	#[inline(always)]
	fn parse(mut bytes: &'a [u8]) -> Result<(Self, bool, &'a [u8]), SchemeParseError>
	{
		let remaining_bytes = &mut bytes;
		let string = Self::parse_first_character(remaining_bytes)?;
		let raw_scheme = Self::parse_subequent_characters(remaining_bytes, string)?;
		
		use Scheme::*;
		
		let (scheme, has_authority_and_absolute_path_with_dns_host_name) = match raw_scheme.as_ref()
		{
			"http" => (http, true),
			
			"https" => (https, true),
			
			"file" => (file, true),
			
			_ => (Unknown(raw_scheme), false),
		};
		
		Ok((scheme, has_authority_and_absolute_path_with_dns_host_name, *remaining_bytes))
	}
	
	#[inline(always)]
	fn parse_first_character(remaining_bytes: &mut &'a [u8]) -> Result<StringSoFar<'a>, SchemeParseError>
	{
		use SchemeParseError::*;
		let mut string = StringSoFar::new_stack(remaining_bytes);
		
		match Self::next_byte(remaining_bytes, DidNotExpectEndParsingFirstCharacter)?
		{
			byte @ A ..= Z => Self::push_lower_case(&mut string, byte)?,
			
			byte @ a ..= z => string.push_ascii(byte as char)?,
			
			invalid @ _ => return Err(InvalidFirstCharacter(invalid))
		}
		
		Ok(string)
	}
	
	#[inline(always)]
	fn parse_subequent_characters(remaining_bytes: &mut &'a [u8], mut string: StringSoFar<'a>) -> Result<Cow<'a, str>, SchemeParseError>
	{
		use SchemeParseError::*;
		loop
		{
			match Self::next_byte(remaining_bytes, DidNotExpectEndParsingSubsequentCharacter)?
			{
				Colon => break,
				
				byte @ A ..= Z => Self::push_lower_case(&mut string, byte)?,
				
				byte @ (_0 ..= _9 | a ..= z | PlusSign | MinusSign | Period) => string.push_ascii(byte as char)?,
				
				invalid @ _ => return Err(InvalidSubsequentCharacter(invalid))
			}
		}
		Ok(string.to_cow())
	}
	
	#[inline(always)]
	fn push_lower_case(string: &mut StringSoFar<'a>, byte: u8) -> Result<(), SchemeParseError>
	{
		string.push_forcing_heap_ascii_to_lower_case(byte as char).map_err(SchemeParseError::OutOfMemoryMakingAsciiLowerCase)
	}
	
	#[inline(always)]
	fn next_byte(remaining_bytes: &mut &'a [u8], error: SchemeParseError) -> Result<u8, SchemeParseError>
	{
		get_0(remaining_bytes).ok_or(error)
	}
}
