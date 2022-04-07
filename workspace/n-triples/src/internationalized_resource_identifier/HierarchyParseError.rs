// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HierarchyParseError
{
	#[allow(missing_docs)]
	InvalidPercentEncodedUtf8Parse(InvalidUtf8ParseError<PercentDecodeError>),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
	
	#[allow(missing_docs)]
	InvalidCharacter(char),
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingCharacter,
}

impl const From<InvalidUtf8ParseError<PercentDecodeError>> for HierarchyParseError
{
	#[inline(always)]
	fn from(cause: InvalidUtf8ParseError<PercentDecodeError>) -> Self
	{
		HierarchyParseError::InvalidPercentEncodedUtf8Parse(cause)
	}
}

impl const From<TryReserveError> for HierarchyParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		HierarchyParseError::OutOfMemory(cause)
	}
}

impl Display for HierarchyParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for HierarchyParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use HierarchyParseError::*;
		
		match self
		{
			InvalidPercentEncodedUtf8Parse(cause) => Some(cause),
			
			OutOfMemory(cause) => Some(cause),
			
			_ => None,
		}
	}
}
