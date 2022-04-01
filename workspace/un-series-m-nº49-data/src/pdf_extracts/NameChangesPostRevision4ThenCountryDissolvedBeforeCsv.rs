// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
const NameChangesPostRevision4ThenCountryDissolvedBeforeCsv: [(M49Code, NamesInEnglishAndFrench); 1] =
{
	#[inline(always)]
	const fn name_change_post_revision_4_then_country_dissolved_before_csv(m49_code: StaticM49Code, english_name: StaticEnglishName, french_name: StaticFrenchName) -> (M49Code, NamesInEnglishAndFrench)
	{
		(M49Code::from(m49_code), NamesInEnglishAndFrench::new(english_name, french_name))
	}
	
	[
		// Had different ISO codes, too, of CS and SCG, which were used by Czechoslovakiva.
		name_change_post_revision_4_then_country_dissolved_before_csv(b"891", "Serbia and Montenegro", "Serbie-et-Monténégro"),
	]
};
