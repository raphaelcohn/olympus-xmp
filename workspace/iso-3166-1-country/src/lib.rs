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
#![feature(const_trait_impl)]
#![feature(generic_arg_infer)]


//! #iso-3166-1-country
//! 
//! ISO country and country code domain types.


use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::str::FromStr;
use std::str::from_utf8_unchecked;
use Iso3166Dash1Alpha2CountryCode::*;
use Iso3166Dash1Alpha3CountryCode::*;
use Iso3166Dash1AlphaCountryCode::*;
use Iso3166Dash1NumericCountryCode::*;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::memchr::MemoryCharacter;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("letter_to_number_scaled.rs");
include!("letter_to_number_unchecked.rs");
include!("Iso3166Dash1Country.rs");
include!("Iso3166Dash1Alpha2CountryCode.rs");
include!("Iso3166Dash1Alpha3CountryCode.rs");
include!("Iso3166Dash1NumericCountryCode.rs");
include!("Iso3166Dash1AlphaCountryCode.rs");
include!("Iso3166Dash1AlphaCountryCodeParseError.rs");
include!("UnknownStringVariantParseError.rs");
include!("UnknownIso3166Dash1CodeError.rs");
