// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::mem::transmute;
use std::str::FromStr;
use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;


/// Flash bit fields.
///
/// In XMP, the EXIF 'Flash' tag's various bits are actually modelled as attributes.
pub mod flash;


/// Lens information.
pub mod lens_information;


/// Exif version.
pub mod version;


include!("ExifContrastOrSharpness.rs");
include!("ExifCustomRendered.rs");
include!("ExifGainControl.rs");
include!("ExifExposureMode.rs");
include!("ExifExposureProgram.rs");
include!("ExifFileSource.rs");
include!("ExifLightSource.rs");
include!("ExifMeteringMode.rs");
include!("ExifResolutionUnit.rs");
include!("ExifSaturation.rs");
include!("ExifSceneCaptureType.rs");
include!("ExifSensitivityType.rs");
include!("ExifWhiteBalanceMode.rs");
