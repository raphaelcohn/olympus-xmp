// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LanguageSubtagParseError
{
	FirstSubtagLengthIsOneAndIsNotPrivateUseOrGrandfatheredIrregularTag(u8),
	
	InvalidSubtagLength(InvalidSubtagLengthError),
	
	InvalidAlpha(InvalidAlphaError),

	LanguageExtensionSubtag(LanguageExtensionSubtagParseError)
}

impl From<InvalidSubtagLengthError> for LanguageSubtagParseError
{
	#[inline(always)]
	fn from(cause: InvalidSubtagLengthError) -> Self
	{
		LanguageSubtagParseError::InvalidSubtagLength(cause)
	}
}

impl From<LanguageExtensionSubtagParseError> for LanguageSubtagParseError
{
	#[inline(always)]
	fn from(cause: LanguageExtensionSubtagParseError) -> Self
	{
		LanguageSubtagParseError::LanguageExtensionSubtag(cause)
	}
}

impl Display for LanguageSubtagParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for LanguageSubtagParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use LanguageSubtagParseError::*;
		
		match self
		{
			InvalidSubtagLength(cause) => Some(cause),
			
			InvalidAlpha(cause) => Some(cause),
			
			LanguageExtensionSubtag(cause) => Some(cause),
			
			_ => None,
		}
	}
}
