// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Only exists because of a design flaw in Rust that does not allow const impl Traits contain default functions.
#[inline(always)]
const fn new_array_unchecked_ref<RBC: ~const RestrictedByteConst, const length: usize>(value: &[u8; length]) -> [RBC; length]
{
	new_array_unchecked_validation::<RBC, length>(value);
	unsafe { transmute_copy(value) }
}
