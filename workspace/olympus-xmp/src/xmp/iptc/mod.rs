// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use super::attribute_parse_errors::UnknownStringVariantParseError;
use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;


include!("IptcDigitalSourceType.rs");
include!("IptcWorldRegion.rs");


/// Legacy urgency support.
pub mod urgency;
