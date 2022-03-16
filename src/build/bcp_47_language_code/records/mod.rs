// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::collections::HashMap;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::str::FromStr;
use std::str::SplitN;
use super::date::Date;
use super::date::DateParseError;
use super::parser::Event;
use super::parser::FieldEvent;
use super::parser::PullEventParser;
use super::parser::PullEventParserError;
use super::LanguageSubtagRegistryFileParseError;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("ExtlangRecord.rs");
include!("FieldError.rs");
include!("FieldNotPermittedError.rs");
include!("FieldParseError.rs");
include!("GrandfatheredRecord.rs");
include!("KeyParseError.rs");
include!("LanguageRecord.rs");
include!("LanguageSubtag.rs");
include!("ParseRecord.rs");
include!("Record.rs");
include!("RecordFields.rs");
include!("MissingFieldError.rs");
include!("Records.rs");
include!("RecordParseError.rs");
include!("RecordsFileHeaderParseError.rs");
include!("RedundantRecord.rs");
include!("RegionRecord.rs");
include!("Scope.rs");
include!("ScriptRecord.rs");
include!("TagOrSubtag.rs");
include!("TagOrSubtagRangeError.rs");
include!("Type.rs");
include!("UnknownStringVariantError.rs");
include!("VariantRecord.rs");
