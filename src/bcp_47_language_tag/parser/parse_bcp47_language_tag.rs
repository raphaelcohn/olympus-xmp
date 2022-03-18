// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) fn parse_bcp47_language_tag(language_tag: &str) -> Result<Bcp47LanguageTag, Bcp47LanguageTagParseError>
{
	let mut subtags = MemchrIterator::from_str(language_tag);
	
	let (language, next_subtag) =
	{
		let first_subtag = subtags.next_first();
		use Bcp47LanguageTag::*;
		use LanguageFirstSubtagParseError::*;
		use NextSubtag::Pending;
		use RegisteredLanguageSubtag::*;
		match first_subtag.len()
		{
			0 => Err(FirstSubtagLengthIsZero)?,
			
			1 => match to_lower_case(first_subtag.get_unchecked_value_safe(0usize))
			{
				i => return Ok(Grandfathered(super::Grandfathered::Irregular(IrregularGrandfathered::parse_irregular_i(subtags)?))),
				
				x => return Ok(PrivateUse(super::PrivateUse::parse(subtags)?)),
				
				byte @ _ => Err(FirstSubtagLengthIsOneAndIsNotPrivateUseOrGrandfatheredIrregularTag(byte))?,
			},
			
			2 => OrdinaryLanguage::parse_2(first_subtag, &mut subtags)?,
			
			3 => OrdinaryLanguage::parse_3(first_subtag, &mut subtags)?,
			
			4 => (ReservedLanguageSubtag::parse(first_subtag)?, Pending),
			
			5 => (RegisteredLanguageSubtag::parse::<_, 5>(first_subtag, Alpha5)?, Pending),
			
			6 => (RegisteredLanguageSubtag::parse::<_, 6>(first_subtag, Alpha6)?, Pending),
			
			7 => (RegisteredLanguageSubtag::parse::<_, 7>(first_subtag, Alpha7)?, Pending),
			
			8 => (RegisteredLanguageSubtag::parse::<_, 8>(first_subtag, Alpha8)?, Pending),
			
			_ => Err(FirstSubtagLengthIsMoreThanEight)?,
		}
	};
	
	Ok(unimplemented!())
}
