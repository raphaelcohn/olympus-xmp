// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An u8 parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum U8ParseError
{
	#[allow(missing_docs)]
	InvalidU8(ParseIntError),
	
	#[allow(missing_docs)]
	InvalidValue(u8),
}

impl Display for U8ParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for U8ParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use U8ParseError::*;
		match self
		{
			InvalidU8(cause) => Some(cause),
			
			InvalidValue(_) => None,
		}
	}
}
