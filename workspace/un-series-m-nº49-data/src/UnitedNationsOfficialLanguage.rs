// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum UnitedNationsOfficialLanguage
{
	Arabic,
	
	Chinese,
	
	English,

	French,

	Russian,

	Spanish,
}

impl UnitedNationsOfficialLanguage
{
	#[inline(always)]
	const fn as_str(self) -> &'static str
	{
		use UnitedNationsOfficialLanguage::*;
		
		match self
		{
			Arabic => "Arabic",
			
			Chinese => "Chinese",
			
			English => "English",
			
			French => "French",
			
			Russian => "Russian",
			
			Spanish => "Spanish",
		}
	}
	
	#[inline(always)]
	fn initial(self, names: &mut NamesInUnitedNationsOfficialLanguages, non_empty_name: &'static str)
	{
		use UnitedNationsOfficialLanguage::*;
		
		let field = match self
		{
			Arabic => &mut names.arabic_name,
			
			Chinese => &mut names.chinese_name,
			
			English => &mut names.english_name,
			
			French => &mut names.french_name,
			
			Russian => &mut names.russian_name,
			
			Spanish => &mut names.spanish_name,
		};
		
		let was = *field;
		assert!(was.is_empty(), "Name was not previously empty but was {}", was);
		*field = non_empty_name;
	}
	
	#[inline(always)]
	fn is_english(self) -> bool
	{
		self == UnitedNationsOfficialLanguage::English
	}
}
