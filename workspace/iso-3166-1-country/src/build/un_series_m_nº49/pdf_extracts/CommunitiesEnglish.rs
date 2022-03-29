// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
pub(super) const CommunitiesEnglish: [(M49Code, u8, &'static str, &'static [&'static str]); 18] =
{
	const NoHistoricNames: &'static [&'static str] = &[];
	
	#[inline(always)]
	const fn community(code: &[u8; 3], introduced_in_revision: u8, current_name: &'static str, historic_names: &'static [&'static str]) -> (M49Code, u8, &'static str, &'static [&'static str])
	{
		if introduced_in_revision < 2
		{
			panic!("introduced_in_revision can not be less than 2")
		}
		(M49Code::from(code), introduced_in_revision, current_name, historic_names)
	}
	
	[
		community(b"063", 3, "Andean Common Market (ANCOM)", NoHistoricNames),
		community(b"065", 3, "ASEAN Free Trade Area (AFTA)", NoHistoricNames),
		community(b"066", 3, "Asia-Pacific Economic Cooperation (APEC)", NoHistoricNames),
		community(b"069", 3, "Mercado Comun Sudamericano (MERCOSUR)", NoHistoricNames),
		community(b"071", 3, "North American Free Trade Agreement (NAFTA)", NoHistoricNames),
		community(b"073", 4, "Association of Southeast Asian Nations (ASEAN)", NoHistoricNames),
		community(b"095", 2, "Latin American Integration Association (LAIA)", NoHistoricNames),
		community(b"097", 2, "European Union (EU)", &["European Community (EC)", "European Economic Community (EEC)"]),
		community(b"130", 4, "Caribbean Community and Common Market (CARICOM)", NoHistoricNames),
		community(b"171", 4, "Common Market for Eastern and Southern Africa (COMESA)", NoHistoricNames),
		community(b"172", 4, "Commonwealth of Independent States (CIS)", NoHistoricNames),
		community(b"197", 2, "European Free Trade Association (EFTA)", NoHistoricNames),
		community(b"198", 3, "Organisation for Economic Co-operation and Development (OECD)", NoHistoricNames),
		community(b"395", 3, "Central American Common Market (CACM)", NoHistoricNames),
		community(b"399", 2, "Organization of the Petroleum Exporting Countries (OPEC)", NoHistoricNames),
		community(b"692", 2, "Central African Customs and Economic Union (CACEU)", &["Customs and Economic Union of Central Africa (CEUCA)"]),
		community(b"711", 3, "South African Customs Union (SACU)", NoHistoricNames),
		community(b"892", 2, "Economic Community of West African States (ECOWAS)", NoHistoricNames),
	]
};
