// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Extracted from the PDF for UNSD Series M, Nº49 Revision 2 (1982).
///
/// Non-English names were not officially published.
/// There are no French abbreviations.
/// There are no 4 character abbreviations.
pub(super) const NamesAndAbbreviationsForCountriesRevision2: [(M49Code, &'static str, (LegacyEightCharacterAbbreviation, TwelveCharacterAbbreviation), Option<(Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)>); 217] =
{
	#[inline(always)]
	const fn names_and_abbreviations_for_country_revision_2(m49_code: &'static [u8; 3], english_name: &'static str, legacy_english_eight_character_abbreviation: &'static [u8], english_twelve_character_abbreviation: &'static [u8], iso_3166_alpha_codes: Option<(&'static [u8; 2], &'static [u8; 3])>) -> (M49Code, &'static str, (LegacyEightCharacterAbbreviation, TwelveCharacterAbbreviation), Option<(Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)>)
	{
		#[inline(always)]
		const fn map((iso_3166_alpha_2_code, iso_3166_alpha_3_code): (&'static [u8; 2], &'static [u8; 3])) -> (Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)
		{
			(Iso3166Dash1Alpha2Code::from(iso_3166_alpha_2_code), Iso3166Dash1Alpha3Code::from(iso_3166_alpha_3_code))
		}
		
		(
			M49Code::from(m49_code),
			english_name,
			(
				LegacyEightCharacterAbbreviation::new(legacy_english_eight_character_abbreviation),
				TwelveCharacterAbbreviation::new(english_twelve_character_abbreviation)
			),
			iso_3166_alpha_codes.map(map))
	}
	
	[
		names_and_abbreviations_for_country_revision_2(b"004", "Afghanistan", b"AFGHNSTN", b"AFGHANISTAN", Some((b"AF", b"AFG"))),
		names_and_abbreviations_for_country_revision_2(b"008", "Albania", b"ALBANIA", b"ALBANIA", Some((b"AL", b"ALB"))),
		names_and_abbreviations_for_country_revision_2(b"016", "American Samoa", b"AMER.SAM", b"AMER.SAMOA", Some((b"AS", b"ASM"))),
		names_and_abbreviations_for_country_revision_2(b"024", "Angola", b"ANGOLA", b"ANGOLA", Some((b"AO", b"AGO"))),
		names_and_abbreviations_for_country_revision_2(b"028", "Antigua and Barbuda", b"ANTIGUA", b"ANTIGUA BARB", None),
		names_and_abbreviations_for_country_revision_2(b"036", "Australia", b"AUSTRAL.", b"AUSTRALIA", Some((b"AU", b"AUS"))),
		names_and_abbreviations_for_country_revision_2(b"044", "Bahamas", b"BAHAMAS", b"BAHAMAS", Some((b"BS", b"BHS"))),
		names_and_abbreviations_for_country_revision_2(b"056", "Beigium", b"BELGIUM", b"BELGIUM", Some((b"BE", b"BEL"))),
		names_and_abbreviations_for_country_revision_2(b"068", "Bolivia", b"BOLIVIA", b"BOLIVIA", Some((b"BO", b"BOL"))),
		names_and_abbreviations_for_country_revision_2(b"086", "British Indian Ocean Territory", b"BR.IN.OG", b"BR.IND.OC.TR", Some((b"IO", b"IOT"))),
		names_and_abbreviations_for_country_revision_2(b"092", "British Virgin Islands", b"BR.VIR.I", b"BR.VIRGIN IS", Some((b"VG", b"VGB"))),
		names_and_abbreviations_for_country_revision_2(b"090", "Solomon Islands", b"SOLOMONI", b"SOLOMON ISLS", Some((b"SB", b"SLB"))),
		names_and_abbreviations_for_country_revision_2(b"104", "Burma", b"BURMA", b"BURMA", Some((b"BU", b"BUR"))),
		names_and_abbreviations_for_country_revision_2(b"112", "Byelorussian Soviet Socialist Republic", b"BYELORUS", b"BYELORUSSIA", Some((b"BY", b"BYS"))),
		names_and_abbreviations_for_country_revision_2(b"116", "Democratic Kampuchea", b"DEM.KAMP", b"DM.KAMPUCHEA", Some((b"KH", b"KHM"))),
		names_and_abbreviations_for_country_revision_2(b"120", "United Republic of Cameroon", b"U.RP.CAM", b"UNTD.RP.CAMR", Some((b"CM", b"CMR"))),
		names_and_abbreviations_for_country_revision_2(b"124", "Canada", b"CANADA", b"CANADA", Some((b"CA", b"CAN"))),
		names_and_abbreviations_for_country_revision_2(b"128", "Canton and Enderbury Islands", b"CANTON I", b"CANTON ISLDS", Some((b"CT", b"CTE"))),
		names_and_abbreviations_for_country_revision_2(b"132", "Cape Verde", b"CAPE VRD CAPE", b"VERDE", Some((b"CV", b"CPV"))),
		names_and_abbreviations_for_country_revision_2(b"136", "Cayman Islands", b"CAYMAN I", b"CAYMAN ISLDS", Some((b"KY", b"CYM"))),
		names_and_abbreviations_for_country_revision_2(b"144", "Sri Lanka", b"SR.LANKA", b"SRI LANKA", Some((b"LK", b"LKA"))),
		names_and_abbreviations_for_country_revision_2(b"148", "Chad", b"CHAD", b"CHAD", Some((b"TD", b"TCD"))),
		names_and_abbreviations_for_country_revision_2(b"149", "Central African Republic", b"C.AF.REP.", b"CENT.AF.REP", Some((b"CF", b"CAF"))),
		names_and_abbreviations_for_country_revision_2(b"152", "Chile", b"CHILE", b"CHILE", Some((b"CL", b"CHL"))),
		names_and_abbreviations_for_country_revision_2(b"156", "China", b"CHINA", b"CHINA", Some((b"CN", b"CHN"))),
		names_and_abbreviations_for_country_revision_2(b"162", "Christmas Island [Australia]", b"CHRIS.IS", b"CHRISTMAS IS", Some((b"CX", b"CXR"))),
		names_and_abbreviations_for_country_revision_2(b"166", "Cocos (Keeling) Islands", b"COCOS IS", b"COCOS ISLNDS", Some((b"CC", b"CCK"))),
		names_and_abbreviations_for_country_revision_2(b"174", "Comoros", b"COMOROS", b"COMOROS", Some((b"KM", b"COM"))),
		names_and_abbreviations_for_country_revision_2(b"178", "Congo", b"CONGO", b"CONGO", Some((b"CG", b"COG"))),
		names_and_abbreviations_for_country_revision_2(b"179", "Colombia", b"COLOMBIA", b"COLOMBIA", Some((b"CO", b"COL"))),
		names_and_abbreviations_for_country_revision_2(b"184", "Cook Islands", b"COOK IS.", b"COOK ISLANDS", Some((b"CK", b"COK"))),
		names_and_abbreviations_for_country_revision_2(b"188", "Costa Rica", b"COSTA RC", b"COSTA RICA", Some((b"CR", b"CRI"))),
		names_and_abbreviations_for_country_revision_2(b"189", "Zaire", b"ZAIRE", b"ZAIRE", Some((b"ZR", b"ZAR"))),
		names_and_abbreviations_for_country_revision_2(b"192", "Cuba", b"CUBA", b"CUBA", Some((b"CU", b"CUB"))),
		names_and_abbreviations_for_country_revision_2(b"196", "Cyprus", b"CYPRUS", b"CYPRUS", Some((b"CY", b"CYP"))),
		names_and_abbreviations_for_country_revision_2(b"198", "Burundi", b"BURUNDI", b"BURUNDI", Some((b"BI", b"BDI"))),
		names_and_abbreviations_for_country_revision_2(b"199", "Bulgaria", b"BULGARIA", b"BULGARIA", Some((b"BG", b"BGR"))),
		names_and_abbreviations_for_country_revision_2(b"200", "Czechoslovakia", b"CZCHSLVK", b"CZECHOSLOVAK", Some((b"CS", b"CSK"))),
		names_and_abbreviations_for_country_revision_2(b"204", "Benin", b"BENIN", b"BENIN", Some((b"BJ", b"BEN"))),
		names_and_abbreviations_for_country_revision_2(b"208", "Denmark", b"DENMARK", b"DENMARK", Some((b"DK", b"DNK"))),
		names_and_abbreviations_for_country_revision_2(b"212", "Dominica", b"DOMINICA", b"DOMINICA", Some((b"DM", b"DMA"))),
		names_and_abbreviations_for_country_revision_2(b"214", "Dominican Republic", b"DOMIN.RP DOMINICAN", b"RP.", Some((b"DO", b"DOM"))),
		names_and_abbreviations_for_country_revision_2(b"218", "Ecuador", b"ECUADOR", b"ECUADOR", Some((b"EC", b"ECU"))),
		names_and_abbreviations_for_country_revision_2(b"222", "El Salvador", b"EL SALVD", b"EL SALVADOR", Some((b"SV", b"SLV"))),
		names_and_abbreviations_for_country_revision_2(b"226", "EquatorialGuinea", b"EQ.GUIN", b"EQ.GUINEA", Some((b"GQ", b"GNQ"))),
		names_and_abbreviations_for_country_revision_2(b"234", "Faeroe Islands", b"FAEROE I", b"FAEROE ISLOS", Some((b"FO", b"FRO"))),
		names_and_abbreviations_for_country_revision_2(b"238", "Faikland Islands (Malvinas)", b"FALKLD I", b"FALKLAND ISL", Some((b"FK", b"FLK"))),
		names_and_abbreviations_for_country_revision_2(b"239", "Ethiopia", b"ETHIOPIA", b"ETHIOPIA", Some((b"ET", b"ETH"))),
		names_and_abbreviations_for_country_revision_2(b"242", "Fiji", b"FIJI", b"FIJI", Some((b"FJ", b"FJI"))),
		names_and_abbreviations_for_country_revision_2(b"246", "Finland", b"FINLAND", b"FINLAND", Some((b"FI", b"FIN"))),
		names_and_abbreviations_for_country_revision_2(b"250", "France", b"FRANCE", b"FRANCE", Some((b"FR", b"FRA"))),
		names_and_abbreviations_for_country_revision_2(b"254", "French Guiana", b"FR.GUIAN", b"FR.GUIANA", Some((b"GF", b"GUF"))),
		names_and_abbreviations_for_country_revision_2(b"258", "French Polynesia", b"FR POLYN", b"FR.POLYNESIA", Some((b"PF", b"PYF"))),
		names_and_abbreviations_for_country_revision_2(b"262", "Djibouti", b"DJIBOUTI", b"DJIBOUTI", Some((b"DJ", b"DJI"))),
		names_and_abbreviations_for_country_revision_2(b"266", "Gabon", b"GABON", b"GABON", Some((b"GA", b"GAB"))),
		names_and_abbreviations_for_country_revision_2(b"270", "Gambia", b"GAMBIA", b"GAMBIA", Some((b"GM", b"GMB"))),
		names_and_abbreviations_for_country_revision_2(b"274", "Gaza Strip [Palestine]", b"GAZA STR", b"GAZA STRIP", None),
		names_and_abbreviations_for_country_revision_2(b"278", "German Democratic Repubiic", b"GRMN.DR", b"GERMAN DM RP", Some((b"DD", b"DDR"))),
		names_and_abbreviations_for_country_revision_2(b"280", "Germany, Federal Republic of", b"GRMNY.FR", b"GERMANY, FR", Some((b"DE", b"DEU"))),
		names_and_abbreviations_for_country_revision_2(b"288", "Ghana", b"GHANA", b"GHANA", Some((b"GH", b"GHA"))),
		names_and_abbreviations_for_country_revision_2(b"292", "Gibraltar", b"GIBRLTAR", b"GIBRALTAR", Some((b"GI", b"GIB"))),
		names_and_abbreviations_for_country_revision_2(b"296", "Kiribati", b"KIRIBATI", b"KIRIBATI", Some((b"KI", b"KIR"))),
		names_and_abbreviations_for_country_revision_2(b"300", "Greece", b"GREECE", b"GREECE", Some((b"GR", b"GRC"))),
		names_and_abbreviations_for_country_revision_2(b"304", "Greenland", b"GREENLND", b"GREENLAND", Some((b"GL", b"GRL"))),
		names_and_abbreviations_for_country_revision_2(b"308", "Grenada", b"GRENADA", b"GRENADA", Some((b"GD", b"GRD"))),
		names_and_abbreviations_for_country_revision_2(b"312", "Guadeloupe", b"GUADLOUP", b"GUADELOUPE", Some((b"GP", b"GLP"))),
		names_and_abbreviations_for_country_revision_2(b"316", "Guam", b"GUAM", b"GUAM", Some((b"GU", b"GUM"))),
		names_and_abbreviations_for_country_revision_2(b"324", "Guinea", b"GUINEA", b"GUINEA", Some((b"GN", b"GIN"))),
		names_and_abbreviations_for_country_revision_2(b"328", "Guyana", b"GUYANA", b"GUYANA", Some((b"GY", b"GUY"))),
		names_and_abbreviations_for_country_revision_2(b"329", "Guatemala", b"GUATMALA", b"GUATEMALA", Some((b"GT", b"GTM"))),
		names_and_abbreviations_for_country_revision_2(b"332", "Haiti", b"HAITI", b"HAITI", Some((b"HT", b"HTI"))),
		names_and_abbreviations_for_country_revision_2(b"336", "Holy See", b"HOLY SEE", b"HOLY SEE", Some((b"VA", b"VAT"))),
		names_and_abbreviations_for_country_revision_2(b"340", "Honduras", b"HONDURAS", b"HONDURAS", Some((b"HN", b"HND"))),
		names_and_abbreviations_for_country_revision_2(b"344", "Hong Kong", b"HONG KNG", b"HONG KONG", Some((b"HK", b"HKG"))),
		names_and_abbreviations_for_country_revision_2(b"348", "Hungary", b"HUNGARY", b"HUNGARY", Some((b"HU", b"HUN"))),
		names_and_abbreviations_for_country_revision_2(b"352", "iceland", b"ICELAND", b"ICELAND", Some((b"IS", b"ISL"))),
		names_and_abbreviations_for_country_revision_2(b"356", "India", b"INDIA", b"INDIA", Some((b"IN", b"IND"))),
		names_and_abbreviations_for_country_revision_2(b"360", "Indonesia", b"INDONSIA", b"INDONESIA", Some((b"ID", b"IDN"))),
		names_and_abbreviations_for_country_revision_2(b"364", "Iran", b"IRAN", b"IRAN", Some((b"IR", b"IRN"))),
		names_and_abbreviations_for_country_revision_2(b"368", "Iraq", b"IRAQ", b"IRAQ", Some((b"IQ", b"IRQ"))),
		names_and_abbreviations_for_country_revision_2(b"372", "Ireland", b"IRELAND", b"IRELAND", Some((b"IE", b"IRL"))),
		names_and_abbreviations_for_country_revision_2(b"376", "Israel", b"ISRAEL", b"ISRAEL", Some((b"OL", b"ISR"))),
		names_and_abbreviations_for_country_revision_2(b"380", "Italy", b"ITALY", b"ITALY", Some((b"IT", b"ITA"))),
		names_and_abbreviations_for_country_revision_2(b"384", "Ivory Coast", b"IVORY CT", b"IVORY COAST", Some((b"CI", b"CIV"))),
		names_and_abbreviations_for_country_revision_2(b"388", "Jamaica", b"JAMAICA", b"JAMAICA", Some((b"JM", b"JAM"))),
		names_and_abbreviations_for_country_revision_2(b"392", "Japan", b"JAPAN", b"JAPAN", Some((b"JP", b"JPN"))),
		names_and_abbreviations_for_country_revision_2(b"396", "Johnston Island", b"JOHN.ISL", b"JOHNSTON ISL", Some((b"JT", b"JTN"))),
		names_and_abbreviations_for_country_revision_2(b"404", "Kenya", b"KENYA", b"KENYA", Some((b"KE", b"KEN"))),
		names_and_abbreviations_for_country_revision_2(b"408", "Korea, Democratic People’s Republic of", b"KORE DPR", b"KOREA D P RP", Some((b"KP", b"PRK"))),
		names_and_abbreviations_for_country_revision_2(b"410", "Korea, Republic of", b"KOREA RP", b"KOREA REP.", Some((b"KR", b"KOR"))),
		names_and_abbreviations_for_country_revision_2(b"414", "Kuwait", b"KUWAIT", b"KUWAIT", Some((b"KW", b"KWT"))),
		names_and_abbreviations_for_country_revision_2(b"418", "Lao People’s Democratic Republic", b"LAO PDR", b"LAO P.DEM.R", Some((b"LA", b"LAO"))),
		names_and_abbreviations_for_country_revision_2(b"422", "Lebanon", b"LEBANON", b"LEBANON", Some((b"LB", b"LBN"))),
		names_and_abbreviations_for_country_revision_2(b"426", "Lesotho", b"LESOTHO", b"LESOTHO", Some((b"LS", b"LSO"))),
		names_and_abbreviations_for_country_revision_2(b"430", "Liberia", b"LIBERIA", b"LIBERIA", Some((b"LR", b"LBR"))),
		names_and_abbreviations_for_country_revision_2(b"434", "Libyan Arab Jamahiriya", b"LIBY AR", b"LIBY ARAB JM", Some((b"LY", b"LBY"))),
		names_and_abbreviations_for_country_revision_2(b"438", "Liechtenstein", b"LIECHSTN", b"LIECHTENSTEN", Some((b"LI", b"LIE"))),
		names_and_abbreviations_for_country_revision_2(b"442", "Luxembourg", b"LUXMBORG", b"LUXEMBOURG", Some((b"LU", b"LUX"))),
		names_and_abbreviations_for_country_revision_2(b"446", "Macau", b"MACAU", b"MACAU", Some((b"MO", b"MAG"))),
		names_and_abbreviations_for_country_revision_2(b"450", "Madagascar", b"MADAGSCR", b"MADAGASCAR", Some((b"MG", b"MDG"))),
		names_and_abbreviations_for_country_revision_2(b"454", "Malawi", b"MALAWI", b"MALAWI", Some((b"MW", b"MWI"))),
		names_and_abbreviations_for_country_revision_2(b"458", "Malaysia", b"MALAYSIA", b"MALAYSIA", Some((b"MY", b"MYS"))),
		names_and_abbreviations_for_country_revision_2(b"462", "Maidives", b"MALDIVES", b"MALDIVES", Some((b"MV", b"MDV"))),
		names_and_abbreviations_for_country_revision_2(b"466", "Mali", b"MALI", b"MALI", Some((b"ML", b"MUI"))),
		names_and_abbreviations_for_country_revision_2(b"474", "Martinique", b"MARTNQUE", b"MARTINIQUE", Some((b"MQ", b"MTQ"))),
		names_and_abbreviations_for_country_revision_2(b"478", "Mauritania", b"MAURTNIA", b"MAURITANIA", Some((b"MR", b"MRT"))),
		names_and_abbreviations_for_country_revision_2(b"479", "Malta", b"MALTA", b"MALTA", Some((b"MT", b"MLT"))),
		names_and_abbreviations_for_country_revision_2(b"484", "Mexico", b"MEXICO", b"MEXICO", Some((b"MX", b"MEX"))),
		names_and_abbreviations_for_country_revision_2(b"488", "Midway Islands", b"MIDWAY I", b"MIDWAY ISLDS", Some((b"MI", b"MID"))),
		names_and_abbreviations_for_country_revision_2(b"489", "Mauritius", b"MAURTIUS", b"MAURITIUS", Some((b"MU", b"MUS"))),
		names_and_abbreviations_for_country_revision_2(b"490", "Jordan", b"JORDAN", b"JORDAN", Some((b"JO", b"JOR"))),
		names_and_abbreviations_for_country_revision_2(b"492", "Monaco", b"MONACO", b"MONACO", Some((b"MG", b"MCO"))),
		names_and_abbreviations_for_country_revision_2(b"496", "Mongolia", b"MONGOLIA", b"MONGOLIA", Some((b"MN", b"MNG"))),
		names_and_abbreviations_for_country_revision_2(b"504", "Morocco", b"MOROCCO", b"MOROCCO", Some((b"MA", b"MAR"))),
		names_and_abbreviations_for_country_revision_2(b"508", "Mozambique", b"MOZMBQUE", b"MOZAMBIQUE", Some((b"MZ", b"MOZ"))),
		names_and_abbreviations_for_country_revision_2(b"512", "Oman", b"OMAN", b"OMAN", Some((b"OM", b"OMN"))),
		names_and_abbreviations_for_country_revision_2(b"516", "Namibia", b"NAMIBIA", b"NAMIBIA", Some((b"NA", b"NAM"))),
		names_and_abbreviations_for_country_revision_2(b"520", "Nauru", b"NAURU", b"NAURU", Some((b"NR", b"NRU"))),
		names_and_abbreviations_for_country_revision_2(b"524", "Nepal", b"NEPAL", b"NEPAL", Some((b"NP", b"NPL"))),
		names_and_abbreviations_for_country_revision_2(b"528", "Netherlands", b"NETHLNDS", b"NETHERLANDS", Some((b"NL", b"NLD"))),
		names_and_abbreviations_for_country_revision_2(b"532", "Netherlands Antilles", b"NETH.ANT", b"NETH.ANTILES", Some((b"AN", b"ANT"))),
		names_and_abbreviations_for_country_revision_2(b"540", "New Caledonia", b"NEW CALD", b"NEW CALEDNIA", Some((b"NC", b"NCL"))),
		names_and_abbreviations_for_country_revision_2(b"548", "Vanuatu", b"VANUATU", b"VANUATU", Some((b"VU", b"VUT"))),
		names_and_abbreviations_for_country_revision_2(b"554", "New Zealand", b"NEW ZLND", b"NEW ZEALAND", Some((b"NZ", b"NZL"))),
		names_and_abbreviations_for_country_revision_2(b"558", "Nicaragua", b"NICARGUA", b"NICARAGUA", Some((b"NI", b"NIC"))),
		names_and_abbreviations_for_country_revision_2(b"562", "Niger", b"NIGER", b"NIGER", Some((b"NE", b"NER"))),
		names_and_abbreviations_for_country_revision_2(b"566", "Nigeria", b"NIGERIA", b"NIGERIA", Some((b"NG", b"NGA"))),
		names_and_abbreviations_for_country_revision_2(b"574", "Norfolk Island", b"NORF ISL", b"NORFOLK ISLD", Some((b"NF", b"NFK"))),
		names_and_abbreviations_for_country_revision_2(b"578", "Norway", b"NORWAY", b"NORWAY", Some((b"NO", b"NOR"))),
		names_and_abbreviations_for_country_revision_2(b"579", "Niue", b"NIUE", b"NIUE", Some((b"NU", b"NIU"))),
		names_and_abbreviations_for_country_revision_2(b"582", "Pacific Islands (Trust Territory)", b"PACF ISL", b"PACIFIC ISLD", Some((b"PC", b"PCI"))),
		names_and_abbreviations_for_country_revision_2(b"586", "Pakistan", b"PAKISTAN", b"PAKISTAN", Some((b"PK", b"PAK"))),
		names_and_abbreviations_for_country_revision_2(b"590", "Montserrat", b"MONTSRRT", b"MONTSERRAT", Some((b"MS", b"MSR"))),
		names_and_abbreviations_for_country_revision_2(b"591", "Panama", b"PANAMA", b"PANAMA", Some((b"PA", b"PAN"))),
		names_and_abbreviations_for_country_revision_2(b"598", "Papua New Guinea", b"PAP.N.GN PAPUA", b"N.GUIN", Some((b"PG", b"PNG"))),
		names_and_abbreviations_for_country_revision_2(b"600", "Paraquay", b"PARAGUAY", b"PARAGUAY", Some((b"PY", b"PRY"))),
		names_and_abbreviations_for_country_revision_2(b"604", "Peru", b"PERU", b"PERU", Some((b"PE", b"PER"))),
		names_and_abbreviations_for_country_revision_2(b"612", "Pitcairn Island", b"PITCRN I", b"PITCAIRN ISL", Some((b"PN", b"PCN"))),
		names_and_abbreviations_for_country_revision_2(b"616", "Poland", b"POLAND", b"POLAND", Some((b"PL", b"POL"))),
		names_and_abbreviations_for_country_revision_2(b"620", "Portugal", b"PORTUGAL", b"PORTUGAL", Some((b"PT", b"PRT"))),
		names_and_abbreviations_for_country_revision_2(b"624", "Guinea-Bissau", b"GNA.BISS", b"GUINEABISSAU", Some((b"GW", b"GNB"))),
		names_and_abbreviations_for_country_revision_2(b"626", "East Timor", b"EAST TIM EAST", b"TIMOR", Some((b"TP", b"TMP"))),
		names_and_abbreviations_for_country_revision_2(b"630", "Puerto Rico", b"PUERTO R", b"PUERTO RICO", Some((b"PR", b"PRI"))),
		names_and_abbreviations_for_country_revision_2(b"634", "Qatar", b"QATAR", b"QATAR", Some((b"QA", b"QAT"))),
		names_and_abbreviations_for_country_revision_2(b"638", "Réunion", b"REUNION", b"REUNION", Some((b"RE", b"REU"))),
		names_and_abbreviations_for_country_revision_2(b"642", "Romania", b"ROMANIA", b"ROMANIA", Some((b"RO", b"ROM"))),
		names_and_abbreviations_for_country_revision_2(b"646", "Rwanda", b"RWANDA", b"RWANDA", Some((b"RW", b"RWA"))),
		names_and_abbreviations_for_country_revision_2(b"654", "St. Helena", b"ST.HELEN", b"ST.HELENA", Some((b"SH", b"SHN"))),
		names_and_abbreviations_for_country_revision_2(b"659", "St. Kitts-Nevis", b"ST.KITTS", b"ST.KITTS NEV", None),
		names_and_abbreviations_for_country_revision_2(b"660", "Anguilla", b"ANGUILLA", b"ANGUILLA", None),
		names_and_abbreviations_for_country_revision_2(b"662", "Saint Lucia", b"ST.LUCIA", b"SAINT LUCIA", Some((b"LG", b"LCA"))),
		names_and_abbreviations_for_country_revision_2(b"666", "St. Pierre and Miquelon", b"ST.P.MIQ ST", b"PIER.MIQU", Some((b"PM", b"SPM"))),
		names_and_abbreviations_for_country_revision_2(b"670", "Saint Vincent and the Grenadine", b"ST.VINCT", b"ST.VINCT GRN", Some((b"VC", b"VCT"))),
		names_and_abbreviations_for_country_revision_2(b"674", "San Marino", b"SAN MRNO", b"SAN MARINO", Some((b"SM", b"SMR"))),
		names_and_abbreviations_for_country_revision_2(b"678", "Sao Tome and Principe", b"S.TM.PRN", b"SAO TOME PRN", Some((b"ST", b"STP"))),
		names_and_abbreviations_for_country_revision_2(b"682", "Saudi Arabia", b"SAUD.ARB", b"SAUDI ARABIA", Some((b"SA", b"SAU"))),
		names_and_abbreviations_for_country_revision_2(b"686", "Senegal", b"SENEGAL", b"SENEGAL", Some((b"SN", b"SEN"))),
		names_and_abbreviations_for_country_revision_2(b"694", "Sierra Leone", b"SIER LNE", b"SIERRA LEONE", Some((b"SL", b"SLE"))),
		names_and_abbreviations_for_country_revision_2(b"698", "Philippines", b"PHILIPP.", b"PHILIPPINES", Some((b"PH", b"PHL"))),
		names_and_abbreviations_for_country_revision_2(b"699", "Seychelles", b"SEYCHLLS", b"SEYCHELLES", Some((b"SC", b"SYC"))),
		names_and_abbreviations_for_country_revision_2(b"702", "Singapore", b"SINGAPOR", b"SINGAPORE", Some((b"SG", b"SGP"))),
		names_and_abbreviations_for_country_revision_2(b"704", "Viet Nam", b"VIET NAM", b"VIET NAM", Some((b"VN", b"VNM"))),
		names_and_abbreviations_for_country_revision_2(b"710", "South Africa", b"S.AFRICA", b"SOUTH AFRICA", Some((b"ZA", b"ZAF"))),
		names_and_abbreviations_for_country_revision_2(b"716", "Zimbabwe", b"ZIMBABWE", b"ZIMBABWE", Some((b"ZW", b"ZWE"))),
		names_and_abbreviations_for_country_revision_2(b"724", "Spain", b"SPAIN", b"SPAIN", Some((b"ES", b"ESP"))),
		names_and_abbreviations_for_country_revision_2(b"729", "Democratic Yemen", b"D.YEMEN", b"DEM.YEMEN", Some((b"YD", b"YMD"))),
		names_and_abbreviations_for_country_revision_2(b"732", "Western Sahara", b"W.SAHARA", b"WESTN.SAHARA", Some((b"EH", b"ESH"))),
		names_and_abbreviations_for_country_revision_2(b"736", "Sudan", b"SUDAN", b"SUDAN", Some((b"SD", b"SDN"))),
		names_and_abbreviations_for_country_revision_2(b"744", "Svalbard and Jan Mayen Islands", b"SVALBARD", b"SVALBARD ISL", Some((b"SJ", b"SUM"))),
		names_and_abbreviations_for_country_revision_2(b"748", "Swaziland", b"SWAZILND", b"SWAZILAND", Some((b"SZ", b"SWZ"))),
		names_and_abbreviations_for_country_revision_2(b"749", "Suriname", b"SURINAME", b"SURINAME", Some((b"SR", b"SUR"))),
		names_and_abbreviations_for_country_revision_2(b"752", "Sweden", b"SWEDEN", b"SWEDEN", Some((b"SE", b"SWE"))),
		names_and_abbreviations_for_country_revision_2(b"756", "Switzerland", b"SWITZRLD", b"SWITZERLAND", Some((b"CH", b"CHE"))),
		names_and_abbreviations_for_country_revision_2(b"760", "Syrian Arab Repubiic", b"SYRNAR", b"SYRN ARAB RP", Some((b"SY", b"SYR"))),
		names_and_abbreviations_for_country_revision_2(b"764", "Thailand", b"THAILAND", b"THAILAND", Some((b"TH", b"THA"))),
		names_and_abbreviations_for_country_revision_2(b"768", "Togo", b"TOGO", b"TOGO", Some((b"TG", b"TGO"))),
		names_and_abbreviations_for_country_revision_2(b"772", "Tokelau", b"TOKELAU", b"TOKELAU", Some((b"TK", b"TKL"))),
		names_and_abbreviations_for_country_revision_2(b"776", "Tonga", b"TONGA", b"TONGA", Some((b"TO", b"TON"))),
		names_and_abbreviations_for_country_revision_2(b"784", "United Arab Emirates", b"ULAR EMR UNTD ARAB", b"EM", Some((b"AE", b"ARE"))),
		names_and_abbreviations_for_country_revision_2(b"788", "Tunisia", b"TUNISIA", b"TUNISIA", Some((b"TN", b"TUN"))),
		names_and_abbreviations_for_country_revision_2(b"789", "Trinidad and Tabago", b"TRINIDAD", b"TRINIDAD TBG", Some((b"TT", b"TTO"))),
		names_and_abbreviations_for_country_revision_2(b"792", "Turkey", b"TURKEY", b"TURKEY", Some((b"TR", b"TUR"))),
		names_and_abbreviations_for_country_revision_2(b"796", "Somalia", b"SOMALIA", b"SOMALIA", Some((b"SO", b"SOM"))),
		names_and_abbreviations_for_country_revision_2(b"796", "Turks and Caicos Islands", b"TURKS IS", b"TURKS CA.ISL", Some((b"TC", b"TCA"))),
		names_and_abbreviations_for_country_revision_2(b"798", "Tuvalu", b"TUVALU", b"TUVALU", Some((b"TV", b"TUV"))),
		names_and_abbreviations_for_country_revision_2(b"804", "Ukrainian Soviet Socialist Republic", b"UKRAINE", b"UKRAINE SSR", Some((b"UA", b"UKR"))),
		names_and_abbreviations_for_country_revision_2(b"809", "Uganda", b"UGANDA", b"UGANDA", Some((b"UG", b"UGA"))),
		names_and_abbreviations_for_country_revision_2(b"818", "Egypt", b"EGYPT", b"EGYPT", Some((b"EG", b"EGY"))),
		names_and_abbreviations_for_country_revision_2(b"819", "Union of Soviet Socialist Republics", b"USSR", b"USSR", Some((b"SU", b"SUN"))),
		names_and_abbreviations_for_country_revision_2(b"826", "United Kingdom", b"UK", b"UK", Some((b"GB", b"GBR"))),
		names_and_abbreviations_for_country_revision_2(b"833", "Isle of Man", b"ISL.MAN", b"ISLE OF MAN", None),
		names_and_abbreviations_for_country_revision_2(b"834", "United Republic of Tanzania", b"U.RP TNZ UNTD.RP", b"TANZ", Some((b"TZ", b"TZA"))),
		names_and_abbreviations_for_country_revision_2(b"839", "Channel Islands", b"CHNL IS.", b"CHANNL ISLDS", None),
		names_and_abbreviations_for_country_revision_2(b"840", "United States", b"USA", b"USA", Some((b"US", b"USA"))),
		names_and_abbreviations_for_country_revision_2(b"854", "Upper Voita", b"UPPR VLT", b"UPPER VOLTA", Some((b"HV", b"HVO"))),
		names_and_abbreviations_for_country_revision_2(b"858", "Uruguay", b"URUGUAY", b"URUGUAY", Some((b"UY", b"URY"))),
		names_and_abbreviations_for_country_revision_2(b"859", "United States Virgin Islands", b"U.S.VAS", b"US.VIRGIN IS", Some((b"VI", b"VIR"))),
		names_and_abbreviations_for_country_revision_2(b"862", "Venezuela", b"VENZUELA", b"VENEZUELA", Some((b"VE", b"VEN"))),
		names_and_abbreviations_for_country_revision_2(b"872", "Wake Island", b"WAKE ISL.", b"WAKE ISLAND", Some((b"WK", b"WAK"))),
		names_and_abbreviations_for_country_revision_2(b"876", "Wallis and Futuna Islands", b"WALLIS I", b"WALLIS FUT.", Some((b"WE", b"WLF"))),
		names_and_abbreviations_for_country_revision_2(b"882", "Samoa", b"SAMOA", b"SAMOA", Some((b"WS", b"WSM"))),
		names_and_abbreviations_for_country_revision_2(b"886", "Yemen", b"YEMEN", b"YEMEN", Some((b"YE", b"YEM"))),
		names_and_abbreviations_for_country_revision_2(b"894", "Zambia", b"ZAMBIA", b"ZAMBIA", Some((b"ZM", b"ZMB"))),
		names_and_abbreviations_for_country_revision_2(b"890", "Yugoslavia", b"YUGOSLAV", b"YUGOSLAVIA", Some((b"YU", b"YUG"))),
		names_and_abbreviations_for_country_revision_2(b"912", "Algeria", b"ALGERIA", b"ALGERIA", Some((b"DZ", b"DZA"))),
		names_and_abbreviations_for_country_revision_2(b"929", "Andorra", b"ANDORRA", b"ANDORRA", Some((b"AD", b"AND"))),
		names_and_abbreviations_for_country_revision_2(b"932", "Argentina", b"ARGNTINA", b"ARGENTINA", Some((b"AR", b"ARG"))),
		names_and_abbreviations_for_country_revision_2(b"948", "Bahrain", b"BAHRAIN", b"BAHRAIN", Some((b"BH", b"BHA"))),
		names_and_abbreviations_for_country_revision_2(b"949", "Austria", b"AUSTRIA", b"AUSTRIA", Some((b"AT", b"AUT"))),
		names_and_abbreviations_for_country_revision_2(b"950", "Bangladesh", b"BANGLDSH", b"BANGLADESH", Some((b"BD", b"BGD"))),
		names_and_abbreviations_for_country_revision_2(b"952", "Barbados", b"BARBADOS.", b"BARBADOS", Some((b"BB", b"BRB"))),
		names_and_abbreviations_for_country_revision_2(b"960", "Bermuda", b"BERMUDA", b"BERMUDA", Some((b"BM", b"BMU"))),
		names_and_abbreviations_for_country_revision_2(b"964", "Bhutan", b"BHUTAN", b"BHUTAN", Some((b"BT", b"BTN"))),
		names_and_abbreviations_for_country_revision_2(b"972", "Botswana", b"BOTSWANA", b"BOTSWANA", Some((b"BW", b"BWA"))),
		names_and_abbreviations_for_country_revision_2(b"976", "Brazil", b"BRAZIL", b"BRAZIL", Some((b"BR", b"BRA"))),
		names_and_abbreviations_for_country_revision_2(b"984", "Belize", b"BELIZE", b"BELIZE", Some((b"BZ", b"BLZ"))),
		names_and_abbreviations_for_country_revision_2(b"996", "Brunei", b"BRUNEI", b"BRUNEI", Some((b"BN", b"BRN"))),
	]
};
