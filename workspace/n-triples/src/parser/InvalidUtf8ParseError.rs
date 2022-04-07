// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::parser::utf8::PercentDecodeError;

/// A parse error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InvalidUtf8ParseError<E: error::Error>
{
	#[allow(missing_docs)]
	DidNotExpectEndParsingOneByteUtf8Character,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingTwoByteUtf8Character,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingThreeByteUtf8Character,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingFourByteUtf8Character,
	
	#[allow(missing_docs)]
	InvalidStartToUtf8Sequence,
	
	#[allow(missing_docs)]
	InvalidUtf8CodePoint(CharTryFromError),
	
	#[allow(missing_docs)]
	Inner(E),
}

impl From<PercentDecodeError> for InvalidUtf8ParseError<PercentDecodeError>
{
	#[inline(always)]
	fn from(cause: PercentDecodeError) -> Self
	{
		InvalidUtf8ParseError::Inner(cause)
	}
}

impl<E: error::Error> Display for InvalidUtf8ParseError<E>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<E: error::Error> error::Error for InvalidUtf8ParseError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use InvalidUtf8ParseError::*;
		
		match self
		{
			InvalidUtf8CodePoint(cause) => Some(cause),
			
			Inner(cause) => Some(cause),
			
			_ => None,
		}
	}
}
