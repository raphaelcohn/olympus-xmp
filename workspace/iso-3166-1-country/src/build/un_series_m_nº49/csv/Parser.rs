// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::build::un_series_m_nº49::abbreviations::Abbreviations;
use crate::build::un_series_m_nº49::LegacyEightCharacterAbbreviation;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(in crate::build) struct Parser(BTreeMap<M49Code, M49CodeType>);

impl Default for Parser
{
	fn default() -> Self
	{
		Self(BTreeMap::default())
	}
}

impl Parser
{
	pub(in crate::build) fn parse() -> BTreeMap<M49Code, M49CodeType>
	{
		let mut this = Self::default();
		this.parse_inner();
		this.0
	}
	
	fn parse_inner(&mut self)
	{
		self.add_known_special_codes_before_all_others();
		self.parse_initial_csv(Series_M_Nº49_English, |names| &mut names.english);
		self.parse_subsequent_csv(Series_M_Nº49_Arabic, |names| &mut names.arabic);
		self.parse_subsequent_csv(Series_M_Nº49_Chinese, |names| &mut names.chinese);
		self.parse_subsequent_csv(Series_M_Nº49_French, |names| &mut names.french);
		self.parse_subsequent_csv(Series_M_Nº49_Russian, |names| &mut names.russian);
		self.parse_subsequent_csv(Series_M_Nº49_Spanish, |names| &mut names.spanish);
	}
	
	fn add_known_special_codes_before_all_others(&mut self)
	{
		self.add_private_use_codes();
		
		macro_rules! replacements
		{
			($first: literal$(, $subsequent: literal)*) =>
			{
				{
					const Replacements: &'static [M49Code] = &[M49Code::from($first)$(, M49Code::from($subsequent))*];
					Replacements
			}	}
		}
		
		const MadeUpAbbreviationForBonaire: &'static [u8] = b"BONAIRE";
		const MadeUpAbbreviationForSerbiaAndMontenegro: &'static [u8] = b"SERBIA.MONTE";
		const PredatesIso3166Dash1: Option<(Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)> = None;
		
		#[inline(always)]
		const fn iso_alpha_codes(alpha_2: &[u8; 2], alpha_3: &[u8; 3]) -> Option<(Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)>
		{
			Some((Iso3166Dash1Alpha2Code(*alpha_2), Iso3166Dash1Alpha3Code(*alpha_3)))
		}
		
		// TODO: Names in other languages.
		self.add_obsolete_subregion(b"062", "", "", "South-central Asia", "", "", "", replacements![b"034", b"143"]);
		self.add_obsolete_other_grouping(b"514", "", "", "Developed regions", "", "", "");
		self.add_obsolete_other_grouping(b"515", "", "", "Developing regions", "", "", "");
		
		self.add_country_like_1999_onwards(b"899", "مناطق غير محددة في أماكن أخرى وغير معروفة", "来另费埠医和不明地区", "Areas not elsewhere specified and unknown", "Zones non spécifiées ailleurs et inconnues", "Районы, не указанные в других местах и неизвестные", "Zonas no especificadas en otra parte y desconocidas");
		self.add_country_like_1996_onwards(b"837", "المستودعات، مخازن السفن", "煤腊、船用补给品", "Bunkers, ship stores", "Combustible de soute et provisions de bord", "Бункеры, судовые запасы", "Tanques de depósito, almacenes de a bordo");
		self.add_country_like_1996_onwards(b"838", "المناطق الحرة", "费由区", "Free zones", "Zones franches", "Свободные зоны", "Zonas francas");
		self.add_country_like_1970(b"000", "مجموع", "总", "Total", "Total", "Итог", "Total", b"TOTAL", b"TOTAL", b"TOT",  b"TOTAL", b"TOTAL", b"TOT");
		self.add_country_like_1970(b"896", "مناطق غير محددة في أماكن أخرى", "来另列地区", "Areas not elsewhere specified", "Zones non spécifiées ailleurs", "Районы, не указанные в других местах", "Zonas no especificadas en otra parte", b"N.E.S.", b"AREA NES", b"ANES", b"REG NON SPEC", b"R.N.SPEC", b"RNS");
		self.add_country_like_1970(b"898", "مناطق غير محددة", "不详地区", "Areas not specified", "Zones non spécifiées", "Неуказанные районы", "Zonas no especificadas", b"N.S.", b"NOT SPEC", b"N.S.", b"NON SPECIFIE", b"NON SPEC", b"N.S.");
		
		self.add_older_name_french(b"896", "Région non spécifée");
		self.add_older_name_english_twelve_character_abbreviation(b"896", b"AREAS N.E.S.");
		self.add_older_name_english_legacy_eight_character_abbreviation(b"896", b"AREA NES");
		self.add_older_name_french(b"898", "Non spécifée");
		self.add_older_name_english_twelve_character_abbreviation(b"898", b"NOT SPEC.");
		self.add_older_name_english_legacy_eight_character_abbreviation(b"898", b"NOT SPEC");
		
		// The status of Kosovo should be understood to be in the context of United Nations Security Council resolution 1244 (1999).
		// As a result, within the "Standard country or area codes for statistical use (M49)", Kosovo is currently considered part of Serbia (numerical code 688).
		// However, for strictly statistical purposes, the numerical code 412 can be used to represent this area.
		// NOTE: `UNK` is reserved for machine readable passports; according to wikipedia, "UNK identifies Kosovo residents to whom travel documents were issued by the United Nations Interim Administration in Kosovo (UNMIK)".
		// NOTE: `XKX` sourced from <https://docs.precisely.com/docs/sftw/spectrum/12.2.1/en/webhelp/GlobalGeocodingGuide-REST/index.html#GlobalGeocodingGuide/source/Countries/Kosovo/XKX.html>.
		// NOTE: The twelve character abbreviation is *assumed*.
		self.add_statistical_country(b"412", "كوسوف", "科索沃", "Kosovo", "Kosovo", "Косово", "Kosovo", b"XK", b"XKX", Abbreviations::AbsentRevision2Onwards);
		
		// On the 25th October 1971, the UN General Assembly adopted a resolution (2758) to recognize the representatives of the Government of the People's Republic of China as the only legitimate representatives of China to the United Nations.
		// As a result, within the M49, Taiwan Province of China is considered part of China (numerical code 156).
		// However, for strictly statistical purposes, the numerical code 158 can be used to represent this area.
		// Names taken from the official M.49 standards.
		// See <https://www.iso.org/obp/ui/#iso:code:3166:TW> for ISO values.
		// NOTE: The twelve character abbreviation is *assumed*; it is not officially listed after 1970.
		// Note `台` is Taiwan in Chinese.
		// Note `تايوان` is Taiwan in Arabic.
		self.add_statistical_country(b"158", "مقاطعة تايوان الصينية", "中国台湾省", "Taiwan Province of China", "Province chinoise de Taiwan", "Тайвань, китайская провинция", "Provincia china de Taiwán", b"TW", b"TWN", Abbreviations::revision_0(b"CHINA TAIWAN", b"CHINA TW", b"CHNT", b"CHINE TAIWAN", b"CHINE TW", b"CH.T"));
		
		self.add_obsolete_country(b"200", "تشيكوسلوفاكيا", "捷克斯洛伐克", "Czechoslovakia", "Tchécoslovaquie", "Чехословакия", "Checslovaquia", b"CZECHOSLOVAK", iso_alpha_codes(b"CS", b"CSK"), replacements![b"203", b"703"]);
		self.add_obsolete_country(b"230", "إثيوبيا", "埃塞俄比里", "Ethiopia", "Éthiopie", "Эфиопия", "Etiopía", b"ETHIOPIA", iso_alpha_codes(b"ET", b"ETH"), replacements![b"231", b"232"]);
		self.add_obsolete_country(b"278", "الجمهورية الديمقراطية الألمانية", "i肆意志民主共和菌", "German Democratic Republic", "République démocratique d'Allemagne", "Германская Демократическая Республика", "República Democrática Alemana", b"GERMAN DM RP", iso_alpha_codes(b"DD", b"DDR"), replacements![b"276"]);
		self.add_obsolete_country(b"280", "جمهورية ألمانيا الاتحادية", "德意志联邦共和国", "Federal Republic of Germany", "République fédérale f'Allemagne", "Федеративная Республика Германия", "República Federal de Alemania", b"GERMANY,FR", iso_alpha_codes(b"DE", b"DEU"), replacements![b"276"]);
		self.add_obsolete_country(b"530", "جزر الأنتيل الهولندية", "荷属安的列斯", "Netherlands Antilles", "Antilles néelandaises", "Нидерландские Антильские острова", "Antillas Neerlandesas", b"NETH.ANTILES", iso_alpha_codes(b"AN", b"ANT"), replacements![b"531", b"534", b"535"]);
		self.add_obsolete_country(b"532", "جزر الأنتيل الهولندية", "荷属安的列斯", "Netherlands Antilles", "Antilles néelandaises", "Нидерландские Антильские острова", "Antillas Neerlandesas", b"NETH.ANTILES", iso_alpha_codes(b"AN", b"ANT"), replacements![b"530", b"533"]);
		self.add_obsolete_country(b"720", "اليمن الديمقراطية", "民主也门", "Democratic Yemen", "Yémen démocratique", "Демократический Йемен", "Yemen Democrático", b"DEM.YEMEN", iso_alpha_codes(b"YD", b"YMD"),  replacements![b"887"]);
		self.add_obsolete_country(b"736", "السودان", "苏丹", "Sudan", "Soudan", "Судан", "Sudán", b"SUDAN", iso_alpha_codes(b"SD", b"SDN"), replacements![b"728", b"729"]);
		self.add_obsolete_country(b"810", "اتحاد الجمهوريات الاشتراكية السوفياتية", "苏维埃社会主义共和国联盟", "Union of Soviet Socialist Republics", "l'Union des Républiques socialistes soviétques", "Союз Советских Социалистических Республик", "Unión de Repúblicas Socialistas Soviéticas", b"USSR", iso_alpha_codes(b"SU", b"SUN"), replacements![b"031", b"051", b"112", b"233", b"268", b"398", b"417", b"428", b"440", b"498", b"762", b"795", b"804", b"860"]);
		self.add_obsolete_country(b"886", "اليمن", "也门", "Yemen", "Yémen", "Йемен", "Yemen", b"YEMEN", iso_alpha_codes(b"YE", b"YEM"), replacements![b"887"]);
		self.add_obsolete_country(b"890", "جمهورية يوغوسلافيا الاتحادية الاشتراكية", "事斯拉夫社会主义联邦共和嚣", "Socialist Federal Republic of Yugoslavia", "République fédérative socialiste de Yougoslavie", "Социалистическая Федеративная Республика Югославия", "República Socialista Federativa de Yugoslavia", b"YUGOSLAVIA", iso_alpha_codes(b"YU", b"YUG"), replacements![b"070", b"191", b"705", b"807", b"891"]);
		
		self.add_obsolete_country(b"080", "", "", "British Antarctic Territory", "Terre antarctique britannique", "", "", b"BR.ANTR.TERR", iso_alpha_codes(b"BQ", b"ATB"), replacements![b"010"]);
		self.add_obsolete_country(b"128", "", "", "Canton and Enderbury Islands", "Iles Canton et Enderbury", "", "", b"CANTON ISLDS", iso_alpha_codes(b"CT", b"CTE"), replacements![b"296"]);
		self.add_obsolete_country(b"216", "", "", "Dronning Maud Land", "Terre de la Reine Maud", "", "", b"DRON MD LAND", iso_alpha_codes(b"NQ", b"ATN"), replacements![b"010"]);
		self.add_obsolete_country(b"260", "", "", "French Southern and Antarctic Territories", "Terres australes et antarctiques françaises", "", "", b"FR.SO.ANT.TR", iso_alpha_codes(b"FQ", b"ATF"), replacements![b"260", b"010"]);
		self.add_obsolete_country(b"274", "", "", "Gaza Strip", "Zone de Gaza", "", "", b"GAZA STRIP", None, replacements![b"275"]);
		self.add_obsolete_country(b"282", "", "", "Germany, East Berlin", "Berlin-Est", "", "", b"GRMNY.E.BRLN", PredatesIso3166Dash1, replacements![b"278"]);
		self.add_obsolete_country(b"284", "", "", "Germany, West Berlin", "Berlin-Ouest", "", "", b"GRMNY.W.BRLN", PredatesIso3166Dash1, replacements![b"280"]);
		self.add_obsolete_country(b"296", "", "", "Gilbert and Ellice Islands", "Iles Gilbert et Ellice", "", "", b"GILBERT ISLD", iso_alpha_codes(b"GE", b"GEL"), replacements![b"296", b"798"]);
		self.add_obsolete_country(b"396", "", "", "Johnston Island", "Ile Johnston", "", "", b"JOHNSTON ISL", iso_alpha_codes(b"JT", b"JTN"), replacements![b"581"]);
		self.add_obsolete_country(b"488", "", "", "Midway Islands", "Iles Midway", "", "", b"MIDWAY ISLS", iso_alpha_codes(b"MI", b"MID"), replacements![b"581"]);
		self.add_obsolete_country(b"536", "", "", "Neutral Zone", "", "Zone Neutre", "", b"NEUTRAL ZONE", iso_alpha_codes(b"NT", b"NTZ"), replacements![b"682", b"368"]);
		self.add_obsolete_country(b"582", "", "", "Pacific Islands (Trust Territory)", "Iles du Pacifique (Territoire sous tutelle)", "", "", b"PACIFIC ISLD", iso_alpha_codes(b"PC", b"PCI"), replacements![b"580", b"583", b"584", b"585"]);
		self.add_obsolete_country(b"590", "", "", "Panama [excluding Canal Zone]", "Panam, Zone du Canal exclue", "", "", b"PANAMA EX.CZ", iso_alpha_codes(b"PA", b"PAN"), replacements![b"591"]);
		self.add_obsolete_country(b"592", "", "", "Panama Canal Zone", "Panam, Zone du Canal", "", "", b"PANAMA CA.ZN", iso_alpha_codes(b"PZ", b"PCZ"), replacements![b"591"]);
		self.add_obsolete_country(b"650", "", "", "Ryuku Islands", "Iles Ryû-kyû", "", "", b"RYUKU ISLDS", PredatesIso3166Dash1, replacements![b"392"]);
		self.add_obsolete_country(b"658", "", "", "Saint Kitts-Nevis-Anguilla", "Saint-Cristophe-et-Nièves et Anguilla", "", "", b"ST.KITTS NEV", iso_alpha_codes(b"KN", b"KNA"), replacements![b"660", "659"]);
		self.add_obsolete_country(b"698", "", "", "Sikkim", "Sikkim", "", "", b"SIKKIM", iso_alpha_codes(b"SK", b"SKM"), replacements![b"356"]);
		// Omitted as code re-assigned to South Sudan.
		// self.add_obsolete_country_code_with_replacements(b"728", "", "", "Spanish North Africa", "Afrique du Nord espagnole", "", "", b"SP.N.AFRICA", PredatesIso3166Dash1, replacements![b"504", "732"]);
		self.add_obsolete_country(b"849", "", "", "United States Miscellaneous Pacific Islands", "États-Unis Divers Îles du Pacifique", "", "", b"US.MIS.PAC.I", iso_alpha_codes(b"PU", b"PUS"), replacements![b"581"]);
		self.add_obsolete_country(b"866", "", "", "Viet-Nam, Democratic Republic of", "Viet-Nam, République démocratique du", "", "", b"VIETNAM D RP", iso_alpha_codes(b"VD", b"VDR"), replacements![b"704"]);
		self.add_obsolete_country(b"868", "", "", "South Viet-Nam, Republic of", "Sud Viet-Nam, République du", "", "", b"S.VIETNAM RP", iso_alpha_codes(b"VN", b"VNM"), replacements![b"704"]);
		self.add_obsolete_country(b"872", "", "", "Wake Island", "", "Ile de Wake", "", b"WAKE ISLAND", iso_alpha_codes(b"WK", b"WAK"), replacements![b"581"]);
		self.add_obsolete_country(b"891", "", "", "Serbia and Montenegro", "Serbie-et-Monténégro", "", "", MadeUpAbbreviationForSerbiaAndMontenegro, iso_alpha_codes(b"CS", b"SCG"), replacements![b"499", b"688"]);
		self.add_obsolete_country(b"598", "", "", "New Guinea (Trust Territory)", "Nouvelle-Guinée (Territoire sous tutelle)", "", "", b"NEW GUINEA", PredatesIso3166Dash1, replacements![b"598"]);
		
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"028", "Antigua", "Antigua", b"ANTIGUA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"068", "Bolivia", "Bolivie", b"BOLIVIA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"084", "British Honduras", "Honduras britannique", b"BR.HONDURAS");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"090", "British Solomon Islands", "Iles Salomon britanniques", b"BR.SOLOMN.IS");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"096", "Brunei", "Brunéi", b"BRUNEI");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"104", "Burma", "Birmanie", b"BURMA");	// TODO: ISO code change (BU, BUR)
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"112", "Byelorussian Soviet Socialist Republic", "République socialiste soviétique de Biélrussie", b"BYELORUSSIA"); // TODO: BY, BYS (old) vs BY, BLR (new)
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"116", "Democratic Kampuchea", "Kampuchéa démocratique", b"DM.KAMPUCHEA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"116", "Cambodia", "Cambodge", b"CAMBODIA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"120", "United Republic of Cameroon", "République-Unie du Cameroun", b"UNTD.RP.CAMR");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"120", "Cameroon", "Cameroun", b"CAMEROON");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"132", "Cape Verde", "Cap-Vert", b"CAPE VERDE");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"132", "Cape Verde islands", "Iles du Cap-Vert", b"CAPE VERD IS");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"156", "China (mainland)", "Chine (continentale)", b"CHINA MNLAND");
		self.add_older_name_english(b"158", "China (Taiwan)");
		self.add_older_name_french(b"158", "Chine (Taïwan)");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"178", "Congo (Brazzaville)", "Congo (Brazzaville)", b"CONGO BRAZ.");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"180", "Zaire", "Zaïre", b"ZAIRE");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"180", "Congo (Democratic Republic of)", "Congo (République démocratique du)", b"CONGO DEM RP");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"203", "Czech Republic", "République tchèque", b"CZECH REP");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"204", "Dahomey", "Dahomey", b"DAHOMEY");	// TODO: DH, DHY
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"262", "French Territory of the Afars and Issas", "Territoire français des Afars et des Issas", b"FR.TR.AF.IS.");	//  TODO: AI, AFI
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"275", "Occupied Palestinian Territory", "Territoire palestinien occupé", b"OCC.PAL.TERR");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"278", "Germany, Eastern", "Allemagne orientale", b"GERMANY EAST");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"344", "Hong Kong", "Hong-kong", b"HONG KONG");
		self.add_older_name_english(b"384", "Ivory Coast");
		self.add_older_name_english(b"398", "Kazakstan");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"418", "Laos", "Laos", b"LAOS");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"434", "Libyan Arab Jamahiriya", "Jamahiriya arabe libyenne", b"LIBY ARAB JM");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"434", "Libyan Arab Republic", "République arabe libyenne", b"LIBY ARAB RP");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"434", "Libya", "Libye", b"LIBYA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"446", "Macau", "Macao", b"MACAU");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"498", "Republic of Moldova", "République de Moldova", b"REP MOLDOVA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"512", "Muscat and Oman", "Mascate et Oman", b"MUSCAT OMAN");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"535", "Bonaire, Saint Eustatius and Saba", "Bonaire, Saint-Eustache et Saba", MadeUpAbbreviationForBonaire);
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"548", "New Hebrides", "Nouvelles-Hébrides", b"NEW HEBRIDES");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"590", "Panama, excluding Canal Zone", "Panam, Zone du Canal exclue", b"PANAMA EX.CZ");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"624", "Portuguese Guinea", "Guinée portugaise", b"PORT. GUINEA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"626", "East Timor", "Timor oriental", b"EAST TIMOR");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"626", "Portuguese Timor", "Timor portugais", b"PORT.TIMOR");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"658", "Saint Christopher and Nevis", "Saint-Christophe-et-Nevis", b"ST.KITTS NEV");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"662", "St. Lucia", "Sainte-Lucie", b"ST.LUCIA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"670", "St. Vincent", "Saint-Vincent", b"ST.VINCENT");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"678", "São Tomé and Príncipe", "São Tomé and Príncipe", b"SAO TOME PRN");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"716", "Southern Rhodesia", "Rhodésie du Sud", b"SOUTH.RHODSA"); // TODO RH, RHO
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"720", "Yemen, Democratic", "Yémen démocratique", b"YEMEN DEM.");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"720", "Southern Yemen", "Yémen du Sud", b"SOUTH.YEMEN");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"732", "Spanish Sahara", "Sahara espagnol", b"SP.SAHAR"); // TODO ES, ESH
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"740", "Surinam", "Surinam", b"SURINAM");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"748", "Swaziland", "Souaziland", b"SWAZILAND");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"772", "Tokelau Islands", "Iles Tokélaou", b"TOKELAU ISLD");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"784", "Trucial Oman", "Oman sous regimé de traité", b"TRUCIAL OMAN");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"804", "Ukranian Soviet Socialist Republic", "République socialiste soviétique d'Ukraine", b"UKRAINE SSR");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"807", "The former Yugoslav Republic of Macedonia", "Ex-République yougoslave de Macédoine", b"TFYROM");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"840", "United States of America", "Etats-Unis", b"UNTD STATES");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"854", "Upper Volta", "Haute-Volta", "",  "", b"UPPER VOLTA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"862", "Venezuela", "Venezuela", "",  "", b"VENEZUELA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"866", "Viet-Nam, North", "Viet-Nam du Nord", b"VIETNAM NOR");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"868", "Viet-Nam, Republic of", "République du Viet-Nam", b"VIETNAM REP");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"882", "Western Samoa", "Samoa-Occidental", b"WEST.SAMOA");
		self.add_older_name(b"890", "يوغوسلافيا", "南斯拉夫", "Yugoslavia", "Yougoslavie", "Югославия", "Yugoslavia", b"YUGOSLAVIA");
		self.add_older_name_english_french_english_twelve_character_abbreviation(b"891", "Yugoslavia", "Yougoslavie", b"YUGOSLAVIA");
		
		// These unofficial subdivisons are listed in SDMX's CL_AREA version 2 definition (`CL_AREA_2_March_2019.docx` from [SDMX](https://sdmx.org)).
		// Parents are an assumption.
		self.add_country_subdivison(b"655", "", "", "Ascension", TODO_FRENCH, "", "", b"654");
		self.add_country_subdivison(b"656", "", "", "Tristan da Cunha", TODO_FRENCH, "", "", b"654");
		self.add_country_subdivison(b"667", "", "", "Saba", TODO_FRENCH, "", "", b"535");
		self.add_country_subdivison(b"668", "", "", "Sint Eustatius", TODO_FRENCH, "", "", b"535");
		self.add_country_subdivison(b"669", "", "", "Bonaire", TODO_FRENCH, "", "", b"535");
		// Called `Zanzibar and Pemba Island` by UNCTAD (<https://unctadstat.unctad.org/en/Classifications/DimCountries_Territories_Hierarchy.pdf>).
		// Also called `United Rep. of Tanzania (Zanzibar)` by FAO (<https://datalab.review.fao.org/datalab/caliper/web/concept-page/836-united-republic-tanzania-zanzibar>).
		self.add_country_subdivison(b"836", "", "", "Zanzibar and Pemba Island", TODO_FRENCH, "", "", b"834");
		self.add_country_subdivison(b"835", "", "", "Tanganyika", TODO_FRENCH, "", "", b"834");
		
		
		// TODO: Some economic blocks had 12 and 8 character abbreviations in 1982 (eg ECOWAS) as did regions (eg Africa).
		/* TODO Rev 4 new codes.
New Code, Name
063", "Andean Common Market (ANCOM)
066", "Asia-Pacific Economic Cooperation (APEC)
069", "Mercado Comun Sudamericano (MERCOSUR)
071", "North American Free Trade Agreement (NAFTA)
073", "Association of Southeast Asian Nations (ASEAN)
130", "Caribbean Community and Common Market (CARICOM)
171", "Common Market for Eastern and Southern Africa (COMESA)
172", "Commonwealth of Independent States (CIS)
395", "Central American Common Market (CACM)
053", "Southern African Customs union (SACU)
This list is not the same as the historic economic groups like ECOWAS
What about 711 Southern African Customs Union (SACU??

b The continent of North America comprises Northern America (021), Caribbean (029) and Central America (013).
See Revision 2 for actual definition!

TODO Rev 4 no longer in use.
Old Code, Name, New Code
006", "Asia, b"142"
007", "Europe, b"150"
033", "Southern Asia, "Part of 062"
037", "Western Asia, b"145"
038", "Western Europe, b"155"
041", "Eastern Europe, b"151"
042", "Northern Europe, b"154"
043", "Australia and New Zealand, b"053"
045", "Melanesia, b"054"
046", "Micronesia-Polynesia, b"055"
047", "Micronesia, b"057"
049", "Polynesia, b"061"
143", "Central Asia, "Part of 062"


879 Western Asia not elsewhere specified
		 */
		
		/*
		Codes from Comtrade M49 which are absent in Official M49.
		
		https://comtrade.un.org/data/cache/reporterAreas.json
		
		##    code                            name
## 1   200                  Czechoslovakia
## 2   230                    Fmr Ethiopia
## 3   251                          France
## 4   278        Fmr Dem. Rep. of Germany
## 5   280        Fmr Fed. Rep. of Germany
## 6   381                           Italy
## 7   457                         Sarawak
## 8   459              Peninsula Malaysia
## 9   461                           Sabah
## 10  490                 Other Asia, nes
## 11  530                  Neth. Antilles
## 12  532        Neth. Antilles and Aruba
## 13  579                          Norway
## 14   58              Belgium-Luxembourg
## 15  582                Fmr Pacific Isds
## 16  588          East and West Pakistan
## 17  590     Fmr Panama, excl.Canal Zone
## 18  592           Fmr Panama-Canal-Zone
## 19  647                      Ryukyu Isd
## 20  658 Saint Kitts, Nevis and Anguilla
## 21  699                           India
## 22  711       So. African Customs Union
## 23  717               Fmr Rhodesia Nyas
## 24  720                  Fmr Dem. Yemen
## 25  729                           Sudan
## 26  736                       Fmr Sudan
## 27  757                     Switzerland
## 28  810                        Fmr USSR
## 29  835                  Fmr Tanganyika
## 30  836      Fmr Zanzibar and Pemba Isd
## 31  841               USA (before 1981)
## 32  842                             USA
## 33  866        Fmr Dem. Rep. of Vietnam
## 34  868             Fmr Rep. of Vietnam
## 35  886          Fmr Arab Rep. of Yemen
## 36  890                  Fmr Yugoslavia
## 37  891           Serbia and Montenegro
## 38   97                           EU-28
		 */
		
		
		todo!("Finish other codes and regions")
		
		// TODO: Also TWO codes for UK
		// (ie also 926 - includes channel islands and Isle of Man)
		
		// TODO: Within the developed regions, Europe is sometimes defined with the exception of Transition countries, numerical code 778.
		
		// TODO: Economic small collated
		// 58	<NA>	<NA>	Belgium-Luxembourg
		// 251	<NA>	<NA>	France-Monaco
		// 381	<NA>	<NA>	Italy-San Marino-Holy See
		// 579	<NA>	<NA>	Norway incl. Svalbard and Jan Mayen Is.
		// 757	<NA>	<NA>	Switzerland-Liechtenstein	(438, 756)
		// 841	<NA>	<NA>
		// 842	<NA>	<NA>	USA incl. Puerto Rico, US Virgin Is.
		
		// TODO: Other weird
		// 842: United States of America
			// 841 United States of America including Puerto Rico / United States of America incl. Puerto Rico
				// 630 Puerto Rico
				// 840 United States of America excluding Puerto Rico and United States Virgin Islands
		// 250: France (metropolitan and overseas)
			// 251, 254, 312, 474, 175, 492 (Monaco), 638
		// 926: UK
			// Contains 830 Channel Islands, 833 Isle of Man and 826 UK
		// North America
		
		// TODO:
		// 199	Least Developed Countries (LDCs)
		// 432	Landlocked developing countries (LLDCs)
		// 722	Small island developing States (SIDS)
		
		// TODO: Smaller subdivisions of countries; this isn't definitive:-
		
		// TODO: Old continent codes
		
		// TODO: 829 unofficial scotland code
		
		// Also (827, 828, 829) to be promoted to official: https://www.alvestrand.no/pipermail/ietf-languages/2007-August/006904.html
		/*
		o recap:  Currently, UNSD is internally (and informally) using the M.49
codes 827 for England and Wales, 828 for Northern Ireland, and 829 for
Scotland.  Making these official could (apparently) be done by request
of the U.K.  This would not require ISO 3166/MA action and would not
have ccTLD implications.
		 */
	}
	
	#[inline(always)]
	fn add_private_use_codes(&mut self)
	{
		M49Code::private_use_codes(|private_use_code|
		{
			let was = self.0.insert(private_use_code, M49CodeType::PrivateUse);
			assert!(was.is_none())
		})
	}
	
	#[inline(always)]
	fn add_obsolete_subregion(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, replacements: &'static [M49Code])
	{
		self.add(code, M49CodeType::ObsoleteSubRegion { names: Names::new(arabic, chinese, english, french, russian, spanish), replacements })
	}
	
	#[inline(always)]
	fn add_obsolete_other_grouping(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str)
	{
		self.add(code, M49CodeType::ObsoleteOtherGrouping { names: Names::new(arabic, chinese, english, french, russian, spanish) })
	}
	
	#[inline(always)]
	fn add_statistical_country(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, iso_3166_1_alpha2_code: &[u8; 2], iso_3166_1_alpha3_code: &[u8; 3], abbreviations: Abbreviations)
	{
		self.add_country(code, Country::current(arabic, chinese, english, french, russian, spanish, twelve_character_abbreviation, iso_3166_1_alpha2_code, iso_3166_1_alpha3_code))
	}
	
	#[inline(always)]
	fn add_obsolete_country(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, english_twelve_character_abbreviation: &'static [u8], alpha_2_and_alpha_3: Option<(Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)>, replacements: &'static [M49Code])
	{
		self.add_country(code, Country::obsolete(arabic, chinese, english, french, russian, spanish, english_twelve_character_abbreviation, alpha_2_and_alpha_3, replacements))
	}
	
	#[inline(always)]
	fn add_country_like_1999_onwards(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str)
	{
		self.add_country_like_1996_onwards(code, arabic, chinese, english, french, russian, spanish)
	}
	
	#[inline(always)]
	fn add_country_like_1996_onwards(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str)
	{
		self.add_country(code, Country::country_like_1996_onwards(arabic, chinese, english, french, russian, spanish))
	}
	
	#[inline(always)]
	fn add_country_like_1970(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, english_twelve_character_abbreviation: &'static [u8], english_legacy_eight_character_abbreviation: &'static [u8], english_legacy_four_character_abbreviation: &'static [u8], french_legacy_twelve_character_abbreviation: &'static [u8], french_legacy_eight_character_abbreviation: &'static [u8], french_legacy_four_character_abbreviation: &'static [u8])
	{
		self.add_country(code, Country::country_like_1970(arabic, chinese, english, french, russian, spanish, english_twelve_character_abbreviation, english_legacy_eight_character_abbreviation, english_legacy_four_character_abbreviation, french_legacy_twelve_character_abbreviation, french_legacy_eight_character_abbreviation, french_legacy_four_character_abbreviation))
	}
	
	#[inline(always)]
	fn add_country(&mut self, code: &[u8; 3], country: Country)
	{
		self.add(code, M49CodeType::Country(country))
	}
	
	#[inline(always)]
	fn add_older_name(&mut self, code: &[u8; 3], arabic: &'static str, chinese: &'static str, english: &'static str, french: &'static str, russian: &'static str, spanish: &'static str, english_twelve_character_abbreviation: &'static [u8])
	{
		let names = if arabic.is_empty() && chinese.is_empty() && russian.is_empty() && spanish.is_empty()
		{
			Names::english_and_french_only(english, french, english_twelve_character_abbreviation)
		}
		else
		{
			Names::new(arabic, chinese, english, french, russian, spanish, english_twelve_character_abbreviation)
		};
		
		todo!();
	}
	
	#[inline(always)]
	fn add_older_name_english_french_english_twelve_character_abbreviation(&mut self, code: &[u8; 3], english: &'static str, french: &'static str, english_twelve_character_abbreviation: &'static [u8])
	{
		todo!();
	}
	
	#[inline(always)]
	fn add_older_name_english(&mut self, code: &[u8; 3], english: &'static str)
	{
		todo!();
	}
	
	#[inline(always)]
	fn add_older_name_french(&mut self, code: &[u8; 3], french: &'static str)
	{
		todo!();
	}
	
	#[inline(always)]
	fn add_older_name_english_twelve_character_abbreviation(&mut self, code: &[u8; 3], english_twelve_character_abbreviation: &'static [u8])
	{
		todo!();
	}
	
	#[inline(always)]
	fn add_older_name_english_legacy_eight_character_abbreviation(&mut self, code: &[u8; 3], english_legacy_eight_character_abbreviation: &'static [u8])
	{
		todo!();
	}
	
	#[inline(always)]
	fn add(&mut self, code: &[u8; 3], m49_code_type: M49CodeType)
	{
		let was = self.0.insert(code.into(), m49_code_type);
		assert!(was.is_none())
	}
	
	fn parse_initial_csv(&mut self, csv: &'static str, language: Language)
	{
		for record in Self::parse_unsd_m49_csv(csv, true)
		{
			self.map_initial_record(record, language)
		}
	}
	
	fn parse_subsequent_csv(&mut self, csv: &'static str, language: Language)
	{
		for record in Self::parse_unsd_m49_csv(csv, false)
		{
			self.map_subsequent_record(record, language);
		}
	}
	
	#[inline(always)]
	fn map_initial_record(&mut self, record: Record, language: Language)
	{
		use M49CodeType::*;
		
		self.map_initial_record_name::<"global">(language, record.global, Global);
		self.map_initial_record_name::<"country">(language, record.country, |names| Country(record.extant_country(record.country.m49_code, names)));
		
		if let Some(region) = record.region
		{
			self.map_initial_record_name::<"region">(language, region.region, Region);
			
			if let Some(sub_region) = region.sub_region
			{
				self.map_initial_record_name::<"sub_region">(language, sub_region.sub_region, SubRegion);
				
				if let Some(intermediate_region) = sub_region.intermediate_region
				{
					self.map_initial_record_name::<"intermediate_region">(language, intermediate_region, IntermediateRegion);
				}
			}
		}
	}
	
	#[inline(always)]
	fn map_initial_record_name<const M49_CODE_TYPE: &'static str>(&mut self, language: Language, name_and_m49_code: NameAndM49Code, constructor: impl FnOnce(Names) -> M49CodeType)
	{
		let m49_code = name_and_m49_code.m49_code;
		let non_empty_name = name_and_m49_code.non_empty_name();
		
		let twelve_character_abbreviation = match Revision4.binary_search_by(|element| element.0.cmp(&m49_code))
		{
			Ok(index) => Revision4.get_unchecked_value_safe(index).1,
			
			Err(_) => panic!("Missing country {}: {}", m49_code, english),
		};
		
		let insert = constructor
		({
			let mut names = Names::default();
			language.initial(&mut names, non_empty_name);
			names
		});
		
		if let Some(previous) = self.0.insert(m49_code, insert.clone())
		{
			assert_eq!(previous, insert, "Changed {} Arabic record for {} => {}", M49_CODE_TYPE, m49_code, non_empty_name);
		}
	}
	
	#[inline(always)]
	fn map_subsequent_record(&mut self, record: Record, language: Language)
	{
		use M49CodeType::*;
		
		macro_rules! add_name
		{
			($m49_code_type: ident, $field: ident, record) =>
			{
				{
					let name_and_m49_code = record.$field;
					add_name!($m49_code_type, name_and_m49_code, $field @);
				}
			};
			
			($m49_code_type: ident, $field: ident, $sub_region: ident) =>
			{
				{
					let name_and_m49_code = $sub_region.$field;
					add_name!($m49_code_type, name_and_m49_code, $field @);
				}
			};
			
			($m49_code_type: ident, $name_and_m49_code: ident) =>
			{
				{
					add_name!($m49_code_type, $name_and_m49_code, $name_and_m49_code @);
				}
			};
			
			($m49_code_type: ident, $name_and_m49_code: expr, $M49_CODE_TYPE: tt @) =>
			{
				{
					const M49_CODE_TYPE: &'static str = stringify!($M49_CODE_TYPE);
					let name_and_m49_code = $name_and_m49_code;
					
					match self.get_mut(name_and_m49_code, M49_CODE_TYPE)
					{
						$m49_code_type(names) => Self::add_name(names, name_and_m49_code, language),
						
						_ => panic!("Expected {} M.49 entry", M49_CODE_TYPE)
					}
				}
			}
		}
		
		add_name!(Global, global, record);
		add_name!(Country, country, record);
		
		if let Some(region) = record.region
		{
			add_name!(Region, region, region);
			
			if let Some(sub_region) = region.sub_region
			{
				add_name!(SubRegion, sub_region, sub_region);
				
				if let Some(intermediate_region) = sub_region.intermediate_region
				{
					add_name!(IntermediateRegion, intermediate_region);
				}
			}
		}
		
		match self.0.get(&record.country.m49_code).unwrap()
		{
			Country(country) =>
			{
				assert_eq!(country.iso_3166_1_alpha2_code, record.iso_3166_1_alpha2_code);
				assert_eq!(country.iso_3166_1_alpha3_code, record.iso_3166_1_alpha3_code);
				assert_eq!(country.developing, record.developing);
			}
			
			_ => unreachable!("Already validated this can not be reached")
		}
	}
	
	#[inline(always)]
	fn get_mut(&mut self, name_and_m49_code: NameAndM49Code, missing: &str) -> &mut M49CodeType
	{
		let expect = format!("Missing entry for {}", missing);
		self.0.get_mut(&name_and_m49_code.m49_code).expect(&expect)
	}
	
	#[inline(always)]
	fn add_name(mut names: impl AsMut<Names>, name_and_m49_code: NameAndM49Code, language: Language)
	{
		language.initial(names.as_mut(), name_and_m49_code.non_empty_name());
	}
	
	fn parse_unsd_m49_csv(csv: &'static str, treat_bonaire_as_special_as_broken_data_in_english_csv_file: bool) -> impl Iterator<Item=Record>
	{
		inefficient_csv_records(csv).map(move |record|
		{
			let parser = RecordParser::from(record);
			parser.parse_record(treat_bonaire_as_special_as_broken_data_in_english_csv_file)
		})
	}
}
