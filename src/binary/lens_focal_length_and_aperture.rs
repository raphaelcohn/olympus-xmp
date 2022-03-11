// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn lens_focal_length_and_aperture<'a>(collated: &mut Collated, Description: &XmpElement<'a, 'static, 'static, 'static>, crop_factor: NonZeroUnsignedTiffRational)
{
	use XmpValidationError::*;
	
	let focal_length =
	{
		let focal_length = collated.validate(Description.get_attribute_or_error::<NonZeroUnsignedTiffRational>(xml_name!(exif, "FocalLength")));
		let focal_length_in_35mm_film = collated.validate(Description.get_attribute_or_error::<Option<NonZeroU16>>(xml_name!(exif, "FocalLengthIn35mmFilm")));
		
		match (focal_length, focal_length_in_35mm_film)
		{
			(Some(focal_length), Some(Some(focal_length_in_35mm_film))) =>
			{
				let expected_focal_length_in_35mm_film = focal_length * crop_factor;
				let actual_focal_length_in_35mm_film = NonZeroUnsignedTiffRational::from(focal_length_in_35mm_film);
				if expected_focal_length_in_35mm_film != actual_focal_length_in_35mm_film
				{
					collated.push(InvalidFocalLengthIn35mmFilmForCropFactor);
				}
				Some(focal_length)
			},
			
			(_, Some(None)) =>
			{
				collated.push(UnknownFocalLengthIn35mmFilm);
				None
			}
			
			_ => None,
		}
	};

	let f_number = collated.validate(Description.get_attribute_or_error::<NonZeroUnsignedTiffRational>(xml_name!(exif, "FNumber")));
	
	if let Some(lens_information) = collated.validate(Description.get_attribute_or_error::<LensInformation>(xml_name!(aux, "LensInfo")))
	{
		if !lens_information.contains_focal_length(focal_length, lens_information)
		{
			collated.push(LensInformationDoesNotContainShotFocalLength);
		}
		if !lens_information.contains_f_number(focal_length, lens_information)
		{
			collated.push(LensInformationDoesNotContainShotFNumber);
		}
	}
	
	// TODO: These aperture values are in APEX, and should correlate to f-number somehow.
	// TODO: exif:ApertureValue="1695994/1000000"
	// TODO: exif:MaxApertureValue="434/256"	=> Smallest (larger number) f-number of the lens.
	//  	logb(x / y) = logb(x) - logb(y)
	// Av=log₂A², where Av is ApertureValue and A is f-number
	// (A²) = 2^Av
	// A = SqRoot(2^Av)
	
	// We can back-calculate from the rounded number, I suppose.
	// https://www.rapidtables.com/math/algebra/Logarithm.html
	xxxxx;
}
