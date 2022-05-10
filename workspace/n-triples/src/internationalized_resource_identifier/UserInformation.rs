// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// User information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserInformation<'a>(Cow<'a, str>);

impl<'a> Display for UserInformation<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.0.deref())
	}
}

impl<'a> TryToOwnInPlace for UserInformation<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for UserInformation<'a>
{
	type TryToOwned = UserInformation<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> UserInformation<'a>
{
	/// `iuserinfo      = *( iunreserved / pct-encoded / sub-delims / ":" )`
	/// `iunreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~" / ucschar`.
	#[inline(always)]
	fn parse(mut user_info_string: &'a str, scheme_specific_parsing_rule: &SchemeSpecificParsingRule) -> Result<Self, UserInformationParseError>
	{
		use UserInformationParseError::*;
		
		let remaining = &mut user_info_string;
		if scheme_specific_parsing_rule.user_information_should_not_be_present()
		{
			return Err(SchemeDoesNotSupportUserInformation)
		}
		
		let mut string = Utf8SequencesParser::new_stack(remaining);
		loop
		{
			match remaining.decode_next_utf8_validity_already_checked()
			{
				None => return Ok(Self(string.to_cow())),
				
				Some(Utf8SequenceAndCharacter(utf8_sequence, character)) => match character
				{
					iunreserved_without_ucschar!() => string.push_ascii_character(character)?,
					iunreserved_with_ucschar_2!()  => string.push_utf8_sequence_enum_2(utf8_sequence)?,
					iunreserved_with_ucschar_3!()  => string.push_utf8_sequence_enum_3(utf8_sequence)?,
					iunreserved_with_ucschar_4!()  => string.push_utf8_sequence_enum_4(utf8_sequence)?,
					pct_encoded!()                 => string.push_forcing_heap_percent_encoded::<UserInformationParseError, false>(remaining)?,
					sub_delims!()                  => string.push_ascii_character(character)?,
					ColonChar                      => string.push_ascii_character(character)?,
					
					_ => return Err(InvalidCharacterInUserInformation(character)),
				},
			}
		}
	}
}
