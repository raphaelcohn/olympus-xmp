// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::a_to_z::_0;
use crate::a_to_z::_9;
use crate::a_to_z::a;
use crate::a_to_z::w;
use crate::a_to_z::x;
use crate::a_to_z::y;
use crate::a_to_z::z;
use crate::a_to_z::A;
use crate::a_to_z::W;
use crate::a_to_z::X;
use crate::a_to_z::Y;
use crate::a_to_z::Z;
use crate::a_to_z::Hyphen;
use arrayvec::ArrayVec;
use either::Left;
use either::Right;
use memchr::memchr;
use restricted_byte::InvalidAlphaError;
use restricted_byte::InvalidAlphanumericError;
use restricted_byte::InvalidDigitError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::Bcp47LanguageTag;
use super::Extension;
use super::ExtensionPortion;
use super::IanaRegisteredIso15924ScriptCode;
use super::IanaRegisteredRegionCode;
use super::IanaRegisteredUnM49RegionCode;
use super::Language;
use super::Normal;
use super::OneWithOptionalSuffixes;
use super::OrdinaryLanguage;
use super::PrivateUse;
use super::PrivateUsePortion;
use super::RegisteredLanguageSubtag;
use super::ReservedLanguageSubtag;
use super::Variant;
use super::Bcp47LanguageTagParseError::FirstSubtag;
use super::restricted_byte::Alpha;
use super::restricted_byte::Alphanumeric;
use super::restricted_byte::Digit;
use super::restricted_byte::InvalidAlphaError;
use super::restricted_byte::InvalidAlphanumericError;
use super::restricted_byte::InvalidDigitError;
use super::restricted_byte::InvalidSingletonError;
use super::restricted_byte::InvalidUpperCaseAlphaError;
use super::restricted_byte::RestrictedByte;
use super::restricted_byte::Singleton;
use super::restricted_byte::UpperCaseAlpha;
use restricted_byte::InvalidUpperCaseAlphaError;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::unreachable_code;


#[macro_use]
pub(in crate::bcp_47_language_tag) mod macros;


include!("array_vec_u8.rs");
include!("Bcp47LanguageTagParseError.rs");
include!("ExtensionParseError.rs");
include!("GrandfatheredIrregularISubtagParseError.rs");
include!("LanguageExtensionSubtagParseError.rs");
include!("LanguageFirstSubtagParseError.rs");
include!("MemchrIterator.rs");
include!("NextSubtag.rs");
include!("PrivateUseSubtagsParseError.rs");
include!("RegionParseError.rs");
include!("ScriptParseError.rs");
include!("UninitialisedArray.rs");
include!("VariantParseError.rs");
