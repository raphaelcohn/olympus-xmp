// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in sort order.
const ExtantCountriesAbsentFromCsvForPoliticalReasons: [(M49Code, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code); 2] =
{
	#[inline(always)]
	const fn add_statistical_country(m49_code: &'static [u8; 3], arabic_name: &'static str, chinese_name: &'static str, english_name: &'static str, french_name: &'static str, russian_name: &'static str, spanish_name: &'static str, iso_3166_1_alpha2_code: &[u8; 2], iso_3166_1_alpha3_code: &[u8; 3]) -> (M49Code, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)
	{
		(M49Code::from(m49_code), arabic_name, chinese_name, english_name, french_name, russian_name, spanish_name, Iso3166Dash1Alpha2Code::from(iso_3166_1_alpha2_code), Iso3166Dash1Alpha3Code::from(iso_3166_1_alpha3_code))
	}
	
	[
		// On the 25th October 1971, the UN General Assembly adopted a resolution (2758) to recognize the representatives of the Government of the People's Republic of China as the only legitimate representatives of China to the United Nations.
		// As a result, within the M49, Taiwan Province of China is considered part of China (numerical code 156).
		// However, for strictly statistical purposes, the numerical code 158 can be used to represent this area.
		// Names taken from the official M.49 standards.
		// See <https://www.iso.org/obp/ui/#iso:code:3166:TW> for ISO values.
		// NOTE: The twelve character abbreviation is *assumed*; it is not officially listed after 1970.
		// Note `台` is Taiwan in Chinese.
		// Note `تايوان` is Taiwan in Arabic.
		add_statistical_country(b"158", "مقاطعة تايوان الصينية", "中国台湾省", "Taiwan Province of China", "Province chinoise de Taiwan", "Тайвань, китайская провинция", "Provincia china de Taiwán", b"TW", b"TWN"),
		
		// The status of Kosovo should be understood to be in the context of United Nations Security Council resolution 1244 (1999).
		// As a result, within the "Standard country or area codes for statistical use (M49)", Kosovo is currently considered part of Serbia (numerical code 688).
		// However, for strictly statistical purposes, the numerical code 412 can be used to represent this area.
		// NOTE: `UNK` is reserved for machine readable passports; according to wikipedia, "UNK identifies Kosovo residents to whom travel documents were issued by the United Nations Interim Administration in Kosovo (UNMIK)".
		// NOTE: `XKX` sourced from <https://docs.precisely.com/docs/sftw/spectrum/12.2.1/en/webhelp/GlobalGeocodingGuide-REST/index.html#GlobalGeocodingGuide/source/Countries/Kosovo/XKX.html>.
		// NOTE: The twelve character abbreviation is *assumed*.
		add_statistical_country(b"412", "كوسوف", "科索沃", "Kosovo", "Kosovo", "Косово", "Kosovo", b"XK", b"XKX"),
	]
};
