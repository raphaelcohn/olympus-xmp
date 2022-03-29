// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Only exist in the original 1970 version of Series M, Nº49.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct LegacyFrenchAbbreviations
{
	twelve_character_abbreviation: TwelveCharacterAbbreviation,
	
	eight_character_abbreviation: LegacyEightCharacterAbbreviation,
	
	four_character_abbreviation: LegacyFourCharacterAbbreviation,
}

impl LegacyFrenchAbbreviations
{
	#[inline(always)]
	pub(super) const fn new(twelve_character_abbreviation: &'static [u8], eight_character_abbreviation: &'static [u8], four_character_abbreviation: &'static [u8]) -> Self
	{
		Self
		{
			twelve_character_abbreviation: TwelveCharacterAbbreviation::new(twelve_character_abbreviation),
			eight_character_abbreviation: LegacyEightCharacterAbbreviation::new(eight_character_abbreviation),
			four_character_abbreviation: LegacyFourCharacterAbbreviation::new(eight_character_abbreviation),
		}
	}
}
