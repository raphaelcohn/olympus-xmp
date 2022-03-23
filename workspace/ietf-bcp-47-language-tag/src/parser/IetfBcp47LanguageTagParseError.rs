// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IetfBcp47LanguageTagParseError
{
	FirstSubtag(LanguageSubtagParseError),

	GrandfatheredIrregularI(GrandfatheredIrregularISubtagParseError),
	
	PrivateUseSubtags(PrivateUseSubtagsParseError),

	Script(ScriptParseError),

	Region(RegionParseError),

	Variant(VariantParseError),

	Extension(ExtensionParseError),
}

impl const From<LanguageSubtagParseError> for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: LanguageSubtagParseError) -> Self
	{
		IetfBcp47LanguageTagParseError::FirstSubtag(cause)
	}
}

impl const From<GrandfatheredIrregularISubtagParseError> for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: GrandfatheredIrregularISubtagParseError) -> Self
	{
		IetfBcp47LanguageTagParseError::GrandfatheredIrregularI(cause)
	}
}

impl const From<PrivateUseSubtagsParseError> for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: PrivateUseSubtagsParseError) -> Self
	{
		IetfBcp47LanguageTagParseError::PrivateUseSubtags(cause)
	}
}

impl const From<RegionParseError> for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: RegionParseError) -> Self
	{
		IetfBcp47LanguageTagParseError::Region(cause)
	}
}

impl const From<ScriptParseError> for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: ScriptParseError) -> Self
	{
		IetfBcp47LanguageTagParseError::Script(cause)
	}
}

impl const From<VariantParseError> for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: VariantParseError) -> Self
	{
		IetfBcp47LanguageTagParseError::Variant(cause)
	}
}

impl const From<ExtensionParseError> for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn from(cause: ExtensionParseError) -> Self
	{
		IetfBcp47LanguageTagParseError::Extension(cause)
	}
}

impl Display for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for IetfBcp47LanguageTagParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use IetfBcp47LanguageTagParseError::*;
		
		match self
		{
			FirstSubtag(cause) => Some(cause),
			
			GrandfatheredIrregularI(cause) => Some(cause),
			
			PrivateUseSubtags(cause) => Some(cause),
			
			Region(cause) => Some(cause),
			
			Script(cause) => Some(cause),
			
			Variant(cause) => Some(cause),
			
			Extension(cause) => Some(cause),
		}
	}
}
