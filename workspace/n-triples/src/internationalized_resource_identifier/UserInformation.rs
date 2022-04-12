// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// User information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserInformation<'a>(Cow<'a, str>);

impl<'a> UserInformation<'a>
{
	/// `iuserinfo      = *( iunreserved / pct-encoded / sub-delims / ":" )`
	/// `iunreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~" / ucschar`.
	#[inline(always)]
	fn parse(mut user_info_bytes: &'a [u8]) -> Result<Self, UserInformationParseError>
	{
		use UserInformationParseError::*;
		
		let remaining_utf8_bytes = &mut user_info_bytes;
		let mut string = StringSoFar::new_stack(remaining_utf8_bytes);
		loop
		{
			use Utf8CharacterLength::*;
			
			match StringSoFar::decode_next_utf8_validity_already_checked(remaining_utf8_bytes)
			{
				None => return Ok(Self(string.to_cow())),
				
				Some(character) => match character
				{
					iunreserved_without_ucschar!() => string.push(character, One)?,
					iunreserved_with_ucschar_2!()  => string.push(character, Two)?,
					iunreserved_with_ucschar_3!()  => string.push(character, Three)?,
					iunreserved_with_ucschar_4!()  => string.push(character, Four)?,
					pct_encoded!()                 => string.push_forcing_heap_percent_encoded(remaining_utf8_bytes)?,
					sub_delims!()                  => string.push(character, One)?,
					ColonChar                      => string.push(character, One)?,
					
					_ => return Err(InvalidCharacterInUserInformation(character)),
				},
			}
		}
	}
}
