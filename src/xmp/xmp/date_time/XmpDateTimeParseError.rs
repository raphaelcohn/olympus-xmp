// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum XmpDateTimeParseError
{
	#[allow(missing_docs)]
	YearIsTooShort,
	
	#[allow(missing_docs)]
	YearIsNotValidNonZeroU16(ParseIntError),
	
	#[allow(missing_docs)]
	YearIsNotFollowedByHyphen,
	
	#[allow(missing_docs)]
	MonthIsTooShort,
	
	#[allow(missing_docs)]
	MonthIsNotValidNonZeroU8(ParseIntError),
	
	#[allow(missing_docs)]
	MonthIsTooLarge(TooLargeError<NonZeroU8>),
	
	#[allow(missing_docs)]
	MonthIsNotFollowedByHyphen,
	
	#[allow(missing_docs)]
	DayIsTooShort,
	
	#[allow(missing_docs)]
	DayIsNotValidNonZeroU8(ParseIntError),
	
	#[allow(missing_docs)]
	DayIsTooLarge(TooLargeError<NonZeroU8>),
	
	#[allow(missing_docs)]
	DayIsNotFollowedByHyphen,
	
	#[allow(missing_docs)]
	DayIsInvalidForMonth,
	
	#[allow(missing_docs)]
	HourIsTooShort,
	
	#[allow(missing_docs)]
	HourIsTooLarge(TooLargeError<u8>),
	
	#[allow(missing_docs)]
	HourIsNotValidU8(ParseIntError),
	
	#[allow(missing_docs)]
	HourIsNotFollowedByColon,
	
	#[allow(missing_docs)]
	HourWithoutMinute,
	
	#[allow(missing_docs)]
	MinuteIsTooShort,
	
	#[allow(missing_docs)]
	MinuteIsTooLarge(TooLargeError<u8>),
	
	#[allow(missing_docs)]
	MinuteIsNotValidU8(ParseIntError),
	
	#[allow(missing_docs)]
	MinuteFollowedByInvalidSeparator,
	
	#[allow(missing_docs)]
	SecondIsTooShort,
	
	#[allow(missing_docs)]
	SecondIsTooLarge(TooLargeError<u8>),
	
	#[allow(missing_docs)]
	SecondIsNotValidU8(ParseIntError),
	
	#[allow(missing_docs)]
	SecondFollowedByInvalidSeparator,
	
	#[allow(missing_docs)]
	TimeZoneHourIsNotValidU8(ParseIntError),
	
	#[allow(missing_docs)]
	TimeZoneHourIsTooLarge(TooLargeError<u8>),
	
	#[allow(missing_docs)]
	TimeZoneMinuteIsNotValidU8(ParseIntError),
	
	#[allow(missing_docs)]
	TimeZoneMinuteIsTooLarge(TooLargeError<u8>),
	
	#[allow(missing_docs)]
	NotEnoughBytesForTimeZoneOffset,
	
	#[allow(missing_docs)]
	TimeZoneOffsetHoursNotFollowedByColon,
	
	#[allow(missing_docs)]
	SubsecondIsEmpty,
	
	#[allow(missing_docs)]
	SubsecondIsNotValidU8(ParseIntError),
	
	#[allow(missing_docs)]
	SubsecondIsNotValidU16(ParseIntError),
	
	#[allow(missing_docs)]
	SubsecondIsNotValidU32(ParseIntError),
	
	#[allow(missing_docs)]
	SubsecondIsTooLong,
	
	#[allow(missing_docs)]
	SubsecondFollowedByInvalidSeparator,
	
	#[allow(missing_docs)]
	DecisecondTooLarge(TooLargeError<u8>),
	
	#[allow(missing_docs)]
	CentisecondTooLarge(TooLargeError<u8>),
	
	#[allow(missing_docs)]
	MillisecondTooLarge(TooLargeError<u16>),
	
	#[allow(missing_docs)]
	DecimillisecondTooLarge(TooLargeError<u16>),
	
	#[allow(missing_docs)]
	CentimillisecondTooLarge(TooLargeError<u32>),
	
	#[allow(missing_docs)]
	MicrosecondTooLarge(TooLargeError<u32>),
}

impl Display for XmpDateTimeParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for XmpDateTimeParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use XmpDateTimeParseError::*;
		match self
		{
			YearIsTooShort => None,
			
			YearIsNotValidNonZeroU16(cause) => Some(cause),
			
			YearIsNotFollowedByHyphen => None,
			
			MonthIsTooShort => None,
			
			MonthIsNotValidNonZeroU8(cause) => Some(cause),
			
			MonthIsTooLarge(cause) => Some(cause),
			
			MonthIsNotFollowedByHyphen => None,
			
			DayIsTooShort => None,
			
			DayIsNotValidNonZeroU8(cause) => Some(cause),
			
			DayIsTooLarge(cause) => Some(cause),
			
			DayIsNotFollowedByHyphen => None,
			
			DayIsInvalidForMonth => None,
			
			HourIsTooShort => None,
			
			HourIsNotValidU8(cause) => Some(cause),
			
			HourIsTooLarge(cause) => Some(cause),
			
			HourIsNotFollowedByColon => None,
			
			HourWithoutMinute => None,
			
			MinuteIsTooShort => None,
			
			MinuteIsTooLarge(cause) => Some(cause),
			
			MinuteIsNotValidU8(cause) => Some(cause),
			
			MinuteFollowedByInvalidSeparator => None,
			
			SecondIsTooShort => None,
			
			SecondIsTooLarge(cause) => Some(cause),
			
			SecondIsNotValidU8(cause) => Some(cause),
			
			SecondFollowedByInvalidSeparator => None,
			
			TimeZoneHourIsNotValidU8(cause) => Some(cause),
			
			TimeZoneHourIsTooLarge(cause) => Some(cause),
			
			TimeZoneMinuteIsNotValidU8(cause) => Some(cause),
			
			TimeZoneMinuteIsTooLarge(cause) => Some(cause),
			
			NotEnoughBytesForTimeZoneOffset => None,
			
			TimeZoneOffsetHoursNotFollowedByColon => None,
			
			SubsecondIsEmpty => None,
			
			SubsecondIsNotValidU8(cause) => Some(cause),
			
			SubsecondIsNotValidU16(cause) => Some(cause),
			
			SubsecondIsNotValidU32(cause) => Some(cause),
			
			SubsecondIsTooLong => None,
			
			SubsecondFollowedByInvalidSeparator => None,
			
			DecisecondTooLarge(cause) => Some(cause),
			
			CentisecondTooLarge(cause) => Some(cause),
			
			MillisecondTooLarge(cause) => Some(cause),
			
			DecimillisecondTooLarge(cause) => Some(cause),
			
			CentimillisecondTooLarge(cause) => Some(cause),
			
			MicrosecondTooLarge(cause) => Some(cause),
		}
	}
}
