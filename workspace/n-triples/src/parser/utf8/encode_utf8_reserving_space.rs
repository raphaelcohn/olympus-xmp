// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn encode_utf8_reserving_space(buffer: &mut Vec<u8>, character: char) -> Result<(), TryReserveError>
{
	#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
	struct ReserveEncodeUtf8;
	
	impl EncodeUtf8 for ReserveEncodeUtf8
	{
		type R = Result<(), TryReserveError>;
		
		#[inline(always)]
		fn push_unchecked<const length: usize>(buffer: &mut Vec<u8>, offset: usize, encoded_utf8_bytes: [u8; length]) -> Self::R
		{
			buffer.try_reserve(length)?;
			UnreservedEncodeUtf8::push_unchecked::<length>(buffer, offset, encoded_utf8_bytes);
			Ok(())
		}
	}
	
	ReserveEncodeUtf8::encode_utf8(buffer, character, buffer.len())
}
