// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// Extracted from the PDF for UNSD Series M, Nº49 Revision 3 (1996).
///
/// TODO: Non-English names.
/// There are no French abbreviations.
/// There are no 4 character abbreviations.
/// There are no 8 character abbreviations.
pub(super) const NamesAndAbbreviationsForCountriesRevision3: [(M49Code, &'static str, Option<TwelveCharacterAbbreviation>, Option<Iso3166Dash1Alpha3Code>); 233] =
{
	#[inline(always)]
	const fn names_and_abbreviations_for_country_revision_3(m49_code: &'static [u8; 3], english_name: &'static str, english_twelve_character_abbreviation: Option<&'static [u8]>, iso_3166_alpha_3_code: Option<&'static [u8; 3]>) -> (M49Code, &'static str, Option<TwelveCharacterAbbreviation>, Option<Iso3166Dash1Alpha3Code>)
	{
		#[inline(always)]
		const fn map(iso_3166_alpha_3_code: &'static [u8; 3]) -> Iso3166Dash1Alpha3Code
		{
			Iso3166Dash1Alpha3Code::from(iso_3166_alpha_3_code)
		}
		
		(
			M49Code::from(m49_code),
			english_name,
			english_twelve_character_abbreviation.map(TwelveCharacterAbbreviation::new),
			iso_3166_alpha_3_code.map(map)
		)
	}
	
	[
		names_and_abbreviations_for_country_revision_3(b"004", "Afghanistan", Some(b"AFGHANISTAN"), Some(b"AFG")),
		names_and_abbreviations_for_country_revision_3(b"008", "Albania", Some(b"ALBANIA"), Some(b"ALB")),
		names_and_abbreviations_for_country_revision_3(b"012", "Algeria", Some(b"ALGERIA"), Some(b"DZA")),
		names_and_abbreviations_for_country_revision_3(b"016", "American Samoa", Some(b"AMER SAMOA"), Some(b"ASM")),
		names_and_abbreviations_for_country_revision_3(b"020", "Andorra", Some(b"ANDORRA"), Some(b"AND")),
		names_and_abbreviations_for_country_revision_3(b"024", "Angola", Some(b"ANGOLA"), Some(b"AGO")),
		names_and_abbreviations_for_country_revision_3(b"028", "Antigua and Barbuda", Some(b"ANTIGUA,BARB"), Some(b"ATG")),
		names_and_abbreviations_for_country_revision_3(b"031", "Azerbaijan", Some(b"AZERBAIJAN"), Some(b"AZE")),
		names_and_abbreviations_for_country_revision_3(b"032", "Argentina", Some(b"ARGENTINA"), Some(b"ARG")),
		names_and_abbreviations_for_country_revision_3(b"036", "Australia", Some(b"AUSTRALIA"), Some(b"AUS")),
		names_and_abbreviations_for_country_revision_3(b"040", "Austria", Some(b"AUSTRIA"), Some(b"AUT")),
		names_and_abbreviations_for_country_revision_3(b"044", "Bahamas", Some(b"BAHAMAS"), Some(b"BHS")),
		names_and_abbreviations_for_country_revision_3(b"048", "Bahrain", Some(b"BAHRAIN"), Some(b"BHR")),
		names_and_abbreviations_for_country_revision_3(b"050", "Bangladesh", Some(b"BANGLADESH"), Some(b"BGD")),
		names_and_abbreviations_for_country_revision_3(b"051", "Armenia", Some(b"ARMENIA"), Some(b"ARM")),
		names_and_abbreviations_for_country_revision_3(b"052", "Barbados", Some(b"BARBADOS"), Some(b"BRB")),
		names_and_abbreviations_for_country_revision_3(b"056", "Belgium", Some(b"BELGIUM"), Some(b"BEL")),
		names_and_abbreviations_for_country_revision_3(b"060", "Bermuda", Some(b"BERMUDA"), Some(b"BMU")),
		names_and_abbreviations_for_country_revision_3(b"064", "Bhutan", Some(b"BHUTAN"), Some(b"BTN")),
		names_and_abbreviations_for_country_revision_3(b"068", "Bolivia", Some(b"BOLIVIA"), Some(b"BOL")),
		names_and_abbreviations_for_country_revision_3(b"070", "Bosnia and Herzegovina", Some(b"BOSNIA HERZG"), Some(b"BIH")),
		names_and_abbreviations_for_country_revision_3(b"072", "Botswana", Some(b"BOTSWANA"), Some(b"BWA")),
		names_and_abbreviations_for_country_revision_3(b"076", "Brazil", Some(b"BRAZIL"), Some(b"BRA")),
		names_and_abbreviations_for_country_revision_3(b"084", "Belize", Some(b"BELIZE"), Some(b"BLZ")),
		names_and_abbreviations_for_country_revision_3(b"090", "Solomon Islands", Some(b"SOLOMON IS"), Some(b"SLB")),
		names_and_abbreviations_for_country_revision_3(b"092", "British Virgin Islands", Some(b"BR.VIRGIN IS"), Some(b"VGB")),
		names_and_abbreviations_for_country_revision_3(b"096", "Brunei Darussalam", Some(b"BRUNEI DARSM"), Some(b"BRN")),
		names_and_abbreviations_for_country_revision_3(b"100", "Bulgaria", Some(b"BULGARIA"), Some(b"BGR")),
		names_and_abbreviations_for_country_revision_3(b"104", "Myanmar", Some(b"MYANMAR"), Some(b"MMR")),
		names_and_abbreviations_for_country_revision_3(b"108", "Burundi", Some(b"BURUNDI"), Some(b"BDI")),
		names_and_abbreviations_for_country_revision_3(b"112", "Belarus", Some(b"BELARUS"), Some(b"BLR")),
		names_and_abbreviations_for_country_revision_3(b"116", "Cambodia", Some(b"CAMBODIA"), Some(b"KHM")),
		names_and_abbreviations_for_country_revision_3(b"120", "Cameroon", Some(b"CAMEROON"), Some(b"CMR")),
		names_and_abbreviations_for_country_revision_3(b"124", "Canada", Some(b"CANADA"), Some(b"CAN")),
		names_and_abbreviations_for_country_revision_3(b"132", "Cape Verde", Some(b"CAPE VERDE"), Some(b"CPV")),
		names_and_abbreviations_for_country_revision_3(b"136", "Cayman Islands", Some(b"CAYMAN IS"), Some(b"CYM")),
		names_and_abbreviations_for_country_revision_3(b"140", "Central African Republic", Some(b"CENT.AFR.REP"), Some(b"CAF")),
		names_and_abbreviations_for_country_revision_3(b"144", "Sri Lanka", Some(b"SRI LANKA"), Some(b"LKA")),
		names_and_abbreviations_for_country_revision_3(b"148", "Chad", Some(b"CHAD"), Some(b"TCD")),
		names_and_abbreviations_for_country_revision_3(b"152", "Chile", Some(b"CHILE"), Some(b"CHN")),
		names_and_abbreviations_for_country_revision_3(b"156", "China", Some(b"CHINA"), Some(b"COL")),
		names_and_abbreviations_for_country_revision_3(b"158", "Taiwan Province of China", None, Some(b"TWN")),
		names_and_abbreviations_for_country_revision_3(b"170", "Colombia", Some(b"COLOMBIA"), Some(b"COM")),
		names_and_abbreviations_for_country_revision_3(b"174", "Comoros", Some(b"COMOROS"), Some(b"COG")),
		names_and_abbreviations_for_country_revision_3(b"178", "Congo", Some(b"CONGO"), Some(b"COK")),
		names_and_abbreviations_for_country_revision_3(b"180", "Zaire", Some(b"ZAIRE"), Some(b"ZAR")),
		names_and_abbreviations_for_country_revision_3(b"184", "Cook Islands", Some(b"COOK IS"), Some(b"CRI")),
		names_and_abbreviations_for_country_revision_3(b"188", "Costa Rica", Some(b"COSTA RICA"), Some(b"CIV")),
		names_and_abbreviations_for_country_revision_3(b"191", "Croatia", Some(b"CROATIA"), Some(b"AFG")),
		names_and_abbreviations_for_country_revision_3(b"192", "Cuba", Some(b"CUBA"), Some(b"CUB")),
		names_and_abbreviations_for_country_revision_3(b"196", "Cyprus", Some(b"CYPRUS"), Some(b"CYP")),
		names_and_abbreviations_for_country_revision_3(b"203", "Czech Republic", Some(b"CZECH REP"), Some(b"CZE")),
		names_and_abbreviations_for_country_revision_3(b"204", "Benin", Some(b"BENIN"), Some(b"BEN")),
		names_and_abbreviations_for_country_revision_3(b"208", "Denmark", Some(b"DENMARK"), Some(b"DNK")),
		names_and_abbreviations_for_country_revision_3(b"212", "Dominica", Some(b"DOMINICA"), Some(b"DMA")),
		names_and_abbreviations_for_country_revision_3(b"214", "Dominican Republic", Some(b"DOMINICAN RP"), Some(b"DOM")),
		names_and_abbreviations_for_country_revision_3(b"218", "Ecuador", Some(b"ECUADOR"), Some(b"ECU")),
		names_and_abbreviations_for_country_revision_3(b"222", "El Salvador", Some(b"EL SALVADOR"), Some(b"SLV")),
		names_and_abbreviations_for_country_revision_3(b"226", "Equatorial Guinea", Some(b"EQ.GUINEA"), Some(b"GNQ")),
		names_and_abbreviations_for_country_revision_3(b"231", "Ethiopia", Some(b"ETHIOPIA"), Some(b"ETH")),
		names_and_abbreviations_for_country_revision_3(b"232", "Eritrea", Some(b"ERITREA"), Some(b"ERI")),
		names_and_abbreviations_for_country_revision_3(b"233", "Estonia", Some(b"ESTONIA"), Some(b"EST")),
		names_and_abbreviations_for_country_revision_3(b"234", "Faeroe Islands", Some(b"FAEROE IS"), Some(b"FRO")),
		names_and_abbreviations_for_country_revision_3(b"238", "Falkland Islands (Malvinas)", Some(b"FALKLAND IS"), Some(b"FLK")),
		names_and_abbreviations_for_country_revision_3(b"242", "Fiji", Some(b"FIJI"), Some(b"FJI")),
		names_and_abbreviations_for_country_revision_3(b"246", "Finland", Some(b"FINLAND"), Some(b"FIN")),
		names_and_abbreviations_for_country_revision_3(b"250", "France", Some(b"FRANCE"), Some(b"FRA")),
		names_and_abbreviations_for_country_revision_3(b"254", "French Guiana", Some(b"FR.GUIANA"), Some(b"GUF")),
		names_and_abbreviations_for_country_revision_3(b"258", "French Polynesia", Some(b"FR.POLYNESIA"), Some(b"PYF")),
		names_and_abbreviations_for_country_revision_3(b"262", "Djibouti", Some(b"DJIBOUTI"), Some(b"DJI")),
		names_and_abbreviations_for_country_revision_3(b"266", "Gabon", Some(b"GABON"), Some(b"GAB")),
		names_and_abbreviations_for_country_revision_3(b"268", "Georgia", Some(b"GEORGIA"), Some(b"GEO")),
		names_and_abbreviations_for_country_revision_3(b"270", "Gambia", Some(b"GAMBIA"), Some(b"GMB")),
		names_and_abbreviations_for_country_revision_3(b"274", "Gaza Strip", None, None),
		names_and_abbreviations_for_country_revision_3(b"276", "Germany", Some(b"GERMANY"), Some(b"DEU")),
		names_and_abbreviations_for_country_revision_3(b"288", "Ghana", Some(b"GHANA"), Some(b"GHA")),
		names_and_abbreviations_for_country_revision_3(b"292", "Gibraltar", Some(b"GIBRALTAR"), Some(b"GIB")),
		names_and_abbreviations_for_country_revision_3(b"296", "Kiribati", Some(b"KIRIBATI"), Some(b"KIR")),
		names_and_abbreviations_for_country_revision_3(b"300", "Greece", Some(b"GREECE"), Some(b"GRC")),
		names_and_abbreviations_for_country_revision_3(b"304", "Greenland", Some(b"GREENLAND"), Some(b"GRL")),
		names_and_abbreviations_for_country_revision_3(b"308", "Grenada", Some(b"GRENADA"), Some(b"GRD")),
		names_and_abbreviations_for_country_revision_3(b"312", "Guadeloupe", Some(b"GUADELOUPE"), Some(b"GLP")),
		names_and_abbreviations_for_country_revision_3(b"316", "Guam", Some(b"GUAM"), Some(b"GUM")),
		names_and_abbreviations_for_country_revision_3(b"320", "Guatemala", Some(b"GUATEMALA"), Some(b"GTM")),
		names_and_abbreviations_for_country_revision_3(b"324", "Guinea", Some(b"GUINEA"), Some(b"GIN")),
		names_and_abbreviations_for_country_revision_3(b"328", "Guyana", Some(b"GUYANA"), Some(b"GUY")),
		names_and_abbreviations_for_country_revision_3(b"332", "Haiti", Some(b"HAITI"), Some(b"HTI")),
		names_and_abbreviations_for_country_revision_3(b"336", "Holy See", Some(b"HOLY SEE"), Some(b"VAT")),
		names_and_abbreviations_for_country_revision_3(b"340", "Honduras", Some(b"HONDURAS"), Some(b"HND")),
		names_and_abbreviations_for_country_revision_3(b"344", "Hong Kong", Some(b"HONG KONG"), Some(b"HKG")),
		names_and_abbreviations_for_country_revision_3(b"348", "Hungary", Some(b"HUNGARY"), Some(b"HUN")),
		names_and_abbreviations_for_country_revision_3(b"352", "Iceland", Some(b"ICELAND"), Some(b"ISL")),
		names_and_abbreviations_for_country_revision_3(b"356", "India", Some(b"INDIA"), Some(b"IND")),
		names_and_abbreviations_for_country_revision_3(b"360", "Indonesia", Some(b"INDONESIA"), Some(b"IDN")),
		names_and_abbreviations_for_country_revision_3(b"364", "Iran (Islamic Republic of)", Some(b"IRAN"), Some(b"IRN")),
		names_and_abbreviations_for_country_revision_3(b"368", "Iraq", Some(b"IRAQ"), Some(b"IRQ")),
		names_and_abbreviations_for_country_revision_3(b"372", "Ireland", Some(b"IRELAND"), Some(b"IRL")),
		names_and_abbreviations_for_country_revision_3(b"376", "Israel", Some(b"ISRAEL"), Some(b"ISR")),
		names_and_abbreviations_for_country_revision_3(b"380", "Italy", Some(b"ITALY"), Some(b"ITA")),
		names_and_abbreviations_for_country_revision_3(b"384", "Côte d'Ivoire", Some(b"COTE DIVOIRE"), Some(b"HRV")),
		names_and_abbreviations_for_country_revision_3(b"388", "Jamaica", Some(b"JAMAICA"), Some(b"JAM")),
		names_and_abbreviations_for_country_revision_3(b"392", "Japan", Some(b"JAPAN"), Some(b"JPN")),
		names_and_abbreviations_for_country_revision_3(b"398", "Kazakstan", Some(b"KAZAKSTAN"), Some(b"KAZ")),
		names_and_abbreviations_for_country_revision_3(b"400", "Jordan", Some(b"JORDAN"), Some(b"JOR")),
		names_and_abbreviations_for_country_revision_3(b"404", "Kenya", Some(b"KENYA"), Some(b"KEN")),
		names_and_abbreviations_for_country_revision_3(b"408", "Democratic People's Rep. of Korea", Some(b"KOREA D P RP"), Some(b"PRK")),
		names_and_abbreviations_for_country_revision_3(b"410", "Republic of Korea", Some(b"KOREA REP."), Some(b"KOR")),
		names_and_abbreviations_for_country_revision_3(b"414", "Kuwait", Some(b"KUWAIT"), Some(b"KWT")),
		names_and_abbreviations_for_country_revision_3(b"417", "Kyrgyzstan", Some(b"KYRGYZSTAN"), Some(b"KGZ")),
		names_and_abbreviations_for_country_revision_3(b"418", "Lao People's Democratic Republic", Some(b"LAO P.DEM.R."), Some(b"LAO")),
		names_and_abbreviations_for_country_revision_3(b"422", "Lebanon", Some(b"LEBANON"), Some(b"LBN")),
		names_and_abbreviations_for_country_revision_3(b"426", "Lesotho", Some(b"LESOTHO"), Some(b"LSO")),
		names_and_abbreviations_for_country_revision_3(b"428", "Latvia", Some(b"LATVIA"), Some(b"LVA")),
		names_and_abbreviations_for_country_revision_3(b"430", "Liberia", Some(b"LIBERIA"), Some(b"LBR")),
		names_and_abbreviations_for_country_revision_3(b"434", "Libyan Arab Jamahiriya", Some(b"LIBYA"), Some(b"LBY")),
		names_and_abbreviations_for_country_revision_3(b"438", "Liechtenstein", Some(b"LIECHTENSTEN"), Some(b"LIE")),
		names_and_abbreviations_for_country_revision_3(b"440", "Lithuania", Some(b"LITHUANIA"), Some(b"LTU")),
		names_and_abbreviations_for_country_revision_3(b"442", "Luxembourg", Some(b"LUXEMBOURG"), Some(b"LUX")),
		names_and_abbreviations_for_country_revision_3(b"446", "Macau", Some(b"MACAU"), Some(b"MAC")),
		names_and_abbreviations_for_country_revision_3(b"450", "Madagascar", Some(b"MADAGASCAR"), Some(b"MDG")),
		names_and_abbreviations_for_country_revision_3(b"454", "Malawi", Some(b"MALAWI"), Some(b"MWI")),
		names_and_abbreviations_for_country_revision_3(b"458", "Malaysia", Some(b"MALAYSIA"), Some(b"MYS")),
		names_and_abbreviations_for_country_revision_3(b"462", "Maldives", Some(b"MALDIVES"), Some(b"MDV")),
		names_and_abbreviations_for_country_revision_3(b"466", "Mali", Some(b"MALI"), Some(b"MLI")),
		names_and_abbreviations_for_country_revision_3(b"470", "Malta", Some(b"MALTA"), Some(b"MLT")),
		names_and_abbreviations_for_country_revision_3(b"474", "Martinique", Some(b"MARTINIQUE"), Some(b"MTQ")),
		names_and_abbreviations_for_country_revision_3(b"478", "Mauritania", Some(b"MAURITANIA"), Some(b"MRT")),
		names_and_abbreviations_for_country_revision_3(b"480", "Mauritius", Some(b"MAURITIUS"), Some(b"MUS")),
		names_and_abbreviations_for_country_revision_3(b"484", "Mexico", Some(b"MEXICO"), Some(b"MEX")),
		names_and_abbreviations_for_country_revision_3(b"492", "Monaco", Some(b"MONACO"), Some(b"MCO")),
		names_and_abbreviations_for_country_revision_3(b"496", "Mongolia", Some(b"MONGOLIA"), Some(b"MNG")),
		names_and_abbreviations_for_country_revision_3(b"498", "Republic of Moldova", Some(b"REP MOLDOVA"), Some(b"MDA")),
		names_and_abbreviations_for_country_revision_3(b"500", "Montserrat", Some(b"MONTSERRAT"), Some(b"MSR")),
		names_and_abbreviations_for_country_revision_3(b"504", "Morocco", Some(b"MOROCCO"), Some(b"MAR")),
		names_and_abbreviations_for_country_revision_3(b"508", "Mozambique", Some(b"MOZAMBIQUE"), Some(b"MOZ")),
		names_and_abbreviations_for_country_revision_3(b"512", "Oman", Some(b"OMAN"), Some(b"OMN")),
		names_and_abbreviations_for_country_revision_3(b"516", "Namibia", Some(b"NAMIBIA"), Some(b"NAM")),
		names_and_abbreviations_for_country_revision_3(b"520", "Nauru", Some(b"NAURU"), Some(b"NRU")),
		names_and_abbreviations_for_country_revision_3(b"524", "Nepal", Some(b"NEPAL"), Some(b"NPL")),
		names_and_abbreviations_for_country_revision_3(b"528", "Netherlands", Some(b"NETHERLANDS"), Some(b"NLD")),
		names_and_abbreviations_for_country_revision_3(b"530", "Netherlands Antilles", Some(b"NETH.ANTILES"), Some(b"ANT")),
		names_and_abbreviations_for_country_revision_3(b"533", "Aruba", Some(b"ARUBA"), Some(b"ABW")),
		names_and_abbreviations_for_country_revision_3(b"540", "New Caledonia", Some(b"NEW CALEDNIA"), Some(b"NCL")),
		names_and_abbreviations_for_country_revision_3(b"548", "Vanuatu", Some(b"VANUATU"), Some(b"VUT")),
		names_and_abbreviations_for_country_revision_3(b"554", "New Zealand", Some(b"NEW ZEALAND"), Some(b"NZL")),
		names_and_abbreviations_for_country_revision_3(b"558", "Nicaragua", Some(b"NICARAGUA"), Some(b"NIC")),
		names_and_abbreviations_for_country_revision_3(b"562", "Niger", Some(b"NIGER"), Some(b"NER")),
		names_and_abbreviations_for_country_revision_3(b"566", "Nigeria", Some(b"NIGERIA"), Some(b"NGA")),
		names_and_abbreviations_for_country_revision_3(b"570", "Niue", Some(b"NIUE"), Some(b"NIU")),
		names_and_abbreviations_for_country_revision_3(b"574", "Norfolk Island", Some(b"NORFOLK IS"), Some(b"NFK")),
		names_and_abbreviations_for_country_revision_3(b"578", "Norway", Some(b"NORWAY"), Some(b"NOR")),
		names_and_abbreviations_for_country_revision_3(b"580", "Northern Mariana Islands", Some(b"N.MARIANA IS"), Some(b"MNP")),
		names_and_abbreviations_for_country_revision_3(b"583", "Micronesia, Federated States of", Some(b"MICRONESIA"), Some(b"FSM")),
		names_and_abbreviations_for_country_revision_3(b"584", "Marshall Islands", Some(b"MARSHALL IS"), Some(b"MHL")),
		names_and_abbreviations_for_country_revision_3(b"585", "Palau", Some(b"PALAU"), Some(b"PLW")),
		names_and_abbreviations_for_country_revision_3(b"586", "Pakistan", Some(b"PAKISTAN"), Some(b"PAK")),
		names_and_abbreviations_for_country_revision_3(b"591", "Panama", Some(b"PANAMA"), Some(b"PAN")),
		names_and_abbreviations_for_country_revision_3(b"598", "Papua New Guinea", Some(b"PAPUA N.GUIN"), Some(b"PNG")),
		names_and_abbreviations_for_country_revision_3(b"600", "Paraguay", Some(b"PARAGUAY"), Some(b"PRY")),
		names_and_abbreviations_for_country_revision_3(b"604", "Peru", Some(b"PERU"), Some(b"PER")),
		names_and_abbreviations_for_country_revision_3(b"608", "Philippines", Some(b"PHILIPPINES"), Some(b"PHL")),
		names_and_abbreviations_for_country_revision_3(b"612", "Pitcairn", Some(b"PITCAIRN"), Some(b"PCN")),
		names_and_abbreviations_for_country_revision_3(b"616", "Poland", Some(b"POLAND"), Some(b"POL")),
		names_and_abbreviations_for_country_revision_3(b"620", "Portugal", Some(b"PORTUGAL"), Some(b"PRT")),
		names_and_abbreviations_for_country_revision_3(b"624", "Guinea-Bissau", Some(b"GUINEABISSAU"), Some(b"GNB")),
		names_and_abbreviations_for_country_revision_3(b"626", "East Timor", Some(b"EAST TIMOR"), Some(b"TMP")),
		names_and_abbreviations_for_country_revision_3(b"630", "Puerto Rico", Some(b"PUERTO RICO"), Some(b"PRI")),
		names_and_abbreviations_for_country_revision_3(b"634", "Qatar", Some(b"QATAR"), Some(b"QAT")),
		names_and_abbreviations_for_country_revision_3(b"638", "Réunion", Some(b"REUNION"), Some(b"REU")),
		names_and_abbreviations_for_country_revision_3(b"642", "Romania", Some(b"ROMANIA"), Some(b"ROM")),
		names_and_abbreviations_for_country_revision_3(b"643", "Russian Federation", Some(b"RUSSIAN FED"), Some(b"RUS")),
		names_and_abbreviations_for_country_revision_3(b"646", "Rwanda", Some(b"RWANDA"), Some(b"RWA")),
		names_and_abbreviations_for_country_revision_3(b"654", "Saint Helena", Some(b"ST.HELENA"), Some(b"SHN")),
		names_and_abbreviations_for_country_revision_3(b"659", "Saint Kitts and Nevis", Some(b"ST.KITTS-NEV"), Some(b"KNA")),
		names_and_abbreviations_for_country_revision_3(b"660", "Anguilla", Some(b"ANGUILLA"), Some(b"AIA")),
		names_and_abbreviations_for_country_revision_3(b"662", "Saint Lucia", Some(b"ST.LUCIA"), Some(b"LCA")),
		names_and_abbreviations_for_country_revision_3(b"666", "Saint Pierre and Miquelon", Some(b"ST.PIERRE,MQ"), Some(b"SPM")),
		names_and_abbreviations_for_country_revision_3(b"670", "Saint Vincent and the Grenadines", Some(b"ST.VINCENT,G"), Some(b"VCT")),
		names_and_abbreviations_for_country_revision_3(b"674", "San Marino", Some(b"SAN MARINO"), Some(b"SMR")),
		names_and_abbreviations_for_country_revision_3(b"678", "Sao Tome and Principe", Some(b"SAO TOME PRN"), Some(b"STP")),
		names_and_abbreviations_for_country_revision_3(b"682", "Saudi Arabia", Some(b"SAUDI ARABIA"), Some(b"SAU")),
		names_and_abbreviations_for_country_revision_3(b"686", "Senegal", Some(b"SENEGAL"), Some(b"SEN")),
		names_and_abbreviations_for_country_revision_3(b"690", "Seychelles", Some(b"SEYCHELLES"), Some(b"SYC")),
		names_and_abbreviations_for_country_revision_3(b"694", "Sierra Leone", Some(b"SIERRA LEONE"), Some(b"SLE")),
		names_and_abbreviations_for_country_revision_3(b"702", "Singapore", Some(b"SINGAPORE"), Some(b"SGP")),
		names_and_abbreviations_for_country_revision_3(b"703", "Slovakia", Some(b"SLOVAKIA"), Some(b"SVK")),
		names_and_abbreviations_for_country_revision_3(b"704", "Viet Nam", Some(b"VIET NAM"), Some(b"VNM")),
		names_and_abbreviations_for_country_revision_3(b"705", "Slovenia", Some(b"SLOVENIA"), Some(b"SVN")),
		names_and_abbreviations_for_country_revision_3(b"706", "Somalia", Some(b"SOMALIA"), Some(b"SOM")),
		names_and_abbreviations_for_country_revision_3(b"710", "South Africa", Some(b"SOUTH AFRICA"), Some(b"ZAF")),
		names_and_abbreviations_for_country_revision_3(b"716", "Zimbabwe", Some(b"ZIMBABWE"), Some(b"ZWE")),
		names_and_abbreviations_for_country_revision_3(b"724", "Spain", Some(b"SPAIN"), Some(b"ESP")),
		names_and_abbreviations_for_country_revision_3(b"732", "Western Sahara", Some(b"WESTN.SAHARA"), Some(b"ESH")),
		names_and_abbreviations_for_country_revision_3(b"736", "Sudan", Some(b"SUDAN"), Some(b"SDN")),
		names_and_abbreviations_for_country_revision_3(b"740", "Suriname", Some(b"SURINAME"), Some(b"SUR")),
		names_and_abbreviations_for_country_revision_3(b"744", "Svalbard and Jan Mayen Islands", Some(b"SVALBARD IS"), Some(b"SJM")),
		names_and_abbreviations_for_country_revision_3(b"748", "Swaziland", Some(b"SWAZILAND"), Some(b"SWZ")),
		names_and_abbreviations_for_country_revision_3(b"752", "Sweden", Some(b"SWEDEN"), Some(b"SWE")),
		names_and_abbreviations_for_country_revision_3(b"756", "Switzerland", Some(b"SWITZERLAND"), Some(b"CHE")),
		names_and_abbreviations_for_country_revision_3(b"760", "Syrian Arab Republic", Some(b"SYRIA"), Some(b"SYR")),
		names_and_abbreviations_for_country_revision_3(b"762", "Tajikistan", Some(b"TAJIKISTAN"), Some(b"TJK")),
		names_and_abbreviations_for_country_revision_3(b"764", "Thailand", Some(b"THAILAND"), Some(b"THA")),
		names_and_abbreviations_for_country_revision_3(b"768", "Togo", Some(b"TOGO"), Some(b"TGO")),
		names_and_abbreviations_for_country_revision_3(b"772", "Tokelau", Some(b"TOKELAU"), Some(b"TKL")),
		names_and_abbreviations_for_country_revision_3(b"776", "Tonga", Some(b"TONGA"), Some(b"TON")),
		names_and_abbreviations_for_country_revision_3(b"780", "Trinidad and Tobago", Some(b"TRINIDAD TBG"), Some(b"TTO")),
		names_and_abbreviations_for_country_revision_3(b"784", "United Arab Emirates", Some(b"UNTD ARAB EM"), Some(b"ARE")),
		names_and_abbreviations_for_country_revision_3(b"788", "Tunisia", Some(b"TUNISIA"), Some(b"TUN")),
		names_and_abbreviations_for_country_revision_3(b"792", "Turkey", Some(b"TURKEY"), Some(b"TUR")),
		names_and_abbreviations_for_country_revision_3(b"795", "Turkmenistan", Some(b"TURKMENISTAN"), Some(b"TKM")),
		names_and_abbreviations_for_country_revision_3(b"796", "Turks and Caicos Islands", Some(b"TURKS,CAICOS"), Some(b"TCA")),
		names_and_abbreviations_for_country_revision_3(b"798", "Tuvalu", Some(b"TUVALU"), Some(b"TUV")),
		names_and_abbreviations_for_country_revision_3(b"800", "Uganda", Some(b"UGANDA"), Some(b"UGA")),
		names_and_abbreviations_for_country_revision_3(b"804", "Ukraine", Some(b"UKRAINE"), Some(b"UKR")),
		names_and_abbreviations_for_country_revision_3(b"807", "The former Yugoslav Republic of Macedonia", Some(b"TFYROM"), Some(b"MKD")),
		names_and_abbreviations_for_country_revision_3(b"818", "Egypt", Some(b"EGYPT"), Some(b"EGY")),
		names_and_abbreviations_for_country_revision_3(b"826", "United Kingdom", Some(b"UK"), Some(b"GBR")),
		names_and_abbreviations_for_country_revision_3(b"830", "Channel Islands", Some(b"CHANNEL IS"), Some(b"CHL")),
		names_and_abbreviations_for_country_revision_3(b"833", "Isle of Man", Some(b"ISLE OF MAN"), None),
		names_and_abbreviations_for_country_revision_3(b"834", "United Republic of Tanzania", Some(b"TANZANIA"), Some(b"TZA")),
		names_and_abbreviations_for_country_revision_3(b"840", "United States", Some(b"USA"), Some(b"USA")),
		names_and_abbreviations_for_country_revision_3(b"850", "United States Virgin Islands", Some(b"US.VIRGIN IS"), Some(b"VIR")),
		names_and_abbreviations_for_country_revision_3(b"854", "Burkina Faso", Some(b"BURKINA FASO"), Some(b"BFA")),
		names_and_abbreviations_for_country_revision_3(b"858", "Uruguay", Some(b"URUGUAY"), Some(b"URY")),
		names_and_abbreviations_for_country_revision_3(b"860", "Uzbekistan", Some(b"UZBEKISTAN"), Some(b"UZB")),
		names_and_abbreviations_for_country_revision_3(b"862", "Venezuela", Some(b"VENEZUELA"), Some(b"VEN")),
		names_and_abbreviations_for_country_revision_3(b"876", "Wallis and Futuna Islands", Some(b"WALLIS FUT.I"), Some(b"WLF")),
		names_and_abbreviations_for_country_revision_3(b"882", "Samoa", Some(b"SAMOA"), Some(b"WSM")),
		names_and_abbreviations_for_country_revision_3(b"887", "Yemen", Some(b"YEMEN"), Some(b"YEM")),
		names_and_abbreviations_for_country_revision_3(b"891", "Yugoslavia", Some(b"YUGOSLAVIA"), Some(b"YUG")),
		names_and_abbreviations_for_country_revision_3(b"894", "Zambia", Some(b"ZAMBIA"), Some(b"ZMB")),
		names_and_abbreviations_for_country_revision_3(b"896", "Areas not elsewhere specified", Some(b"N.E.S."), None),
		names_and_abbreviations_for_country_revision_3(b"898", "Areas not specified", Some(b"N.S."), None),
	]
};
