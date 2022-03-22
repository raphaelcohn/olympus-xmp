// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug)]
pub(super) struct Collated<'name, 'namespace, 'local_name>(Vec<XmpValidationError<'name, 'namespace, 'local_name>>);

impl<'name, 'namespace, 'local_name> Collated<'name, 'namespace, 'local_name>
{
	#[inline(always)]
	pub(super) fn check<R>(&mut self, check: Result<R, XmpValidationError<'name, 'namespace, 'local_name>>)
	{
		if let Err(xmp_validation_error) = check
		{
			self.push(xmp_validation_error)
		}
	}
	
	#[inline(always)]
	pub(super) fn validate<R>(&mut self, check: Result<R, XmpValidationError<'name, 'namespace, 'local_name>>) -> Option<R>
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
	pub(super) fn push(&mut self, error: XmpValidationError<'name, 'namespace, 'local_name>)
	{
		self.0.push(error)
	}
}
