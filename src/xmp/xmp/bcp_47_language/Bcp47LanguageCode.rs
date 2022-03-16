// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use arrayvec::ArrayVec;
use super::super::iso_country::Iso3166Dash1Alpha2CountryCode;
use super::super::iso_country::Iso3166Dash1NumericCountryCode;

/*
	Parsing:
		split by hyphen.
		parse tags as appropriate.
		first tag is always language.
		will need to match grandfathered exceptions as they occur.
	
	https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
	
	The subtags in the range 'qaa' through 'qtz' are reserved for private use in language tags.  These subtags correspond to codes reserved by ISO 639-2 for private use.
	These codes MAY be used for non-registered primary language subtags (instead of using private use subtags following 'x-').  Please refer to Section 4.6 for more information on private use subtags.

	The script subtags 'Qaaa' through 'Qabx' are reserved for private use in language tags.  These subtags correspond to codes reserved by ISO 15924 for private use.
	These codes MAY be used for non-registered script values.  Please refer to Section 4.6 for more information on private use subtags.
	
	ISO 3166-1 A
	
	
	
	
	
	.lpha 2 codes are a slight subset.
	M.49 codes are a subset.
 */
/// As defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt).
/// And RFC 5646.
/// <https://en.wikipedia.org/wiki/IETF_language_tag> is helpful.
/// <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry> is official.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Bcp47LanguageCode
{
	#[allow(missing_docs)]
	Normal(Normal),
	
	#[allow(missing_docs)]
	PrivateUse(PrivateUse),

	#[allow(missing_docs)]
	Grandfathered(Grandfathered),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Normal
{
	language: Language,
	
	/// Presence detected as consists of 4 ALPHA characters, whereas region consists of 2 ALPHA or 3 DIGIT.
	script: Option<Iso15924ScriptCode>,
	
	/// Presence detected as consists of 2 ALPHA or 3 DIGIT.
	region: Option<Iso3166Dash1AlphaCountryCodeOrM49RegionCode>,
	
	/// Presence detected as consists of 5 - 8 ALPHA or starts with one DIGIT.
	variant: IndexSet<Variant>,

	/// Presence detected as initially consists of 1 SINGLETON.
	extension: IndexMap<Singleton, OneWithOptionalSuffixes<PrivateUsePortion>>,
	
	/// Presence detected because starts with `x`, whereas `extension` starts with anything but `x` or `X`.
	/// Could be modelled by re-using the `extension` field and changing the key type from `Singleton` to `Alpha`.
	private_use: Option<PrivateUse>,
}

pub enum Iso3166Dash1AlphaCountryCodeOrM49RegionCode
{
	#[allow(missing_docs)]
	Alpha2(Iso3166Dash1Alpha2CountryCode),
	
	/// Standard Country or Area Codes for Statistical Use (Series M, No. 49).
	/// TODO: See <https://en.wikipedia.org/wiki/UN_M49>.
	#[allow(missing_docs)]
	M49RegionCode(u16),
}

/// Where there are 2 and 3 character ISO 639 codes, only the 2 character code will be in the IANA registry unless the 2 character code is defined by ISO 639 after the 3 character code.
/// In other words, codes will be unique.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IanaRegisteredIso639Code
{
	#[allow(missing_docs)]
	Alpha2(Iso639Alpha2Code),
	
	#[allow(missing_docs)]
	Alpha3(Iso639Alpha3Code),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Iso639Alpha2Code([Alpha; 2]);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Iso639Alpha3Code([Alpha; 3]);

/// Basically, a 2 - 8 byte alpha code.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Language
{
	#[allow(missing_docs)]
	Iso
	{
		#[allow(missing_docs)]
		shortest_iso_639_code: IanaRegisteredIso639Code,
		
		#[allow(missing_docs)]
		extension: Option<LanguageExtension>,
	},

	/// Reserved for future use.
	Alpha4([Alpha; 4]),
	
	#[allow(missing_docs)]
	RegisteredLanguageSubTag(RegisteredLanguageSubTag),
}

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RegisteredLanguageSubTag
{
	Alpha5([Alpha; 5]),
	
	Alpha6([Alpha; 6]),
	
	Alpha7([Alpha; 7]),
	
	Alpha8([Alpha; 8]),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LanguageExtension
{
	/// Selected ISO 639 codes.
	alpha3: Iso639Alpha3Code,
	
	/// Will always be empty.
	permanently_reserved: ArrayVec<Iso639Alpha3Code, 2>,
}

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Iso15924ScriptCode([Alpha; 4]);

/// Must be registered `5*8alphanum / DIGIT 3alphanum`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Variant
{
	#[allow(missing_docs)]
	Alphanumeric5([Alphanumeric; 5]),
	
	#[allow(missing_docs)]
	Alphanumeric6([Alphanumeric; 6]),
	
	#[allow(missing_docs)]
	Alphanumeric7([Alphanumeric; 7]),
	
	#[allow(missing_docs)]
	Alphanumeric8([Alphanumeric; 8]),
	
	#[allow(missing_docs)]
	DigitAlphanumeric3(Digit, [Alphanumeric; 3])
}
