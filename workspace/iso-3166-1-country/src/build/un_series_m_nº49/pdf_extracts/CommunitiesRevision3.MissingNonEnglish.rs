// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Extracted from the PDF for UNSD Series M, Nº49 Revision 3 (1995).
///
/// TODO: Non-English names.
pub(super) const CommunitiesRevision3: [(M49Code, &'static str); 14] =
{
	#[inline(always)]
	const fn community_revision_3(code: &[u8; 3], english_name: &'static str) -> (M49Code, &'static str)
	{
		(M49Code::from(code), english_name)
	}
	[
		community_revision_3(b"063", "Andean Common Market (ANCOM)"),
		community_revision_3(b"065", "ASEAN Free Trade Area (AFTA)"),
		community_revision_3(b"066", "Asia-Pacific Economic Cooperation (APEC)"),
		community_revision_3(b"069", "Mercado Comun Sudamericano (MERCOSUR)"),
		community_revision_3(b"071", "North American Free Trade Agreement (NAFTA)"),
		community_revision_3(b"095", "Latin American Integration Association (LAIA)"),
		community_revision_3(b"097", "European Community (EC)"),
		community_revision_3(b"197", "European Free Trade Association (EFTA)"),
		community_revision_3(b"198", "Organisation for Economic Co-operation and Development (OECD)"),
		community_revision_3(b"395", "Central American Common Market (CACM)"),
		community_revision_3(b"399", "Organization of the Petroleum Exporting Countries (OPEC)"),
		community_revision_3(b"692", "Customs and Economic Union of Central Africa (CEUCA)"),
		community_revision_3(b"711", "South African Customs Union (SACU)"),
		community_revision_3(b"892", "Economic Community of West African States (ECOWAS)"),
	]
};
