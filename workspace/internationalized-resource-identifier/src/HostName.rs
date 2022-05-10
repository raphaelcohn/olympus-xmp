// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Can be empty; can start with a digit.
///
/// Will have been forced to be lower case.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HostName<'a>(Cow<'a, str>);

impl<'a> PercentEncodable<'a> for HostName<'a>
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
			
			_ => true,
		}
	}
}

impl<'a> Display for HostName<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		self.display_fmt(f)
	}
}

impl<'a> TryToOwnInPlace for HostName<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for HostName<'a>
{
	type TryToOwned = HostName<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> const FromUnchecked<Cow<'a, str>> for HostName<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const FromUnchecked<&'a str> for HostName<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self(Cow::Borrowed(value))
	}
}

impl<'a> const FromUnchecked<String> for HostName<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self(Cow::Owned(value))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for HostName<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for HostName<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8; Count]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a> Into<Cow<'a, str>> for HostName<'a>
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
	fn parse<const to_ascii_lower_case: bool>(mut ihost_and_port_string: &'a str) -> Result<(Self, &'a [u8]), HostNameParseError>
	{
		use HostNameParseError::*;
		
		let remaining_utf8_bytes = &mut ihost_and_port_string;
		let mut string = Utf8SequencesParser::new_stack(remaining_utf8_bytes);
		
		use Utf8CharacterLength::*;
		let port_bytes_including_colon = loop
		{
			match remaining_utf8_bytes.decode_next_utf8_validity_already_checked()
			{
				None => break b"" as &[u8],
				
				#[allow(unused_qualifications)]
				Some(Utf8SequenceAndCharacter(utf8_sequence, character)) => match character
				{
					ColonChar                                                                         => break
					{
						let bytes = *remaining_utf8_bytes;
						let pointer = bytes.rewind_buffer(One).as_ptr() as *const u8;
						let length = One.add_from_str(bytes);
						unsafe { from_raw_parts(pointer, length) }
					},
					
					AChar ..= ZChar                                                                   => if to_ascii_lower_case
					{
						string.push_ascii_character(character)?
					}
					else
					{
						string.push_forcing_heap_ascii_character::<true>(character)?
					},
					
					aChar ..= zChar | DIGIT!() | HyphenChar | PeriodChar | UnderscoreChar | TildeChar => string.push_ascii_character(character)?,
					iunreserved_with_ucschar_2!()                                                     => string.push_utf8_sequence_enum_2(utf8_sequence)?,
					iunreserved_with_ucschar_3!()                                                     => string.push_utf8_sequence_enum_3(utf8_sequence)?,
					iunreserved_with_ucschar_4!()                                                     => string.push_utf8_sequence_enum_4(utf8_sequence)?,
					pct_encoded!()                                                                    => string.push_forcing_heap_percent_encoded::<HostNameParseError, false>(remaining_utf8_bytes)?,
					sub_delims!()                                                                     => string.push_ascii_character(character)?,
					
					_ => return Err(InvalidCharacterInHostName(character)),
				},
			}
		};
		
		Ok((Self(string.to_cow()), port_bytes_including_colon))
	}
}

impl HostName<'static>
{
	/// Empty host name.
	pub const Empty: Self = HostName(Cow::Borrowed(""));
	
	/// Localhost host name
	pub const localhost: Self = HostName(Cow::Borrowed("localhost"));
}
