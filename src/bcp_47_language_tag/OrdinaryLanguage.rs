// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Basically, a 2 or 3 byte alpha code with an optional `extlang`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OrdinaryLanguage
{
	iana_registered_iso_639_code: IanaRegisteredIso639Code,
	
	language_extension: Option<LanguageExtension>,
}

impl OrdinaryLanguage
{
	#[inline(always)]
	fn parse_2<'a>(first_subtag: &'a [u8], subtags: &mut MemchrIterator<'a, Hyphen>) -> Result<Either<(Language, NextSubtag<'a>), Bcp47LanguageTag>, LanguageFirstSubtagParseError>
	{
		Self::parse_n::<_, 2>(first_subtag, subtags, |alpha_array| IanaRegisteredIso639Code::Alpha2(IanaRegisteredIso639Alpha2Code(alpha_array)))
	}
	
	#[inline(always)]
	fn parse_3<'a>(first_subtag: &'a [u8], subtags: &mut MemchrIterator<'a, Hyphen>) -> Result<Either<(Language, NextSubtag<'a>), Bcp47LanguageTag>, LanguageFirstSubtagParseError>
	{
		Self::parse_n::<_, 3>(first_subtag, subtags, |alpha_array| IanaRegisteredIso639Code::Alpha3(IanaRegisteredIso639Alpha3Code(alpha_array)))
	}
	
	#[inline(always)]
	fn parse_n<'a, Constructor: FnOnce([Alpha; length]) -> IanaRegisteredIso639Code, const length: usize>(first_subtag: &'a [u8], subtags: &mut MemchrIterator<'a, Hyphen>, constructor: Constructor) -> Result<Either<(Language, NextSubtag<'a>), Bcp47LanguageTag>, LanguageFirstSubtagParseError>
	{
		let iana_registered_iso_639_code = Alpha::validate_alpha_to_lower_case::<_, _, _, _, length>(first_subtag, constructor, FirstSubtagLengthIsTwoToEightButInvalidAlpha)?;
		Ok(LanguageExtension::parse(subtags, iana_registered_iso_639_code)?)
	}
}
