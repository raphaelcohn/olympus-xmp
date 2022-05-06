// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Percent encodable element.
pub trait PercentEncodable<'a>: Sized + Into<Cow<'a, str>>
{
	/// Percent encodes reserved characters and all non-ASCII characters.
	#[inline(always)]
	fn percent_encoded_and_suitable_for_an_uniform_resource_identifier<'x>(&'x self) -> Result<Cow<'x, str>, TryReserveError>
	where 'a: 'x
	{
		encode_utf8_percent_encoded(self.as_str(), Self::percent_encode_ascii)
	}
	
	/// Percent encodes reserved characters and all non-ASCII characters.
	///
	/// More efficient than calling `self.percent_encoded_and_suitable_for_an_uniform_resource_identifier().to_owned()`.
	#[inline(always)]
	fn into_percent_encoded_and_suitable_for_an_uniform_resource_identifier(self) -> Result<String, TryReserveError>
	{
		use Cow::*;
		if let Owned(owned) = self.percent_encoded_and_suitable_for_an_uniform_resource_identifier()?
		{
			return Ok(owned)
		}
		
		match self.into()
		{
			Owned(owned) => Ok(owned),
			
			Borrowed(borrowed) => borrowed.try_to_owned(),
		}
	}
	
	#[doc(hidden)]
	fn display_fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		match self.percent_encoded_and_suitable_for_an_uniform_resource_identifier()
		{
			Ok(percent_encoded_string) => f.write_str(percent_encoded_string.as_ref()),
			
			Err(_) => Err(fmt::Error),
		}
	}
	
	#[doc(hidden)]
	fn as_str(&self) -> &str;
	
	#[doc(hidden)]
	fn percent_encode_ascii(ascii_byte: u8) -> bool;
}
