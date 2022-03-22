// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XMP validation error.
#[derive(Debug)]
pub enum XmpValidationError<'name, 'namespace, 'local_name>
{
	#[allow(missing_docs)]
	MissingOnlyElement
	{
		path: XmpElementPath<'name, 'namespace, 'local_name>,
		
		cause: NotExactlyOneElementError,
	},
	
	#[allow(missing_docs)]
	MissingAttribute
	{
		path: XmpElementPath<'name, 'namespace, 'local_name>,
		
		attribute_name: &'name XmlName<'namespace, 'local_name>,
	},
	
	#[allow(missing_docs)]
	HasAttributeWhichShouldNotBePresent
	{
		path: XmpElementPath<'name, 'namespace, 'local_name>,
		
		attribute_name: &'name XmlName<'namespace, 'local_name>,
	},
	
	#[allow(missing_docs)]
	HasAttributesInNamespace
	{
		path: XmpElementPath<'name, 'namespace, 'local_name>,
		
		namespace_prefix: &'static str,
	},
	
	#[allow(missing_docs)]
	HasAttributeWhichIsObsolete
	{
		path: XmpElementPath<'name, 'namespace, 'local_name>,
		
		attribute_name: &'name XmlName<'namespace, 'local_name>,
	},
	
	#[allow(missing_docs)]
	CouldNotParseAttribute
	{
		path: XmpElementPath<'name, 'namespace, 'local_name>,
		
		attribute_name: &'name XmlName<'namespace, 'local_name>,
	
		cause: XmpAttributeValueParseError,
	},
	
	#[allow(missing_docs)]
	AttributeDoesNotHaveExpectedValue
	{
		path: XmpElementPath<'name, 'namespace, 'local_name>,
		
		attribute_name: &'name XmlName<'namespace, 'local_name>,
	},
	
	#[allow(missing_docs)]
	TiffWidthDoesNotMatchExifWidth(NonZeroU32, NonZeroU32),
	
	#[allow(missing_docs)]
	TiffHeightDoesNotMatchExifHeight(NonZeroU32, NonZeroU32),
	
	#[allow(missing_docs)]
	OriginalDocumentIdentifierDoesNotMatchDocumentIdentifier
	{
		original_document_identifier: XmpUniversallyUniqueIdentifier,
		
		document_identifier: XmpUniversallyUniqueIdentifier,
	},
	
	#[allow(missing_docs)]
	MismatchedLensModels
	{
		exifEx_lens_model: String,
		
		aux_lens_model: String,
	},
	
	#[allow(missing_docs)]
	InvalidFocalLengthIn35mmFilmForCropFactor,
	
	#[allow(missing_docs)]
	UnknownFocalLengthIn35mmFilm,
	
	#[allow(missing_docs)]
	LensInformationDoesNotContainShotFocalLength,
	
	#[allow(missing_docs)]
	LensInformationDoesCouldNotHaveShotFNumber,
}

impl<'name, 'namespace, 'local_name> Display for XmpValidationError<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<'name, 'namespace, 'local_name> error::Error for XmpValidationError<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use XmpValidationError::*;
		match self
		{
			MissingOnlyElement { cause, .. } => Some(cause),
			
			CouldNotParseAttribute { cause, .. } => Some(cause),
			
			AttributeDoesNotHaveExpectedValue { .. } => None,
			
			TiffWidthDoesNotMatchExifWidth(..) => None,
			
			TiffHeightDoesNotMatchExifHeight(..) => None,
			
			OriginalDocumentIdentifierDoesNotMatchDocumentIdentifier { .. } => None,
			
			MismatchedLensModels { .. } => None,
			
			InvalidFocalLengthIn35mmFilmForCropFactor => None,
			
			UnknownFocalLengthIn35mmFilm => None,
			
			LensInformationDoesNotContainShotFocalLength => None,
			
			LensInformationDoesCouldNotHaveShotFNumber => None,
			
			HasAttributesInNamespace { .. } => None,
			
			HasAttributeWhichShouldNotBePresent { .. } => None,
			
			HasAttributeWhichIsObsolete { .. } => None,
			
			MissingAttribute { .. } => None,
		}
	}
}
