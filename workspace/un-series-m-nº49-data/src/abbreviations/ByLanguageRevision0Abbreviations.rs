// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Only exist in the original 1970 version of Series M, Nº49.
///
/// Represent the set of abbreviations for either English or French
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct ByLanguageRevision0Abbreviations
{
	pub(super) twelve_character_abbreviation: TwelveCharacterAbbreviation,
	
	pub(super) eight_character_abbreviation: EightCharacterAbbreviation,
	
	pub(super) four_character_abbreviation: FourCharacterAbbreviation,
}
