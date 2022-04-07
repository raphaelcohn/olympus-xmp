// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use internationalized_resource_identifier::AbsoluteInternationalizedResourceIdentifier;
use memchr::memchr;
use memchr::memchr2;
use std::borrow::Cow;
use std::char::CharTryFromError;
use std::collections::TryReserveError;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::mem::size_of;
use std::num::NonZeroU8;
use std::ptr::NonNull;
use std::slice::from_raw_parts;
use std::str::from_utf8;
use std::str::from_utf8_unchecked;
use std::str::Utf8Error;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::F;
use swiss_army_knife::a_to_z::U;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::b;
use swiss_army_knife::a_to_z::f;
use swiss_army_knife::a_to_z::n;
use swiss_army_knife::a_to_z::r;
use swiss_army_knife::a_to_z::t;
use swiss_army_knife::a_to_z::u;
use swiss_army_knife::a_to_z::Period;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::vec::VecExt;
use swiss_army_knife::a_to_z::LineFeed;
use swiss_army_knife::a_to_z::Space;
use swiss_army_knife::non_zero::new_non_null;
use super::BlankNodeLabel;
use super::AbsoluteInternationalizedResourceIdentifier;
use super::Predicate;
use super::RawIetfBcp47LanguageTag;
use super::Subject;
use super::Hash;
use super::Tab;
use super::DoubleQuote;
use super::OpenAngleBracket;
use super::Underscore;
use super::Caret;
use super::AtSign;
use super::x7F;
use utf8::decode_next_percent_encoded_utf8;
use utf8::decode_next_utf8;
use utf8::encode_utf8_not_reserving_space;
use utf8::encode_utf8_push_unchecked;
use utf8::encode_utf8_reserving_space;
use utf8::encode_utf8_bytes_1;
use utf8::encode_utf8_bytes_2;
use utf8::encode_utf8_bytes_3;
use utf8::encode_utf8_bytes_4;
use utf8::Utf8CharacterLength;


pub(super) mod utf8;


include!("BlankNodeLabelParseError.rs");
include!("CommentParseError.rs");
include!("get_0.rs");
include!("GetUncheckedExt.rs");
include!("InvalidUtf8ParseError.rs");
include!("LiteralTag.rs");
include!("NTriple.rs");
include!("NTripleParseError.rs");
include!("NTriplesParseError.rs");
include!("Object.rs");
include!("ObjectParseError.rs");
include!("OutOfMemoryOrInvalidUtf8PercentDecodeParseError.rs");
include!("OutOfMemoryOrUCHARParseError.rs");
include!("PercentDecodeError.rs");
include!("PeriodParseError.rs");
include!("PredicateParseError.rs");
include!("StringLiteral.rs");
include!("StringLiteralParseError.rs");
include!("StringSoFar.rs");
include!("SubjectParseError.rs");
include!("UCHARParseError.rs");
include!("UCHARParser.rs");
