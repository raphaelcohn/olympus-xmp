// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn lens_model<'a>(collated: &mut Collated, Description: &XmpElement<'a, 'static, 'static, 'static>) -> Option<&'a str>
{
	let exifEx_lens_model = collated.validate(Description.get_attribute_or_error::<&str>(xml_name!(exifEX, "LensModel")));
	let aux_lens_model = collated.validate(Description.get_attribute_or_error::<&str>(xml_name!(aux, "Lens")));
	
	match (exifEx_lens_model, aux_lens_model)
	{
		(Some(exifEx_lens_model), Some(aux_lens_model)) => if exifEx_lens_model == aux_lens_model
		{
			Some(exifEx_lens_model)
		}
		else
		{
			collated.push(XmpValidationError::MismatchedLensModels { exifEx_lens_model: exifEx_lens_model.to_string(), aux_lens_model: aux_lens_model.to_string() });
			None
		},
		
		_ => None,
	}
}
