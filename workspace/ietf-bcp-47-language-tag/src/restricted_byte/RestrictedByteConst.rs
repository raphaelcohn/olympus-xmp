// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(crate) trait RestrictedByteConst: Copy + Debug
{
	type Error: error::Error;
	
	fn construct(validated_byte: u8) -> Self;
	
	fn error<const length: usize>(index: usize, byte: u8) -> Self::Error;
	
	fn new_array_unchecked<const length: usize>(value: [u8; length]) -> [Self; length];
	
	fn new_array_unchecked_ref<const length: usize>(value: &[u8; length]) -> [Self; length];
	
	fn validate_byte(byte: u8) -> bool;
}
