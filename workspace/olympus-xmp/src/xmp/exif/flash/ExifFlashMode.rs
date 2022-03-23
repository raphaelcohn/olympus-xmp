// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Values are 0 to 3 inclusive as they represent a 2-bit value.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ExifFlashMode
{
	#[allow(missing_docs)]
	Unknown = 0b00,
	
	#[allow(missing_docs)]
	CompulsoryFlashFiring = 0b01,
	
	#[allow(missing_docs)]
	CompulsoryFlashSuppression = 0b10,
	
	#[allow(missing_docs)]
	Automatic = 0b11,
}

impl_xmp_attribute_value_parse_transmute_u8!(ExifFlashMode, ExifFlashMode, 0 ..= 3);
