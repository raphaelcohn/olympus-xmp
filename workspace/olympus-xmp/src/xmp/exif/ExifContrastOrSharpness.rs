// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Contrast or sharpness.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ExifContrastOrSharpness
{
	#[allow(missing_docs)]
	Normal = 0,

	#[allow(missing_docs)]
	Soft = 1,
	
	#[allow(missing_docs)]
	Hard = 2,
}

impl const Default for ExifContrastOrSharpness
{
	#[inline(always)]
	fn default() -> Self
	{
		ExifContrastOrSharpness::Normal
	}
}

impl_xmp_attribute_value_parse_transmute_u16!(ExifContrastOrSharpness, ExifContrastOrSharpness, 0 ..= 2);
