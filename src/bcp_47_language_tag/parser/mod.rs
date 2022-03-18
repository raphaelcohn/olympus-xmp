// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use arrayvec::ArrayVec;
use memchr::memchr;
use restricted_byte::InvalidAlphaError;
use restricted_byte::InvalidAlphanumericError;
use restricted_byte::InvalidDigitError;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use super::Bcp47LanguageTag;
use super::IanaRegisteredIso639Alpha2Code;
use super::IanaRegisteredIso639Alpha3Code;
use super::IanaRegisteredIso639Code;
use super::IrregularGrandfathered;
use super::Language;
use super::RegisteredLanguageSubtag;
use super::ReservedLanguageSubtag;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("finished.rs");
include!("next_mandatory.rs");


pub(super) mod restricted_byte;

include!("Bcp47LanguageTagParseError.rs");
include!("GrandfatheredIrregularISubtagParseError.rs");
include!("Hyphen.rs");
include!("LanguageExtensionTagParseError.rs");
include!("LanguageFirstSubtagParseError.rs");
include!("MemchrIterator.rs");
include!("PrivateUseSubtagsParseError.rs");


fn parse_bcp47_language_tag(language_tag: &str) -> Result<Bcp47LanguageTag, Bcp47LanguageTagParseError>
{
	let mut subtags = MemchrIterator::from_str(language_tag);
	
	let language =
	{
		const i: u8 = b'i';
		const x: u8 = b'x';
		
		let first_subtag = subtags.next_first();
		use Bcp47LanguageTag::*;
		use IanaRegisteredIso639Code::*;
		use Language::*;
		use LanguageFirstSubtagParseError::*;
		use RegisteredLanguageSubtag::*;
		match first_subtag.len()
		{
			0 => Err(FirstSubtagLengthIsZero)?,
			
			1 => match first_subtag.get_unchecked_value_safe(0usize)
			{
				i => return Ok(Grandfathered(super::Grandfathered::Irregular(IrregularGrandfathered::parse_irregular_i(subtags)?))),
				
				x => return Ok(PrivateUse(super::PrivateUse::parse(subtags)?)),
				
				byte @ _ => Err(FirstSubtagLengthIsOneAndIsNotPrivateUseOrGrandfatheredIrregularTag(byte))?,
			},
			
			2 => Iso
			{
				shortest_iso_639_code: Alpha2(IanaRegisteredIso639Alpha2Code::parse(first_subtag)?),
				
				extension:
				{
					unimplemented!()
				}
			},
			
			3 => Iso
			{
				shortest_iso_639_code: Alpha3(IanaRegisteredIso639Alpha3Code::parse(first_subtag)?),
				
				extension:
				{
					unimplemented!()
				}
			},
			
			4 => Reserved(ReservedLanguageSubtag::parse(first_subtag)?),
			
			5 => Registered(RegisteredLanguageSubtag::parse::<_, 5>(first_subtag, Alpha5)?),
			
			6 => Registered(RegisteredLanguageSubtag::parse::<_, 6>(first_subtag, Alpha6)?),
			
			7 => Registered(RegisteredLanguageSubtag::parse::<_, 7>(first_subtag, Alpha7)?),
			
			8 => Registered(RegisteredLanguageSubtag::parse::<_, 8>(first_subtag, Alpha8)?),
			
			_ => Err(FirstSubtagLengthIsMoreThanEight)?,
		}
	};
	
	Ok(unimplemented!())
}
