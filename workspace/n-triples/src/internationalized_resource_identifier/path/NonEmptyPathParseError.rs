// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NonEmptyPathParseError
{
	#[allow(missing_docs)]
	PathSegmentParse(PathSegmentParseError),
	
	#[allow(missing_docs)]
	PathSegmentsParse(PathSegmentsParseError),
}

impl const From<PathSegmentParseError> for NonEmptyPathParseError
{
	#[inline(always)]
	fn from(cause: PathSegmentParseError) -> Self
	{
		NonEmptyPathParseError::PathSegmentParse(cause)
	}
}

impl const From<PathSegmentsParseError> for NonEmptyPathParseError
{
	#[inline(always)]
	fn from(cause: PathSegmentsParseError) -> Self
	{
		NonEmptyPathParseError::PathSegmentsParse(cause)
	}
}

impl Display for NonEmptyPathParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for NonEmptyPathParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use NonEmptyPathParseError::*;
		
		match self
		{
			PathSegmentParse(cause) => Some(cause),
			
			PathSegmentsParse(cause) => Some(cause),
		}
	}
}
