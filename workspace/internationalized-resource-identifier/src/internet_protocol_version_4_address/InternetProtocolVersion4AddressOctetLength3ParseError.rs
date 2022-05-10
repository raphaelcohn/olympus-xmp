// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InternetProtocolVersion4AddressOctetLength3ParseError
{
	#[allow(missing_docs)]
	FirstDigitMustBe1Or2,
	
	#[allow(missing_docs)]
	SecondDigitMustBeBetween0To9Inclusive,
	
	#[allow(missing_docs)]
	SecondDigitMustBeBetween0To5Inclusive,
	
	#[allow(missing_docs)]
	ThirdDigitMustBeBetween0To9Inclusive,
	
	#[allow(missing_docs)]
	ThirdDigitMustBeBetween0To5Inclusive,
}

impl Display for InternetProtocolVersion4AddressOctetLength3ParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for InternetProtocolVersion4AddressOctetLength3ParseError
{
}
