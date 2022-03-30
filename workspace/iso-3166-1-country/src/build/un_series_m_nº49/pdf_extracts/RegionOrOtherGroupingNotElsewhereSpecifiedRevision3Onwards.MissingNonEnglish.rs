// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Introduced with Revision 3.
///
/// TODO: Non-English names.
pub(super) const RegionOrOtherGroupingNotElsewhereSpecifiedRevision3Onwards: [(M49Code, &'static str); 14] =
{
	#[inline(always)]
	const fn region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(m49_code: &'static [u8; 3], english_name: &'static str) -> (M49Code, &'static str)
	{
		(M49Code::from(m49_code), english_name)
	}
	
	[
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"129", "Caribbean not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"220", "Eastern Asia not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"221", "Eastern Europe not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"290", "Northern Africa not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"471", "Central American Common Market not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"472", "Customs and Economic Union of Central Africa not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"473", "Latin American Integration Association not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"490", "Other Asia not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"527", "Oceania not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"568", "Other Europe not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"577", "Other Africa not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"636", "Rest of America not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"697", "European Free Trade Association not elsewhere specified"),
		region_or_other_grouping_not_elsewhere_specified_revision_3_onwards(b"879", "Western Asia not elsewhere specified"),
	]
};
