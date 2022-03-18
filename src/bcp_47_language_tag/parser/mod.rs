// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::a_to_z::i;
use crate::a_to_z::x;
use arrayvec::ArrayVec;
use memchr::memchr;
use restricted_byte::InvalidAlphaError;
use restricted_byte::InvalidAlphanumericError;
use restricted_byte::InvalidDigitError;
use restricted_byte::to_lower_case;
use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use super::Bcp47LanguageTag;
use super::IanaRegisteredIso15924ScriptCode;
use super::IanaRegisteredRegionCode;
use super::IanaRegisteredUnM49RegionCode;
use super::IrregularGrandfathered;
use super::Language;
use super::OrdinaryLanguage;
use super::RegisteredLanguageSubtag;
use super::ReservedLanguageSubtag;
use swiss_army_knife::get_unchecked::GetUnchecked;



include!("finished.rs");
include!("next_mandatory.rs");


pub(super) mod restricted_byte;

include!("array_vec_u8.rs");
include!("Bcp47LanguageTagParseError.rs");
include!("GrandfatheredIrregularISubtagParseError.rs");
include!("LanguageExtensionSubtagParseError.rs");
include!("LanguageFirstSubtagParseError.rs");
include!("MemchrIterator.rs");
include!("NextSubtag.rs");
include!("parse_bcp47_language_tag.rs");
include!("PrivateUseSubtagsParseError.rs");
