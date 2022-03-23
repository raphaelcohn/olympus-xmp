// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A metering mode.
///
/// Defaults to `Unknown`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ExifMeteringMode
{
	#[allow(missing_docs)]
	Unknown = 0,
	
	#[allow(missing_docs)]
	Average = 1,
	
	#[allow(missing_docs)]
	CenterWeightedAverage = 2,
	
	#[allow(missing_docs)]
	Spot = 3,
	
	#[allow(missing_docs)]
	MultiSpot = 4,
	
	#[allow(missing_docs)]
	Pattern = 5,
	
	#[allow(missing_docs)]
	Partial = 6,
	
	#[allow(missing_docs)]
	Other = 255,
}

impl const Default for ExifMeteringMode
{
	#[inline(always)]
	fn default() -> Self
	{
		ExifMeteringMode::Unknown
	}
}

impl_xmp_attribute_value_parse_transmute_u16!(ExifMeteringMode, ExifMeteringMode, (0 ..= 6 | 255));
