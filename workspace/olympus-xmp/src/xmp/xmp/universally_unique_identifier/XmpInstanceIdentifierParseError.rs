// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An instance identifier parse error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum XmpInstanceIdentifierParseError
{
	#[allow(missing_docs)]
	Not44CharactersLong(usize),
	
	#[allow(missing_docs)]
	NotPrefixedCorrectly,
	
	#[allow(missing_docs)]
	MissingHyphen
	{
		byte: u8,
		
		index: usize,
	},
	
	#[allow(missing_docs)]
	InvalidByteAtIndex
	{
		byte: u8,
		
		index: usize,
	},
	
	#[allow(missing_docs)]
	Variant(VariantParseError),
	
	#[allow(missing_docs)]
	Version(VersionParseError),
}

impl const From<VariantParseError> for XmpInstanceIdentifierParseError
{
	#[inline(always)]
	fn from(value: VariantParseError) -> Self
	{
		XmpInstanceIdentifierParseError::Variant(value)
	}
}

impl const From<VersionParseError> for XmpInstanceIdentifierParseError
{
	#[inline(always)]
	fn from(value: VersionParseError) -> Self
	{
		XmpInstanceIdentifierParseError::Version(value)
	}
}

impl Display for XmpInstanceIdentifierParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for XmpInstanceIdentifierParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use XmpInstanceIdentifierParseError::*;
		match self
		{
			Variant(cause) => Some(cause),
			
			Version(cause) => Some(cause),
			
			_ => None,
		}
	}
}
