// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use crate::internationalized_resource_identifier::MoreThanOneError;
pub use chrono::DateTime;
pub use chrono::FixedOffset;
pub use chrono::format::ParseError as ParseDateTimeError;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::TryReserveError;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::iter::FusedIterator;
use std::iter::TrustedLen;
use std::marker::PhantomData;
use std::mem::transmute;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;
use std::str::ParseBoolError;
use super::NaiveIetfBcp47LanguageTag;
use super::NonEmptyVec;
use super::internationalized_resource_identifier::AbsoluteInternationalizedResourceIdentifier;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_usize;
use swiss_army_knife::try_to_own::MutableKey;
use swiss_army_knife::try_to_own::MutableKeyHashMap;
use swiss_army_knife::try_to_own::TryToOwn;
use swiss_army_knife::try_to_own::TryToOwnInPlace;


include!("Integer.rs");
include!("OnlyOneXmlSchemaValueError.rs");
include!("OptionalXmlSchemaValueError.rs");
include!("StringLiteralsMap.rs");
include!("StringLiteralsMapValuesIterator.rs");
include!("StringLiteralToDomainTypeParseError.rs");
include!("StrParser.rs");
include!("TryFromStrParser.rs");
include!("XmlSchemaValueKind.rs");
