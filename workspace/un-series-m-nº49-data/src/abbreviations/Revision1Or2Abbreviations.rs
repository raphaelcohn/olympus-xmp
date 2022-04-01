// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Only exist in the Revisions 1 and 2 of Series M, Nº49.
///
/// Only exist for regions and other groupings in Revision 2.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Revision1Or2Abbreviations
{
	pub(super) english_twelve_character_abbreviation: TwelveCharacterAbbreviation,
	
	pub(super) english_eight_character_abbreviation: EightCharacterAbbreviation,
}
