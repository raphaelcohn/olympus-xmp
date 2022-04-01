// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InvalidUtf8ParseError
{
	#[allow(missing_docs)]
	DidNotExpectEndParsingUtf8Character,
	
	#[allow(missing_docs)]
	InvalidStartToUtf8Sequence,
	
	#[allow(missing_docs)]
	InvalidUtf8CodePoint(CharTryFromError),
}

impl Display for InvalidUtf8ParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for InvalidUtf8ParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use InvalidUtf8ParseError::*;
		
		match self
		{
			InvalidUtf8CodePoint(cause) => Some(cause),
			
			_ => None,
		}
	}
}
