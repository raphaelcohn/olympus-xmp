// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Normal
{
	language: Language,
	
	/// Presence detected as consists of 4 ALPHA characters, whereas region consists of 2 ALPHA or 3 DIGIT.
	script: Option<Iso15924ScriptCode>,
	
	/// Presence detected as consists of 2 ALPHA or 3 DIGIT.
	region: Option<IanaRegisteredIso3166Dash1AlphaCountryCodeOrIanaRegisteredUnM49RegionCode>,
	
	/// Presence detected as consists of 5 - 8 ALPHA or starts with one DIGIT.
	variant: IndexSet<Variant>,

	/// Presence detected as initially consists of 1 SINGLETON.
	extension: IndexMap<Singleton, OneWithOptionalSuffixes<PrivateUsePortion>>,
	
	/// Presence detected because starts with `x`, whereas `extension` starts with anything but `x` or `X`.
	/// Could be modelled by re-using the `extension` field and changing the key type from `Singleton` to `Alpha`.
	private_use: Option<PrivateUse>,
}

pub enum IanaRegisteredIso3166Dash1AlphaCountryCodeOrIanaRegisteredUnM49RegionCode
{
	#[allow(missing_docs)]
	Alpha2([u8; Alpha2]),
	
	/// Standard Country or Area Codes for Statistical Use (Series M, No. 49).
	/// TODO: See <https://en.wikipedia.org/wiki/UN_M49>.
	#[allow(missing_docs)]
	M49RegionCode([u8; 3]),
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
