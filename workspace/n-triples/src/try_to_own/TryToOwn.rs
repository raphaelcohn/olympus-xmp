// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Try to own a Cow-backed object.
pub trait TryToOwn: TryToOwnInPlace
{
	/// Static self.
	type TryToOwned: 'static + TryToOwn;
	
	/// Try to own a Cow-backed object that might require allocation that could fail.
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>;
}

impl<T: TryToOwn + TryToOwnInPlace, const N: usize> TryToOwn for ConstSmallVec<T, N>
{
	type TryToOwned = ConstSmallVec<T::TryToOwned, N>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}
