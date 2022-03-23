// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An i8 parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum I8ParseError
{
	#[allow(missing_docs)]
	InvalidI8(ParseIntError),
	
	#[allow(missing_docs)]
	InvalidValue(i8),
}

impl const From<ParseIntError> for I8ParseError
{
	#[inline(always)]
	fn from(cause: ParseIntError) -> Self
	{
		I8ParseError::InvalidI8(cause)
	}
}

impl Display for I8ParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for I8ParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use I8ParseError::*;
		match self
		{
			InvalidI8(cause) => Some(cause),
			
			InvalidValue(_) => None,
		}
	}
}
