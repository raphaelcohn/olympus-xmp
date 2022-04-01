// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UCHARParseError
{
	#[allow(missing_docs)]
	TooFewBytesRemain
	{
		expected_length: usize,
		
		actual_length: usize,
	},
	
	#[allow(missing_docs)]
	InvalidUtf8CodePoint(CharTryFromError),
	
	#[allow(missing_docs)]
	InvalidHexDigit(u8),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
}

impl const From<CharTryFromError> for UCHARParseError
{
	#[inline(always)]
	fn from(cause: CharTryFromError) -> Self
	{
		UCHARParseError::InvalidUtf8CodePoint(cause)
	}
}

impl const From<TryReserveError> for UCHARParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		UCHARParseError::OutOfMemory(cause)
	}
}

impl Display for UCHARParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for UCHARParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use UCHARParseError::*;
		
		match self
		{
			InvalidUtf8CodePoint(cause) => Some(cause),
			
			OutOfMemory(cause) => Some(cause),
			
			_ => None,
		}
	}
}
