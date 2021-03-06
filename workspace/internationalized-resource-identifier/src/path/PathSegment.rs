// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A (potentially empty) path segment.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PathSegment<'a>(Cow<'a, str>);

impl<'a> TryToOwnInPlace for PathSegment<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for PathSegment<'a>
{
	type TryToOwned = PathSegment<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> const FromUnchecked<Cow<'a, str>> for PathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const FromUnchecked<&'a str> for PathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self(Cow::Borrowed(value))
	}
}

impl<'a> const FromUnchecked<String> for PathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self(Cow::Owned(value))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for PathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for PathSegment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8; Count]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a> const From<NonEmptyPathSegment<'a>> for PathSegment<'a>
{
	#[inline(always)]
	fn from(value: NonEmptyPathSegment<'a>) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl<'a> Into<Cow<'a, str>> for PathSegment<'a>
{
	#[inline(always)]
	fn into(self) -> Cow<'a, str>
	{
		self.0
	}
}

impl<'a> const Borrow<str> for PathSegment<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const AsRef<str> for PathSegment<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const Deref for PathSegment<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a> Display for PathSegment<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		self.display_fmt(f)
	}
}

impl<'a> PercentEncodable<'a> for PathSegment<'a>
{
	#[inline(always)]
	fn as_str(&self) -> &str
	{
		self.0.as_ref()
	}
	
	#[inline(always)]
	fn percent_encode_ascii(ascii_byte: u8) -> bool
	{
		match ascii_byte
		{
			A ..= Z | a ..= z | _0 ..= _9 | Hyphen | Period | Underscore | Tilde => false,
			
			ExclamationMark | DollarSign | Ampersand | Apostrophe | OpenRoundBracket | CloseRoundBracket | Asterisk | PlusSign | Comma | Semicolon | EqualsSign => false,
			
			Colon | AtSign => false,
			
			_ => true,
		}
	}
}

impl<'a> PathSegment<'a>
{
	#[inline(always)]
	fn decode_percent_encoded_path_segment(mut percent_encoded_path_segment: &'a str) -> Result<Self, PathSegmentParseError>
	{
		let percent_encoded_path_segment = &mut percent_encoded_path_segment;
		let string = Utf8SequencesParser::new_stack(percent_encoded_path_segment);
		Self::decode_percent_encoded_path_segment_common(string, percent_encoded_path_segment, Self)
	}
	
	/// [RFC 3987, Section 2.2](https://datatracker.ietf.org/doc/html/rfc3987#section-2.2).
	///
	/// `isegment    = *ipchar`.
	/// `isegment-nz = 1*ipchar`.
	#[inline(always)]
	fn decode_percent_encoded_path_segment_common<R>(mut string: Utf8SequencesParser<'a>, remaining: &mut &'a str, constructor: impl FnOnce(Cow<'a, str>) -> R) -> Result<R, PathSegmentParseError>
	{
		loop
		{
			match remaining.decode_next_utf8_validity_already_checked()
			{
				None => break,
				
				Some(Utf8SequenceAndCharacter(utf8_sequence, character)) => match character
				{
					ipchar_iunreserved_without_ucschar!() => string.push_ascii_character(character)?,
					ipchar_iunreserved_with_ucschar_2!()  => string.push_utf8_sequence_enum_2(utf8_sequence)?,
					ipchar_iunreserved_with_ucschar_3!()  => string.push_utf8_sequence_enum_3(utf8_sequence)?,
					ipchar_iunreserved_with_ucschar_4!()  => string.push_utf8_sequence_enum_4(utf8_sequence)?,
					ipchar_pct_encoded!()                 => string.push_forcing_heap_percent_encoded::<PathSegmentParseError, false>(remaining)?,
					ipchar_sub_delims!()                  => string.push_ascii_character(character)?,
					ipchar_other!()                       => string.push_ascii_character(character)?,
					
					_ => return Err(PathSegmentParseError::InvalidCharacter(character)),
				}
			}
		}
		
		Ok(constructor(string.to_cow()))
	}
}

impl PathSegment<'static>
{
	pub(super) const _1999: Self = unsafe { Self::from_unchecked("1999") };
	
	pub(super) const _2001: Self = unsafe { Self::from_unchecked("2001") };
	
	pub(super) const _2002: Self = unsafe { Self::from_unchecked("2002") };
	
	pub(super) const _2004: Self = unsafe { Self::from_unchecked("2004") };
	
	pub(super) const _02: Self = unsafe { Self::from_unchecked("02") };
	
	pub(super) const _07: Self = unsafe { Self::from_unchecked("07") };
	
	pub(super) const _22_rdf_syntax_ns: Self = unsafe { Self::from_unchecked("22-rdf-syntax-ns") };
	
	pub(super) const owl: Self = unsafe { Self::from_unchecked("owl") };
	
	pub(super) const skos: Self = unsafe { Self::from_unchecked("skos") };
	
	pub(super) const core: Self = unsafe { Self::from_unchecked("core") };
	
	pub(super) const XMLSchema: Self = unsafe { Self::from_unchecked("XMLSchema") };
	
	pub(super) const dc: Self = unsafe { Self::from_unchecked("dc") };
	
	pub(super) const terms: Self = unsafe { Self::from_unchecked("terms") };
}
