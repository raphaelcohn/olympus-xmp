// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AbsoluteInternationalizedResourceIdentifierParseError
{
	#[allow(missing_docs)]
	AbsoluteInternationalizedResourceNTripleEscapedIdentifierParse(AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError),
	
	#[allow(missing_docs)]
	AbsoluteInternationalizedResourceIdentifierStringParse(AbsoluteInternationalizedResourceIdentifierStringParseError),
}

impl const From<AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError> for AbsoluteInternationalizedResourceIdentifierParseError
{
	#[inline(always)]
	fn from(cause: AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError) -> Self
	{
		AbsoluteInternationalizedResourceIdentifierParseError::AbsoluteInternationalizedResourceNTripleEscapedIdentifierParse(cause)
	}
}

impl const From<AbsoluteInternationalizedResourceIdentifierStringParseError> for AbsoluteInternationalizedResourceIdentifierParseError
{
	#[inline(always)]
	fn from(cause: AbsoluteInternationalizedResourceIdentifierStringParseError) -> Self
	{
		AbsoluteInternationalizedResourceIdentifierParseError::AbsoluteInternationalizedResourceIdentifierStringParse(cause)
	}
}

impl Display for AbsoluteInternationalizedResourceIdentifierParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for AbsoluteInternationalizedResourceIdentifierParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use AbsoluteInternationalizedResourceIdentifierParseError::*;
		
		match self
		{
			AbsoluteInternationalizedResourceNTripleEscapedIdentifierParse(cause) => Some(cause),
			
			AbsoluteInternationalizedResourceIdentifierStringParse(cause) => Some(cause),
		}
	}
}
