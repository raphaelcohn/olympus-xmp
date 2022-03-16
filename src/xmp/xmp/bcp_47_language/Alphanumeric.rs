// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.

/// Value is one of:
/// * `A-Z`.
/// * `0-9`.
/// Case insensitive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Alphanumeric(u8);

impl Alphanumeric
{
	#[inline(always)]
	const fn _1(array: &[u8; 1]) -> [Self; 1]
	{
		[Self(array[0])]
	}
	
	#[inline(always)]
	const fn _2(array: &[u8; 2]) -> [Self; 2]
	{
		[Self(array[0]), Self(array[1])]
	}
	
	#[inline(always)]
	const fn _3(array: &[u8; 3]) -> [Self; 3]
	{
		[Self(array[0]), Self(array[1]), Self(array[2])]
	}
	
	#[inline(always)]
	const fn _4(array: &[u8; 4]) -> [Self; 4]
	{
		[Self(array[0]), Self(array[1]), Self(array[2]), Self(array[3])]
	}
	
	#[inline(always)]
	const fn _5(array: &[u8; 5]) -> [Self; 5]
	{
		[Self(array[0]), Self(array[1]), Self(array[2]), Self(array[3]), Self(array[4])]
	}
	
	#[inline(always)]
	const fn _6(array: &[u8; 6]) -> [Self; 6]
	{
		[Self(array[0]), Self(array[1]), Self(array[2]), Self(array[3]), Self(array[4]), Self(array[5])]
	}
	
	#[inline(always)]
	const fn _7(array: &[u8; 7]) -> [Self; 7]
	{
		[Self(array[0]), Self(array[1]), Self(array[2]), Self(array[3]), Self(array[4]), Self(array[5]), Self(array[6])]
	}
	
	#[inline(always)]
	const fn _8(array: &[u8; 8]) -> [Self; 8]
	{
		[Self(array[0]), Self(array[1]), Self(array[2]), Self(array[3]), Self(array[4]), Self(array[5]), Self(array[6]), Self(array[7])]
	}
}
