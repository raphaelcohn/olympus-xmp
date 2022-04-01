// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Parents are an assumption.
/// United Nations Conference on Trade and Development (UNCTAD) classification names are preferred.
///
/// Sources of information:-
///
/// 1. United Nations Conference on Trade and Development (UNCTAD) classification, subdivisions of target economies (<https://unctadstat.unctad.org/en/Classifications/DimCountries_Territories_Hierarchy.pdf>, 09 June 2021).
/// 2. UN Comtrade (<https://comtrade.un.org/data/cache/reporterAreas.json>).
/// 3. FAO Caliper (eg <https://stats-class.fao.uniroma2.it/geo/m49/836> or <https://datalab.review.fao.org/datalab/caliper/web/concept-page/836-united-republic-tanzania-zanzibar>).
/// 4. SDMX's CL_AREA version 2 definition [`CL_AREA_2_March_2019.docx` from SDMX](https://sdmx.org/wp-content/uploads/CL_AREA_2_0_March_2019.docx)).
/// 5. Internal UNSD usage (<https://www.alvestrand.no/pipermail/ietf-languages/2007-August/006904.html>).
const UnofficialCountrySubdivision: [(M49Code, (&'static [StaticEnglishName], Option<StaticFrenchName>, Option<StaticSpanishName>), M49Code); 13] =
{
	#[inline(always)]
	const fn unofficial_country_subdivision(m49_code: StaticM49Code, english_names: &'static [StaticEnglishName], french_name: Option<StaticFrenchName>, spanish_name: Option<StaticSpanishName>, parent_country_m49_code: StaticM49Code) -> (M49Code, (&'static [StaticEnglishName], Option<StaticFrenchName>, Option<StaticSpanishName>), M49Code)
	{
		(M49Code::from(m49_code), (english_names, french_name, spanish_name), M49Code::from(parent_country_m49_code))
	}
	
	[
		// Subdivisions of Malaysia, sourced from (2) and (3).
		unofficial_country_subdivision(b"457", &["Sarawak"], None, None, b"458"),
		unofficial_country_subdivision(b"459", &["Peninsula Malaysia"], None, None, b"458"),
		unofficial_country_subdivision(b"461", &["Sabah"], None, None, b"458"),
		
		// Subdivsions of Tanzania, sourced from (1), (2) and (3).
		// Called:
		//	* `Zanzibar and Pemba Island` by (1).
		//  * `United Rep. of Tanzania (Zanzibar)` by (2).
		//  * `Fmr Zanzibar and Pemba Isd` by (3).
		unofficial_country_subdivision(b"836", &["Zanzibar and Pemba Island", "United Rep. of Tanzania (Zanzibar)", "Fmr Zanzibar and Pemba Isd"], None, None, b"834"),
		// Called:
		//  * `Tanganyika` by (1).
		//  * `United Republic of Tanzania, mainland (ex Tanganyika)` by (2); there are also French and Spanish names defined by the FAO at <http://stats-class.fao.uniroma2.it/geo/m49/835>.
		//	* `Fmr Tanganyika` by (3).
		unofficial_country_subdivision(b"835", &["Tanganyika", "United Republic of Tanzania, mainland (ex Tanganyika)", "Fmr Tanganyika"], Some("République-Unie de Tanzanie, continentale (ex Tanganyika)"), Some("República Unida de Tanzania, Continental (ex Tanganica)"), b"834"),
		
		// Subdivisons of St Helena, sourced from (4).
		unofficial_country_subdivision(b"655", &["Ascension"], None, None, b"654"),
		unofficial_country_subdivision(b"656", &["Tristan da Cunha"], None, None, b"654"),
		
		// Subdivisons of Bonaire, Sint Eustatius and Saba, sourced from (4).
		unofficial_country_subdivision(b"667", &["Saba"], None, None, b"535"),
		unofficial_country_subdivision(b"668", &["Sint Eustatius"], None, None, b"535"),
		unofficial_country_subdivision(b"669", &["Bonaire"], None, None, b"535"),
		
		// Subdivsions of United Kingdom, sourced from (5).
		unofficial_country_subdivision(b"827", &["England and Wales"], None, None, b"826"),
		unofficial_country_subdivision(b"828", &["Northern Ireland"], None, None, b"826"),
		unofficial_country_subdivision(b"829", &["Scotland"], None, None, b"826"),
	]
};
