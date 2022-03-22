// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::a_to_z::Hyphen;
use arrayvec::ArrayVec;
use memchr::memchr;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::MaybeUninit;
use super::IanaRegisteredUnM49RegionCode;
use super::Variant;
use super::restricted_byte::InvalidAlphaError;
use super::restricted_byte::InvalidAlphanumericError;
use super::restricted_byte::InvalidDigitError;
use super::restricted_byte::InvalidUpperCaseAlphaError;
use super::restricted_byte::RestrictedByte;
use super::restricted_byte::Singleton;
use swiss_army_knife::get_unchecked::GetUnchecked;


#[macro_use]
pub(in crate::bcp_47_language_tag) mod macros;


include!("array_vec_u8.rs");
include!("Bcp47LanguageTagParseError.rs");
include!("ExtensionParseError.rs");
include!("GrandfatheredIrregularISubtagParseError.rs");
include!("InvalidSubtagLengthError.rs");
include!("LanguageExtensionSubtagParseError.rs");
include!("LanguageSubtagParseError.rs");
include!("MemchrIterator.rs");
include!("NextSubtagAfterLanguageExtension.rs");
include!("PrivateUseSubtagsParseError.rs");
include!("RegionParseError.rs");
include!("ScriptParseError.rs");
include!("Subtag.rs");
include!("Subtags.rs");
include!("UninitialisedArray.rs");
include!("VariantParseError.rs");
