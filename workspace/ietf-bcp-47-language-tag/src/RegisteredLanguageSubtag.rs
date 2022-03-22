// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RegisteredLanguageSubtag
{
	Alpha5([Alpha; 5]),
	
	Alpha6([Alpha; 6]),
	
	Alpha7([Alpha; 7]),
	
	Alpha8([Alpha; 8]),
}

impl const From<[u8; 5]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [u8; 5]) -> Self
	{
		Self::from(Alpha::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 5]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: &'a [u8; 5]) -> Self
	{
		Self::from(Alpha::new_array_unchecked_ref(value))
	}
}

impl const From<[Alpha; 5]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [Alpha; 5]) -> Self
	{
		RegisteredLanguageSubtag::Alpha5(value)
	}
}

impl const From<[u8; 6]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [u8; 6]) -> Self
	{
		Self::from(Alpha::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 6]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: &'a [u8; 6]) -> Self
	{
		Self::from(Alpha::new_array_unchecked_ref(value))
	}
}

impl const From<[Alpha; 6]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [Alpha; 6]) -> Self
	{
		RegisteredLanguageSubtag::Alpha6(value)
	}
}

impl const From<[u8; 7]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [u8; 7]) -> Self
	{
		Self::from(Alpha::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 7]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: &'a [u8; 7]) -> Self
	{
		Self::from(Alpha::new_array_unchecked_ref(value))
	}
}

impl const From<[Alpha; 7]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [Alpha; 7]) -> Self
	{
		RegisteredLanguageSubtag::Alpha7(value)
	}
}

impl const From<[u8; 8]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [u8; 8]) -> Self
	{
		Self::from(Alpha::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 8]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: &'a [u8; 8]) -> Self
	{
		Self::from(Alpha::new_array_unchecked_ref(value))
	}
}

impl const From<[Alpha; 8]> for RegisteredLanguageSubtag
{
	#[inline(always)]
	fn from(value: [Alpha; 8]) -> Self
	{
		RegisteredLanguageSubtag::Alpha8(value)
	}
}

impl RegisteredLanguageSubtag
{
	#[inline(always)]
	fn parse<const length: usize>(subtag: &[u8]) -> Result<Language, LanguageSubtagParseError>
	where RegisteredLanguageSubtag: From<[Alpha; length]>
	{
		const InclusiveMinimumLength: usize = 5;
		const InclusiveMaximumLength: usize = 8;
		debug_assert!(length >= InclusiveMinimumLength);
		debug_assert!(length <= InclusiveMaximumLength);
		debug_assert!(subtag.len() >= InclusiveMinimumLength);
		debug_assert!(subtag.len() <= InclusiveMaximumLength);
		
		Alpha::validate_and_convert_array::<_, _, _, _, length>(subtag, |alpha_array| Language::Registered(RegisteredLanguageSubtag::from(alpha_array)), LanguageSubtagParseError::InvalidAlpha)
	}
}
