// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use ExifVersionField::_0;
use ExifVersionField::_1;
use ExifVersionField::_2;
use ExifVersionField::_3;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("ExifVersion.rs");
include!("ExifVersionParseError.rs");
include!("ExifVersionField.rs");
include!("ExifVersionFieldParseError.rs");
