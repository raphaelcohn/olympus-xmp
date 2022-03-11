// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::cmp::Ordering;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::Split;
use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;
use super::tiff_rational::NonZeroUnsignedTiffRational;
use super::tiff_rational::NonZeroUnsignedTiffRationalParseError;


include!("LensInformation.rs");
include!("FocalLengthAndWidestAperture.rs");
include!("LensInformationParseError.rs");
