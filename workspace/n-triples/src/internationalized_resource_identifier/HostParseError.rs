// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HostParseError
{
	#[allow(missing_docs)]
	IpLiteralIsNotClosedBySquareBracket,
	
	#[allow(missing_docs)]
	FutureInternetProtocolAddressParsingIsUnsupported,
	
	#[allow(missing_docs)]
	InternetProtocolVersion6AddressIsTooLong,
	
	#[allow(missing_docs)]
	CouldNotParseInternetProtocolVersion6Address,
	
	#[allow(missing_docs)]
	InternetProtocolVersion4AddressParse(InternetProtocolVersion4AddressParseError),
	
	#[allow(missing_docs)]
	HostNameParse(HostNameParseError),
}

impl const From<HostNameParseError> for HostParseError
{
	#[inline(always)]
	fn from(cause: HostNameParseError) -> Self
	{
		HostParseError::HostNameParse(cause)
	}
}

impl Display for HostParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for HostParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use HostParseError::*;
		
		match self
		{
			InternetProtocolVersion4AddressParse(cause) => Some(cause),
			
			HostNameParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
