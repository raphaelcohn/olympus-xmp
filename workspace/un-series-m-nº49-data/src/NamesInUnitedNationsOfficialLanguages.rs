// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamesInUnitedNationsOfficialLanguages
{
	pub arabic_name: StaticArabicName,
	
	pub chinese_name: StaticChineseName,
	
	pub names_in_english_and_french: NamesInEnglishAndFrench,
	
	pub russian_name: StaticRussianName,
	
	pub spanish_name: StaticSpanishName,
}

impl AsMut<NamesInUnitedNationsOfficialLanguages> for NamesInUnitedNationsOfficialLanguages
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut Self
	{
		self
	}
}

impl NamesInUnitedNationsOfficialLanguages
{
	#[inline(always)]
	const fn new(arabic_name: StaticArabiccName, chinese_name: StaticChineseName, english_name: StaticEnglishName, french_name: StaticFrenchName, russian_name: StaticRussianName, spanish_name: StaticSpanishName) -> Self
	{
		use UnitedNationsOfficialLanguage::*;
		
		Self
		{
			arabic_name: Self::validate_not_empty::<Arabic>(arabic_name),
			chinese_name: Self::validate_not_empty::<Chinese>(chinese_name),
			names_in_english_and_french: NamesInEnglishAndFrench::new(english_name, french_name),
			russian_name: Self::validate_not_empty::<Russian>(russian_name),
			spanish_name: Self::validate_not_empty::<Spanish>(spanish_name),
		}
	}
	
	#[inline(always)]
	const fn validate_not_empty<const language: UnitedNationsOfficialLanguage>(name: StaticName) -> StaticName
	{
		if name.is_empty()
		{
			let message = language.as_str();
			panic!("{} is empty", message);
		}
		name
	}
}
