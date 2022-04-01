// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(uncommon_codepoints)]
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


#![feature(const_trait_impl)]


//! #un-series-m-nº49-data
//!
//! Data.


use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;


// mod abbreviations;
//
//
// mod pdf_extracts;


include!("include_generated_file.rs");


#[inline(always)]
const fn comtrade(m49_code: StaticM49Code, english_name: StaticEnglishName) -> (M49Code, StaticEnglishName)
{
	(M49Code::from(m49_code), english_name)
}

include_generated_file!("comtrade.rs");


// include!("Developing.rs");
// include!("Iso3166Dash1Alpha2Code.rs");
// include!("Iso3166Dash1Alpha3Code.rs");
// include!("Iso3166Dash1AlphaCode.rs");
include!("M49Code.rs");
// include!("NamesInEnglishAndFrench.rs");
// include!("NamesInUnitedNationsOfficialLanguages.rs");
// include!("StaticArabicName.rs");
// include!("StaticChineseName.rs");
// include!("StaticConstituents.rs");
include!("StaticEnglishName.rs");
// include!("StaticFrenchName.rs");
// include!("StaticIso3166Dash1Alpha2Code.rs");
// include!("StaticIso3166Dash1Alpha3Code.rs");
include!("StaticM49Code.rs");
include!("StaticName.rs");
// include!("StaticRussianName.rs");
// include!("StaticSpanishName.rs");
// include!("UnitedNationsOfficialLanguage.rs");
