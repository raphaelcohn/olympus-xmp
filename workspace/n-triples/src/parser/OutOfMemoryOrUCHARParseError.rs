// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OutOfMemoryOrUCHARParseError
{
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
	
	#[allow(missing_docs)]
	UCHARParse(UCHARParseError),
}

impl const From<TryReserveError> for OutOfMemoryOrUCHARParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		OutOfMemoryOrUCHARParseError::OutOfMemory(cause)
	}
}

impl const From<UCHARParseError> for OutOfMemoryOrUCHARParseError
{
	#[inline(always)]
	fn from(cause: UCHARParseError) -> Self
	{
		OutOfMemoryOrUCHARParseError::UCHARParse(cause)
	}
}

impl Display for OutOfMemoryOrUCHARParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for OutOfMemoryOrUCHARParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use OutOfMemoryOrUCHARParseError::*;
		
		match self
		{
			OutOfMemory(cause) => Some(cause),
			
			UCHARParse(cause) => Some(cause),
		}
	}
}
