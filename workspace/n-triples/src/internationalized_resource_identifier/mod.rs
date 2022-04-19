// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use internet_protocol_version_4_address::InternetProtocolVersion4AddressParseError;
use internet_protocol_version_4_address::InternetProtocolVersion4AddressParser;
use memchr::memchr;
use memchr::memrchr;
use memchr::memchr3;
use path::NonEmptyPath;
use path::NonEmptyPathSegment;
use path::NonEmptyPathParseError;
use path::PathSegment;
use path::PathSegments;
use path::PathSegmentsParseError;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::cmp::min;
use std::collections::TryReserveError;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::forget;
use std::mem::transmute;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::slice::from_raw_parts;
use std::str::FromStr;
use std::str::from_utf8_unchecked;
use super::PlusSign;
use super::MinusSign;
use super::Hash;
use super::QuestionMark;
use super::AtSign;
use super::CloseSquareBracket;
use super::OpenSquareBracket;
use super::CloseAngleBracketChar;
use super::AChar;
use super::ZChar;
use super::aChar;
use super::zChar;
use super::ColonChar;
use super::HashChar;
use super::HyphenChar;
use super::PeriodChar;
use super::QuestionMarkChar;
use super::SlashChar;
use super::TildeChar;
use super::UnderscoreChar;
use super::x00;
use super::x20;
use super::FromUnchecked;
use super::parser::get_0;
use super::parser::GetUncheckedExt;
use super::parser::InvalidUtf8ParseError;
use super::parser::OutOfMemoryOrInvalidUtf8PercentDecodeParseError;
use super::parser::OutOfMemoryOrUCHARParseError;
use super::parser::PercentDecodeError;
use super::parser::StringSoFar;
use super::parser::utf8::decode_next_percent_encoded_utf8;
use super::parser::utf8::decode_next_utf8;
use super::parser::utf8::Utf8CharacterLength;
use super::predicate::MoreThanOneError;
use super::try_to_own::TryToOwn;
use super::try_to_own::TryToOwnInPlace;
use swiss_army_knife::unreachable_code_const;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::a_to_z::U;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::u;
use swiss_army_knife::a_to_z::v;
use swiss_army_knife::a_to_z::z;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::Colon;
use swiss_army_knife::a_to_z::Period;
use swiss_army_knife::a_to_z::Slash;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_u16;
use swiss_army_knife::non_zero::new_non_zero_u32;
use swiss_army_knife::non_zero::new_non_zero_usize;


#[macro_use]
mod macros;


/// Internet Protocol Version 4 address support.
pub mod internet_protocol_version_4_address;


/// Hierarchy path.
pub mod path;


include!("Authority.rs");
include!("AuthorityParseError.rs");
include!("AbsoluteInternationalizedResourceIdentifierComponentsParseError.rs");
include!("AbsoluteInternationalizedResourceIdentifierParseError.rs");
include!("AbsoluteInternationalizedResourceIdentifier.rs");
include!("AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError.rs");
include!("HashFragment.rs");
include!("HashFragmentParseError.rs");
include!("Hierarchy.rs");
include!("HierarchyParseError.rs");
include!("Host.rs");
include!("HostName.rs");
include!("HostNameParseError.rs");
include!("HostParseError.rs");
include!("ParseNextAfterHierarchy.rs");
include!("PortParseError.rs");
include!("RemovePrefixError.rs");
include!("Query.rs");
include!("QueryParseError.rs");
include!("Scheme.rs");
include!("SchemeParseError.rs");
include!("UserInformation.rs");
include!("UserInformationParseError.rs");
include!("WithPathSegmentError.rs");
