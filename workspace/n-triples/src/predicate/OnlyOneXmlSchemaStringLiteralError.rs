// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnlyOneXmlSchemaStringLiteralError<'a>
{
	#[allow(missing_docs)]
	String(Predicate<'a>, OnlyOneError<Infallible>),
	
	#[allow(missing_docs)]
	Boolean(Predicate<'a>, OnlyOneError<ParseBoolError>),
	
	#[allow(missing_docs)]
	Integer(Predicate<'a>, OnlyOneError<ParseIntError>),
	
	#[allow(missing_docs)]
	DateTime(Predicate<'a>, OnlyOneError<ParseDateTimeError>),
}

impl<'a> Display for OnlyOneXmlSchemaStringLiteralError<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<'a> error::Error for OnlyOneXmlSchemaStringLiteralError<'a>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use OnlyOneXmlSchemaStringLiteralError::*;
		
		match self
		{
			String(_, cause) => Some(cause),
			
			Boolean(_, cause) => Some(cause),
			
			Integer(_, cause) => Some(cause),
			
			DateTime(_, cause) => Some(cause),
		}
	}
}
