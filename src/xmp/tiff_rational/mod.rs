// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::a_to_z::Slash;
use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;
use gcd::binary_u32;
use gcd::binary_u64;
use memchr::memchr;
use std::cmp::Ordering;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::num::NonZeroU8;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ops::Mul;
use std::str::from_utf8_unchecked;
use std::str::FromStr;
use swiss_army_knife::get_unchecked::AsUsizeRange;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_u32;


include!("NonZeroUnsignedTiffRational.rs");
include!("NonZeroTiffRationalParseError.rs");
include!("UnsignedTiffRational.rs");
include!("UnsignedTiffRationalParseError.rs");
