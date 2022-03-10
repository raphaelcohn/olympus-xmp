// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XMP attribute value parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum XmpAttributeValueParseError
{
	#[allow(missing_docs)]
	Boolean(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	DateTime(XmpDateTimeParseError),
	
	#[allow(missing_docs)]
	ExifSceneCaptureType(U8ParseError),
	
	#[allow(missing_docs)]
	IptcDigitalSourceType(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	PhotoshopColorMode(U8ParseError),
	
	#[allow(missing_docs)]
	NonZeroU32(ParseIntError),
	
	#[allow(missing_docs)]
	OptionNonZeroU16(ParseIntError),
	
	#[allow(missing_docs)]
	PlusModelReleaseStatus(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	PlusPropertyReleaseStatus(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	TiffRational(TiffRationalParseError),
	
	#[allow(missing_docs)]
	UniversallyUniqueIdentifier(XmpUniversallyUniqueIdentifierParseError),
	
	#[allow(missing_docs)]
	XmpLabel(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	XmpRating(I8ParseError),
}

impl Display for XmpAttributeValueParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for XmpAttributeValueParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use XmpAttributeValueParseError::*;
		match self
		{
			Boolean(cause) => Some(cause),
			
			DateTime(cause) => Some(cause),
			
			ExifSceneCaptureType(cause) => Some(cause),
			
			IptcDigitalSourceType(cause) => Some(cause),
			
			NonZeroU32(cause) => Some(cause),
			
			OptionNonZeroU16(cause) => Some(cause),
			
			PhotoshopColorMode(cause) => Some(cause),
			
			PlusModelReleaseStatus(cause) => Some(cause),
			
			PlusPropertyReleaseStatus(cause) => Some(cause),
			
			TiffRational(cause) => Some(cause),
			
			UniversallyUniqueIdentifier(cause) => Some(cause),
			
			XmpLabel(cause) => Some(cause),
			
			XmpRating(cause) => Some(cause),
		}
	}
}
