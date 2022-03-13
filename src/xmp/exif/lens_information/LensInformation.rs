// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Lens information.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LensInformation
{
	#[allow(missing_docs)]
	Prime(FocalLengthAndWidestAperture),
	
	#[allow(missing_docs)]
	Zoom
	{
		minimum: FocalLengthAndWidestAperture,
		
		maximum: FocalLengthAndWidestAperture,
	}
}

impl<'a> XmpAttributeValue<'a> for LensInformation
{
	type Error = LensInformationParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use LensInformationParseError::*;
		
		let mut iterator = value.split(' ');
		let minimum_focal_length_in_millimetres = Self::parse_field(&mut iterator, NoMinimumFocalLength, MinimumFocalLength)?;
		let maximum_focal_length_in_millimetres = Self::parse_field(&mut iterator, NoMaximumFocalLength, MaximumFocalLength)?;
		let minimum_widest_aperture_at_focal_length_in_millimetres = Self::parse_field(&mut iterator, NoWidestApertureAtMinimumFocalLength, WidestApertureAtMinimumFocalLength)?;
		let maximum_widest_aperture_at_focal_length_in_millimetres = Self::parse_field(&mut iterator, NoWidestApertureAtMaximumFocalLength, WidestApertureAtMaximumFocalLength)?;
		if iterator.next().is_some()
		{
			return Err(MoreThanFourFields)
		}
		use LensInformation::*;
		use Ordering::*;
		match minimum_focal_length_in_millimetres.cmp(&maximum_focal_length_in_millimetres)
		{
			Less => Ok
			(
				Zoom
				{
					minimum: FocalLengthAndWidestAperture
					{
						focal_length_in_millimetres: minimum_focal_length_in_millimetres,
						widest_aperture_at_focal_length_in_millimetres: minimum_widest_aperture_at_focal_length_in_millimetres,
					},
					maximum: FocalLengthAndWidestAperture
					{
						focal_length_in_millimetres: maximum_focal_length_in_millimetres,
						widest_aperture_at_focal_length_in_millimetres: maximum_widest_aperture_at_focal_length_in_millimetres,
					}
				}
			),
			
			Equal if minimum_widest_aperture_at_focal_length_in_millimetres == maximum_widest_aperture_at_focal_length_in_millimetres => Ok
			(
				Prime
				(
					FocalLengthAndWidestAperture
					{
						focal_length_in_millimetres: minimum_focal_length_in_millimetres,
						widest_aperture_at_focal_length_in_millimetres: minimum_widest_aperture_at_focal_length_in_millimetres,
					}
				)
			),
			
			Equal => Err(PrimeHasDifferingWidestApertures { minimum_widest_aperture_at_focal_length_in_millimetres, maximum_widest_aperture_at_focal_length_in_millimetres }),
			
			Greater => Err(MinimumIsGreaterThanMaximum { minimum_focal_length_in_millimetres, maximum_focal_length_in_millimetres })
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::LensInformation(error)
	}
}

impl LensInformation
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn contains_focal_length(&self, focal_length: NonZeroUnsignedTiffRational) -> bool
	{
		use LensInformation::*;
		match self
		{
			Prime(prime) => prime.focal_length_in_millimetres == focal_length,
			
			Zoom { minimum, maximum} => minimum.focal_length_in_millimetres <= focal_length && maximum.focal_length_in_millimetres >= focal_length,
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn could_have_f_number(&self, f_number: NonZeroUnsignedTiffRational) -> bool
	{
		use LensInformation::*;
		match self
		{
			Prime(prime) => prime.widest_aperture_at_focal_length_in_millimetres <= f_number,
			
			Zoom { minimum, maximum} => minimum.widest_aperture_at_focal_length_in_millimetres <= f_number && maximum.widest_aperture_at_focal_length_in_millimetres <= f_number,
		}
	}
	
	#[inline(always)]
	fn parse_field(iterator: &mut Split<char>, missing_error: LensInformationParseError, invalid_error: impl FnOnce(NonZeroUnsignedTiffRationalParseError) -> LensInformationParseError) -> Result<NonZeroUnsignedTiffRational, LensInformationParseError>
	{
		NonZeroUnsignedTiffRational::parse(iterator.next().ok_or(missing_error)?).map_err(invalid_error)
	}
}
