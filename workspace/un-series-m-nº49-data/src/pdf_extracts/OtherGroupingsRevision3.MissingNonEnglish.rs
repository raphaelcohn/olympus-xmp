// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Extracted from the PDF for UNSD Series M, Nº49 Revision 3 (1995).
///
/// TODO: Non-English names.
/// Includes the "Least developed countries (LDCs)": "Classified by the General Assembly as of 31 March 1996. The General Assembly, on the recommendation of the Committee for Development Planning, decides on the countries to be included in the list of the least developed countries".
pub(super) const OtherGroupingsRevision3: [(M49Code, StaticEnglishName); 15] =
{
	#[inline(always)]
	const fn other_grouping_revision_3(m49_code: StaticM49Code, english_name: StaticEnglishName) -> (M49Code, StaticEnglishName)
	{
		(M49Code::from(m49_code), english_name)
	}
	
	[
		other_grouping_revision_3(b"063", "Andean Common Market (ANCOM)"),
		other_grouping_revision_3(b"065", "ASEAN Free Trade Area (AFTA)"),
		other_grouping_revision_3(b"066", "Asia-Pacific Economic Cooperation (APEC)"),
		other_grouping_revision_3(b"069", "Mercado Comun Sudamericano (MERCOSUR)"),
		other_grouping_revision_3(b"071", "North American Free Trade Agreement (NAFTA)"),
		other_grouping_revision_3(b"095", "Latin American Integration Association (LAIA)"),
		other_grouping_revision_3(b"097", "European Community (EC)"),
		other_grouping_revision_3(b"197", "European Free Trade Association (EFTA)"),
		other_grouping_revision_3(b"198", "Organisation for Economic Co-operation and Development (OECD)"),
		other_grouping_revision_3(b"199", "Least developed countries (LDCs)"),
		other_grouping_revision_3(b"395", "Central American Common Market (CACM)"),
		other_grouping_revision_3(b"399", "Organization of the Petroleum Exporting Countries (OPEC)"),
		other_grouping_revision_3(b"692", "Customs and Economic Union of Central Africa (CEUCA)"),
		other_grouping_revision_3(b"711", "South African Customs Union (SACU)"),
		other_grouping_revision_3(b"892", "Economic Community of West African States (ECOWAS)"),
	]
};
