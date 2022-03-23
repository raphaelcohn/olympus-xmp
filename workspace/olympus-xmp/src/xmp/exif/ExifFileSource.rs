// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A file source.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ExifFileSource
{
	#[allow(missing_docs)]
	Others = 0,
	
	#[allow(missing_docs)]
	ScannerOfTransparentType = 1,
	
	#[allow(missing_docs)]
	ScannerOfReflexType = 2,
	
	/// Digital Still Camera (DSC).
	DigitalStillCamera = 3,
}

impl const Default for ExifFileSource
{
	#[inline(always)]
	fn default() -> Self
	{
		ExifFileSource::DigitalStillCamera
	}
}

impl_xmp_attribute_value_parse_transmute_u8!(ExifFileSource, ExifFileSource, 0 ..= 3);
