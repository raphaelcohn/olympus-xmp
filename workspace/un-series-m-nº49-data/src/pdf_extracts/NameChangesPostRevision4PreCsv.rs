// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
const NameChangesPostRevision4PreCsv: [(M49Code, StaticEnglishName); 1] =
{
	#[inline(always)]
	const fn name_change_post_revision_4_pre_csv(m49_code: StaticM49Code, previous_english_name: StaticEnglishName) -> (M49Code, StaticEnglishName)
	{
		(M49Code::from(m49_code), previous_english_name)
	}
	
	[
		name_change_post_revision_4_pre_csv(b"535", "Bonaire, Saint Eustatius and Saba")
	]
};
