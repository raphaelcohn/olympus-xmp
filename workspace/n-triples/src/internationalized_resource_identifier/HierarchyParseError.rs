// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.

/// A parse error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HierarchyParseError
{
	#[allow(missing_docs)]
	DidNotExpectEndParsingFirstCharacter,
	
	#[allow(missing_docs)]
	InvalidPercentEncodedUtf8ParseFirstCharacter(InvalidUtf8ParseError<PercentDecodeError>),
	
	#[allow(missing_docs)]
	InvalidFirstCharacter(char),
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingSecondCharacter,
	
	#[allow(missing_docs)]
	InvalidPercentEncodedUtf8ParseSecondCharacter(InvalidUtf8ParseError<PercentDecodeError>),
	
	#[allow(missing_docs)]
	InvalidSecondCharacter(char),
	
	#[allow(missing_docs)]
	AuthorityParse(AuthorityParseError),
	
	#[allow(missing_docs)]
	IPathAbemptyParse(PathSegmentsParseError),
	
	#[allow(missing_docs)]
	IPathRootlessParse(NonEmptyPathParseError),
	
	#[allow(missing_docs)]
	IPathAbsoluteParse(NonEmptyPathParseError),
}

impl const From<AuthorityParseError> for HierarchyParseError
{
	#[inline(always)]
	fn from(cause: AuthorityParseError) -> Self
	{
		HierarchyParseError::AuthorityParse(cause)
	}
}

impl const From<PathSegmentsParseError> for HierarchyParseError
{
	#[inline(always)]
	fn from(cause: PathSegmentsParseError) -> Self
	{
		HierarchyParseError::IPathAbemptyParse(cause)
	}
}

impl Display for HierarchyParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for HierarchyParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use HierarchyParseError::*;
		
		match self
		{
			InvalidPercentEncodedUtf8ParseFirstCharacter(cause) => Some(cause),
			
			InvalidPercentEncodedUtf8ParseSecondCharacter(cause) => Some(cause),
			
			AuthorityParse(cause) => Some(cause),
			
			IPathAbemptyParse(cause) => Some(cause),
			
			IPathRootlessParse(cause) => Some(cause),
			
			IPathAbsoluteParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
