// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Extracted from the PDF for UNSD Series M, Nº49 Revision 4 (1999).
///
/// TODO: Non-English names.
pub(super) const CommunitiesRevision4: [(M49Code, &'static str); 14] =
{
	#[inline(always)]
	const fn community_revision_4(code: &[u8; 3], english_name: &'static str) -> (M49Code, &'static str)
	{
		(M49Code::from(code), english_name)
	}
	[
		community(b"063", "Andean Common Market (ANCOM)"),
		community(b"065", "ASEAN Free Trade Area (AFTA)"),
		community(b"066", "Asia-Pacific Economic Cooperation (APEC)"),
		community(b"069", "Mercado Comun Sudamericano (MERCOSUR)"),
		community(b"071", "North American Free Trade Agreement (NAFTA)"),
		community(b"073", "Association of Southeast Asian Nations (ASEAN)"),
		community(b"095", "Latin American Integration Association (LAIA)"),
		community(b"097", "European Union (EU)"),
		community(b"130", "Caribbean Community and Common Market (CARICOM)"),
		community(b"171", "Common Market for Eastern and Southern Africa (COMESA)"),
		community(b"172", "Commonwealth of Independent States (CIS)"),
		community(b"197", "European Free Trade Association (EFTA)"),
		community(b"198", "Organisation for Economic Co-operation and Development (OECD)"),
		community(b"395", "Central American Common Market (CACM)"),
		community(b"399", "Organization of the Petroleum Exporting Countries (OPEC)"),
		community(b"692", "Central African Customs and Economic Union (CACEU)"),
		community(b"711", "South African Customs Union (SACU)"),
		community(b"892", "Economic Community of West African States (ECOWAS)"),
	]
};
