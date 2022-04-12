// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InternetProtocolVersion4AddressParseError
{
	#[allow(missing_docs)]
	InternetProtocolVersion4AddressOctetParse
	{
		cause: InternetProtocolVersion4AddressOctetParseError,
	
		octet_number: InternetProtocolVersion4AddressOctetNumber,
		
		invalid: u8,
	},
	
	#[allow(missing_docs)]
	RemainingThreeOctetsTooShort(usize),
	
	#[allow(missing_docs)]
	RemainingTwoOctetsTooShort(usize),
	
	#[allow(missing_docs)]
	RemainingOctetTooShort,
	
	#[allow(missing_docs)]
	ThreeDigitSecondOctetNotFollowedByPeriod,
	
	#[allow(missing_docs)]
	ThreeDigitThirdOctetEndsBeforePeriod,
	
	#[allow(missing_docs)]
	ThreeDigitThirdOctetNotFollowedByPeriod,
}

impl Display for InternetProtocolVersion4AddressParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for InternetProtocolVersion4AddressParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use InternetProtocolVersion4AddressParseError::*;
		
		match self
		{
			InternetProtocolVersion4AddressOctetParse { cause, .. } => Some(cause),
			
			_ => None,
		}
	}
}
