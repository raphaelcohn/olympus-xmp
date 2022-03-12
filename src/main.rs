// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(invalid_html_tags)]
#![deny(macro_use_extern_crate)]
#![deny(missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(pointer_structural_match)]
#![deny(unaligned_references)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![deny(unused_must_use)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_crate_dependencies)]


//! #olympus-xmp
//!
//! This is a rust binary


use arrayvec as _;
use gcd as _;
use memchr as _;
use swiss_army_knife as _;
use xml as _;


use swiss_army_knife::non_zero::new_non_zero_u32;
use olympus_xmp::xml_name;
use olympus_xmp::xml::XmlDocument;
use olympus_xmp::xmp::{ExifGainControl, ExifResolutionUnit, ExifSaturation};
use olympus_xmp::xmp::ExifMeteringMode;
use olympus_xmp::xmp::ExifLightSource;
use olympus_xmp::xmp::ExifCustomRendered;
use olympus_xmp::xmp::ExifContrastOrSharpness;
use olympus_xmp::xmp::ExifExposureMode;
use olympus_xmp::xmp::ExifExposureProgram;
use olympus_xmp::xmp::ExifSensitivityType;
use olympus_xmp::xmp::ExifFileSource;
use olympus_xmp::xmp::ExifSceneCaptureType;
use olympus_xmp::xmp::IptcDigitalSourceType;
use olympus_xmp::xmp::PlusModelReleaseStatus;
use olympus_xmp::xmp::PlusPropertyReleaseStatus;
use olympus_xmp::xmp::XmpElement;
use olympus_xmp::xmp::XmpValidationError;
use olympus_xmp::xmp::namespaces::Iptc4xmpCore;
use olympus_xmp::xmp::namespaces::Iptc4xmpExt;
use olympus_xmp::xmp::namespaces::aux;
use olympus_xmp::xmp::namespaces::dc;
use olympus_xmp::xmp::namespaces::exif;
use olympus_xmp::xmp::namespaces::photoshop;
use olympus_xmp::xmp::namespaces::plus;
use olympus_xmp::xmp::namespaces::rdf;
use olympus_xmp::xmp::namespaces::tiff;
use olympus_xmp::xmp::namespaces::x;
use olympus_xmp::xmp::namespaces::xmp;
use olympus_xmp::xmp::namespaces::xmpRights;
use olympus_xmp::xmp::PhotoshopColorMode;
use olympus_xmp::xmp::XmpLabel;
use olympus_xmp::xmp::XmpRating;
use olympus_xmp::xmp::date_time::XmpDateTime;
use olympus_xmp::xmp::exif_version::ExifVersion;
use olympus_xmp::xmp::iso_country::Iso3166Dash1AlphaCountryCode;
use olympus_xmp::xmp::iso_country::Iso3166Dash1Country;
use olympus_xmp::xmp::non_empty_str::NonEmptyStr;
use olympus_xmp::xmp::tiff_rational::NonZeroUnsignedTiffRational;
use olympus_xmp::xmp::tiff_rational::UnsignedTiffRational;
use olympus_xmp::xmp::universally_unique_identifier::XmpUniversallyUniqueIdentifier;
use olympus_xmp::xmp::urgency::Urgency;
use crate::binary::lens_model;
use crate::binary::lens_focal_length_and_aperture;
use crate::binary::width_or_height;
use crate::binary::document_identifier;
use crate::binary::Collated;
use crate::binary::XmpOutcomeOfValidationError;


mod binary;


fn main()
{
	let path = "/path/to/file.xml";
	let (xml_document, file) = XmlDocument::parse_path(path, true).unwrap();
	validate(&xml_document).unwrap();
	xml_document.write_file(file).unwrap();
}

fn validate(xml_document: &XmlDocument) -> Result<(), XmpOutcomeOfValidationError<'static, 'static, 'static>>
{
	use XmpValidationError::*;
	
	let xmpmeta = XmpOutcomeOfValidationError::fundamental(XmpElement::root(&xml_document, xml_name!(x, "xmpmeta")))?;
	
	let Description =
	{
		let RDF = XmpOutcomeOfValidationError::fundamental(xmpmeta.child(xml_name!(rdf, "RDF")))?;
		XmpOutcomeOfValidationError::fundamental(RDF.child(xml_name!(rdf, "Description")))?
	};
	
	if Description.get_attribute_str(xml_name!(photoshop, "SidecarForExtension")) != Some("ORF")
	{
		return Ok(())
	}
	
	let mut collated = Collated::default();
	
	// TODO: Read this first, as some fields will not be present; interprete 2.1, 2.2, 2.3x appropriately (tiff, exif, exifEX, aux).
	collated.check(Description.has_attribute_with_expected_value::<ExifVersion>(xml_name!(exif, "ExifVersion"), ExifVersion::Version_2_30));
	
	// TODO: Use thse to set Iptc4xmpExt:MaxAvailWidth and Iptc4xmpExt:MaxAvailHeight.
	let pixel_x = width_or_height(&mut collated, &Description, xml_name!(tiff, "ImageLength"), xml_name!(exif, "PixelXDimension"), TiffWidthDoesNotMatchExifWidth);
	let pixel_y = width_or_height(&mut collated, &Description, xml_name!(tiff, "ImageWidth"), xml_name!(exif, "PixelYDimension"), TiffHeightDoesNotMatchExifHeight);
	
	let document_identifier = document_identifier(&mut collated, &Description);
	
	/*
		TODO: For Canon 200mm, add in.
		   exifEX:LensModel="CANON FD 200mm F2.8"
		   aux:Lens="CANON FD 200mm F2.8"
		   aux:LensSerialNumber="33574"
		   aux:LensInfo="200/1 200/1 28/10 28/10"
		   exif:FNumber="28/10" ??
		   exif:FocalLength="200"
		   exif:FocalLengthIn35mmFilm="400"
	 */
	collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(aux, "LensSerialNumber")));
	let lens_model = lens_model(&mut collated, &Description);
	let MicroFourThirdsCropFactor: NonZeroUnsignedTiffRational = NonZeroUnsignedTiffRational::from(new_non_zero_u32(2));
	lens_focal_length_and_aperture(&mut collated, &Description, MicroFourThirdsCropFactor);
	
	let photoshop_created_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(photoshop, "DateCreated")));
	let xmp_create_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(xmp, "CreateDate")));
	let xmp_modify_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(xmp, "ModifyDate")));
	let xmp_metadata_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(xmp, "MetadataDate")));
	let exif_date_original = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(exif, "DateTimeOriginal")));
	
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "Artist")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "Copyright")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "DateTime")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "ImageDescription")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "Software")));
	collated.check(Description.does_not_have_attribute(xml_name!(exif, "DateTimeDigitized")));
	// TODO xmpDM:* xmpTPg:*, which in xmpMM?
	// TODO photoshop:SupplementalCategories (set of category codes; legacy; not sure how codes are separated)
	// TODO photoshop:Category (could be parsed; 3 ASCII characters; legacy)
	collated.check(Description.get_attribute::<XmpRating>(xml_name!(xmp, "Rating")));
	collated.check(Description.get_attribute::<XmpLabel>(xml_name!(xmp, "Label")));
	let _ = collated.validate(Description.get_attribute::<ExifMeteringMode>(xml_name!(exif, "MeteringMode"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifLightSource>(xml_name!(exif, "LightSource"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifSaturation>(xml_name!(exif, "Saturation"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifExposureProgram>(xml_name!(exif, "ExposureProgram"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifContrastOrSharpness>(xml_name!(exif, "Contrast"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifContrastOrSharpness>(xml_name!(exif, "Sharpness"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifSceneCaptureType>(xml_name!(exif, "SceneCaptureType"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifFileSource>(xml_name!(exif, "FileSource"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifGainControl>(xml_name!(exif, "GainControl"))).unwrap_or_default();
	let _ = collated.validate(Description.get_attribute::<ExifResolutionUnit>(xml_name!(exif, "FocalPlaneResolutionUnit"))).unwrap_or_default();
	collated.check(xmpmeta.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(x, "xmptk"))); // TODO: Parse this value?
	collated.check(Description.has_attribute_with_any_value::<ExifCustomRendered>(xml_name!(exif, "CustomRendered")));
	collated.check(Description.has_attribute_with_any_value::<NonZeroUnsignedTiffRational>(xml_name!(exif, "FocalPlaneXResolution")));
	collated.check(Description.has_attribute_with_any_value::<NonZeroUnsignedTiffRational>(xml_name!(exif, "FocalPlaneYResolution")));
	collated.check(Description.has_attribute_with_any_value::<NonZeroUnsignedTiffRational>(xml_name!(exif, "ExposureTime")));
	collated.check(Description.has_attribute_with_any_value::<ExifExposureMode>(xml_name!(exif, "ExposureMode")));
	collated.check(Description.has_attribute_with_any_value::<ExifSensitivityType>(xml_name!(exif, "SensitivityType")));
	collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(xmp, "CreatorTool")));
	collated.check(Description.has_attribute_with_any_value::<Urgency>(xml_name!(photoshop, "Urgency")));
	collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(tiff, "Make")));
	collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(tiff, "Model")));
	collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(aux, "SerialNumber"))); // Body serial number.
	collated.check(Description.has_attribute_with_any_value::<UnsignedTiffRational>(xml_name!(aux, "FlashCompensation")));
	collated.check(Description.has_attribute_with_any_value::<UnsignedTiffRational>(xml_name!(exif, "ExifWhiteBalanceMode")));
	collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(dc, "format"), NonEmptyStr("image/x-olympus-raw")));
	collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(rdf, "about"), ""));
	collated.check(Description.has_attribute_with_expected_value::<PhotoshopColorMode>(xml_name!(photoshop, "ColorMode"), PhotoshopColorMode::RgbColor));
	collated.check(Description.has_attribute_with_expected_value::<XmpUniversallyUniqueIdentifier>(xml_name!(photoshop, "EmbeddedXMPDigest"), XmpUniversallyUniqueIdentifier::Zero));
	collated.check(Description.has_attribute_with_expected_value::<bool>(xml_name!(xmpRights, "Marked"), true));
	collated.check(Description.has_attribute_with_expected_value::<IptcDigitalSourceType>(xml_name!(Iptc4xmpExt, "DigitalSourceType"), IptcDigitalSourceType::OriginalDigitalCapture));

	// TODO: Overwrite with location shown data.
	{
		collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "Location")));
		collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(photoshop, "City")));
		collated.check(Description.has_attribute_with_any_value::<NonEmptyStr>(xml_name!(photoshop, "State")));
		collated.check(Description.has_attribute_with_any_value::<Iso3166Dash1Country>(xml_name!(photoshop, "Country")));
		collated.check(Description.has_attribute_with_any_value::<Iso3166Dash1AlphaCountryCode>(xml_name!(Iptc4xmpCore, "CountryCode")));
	}
	
	// Not so much checks as data that should be present, or, for creator details, should be complete.
	{
		const PhotographerProperName: NonEmptyStr = NonEmptyStr("Raphael James Cohn");
		collated.check(Description.has_attribute_with_expected_value::<PlusPropertyReleaseStatus>(xml_name!(plus, "PropertyReleaseStatus"), PlusPropertyReleaseStatus::NotApplicable));
		collated.check(Description.has_attribute_with_expected_value::<PlusModelReleaseStatus>(xml_name!(plus, "ModelReleaseStatus"), PlusModelReleaseStatus::NotApplicable));
		collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(photoshop, "AuthorsPosition"), NonEmptyStr("Photographer")));
		collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(photoshop, "Credit"), PhotographerProperName));
		collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(photoshop, "Source"), PhotographerProperName));
		collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(photoshop, "CaptionWriter"), PhotographerProperName));
		
		// TODO: Create this from make and model?
		collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(photoshop, "Instructions"), NonEmptyStr("Original RAW capture Olympus E-PL8")));
		
		if let Some(CreatorContactInfo) = collated.validate(Description.child(xml_name!(Iptc4xmpCore, "CreatorContactInfo")))
		{
			collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "CiAdrExtadr"), NonEmptyStr("6 Eller Mews")));
			collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "CiAdrCity"), NonEmptyStr("Skipton")));
			collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "CiAdrRegion"), NonEmptyStr("North Yorkshire")));
			collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "CiAdrPcode"), NonEmptyStr("BD23 2TG")));
			collated.check(Description.has_attribute_with_expected_value::<Iso3166Dash1Country>(xml_name!(Iptc4xmpCore, "CiAdrCtry"), Iso3166Dash1Country::UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND));
			collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "CiTelWork"), NonEmptyStr("+44 7590 675 756"))); // TODO: Normalise this without spaces.
			collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "CiEmailWork"), NonEmptyStr("raphael.cohn@stormmq.com")));
			collated.check(Description.has_attribute_with_expected_value::<NonEmptyStr>(xml_name!(Iptc4xmpCore, "CiUrlWork"), NonEmptyStr("https://photos.stormmq.com/")));
		}
	}
	
	// TODO: Seq + li with text
	// TODO: Alt + xml:lang with text
	// TODO: Bag + li with properties but no text
	// TODO: Should not be present photoshop:TextLayers
	
   /*
   
   // TODO: Default this.
   photoshop:ICCProfile=""
   
   // TODO: Verify this is a lower-case hyphen separated UUID.
   xmpMM:InstanceID="xmp.iid:22b0d4af-ade9-4d85-a82d-80f0048494fa"
   
   TODO: Overwrite this using LocationShown's first entry.
   Iptc4xmpCore:Location="Addingham Churchyard"
   photoshop:City="Addingham"
   photoshop:State="North Yorkshire"
   photoshop:Country="United Kingdom of Great Britain and Northern Ireland (the)" #TODO: Use the official ISO name here
   Iptc4xmpCore:CountryCode="GBR"
   
	
	// TODO: To do the comparison, round to two decimal places; consider expressing f-number roundings to 1 dp if above 1 and 2 dp if below 1 (eg f/0.95).
	// This is similar to the 'normal' accuracy used in Exif.
	
	// TODO: These aperture values are in APEX, and correlate to f-number.
	// TODO: exif:ApertureValue="1695994/1000000"
	// TODO: exif:MaxApertureValue="434/256"	=> Smallest (larger number) f-number of the lens.
	//  	logb(x / y) = logb(x) - logb(y)
	// Av=log₂A², where Av is ApertureValue and A is f-number
	// (A²) = 2^Av
	// A = SqRoot(2^Av)
	
	// We can back-calculate from the rounded number, I suppose.
	// https://www.rapidtables.com/math/algebra/Logarithm.html
	xxxxx;
   
   exif:ApertureValue="1695994/1000000"
   Av=log₂A², where Av is ApertureValue and A is f-number
   
   exif:MaxApertureValue="434/256" 			=> Smallest (larger number) f-number of the lens as RATIONAL APEX value.
   
   exif:ShutterSpeedValue="11965784/1000000"	SRATIONAL APEX	Numerator of 0xFFFF_FFFF means infinity for some fields; 0 means unknown for some fields (surely this should be impossible for this particular field).
   ShutterSpeedValue = -log₂(exif:ExposureTime).
   
   exif:ExposureBiasValue="0/10"	SRATIONAL APEX
   
   exif:DigitalZoomRatio="100/100"	RATIONAL	numerator=0 => digital zoom not used
   
   <dc:creator>
   <rdf:Seq>
   <rdf:li>Raphael James Cohn</rdf:li>
   </rdf:Seq>
   </dc:creator>
   
   <dc:rights>
   <rdf:Alt>
   <rdf:li xml:lang="x-default">Copyright © 2021 Raphael James Cohn, all rights reserved</rdf:li>
   </rdf:Alt>
   </dc:rights>
   
   <Iptc4xmpExt:LocationCreated>
    <rdf:Bag>
     <rdf:li
      Iptc4xmpExt:ProvinceState="North Yorkshire"
      Iptc4xmpExt:CountryName="United Kingdom of Great Britain and Northern Ireland (the)"
      Iptc4xmpExt:CountryCode="GBR" TODO: Validate this is an ISO 3-digit code [2 digit is permitted]; there are also some non-standard extension codes eg for england
      Iptc4xmpExt:WorldRegion="Europe"
      Iptc4xmpExt:Sublocation="Addingham Churchyard"
      Iptc4xmpExt:City="Addingham"/>
    </rdf:Bag>
   </Iptc4xmpExt:LocationCreated>
   
   <Iptc4xmpExt:LocationShown>
    <rdf:Bag>
     <rdf:li
      Iptc4xmpExt:Sublocation="Addingham Churchyard"
      Iptc4xmpExt:City="Addingham"
      Iptc4xmpExt:ProvinceState="North Yorkshire"
      Iptc4xmpExt:CountryName="United Kingdom of Great Britain and Northern Ireland (the)"
      Iptc4xmpExt:CountryCode="GBR" (2 or 3 digit is permitted)
      Iptc4xmpExt:WorldRegion="Europe"/> (Type done)
    </rdf:Bag>
   </Iptc4xmpExt:LocationShown>
   
   <Iptc4xmpExt:Event>
    <rdf:Alt>
     <rdf:li xml:lang="x-default">Addingham Churchyard 5th April 2021</rdf:li>
    </rdf:Alt>
   </Iptc4xmpExt:Event>
   
   <Iptc4xmpExt:RegistryId>
    <rdf:Bag>
     <rdf:li
      Iptc4xmpExt:RegOrgId="raphael.cohn@stormmq.com"/>
    </rdf:Bag>
   </Iptc4xmpExt:RegistryId>
   
   <plus:ImageCreator>
    <rdf:Seq>
     <rdf:li
      plus:ImageCreatorName="Raphael James Cohn"
      plus:ImageCreatorID="raphael.cohn@stormmq.com"/>
    </rdf:Seq>
   </plus:ImageCreator>
   
   <plus:CopyrightOwner>
    <rdf:Seq>
     <rdf:li
      plus:CopyrightOwnerName="Raphael James Cohn"
      plus:CopyrightOwnerID="raphael.cohn@stormmq.com"/>
    </rdf:Seq>
   </plus:CopyrightOwner>
   
   <plus:Licensor>
    <rdf:Seq>
     <rdf:li
      plus:LicensorName="Raphael James Cohn"
      plus:LicensorID="raphael.cohn@stormmq.com"
      plus:LicensorTelephoneType1="http://ns.useplus.org/ldf/vocab/cell"
      plus:LicensorTelephone1="+44 7590 675 756"
      plus:LicensorTelephoneType2="http://ns.useplus.org/ldf/vocab/cell"
      plus:LicensorTelephone2="+44 7590 675 756"
      plus:LicensorEmail="raphael.cohn@stormmq.com"
      plus:LicensorURL="https://photos.stormmq.com/"/>
    </rdf:Seq>
   </plus:Licensor>
   
   <plus:ImageSupplier>
    <rdf:Seq>
     <rdf:li
      plus:ImageSupplierName="Raphael James Cohn"
      plus:ImageSupplierID="raphael.cohn@stormmq.com"/>
    </rdf:Seq>
   </plus:ImageSupplier>
   
   
    http://ns.useplus.org/ldf/vocab/PR-NON (None)
    http://ns.useplus.org/ldf/vocab/PR-NAP (Not Applicable)
    http://ns.useplus.org/ldf/vocab/PR-UPR (Unlimited Property Releases)
    http://ns.useplus.org/ldf/vocab/PR-LPR (Limited or Incomplete Property Releases)

    http://ns.useplus.org/ldf/vocab/MR-NON (None)
    http://ns.useplus.org/ldf/vocab/MR-NAP (Not Applicable)
    http://ns.useplus.org/ldf/vocab/MR-UMR (Unlimited Model Releases)
    http://ns.useplus.org/ldf/vocab/MR-LMR (Limited or Incomplete Model Releases)
    
    http://ns.useplus.org/ldf/vocab/AG-UNK (Age Unknown)
    http://ns.useplus.org/ldf/vocab/AG-A25 (Age 25 or Over)
    http://ns.useplus.org/ldf/vocab/AG-A24 (Age 24)
    http://ns.useplus.org/ldf/vocab/AG-A23 (Age 23)
    http://ns.useplus.org/ldf/vocab/AG-A22 (Age 22)
    http://ns.useplus.org/ldf/vocab/AG-A21 (Age 21)
    http://ns.useplus.org/ldf/vocab/AG-A20 (Age 20)
    http://ns.useplus.org/ldf/vocab/AG-A19 (Age 19)
    http://ns.useplus.org/ldf/vocab/AG-A18 (Age 18)
    http://ns.useplus.org/ldf/vocab/AG-A17 (Age 17)
    http://ns.useplus.org/ldf/vocab/AG-A16 (Age 16)
    http://ns.useplus.org/ldf/vocab/AG-A15 (Age 15)
    http://ns.useplus.org/ldf/vocab/AG-U14 (Age 14 or Under)
    
    

    http://ns.useplus.org/ldf/vocab/work (work)
    http://ns.useplus.org/ldf/vocab/cell (cell)
    http://ns.useplus.org/ldf/vocab/fax (fax)
    http://ns.useplus.org/ldf/vocab/home (home)
    http://ns.useplus.org/ldf/vocab/pager (pager)


	 */
	
	Ok(())
}
