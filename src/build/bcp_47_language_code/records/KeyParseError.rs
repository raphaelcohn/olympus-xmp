// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum KeyParseError
{
	SubtagInvalidLength
	{
		length: usize,
		
		minimum: usize,
	
		maximum: usize,
		
		subtag: String,
	},
	
	TagOrSubtagByteIsNotDigit
	{
		index: usize,
	
		byte: u8,
	},
	
	TagOrSubtagByteIsNotUpperCaseAlpha
	{
		index: usize,
	
		byte: u8,
	},
	
	TagOrSubtagByteIsNotLowerCaseAlpha
	{
		index: usize,
	
		byte: u8,
	},
	
	TagOrSubtagRange(TagOrSubtagRangeError),
	
	DuplicateRecord,
}

impl From<TagOrSubtagRangeError> for KeyParseError
{
	#[inline(always)]
	fn from(cause: TagOrSubtagRangeError) -> Self
	{
		KeyParseError::TagOrSubtagRange(cause)
	}
}

impl Display for KeyParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for KeyParseError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use KeyParseError::*;
		
		match self
		{
			TagOrSubtagRange(cause) => Some(cause),
			
			_ => None,
		}
	}
}
