// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) trait Utf8Sequence: Sized
{
	const Length: Utf8CharacterLength;
	
	type Remainder: Copy + Eq + Ord + Debug;
	
	fn construct(first: u8, remainder: Self::Remainder) -> Self;
	
	fn is(first: u8) -> bool;
	
	fn into_raw_unicode_code_point(self) -> u32;
	
	fn slice_length<BP: ByteProvider>() -> NonZeroUsize;
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations
	fn try_into_char(self) -> Result<char, CharTryFromError>;
	
	/// Rust limitation: This method is the same in all implementations, but Rust does not permit `const impl` traits to have default function implementations
	unsafe fn unchecked_into_char(self) -> char;
}
