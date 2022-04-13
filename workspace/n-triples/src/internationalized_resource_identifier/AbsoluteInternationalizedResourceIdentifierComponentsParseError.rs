// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AbsoluteInternationalizedResourceIdentifierComponentsParseError
{
	#[allow(missing_docs)]
	SchemeParse(SchemeParseError),
	
	#[allow(missing_docs)]
	HierarchyParse(HierarchyParseError),
	
	#[allow(missing_docs)]
	QueryParse(QueryParseError),
	
	#[allow(missing_docs)]
	HashFragmentParse(HashFragmentParseError),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
}

impl const From<HashFragmentParseError> for AbsoluteInternationalizedResourceIdentifierComponentsParseError
{
	#[inline(always)]
	fn from(cause: HashFragmentParseError) -> Self
	{
		AbsoluteInternationalizedResourceIdentifierComponentsParseError::HashFragmentParse(cause)
	}
}

impl const From<TryReserveError> for AbsoluteInternationalizedResourceIdentifierComponentsParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		AbsoluteInternationalizedResourceIdentifierComponentsParseError::OutOfMemory(cause)
	}
}

impl Display for AbsoluteInternationalizedResourceIdentifierComponentsParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for AbsoluteInternationalizedResourceIdentifierComponentsParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use AbsoluteInternationalizedResourceIdentifierComponentsParseError::*;
		
		match self
		{
			SchemeParse(cause) => Some(cause),
			
			HierarchyParse(cause) => Some(cause),
			
			QueryParse(cause) => Some(cause),
			
			HashFragmentParse(cause) => Some(cause),
			
			OutOfMemory(cause) => Some(cause),
		}
	}
}
