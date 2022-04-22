// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An iterator.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StringLiteralsMapValuesIterator<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize>(&'string_literals_map [Cow<'a, str>], PhantomData<SP>);

impl<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize> Iterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP, PathDepth>
{
	type Item = Result<SP::Item, SP::Error>;
	
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
		
		Some(SP::parse(next))
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		let length = self.len();
		(length, Some(length))
	}
}

impl<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize> DoubleEndedIterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP, PathDepth>
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
		Some(SP::parse(next_back))
	}
}

impl<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize> ExactSizeIterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP, PathDepth>
{
	#[inline(always)]
	fn len(&self) -> usize
	{
		self.0.len()
	}
}

unsafe impl<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize> const TrustedLen for StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP, PathDepth>
{
}

impl<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize> const FusedIterator for StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP, PathDepth>
{
}

impl<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize> const Default for StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP, PathDepth>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Empty
	}
}

impl<'a: 'string_literals_map, 'string_literals_map, SP: StrParser<'string_literals_map, PathDepth>, const PathDepth: usize> StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP, PathDepth>
{
	#[inline(always)]
	const fn new(slice: &'string_literals_map [Cow<'a, str>]) -> Self
	{
		Self(slice, PhantomData)
	}
	
	const Empty: Self =
	{
		const EmptySlice: &'static [Cow<'static, str>] = &[];
		Self::new(EmptySlice)
	};
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn zero_or_one(self) -> Result<Option<SP::Item>, OptionalXmlSchemaValueError<SP::Error>>
	{
		self.zero_or_one_inner(Ok(None), Some, |error| error)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn only_one(self) -> Result<SP::Item, OnlyOneXmlSchemaValueError<SP::Error>>
	{
		use OnlyOneXmlSchemaValueError::*;
		self.zero_or_one_inner(Err(Missing { xml_schema_value_kind: SP::Kind }), |item| item, |error| Optional(error))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	fn zero_or_one_inner<R, Error>(mut self, zero: Result<R, Error>, constructor: impl FnOnce(SP::Item) -> R, error: impl FnOnce(OptionalXmlSchemaValueError<SP::Error>) -> Error) -> Result<R, Error>
	{
		use OptionalXmlSchemaValueError::*;
		match self.len()
		{
			0 => zero,
			
			1 =>
			{
				let next = self.next();
				match unsafe { next.unwrap_unchecked() }
				{
					Err(cause) => Err(error(StrParse { cause, xml_schema_value_kind: SP::Kind })),
					
					Ok(item) => Ok(constructor(item))
				}
			}
			
			length @ _ => Err(error(TooMany { cause: MoreThanOneError { count: new_non_zero_usize(length) }, xml_schema_value_kind: SP::Kind }))
		}
	}
}
