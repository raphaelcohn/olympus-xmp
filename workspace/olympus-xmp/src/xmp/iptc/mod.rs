// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use arrayvec::ArrayVec;
use iso_3166_1_country::UnknownStringVariantParseError;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::C;
use swiss_army_knife::a_to_z::E;
use swiss_army_knife::a_to_z::H;
use swiss_army_knife::a_to_z::L;
use swiss_army_knife::a_to_z::P;
use swiss_army_knife::a_to_z::R;
use swiss_army_knife::a_to_z::S;
use swiss_army_knife::a_to_z::W;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("Choice.rs");
include!("IimCategoryCode.rs");
include!("IimCategoryCodeParseError.rs");
include!("IimSupplementalCategories.rs");
include!("IimSupplementalCategoriesParseError.rs");
include!("IptcDigitalSourceType.rs");
include!("IptcWorldRegion.rs");


/// Legacy urgency support.
pub mod urgency;
