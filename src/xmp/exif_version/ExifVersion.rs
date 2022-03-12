// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Exif version.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ExifVersion
{
	epoch: ExifVersionField,
	
	major: ExifVersionField,

	minor: ExifVersionField,

	revision: ExifVersionField,
}

impl Default for ExifVersion
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Version_2_30
	}
}

impl<'a> XmpAttributeValue<'a> for ExifVersion
{
	type Error = ExifVersionParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		{
			let count = value.len();
			if count != 4
			{
				return Err(ExifVersionParseError::NotExactly4Bytes { count })
			}
		}
		let bytes = value.as_bytes();
		Ok
		(
			Self
			{
				epoch: Self::parse_field::<0>(bytes)?,
				major: Self::parse_field::<1>(bytes)?,
				minor: Self::parse_field::<2>(bytes)?,
				revision: Self::parse_field::<3>(bytes)?,
			}
		)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::ExifVersion(error)
	}
}

impl ExifVersion
{
	#[allow(missing_docs)]
	pub const Version_2_1: Self = Self::version_2_unrevised(_1);
	
	#[allow(missing_docs)]
	pub const Version_2_2: Self = Self::version_2_unrevised(_2);
	
	#[allow(missing_docs)]
	pub const Version_2_30: Self = Self::version_2_3(_0);
	
	#[allow(missing_docs)]
	pub const Version_2_31: Self = Self::version_2_3(_1);
	
	#[allow(missing_docs)]
	pub const Version_2_32: Self = Self::version_2_3(_2);
	
	#[inline(always)]
	const fn version_2_unrevised(minor: ExifVersionField) -> Self
	{
		Self::version_2(minor, _0)
	}
	
	#[inline(always)]
	const fn version_2_3(revision: ExifVersionField) -> Self
	{
		Self::version_2(_3, revision)
	}
	
	#[inline(always)]
	const fn version_2(minor: ExifVersionField, revision: ExifVersionField) -> Self
	{
		Self::epoch_0(_2, minor, revision)
	}
	
	#[inline(always)]
	const fn epoch_0(version: ExifVersionField, minor: ExifVersionField, revision: ExifVersionField) -> Self
	{
		Self::new(_0, version, minor, revision)
	}
	
	#[inline(always)]
	const fn new(epoch: ExifVersionField, major: ExifVersionField, minor: ExifVersionField, revision: ExifVersionField) -> Self
	{
		Self
		{
			epoch,
			
			major,
		
			minor,
			
			revision,
		}
	}
	
	#[inline(always)]
	fn parse_field<const field_index: u8>(bytes: &[u8]) -> Result<ExifVersionField, ExifVersionParseError>
	{
		ExifVersionField::parse::<field_index>(bytes).map_err(|cause| ExifVersionParseError::InvalidField { cause, field_index })
	}
}
