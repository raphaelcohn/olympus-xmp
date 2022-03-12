// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::num::NonZeroU16;
use std::num::NonZeroU32;
use olympus_xmp::xml_name;
use olympus_xmp::xml::XmlName;
use olympus_xmp::xmp::XmpElement;
use olympus_xmp::xmp::XmpValidationError;
use olympus_xmp::xmp::lens_information::LensInformation;
use olympus_xmp::xmp::namespaces::aux;
use olympus_xmp::xmp::namespaces::exif;
use olympus_xmp::xmp::namespaces::exifEX;
use olympus_xmp::xmp::namespaces::xmpMM;
use olympus_xmp::xmp::tiff_rational::NonZeroUnsignedTiffRational;
use olympus_xmp::xmp::universally_unique_identifier::XmpUniversallyUniqueIdentifier;


include!("Collated.rs");
include!("document_identifier.rs");
include!("lens_focal_length_and_aperture.rs");
include!("lens_model.rs");
include!("width_or_height.rs");
include!("XmpOutcomeOfValidationError.rs");
