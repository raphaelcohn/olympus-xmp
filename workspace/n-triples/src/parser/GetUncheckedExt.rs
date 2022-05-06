// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) trait GetUncheckedExt<T>: GetUnchecked<T>
{
	fn rewind_buffer(&self, utf8_character_length: Utf8CharacterLength) -> *const T;
}

impl<T> GetUncheckedExt<T> for [T]
{
	#[inline(always)]
	fn rewind_buffer(&self, utf8_character_length: Utf8CharacterLength) -> *const T
	{
		let pointer = self.as_ptr();
		let slice_length = utf8_character_length.into();
		let rewound_buffer = unsafe { pointer.sub(slice_length) };
		rewound_buffer
	}
}
