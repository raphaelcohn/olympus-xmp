// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// The ISO 3166-1 alpha codes are only included if they were never defined in Revision 2 of Series M Nº49.
/// As far as is known, this only applies to Sikkim.
///
/// Spanish North Africa is included, even though its code was reassigned to South Sudan.
const FormerCountries: [(M49Code, OptionalIso3166Dash1AlphaCodes, StaticConstituents); 33] =
{
	type StaticLastKnownArabicChineseRussianSpanishNames = Option<(StaticChineseName, StaticChineseName, StaticRussianName, StaticSpanishName)>;
	
	type StaticOptionalIso3166Dash1AlphaCodes = Option<(StaticIso3166Dash1Alpha2Code, StaticIso3166Dash1Alpha3Code)>;
	
	const AssignedInRevision1Or2: StaticOptionalIso3166Dash1AlphaCodes = None;
	const NeverAssigned: StaticOptionalIso3166Dash1AlphaCodes = None;
	const FormerBeforeIso3166Dash1: StaticOptionalIso3166Dash1AlphaCodes = None;
	
	const ReassignedToSouthSudan: StaticConstituents = &[];
	
	#[inline(always)]
	const fn last_known_arabic_chinese_russian_spanish_names(last_known_arabic_name: StaticArabicName, last_known_chinese_name: StaticChineseName, last_known_russian_name: StaticRussianName, last_known_spanish_name: StaticSpanishName) -> StaticLastKnownArabicChineseRussianSpanishNames
	{
		Some((last_known_arabic_name, last_known_chinese_name, last_known_russian_name, last_known_spanish_name))
	}
	
	#[inline(always)]
	const fn former_country(m49_code: StaticM49Code, _last_known_english_name: StaticEnglishName, _last_known_french_name: StaticFrenchName, iso_3166_dash_1_alpha_codes: StaticOptionalIso3166Dash1AlphaCodes, replacements: StaticConstituents, _last_known_arabic_chinese_russian_spanish_names: StaticLastKnownArabicChineseRussianSpanishNames) -> (M49Code, OptionalIso3166Dash1AlphaCodes, StaticConstituents)
	{
		let length = replacements.len();
		if length != 0
		{
			let mut index = 0;
			while index != length
			{
				if replacements[index].0 == *m49_code
				{
					panic!("Former country in replacements")
				}
				
				index += 1;
			}
		}
		
		(M49Code::from(m49_code), iso_3166_dash_1_alpha_codes.map(const_iso_3166_dash_1_alpha_codes_from), replacements)
	}
	
	[
		former_country(b"080", "British Antarctic Territory", "Terre antarctique britannique", AssignedInRevision1Or2, constituents![b"010"], None),
		former_country(b"128", "Canton and Enderbury Islands", "Iles Canton et Enderbury", AssignedInRevision1Or2, constituents![b"296"], None),
		former_country(b"200", "Czechoslovakia", "Tchécoslovaquie", AssignedInRevision1Or2, constituents![b"203", b"703"], last_known_arabic_chinese_russian_spanish_names("تشيكوسلوفاكيا", "捷克斯洛伐克", "Чехословакия", "Checslovaquia")),
		former_country(b"216", "Dronning Maud Land", "Terre de la Reine Maud", AssignedInRevision1Or2, constituents![b"010"], None),
		former_country(b"230", "Ethiopia", "Éthiopie", AssignedInRevision1Or2, constituents![b"231", b"232"], last_known_arabic_chinese_russian_spanish_names("إثيوبيا", "埃塞俄比里", "Эфиопия", "Etiopía")),
		former_country(b"260", "French Southern and Antarctic Territories", "Terres australes et antarctiques françaises", AssignedInRevision1Or2, constituents![b"010", b"260"], None),
		former_country(b"274", "Gaza Strip", "Zone de Gaza", NeverAssigned, constituents![b"275"], None),
		former_country(b"278", "German Democratic Republic", "République démocratique d'Allemagne", AssignedInRevision1Or2, constituents![b"276"], last_known_arabic_chinese_russian_spanish_names("الجمهورية الديمقراطية الألمانية", "i肆意志民主共和菌", "Германская Демократическая Республика", "República Democrática Alemana")),
		former_country(b"280", "Federal Republic of Germany", "République fédérale d'Allemagne", AssignedInRevision1Or2, constituents![b"276"], last_known_arabic_chinese_russian_spanish_names("جمهورية ألمانيا الاتحادية", "德意志联邦共和国", "Федеративная Республика Германия", "República Federal de Alemania")),
		former_country(b"282", "Germany, East Berlin", "Berlin-Est", FormerBeforeIso3166Dash1, constituents![b"278"], None),
		former_country(b"284", "Germany, West Berlin", "Berlin-Ouest", FormerBeforeIso3166Dash1, constituents![b"280"], None),
		// Tuvalu, 798 split off; rump became Kiribati and ISO 3166-1 alpha codes changed.
		// obsolete_country(b"296", "Gilbert and Ellice Islands", "Iles Gilbert et Ellice", AssignedInRevision1Or2, constituents![b"296", b"798"], None),
		former_country(b"396", "Johnston Island", "Ile Johnston", AssignedInRevision1Or2, constituents![b"581"], None),
		former_country(b"488", "Midway Islands", "Iles Midway", AssignedInRevision1Or2, constituents![b"581"], None),
		former_country(b"530", "Netherlands Antilles", "Antilles néelandaises", AssignedInRevision1Or2, constituents![b"531", b"534", b"535"], last_known_arabic_chinese_russian_spanish_names("جزر الأنتيل الهولندية", "荷属安的列斯", "Нидерландские Антильские острова", "Antillas Neerlandesas")),
		former_country(b"532", "Netherlands Antilles", "Antilles néelandaises", AssignedInRevision1Or2, constituents![b"530", b"533"], last_known_arabic_chinese_russian_spanish_names("جزر الأنتيل الهولندية", "荷属安的列斯", "Нидерландские Антильские острова", "Antillas Neerlandesas")),
		former_country(b"536", "Neutral Zone", "Zone Neutre", AssignedInRevision1Or2, constituents![b"368", b"682"], None),
		former_country(b"582", "Pacific Islands (Trust Territory)", "Iles du Pacifique (Territoire sous tutelle)", AssignedInRevision1Or2, constituents![b"580", b"583", b"584", b"585"], None),
		former_country(b"590", "Panama [excluding Canal Zone]", "Panam, Zone du Canal exclue", AssignedInRevision1Or2, constituents![b"591"], None),
		former_country(b"592", "Panama Canal Zone", "Panam, Zone du Canal", AssignedInRevision1Or2, constituents![b"591"], None),
		former_country(b"598", "New Guinea (Trust Territory)", "Nouvelle-Guinée (Territoire sous tutelle)", FormerBeforeIso3166Dash1, constituents![b"598"], None),
		former_country(b"650", "Ryuku Islands", "Iles Ryû-kyû", FormerBeforeIso3166Dash1, constituents![b"392"], None),
		former_country(b"658", "Saint Kitts-Nevis-Anguilla", "Saint-Cristophe-et-Nièves et Anguilla", AssignedInRevision1Or2, constituents![b"659", b"660"], None),
		former_country(b"698", "Sikkim", "Sikkim", Some((b"SK", b"SKM")), constituents![b"356"], None),
		former_country(b"720", "Democratic Yemen", "Yémen démocratique", AssignedInRevision1Or2,  constituents![b"887"], last_known_arabic_chinese_russian_spanish_names("اليمن الديمقراطية", "民主也门", "Демократический Йемен", "Yemen Democrático")),
		// Known only from UN Comtrade (<https://comtrade.un.org/data/cache/reporterAreas.json>), believed to be the Federation of Rhodesia and Nyasaland which became (eventually) Zimbabwe, Zambia and Malawi.
		// former_country(b"717", "Fmr Rhodesia Nyas", ???, FormerBeforeIso3166Dash1, constituents![b"454", b"716", b"894"], None),
		former_country(b"728", "Spanish North Africa", "Afrique du Nord espagnole", FormerBeforeIso3166Dash1, ReassignedToSouthSudan, None),
		former_country(b"736", "Sudan", "Soudan", AssignedInRevision1Or2, constituents![b"728", b"729"], last_known_arabic_chinese_russian_spanish_names("السودان", "苏丹", "Судан", "Sudán")),
		former_country(b"810", "Union of Soviet Socialist Republics", "l'Union des Républiques socialistes soviétques", AssignedInRevision1Or2, constituents![b"031", b"051", b"112", b"233", b"268", b"398", b"417", b"428", b"440", b"498", b"762", b"795", b"804", b"860"], last_known_arabic_chinese_russian_spanish_names("اتحاد الجمهوريات الاشتراكية السوفياتية", "苏维埃社会主义共和国联盟", "Союз Советских Социалистических Республик", "Unión de Repúblicas Socialistas Soviéticas")),
		former_country(b"849", "United States Miscellaneous Pacific Islands", "États-Unis Divers Îles du Pacifique", AssignedInRevision1Or2, constituents![b"581"], None),
		former_country(b"866", "Viet-Nam, Democratic Republic of", "Viet-Nam, République démocratique du", AssignedInRevision1Or2, constituents![b"704"], None),
		former_country(b"868", "South Viet-Nam, Republic of", "Sud Viet-Nam, République du", AssignedInRevision1Or2, constituents![b"704"], None),
		former_country(b"872", "Wake Island", "Ile de Wake", AssignedInRevision1Or2, constituents![b"581"], None),
		former_country(b"886", "Yemen", "Yémen", AssignedInRevision1Or2, constituents![b"887"], last_known_arabic_chinese_russian_spanish_names("اليمن", "也门", "Йемен", "Yemen")),
		former_country(b"890", "Socialist Federal Republic of Yugoslavia", "République fédérative socialiste de Yougoslavie", AssignedInRevision1Or2, constituents![b"070", b"191", b"705", b"807", b"891"], last_known_arabic_chinese_russian_spanish_names("جمهورية يوغوسلافيا الاتحادية الاشتراكية", "事斯拉夫社会主义联邦共和嚣", "Социалистическая Федеративная Республика Югославия", "República Socialista Federativa de Yugoslavia")),
	]
};
