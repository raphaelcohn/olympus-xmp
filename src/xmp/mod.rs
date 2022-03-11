// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use arrayvec::ArrayVec;
use crate::xml::NotExactlyOneElementError;
use crate::xml::XmlDocument;
use crate::xml::XmlDocumentOrXmlElement;
use crate::xml::XmlElement;
use crate::xml::XmlName;
use date_time::XmpDateTimeParseError;
use lens_information::LensInformationParseError;
use memchr::memrchr;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::num::ParseIntError;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;
use std::str::FromStr;
use std::str::from_utf8_unchecked;
use swiss_army_knife::get_unchecked::GetUnchecked;
use tiff_rational::NonZeroUnsignedTiffRationalParseError;
use tiff_rational::UnsignedTiffRationalParseError;
use universally_unique_identifier::XmpUniversallyUniqueIdentifier;
use universally_unique_identifier::XmpUniversallyUniqueIdentifierParseError;
use urgency::UrgencyParseError;


include!("xml_name.rs");


/// Date definition.
pub mod date_time;


/// Lens information.
pub mod lens_information;


/// Namespace definitions.
pub mod namespaces;


/// TIFF RATIONAL definition.
pub mod tiff_rational;


/// Legacy urgency support.
pub mod urgency;


/// Universally Unique Identifiers (UUID).
pub mod universally_unique_identifier;


include!("ExifSceneCaptureType.rs");
include!("I8ParseError.rs");
include!("IptcDigitalSourceType.rs");
include!("PhotoshopColorMode.rs");
include!("PlusModelReleaseStatus.rs");
include!("PlusPropertyReleaseStatus.rs");
include!("U8ParseError.rs");
include!("UnknownStringVariantParseError.rs");
include!("XmpAttributeValue.rs");
include!("XmpAttributeValueParseError.rs");
include!("XmpElement.rs");
include!("XmpElementPath.rs");
include!("XmpLabel.rs");
include!("XmpRating.rs");
include!("XmpValidationError.rs");
