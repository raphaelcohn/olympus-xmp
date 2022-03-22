// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use arrayvec::ArrayVec;
use crate::xml::NotExactlyOneElementError;
use crate::xml::XmlDocument;
use crate::xml::XmlDocumentOrXmlElement;
use crate::xml::XmlElement;
use crate::xml::XmlName;
use attribute_parse_errors::I8ParseError;
use attribute_parse_errors::U8ParseError;
use attribute_parse_errors::U16ParseError;
use attribute_parse_errors::UnknownStringVariantParseError;
use exif::version::ExifVersionParseError;
use exif::lens_information::LensInformationParseError;
use iptc::urgency::UrgencyParseError;
use iso_country::Iso3166Dash1AlphaCountryCodeParseError;
use non_empty_str::NonEmptyStrParseError;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;
use std::str::FromStr;
use tiff_rational::NonZeroUnsignedTiffRationalParseError;
use tiff_rational::UnsignedTiffRationalParseError;
use xmp::date_time::XmpDateTimeParseError;
use xmp::universally_unique_identifier::XmpUniversallyUniqueIdentifier;
use xmp::universally_unique_identifier::XmpUniversallyUniqueIdentifierParseError;


include!("xml_name.rs");


/// Attribute parse errors.
pub mod attribute_parse_errors;


/// Exif domain types.
pub mod exif;


/// International Press Telecommunications Council [IPTC](https://iptc.org/) domain types.
pub mod iptc;


/// ISO country and country code domain types.
pub mod iso_country;


/// Namespace definitions.
pub mod namespaces;


/// Non-empty str domain types.
pub mod non_empty_str;


/// Photoshop.
pub mod photoshop;


/// Picture Licensing Universal System [PLUS](http://ns.useplus.org/LDF/ldf-XMPSpecification) domain types.
pub mod plus;


/// TIFF RATIONAL domain types..
pub mod tiff_rational;


/// XMP domain types.
pub mod xmp;


include!("XmpAttributeValue.rs");
include!("XmpAttributeValueParseError.rs");
include!("XmpElement.rs");
include!("XmpElementPath.rs");
include!("XmpValidationError.rs");