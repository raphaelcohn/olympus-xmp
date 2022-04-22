// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A type that can be parsed similar to `FromStr::from_str()` but with stronger guarantees on `Error` and the ability to choose a different implementation (needed for `DateTime`, as `DateTime::from_str` is not appropriate).
pub trait StrParser<'string_literals_map, const PathDepth: usize>
{
	/// Item.
	type Item: Sized;
	
	/// Error.
	type Error: error::Error;
	
	/// Kind of value being parsed.
	const Kind: XmlSchemaValueKind;
	
	#[doc(hidden)]
	const Key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>;
	
	/// Parse.
	fn parse(value: &'string_literals_map str) -> Result<Self::Item, Self::Error>;
}

impl<'string_literals_map, const PathDepth: usize> StrParser<'string_literals_map, PathDepth> for &'string_literals_map str
{
	type Item = Self;
	
	type Error = Infallible;
	
	const Kind: XmlSchemaValueKind = XmlSchemaValueKind::String;
	
	const Key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth> = AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaString;
	
	#[inline(always)]
	fn parse(value: &'string_literals_map str) -> Result<Self::Item, Self::Error>
	{
		Ok(value)
	}
}

impl<'string_literals_map, const PathDepth: usize> StrParser<'string_literals_map, PathDepth> for bool
{
	type Item = Self;
	
	type Error = ParseBoolError;
	
	const Kind: XmlSchemaValueKind = XmlSchemaValueKind::Boolean;
	
	const Key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth> = AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaBoolean;
	
	#[inline(always)]
	fn parse(value: &'string_literals_map str) -> Result<Self::Item, Self::Error>
	{
		bool::from_str(value)
	}
}

impl<'string_literals_map, const PathDepth: usize> StrParser<'string_literals_map, PathDepth> for Integer
{
	type Item = Self;
	
	type Error = ParseIntError;
	
	const Kind: XmlSchemaValueKind = XmlSchemaValueKind::Integer;
	
	const Key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth> = AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaInteger;
	
	#[inline(always)]
	fn parse(value: &'string_literals_map str) -> Result<Self::Item, Self::Error>
	{
		Integer::from_str(value)
	}
}

impl<'string_literals_map, const PathDepth: usize> StrParser<'string_literals_map, PathDepth> for DateTime<FixedOffset>
{
	type Item = Self;
	
	type Error = ParseDateTimeError;
	
	const Kind: XmlSchemaValueKind = XmlSchemaValueKind::DateTime;
	
	const Key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth> = AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaDateTime;
	
	#[inline(always)]
	fn parse(value: &'string_literals_map str) -> Result<Self::Item, Self::Error>
	{
		DateTime::parse_from_rfc3339(value)
	}
}
