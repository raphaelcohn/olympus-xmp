// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use chrono::DateTime;
use chrono::FixedOffset;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::lazy::SyncLazy;
use std::mem::transmute;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::ParseBoolError;
use super::BlankNodeLabel;
use super::NaiveIetfBcp47LanguageTag;
use super::Objects;
use super::PathDepth;
use super::internationalized_resource_identifier::AbsoluteInternationalizedResourceIdentifier;
use super::string_literals_map::OnlyOneError;
use super::string_literals_map::ParseDateTimeError;
use super::string_literals_map::StringLiteralsMap;


include!("OnlyOneXmlSchemaStringLiteralError.rs");
include!("Predicate.rs");
include!("Predicates.rs");
