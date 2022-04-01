// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Introduced with Revision 3.
///
/// TODO: Non-English names.
/// Note that "033" ("Southern Asia") and "143" ("Central Asia") were merged into "062" (South-central Asia) in Revision 3; this was continued into Revision 4.
/// `South-central Asia` was then later split into "034" (Southern Asia) and "143" ("Central Asia") after revision 4!
pub(super) const ObsoleteRegionsRevision3Onwards: [(M49Code, StaticEnglishName, StaticConstituents); 14] =
{
	#[inline(always)]
	const fn obsolete_region_revision_3_onwards(old_m49_code: &[u8; 3], english_name: StaticEnglishName, replacements: StaticConstituents) -> (M49Code, StaticEnglishName, StaticConstituents)
	{
		(M49Code::from(old_m49_code), english_name, replacements)
	}
	
	[
		// Not really obsolete, but also not widely used and last properly defined in Revision 2.
		obsolete_region_revision_3_onwards(b"003", "North America", constituents![b"013", b"021", b"029"]),
		
		obsolete_region_revision_3_onwards(b"006", "Asia", constituents![b"142"]),
		obsolete_region_revision_3_onwards(b"007", "Europe", constituents![b"150"]),
		
		// Technically, was transferred to 062, but this has since been made obsolete and broken back up into Southern Asia (032) and Central Asia (143).
		// obsolete_region_revision_3_onwards(b"033", "Southern Asia", constituents![b"062"]),
		obsolete_region_revision_3_onwards(b"033", "Southern Asia", constituents![b"032"]),
		
		obsolete_region_revision_3_onwards(b"037", "Western Asia", constituents![b"145"]),
		obsolete_region_revision_3_onwards(b"038", "Western Europe", constituents![b"155"]),
		obsolete_region_revision_3_onwards(b"041", "Eastern Europe", constituents![b"151"]),
		obsolete_region_revision_3_onwards(b"042", "Northern Europe", constituents![b"154"]),
		obsolete_region_revision_3_onwards(b"043", "Australia and New Zealand", constituents![b"053"]),
		obsolete_region_revision_3_onwards(b"045", "Melanesia", constituents![b"054"]),
		obsolete_region_revision_3_onwards(b"046", "Micronesia-Polynesia", constituents![b"055"]),
		obsolete_region_revision_3_onwards(b"047", "Micronesia", constituents![b"057"]),
		obsolete_region_revision_3_onwards(b"049", "Polynesia", constituents![b"061"]),
	
		// Post revision 4.
		obsolete_region_revision_3_onwards(b"062", "South-central Asia", constituents![b"034", b"143"]),
		
		// Was made obsolete in Revision 3, but then was made non-obsolete post Revision 4 when region 062 was made obsolete!
		// obsolete_region_revision_3_onwards(b"143", "Central Asia", b"062"),
	]
};
