// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Try to own an object in-place.
pub trait TryToOwnInPlace
{
	/// Afer calling this method, all lifetimes will have been erased and replaced with static; all data will be owned.
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>;
}

impl<'a, B: 'static + TryToOwned + ?Sized> TryToOwnInPlace for Cow<'a, B>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		try_to_own_in_place_cow(self)
	}
}

impl<TTOIP: TryToOwnInPlace> TryToOwnInPlace for Vec<TTOIP>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		let length = self.len();
		for index in 0 .. length
		{
			let ttoip = self.get_unchecked_mut_safe(index);
			ttoip.try_to_own_in_place()?;
		}
		Ok(())
	}
}

impl<K: TryToOwnInPlace, V: TryToOwnInPlace> TryToOwnInPlace for MutableKeyHashMap<K, V>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		for (key, value) in self.iter_mut()
		{
			key.try_to_own_in_place()?;
			value.try_to_own_in_place()?;
		}
		Ok(())
	}
}