// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A hash fragment.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HashFragment<'a>(Cow<'a, str>);

impl<'a> Display for HashFragment<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.0.as_ref())
	}
}

impl<'a> TryToOwnInPlace for HashFragment<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for HashFragment<'a>
{
	type TryToOwned = HashFragment<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> TryToOwn for Option<HashFragment<'a>>
{
	type TryToOwned = Option<HashFragment<'static>>;
	
	#[inline(always)]
	fn try_to_own(self) -> Result<Self::TryToOwned, TryReserveError>
	{
		if let Some(value) = self
		{
			Ok(Some(value.try_to_own()?))
		}
		else
		{
			Ok(None)
		}
	}
}

impl<'a> const FromUnchecked<Cow<'a, str>> for HashFragment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: Cow<'a, str>) -> Self
	{
		Self(value)
	}
}

impl<'a> const FromUnchecked<&'a str> for HashFragment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a str) -> Self
	{
		Self(Cow::Borrowed(value))
	}
}

impl<'a> const FromUnchecked<String> for HashFragment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: String) -> Self
	{
		Self(Cow::Owned(value))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for HashFragment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for HashFragment<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: &'a [u8; Count]) -> Self
	{
		Self::from_unchecked(from_utf8_unchecked(value))
	}
}

impl<'a> Into<Cow<'a, str>> for HashFragment<'a>
{
	#[inline(always)]
	fn into(self) -> Cow<'a, str>
	{
		self.0
	}
}

impl<'a> const Borrow<str> for HashFragment<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const AsRef<str> for HashFragment<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.deref()
	}
}

impl<'a> const Deref for HashFragment<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a> HashFragment<'a>
{
	/// `ifragment  = *( ipchar / "/" / "?" )`.
	#[inline(always)]
	fn parse(mut remaining_utf8_bytes: &'a [u8], scheme_specific_parsing_rule: &SchemeSpecificParsingRule) -> Result<Self, HashFragmentParseError>
	{
		use HashFragmentParseError::*;
		
		let remaining_utf8_bytes = &mut remaining_utf8_bytes;
		
		if scheme_specific_parsing_rule.hash_fragment_should_not_be_present(remaining_utf8_bytes)
		{
			return Err(HashFragmentNotAllowedForScheme)
		}
		
		let mut string = StringSoFar::new_stack(remaining_utf8_bytes);
		
		loop
		{
			use Utf8CharacterLength::*;
			
			match StringSoFar::decode_next_utf8_validity_already_checked(remaining_utf8_bytes)
			{
				None => break,
				
				Some(character) => match character
				{
					ipchar_iunreserved_without_ucschar!() => string.push(character, One)?,
					ipchar_iunreserved_with_ucschar_2!()  => string.push(character, Two)?,
					ipchar_iunreserved_with_ucschar_3!()  => string.push(character, Three)?,
					ipchar_iunreserved_with_ucschar_4!()  => string.push(character, Four)?,
					ipchar_pct_encoded!()                 => string.push_forcing_heap_percent_encoded::<false>(remaining_utf8_bytes)?,
					ipchar_sub_delims!()                  => string.push(character, One)?,
					ipchar_other!()                       => string.push(character, One)?,
					
					SlashChar | QuestionMarkChar          => string.push(character, One)?,
					
					_ => return Err(InvalidCharacterInHashFragment(character)),
				},
			};
		}
		Ok(Self(string.to_cow()))
	}
}
