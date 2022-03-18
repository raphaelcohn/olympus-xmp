// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::a_to_z::Hyphen;
use crate::a_to_z::_0;
use crate::a_to_z::_9;
use crate::a_to_z::A;
use crate::a_to_z::Z;
use crate::a_to_z::a;
use crate::a_to_z::b;
use crate::a_to_z::h;
use crate::a_to_z::i;
use crate::a_to_z::p;
use crate::a_to_z::t;
use crate::a_to_z::o;
use crate::a_to_z::y;
use crate::a_to_z::s;
use crate::a_to_z::d;
use crate::a_to_z::k;
use crate::a_to_z::u;
use crate::a_to_z::z;
use arrayvec::ArrayVec;
use either::Either;
use either::Left;
use either::Right;
use swiss_army_knife::get_unchecked::GetUnchecked;
use parser::array_vec_u8;
use parser::Bcp47LanguageTagParseError;
use parser::GrandfatheredIrregularISubtagParseError;
use parser::LanguageExtensionTagParseError;
use parser::LanguageFirstSubtagParseError;
use parser::LanguageFirstSubtagParseError::FirstSubtagLengthIsTwoToEightButInvalidAlpha;
use parser::parse_bcp47_language_tag;
use parser::PrivateUseSubtagsParseError;
use parser::restricted_byte::InvalidAlphaError;
use parser::restricted_byte::InvalidDigitError;
use parser::restricted_byte::UninitialisedArray;
use parser::restricted_byte::to_lower_case;
use parser::MemchrIterator;
use restricted_byte::Digit;
use restricted_byte::Alpha;
use restricted_byte::Alphanumeric;
use restricted_byte::RestrictedByte;


#[macro_use]
mod parser;


mod restricted_byte;


include!("Bcp47LanguageTag.rs");
include!("Grandfathered.rs");
include!("IanaRegisteredIso639Code.rs");
include!("IanaRegisteredIso639Alpha2Code.rs");
include!("IanaRegisteredIso639Alpha3Code.rs");
include!("IanaRegisteredUnM49RegionCode.rs");
include!("IrregularGrandfathered.rs");
include!("Language.rs");
include!("LanguageExtension.rs");
include!("Normal.rs");
include!("OneWithOptionalSuffixes.rs");
include!("PrivateUse.rs");
include!("PrivateUsePortion.rs");
include!("ReservedLanguageSubtag.rs");
include!("RegisteredLanguageSubtag.rs");
include!("RegularGrandfathered.rs");
