// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn try_to_own_in_place_cow<'a, B: 'a + TryToOwned + ?Sized>(cow: &mut Cow<'a, B>) -> Result<(), TryReserveError>
{
	use Cow::*;
	if let Borrowed(borrowed) = cow
	{
		let string = (*borrowed).try_to_owned()?;
		*cow = Owned(string)
	}
	Ok(())
}
