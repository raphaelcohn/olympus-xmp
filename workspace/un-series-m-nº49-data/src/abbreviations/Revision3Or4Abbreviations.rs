// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Defined for Revision 3 and Revision 4 of Series M, Nº49.
///
/// Only defined for countries and some statistical like countries.
///
/// Seem to have ceased being used sometime after Revision 4.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Revision3Or4Abbreviations
{
	pub(super) english_twelve_character_abbreviation: TwelveCharacterAbbreviation,
}
