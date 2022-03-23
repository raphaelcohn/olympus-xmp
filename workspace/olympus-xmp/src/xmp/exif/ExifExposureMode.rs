// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Exposure mode.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ExifExposureMode
{
	#[allow(missing_docs)]
	AutomaticExposure = 0,

	#[allow(missing_docs)]
	ManualExposure = 1,
	
	#[allow(missing_docs)]
	AutomaticBracketing = 2,
}

impl_xmp_attribute_value_parse_transmute_u16!(ExifExposureMode, ExifExposureMode, 0 ..= 2);
