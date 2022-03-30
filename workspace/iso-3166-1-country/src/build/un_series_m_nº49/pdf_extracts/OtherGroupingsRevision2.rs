// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Extracted from the PDF for UNSD Series M, Nº49 Revision 2 (1982).
///
/// Non-English names were not officially published.
pub(super) const OtherGroupingsRevision2: [(M49Code, &'static str); 7] =
{
	#[inline(always)]
	const fn other_grouping_revision_2(code: &[u8; 3], english_name: &'static str) -> (M49Code, &'static str)
	{
		(M49Code::from(code), english_name)
	}
	[
		other_grouping_revision_2(b"095", "Latin American Integration Association (LAIA)"),
		other_grouping_revision_2(b"097", "European Economic Community (EEC)"),
		other_grouping_revision_2(b"197", "European Free Trade Association (EFTA)"),
		other_grouping_revision_2(b"199", "Least developed countries (LDCs)"),
		other_grouping_revision_2(b"399", "Organization of the Petroleum Exporting Countries (OPEC)"),
		other_grouping_revision_2(b"692", "Customs and Economic Union of Central Africa (CEUCA)"),
		other_grouping_revision_2(b"892", "Economic Community of West African States (ECOWAS)"),
	]
};
