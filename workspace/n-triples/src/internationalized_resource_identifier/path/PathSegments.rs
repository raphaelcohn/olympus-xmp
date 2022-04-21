// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Path segments.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PathSegments<'a, const PathDepth: usize>(ConstSmallVec<PathSegment<'a>, PathDepth>);

impl<'a: 'b, 'b, const PathDepth: usize> IntoIterator for &'b PathSegments<'a, PathDepth>
{
	type Item = &'b PathSegment<'a>;
	
	type IntoIter = std::slice::Iter<'b, PathSegment<'a>>;
	
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		self.deref().iter()
	}
}

impl<'a, const PathDepth: usize> Display for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		let slice = self.0.deref();
		for path_segment in slice.into_iter()
		{
			write!(f, "/{}", path_segment)?;
		}
		Ok(())
	}
}

impl<'a, const PathDepth: usize> Deref for PathSegments<'a, PathDepth>
{
	type Target = [PathSegment<'a>];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a, const PathDepth: usize> DerefMut for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.0.deref_mut()
	}
}

impl<'a, const PathDepth: usize> TryToOwnInPlace for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a, const PathDepth: usize> TryToOwn for PathSegments<'a, PathDepth>
{
	type TryToOwned = PathSegments<'static, PathDepth>;
	
	#[inline(always)]
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.0.try_to_own().map(PathSegments::<'static, PathDepth>)
	}
}

impl<'a, P, const PathDepth: usize, const M: usize> const FromUnchecked<[P; M]> for PathSegments<'a, PathDepth>
where PathSegment<'a>: ~const FromUnchecked<P>,
{
	#[inline(always)]
	unsafe fn from_unchecked(array: [P; M]) -> Self
	{
		if M > PathDepth
		{
			panic!("array is too large to allocate on the stack, and heap allocation is not possible at build time")
		}
		
		let mut into_array: [MaybeUninit<PathSegment<'a>>; M] = MaybeUninit::uninit_array();
		let array_pointer = array.as_ptr();
		let mut index = 0;
		while index != M
		{
			let from_unchecked = read(array_pointer.add(index));
			let into = PathSegment::from_unchecked(from_unchecked);
			let _ = into_array[index].write(into);
			index += 1;
		}
		
		let _ = ManuallyDrop::new(array);
		
		// Substitute for non-const MaybeUninit::array_assume_init().
		#[inline(always)]
		const unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N]
		{
			let init = (&array as *const _ as *const [T; N]).read();
			let _ = ManuallyDrop::new(array);
			init
		}
		
		Self::from(array_assume_init(into_array))
	}
}

impl<'a, const PathDepth: usize, const M: usize> const From<[PathSegment<'a>; M]> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn from(array: [PathSegment<'a>; M]) -> Self
	{
		if M > PathDepth
		{
			panic!("array is too large to allocate on the stack, and heap allocation is not possible at build time")
		}
		
		Self::from(ConstSmallVec::from_panic_array(array))
	}
}

impl<'a, const PathDepth: usize> const From<ConstSmallVec<PathSegment<'a>, PathDepth>> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn from(value: ConstSmallVec<PathSegment<'a>, PathDepth>) -> Self
	{
		Self(value)
	}
}

impl<'a, const PathDepth: usize> Borrow<[PathSegment<'a>]> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn borrow(&self) -> &[PathSegment<'a>]
	{
		self.deref()
	}
}

impl<'a, const PathDepth: usize> AsRef<[PathSegment<'a>]> for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn as_ref(&self) -> &[PathSegment<'a>]
	{
		self.deref()
	}
}

impl<'a, const PathDepth: usize> const Default for PathSegments<'a, PathDepth>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(ConstSmallVec::default())
	}
}

impl<'a, const PathDepth: usize> PathSegments<'a, PathDepth>
{
	/// Empty.
	pub const Empty: Self = Self::default();
	
	/// Appends a path segment.
	///
	/// Can fail with an `Err()` if there is not enough heap memory.
	#[inline(always)]
	pub fn with_path_segment(&mut self, path_segment: PathSegment<'a>) -> Result<(), TryReserveError>
	{
		self.0.try_reserve_push(path_segment)
	}
	
	/// Appends a path segment.
	///
	/// Can fail with an `Err()` if there is not enough memory to allocate on the stack, i.e. the length (`len()`) is already equal to `PathDepth`.
	#[inline(always)]
	pub const fn with_path_segment_const(&mut self, path_segment: PathSegment<'a>) -> Result<(), PathSegment<'a>>
	{
		self.0.try_stack_push(path_segment)
	}
	
	/// Removes `other`.
	#[inline(always)]
	pub fn remove(&self, prefix: &PathSegments<PathDepth>) -> Option<&[PathSegment<'a>]>
	{
		let this = self.0.deref();
		let other = prefix.0.deref();
		
		let other_length = other.len();
		if this.len() < other_length
		{
			return None
		}
		
		for index in 0 .. other_length
		{
			if this.get_unchecked_safe(index) != other.get_unchecked_safe(index)
			{
				return None
			}
		}
		Some(this.get_unchecked_range_safe(other_length .. ))
	}
	
	/// Assumes that on input `remaining_utf8_bytes` is positioned just to the right of any `Slash`, i.e. `Slash` would be at `remaining_utf8_bytes[-1]`.
	#[inline(always)]
	pub(super) fn parse(&mut self, mut remaining_utf8_bytes: &'a [u8]) -> Result<ParseNextAfterHierarchy<'a>, PathSegmentsParseError>
	{
		let parse_next_after_hierarchy = loop
		{
			match memchr3(QuestionMark, Hash, Slash, remaining_utf8_bytes)
			{
				None =>
				{
					self.decode_percent_encoded_path_segment(remaining_utf8_bytes)?;
					break ParseNextAfterHierarchy::NoQueryNoFragment
				}
				
				Some(index) =>
				{
					self.decode_percent_encoded_path_segment(remaining_utf8_bytes.before_index(index))?;
					let after_path_segment_bytes = remaining_utf8_bytes.after_index(index);
					
					match remaining_utf8_bytes.get_unchecked_value_safe(index)
					{
						QuestionMark => break ParseNextAfterHierarchy::query(after_path_segment_bytes),
						
						Hash => break ParseNextAfterHierarchy::fragment_no_query(after_path_segment_bytes),
						
						Slash =>
						{
							remaining_utf8_bytes = after_path_segment_bytes;
						}
						
						_ => unreachable_code_const("memchr3"),
					}
				}
			}
		};
		Ok(parse_next_after_hierarchy)
	}
	
	#[inline(always)]
	fn decode_percent_encoded_path_segment(&mut self, percent_encoded_path_segment_utf8_bytes: &'a [u8]) -> Result<(), PathSegmentsParseError>
	{
		use PathSegmentsParseError::*;
		
		let path_segment = PathSegment::decode_percent_encoded_path_segment(percent_encoded_path_segment_utf8_bytes).map_err(PathSegmentParse)?;
		self.0.try_reserve_push(path_segment).map_err(OutOfMemory)
	}
}
