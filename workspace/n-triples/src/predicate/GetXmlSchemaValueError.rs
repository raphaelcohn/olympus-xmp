// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetXmlSchemaValueError<'a, Cause: error::Error>
{
	/// Predicate.
	pub predicate: Predicate<'a>,
	
	/// Cause.
	pub cause: Cause,
}

impl<'a, Cause: error::Error> Display for GetXmlSchemaValueError<'a, Cause>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<'a, Cause: 'static + error::Error> error::Error for GetXmlSchemaValueError<'a, Cause>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		Some(&self.cause)
	}
}

impl<'a, Cause: error::Error> GetXmlSchemaValueError<'a, Cause>
{
	#[inline(always)]
	fn new(predicate: Predicate<'a>, cause: Cause) -> Self
	{
		Self
		{
			predicate,
			cause,
		}
	}
}
