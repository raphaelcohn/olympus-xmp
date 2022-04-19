// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An iterator.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StringLiteralsMapValuesIterator<'a: 'string_literals_map, 'string_literals_map, Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item>(&'string_literals_map [Cow<'a, str>], Parser);

impl<'a: 'string_literals_map, 'string_literals_map, Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item> Iterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, Item, Parser>
{
	type Item = Item;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let length = self.0.len();
		if length == 0
		{
			return None
		}
		
		let next = self.0.get_unchecked_safe(0);
		let next = next.deref();
		*(&mut self.0) = self.0.get_unchecked_range_safe(1 .. );
		Some((self.1)(next))
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		let length = self.len();
		(length, Some(length))
	}
}

impl<'a: 'string_literals_map, 'string_literals_map, Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item> DoubleEndedIterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, Item, Parser>
{
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item>
	{
		let length = self.0.len();
		if length == 0
		{
			return None
		}
		
		let last_index = length - 1;
		let next_back = self.0.get_unchecked_safe(last_index);
		let next_back = next_back.deref();
		*(&mut self.0) = self.0.get_unchecked_range_safe(..last_index);
		Some((self.1)(next_back))
	}
}

impl<'a: 'string_literals_map, 'string_literals_map, Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item> ExactSizeIterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, Item, Parser>
{
	#[inline(always)]
	fn len(&self) -> usize
	{
		self.0.len()
	}
}

unsafe impl<'a: 'string_literals_map, 'string_literals_map, Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item> TrustedLen for StringLiteralsMapValuesIterator<'a, 'string_literals_map, Item, Parser>
{
}

impl<'a: 'string_literals_map, 'string_literals_map, Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item> FusedIterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, Item, Parser>
{
}

impl<'a: 'string_literals_map, 'string_literals_map, Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item> StringLiteralsMapValuesIterator<'a, 'string_literals_map, Item, Parser>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn zero_or_one<E: error::Error>(mut self) -> Result<Option<Item>, ZeroOrOneError<E>>
	{
		use ZeroOrOneError::*;
		match self.len()
		{
			0 => Ok(None),
			
			1 =>
			{
				let next = self.next();
				Ok(Some(unsafe { next.unwrap_unchecked() }))
			}
			
			length @ _ => Err(TooMany(MoreThanOneError { count: new_non_zero_usize(length) }))
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn only_one<E: error::Error>(mut self) -> Result<Item, OnlyOneError<E>>
	{
		use ZeroOrOneError::*;
		use OnlyOneError::*;
		match self.len()
		{
			0 => Err(Missing),
			
			1 =>
			{
				let next = self.next();
				Ok(unsafe { next.unwrap_unchecked() })
			}
			
			length @ _ => Err(ZeroOrOne(TooMany(MoreThanOneError { count: new_non_zero_usize(length) })))
		}
	}
}
