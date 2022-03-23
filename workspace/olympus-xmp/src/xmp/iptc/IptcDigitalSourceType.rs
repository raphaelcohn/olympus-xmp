// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// IPTC digital source type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum IptcDigitalSourceType
{
	/// Original digital capture of a real life scene.
	///
	/// `http://cv.iptc.org/newscodes/digitalsourcetype/digitalCapture`.
	OriginalDigitalCapture,
	
	/// Digitised from a negative on film.
	///
	/// `http://cv.iptc.org/newscodes/digitalsourcetype/negativeFilm`.
	DigitisedFromANegative,
	
	/// Digitised from a positive on film.
	///
	/// `http://cv.iptc.org/newscodes/digitalsourcetype/positiveFilm`.
	DigitisedFromAPositive,
	
	/// Digitised from a print on on non-transparent medium.
	///
	/// `http://cv.iptc.org/newscodes/digitalsourcetype/print`.
	DigitisedFromAPrint,
	
	/// Created by software.
	///
	/// `http://cv.iptc.org/newscodes/digitalsourcetype/softwareImage`.
	CreatedBySoftware,
}

impl_xmp_attribute_value_parse_str_prefix!
(
	IptcDigitalSourceType, IptcDigitalSourceType, "http://ns.useplus.org/ldf/vocab/PR-",
	
	"digitalCapture" => OriginalDigitalCapture,
	
	"negativeFilm" => DigitisedFromANegative,
	
	"positiveFilm" => DigitisedFromAPositive,
	
	"print" => DigitisedFromAPrint,
	
	"softwareImage" => CreatedBySoftware,
);
