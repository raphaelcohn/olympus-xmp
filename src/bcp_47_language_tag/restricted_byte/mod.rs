// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::a_to_z::a;
use crate::a_to_z::w;
use crate::a_to_z::y;
use crate::a_to_z::z;
use crate::a_to_z::A;
use crate::a_to_z::Z;
use crate::a_to_z::_0;
use crate::a_to_z::_9;
use std::error;
use std::fmt::Debug;
use std::mem::transmute_copy;
use super::parser::restricted_byte::InvalidAlphaError;
use super::parser::restricted_byte::InvalidAlphanumericError;
use super::parser::restricted_byte::InvalidDigitError;
use super::parser::restricted_byte::InvalidSingletonError;
use super::parser::restricted_byte::UninitialisedArray;
use super::parser::restricted_byte::to_lower_case;
use swiss_army_knife::get_unchecked::AsUsizeIndex;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("Alpha.rs");
include!("Alphanumeric.rs");
include!("Digit.rs");
include!("RestrictedByte.rs");
include!("Singleton.rs");
