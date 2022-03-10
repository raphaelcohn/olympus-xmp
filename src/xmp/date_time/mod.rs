// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;
use hours_and_minutes::Hour;
use hours_and_minutes::Minute;
use hours_and_minutes::HourMinute;
use hours_and_minutes::TimeZone;
use memchr::memchr3;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::mem::size_of;
use std::mem::transmute;
use std::num::NonZeroU8;
use std::num::NonZeroU16;
use std::num::ParseIntError;
use std::str::FromStr;
use std::str::from_utf8_unchecked;
use subseconds::Centisecond;
use subseconds::Centimillisecond;
use subseconds::Decimillisecond;
use subseconds::Decisecond;
use subseconds::Microsecond;
use subseconds::Millisecond;
use subseconds::Subsecond;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_u8;


/// Hours and minutes.
pub mod hours_and_minutes;


/// Subseconds.
pub mod subseconds;


include!("DayOfMonth.rs");
include!("Month.rs");
include!("Second.rs");
include!("TooLargeError.rs");
include!("XmpDateTime.rs");
include!("XmpDateTimeParseError.rs");
include!("Year.rs");
include!("YearMonth.rs");
include!("YearMonthDay.rs");
include!("YearMonthDayHourMinute.rs");
include!("YearMonthDayHourMinuteSecondSubsecond.rs");
