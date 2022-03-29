// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Introduced with Revision 3.
///
/// Not that "Southern Asia" and "Central Asia" were merged into "062".
pub(super) const ObsoleteRegionsRevision3Onwards: [(M49Code, &'static str, M49Code); 13] =
{
	#[inline(always)]
	const fn obsolete_region_revision_3_onwards(old_code: &[u8; 3], english_name: &'static str, new_code: &[u8; 3]) -> (M49Code, &'static str, M49Code)
	{
		(M49Code::from(old_code), english_name, M49Code::from(new_code))
	}
	
	[
		obsolete_region_revision_3_onwards(b"006", "Asia", b"142"),
		obsolete_region_revision_3_onwards(b"007", "Europe", b"150"),
		obsolete_region_revision_3_onwards(b"033", "Southern Asia", b"062"),
		obsolete_region_revision_3_onwards(b"037", "Western Asia", b"145"),
		obsolete_region_revision_3_onwards(b"038", "Western Europe", b"155"),
		obsolete_region_revision_3_onwards(b"041", "Eastern Europe", b"151"),
		obsolete_region_revision_3_onwards(b"042", "Northern Europe", b"154"),
		obsolete_region_revision_3_onwards(b"043", "Australia and New Zealand", b"053"),
		obsolete_region_revision_3_onwards(b"045", "Melanesia", b"054"),
		obsolete_region_revision_3_onwards(b"046", "Micronesia-Polynesia", b"055"),
		obsolete_region_revision_3_onwards(b"047", "Micronesia", b"057"),
		obsolete_region_revision_3_onwards(b"049", "Polynesia", b"061"),
		obsolete_region_revision_3_onwards(b"143", "Central Asia", b"062"),
	]
};
