// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// As defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt).
/// And RFC 5646.
/// <https://en.wikipedia.org/wiki/IETF_language_tag> is helpful.
/// <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry> is official.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Bcp47LanguageTag
{
	#[allow(missing_docs)]
	Normal(Normal),
	
	#[allow(missing_docs)]
	PrivateUse(PrivateUse),

	#[allow(missing_docs)]
	Grandfathered(Grandfathered),
}

impl const From<Normal> for Bcp47LanguageTag
{
	#[inline(always)]
	fn from(normal: Normal) -> Self
	{
		Bcp47LanguageTag::Normal(normal)
	}
}

impl const From<Grandfathered> for Bcp47LanguageTag
{
	#[inline(always)]
	fn from(grandfathered: Grandfathered) -> Self
	{
		Bcp47LanguageTag::Grandfathered(grandfathered)
	}
}

impl const From<PrivateUse> for Bcp47LanguageTag
{
	#[inline(always)]
	fn from(private_use: PrivateUse) -> Self
	{
		Bcp47LanguageTag::PrivateUse(private_use)
	}
}

impl Bcp47LanguageTag
{
	#[allow(missing_docs)]
	pub fn parse(language_tag: &str) -> Result<Self, Bcp47LanguageTagParseError>
	{
		let subtags = MemchrIterator::from_str(language_tag);
		parse_bcp47_language_subtag(subtags)
	}
	
	#[inline(always)]
	fn parse_x_or_i_subtag<'a>(first_subtag: &'a [u8], subtags: MemchrIterator<'a, Hyphen>) -> Result<Self, Bcp47LanguageTagParseError>
	{
		match to_lower_case(first_subtag.get_unchecked_value_safe(0usize))
		{
			i => Ok(Self::irregular_grandfathered(IrregularGrandfathered::parse_irregular_i(subtags)?)),
			
			x => Ok(PrivateUse::parse(subtags)?.into()),
			
			byte @ _ => Err(LanguageFirstSubtagParseError::FirstSubtagLengthIsOneAndIsNotPrivateUseOrGrandfatheredIrregularTag(byte))?,
		}
	}
	
	#[inline(always)]
	const fn regular_grandfathered(regular_grandfathered: RegularGrandfathered) -> Self
	{
		Grandfathered::Regular(regular_grandfathered).into()
	}
	
	#[inline(always)]
	const fn irregular_grandfathered(irregular_grandfathered: IrregularGrandfathered) -> Self
	{
		Grandfathered::Irregular(irregular_grandfathered).into()
	}
	
	#[inline(always)]
	const fn from_language_ok(language: Language) -> Result<Self, Bcp47LanguageTagParseError>
	{
		Ok(Normal::from(language).into())
	}
}
