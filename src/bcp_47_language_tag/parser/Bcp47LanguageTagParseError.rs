// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Bcp47LanguageTagParseError
{
	FirstSubtag(LanguageFirstSubtagParseError),

	GrandfatheredIrregularI(GrandfatheredIrregularISubtagParseError),
	
	PrivateUseSubtags(PrivateUseSubtagsParseError),

	Variant(VariantParseError),
}

impl From<LanguageFirstSubtagParseError> for Bcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: LanguageFirstSubtagParseError) -> Self
	{
		Bcp47LanguageTagParseError::FirstSubtag(cause)
	}
}

impl From<GrandfatheredIrregularISubtagParseError> for Bcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: GrandfatheredIrregularISubtagParseError) -> Self
	{
		Bcp47LanguageTagParseError::GrandfatheredIrregularI(cause)
	}
}

impl From<PrivateUseSubtagsParseError> for Bcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: PrivateUseSubtagsParseError) -> Self
	{
		Bcp47LanguageTagParseError::PrivateUseSubtags(cause)
	}
}

impl From<VariantParseError> for Bcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: VariantParseError) -> Self
	{
		Bcp47LanguageTagParseError::Variant(cause)
	}
}

impl Display for Bcp47LanguageTagParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for Bcp47LanguageTagParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use Bcp47LanguageTagParseError::*;
		
		match self
		{
			FirstSubtag(cause) => Some(cause),
			
			GrandfatheredIrregularI(cause) => Some(cause),
			
			PrivateUseSubtags(cause) => Some(cause),
			
			Variant(cause) => Some(cause),
		}
	}
}
