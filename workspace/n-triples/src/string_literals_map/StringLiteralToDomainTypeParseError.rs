// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug)]
pub enum StringLiteralToDomainTypeParseError<StrParseError: error::Error, TryFromError: error::Error>
{
	#[allow(missing_docs)]
	StrParse(StrParseError),

	#[allow(missing_docs)]
	TryFrom(TryFromError)
}

impl<StrParseError: error::Error, TryFromError: error::Error> Display for StringLiteralToDomainTypeParseError<StrParseError, TryFromError>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<StrParseError: 'static + error::Error, TryFromError: 'static + error::Error> error::Error for StringLiteralToDomainTypeParseError<StrParseError, TryFromError>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use StringLiteralToDomainTypeParseError::*;
		
		match self
		{
			StrParse(cause) => Some(cause),
			
			TryFrom(cause) => Some(cause),
		}
	}
}
