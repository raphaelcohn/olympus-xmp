// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HostNameParseError
{
	#[allow(missing_docs)]
	InvalidCharacterInHostName(char),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
	
	#[allow(missing_docs)]
	InvalidUtf8PercentDecodeParse(InvalidUtf8ParseError<PercentDecodeError>),
}

impl const From<TryReserveError> for HostNameParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		HostNameParseError::OutOfMemory(cause)
	}
}

impl const From<InvalidUtf8ParseError<PercentDecodeError>> for HostNameParseError
{
	#[inline(always)]
	fn from(cause: InvalidUtf8ParseError<PercentDecodeError>) -> Self
	{
		HostNameParseError::InvalidUtf8PercentDecodeParse(cause)
	}
}

impl Display for HostNameParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for HostNameParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use HostNameParseError::*;
		
		match self
		{
			OutOfMemory(cause) => Some(cause),
			
			InvalidUtf8PercentDecodeParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
