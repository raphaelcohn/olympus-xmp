// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XMP date time.
///
/// Can omit the day of the month; somewhat unusual permutations.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum XmpDateTime
{
	#[allow(missing_docs)]
	Year(Year),
	
	#[allow(missing_docs)]
	YearMonth(YearMonth),
	
	#[allow(missing_docs)]
	YearMonthDay(YearMonthDay),
	
	#[allow(missing_docs)]
	YearMonthDayHourMinute(YearMonthDayHourMinute),
	
	#[allow(missing_docs)]
	YearMonthDayHourMinuteSecondSubsecond(YearMonthDayHourMinuteSecondSubsecond),
}

impl<'a> XmpAttributeValue<'a> for XmpDateTime
{
	type Error = XmpDateTimeParseError;
	
	/// Format is:
	///
	/// * `YYYY`
	/// * `YYYY-MM`
	/// * `YYYY-MM-DD`
	/// * `YYYY-MM-DDThh:mmTZD`
	/// * `YYYY-MM-DDThh:mm:ssTZD`
	/// * `YYYY-MM-DDThh:mm:ss.sTZD`
	///
	/// Where `TZD` can be:
	///
	/// * ``
	/// * `Z`
	/// * `-hh:mm`
	/// * `+hh:mm`
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use XmpDateTimeParseError::*;
		use TimeZone::UniversalTimeCoordinated;
		
		const SeparatorWidth: usize = size_of::<u8>();
		const FourDigitWidth: usize = 4;
		const TwoDigitWidth: usize = 2;
		
		let bytes = value.as_bytes();
		
		macro_rules! if_finished
		{
			($parsed: ident, $finish_parsing: ident, $constructor: ident) =>
			{
				{
					if $finish_parsing
					{
						return Ok(XmpDateTime::$constructor($parsed))
					}
					$parsed
				}
			}
		}
		
		let year =
		{
			const InclusiveFromIndex: usize = 0;
			let (parsed, finish_parsing) = Self::parse_digits_and_check_separator::<NonZeroU16, Year, _, _, Hyphen, InclusiveFromIndex, FourDigitWidth>(bytes, YearIsTooShort, YearIsNotFollowedByHyphen, YearIsNotValidNonZeroU16, |raw_digits| Ok(Year(raw_digits)))?;
			if_finished!(parsed, finish_parsing, Year)
		};
		
		let year_month =
		{
			const InclusiveFromIndex: usize = 0 + FourDigitWidth + SeparatorWidth;
			let (parsed, finish_parsing) = Self::parse_digits_and_check_separator::<NonZeroU8, YearMonth, _, _, Hyphen, InclusiveFromIndex, TwoDigitWidth>(bytes, MonthIsTooShort, MonthIsNotFollowedByHyphen, MonthIsNotValidNonZeroU8, |raw_digits| match Month::try_from(raw_digits)
			{
				Err(error) => Err(MonthIsTooLarge(error)),
				
				Ok(month) => Ok(YearMonth(year, month)),
			})?;
			if_finished!(parsed, finish_parsing, YearMonth)
		};
		
		let year_month_day =
		{
			const InclusiveFromIndex: usize = 0 + FourDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth;
			let (parsed, finish_parsing) = Self::parse_digits_and_check_separator::<NonZeroU8, YearMonthDay, _, _, T, InclusiveFromIndex, TwoDigitWidth>(bytes, DayIsTooShort, DayIsNotFollowedByHyphen, DayIsNotValidNonZeroU8, |raw_digits| match DayOfMonth::try_from(raw_digits)
			{
				Err(error) => Err(DayIsTooLarge(error)),
				
				Ok(day) => if day.is_valid_for(year_month)
				{
					Ok(YearMonthDay(year_month, day))
				}
				else
				{
					Err(DayIsInvalidForMonth)
				},
			})?;
			if_finished!(parsed, finish_parsing, YearMonthDay)
		};
		
		let hour_minute =
		{
			let hour =
			{
				const InclusiveFromIndex: usize = 0 + FourDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth;
				let (parsed, finish_parsing) = Self::parse_digits_and_check_separator::<u8, Hour, _, _, Colon, InclusiveFromIndex, TwoDigitWidth>(bytes, HourIsTooShort, HourIsNotFollowedByColon, HourIsNotValidU8, |raw_digits| Hour::try_from(raw_digits).map_err(HourIsTooLarge))?;
				if finish_parsing
				{
					return Err(HourWithoutMinute)
				}
				parsed
			};
			const InclusiveFromIndex: usize = 0 + FourDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth;
			let hour_minute = HourMinute
			{
				hour,
				minute: Self::parse_digits::<u8, Minute, _, _, InclusiveFromIndex, TwoDigitWidth>(bytes, MinuteIsTooShort, MinuteIsNotValidU8, |raw_digits| Minute::try_from(raw_digits).map_err(MinuteIsTooLarge))?
			};
			
			const SeparatorIndex: usize = InclusiveFromIndex + TwoDigitWidth;
			match bytes.get(SeparatorIndex)
			{
				None => return Self::ok_year_month_day_hour_minute(year_month_day, hour_minute, None),
				
				Some(&Z) => return Self::ok_year_month_day_hour_minute(year_month_day, hour_minute, Some(UniversalTimeCoordinated)),
				
				Some(&PlusSign) => return Self::ok_year_month_day_hour_minute(year_month_day, hour_minute, Self::parse_positive_time_zone(bytes, SeparatorIndex)?),
				
				Some(&MinusSign) => return Self::ok_year_month_day_hour_minute(year_month_day, hour_minute, Self::parse_negative_time_zone(bytes, SeparatorIndex)?),
				
				Some(&Colon) => hour_minute,
				
				Some(_) => return Err(MinuteFollowedByInvalidSeparator)
			}
		};
		
		let second =
		{
			const InclusiveFromIndex: usize = 0 + FourDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth;
			let second = Self::parse_digits::<u8, Second, _, _, InclusiveFromIndex, TwoDigitWidth>(bytes, SecondIsTooShort, SecondIsNotValidU8, |raw_digits| Second::try_from(raw_digits).map_err(SecondIsTooLarge))?;
			
			const SeparatorIndex: usize = InclusiveFromIndex + TwoDigitWidth;
			match bytes.get(SeparatorIndex)
			{
				None => return Self::ok_year_month_day_hour_minute_second_subsecond(year_month_day, hour_minute, second, None, None),
				
				Some(&Z) => return Self::ok_year_month_day_hour_minute_second_subsecond(year_month_day, hour_minute, second, None, Some(UniversalTimeCoordinated)),
				
				Some(&PlusSign) => return Self::ok_year_month_day_hour_minute_second_subsecond(year_month_day, hour_minute, second, None, Self::parse_positive_time_zone(bytes, SeparatorIndex)?),
				
				Some(&MinusSign) => return Self::ok_year_month_day_hour_minute_second_subsecond(year_month_day, hour_minute, second, None, Self::parse_negative_time_zone(bytes, SeparatorIndex)?),
				
				Some(&Period) => second,
				
				Some(_) => return Err(SecondFollowedByInvalidSeparator)
			}
		};
		
		let (time_zone, subsecond) =
		{
			const InclusiveFromIndex: usize = 0 + FourDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth + TwoDigitWidth + SeparatorWidth;
			let remaining_bytes = bytes.get_unchecked_range_safe(InclusiveFromIndex .. );
			match remaining_bytes.memchr3(Z, PlusSign, MinusSign)
			{
				None =>
				{
					(
						None,
						Self::parse_subsecond(remaining_bytes, remaining_bytes.len())?
					)
				}
				
				Some(separator_index) =>
				{
					(
						match remaining_bytes.get_unchecked_value_safe(separator_index)
						{
							Z => Some(UniversalTimeCoordinated),
							
							PlusSign => Self::parse_positive_time_zone(bytes, separator_index)?,
							
							MinusSign => Self::parse_negative_time_zone(bytes, separator_index)?,
							
							_ => return Err(SubsecondFollowedByInvalidSeparator),
						},
						Self::parse_subsecond(remaining_bytes, separator_index)?
					)
				}
			}
		};
		
		Self::ok_year_month_day_hour_minute_second_subsecond(year_month_day, hour_minute, second, subsecond, time_zone)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::DateTime(error)
	}
}

impl XmpDateTime
{
	#[inline(always)]
	fn parse_digits<U: FromStr<Err=ParseIntError>, P, NVUIE: FnOnce(ParseIntError) -> XmpDateTimeParseError, Parser: FnOnce(U) -> Result<P, XmpDateTimeParseError>, const InclusiveFromIndex: usize, const Width: usize>(bytes: &[u8], too_short_error: XmpDateTimeParseError, not_valid_unsigned_integer_error: NVUIE, parser: Parser) -> Result<P, XmpDateTimeParseError>
	{
		let bytes = Self::slice::<InclusiveFromIndex, Width>(bytes, too_short_error)?;
		let unsigned_integer = U::from_str(Self::bytes_to_str(bytes)).map_err(not_valid_unsigned_integer_error)?;
		parser(unsigned_integer)
	}
	
	#[inline(always)]
	fn parse_digits_and_check_separator<U: FromStr<Err=ParseIntError>, P, NVUIE: FnOnce(ParseIntError) -> XmpDateTimeParseError, Parser: FnOnce(U) -> Result<P, XmpDateTimeParseError>, const Separator: u8, const InclusiveFromIndex: usize, const Width: usize>(bytes: &[u8], too_short_error: XmpDateTimeParseError, separator_error: XmpDateTimeParseError, not_valid_unsigner_integer_error: NVUIE, parser: Parser) -> Result<(P, bool), XmpDateTimeParseError>
	{
		let parsed = Self::parse_digits::<U, P, _, _, InclusiveFromIndex, Width>(bytes, too_short_error, not_valid_unsigner_integer_error, parser)?;
		
		let finish_parsing =
		{
			let separator_index = InclusiveFromIndex + Width;
			debug_assert!(separator_index <= bytes.len());
			if separator_index != bytes.len()
			{
				let separator = bytes.get_unchecked_value_safe(separator_index);
				if separator != Separator
				{
					return Err(separator_error)
				}
				false
			}
			else
			{
				true
			}
		};
		
		Ok((parsed, finish_parsing))
	}
	
	#[inline(always)]
	fn ok_year_month_day_hour_minute(year_month_day: YearMonthDay, hour_minute: HourMinute, time_zone: Option<TimeZone>) -> Result<Self, XmpDateTimeParseError>
	{
		Ok(XmpDateTime::YearMonthDayHourMinute(year_month_day.with(hour_minute, time_zone)))
	}
	
	#[inline(always)]
	fn ok_year_month_day_hour_minute_second_subsecond(year_month_day: YearMonthDay, hour_minute: HourMinute, second: Second, subsecond: Option<Subsecond>, time_zone: Option<TimeZone>) -> Result<Self, XmpDateTimeParseError>
	{
		Ok(XmpDateTime::YearMonthDayHourMinuteSecondSubsecond(year_month_day.with(hour_minute, time_zone).with(second, subsecond)))
	}
	
	#[inline(always)]
	fn parse_subsecond_enum_member<U: FromStr<Err=ParseIntError> + Debug + Copy + Ord + Eq + Hash, P: TryFrom<U, Error=TooLargeError<U>>, NVUIE: FnOnce(ParseIntError) -> XmpDateTimeParseError, C: FnOnce(P) -> Subsecond, TLE: FnOnce(TooLargeError<U>) -> XmpDateTimeParseError, const Width: usize>(remaining_bytes: &[u8], not_valid_unsigned_integer_error: NVUIE, constructor: C, too_large_error: TLE) -> Result<Subsecond, XmpDateTimeParseError>
	{
		const NeverHappens: XmpDateTimeParseError = XmpDateTimeParseError::SecondIsTooShort;
		Self::parse_digits::<U, Subsecond, _, _, 0, Width>(remaining_bytes, NeverHappens, not_valid_unsigned_integer_error, |raw_digits| match P::try_from(raw_digits)
		{
			Ok(parsed) => Ok(constructor(parsed)),
			
			Err(error) => Err(too_large_error(error)),
		})
	}
	
	#[inline(always)]
	fn parse_subsecond(remaining_bytes: &[u8], separator_index: usize) -> Result<Option<Subsecond>, XmpDateTimeParseError>
	{
		use XmpDateTimeParseError::*;
		
		let subsecond = match separator_index
		{
			0 => return Err(SubsecondIsEmpty),
			
			1 => Self::parse_subsecond_enum_member::<u8, Decisecond, _, _, _, 1>(remaining_bytes, SubsecondIsNotValidU8, Subsecond::Decisecond, DecisecondTooLarge)?,
			
			2 => Self::parse_subsecond_enum_member::<u8, Centisecond, _, _, _, 2>(remaining_bytes, SubsecondIsNotValidU8, Subsecond::Centisecond, CentisecondTooLarge)?,
			
			3 => Self::parse_subsecond_enum_member::<u16, Millisecond, _, _, _, 3>(remaining_bytes, SubsecondIsNotValidU16, Subsecond::Millisecond, MillisecondTooLarge)?,
			
			4 => Self::parse_subsecond_enum_member::<u16, Decimillisecond, _, _, _, 4>(remaining_bytes, SubsecondIsNotValidU16, Subsecond::Decimillisecond, DecimillisecondTooLarge)?,
			
			5 => Self::parse_subsecond_enum_member::<u32, Centimillisecond, _, _, _, 5>(remaining_bytes, SubsecondIsNotValidU32, Subsecond::Centimillisecond, CentimillisecondTooLarge)?,
			
			6 => Self::parse_subsecond_enum_member::<u32, Microsecond, _, _, _, 6>(remaining_bytes, SubsecondIsNotValidU32, Subsecond::Microsecond, MicrosecondTooLarge)?,
			
			_ => return Err(SubsecondIsTooLong),
		};
		Ok(Some(subsecond))
	}
	
	#[inline(always)]
	fn parse_positive_time_zone(bytes: &[u8], separator_index: usize) -> Result<Option<TimeZone>, XmpDateTimeParseError>
	{
		Ok(Some(Self::parse_offset_time_zone(bytes, separator_index, TimeZone::Positive)?))
	}
	
	#[inline(always)]
	fn parse_negative_time_zone(bytes: &[u8], separator_index: usize) -> Result<Option<TimeZone>, XmpDateTimeParseError>
	{
		Ok(Some(Self::parse_offset_time_zone(bytes, separator_index, TimeZone::Negative)?))
	}
	
	#[inline(always)]
	fn parse_offset_time_zone(bytes: &[u8], separator_index: usize, time_zone: impl FnOnce(HourMinute) -> TimeZone) -> Result<TimeZone, XmpDateTimeParseError>
	{
		use XmpDateTimeParseError::*;
		
		let remainder = bytes.get_unchecked_range_safe((separator_index + 1) .. );
		if remainder.len() != 5
		{
			return Err(NotEnoughBytesForTimeZoneOffset)
		}
		
		if bytes.get_unchecked_value_safe(2) != Colon
		{
			return Err(TimeZoneOffsetHoursNotFollowedByColon)
		}
		
		#[inline(always)]
		fn parse_digits<P: TryFrom<u8, Error=TooLargeError<u8>>, NVUIE: FnOnce(ParseIntError) -> XmpDateTimeParseError, TLE: FnOnce(TooLargeError<u8>) -> XmpDateTimeParseError, const From: usize>(remainder: &[u8], not_valid_unsigner_integer_error: NVUIE, too_large_error: TLE) -> Result<P, XmpDateTimeParseError>
		{
			let bytes = remainder.get_unchecked_range_safe(From .. (From + 2));
			let raw_digits = u8::from_str(XmpDateTime::bytes_to_str(bytes)).map_err(not_valid_unsigner_integer_error)?;
			P::try_from(raw_digits).map_err(too_large_error)
		}
		
		Ok
		(
			time_zone
			(
				HourMinute
				{
					hour: parse_digits::<Hour, _, _, 0>(remainder, TimeZoneHourIsNotValidU8, TimeZoneHourIsTooLarge)?,
				
					minute: parse_digits::<Minute, _, _, 3>(remainder, TimeZoneMinuteIsNotValidU8, TimeZoneMinuteIsTooLarge)?,
				}
			)
		)
	}
	
	#[inline(always)]
	fn bytes_to_str(bytes: &[u8]) -> &str
	{
		unsafe { from_utf8_unchecked(bytes)}
	}
	
	#[inline(always)]
	fn slice<const InclusiveFromIndex: usize, const Width: usize>(bytes: &[u8], error: XmpDateTimeParseError) -> Result<&[u8], XmpDateTimeParseError>
	{
		let exclusive_end_index = InclusiveFromIndex + Width;
		if exclusive_end_index > bytes.len()
		{
			Err(error)
		}
		else
		{
			Ok(bytes.get_unchecked_range_safe(InclusiveFromIndex .. exclusive_end_index ))
		}
	}
}
