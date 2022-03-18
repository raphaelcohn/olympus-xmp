// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Lens information parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LensInformationParseError
{
	#[allow(missing_docs)]
	NoMinimumFocalLength,
	
	#[allow(missing_docs)]
	NoWidestApertureAtMinimumFocalLength,
	
	#[allow(missing_docs)]
	NoMaximumFocalLength,
	
	#[allow(missing_docs)]
	NoWidestApertureAtMaximumFocalLength,
	
	#[allow(missing_docs)]
	MoreThanFourFields,
	
	#[allow(missing_docs)]
	MinimumFocalLength(NonZeroUnsignedTiffRationalParseError),
	
	#[allow(missing_docs)]
	WidestApertureAtMinimumFocalLength(NonZeroUnsignedTiffRationalParseError),
	
	#[allow(missing_docs)]
	MaximumFocalLength(NonZeroUnsignedTiffRationalParseError),
	
	#[allow(missing_docs)]
	WidestApertureAtMaximumFocalLength(NonZeroUnsignedTiffRationalParseError),
	
	#[allow(missing_docs)]
	PrimeHasDifferingWidestApertures
	{
		minimum_widest_aperture_at_focal_length_in_millimetres: NonZeroUnsignedTiffRational,
		
		maximum_widest_aperture_at_focal_length_in_millimetres: NonZeroUnsignedTiffRational,
	},
	
	#[allow(missing_docs)]
	MinimumIsGreaterThanMaximum
	{
		minimum_focal_length_in_millimetres: NonZeroUnsignedTiffRational,
		
		maximum_focal_length_in_millimetres: NonZeroUnsignedTiffRational,
	},
}

impl Display for LensInformationParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for LensInformationParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use LensInformationParseError::*;
		match self
		{
			NoMinimumFocalLength => None,
			
			NoWidestApertureAtMinimumFocalLength => None,
			
			NoMaximumFocalLength => None,
			
			NoWidestApertureAtMaximumFocalLength => None,
			
			MoreThanFourFields => None,
			
			MinimumFocalLength(cause) => Some(cause),
			
			WidestApertureAtMinimumFocalLength(cause) => Some(cause),
			
			MaximumFocalLength(cause) => Some(cause),
			
			WidestApertureAtMaximumFocalLength(cause) => Some(cause),
			
			PrimeHasDifferingWidestApertures { .. } => None,
			
			MinimumIsGreaterThanMaximum { .. } => None,
		}
	}
}
