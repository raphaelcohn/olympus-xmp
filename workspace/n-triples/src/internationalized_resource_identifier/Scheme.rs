// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Scheme.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Scheme<'a>
{
	#[allow(missing_docs)]
	file,
	
	#[allow(missing_docs)]
	ftp,
	
	#[allow(missing_docs)]
	http,

	#[allow(missing_docs)]
	https,
	
	#[allow(missing_docs)]
	ws,

	#[allow(missing_docs)]
	wss,

	/// Will have been forced to be a lower case ASCII string.
	Unknown(Cow<'a, [u8]>),
}

impl<'a> TryFrom<&'a str> for Scheme<'a>
{
	type Error = SchemeParseError;
	
	#[inline(always)]
	fn try_from(value: &'a str) -> Result<Self, Self::Error>
	{
		Self::try_from(value.as_bytes())
	}
}

impl<'a> TryFrom<&'a [u8]> for Scheme<'a>
{
	type Error = SchemeParseError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		let (scheme, ..) = Self::parse(value)?;
		Ok(scheme)
	}
}

impl<'a> const FromUnchecked<&'a str> for Scheme<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self::from_unchecked(value.as_bytes())
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for Scheme<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		use Scheme::*;
		
		match value
		{
			b"file" => file,
			
			b"ftp" => ftp,
			
			b"http" => http,
			
			b"https" => https,
			
			b"ws" => ws,
			
			b"wss" => wss,
			
			_ => Unknown(Cow::Borrowed(value))
		}
	}
}

impl<'a> Display for Scheme<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		use Scheme::*;
		match self
		{
			file => write!(f, "file"),
			
			ftp => write!(f, "ftp"),
			
			http => write!(f, "http"),
			
			https => write!(f, "https"),
			
			ws => write!(f, "ws"),
			
			wss => write!(f, "wss"),
			
			Unknown(unknown) =>
			{
				let reference = unknown.as_ref();
				write!(f, "{}", unsafe { from_utf8_unchecked(reference) })
			},
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
	/// `IRI = scheme ":" ihier-part [ "?" iquery ] [ "#" ifragment ]`.
	/// `scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
	#[inline(always)]
	fn parse(mut bytes: &'a [u8]) -> Result<(Self, &SchemeSpecificParsingRule, &'a [u8]), SchemeParseError>
	{
		let remaining_bytes = &mut bytes;
		let string = Self::parse_first_character(remaining_bytes)?;
		let raw_scheme = Self::parse_subequent_characters(remaining_bytes, string)?;
		
		use Scheme::*;
		
		let (scheme, has_authority_and_absolute_path_with_dns_host_name) = match raw_scheme.as_ref()
		{
			b"file" => (file, &SchemeSpecificParsingRule::file),
			
			b"ftp" => (file, &SchemeSpecificParsingRule::ftp),
			
			b"http" => (http, &SchemeSpecificParsingRule::http),
			
			b"https" => (https, &SchemeSpecificParsingRule::https),
			
			b"ws" => (ws, &SchemeSpecificParsingRule::ws),
			
			b"wss" => (wss, &SchemeSpecificParsingRule::wss),
			
			_ => (Unknown(raw_scheme), &SchemeSpecificParsingRule::Unknown),
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
	fn parse_subequent_characters(remaining_bytes: &mut &'a [u8], mut string: StringSoFar<'a>) -> Result<Cow<'a, [u8]>, SchemeParseError>
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
		Ok(string.to_cow_bytes())
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
