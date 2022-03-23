// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use unroll::unroll_for_loops;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use super::super::XmpAttributeValue;
use super::super::XmpAttributeValueParseError;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::F;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::f;
use swiss_army_knife::a_to_z::Hyphen;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::unreachable_code_const;


include!("as_u8_array.rs");
include!("Variant.rs");
include!("VariantParseError.rs");
include!("Version.rs");
include!("VersionParseError.rs");
include!("XmpInstanceIdentifier.rs");
include!("XmpInstanceIdentifierParseError.rs");
include!("XmpUniversallyUniqueIdentifier.rs");
include!("XmpUniversallyUniqueIdentifierParseError.rs");
