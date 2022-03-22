// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Outcome of XMP validation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum XmpOutcomeOfValidationError<'name, 'namespace, 'local_name>
{
	/// Errors beyond which it is not possible to validate further a XMP document.
	Fundamental(XmpValidationError<'name, 'namespace, 'local_name>),
	
	/// Non-empty collection of XMP validation errors.
	Collated(Vec<XmpValidationError<'name, 'namespace, 'local_name>>),
}

impl<'name, 'namespace, 'local_name> XmpOutcomeOfValidationError<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	pub(super) fn fundamental<R>(check: Result<R, XmpValidationError<'name, 'namespace, 'local_name>>) -> Result<R, Self>
	{
		check.map_err(XmpOutcomeOfValidationError::Fundamental)
	}
}
