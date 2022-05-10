// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PathSegmentsParseError
{
	#[allow(missing_docs)]
	PathSegmentParse(PathSegmentParseError),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
}

impl Display for PathSegmentsParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for PathSegmentsParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use PathSegmentsParseError::*;
		
		match self
		{
			PathSegmentParse(cause) => Some(cause),
			
			OutOfMemory(cause) => Some(cause),
		}
	}
}
