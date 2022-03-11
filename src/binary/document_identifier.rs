// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn document_identifier<'a>(collated: &mut Collated, Description: &XmpElement<'a, 'static, 'static, 'static>) -> Option<XmpUniversallyUniqueIdentifier>
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
			collated.push(XmpValidationError::OriginalDocumentIdentifierDoesNotMatchDocumentIdentifier { original_document_identifier, document_identifier });
			None
		}
		
		_ => None,
	}
}
