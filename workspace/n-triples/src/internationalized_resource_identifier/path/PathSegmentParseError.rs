// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PathSegmentParseError
{
	#[allow(missing_docs)]
	InvalidCharacter(char),
	
	#[allow(missing_docs)]
	OutOfMemoryOrInvalidUtf8PercentDecodeParse(OutOfMemoryOrInvalidUtf8PercentDecodeParseError),
}

impl const From<OutOfMemoryOrInvalidUtf8PercentDecodeParseError> for PathSegmentParseError
{
	#[inline(always)]
	fn from(cause: OutOfMemoryOrInvalidUtf8PercentDecodeParseError) -> Self
	{
		PathSegmentParseError::OutOfMemoryOrInvalidUtf8PercentDecodeParse(cause)
	}
}

impl Display for PathSegmentParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for PathSegmentParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use PathSegmentParseError::*;
		
		match self
		{
			OutOfMemoryOrInvalidUtf8PercentDecodeParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
