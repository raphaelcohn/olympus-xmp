// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XMP validation error.
#[derive(Debug, Clone, Eq, PartialEq)]
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
}

impl<'name, 'namespace, 'local_name> Display for XmpValidationError<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
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
			
			MissingAttribute { .. } => None,
			
			CouldNotParseAttribute { cause, .. } => Some(cause),
			
			AttributeDoesNotHaveExpectedValue { .. } => None,
			
			TiffWidthDoesNotMatchExifWidth(..) => None,
			
			TiffHeightDoesNotMatchExifHeight(..) => None,
			
			OriginalDocumentIdentifierDoesNotMatchDocumentIdentifier { .. } => None,
			
			MismatchedLensModels { .. } => None,
		}
	}
}
