// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::FromUnchecked;
use crate::Hash;
use crate::QuestionMark;
use crate::try_to_own::TryToOwn;
use crate::try_to_own::TryToOwnInPlace;
use memchr::memchr3;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::TryReserveError;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::read;
use super::HierarchyParseError;
use super::Hierarchy;
use super::ParseNextAfterHierarchy;
use crate::parser::GetUncheckedExt;
use crate::parser::OutOfMemoryOrInvalidUtf8PercentDecodeParseError;
use crate::parser::StringSoFar;
use crate::parser::utf8::Utf8CharacterLength;
use swiss_army_knife::a_to_z::Slash;
use swiss_army_knife::const_small_vec::ConstSmallVec;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::unreachable_code_const;


include!("NonEmptyPath.rs");
include!("NonEmptyPathParseError.rs");
include!("NonEmptyPathParseState.rs");
include!("NonEmptyPathSegment.rs");
include!("PathSegment.rs");
include!("PathSegmentParseError.rs");
include!("PathSegments.rs");
include!("PathSegmentsParseError.rs");
