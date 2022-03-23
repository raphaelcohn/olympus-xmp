// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A scene capture type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ExifSceneCaptureType
{
	#[allow(missing_docs)]
	Standard = 0,

	#[allow(missing_docs)]
	Landscape = 1,

	#[allow(missing_docs)]
	Portrait = 2,

	#[allow(missing_docs)]
	Night = 3,
}

impl const Default for ExifSceneCaptureType
{
	#[inline(always)]
	fn default() -> Self
	{
		ExifSceneCaptureType::Standard
	}
}

impl_xmp_attribute_value_parse_transmute_u16!(ExifSceneCaptureType, ExifSceneCaptureType, 0 ..= 3);
