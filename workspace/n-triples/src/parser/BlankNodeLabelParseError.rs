// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlankNodeLabelParseError
{
	#[allow(missing_docs)]
	InvalidUtf8Parse(InvalidUtf8ParseError),
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingColon,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingFirstCharacterOfLabel,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingSubsequentCharacterOfLabel,
	
	#[allow(missing_docs)]
	ExpectedColon,
	
	#[allow(missing_docs)]
	InvalidCharacter(char),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
	
	#[allow(missing_docs)]
	PeriodIsNotAllowedAsTheFinalCharacterOfABlankNodeLabel,
}

impl const From<InvalidUtf8ParseError> for BlankNodeLabelParseError
{
	#[inline(always)]
	fn from(cause: InvalidUtf8ParseError) -> Self
	{
		BlankNodeLabelParseError::InvalidUtf8Parse(cause)
	}
}

impl const From<TryReserveError> for BlankNodeLabelParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		BlankNodeLabelParseError::OutOfMemory(cause)
	}
}

impl Display for BlankNodeLabelParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for BlankNodeLabelParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use BlankNodeLabelParseError::*;
		
		match self
		{
			InvalidUtf8Parse(cause) => Some(cause),
			
			OutOfMemory(cause) => Some(cause),
			
			_ => None,
		}
	}
}
