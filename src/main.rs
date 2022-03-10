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
use memchr as _;
use swiss_army_knife as _;
use xml as _;


use std::num::{NonZeroU16, NonZeroU32};
use olympus_xmp::xml_name;
use olympus_xmp::xml::XmlDocument;
use olympus_xmp::xml::XmlName;
use olympus_xmp::xmp::{ExifSceneCaptureType, IptcDigitalSourceType, PlusModelReleaseStatus, PlusPropertyReleaseStatus, XmpElement};
use olympus_xmp::xmp::XmpValidationError;
use olympus_xmp::xmp::namespaces::Iptc4xmpCore;
use olympus_xmp::xmp::namespaces::Iptc4xmpExt;
use olympus_xmp::xmp::namespaces::aux;
use olympus_xmp::xmp::namespaces::dc;
use olympus_xmp::xmp::namespaces::exif;
use olympus_xmp::xmp::namespaces::exifEX;
use olympus_xmp::xmp::namespaces::photoshop;
use olympus_xmp::xmp::namespaces::plus;
use olympus_xmp::xmp::namespaces::rdf;
use olympus_xmp::xmp::namespaces::tiff;
use olympus_xmp::xmp::namespaces::x;
use olympus_xmp::xmp::namespaces::xmp;
use olympus_xmp::xmp::namespaces::xmpMM;
use olympus_xmp::xmp::namespaces::xmpRights;
use olympus_xmp::xmp::PhotoshopColorMode;
use olympus_xmp::xmp::XmpLabel;
use olympus_xmp::xmp::XmpRating;
use olympus_xmp::xmp::date_time::XmpDateTime;
use olympus_xmp::xmp::tiff_rational::TiffRational;
use olympus_xmp::xmp::universally_unique_identifier::XmpUniversallyUniqueIdentifier;


fn main()
{
	let path = "/path/to/file.xml";
	let (xml_tree, file) = XmlDocument::parse_path(path, true).unwrap();
	xml_tree.write_file(file).unwrap();
}

/// Outcome of XMP validation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum XmpOutcomeOfValidationError<'name, 'namespace, 'local_name>
{
	/// Errors beyond which it is not possible to validate further a XMP document.
	Fundamental(XmpValidationError<'name, 'namespace, 'local_name>),
	
	/// Non-empty collection of XMP validation errors.
	Collated(Vec<XmpValidationError<'name, 'namespace, 'local_name>>),
}

impl<'name, 'namespace, 'local_name> XmpOutcomeOfValidationError<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	fn fundamental<R>(check: Result<R, XmpValidationError<'name, 'namespace, 'local_name>>) -> Result<R, Self>
	{
		check.map_err(XmpOutcomeOfValidationError::Fundamental)
	}
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct Collated<'name, 'namespace, 'local_name>(Vec<XmpValidationError<'name, 'namespace, 'local_name>>);

impl<'name, 'namespace, 'local_name> Collated<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	fn check<R>(&mut self, check: Result<R, XmpValidationError<'name, 'namespace, 'local_name>>)
	{
		if let Err(xmp_validation_error) = check
		{
			self.push(xmp_validation_error)
		}
	}
	
	#[inline(always)]
	fn validate<R>(&mut self, check: Result<R, XmpValidationError<'name, 'namespace, 'local_name>>) -> Option<R>
	{
		match check
		{
			Err(xmp_validation_error) =>
			{
				self.push(xmp_validation_error);
				None
			},
			
			Ok(result) => Some(result)
		}
	}
	
	#[inline(always)]
	fn push(&mut self, error: XmpValidationError<'name, 'namespace, 'local_name>)
	{
		self.0.push(error)
	}
}

fn validate(xml_document: XmlDocument) -> Result<(), XmpOutcomeOfValidationError<'static, 'static, 'static>>
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
	
	collated.check(xmpmeta.has_attribute_with_any_value::<&str>(xml_name!(x, "xmptk"))); // TODO: Parse this value?
	collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(dc, "format"), "image/x-olympus-raw"));
	collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(rdf, "about"), ""));
	collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(exif, "ExifVersion"), "0230")); // TODO: Parse this value? Default this value? Update this value?
	
	#[inline(always)]
	fn width_or_height<'a>(collated: &mut Collated, Description: &XmpElement<'a, 'static, 'static, 'static>, tiff_attribute: &'static XmlName<'static, 'static>, exif_attribute: &'static XmlName<'static, 'static>, error: impl FnOnce(NonZeroU32, NonZeroU32) -> XmpValidationError<'static, 'static, 'static>) -> Option<NonZeroU32>
	{
		let tiff_dimension_value = collated.validate(Description.get_attribute_or_error::<NonZeroU32>(tiff_attribute));
		let exif_dimension_value = collated.validate(Description.get_attribute_or_error::<NonZeroU32>(exif_attribute));
		
		match (tiff_dimension_value, exif_dimension_value)
		{
			(Some(tiff_dimension_value), Some(exif_dimension_value)) => if tiff_dimension_value == exif_dimension_value
			{
				Some(tiff_dimension_value)
			}
			else
			{
				collated.push(error(tiff_dimension_value, exif_dimension_value));
				None
			}
			
			_ => None,
		}
	}
	
	// TODO: Use this to set Iptc4xmpExt:MaxAvailWidth
	let pixel_x = width_or_height(&mut collated, &Description, xml_name!(tiff, "ImageLength"), xml_name!(exif, "PixelXDimension"), TiffWidthDoesNotMatchExifWidth);
	
	// TODO: Use this to set Iptc4xmpExt:MaxAvailHeight
	let pixel_y = width_or_height(&mut collated, &Description, xml_name!(tiff, "ImageWidth"), xml_name!(exif, "PixelYDimension"), TiffHeightDoesNotMatchExifHeight);
	
	let document_identifier =
	{
		let original_document_identifier = collated.validate(Description.get_attribute_or_error::<XmpUniversallyUniqueIdentifier>(xml_name!(xmpMM, "OriginalDocumentID")));
		let document_identifier = collated.validate(Description.get_attribute_or_error::<XmpUniversallyUniqueIdentifier>(xml_name!(xmpMM, "DocumentID")));
		
		match (original_document_identifier, document_identifier)
		{
			(Some(original_document_identifier), Some(document_identifier)) => if original_document_identifier == document_identifier
			{
				Some(document_identifier)
			}
			else
			{
				collated.push(OriginalDocumentIdentifierDoesNotMatchDocumentIdentifier { original_document_identifier, document_identifier });
				None
			}
			
			_ => None,
		}
	};
	
	/*
		TODO: For Canon 200mm, add in.
		   exifEX:LensModel="CANON FD.200mm F2.8"
		   aux:Lens="CANON FD.200mm F2.8"
		   aux:LensSerialNumber="33574"
		   aux:LensInfo="XXX"
	 */
	collated.check(Description.has_attribute_with_any_value::<&str>(xml_name!(aux, "LensSerialNumber")));
	collated.check(Description.has_attribute_with_any_value::<&str>(xml_name!(aux, "LensInfo"))); // TODO: Parse as "45/1 45/1 18/10 18/10"
	collated.check(Description.has_attribute_with_any_value::<Option<NonZeroU16>>(xml_name!(exif, "FocalLengthIn35mmFilm"))); // TODO: Verify value matches lens, etc and also exif:FocalLength
	let lens_model =
	{
		let exifEx_lens_model = collated.validate(Description.get_attribute_or_error::<&str>(xml_name!(exifEX, "LensModel")));
		let aux_lens_model = collated.validate(Description.get_attribute_or_error::<&str>(xml_name!(aux, "Lens")));
		
		match (exifEx_lens_model, aux_lens_model)
		{
			(Some(exifEx_lens_model), Some(aux_lens_model)) => if exifEx_lens_model == aux_lens_model
			{
				Some(exifEx_lens_model)
			}
			else
			{
				collated.push(MismatchedLensModels { exifEx_lens_model: exifEx_lens_model.to_string(), aux_lens_model: aux_lens_model.to_string() });
				None
			},
			
			_ => None,
		}
	};
	
	let photoshop_created_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(photoshop, "DateCreated")));
	let xmp_create_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(xmp, "CreateDate")));
	let xmp_modify_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(xmp, "ModifyDate")));
	let xmp_metadata_date = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(xmp, "MetadataDate")));
	let exif_date_original = collated.validate(Description.get_attribute_or_error::<XmpDateTime>(xml_name!(exif, "DateTimeOriginal")));
	
	collated.check(Description.get_attribute::<XmpRating>(xml_name!(xmp, "Rating")));
	collated.check(Description.get_attribute::<XmpLabel>(xml_name!(xmp, "Label")));
	
	collated.check(Description.has_attribute_with_any_value::<&str>(xml_name!(tiff, "Make")));
	collated.check(Description.has_attribute_with_any_value::<&str>(xml_name!(tiff, "Model")));
	collated.check(Description.has_attribute_with_any_value::<&str>(xml_name!(aux, "SerialNumber")));
	collated.check(Description.has_attribute_with_any_value::<&str>(xml_name!(xmp, "CreatorTool")));
	collated.check(Description.has_attribute_with_any_value::<TiffRational>(xml_name!(aux, "FlashCompensation")));
	collated.check(Description.has_attribute_with_any_value::<ExifSceneCaptureType>(xml_name!(exif, "SceneCaptureType")));
	
	
	collated.check(Description.has_attribute_with_expected_value::<PhotoshopColorMode>(xml_name!(photoshop, "ColorMode"), PhotoshopColorMode::RgbColor));
	collated.check(Description.has_attribute_with_expected_value::<XmpUniversallyUniqueIdentifier>(xml_name!(photoshop, "EmbeddedXMPDigest"), XmpUniversallyUniqueIdentifier::Zero));
	
	// Not so much checks as data that should be present, or, for creator details, should be complete.
	{
		const PhotographerProperName: &str = "Raphael James Cohn";
		collated.check(Description.has_attribute_with_expected_value::<bool>(xml_name!(xmpRights, "Marked"), true));
		collated.check(Description.has_attribute_with_expected_value::<IptcDigitalSourceType>(xml_name!(Iptc4xmpExt, "DigitalSourceType"), IptcDigitalSourceType::OriginalDigitalCapture));
		collated.check(Description.has_attribute_with_expected_value::<PlusPropertyReleaseStatus>(xml_name!(plus, "PropertyReleaseStatus"), PlusPropertyReleaseStatus::NotApplicable));
		collated.check(Description.has_attribute_with_expected_value::<PlusModelReleaseStatus>(xml_name!(plus, "ModelReleaseStatus"), PlusModelReleaseStatus::NotApplicable));
		collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(photoshop, "AuthorsPosition"), "Photographer"));
		collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(photoshop, "Credit"), PhotographerProperName));
		collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(photoshop, "Source"), PhotographerProperName));
		collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(photoshop, "CaptionWriter"), PhotographerProperName));
		
		// TODO: Create this from make and model?
		collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(photoshop, "Instructions"), "Original RAW capture Olympus E-PL8"));
		
		if let Some(CreatorContactInfo) = collated.validate(Description.child(xml_name!(Iptc4xmpCore, "CreatorContactInfo")))
		{
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiAdrExtadr"), "6 Eller Mews"));
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiAdrCity"), "Skipton"));
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiAdrRegion"), "North Yorkshire"));
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiAdrPcode"), "BD23 2TG"));
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiAdrCtry"), "United Kingdom of Great Britain and Northern Ireland (the)"));
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiTelWork"), "+44 7590 675 756")); // TODO: Normalise this without spaces.
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiEmailWork"), "raphael.cohn@stormmq.com"));
			collated.check(Description.has_attribute_with_expected_value::<&str>(xml_name!(Iptc4xmpCore, "CiUrlWork"), "https://photos.stormmq.com/"));
		}
	}
	
	// TODO: Seq + li with text
	// TODO: Alt + xml:lang with text
	// TODO: Bag + li with properties but no text
	
   /*
   
   // TODO: Default this.
   photoshop:ICCProfile=""
   
   // TODO: Verify this is a lower-case hyphen separated UUID.
   xmpMM:InstanceID="xmp.iid:22b0d4af-ade9-4d85-a82d-80f0048494fa"
   
   TODO: Overwrite this using LocationShown's first entry.
   Iptc4xmpCore:Location="Addingham Churchyard"
   photoshop:City="Addingham"
   photoshop:State="North Yorkshire"
   photoshop:Country="United Kingdom of Great Britain and Northern Ireland (the)"
   Iptc4xmpCore:CountryCode="GBR"
   
   TODO: Are these lens properties?
   exif:FNumber="18/10"
   exif:FocalLength="45/1"
   exif:FocalLengthIn35mmFilm="90"
   // 0 means unknown. SHORT.
   
   exif:ExposureTime="1/4000"
   exif:ShutterSpeedValue="11965784/1000000"
   exif:ApertureValue="1695994/1000000"
   exif:MaxApertureValue="434/256"
   exif:ExposureProgram="3"
   exif:SensitivityType="1"
   exif:ExposureBiasValue="0/10"
   exif:MeteringMode="5"
   exif:LightSource="9"
   exif:FileSource="3"
   exif:CustomRendered="0"
   exif:ExposureMode="0"
   exif:WhiteBalance="1"
   exif:GainControl="1"		0,1,2,3,4
   exif:Contrast="0"
   exif:Saturation="0"
   exif:Sharpness="0"
   exif:DigitalZoomRatio="100/100"
   exif:FocalPlaneXResolution="87196351/32768"
   exif:FocalPlaneYResolution="87196351/32768"
   exif:FocalPlaneResolutionUnit="3"
   
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
   
   TODO: Can be oj
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
   
   <Iptc4xmpExt:LocationShown>
    <rdf:Bag>
     <rdf:li
      Iptc4xmpExt:ProvinceState="North Yorkshire"
      Iptc4xmpExt:CountryName="United Kingdom of Great Britain and Northern Ireland (the)"
      Iptc4xmpExt:CountryCode="GBR"
      Iptc4xmpExt:WorldRegion="Europe"
      Iptc4xmpExt:Sublocation="Addingham Churchyard"
      Iptc4xmpExt:City="Addingham"/>
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
