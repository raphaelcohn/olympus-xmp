// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Date
{
	year: Year,
	
	month: Month,
	
	day: DayOfMonth,
}

impl FromStr for Date
{
	type Err = DateParseError;
	
	fn from_str(date_string: &str) -> Result<Self, DateParseError>
	{
		use DateParseError::*;
		
		#[inline(always)]
		fn next<'a>(split: &mut Split<'a, char>, error: DateParseError) -> Result<&'a str, DateParseError>
		{
			split.next().ok_or(error)
		}
		
		let mut split = date_string.split('-');
		
		let year = Year::from_str(split.next().unwrap())?;
		let month = Month::from_str(next(&mut split, MissingMonth)?)?;
		let day = DayOfMonth::from_str(next(&mut split, MissingDayOfMonth)?, year, month)?;
		
		if split.next().is_some()
		{
			return Err(TooManyHyphens)
		}
		
		Ok
		(
			Self
			{
				year,
				month,
				day,
			}
		)
	}
}
