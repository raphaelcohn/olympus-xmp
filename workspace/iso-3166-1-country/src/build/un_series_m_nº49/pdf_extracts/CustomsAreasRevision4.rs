// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// TODO: Non-English names.
/// Superficially similar to Revision 3.
/// Note that the United Nations Conference on Trade and Development (UNCTAD) classification, subdivisions of target economies (<https://unctadstat.unctad.org/en/Classifications/DimCountries_Territories_Hierarchy.pdf>, 09 June 2021) has a different definition of "251", and additionally includes "175" (Mayotte), "254" (French Guiana), "312" (Guadeloupe), "474" (Martinique) and "638" (Réunion).
pub(super) const CustomsAreasRevision4: [(M49Code, &'static str, &'static [M49Code]); 8] =
[
	customs_area_revision_3_or_4(b"058", "Belgium-Luxembourg", constituents![b"056", b"442"]),
	customs_area_revision_3_or_4(b"251", "France-Monaco", constituents![b"250", b"492"]),
	customs_area_revision_3_or_4(b"381", "Italy-San Marino-Holy See", constituents![b"336", b"380", b"674"]),
	customs_area_revision_3_or_4(b"579", "Norway including Svalbard and Jan Mayen Islands", constituents![b"578", b"744"]),
	customs_area_revision_3_or_4(b"757", "Switzerland-Liechtenstein", constituents![b"438", b"756"]),
	customs_area_revision_3_or_4(b"841", "United States including Puerto Rico", constituents![b"630", b"840"]),
	customs_area_revision_3_or_4(b"842", "United States including Puerto Rico and United States Virgin Islands", constituents![b"630", b"840", b"850"]),
	
	// Unofficial, but used by the United Nations Conference on Trade and Development (UNCTAD) classification, subdivisions of target economies (<https://unctadstat.unctad.org/en/Classifications/DimCountries_Territories_Hierarchy.pdf>, 09 June 2021).
	// This has been defined differently to the definition presented there, as it would require including a non-country region ("830", (Channel Islands)).
	// Instead, Jersey, Guernsey and Sark have been included directly.
	customs_area_revision_3_or_4(b"926", "United Kingdom", constituents![b"680", b"826", b"831", b"832", b"833"]),
];

/*

001,World,150,Europe,154,Northern Europe,830,Channel Islands,Guernsey,831,GG,GGY,,,
001,World,150,Europe,154,Northern Europe,830,Channel Islands,Jersey,832,JE,JEY,,,
001,World,150,Europe,154,Northern Europe,830,Channel Islands,Sark,680,,,,,
 */
/* Russian

, "Бельгия — Люксембург"
, "Италия — Сан-Марино — Святейший Престол"
, "Норвегия, включая острова Свальбард и Ян-Майен"
, "Соединенные Штаты, включая Пуэрто-Рико"
, "Соединены Штаты, включая Пуэрто-Рико и Виргинские острова Соединенных Штатов"
, "Франция — Монако Швейцария — Лихтенштейн"
 */

/* Chinese
    
, "比利时-卢森堡"
, "法理-摩纳哥"
, "意大利叩圣马力诺"
, "教廷 挥威包括黯瓦幸也知扬马延群岛"
, "瑞士-到文软士登"
, "美国包插法多黎各"
, "美菌包括波多黎各和美属维东京群岛"
 */
