// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamesInEnglishAndFrench
{
	pub english_name: StaticEnglishName,
	
	pub french_name: StaticFrenchName,
}

impl NamesInEnglishAndFrench
{
	#[inline(always)]
	const fn new(english_name: StaticEnglishName, french_name: StaticFrenchName) -> Self
	{
		use UnitedNationsOfficialLanguage::*;
		
		Self
		{
			english_name: NamesInUnitedNationsOfficialLanguages::validate_not_empty::<English>(english_name),
			french_name: NamesInUnitedNationsOfficialLanguages::validate_not_empty::<French>(french_name),
		}
	}
}
