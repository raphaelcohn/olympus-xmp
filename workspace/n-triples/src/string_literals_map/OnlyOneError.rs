// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnlyOneError<E: error::Error>
{
	#[allow(missing_docs)]
	Missing,
	
	#[allow(missing_docs)]
	ZeroOrOne(ZeroOrOneError<E>),
}

impl<E: error::Error> From<E> for OnlyOneError<E>
{
	#[inline(always)]
	fn from(cause: E) -> Self
	{
		OnlyOneError::ZeroOrOne(ZeroOrOneError::Parse(cause))
	}
}

impl<E: error::Error> Display for OnlyOneError<E>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<E: 'static + error::Error> error::Error for OnlyOneError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use OnlyOneError::*;
		
		match self
		{
			ZeroOrOne(cause) => Some(cause),
			
			_ => None,
		}
	}
}
