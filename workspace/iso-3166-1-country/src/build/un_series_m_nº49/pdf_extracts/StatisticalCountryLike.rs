// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Those originally defined in 1970 will have abbreviations, too.
const StatisticalCountryLike: [(M49Code, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str); 7] =
{
	#[inline(always)]
	const fn statistical_country_like(code: &[u8; 3], arabic_name: &'static str, chinese_name: &'static str, english_name: &'static str, french_name: &'static str, russian_name: &'static str, spanish_name: &'static str) -> (M49Code, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str)
	{
		(M49Code::from(code), arabic_name, chinese_name, english_name, french_name, russian_name, spanish_name)
	}
	
	[
		// Defined in 1970; has abbreviations.
		// Translations from the English name have been done for Arabic, Chinese, Russian and Spanish (but not French, as the 1970 standard is bilingual) using Google Translate as no source exists.
		statistical_country_like(b"000", "مجموع", "总", "Total", "Total", "Итог", "Total"),
		
		// From Revision 3 onwards.
		statistical_country_like(b"837", "المستودعات، مخازن السفن", "煤腊、船用补给品", "Bunkers, ship stores", "Combustible de soute et provisions de bord", "Бункеры, судовые запасы", "Tanques de depósito, almacenes de a bordo"),
		
		// From Revision 3 onwards.
		statistical_country_like(b"838", "المناطق الحرة", "费由区", "Free zones", "Zones franches", "Свободные зоны", "Zonas francas"),
		
		// Unofficial from United Nations Conference on Trade and Development (UNCTAD) classification, other territories (<https://unctadstat.unctad.org/EN/Classifications/DimCountries_OtherTerritoriesList_Classification.pdf>, 09 June 2021).
		// Translations from the English name have been done for Arabic, Chinese, French, Russian and Spanish using Google Translate as no source exists.
		statistical_country_like(b"839", "المعلومات السرية والاختلافات", "機密資訊和差異", "Confidential information and differences", "Informations confidentielles et différences", "Конфиденциальная информация и различия", "Información confidencial y diferencias"),
		
		// Defined in 1970; has abbreviations.
		// Has historic changes of name and abbreviation.
		statistical_country_like(b"896", "مناطق غير محددة في أماكن أخرى", "来另列地区", "Areas not elsewhere specified", "Zones non spécifiées ailleurs", "Районы, не указанные в других местах", "Zonas no especificadas en otra parte"),
		
		// Defined in 1970; has abbreviations.
		// Has historic changes of name and abbreviation.
		statistical_country_like(b"898", "مناطق غير محددة", "不详地区", "Areas not specified", "Zones non spécifiées", "Неуказанные районы", "Zonas no especificadas"),
		
		// From Revision 3 onwards.
		statistical_country_like(b"899", "مناطق غير محددة في أماكن أخرى وغير معروفة", "来另费埠医和不明地区", "Areas not elsewhere specified and unknown", "Zones non spécifiées ailleurs et inconnues", "Районы, не указанные в других местах и неизвестные", "Zonas no especificadas en otra parte y desconocidas"),
	]
};
