// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FieldParseError<Err: error::Error>
{
	Repeated,
	
	FromString(Err),
}

impl<Err: error::Error> Display for FieldParseError<Err>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<Err: error::Error + 'static> error::Error for FieldParseError<Err>
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use FieldParseError::*;
		
		match self
		{
			Repeated => None,
			
			FromString(cause) => Some(cause),
		}
	}
}
