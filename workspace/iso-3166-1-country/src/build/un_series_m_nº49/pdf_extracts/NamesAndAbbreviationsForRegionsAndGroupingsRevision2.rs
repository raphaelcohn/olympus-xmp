// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// This listing was not produced for any other Revision, oddly.
pub(super) const NamesAndAbbreviationsForRegionsAndGroupingsRevision2: [(M49Code, &'static str, (LegacyEightCharacterAbbreviation, TwelveCharacterAbbreviation)); 37] =
{
	#[inline(always)]
	const fn names_and_abbreviations_for_region_and_grouping_revision_2(code: &'static [u8; 3], english_name: &'static str, legacy_english_eight_character_abbreviation: &'static [u8], english_twelve_character_abbreviation: &'static [u8]) -> (M49Code, &'static str, (LegacyEightCharacterAbbreviation, TwelveCharacterAbbreviation))
	{
		(M49Code::from(m49_code), english_name, (LegacyEightCharacterAbbreviation::new(legacy_english_eight_character_abbreviation), TwelveCharacterAbbreviation::new(english_twelve_character_abbreviation)))
	}
	
	[
		names_and_abbreviations_for_region_and_grouping_revision_2(b"002", "Africa", b"AFRICA", b"AFRICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"003", "North America", b"N.AMRCA", b"NTH.AMERICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"005", "South America", b"S.AMRCA", b"STH.AMERICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"006", "Asia", b"ASIA", b"ASIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"007", "Europe", b"EUROPE", b"EUROPE"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"009", "Oceania", b"OCEANIA", b"OCEANIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"014", "Western Africa", b"WN.AFRCA", b"WSTN.AFRICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"013", "Central America", b"CN.AMRCA", b"CNTL.AMERICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"014", "Eastern Africa", b"EN.AFRCA", b"ESTN.AFRICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"015", "Northern Africa", b"NN.AFRCA", b"NTHN.AFRICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"017", "Middle Africa", b"MD.AFRCA", b"MDLE.AFRICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"018", "Southern Africa", b"SN.AFRCA", b"STHN.AFRICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"019", "Americas", b"AMERICAS", b"AMERICAS"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"021", "Northern America", b"NN.AMRCA", b"NTHN.AMERICA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"029", "Caribbean", b"CARBBEAN", b"CARIBBEAN"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"030", "Eastern Asia", b"EN.ASIA", b"EASTERN.ASIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"033", "Southern Asia", b"SN.ASIA", b"STHN.ASIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"035", "Southeastern Asia", b"S.EN.ASA", b"ST.ESTN.ASIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"037", "Western Asia", b"WN.ASIA", b"WESTERN.ASIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"038", "Western Europe", b"WN.EUR.", b"WSTN.EUROPE"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"039", "Southern Europe", b"SN.EUR.", b"STHN.EUROPE"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"041", "Eastern Europe", b"EN.EUR.", b"ESTN.EUROPE"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"042", "Northern Europe", b"NN.EUR.", b"NTHN.EUROPE"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"043", "Australia and New Zealand", b"AUS.N.Z.", b"AUS.N.Z."),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"045", "Melanesia", b"MELNESIA", b"MELANESIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"046", "Micronesia-Polynesia", b"MCNSA.P", b"MCRNSA.POLYN"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"047", "Micronesia", b"MICRNSIA", b"MICRONESIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"049", "Polynesia", b"POLNESIA", b"POLYNESIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"095", "Latin American Integration Association", b"LAIA", b"LAIA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"097", "European Economic Community", b"EEC", b"EEC"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"197", "European Free Trade Association", b"EFTA", b"EFTA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"199", "Least Developed Countries", b"LDC", b"LDC"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"393", "Centrai American Common Market", b"CACM", b"CACM"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"399", "Organization of Petroleum Exporting Countries", b"OPEC", b"OPEC"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"692", "Customs and Economic Union of Central Africa", b"CEUCA", b"CEUCA"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"830", "Channel Islands", b"CHNL IS.", b"CHANNL ISLDS"),
		names_and_abbreviations_for_region_and_grouping_revision_2(b"892", "Economic Community of West African States", b"ECOWAS", b"ECOWAS"),
	]
}
