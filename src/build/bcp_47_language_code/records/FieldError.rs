// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum FieldError
{
	InvalidField(PullEventParserError),
	
	FieldType(FieldParseError<UnknownStringVariantError>),
	
	FieldTag(FieldParseError<Infallible>),
	
	FieldSubtag(FieldParseError<Infallible>),
	
	FieldAdded(FieldParseError<DateParseError>),
	
	FieldDeprecated(FieldParseError<DateParseError>),
	
	FieldPreferredValue(FieldParseError<Infallible>),
	
	FieldSuppressScript(FieldParseError<Infallible>),
	
	FieldMacrolanguage(FieldParseError<Infallible>),
	
	FieldScope(FieldParseError<UnknownStringVariantError>),
}

impl Display for FieldError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FieldError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use FieldError::*;
		
		match self
		{
			InvalidField(cause) => Some(cause),
			
			FieldType(cause) => Some(cause),
			
			FieldTag(cause) => Some(cause),
			
			FieldSubtag(cause) => Some(cause),
			
			FieldAdded(cause) => Some(cause),
			
			FieldDeprecated(cause) => Some(cause),
			
			FieldPreferredValue(cause) => Some(cause),
			
			FieldSuppressScript(cause) => Some(cause),
			
			FieldMacrolanguage(cause) => Some(cause),
			
			FieldScope(cause) => Some(cause),
		}
	}
}
