// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZeroOrOneError<E: error::Error>
{
	#[allow(missing_docs)]
	TooMany(MoreThanOneError),
	
	#[allow(missing_docs)]
	Parse(E),
}

impl<E: error::Error> From<E> for ZeroOrOneError<E>
{
	#[inline(always)]
	fn from(cause: E) -> Self
	{
		ZeroOrOneError::Parse(cause)
	}
}

impl<E: error::Error> Display for ZeroOrOneError<E>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<E: 'static + error::Error> error::Error for ZeroOrOneError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use ZeroOrOneError::*;
		
		match self
		{
			TooMany(cause) => Some(cause),
			
			Parse(cause) => Some(cause),
		}
	}
}
