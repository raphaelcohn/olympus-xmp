// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XMP attribute value parse error.
#[derive(Debug)]
pub enum XmpAttributeValueParseError
{
	#[allow(missing_docs)]
	Boolean(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	DateTime(XmpDateTimeParseError),
	
	#[allow(missing_docs)]
	EmailAddress(EmailAddressParseError),
	
	#[allow(missing_docs)]
	ExifContrastOrSharpness(U16ParseError),
	
	#[allow(missing_docs)]
	ExifCustomRendered(U16ParseError),
	
	#[allow(missing_docs)]
	ExifExposureMode(U16ParseError),
	
	#[allow(missing_docs)]
	ExifExposureProgram(U16ParseError),
	
	#[allow(missing_docs)]
	ExifFileSource(U8ParseError),
	
	#[allow(missing_docs)]
	ExifFlashMode(U8ParseError),
	
	#[allow(missing_docs)]
	ExifFlashStatusOfStrobeReturnedLight(U8ParseError),
	
	#[allow(missing_docs)]
	ExifGainControl(U16ParseError),
	
	#[allow(missing_docs)]
	ExifLightSource(U16ParseError),
	
	#[allow(missing_docs)]
	ExifMeteringMode(U16ParseError),
	
	#[allow(missing_docs)]
	ExifResolutionUnit(U16ParseError),
	
	#[allow(missing_docs)]
	ExifSaturation(U16ParseError),
	
	#[allow(missing_docs)]
	ExifSceneCaptureType(U16ParseError),
	
	#[allow(missing_docs)]
	ExifSensitivityType(U16ParseError),
	
	#[allow(missing_docs)]
	ExifVersion(ExifVersionParseError),
	
	#[allow(missing_docs)]
	ExifWhiteBalanceMode(U16ParseError),
	
	#[allow(missing_docs)]
	IimCategoryCode(IimCategoryCodeParseError),
	
	#[allow(missing_docs)]
	IimSupplementalCategories(IimSupplementalCategoriesParseError),
	
	#[allow(missing_docs)]
	InstanceIdentifier(XmpInstanceIdentifierParseError),
	
	#[allow(missing_docs)]
	IptcDigitalSourceType(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	IptcWorldRegion(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	Iso3166Dash1AlphaCountryCode(Iso3166Dash1AlphaCountryCodeParseError),
	
	#[allow(missing_docs)]
	Iso3166Dash1Country(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	LensInformation(LensInformationParseError),
	
	#[allow(missing_docs)]
	LightroomHierarchialSubject(LightroomHierarchialSubjectParseError),
	
	#[allow(missing_docs)]
	PhotoshopColorMode(U8ParseError),
	
	#[allow(missing_docs)]
	NonEmptyStr(NonEmptyStrParseError),
	
	#[allow(missing_docs)]
	NonZeroUnsignedTiffRational(NonZeroUnsignedTiffRationalParseError),
	
	#[allow(missing_docs)]
	NonZeroU32(ParseIntError),
	
	#[allow(missing_docs)]
	OptionNonZeroU16(ParseIntError),
	
	#[allow(missing_docs)]
	PhoneNumber(PhoneNumberParseError),
	
	#[allow(missing_docs)]
	PlusLicensorTelephoneType(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	PlusMinorModelAgeDisclosure(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	PlusModelReleaseStatus(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	PlusPropertyReleaseStatus(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	Subject(NonEmptyStrParseError),
	
	#[allow(missing_docs)]
	UniversallyUniqueIdentifier(XmpUniversallyUniqueIdentifierParseError),
	
	#[allow(missing_docs)]
	UnsignedTiffRational(UnsignedTiffRationalParseError),
	
	#[allow(missing_docs)]
	Urgency(UrgencyParseError),
	
	#[allow(missing_docs)]
	Url(UrlParseError),
	
	#[allow(missing_docs)]
	XmpLabel(UnknownStringVariantParseError),
	
	#[allow(missing_docs)]
	XmpRating(I8ParseError),
}

impl Display for XmpAttributeValueParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
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
			
			EmailAddress(cause) => Some(cause),
			
			ExifContrastOrSharpness(cause) => Some(cause),
			
			ExifCustomRendered(cause) => Some(cause),
			
			ExifExposureMode(cause) => Some(cause),
			
			ExifExposureProgram(cause) => Some(cause),
			
			ExifFileSource(cause) => Some(cause),
			
			ExifFlashMode(cause) => Some(cause),
			
			ExifFlashStatusOfStrobeReturnedLight(cause) => Some(cause),
			
			ExifGainControl(cause) => Some(cause),
			
			ExifLightSource(cause) => Some(cause),
			
			ExifMeteringMode(cause) => Some(cause),
			
			ExifResolutionUnit(cause) => Some(cause),
			
			ExifSaturation(cause) => Some(cause),
			
			ExifSceneCaptureType(cause) => Some(cause),
			
			ExifSensitivityType(cause) => Some(cause),
			
			ExifVersion(cause) => Some(cause),
			
			ExifWhiteBalanceMode(cause) => Some(cause),
			
			IimCategoryCode(cause) => Some(cause),
			
			IimSupplementalCategories(cause) => Some(cause),
			
			InstanceIdentifier(cause) => Some(cause),
			
			IptcDigitalSourceType(cause) => Some(cause),
			
			IptcWorldRegion(cause) => Some(cause),
			
			Iso3166Dash1AlphaCountryCode(cause) => Some(cause),
			
			Iso3166Dash1Country(cause) => Some(cause),
			
			LensInformation(cause) => Some(cause),
			
			LightroomHierarchialSubject(cause) => Some(cause),
			
			NonEmptyStr(cause) => Some(cause),
			
			NonZeroUnsignedTiffRational(cause) => Some(cause),
			
			NonZeroU32(cause) => Some(cause),
			
			OptionNonZeroU16(cause) => Some(cause),
			
			PhoneNumber(cause) => Some(cause),
			
			PhotoshopColorMode(cause) => Some(cause),
			
			PlusLicensorTelephoneType(cause) => Some(cause),
			
			PlusMinorModelAgeDisclosure(cause) => Some(cause),
			
			PlusModelReleaseStatus(cause) => Some(cause),
			
			PlusPropertyReleaseStatus(cause) => Some(cause),
			
			Subject(cause) => Some(cause),
			
			UniversallyUniqueIdentifier(cause) => Some(cause),
			
			UnsignedTiffRational(cause) => Some(cause),
			
			Url(cause) => Some(cause),
			
			Urgency(cause) => Some(cause),
			
			XmpLabel(cause) => Some(cause),
			
			XmpRating(cause) => Some(cause),
		}
	}
}
