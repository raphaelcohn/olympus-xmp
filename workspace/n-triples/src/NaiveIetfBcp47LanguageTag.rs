// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A naive IETF BCP-47 language tag structure.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NaiveIetfBcp47LanguageTag<'a>
{
	first_component: Cow<'a, str>,
	
	subsequent_components: ConstSmallVec<Cow<'a, str>, 4>
}

impl<'a> const FromUnchecked<Cow<'a, str>> for NaiveIetfBcp47LanguageTag<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self
		{
			first_component: value,
		
			subsequent_components: ConstSmallVec::default(),
		}
	}
}

impl<'a> const FromUnchecked<&'a str> for NaiveIetfBcp47LanguageTag<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self::from_unchecked(Cow::Borrowed(value))
	}
}

impl<'a> const FromUnchecked<String> for NaiveIetfBcp47LanguageTag<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self::from_unchecked(Cow::Owned(value))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for NaiveIetfBcp47LanguageTag<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for NaiveIetfBcp47LanguageTag<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8; Count]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a> Display for NaiveIetfBcp47LanguageTag<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.first_component.as_ref())?;
		
		for item in self.subsequent_components.deref()
		{
			write!(f, "-{}", item.as_ref())?;
		}
		
		Ok(())
	}
}

impl<'a> TryToOwnInPlace for NaiveIetfBcp47LanguageTag<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.first_component.try_to_own_in_place()?;
		self.subsequent_components.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for NaiveIetfBcp47LanguageTag<'a>
{
	type TryToOwned = NaiveIetfBcp47LanguageTag<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> NaiveIetfBcp47LanguageTag<'a>
{
	// `LANGTAG ::= `[a-zA-Z]+ ('-' [a-zA-Z0-9]+)*`.
	#[inline(always)]
	fn parse(mut naive_ietf_bcp_47_language_tag_bytes: &'a [u8]) -> Result<Self, NaiveIetfBcp47LanguageTagParseError>
	{
		let remaining_bytes = &mut naive_ietf_bcp_47_language_tag_bytes;
		let (first_component, mut has_more_components) = Self::parse_first_component(remaining_bytes)?;
		
		let mut subsequent_components = ConstSmallVec::default();
		let mut component_index = 1;
		while has_more_components
		{
			let (subsequent_component, has_more_components_after_this_one) = Self::parse_subsequent_component(remaining_bytes, component_index)?;
			subsequent_components.try_reserve_push(subsequent_component)?;
			has_more_components = has_more_components_after_this_one;
			component_index += 1;
		}
		
		Ok
		(
			Self
			{
				first_component,
				subsequent_components,
			}
		)
	}
	
	#[inline(always)]
	fn parse_first_component(remaining_bytes: &mut &'a [u8]) -> Result<(Cow<'a, str>, bool), NaiveIetfBcp47LanguageTagParseError>
	{
		let mut string = StringSoFar::new_stack(remaining_bytes);
		let has_more_components = loop
		{
			match get_0(remaining_bytes)
			{
				None => break false,
				
				Some(byte) => match byte
				{
					Hyphen => break true,
					
					A ..= Z => string.push_forcing_heap_ascii_to_lower_case(byte as char)?,
					
					a ..= z => string.push_ascii(byte as char)?,
					
					_ => return Err(NaiveIetfBcp47LanguageTagParseError::InvalidCharacter(byte))
				}
			}
		};
		Self::finish_component(string, has_more_components, 0)
	}
	
	#[inline(always)]
	fn parse_subsequent_component(remaining_bytes: &mut &'a [u8], component_index: usize) -> Result<(Cow<'a, str>, bool), NaiveIetfBcp47LanguageTagParseError>
	{
		let mut string = StringSoFar::new_stack(remaining_bytes);
		let has_more_components = loop
		{
			match get_0(remaining_bytes)
			{
				None => break false,
				
				Some(byte) => match byte
				{
					Hyphen => break true,
					
					_0 ..= _9 => string.push_ascii(byte as char)?,
					
					A ..= Z => string.push_forcing_heap_ascii_to_lower_case(byte as char)?,
					
					a ..= z => string.push_ascii(byte as char)?,
					
					_ => return Err(NaiveIetfBcp47LanguageTagParseError::InvalidCharacter(byte))
				}
			}
		};
		Self::finish_component(string, has_more_components, component_index)
	}
	
	#[inline(always)]
	fn finish_component(string: StringSoFar<'a>, has_more_components: bool, component_index: usize) -> Result<(Cow<'a, str>, bool), NaiveIetfBcp47LanguageTagParseError>
	{
		let cow = string.to_cow();
		if cow.is_empty()
		{
			return Err(NaiveIetfBcp47LanguageTagParseError::EmptyComponent { component_index })
		}
		Ok((cow, has_more_components))
	}
}

impl NaiveIetfBcp47LanguageTag<'static>
{
	/// English language tag.
	pub const en: Self = unsafe { Self::from_unchecked("en") };
}
