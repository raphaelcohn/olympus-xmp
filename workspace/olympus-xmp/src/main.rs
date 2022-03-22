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
use xml as _;


use iso_3166_1_country::Iso3166Dash1AlphaCountryCode;
use iso_3166_1_country::Iso3166Dash1Country;
use olympus_xmp::xml_name;
use olympus_xmp::xml::XmlDocument;
use olympus_xmp::xmp::exif::ExifGainControl;
use olympus_xmp::xmp::exif::ExifResolutionUnit;
use olympus_xmp::xmp::exif::ExifSaturation;
use olympus_xmp::xmp::exif::ExifMeteringMode;
use olympus_xmp::xmp::exif::ExifLightSource;
use olympus_xmp::xmp::exif::ExifCustomRendered;
use olympus_xmp::xmp::exif::ExifContrastOrSharpness;
use olympus_xmp::xmp::exif::ExifExposureMode;
use olympus_xmp::xmp::exif::ExifExposureProgram;
use olympus_xmp::xmp::exif::ExifSensitivityType;
use olympus_xmp::xmp::exif::ExifFileSource;
use olympus_xmp::xmp::exif::ExifSceneCaptureType;
use olympus_xmp::xmp::exif::version::ExifVersion;
use olympus_xmp::xmp::iptc::{IimCategoryCode, IimSupplementalCategories, IptcDigitalSourceType};
use olympus_xmp::xmp::iptc::urgency::Urgency;
use olympus_xmp::xmp::plus::PlusModelReleaseStatus;
use olympus_xmp::xmp::plus::PlusPropertyReleaseStatus;
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
use olympus_xmp::xmp::namespaces::xml;
use olympus_xmp::xmp::namespaces::xmp;
use olympus_xmp::xmp::namespaces::xmpDM;
use olympus_xmp::xmp::namespaces::xmpMM;
use olympus_xmp::xmp::namespaces::xmpRights;
use olympus_xmp::xmp::namespaces::xmpTPg;
use olympus_xmp::xmp::non_empty_str::NonEmptyStr;
use olympus_xmp::xmp::photoshop::PhotoshopColorMode;
use olympus_xmp::xmp::tiff_rational::NonZeroUnsignedTiffRational;
use olympus_xmp::xmp::tiff_rational::UnsignedTiffRational;
use olympus_xmp::xmp::xmp::XmpLabel;
use olympus_xmp::xmp::xmp::XmpRating;
use olympus_xmp::xmp::xmp::date_time::XmpDateTime;
use olympus_xmp::xmp::xmp::universally_unique_identifier::XmpUniversallyUniqueIdentifier;
use crate::binary::lens_model;
use crate::binary::lens_focal_length_and_aperture;
use crate::binary::width_or_height;
use crate::binary::document_identifier;
use crate::binary::Collated;
use crate::binary::XmpOutcomeOfValidationError;
use swiss_army_knife::non_zero::new_non_zero_u32;


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
	
	// TODO: Root should only have the namespace x and only the attribute xmptk
	// TODO: RDF should have no attributes and only the namespaces x and rdf (rdf defined directly)
	// TODO: Description should have (or have added) the namespaces:-
	// tiff, exif, dc, xmp, aux, exifEX, photoshop, xmpMM, stEvt, crd, xmpRights, Iptc4xmpExt, plus, Iptc4xmpCore, lr
	// Also DICOM and ?crs
	// TODO: Strip unused namespaces.
	
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
	
	collated.check(Description.does_not_have_attribute(xml_name!(xml, "lang")));
	
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "Artist")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "Copyright")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "DateTime")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "ImageDescription")));
	collated.check(Description.does_not_have_attribute(xml_name!(tiff, "Software")));
	collated.check(Description.does_not_have_attribute(xml_name!(exif, "DateTimeDigitized")));
	
	// TODO: Check XML structures are not present for xmpMM::Pantry and xmpMM::Versions
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "DerivedFrom")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "Ingredients")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "ManagedFrom")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "Manager")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "ManageTo")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "ManageUI")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "ManagerVariant")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "RenditionClass"))); // can be defaulted.
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "RenditionParams")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "VersionID")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "RenditionOf")));
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "LastURL"))); // strip if present.
	collated.check(Description.does_not_have_attribute(xml_name!(xmpMM, "SaveID"))); // strip if present.
	
	collated.check(Description.does_not_have_any_attributes_in_namespace::<"xmpDM">(Some(xmpDM)));
	collated.check(Description.does_not_have_any_attributes_in_namespace::<"xmpTPg">(Some(xmpTPg)));
	
	collated.check(Description.does_not_have_attribute_which_is_obsolete::<IimCategoryCode>(xml_name!(photoshop, "Category")));
	collated.check(Description.does_not_have_attribute_which_is_obsolete::<IimSupplementalCategories>(xml_name!(photoshop, "SupplementalCategories")));
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
   
   xmptk
   "Adobe XMP Core 7.0-c000 1.000000, 0000/00/00-00:00:00        "
   - seems to be a name, a version number, a revision, another version number, a date time stamp and a lot of spaces...
   
   Name				Adobe XMP Core			Followed by " "
   Version			7.0-c0000				Followed by " "
   Second Version	1.000000				Followed by ", "
   Timestamp		0000/00/00-00:00:00		YYYY/MM/DD-HH:MM:SS		Invalid date-time stamp
   
   "Adobe XMP Core 5.6-c017 91.164464, 2020/06/15-10:20:05 "
   
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
   
   TODO: Validate this
   <tiff:BitsPerSample>
    <rdf:Seq>
     <rdf:li>16</rdf:li>
     <rdf:li>16</rdf:li>
     <rdf:li>16</rdf:li>
    </rdf:Seq>
   </tiff:BitsPerSample>
   
   TODO: Validate this
   <exif:ISOSpeedRatings>
    <rdf:Seq>
     <rdf:li>200</rdf:li>
    </rdf:Seq>
   </exif:ISOSpeedRatings>
   
   TODO: Validate this
   <exif:Flash
    exif:Fired="False"
    exif:Return="0"
    exif:Mode="1"
    exif:Function="False"
    exif:RedEyeMode="False"/>
    
   TODO: Validate this
   <dc:creator>
   <rdf:Seq>
   <rdf:li>Raphael James Cohn</rdf:li>
   </rdf:Seq>
   </dc:creator>
   
   TODO: Validate this
   <dc:rights>
   <rdf:Alt>
   <rdf:li xml:lang="x-default">Copyright © 2021 Raphael James Cohn, all rights reserved</rdf:li>
   </rdf:Alt>
   </dc:rights>
   
   TODO: Validate this
   <dc:subject>
    <rdf:Bag>
     <rdf:li>Runswick Bay</rdf:li>
     <rdf:li>Boat Houses</rdf:li>
    </rdf:Bag>
   </dc:subject>
   
   TODO: Validate this
   <xmpMM:History>
    <rdf:Seq>
     <rdf:li
      stEvt:action="saved"
      stEvt:instanceID="xmp.iid:eede698f-798c-4257-a8b3-44571c1902a8"
      stEvt:when="2022-02-05T09:09Z"
      stEvt:softwareAgent="Adobe Photoshop Camera Raw 14.1"
      stEvt:changed="/metadata"/>
     <rdf:li
      stEvt:action="saved"
      stEvt:instanceID="xmp.iid:e21382d1-c016-4b24-b2b4-06b2ae7a402f"
      stEvt:when="2022-02-23T08:28:26Z"
      stEvt:softwareAgent="Adobe Photoshop Camera Raw 14.1 (Macintosh)"
      stEvt:changed="/metadata"/>
    </rdf:Seq>
   </xmpMM:History>
   
   TODO: Validate this
   <xmpRights:UsageTerms>
    <rdf:Alt>
     <rdf:li xml:lang="x-default">For consideration only, no reproduction without prior permission</rdf:li>
    </rdf:Alt>
   </xmpRights:UsageTerms>
   
   TODO: Validate this
   <Iptc4xmpExt:LocationCreated>
    <rdf:Bag>
     <rdf:li
      Iptc4xmpExt:ProvinceState="North Yorkshire"
      Iptc4xmpExt:CountryName="United Kingdom of Great Britain and Northern Ireland (the)"
      Iptc4xmpExt:CountryCode="GBR"
      Iptc4xmpExt:WorldRegion="Europe"
      Iptc4xmpExt:Sublocation="Addingham Churchyard"
      Iptc4xmpExt:City="Addingham"/>
    </rdf:Bag>
   </Iptc4xmpExt:LocationCreated>
   
   TODO: Validate this
   <Iptc4xmpExt:LocationShown>
    <rdf:Bag>
     <rdf:li
      Iptc4xmpExt:Sublocation="Addingham Churchyard"
      Iptc4xmpExt:City="Addingham"
      Iptc4xmpExt:ProvinceState="North Yorkshire"	TODO: UK province validator?
      Iptc4xmpExt:CountryName="United Kingdom of Great Britain and Northern Ireland (the)"	DONE ISO 3166 parser
      Iptc4xmpExt:CountryCode="GBR" DONE: ISO 3166 parser
      Iptc4xmpExt:WorldRegion="Europe"/>	DONE: region parser
    </rdf:Bag>
   </Iptc4xmpExt:LocationShown>
   
   TODO: Validate this
   <Iptc4xmpExt:Event>
    <rdf:Alt>
     <rdf:li xml:lang="x-default">Addingham Churchyard 5th April 2021</rdf:li>
    </rdf:Alt>
   </Iptc4xmpExt:Event>
   
   TODO: Validate this
   <Iptc4xmpExt:RegistryId>
    <rdf:Bag>
     <rdf:li
      Iptc4xmpExt:RegOrgId="raphael.cohn@stormmq.com"/>
    </rdf:Bag>
   </Iptc4xmpExt:RegistryId>
   
   TODO: Validate this
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
   
   TODO: Validate this
   <plus:Licensor>
    <rdf:Seq>
     <rdf:li
      plus:LicensorName="Raphael James Cohn"  TODO: Name parser?
      plus:LicensorID="raphael.cohn@stormmq.com"	DONE: email parser parser
      plus:LicensorTelephoneType1="http://ns.useplus.org/ldf/vocab/cell" DONE: type parser
      plus:LicensorTelephone1="+44 7590 675 756"	DONE: phonenumber parser
      plus:LicensorTelephoneType2="http://ns.useplus.org/ldf/vocab/cell" DONE: type parser
      plus:LicensorTelephone2="+44 7590 675 756"	DONE: phonenumber parser
      plus:LicensorEmail="raphael.cohn@stormmq.com"	DONE: email parser parser
      plus:LicensorURL="https://photos.stormmq.com/"/>	DONE: URL parser
    </rdf:Seq>
   </plus:Licensor>
   
   <plus:ImageSupplier>
    <rdf:Seq>
     <rdf:li
      plus:ImageSupplierName="Raphael James Cohn"
      plus:ImageSupplierID="raphael.cohn@stormmq.com"/>
    </rdf:Seq>
   </plus:ImageSupplier>
   
   TODO: Validate this; overlaps with dc:subject
   <lr:hierarchicalSubject>
    <rdf:Bag>
     <rdf:li>Places|United Kingdom|England|Yorkshire|Runswick Bay</rdf:li>	TODO: hierarchicalSubject parser.
     <rdf:li>Buildings|Old|Boat Houses</rdf:li>
    </rdf:Bag>
   </lr:hierarchicalSubject>
   
   
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


	 */
	
	Ok(())
}
