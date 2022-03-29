// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) enum Abbreviations
{
	AbsentRevision2Onwards,
	
	Revision2Onwards
	{
		english_twelve_character_abbreviation: TwelveCharacterAbbreviation,
	},
	
	// Present in Revision 1 (1975) and Revision 2 (1982).
	Revision1AndRevision2
	{
		english_twelve_character_abbreviation: TwelveCharacterAbbreviation,
		
		legacy: LegacyEightCharacterAbbreviation,
	},
	
	// Present only in the original (1970).
	Revision0
	{
		english_twelve_character_abbreviation: TwelveCharacterAbbreviation,
		
		legacy: Legacy1970Abbreviations,
	},
}

impl Abbreviations
{
	#[inline(always)]
	pub(super) const fn revision_0(english_twelve_character_abbreviation: &'static [u8], english_legacy_eight_character_abbreviation: &'static [u8], english_legacy_four_character_abbreviation: &'static [u8], french_legacy_twelve_character_abbreviation: &'static [u8], french_legacy_eight_character_abbreviation: &'static [u8], french_legacy_four_character_abbreviation: &'static [u8]) -> Self
	{
		Abbreviations::Revision0
		{
			english_twelve_character_abbreviation: TwelveCharacterAbbreviation::new(english_twelve_character_abbreviation),
			
			legacy: Legacy1970Abbreviations::new(english_legacy_four_character_abbreviation, french_legacy_twelve_character_abbreviation, french_legacy_eight_character_abbreviation, french_legacy_four_character_abbreviation),
		}
	}
}
