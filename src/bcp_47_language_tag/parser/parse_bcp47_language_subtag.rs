// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! parse_ordinary_language_subtag
{
	($first_subtag: ident, $subtags: ident, $parse_n: ident) =>
	{
		match OrdinaryLanguage::$parse_n($first_subtag, &mut $subtags)?
		{
			Left(ok) => ok,
			
			Right(regular_grandfathered) => return Ok(regular_grandfathered),
		}
	}
}

macro_rules! parse_reserved_language_subtag
{
	($first_subtag: ident) =>
	{
		(ReservedLanguageSubtag::parse($first_subtag)?, Pending)
	}
}

macro_rules! parse_registered_language_subtag
{
	($first_subtag: ident, $n: expr, $alpha_n: ident) =>
	{
		(RegisteredLanguageSubtag::parse::<_, $n>($first_subtag, RegisteredLanguageSubtag::$alpha_n)?, Pending)
	}
}

#[inline(always)]
pub(super) fn parse_bcp47_language_subtag(mut subtags: MemchrIterator<Hyphen>) -> Result<Bcp47LanguageTag, Bcp47LanguageTagParseError>
{
	let (language, next_subtag) =
	{
		use LanguageFirstSubtagParseError::*;
		
		let first_subtag = subtags.next_first();
		match first_subtag.len()
		{
			0 => Err(FirstSubtagLengthIsZero)?,
			
			1 => return Bcp47LanguageTag::parse_x_or_i_subtag(first_subtag, subtags),
			
			2 => parse_ordinary_language_subtag!(first_subtag, subtags, parse_2),
			
			3 => parse_ordinary_language_subtag!(first_subtag, subtags, parse_3),
			
			4 => parse_reserved_language_subtag!(first_subtag),
			
			5 => parse_registered_language_subtag!(first_subtag, 5, Alpha5),
			
			6 => parse_registered_language_subtag!(first_subtag, 6, Alpha6),
			
			7 => parse_registered_language_subtag!(first_subtag, 7, Alpha7),
			
			8 => parse_registered_language_subtag!(first_subtag, 8, Alpha8),
			
			_ => Err(FirstSubtagLengthIsMoreThanEight)?,
		}
	};
	
	use NextSubtag::*;
	match next_subtag
	{
		Exhausted => Bcp47LanguageTag::from_language_ok(language),
		
		Pending => match subtags.next()
		{
			None => Bcp47LanguageTag::from_language_ok(language),
			
			Some(subtag) => parse_from_script(subtag, subtags, language),
		},
		
		Next(subtag) => parse_from_script(subtag, subtags, language),
		
		IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code) => parse_from_variant(subtags, language, None, Some(IanaRegisteredRegionCode::IanaRegisteredUnM49RegionCode(iana_registered_un_m49_region_code)))
	}

	// TODO Script 0 or 1
	// TODO Region 0 or 1
	// TODO variant 0+
	// TODO extension 0+
	// TODO private use 0 or 1
	
	/*
		TODO Irregular grandfathered to check for before throwing an error at OrdinaryLanguage(no extension), Script=None, Region=Some(Alpha2), Variant=Some(WRONG(3) | WRONG(2))
		
				 "en-GB-oed"
		       / "sgn-BE-FR"
               / "sgn-BE-NL"
               / "sgn-CH-DE"
		
		TODO Regular grandfathered that will match OrdinaryLanguage(no extension), Script=None, Region=None, Variant=Some(RIGHT(5..8)):-
		
                 "art-lojban"
               / "cel-gaulish"
               / "zh-guoyu"
               / "zh-hakka"
               / "zh-xiang"
		
		
		
	 */
}

#[inline(always)]
fn parse_from_script<'a>(subtag: &'a [u8], mut subtags: MemchrIterator<'a, Hyphen>, language: Language) -> Result<Bcp47LanguageTag, Bcp47LanguageTagParseError>
{
	unimplemented!()
}

#[inline(always)]
fn parse_from_variant<'a>(mut subtags: MemchrIterator<'a, Hyphen>, language: Language, script: Option<IanaRegisteredIso15924ScriptCode>, region: Option<IanaRegisteredRegionCode>) -> Result<Bcp47LanguageTag, Bcp47LanguageTagParseError>
{
	unimplemented!()
}
