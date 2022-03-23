// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use iso_3166_1_country::UnknownStringVariantParseError;
use std::mem::transmute;
use std::str::FromStr;
use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;


/// Date (and time) domain types.
pub mod date_time;


/// Universally Unique Identifiers (UUID) domain types.
pub mod universally_unique_identifier;


include!("XmpLabel.rs");
include!("XmpRating.rs");
