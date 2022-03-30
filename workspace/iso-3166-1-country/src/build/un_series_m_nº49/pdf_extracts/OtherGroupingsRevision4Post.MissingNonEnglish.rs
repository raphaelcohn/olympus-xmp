// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Source: <https://www.unido.org/sites/default/files/files/2018-03/Country_Grouping_in_UNIDO_Statistics_2013.pdf>, Page 1.
/// TODO: Non-English names.
pub(super) const OtherGroupingsRevision4Post: [(M49Code, &'static str); 3] =
{
	#[inline(always)]
	const fn other_grouping_post_revision_4(code: &[u8; 3], english_name: &'static str) -> (M49Code, &'static str)
	{
		(M49Code::from(code), english_name)
	}
	
	[
		other_grouping_post_revision_4(b"432", "Land Locked Developing Countries (LLDC)"),
		other_grouping_post_revision_4(b"722", "Small Island Developing States (SIDS)"),
		other_grouping_post_revision_4(b"778", "Transition Countries"),
	]
};
