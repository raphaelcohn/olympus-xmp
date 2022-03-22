// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute_copy;
use super::parser::UninitialisedArray;
use super::parser::VariantParseError;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::w;
use swiss_army_knife::a_to_z::y;
use swiss_army_knife::a_to_z::z;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::W;
use swiss_army_knife::a_to_z::Y;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::get_unchecked::AsUsizeIndex;
use swiss_army_knife::get_unchecked::GetUnchecked;
use unroll::unroll_for_loops;


include!("Alpha.rs");
include!("Alphanumeric.rs");
include!("Digit.rs");
include!("RestrictedByte.rs");
include!("RestrictedByteConst.rs");
include!("Singleton.rs");
include!("UpperCaseAlpha.rs");
include!("InvalidAlphaError.rs");
include!("InvalidAlphanumericError.rs");
include!("InvalidDigitError.rs");
include!("InvalidSingletonError.rs");
include!("InvalidUpperCaseAlphaError.rs");
include!("new_array_unchecked.rs");
include!("SetForLowerCaseBit.rs");
include!("to_lower_case.rs");
include!("to_upper_case.rs");
