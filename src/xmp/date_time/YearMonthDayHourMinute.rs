// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Part of a XMP date time.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct YearMonthDayHourMinute(YearMonthDay, HourMinute, Option<TimeZone>);

impl YearMonthDayHourMinute
{
	#[inline(always)]
	fn with(self, second: Second, subsecond: Option<Subsecond>) -> YearMonthDayHourMinuteSecondSubsecond
	{
		YearMonthDayHourMinuteSecondSubsecond(self, second, subsecond)
	}
}
