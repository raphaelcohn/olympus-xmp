// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Value is one of:
/// * `a-w` and `y-z` (missing `x`).
/// * `0-9`.
/// Case insensitive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Singleton(u8);

impl RestrictedByte for Singleton
{
	type Error = InvalidSingletonError;
	
	#[inline(always)]
	fn construct(validated_byte: u8) -> Self
	{
		debug_assert!((validated_byte >= _0 && validated_byte <= _9) || (validated_byte >= a && validated_byte <= w) || (validated_byte >= y && validated_byte <= z));
		Self(validated_byte)
	}
	
	#[inline(always)]
	fn error<const length: usize>(index: usize, byte: u8) -> Self::Error
	{
		InvalidSingletonError { length, index, byte }
	}
}
