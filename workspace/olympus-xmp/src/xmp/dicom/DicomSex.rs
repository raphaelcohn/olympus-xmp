// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Sex.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DicomSex
{
	/// This is stored as "male" im XMP and "M" in DICOM.
	Male,
	
	/// This is stored as "female" im XMP and "F" in DICOM.
	Female,
}

impl_xmp_attribute_value_parse_str!
(
	DicomSex, DicomSex,
	
	"male" => Male,

	"female" => Female,
);
