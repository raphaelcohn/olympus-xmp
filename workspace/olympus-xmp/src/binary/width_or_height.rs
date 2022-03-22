// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn width_or_height<'a>(collated: &mut Collated, Description: &XmpElement<'a, 'static, 'static, 'static>, tiff_attribute: &'static XmlName<'static, 'static>, exif_attribute: &'static XmlName<'static, 'static>, error: impl FnOnce(NonZeroU32, NonZeroU32) -> XmpValidationError<'static, 'static, 'static>) -> Option<NonZeroU32>
{
	let tiff_dimension_value = collated.validate(Description.get_attribute_or_error::<NonZeroU32>(tiff_attribute));
	let exif_dimension_value = collated.validate(Description.get_attribute_or_error::<NonZeroU32>(exif_attribute));
	
	match (tiff_dimension_value, exif_dimension_value)
	{
		(Some(tiff_dimension_value), Some(exif_dimension_value)) => if tiff_dimension_value == exif_dimension_value
		{
			Some(tiff_dimension_value)
		}
		else
		{
			collated.push(error(tiff_dimension_value, exif_dimension_value));
			None
		}
		
		_ => None,
	}
}
