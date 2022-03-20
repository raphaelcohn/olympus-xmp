// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Basically, a 2 - 8 byte alpha code.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Language
{
	/// Typically a 2 or 3 character ISO 639 code, but can be much more.
	Ordinary(OrdinaryLanguage),
	
	/// Reserved for future use.
	Reserved(ReservedLanguageSubtag),
	
	/// Registered for future use.
	Registered(RegisteredLanguageSubtag),
}

impl const From<OrdinaryLanguage> for Language
{
	#[inline(always)]
	fn from(language: OrdinaryLanguage) -> Self
	{
		Language::Ordinary(language)
	}
}

impl const From<ReservedLanguageSubtag> for Language
{
	#[inline(always)]
	fn from(language: ReservedLanguageSubtag) -> Self
	{
		Language::Reserved(language)
	}
}

impl const From<RegisteredLanguageSubtag> for Language
{
	#[inline(always)]
	fn from(language: RegisteredLanguageSubtag) -> Self
	{
		Language::Registered(language)
	}
}

impl const From<IanaRegisteredIso639Code> for Language
{
	#[inline(always)]
	fn from(iana_registered_iso_639_code: IanaRegisteredIso639Code) -> Self
	{
		let ordinary_language: OrdinaryLanguage = value.into();
		Self::from(ordinary_language)
	}
}

impl const From<IanaRegisteredIso639Alpha2Code> for Language
{
	#[inline(always)]
	fn from(value: IanaRegisteredIso639Alpha2Code) -> Self
	{
		let ordinary_language: OrdinaryLanguage = value.into();
		Self::from(ordinary_language)
	}
}

impl const From<IanaRegisteredIso639Alpha3Code> for Language
{
	#[inline(always)]
	fn from(value: IanaRegisteredIso639Alpha3Code) -> Self
	{
		let ordinary_language: OrdinaryLanguage = value.into();
		Self::from(ordinary_language)
	}
}

impl<'a> const From<&'a [u8; 2]> for Language
{
	#[inline(always)]
	fn from(value: &'a [u8; 2]) -> Self
	{
		Self::from(OrdinaryLanguage::from(value))
	}
}

impl const From<[u8; 2]> for Language
{
	#[inline(always)]
	fn from(value: [u8; 2]) -> Self
	{
		Self::from(OrdinaryLanguage::from(value))
	}
}

impl<'a> const From<&'a [u8; 3]> for Language
{
	#[inline(always)]
	fn from(value: &'a [u8; 3]) -> Self
	{
		Self::from(OrdinaryLanguage::from(value))
	}
}

impl const From<[u8; 3]> for Language
{
	#[inline(always)]
	fn from(value: [u8; 3]) -> Self
	{
		Self::from(OrdinaryLanguage::from(value))
	}
}
