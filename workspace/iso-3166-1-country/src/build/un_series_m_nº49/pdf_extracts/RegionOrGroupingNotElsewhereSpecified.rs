// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Introduced with Revision 3.
pub(super) const RegionOrGroupingNotElsewhereSpecified: [(M49Code, &'static str); 14] =
{
	#[inline(always)]
	const fn region_or_grouping_not_elsewhere_specified(code: &'static [u8; 3], english_name: &'static str) -> (M49Code, &'static str)
	{
		(M49Code::from(code), english_name)
	}
	
	[
		region_or_grouping_not_elsewhere_specified(b"129", "Caribbean not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"220", "Eastern Asia not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"221", "Eastern Europe not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"290", "Northern Africa not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"471", "Central American Common Market not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"472", "Customs and Economic Union of Central Africa not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"473", "Latin American Integration Association not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"490", "Other Asia not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"527", "Oceania not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"568", "Other Europe not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"577", "Other Africa not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"636", "Rest of America not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"697", "European Free Trade Association not elsewhere specified"),
		region_or_grouping_not_elsewhere_specified(b"879", "Western Asia not elsewhere specified"),
	]
};
