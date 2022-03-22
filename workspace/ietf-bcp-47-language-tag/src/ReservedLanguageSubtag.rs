// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ReservedLanguageSubtag([Alpha; 4]);

impl const Into<[Alpha; 4]> for ReservedLanguageSubtag
{
	#[inline(always)]
	fn into(self) -> [Alpha; 4]
	{
		self.0
	}
}

impl<'a> const From<&'a [u8; 4]> for ReservedLanguageSubtag
{
	#[inline(always)]
	fn from(value: &'a [u8; 4]) -> Self
	{
		Self(Alpha::new_array_unchecked_ref(value))
	}
}

impl const From<[u8; 4]> for ReservedLanguageSubtag
{
	#[inline(always)]
	fn from(value: [u8; 4]) -> Self
	{
		Self(Alpha::new_array_unchecked(value))
	}
}

impl const From<[Alpha; 4]> for ReservedLanguageSubtag
{
	#[inline(always)]
	fn from(value: [Alpha; 4]) -> Self
	{
		Self(value)
	}
}

impl ReservedLanguageSubtag
{
	#[inline(always)]
	fn parse(subtag: &[u8]) -> Result<Language, LanguageSubtagParseError>
	{
		const length: usize = 4;
		debug_assert_eq!(subtag.len(), length);
		Alpha::validate_and_convert_array::<_, _, _, _, length>(subtag, |alpha_array| Language::Reserved(Self(alpha_array)), LanguageSubtagParseError::InvalidAlpha)
	}
}
