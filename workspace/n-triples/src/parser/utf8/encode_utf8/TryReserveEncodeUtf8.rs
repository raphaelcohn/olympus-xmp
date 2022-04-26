// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) struct TryReserveEncodeUtf8<'a>(UnreservedEncodeUtf8<'a>);

impl<'a> EncodeUtf8 for TryReserveEncodeUtf8<'a>
{
	type R = Result<(), TryReserveError>;
	
	#[inline(always)]
	fn push_unchecked<const length: usize>(mut self, encoded_utf8_bytes: [u8; length]) -> Self::R
	{
		self.0.try_reserve(length)?;
		self.0.push_unchecked::<length>(encoded_utf8_bytes);
		Ok(())
	}
}

impl<'a> TryReserveEncodeUtf8<'a>
{
	#[inline(always)]
	pub(super) const fn new(buffer: &'a mut Vec<u8>, offset: usize) -> Self
	{
		Self(UnreservedEncodeUtf8::new(buffer, offset))
	}
}
