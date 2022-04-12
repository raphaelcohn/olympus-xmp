// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Constructs a new instance without checking the provided `value` is correct.
///
/// Used for constructing constants.
pub trait FromUnchecked<T>: Sized
{
	/// Used for constructing constants.
	unsafe fn from_unchecked(value: T) -> Self;
}

impl<T> const FromUnchecked<T> for T
{
	#[inline(always)]
	unsafe fn from_unchecked(value: T) -> Self
	{
		value
	}
}

impl<'a, T: 'a + FromUnchecked<Cow<str>>> const FromUnchecked<&'a str> for T
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self::from_unchecked(Cow::Borrowed(value))
	}
}

impl<'a, T: 'a + FromUnchecked<Cow<'a, str>>> const FromUnchecked<String> for T
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self::from_unchecked(Cow::Owned(value))
	}
}
