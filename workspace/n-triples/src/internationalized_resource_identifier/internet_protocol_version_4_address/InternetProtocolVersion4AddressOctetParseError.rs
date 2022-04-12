// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InternetProtocolVersion4AddressOctetParseError
{
	#[allow(missing_docs)]
	InternetProtocolVersion4AddressOctetLength1(InternetProtocolVersion4AddressOctetLength1ParseError),
	
	#[allow(missing_docs)]
	InternetProtocolVersion4AddressOctetLength2(InternetProtocolVersion4AddressOctetLength2ParseError),
	
	#[allow(missing_docs)]
	InternetProtocolVersion4AddressOctetLength3(InternetProtocolVersion4AddressOctetLength3ParseError),
}

impl const From<InternetProtocolVersion4AddressOctetLength1ParseError> for InternetProtocolVersion4AddressOctetParseError
{
	#[inline(always)]
	fn from(cause: InternetProtocolVersion4AddressOctetLength1ParseError) -> Self
	{
		InternetProtocolVersion4AddressOctetParseError::InternetProtocolVersion4AddressOctetLength1(cause)
	}
}

impl const From<InternetProtocolVersion4AddressOctetLength2ParseError> for InternetProtocolVersion4AddressOctetParseError
{
	#[inline(always)]
	fn from(cause: InternetProtocolVersion4AddressOctetLength2ParseError) -> Self
	{
		InternetProtocolVersion4AddressOctetParseError::InternetProtocolVersion4AddressOctetLength2(cause)
	}
}

impl const From<InternetProtocolVersion4AddressOctetLength3ParseError> for InternetProtocolVersion4AddressOctetParseError
{
	#[inline(always)]
	fn from(cause: InternetProtocolVersion4AddressOctetLength3ParseError) -> Self
	{
		InternetProtocolVersion4AddressOctetParseError::InternetProtocolVersion4AddressOctetLength3(cause)
	}
}

impl Display for InternetProtocolVersion4AddressOctetParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for InternetProtocolVersion4AddressOctetParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use InternetProtocolVersion4AddressOctetParseError::*;
		
		match self
		{
			InternetProtocolVersion4AddressOctetLength1(cause) => Some(cause),
			
			InternetProtocolVersion4AddressOctetLength2(cause) => Some(cause),
			
			InternetProtocolVersion4AddressOctetLength3(cause) => Some(cause),
			
			_ => None,
		}
	}
}
