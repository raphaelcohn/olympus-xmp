// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(invalid_html_tags)]
#![deny(macro_use_extern_crate)]
#![deny(missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(pointer_structural_match)]
#![deny(unaligned_references)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![deny(unused_must_use)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unreachable_code)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_crate_dependencies)]


#![feature(adt_const_params)]
#![feature(const_convert)]
#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]


//! #ietf-bcp-47-language-tag
//!
//! An IETF BCP 47 language tag parser.


use arrayvec::ArrayVec;
use either::Either;
use either::Left;
use either::Right;
use parser::array_vec_u8;
use parser::IetfBcp47LanguageTagParseError;
use parser::ExtensionParseError;
use parser::GrandfatheredIrregularISubtagParseError;
use parser::InvalidSubtagLengthError;
use parser::LanguageExtensionSubtagParseError;
use parser::LanguageSubtagParseError;
use parser::MemchrIterator;
use parser::NextSubtagAfterLanguageExtension;
use parser::PrivateUseSubtagsParseError;
use parser::RegionParseError;
use parser::ScriptParseError;
use parser::Subtag;
use parser::Subtags;
use parser::UninitialisedArray;
use parser::VariantParseError;
use restricted_byte::Alpha;
use restricted_byte::Alphanumeric;
use restricted_byte::Digit;
use restricted_byte::RestrictedByte;
use restricted_byte::RestrictedByteConst;
use restricted_byte::Singleton;
use restricted_byte::to_lower_case;
use restricted_byte::to_upper_case;
use restricted_byte::UpperCaseAlpha;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::mem::transmute;
use std::mem::transmute_copy;
use swiss_army_knife::unreachable_code;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::W;
use swiss_army_knife::a_to_z::Y;
use swiss_army_knife::a_to_z::X;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::b;
use swiss_army_knife::a_to_z::h;
use swiss_army_knife::a_to_z::i;
use swiss_army_knife::a_to_z::o;
use swiss_army_knife::a_to_z::p;
use swiss_army_knife::a_to_z::t;
use swiss_army_knife::a_to_z::s;
use swiss_army_knife::a_to_z::d;
use swiss_army_knife::a_to_z::k;
use swiss_army_knife::a_to_z::u;
use swiss_army_knife::a_to_z::w;
use swiss_army_knife::a_to_z::x;
use swiss_army_knife::a_to_z::y;
use swiss_army_knife::a_to_z::z;
use swiss_army_knife::get_unchecked::GetUnchecked;
use unroll::unroll_for_loops;


#[macro_use]
mod parser;


/// A byte with a restricted range of valid values.
pub mod restricted_byte;


include!("Grandfathered.rs");
include!("Extension.rs");
include!("ExtensionPortion.rs");
include!("IanaRegisteredIso3166Dash1Alpha2CountryCode.rs");
include!("IanaRegisteredRegionCode.rs");
include!("IanaRegisteredIso639Code.rs");
include!("IanaRegisteredIso639Alpha2Code.rs");
include!("IanaRegisteredIso639Alpha3Code.rs");
include!("IanaRegisteredUnM49RegionCode.rs");
include!("IanaRegisteredIso15924ScriptCode.rs");
include!("IetfBcp47LanguageTag.rs");
include!("IrregularGrandfathered.rs");
include!("Language.rs");
include!("LanguageExtension.rs");
include!("Normal.rs");
include!("OrdinaryLanguage.rs");
include!("OneWithOptionalSuffixes.rs");
include!("PrivateUse.rs");
include!("PrivateUsePortion.rs");
include!("ReservedLanguageSubtag.rs");
include!("RegisteredLanguageSubtag.rs");
include!("RegularGrandfathered.rs");
include!("Variant.rs");
