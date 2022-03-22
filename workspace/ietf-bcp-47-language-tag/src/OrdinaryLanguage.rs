// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Basically, a 2 or 3 byte alpha code with an optional `extlang`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OrdinaryLanguage
{
	iana_registered_iso_639_code: IanaRegisteredIso639Code,
	
	language_extension: Option<LanguageExtension>,
}

impl const From<IanaRegisteredIso639Code> for OrdinaryLanguage
{
	#[inline(always)]
	fn from(iana_registered_iso_639_code: IanaRegisteredIso639Code) -> Self
	{
		OrdinaryLanguage
		{
			iana_registered_iso_639_code,
		
			language_extension: None,
		}
	}
}

impl const From<IanaRegisteredIso639Alpha2Code> for OrdinaryLanguage
{
	#[inline(always)]
	fn from(value: IanaRegisteredIso639Alpha2Code) -> Self
	{
		let iana_registered_iso_639_code: IanaRegisteredIso639Code = value.into();
		OrdinaryLanguage::from(iana_registered_iso_639_code)
	}
}

impl const From<IanaRegisteredIso639Alpha3Code> for OrdinaryLanguage
{
	#[inline(always)]
	fn from(value: IanaRegisteredIso639Alpha3Code) -> Self
	{
		let iana_registered_iso_639_code: IanaRegisteredIso639Code = value.into();
		OrdinaryLanguage::from(iana_registered_iso_639_code)
	}
}

impl<'a> const From<&'a [u8; 2]> for OrdinaryLanguage
{
	#[inline(always)]
	fn from(value: &'a [u8; 2]) -> Self
	{
		Self::from(IanaRegisteredIso639Code::from(value))
	}
}

impl const From<[u8; 2]> for OrdinaryLanguage
{
	#[inline(always)]
	fn from(value: [u8; 2]) -> Self
	{
		Self::from(IanaRegisteredIso639Code::from(value))
	}
}

impl<'a> const From<&'a [u8; 3]> for OrdinaryLanguage
{
	#[inline(always)]
	fn from(value: &'a [u8; 3]) -> Self
	{
		Self::from(IanaRegisteredIso639Code::from(value))
	}
}

impl const From<[u8; 3]> for OrdinaryLanguage
{
	#[inline(always)]
	fn from(value: [u8; 3]) -> Self
	{
		Self::from(IanaRegisteredIso639Code::from(value))
	}
}

impl OrdinaryLanguage
{
	#[inline(always)]
	fn parse_2<'a>(first_subtag: &'a [u8], subtags: &mut Subtags<'a>) -> Result<Either<(Language, NextSubtagAfterLanguageExtension<'a>), Bcp47LanguageTag>, LanguageSubtagParseError>
	{
		Self::parse_n::<_, 2>(first_subtag, subtags, |alpha_array| IanaRegisteredIso639Code::Alpha2(IanaRegisteredIso639Alpha2Code(alpha_array)))
	}
	
	#[inline(always)]
	fn parse_3<'a>(first_subtag: &'a [u8], subtags: &mut Subtags<'a>) -> Result<Either<(Language, NextSubtagAfterLanguageExtension<'a>), Bcp47LanguageTag>, LanguageSubtagParseError>
	{
		Self::parse_n::<_, 3>(first_subtag, subtags, |alpha_array| IanaRegisteredIso639Code::Alpha3(IanaRegisteredIso639Alpha3Code(alpha_array)))
	}
	
	#[inline(always)]
	fn parse_n<'a, Constructor: FnOnce([Alpha; length]) -> IanaRegisteredIso639Code, const length: usize>(first_subtag: &'a [u8], subtags: &mut Subtags<'a>, constructor: Constructor) -> Result<Either<(Language, NextSubtagAfterLanguageExtension<'a>), Bcp47LanguageTag>, LanguageSubtagParseError>
	{
		let iana_registered_iso_639_code = Alpha::validate_and_convert_array::<_, _, _, _, length>(first_subtag, constructor, LanguageSubtagParseError::InvalidAlpha)?;
		Ok(LanguageExtension::parse(subtags, iana_registered_iso_639_code)?)
	}
}
