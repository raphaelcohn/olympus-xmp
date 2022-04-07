// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use memchr::memchr2;
use memchr::memchr3;
use std::borrow::Cow;
use std::collections::TryReserveError;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::str::from_utf8_unchecked;
use swiss_army_knife::unreachable_code_const;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::z;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::Period;
use swiss_army_knife::a_to_z::Slash;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::vec::VecExt;
use super::PlusSign;
use super::MinusSign;
use super::Hash;
use super::QuestionMark;
use super::AChar;
use super::ZChar;
use super::aChar;
use super::zChar;
use super::_0Char;
use super::_9Char;
use super::AtSignChar;
use super::ColonChar;
use super::HashChar;
use super::HyphenChar;
use super::PercentChar;
use super::PeriodChar;
use super::QuestionMarkChar;
use super::SlashChar;
use super::TildeChar;
use super::UnderscoreChar;
use super::x00;
use super::x20;
use super::xA0;
use super::x07FF;
use super::x0800;
use super::xD7FF;
use super::xF900;
use super::xFDCF;
use super::xFDF0;
use super::xFFEF;
use super::x10000;
use super::x1FFFD;
use super::x20000;
use super::x2FFFD;
use super::x30000;
use super::x3FFFD;
use super::x40000;
use super::x4FFFD;
use super::x50000;
use super::x5FFFD;
use super::x60000;
use super::x6FFFD;
use super::x70000;
use super::x7FFFD;
use super::x80000;
use super::x8FFFD;
use super::x90000;
use super::x9FFFD;
use super::xA0000;
use super::xAFFFD;
use super::xB0000;
use super::xBFFFD;
use super::xC0000;
use super::xCFFFD;
use super::xD0000;
use super::xDFFFD;
use super::xE1000;
use super::xEFFFD;
use super::parser::decode_next_utf8_validity_already_checked;
use super::parser::AbsoluteInternationalizedResourceIdentifierComponentsParseError;
use super::parser::AbsoluteInternationalizedResourceIdentifierParseError;
use super::parser::get_0;
use super::parser::GetUncheckedExt;
use super::parser::IHierPartParseError;
use super::parser::InvalidUtf8ParseError;
use super::parser::OutOfMemoryOrInvalidUtf8PercentDecodeParseError;
use super::parser::OutOfMemoryOrUCHARParseError;
use super::parser::SchemeParseError;
use super::parser::StringSoFar;
use super::parser::utf8::decode_next_percent_encoded_utf8;
use super::parser::utf8::decode_next_utf8;
use super::parser::utf8::decode_next_utf8_validity_already_checked;
use super::parser::utf8::PercentDecodeError;
use super::parser::utf8::Utf8CharacterLength;
use super::try_to_own::try_to_own_in_place_cow;
use super::try_to_own::TryToOwn;
use super::try_to_own::TryToOwnInPlace;


include!("ALPHA.rs");
include!("DIGIT.rs");
include!("ipchar_iunreserved_without_ucschar.rs");
include!("ipchar_iunreserved_ucschar_2.rs");
include!("ipchar_iunreserved_ucschar_3.rs");
include!("ipchar_iunreserved_ucschar_4.rs");
include!("ipchar_other.rs");
include!("ipchar_pct_encoded.rs");
include!("ipchar_sub_delims.rs");
include!("pct_encoded.rs");
include!("sub_delims.rs");


include!("Authority.rs");
include!("AbsoluteInternationalizedResourceIdentifierComponentsParseError.rs");
include!("AbsoluteInternationalizedResourceIdentifierParseError.rs");
include!("AbsoluteInternationalizedResourceIdentifier.rs");
include!("Hierarchy.rs");
include!("HierarchyParseError.rs");
include!("NonEmptyPath.rs");
include!("NonEmptyPathParseState.rs");
include!("NonEmptyPathSegment.rs");
include!("ParseNext.rs");
include!("PathSegment.rs");
include!("PathSegments.rs");
include!("Scheme.rs");
include!("SchemeParseError.rs");