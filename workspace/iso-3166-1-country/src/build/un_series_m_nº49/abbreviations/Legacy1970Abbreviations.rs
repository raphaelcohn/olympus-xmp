// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Only exist in the original 1970 version of Series M, Nº49.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Legacy1970Abbreviations
{
	english_legacy_four_character_abbreviation: LegacyFourCharacterAbbreviation,
	
	french_legacy_twelve_character_abbreviation: TwelveCharacterAbbreviation,
	
	french_legacy_eight_character_abbreviation: LegacyEightCharacterAbbreviation,
	
	french_legacy_four_character_abbreviation: LegacyFourCharacterAbbreviation,
}

impl Legacy1970Abbreviations
{
	#[inline(always)]
	pub(super) const fn new(english_legacy_four_character_abbreviation: &'static [u8], french_legacy_twelve_character_abbreviation: &'static [u8], french_legacy_eight_character_abbreviation: &'static [u8], french_legacy_four_character_abbreviation: &'static [u8]) -> Self
	{
		Self
		{
			english_legacy_four_character_abbreviation: LegacyFourCharacterAbbreviation::new(english_legacy_four_character_abbreviation),
			french_legacy_twelve_character_abbreviation: TwelveCharacterAbbreviation::new(french_legacy_twelve_character_abbreviation),
			french_legacy_eight_character_abbreviation: LegacyEightCharacterAbbreviation::new(french_legacy_eight_character_abbreviation),
			french_legacy_four_character_abbreviation: LegacyFourCharacterAbbreviation::new(french_legacy_eight_character_abbreviation),
		}
	}
}
