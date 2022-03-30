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
const UnofficialCountrySubdivision: [(M49Code, &'static str, M49Code); 13] =
{
	#[inline(always)]
	const fn unofficial_country_subdivision(m49_code: &'static [u8; 3], english_name: &str, parent_country_m49_code: &'static [u8; 3]) -> (M49Code, &'static str, M49Code)
	{
		(M49Code::from(m49_code), english_name, M49Code::from(parent_country_m49_code))
	}
	
	[
		// Subdivisions of Malaysia, sourced from (3).
		unofficial_country_subdivision(b"457", "Sarawak", b"458"),
		unofficial_country_subdivision(b"459", "Peninsula Malaysia", b"458"),
		unofficial_country_subdivision(b"461", "Sabah", b"458"),
		
		// Subdivsions of Tanzania, sourced from (1), (2) and (3).
		// Called:
		//	* `Zanzibar and Pemba Island` by (1).
		//  * `United Rep. of Tanzania (Zanzibar)` by (2).
		//  * `Fmr Zanzibar and Pemba Isd` by (3).
		unofficial_country_subdivision(b"836", "Zanzibar and Pemba Island", b"834"),
		// Called:
		//  * `Tanganyika` by (1).
		//  * `United Republic of Tanzania, mainland (ex Tanganyika)` by (2); there are also French and Spanish names defined by the FAO at <http://stats-class.fao.uniroma2.it/geo/m49/835>.
		//	* `Fmr Tanganyika` by (3).
		unofficial_country_subdivision(b"835", "Tanganyika", b"834"),
		
		// Subdivisons of St Helena, sourced from (4).
		unofficial_country_subdivision(b"655", "Ascension", b"654"),
		unofficial_country_subdivision(b"656", "Tristan da Cunha", b"654"),
		
		// Subdivisons of Bonaire, Sint Eustatius and Saba, sourced from (4).
		unofficial_country_subdivision(b"667", "Saba", b"535"),
		unofficial_country_subdivision(b"668", "Sint Eustatius", b"535"),
		unofficial_country_subdivision(b"669", "Bonaire", b"535"),
		
		// Subdivsions of United Kingdom, sourced from (5).
		unofficial_country_subdivision(b"827", "England and Wales", b"826"),
		unofficial_country_subdivision(b"828", "Northern Ireland", b"826"),
		unofficial_country_subdivision(b"829", "Scotland", b"826"),
	]
};
