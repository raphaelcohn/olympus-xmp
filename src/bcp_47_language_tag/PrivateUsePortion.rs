// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// `"-" (1*8alphanum)`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PrivateUsePortion
{
	#[allow(missing_docs)]
	Alphanumeric1([Alphanumeric; 1]),
	
	#[allow(missing_docs)]
	Alphanumeric2([Alphanumeric; 2]),
	
	#[allow(missing_docs)]
	Alphanumeric3([Alphanumeric; 3]),
	
	#[allow(missing_docs)]
	Alphanumeric4([Alphanumeric; 4]),
	
	#[allow(missing_docs)]
	Alphanumeric5([Alphanumeric; 5]),
	
	#[allow(missing_docs)]
	Alphanumeric6([Alphanumeric; 6]),
	
	#[allow(missing_docs)]
	Alphanumeric7([Alphanumeric; 7]),
	
	#[allow(missing_docs)]
	Alphanumeric8([Alphanumeric; 8]),
}

impl PrivateUsePortion
{
	#[inline(always)]
	fn parse(subtag: &[u8]) -> Result<Self, PrivateUseSubtagsParseError>
	{
		use PrivateUsePortion::*;
		use PrivateUseSubtagsParseError::*;
		
		match subtag.len()
		{
			0 => Err(LengthIsZero),
			
			1 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 1>(subtag, Alphanumeric1, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			2 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 2>(subtag, Alphanumeric2, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			3 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 3>(subtag, Alphanumeric3, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			4 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 4>(subtag, Alphanumeric4, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			5 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 5>(subtag, Alphanumeric5, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			6 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 6>(subtag, Alphanumeric6, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			7 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 7>(subtag, Alphanumeric7, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			8 => Alphanumeric::validate_alphanumeric_to_lower_case::<_, _, _, _, 8>(subtag, Alphanumeric8, PrivateUseSubtagLengthIsTwoToEightButInvalidAlpha),
			
			_ => Err(LengthIsGreaterThanEight),
		}
	}
}
