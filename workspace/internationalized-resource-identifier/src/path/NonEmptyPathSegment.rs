// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Can not be empty.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NonEmptyPathSegment<'a>(Cow<'a, str>);

impl<'a> Display for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.0.as_ref())
	}
}

impl<'a> TryToOwnInPlace for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for NonEmptyPathSegment<'a>
{
	type TryToOwned = NonEmptyPathSegment<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> Into<Cow<'a, str>> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn into(self) -> Cow<'a, str>
	{
		self.0
	}
}

impl<'a> const Borrow<str> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const AsRef<str> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const Deref for NonEmptyPathSegment<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a> const FromUnchecked<Cow<'a, str>> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const FromUnchecked<&'a str> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self(Cow::Borrowed(value))
	}
}

impl<'a> const FromUnchecked<String> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self(Cow::Owned(value))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8; Count]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a> const FromUnchecked<PathSegment<'a>> for NonEmptyPathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: PathSegment<'a>) -> Self
	{
		transmute(value)
	}
}

impl<'a> const TryFrom<PathSegment<'a>> for NonEmptyPathSegment<'a>
{
	type Error = PathSegment<'a>;
	
	#[inline(always)]
	fn try_from(value: PathSegment<'a>) -> Result<Self, Self::Error>
	{
		if value.is_empty()
		{
			Err(value)
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl<'a> NonEmptyPathSegment<'a>
{
	#[inline(always)]
	fn decode_percent_encoded_path_segment_remainder(first_character_of_first_path_segment: (bool, Utf8SequenceEnum), mut remaining_percent_encoded_path_segment: &'a str) -> Result<NonEmptyPathSegment, PathSegmentParseError>
	{
		let string = Utf8SequencesParser::new_percent_encoded_non_empty_path_segment(first_character_of_first_path_segment, remaining_percent_encoded_path_segment)?;
		PathSegment::decode_percent_encoded_path_segment_common(string, &mut remaining_percent_encoded_path_segment, Self)
	}
}
