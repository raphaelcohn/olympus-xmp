// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RecordParseError
{
	MissingField(MissingFieldError),
	
	TagUsedInsteadOfSubtag,
	
	SubtagUsedInsteadOfTag,
	
	FieldNotPermitted(FieldNotPermittedError),
	
	Key(KeyParseError),
}

impl const From<FieldNotPermittedError> for RecordParseError
{
	#[inline(always)]
	fn from(cause: FieldNotPermittedError) -> Self
	{
		RecordParseError::FieldNotPermitted(cause)
	}
}

impl const From<KeyParseError> for RecordParseError
{
	#[inline(always)]
	fn from(cause: KeyParseError) -> Self
	{
		RecordParseError::Key(cause)
	}
}

impl Display for RecordParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for RecordParseError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use RecordParseError::*;
		
		match self
		{
			MissingField(cause) => Some(cause),
			
			TagUsedInsteadOfSubtag => None,
			
			SubtagUsedInsteadOfTag => None,
			
			FieldNotPermitted(cause) => Some(cause),
			
			Key(cause) => Some(cause),
		}
	}
}
