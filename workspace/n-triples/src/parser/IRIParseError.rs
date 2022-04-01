// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IRIParseError
{
	#[allow(missing_docs)]
	InvalidUtf8Parse(InvalidUtf8ParseError),
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingBody,
	
	#[allow(missing_docs)]
	InvalidCharacter(char),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
	
	#[allow(missing_docs)]
	EndOfFileParsingEscapeSequence,
	
	#[allow(missing_docs)]
	InvalidUCHAR4EscapeSequence(UCHARParseError),
	
	#[allow(missing_docs)]
	InvalidUCHAR8EscapeSequence(UCHARParseError),
	
	#[allow(missing_docs)]
	InvalidEscapeSequence(u8),
}

impl const From<InvalidUtf8ParseError> for IRIParseError
{
	#[inline(always)]
	fn from(cause: InvalidUtf8ParseError) -> Self
	{
		IRIParseError::InvalidUtf8Parse(cause)
	}
}

impl const From<TryReserveError> for IRIParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		IRIParseError::OutOfMemory(cause)
	}
}

impl Display for IRIParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for IRIParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use IRIParseError::*;
		
		match self
		{
			InvalidUtf8Parse(cause) => Some(cause),
			
			OutOfMemory(cause) => Some(cause),
			
			InvalidUCHAR4EscapeSequence(cause) => Some(cause),
			
			InvalidUCHAR8EscapeSequence(cause) => Some(cause),
			
			_ => None,
		}
	}
}
