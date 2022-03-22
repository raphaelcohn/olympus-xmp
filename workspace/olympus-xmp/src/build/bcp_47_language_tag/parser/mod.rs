// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::a_to_z::_0;
use crate::a_to_z::_9;
use crate::a_to_z::a;
use crate::a_to_z::A;
use crate::a_to_z::z;
use crate::a_to_z::Z;
use crate::a_to_z::Colon;
use crate::a_to_z::Hyphen;
use crate::a_to_z::LineFeed;
use crate::a_to_z::Percent;
use crate::a_to_z::Space;
use memchr::memchr;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::from_utf8_unchecked;
use swiss_army_knife::get_unchecked::AsUsizeRange;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("Event.rs");
include!("FieldEvent.rs");
include!("PullEventParser.rs");
include!("PullEventParserError.rs");
include!("UnfoldedLinesIterator.rs");