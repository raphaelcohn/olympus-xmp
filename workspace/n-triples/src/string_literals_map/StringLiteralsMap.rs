// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// If an entry is present, its value will never be an empty Vec.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringLiteralsMap<'a, K: Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash>(MutableKeyHashMap<K, NonEmptyVec<Cow<'a, str>>>);

impl<'a, K: Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> Default for StringLiteralsMap<'a, K>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(MutableKeyHashMap::default())
	}
}

impl<'a, K: Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + TryToOwnInPlace> TryToOwnInPlace for StringLiteralsMap<'a, K>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.0.try_to_own_in_place()
	}
}

impl<'a, K: 'static + Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + TryToOwn> TryToOwn for StringLiteralsMap<'a, K>
{
	type TryToOwned = StringLiteralsMap<'static, K>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a, K: Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> StringLiteralsMap<'a, K>
{
	/// Get.
	#[inline(always)]
	pub fn get(&self, key: &K) -> Option<&NonEmptyVec<Cow<'a, str>>>
	{
		self.0.get(key)
	}
	
	/// Iterator.
	#[inline(always)]
	pub fn iter(&self) -> impl Iterator<Item=(&K, &NonEmptyVec<Cow<'a, str>>)>
	{
		self.0.iter().map(|(key, value)| (key.borrow(), value))
	}
	
	#[inline(always)]
	pub(super) fn entry(&mut self, key: K) -> &mut NonEmptyVec<Cow<'a, str>>
	{
		self.0.entry(MutableKey::new(key)).or_default()
	}
}

#[allow(suspicious_auto_trait_impls)]
unsafe impl<'a> Send for StringLiteralsMap<'a, NaiveIetfBcp47LanguageTag<'a>>
{
}

#[allow(suspicious_auto_trait_impls)]
unsafe impl<'a> Sync for StringLiteralsMap<'a, NaiveIetfBcp47LanguageTag<'a>>
{
}

#[allow(suspicious_auto_trait_impls)]
unsafe impl<'a, const PathDepth: usize> Send for StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>
{
}

#[allow(suspicious_auto_trait_impls)]
unsafe impl<'a, const PathDepth: usize> Sync for StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>
{
}

impl<'a: 'string_literals_map, 'string_literals_map, const PathDepth: usize> StringLiteralsMap<'a, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn xml_schema_string(&'string_literals_map self) -> Result<&'string_literals_map str, OnlyOneError<Infallible>>
	{
		self.get_xml_schema_strings().only_one::<Infallible>()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn xml_schema_boolean(&'string_literals_map self) -> Result<bool, OnlyOneError<ParseBoolError>>
	{
		Self::xml_schema_result(self.get_xml_schema_booleans())
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn xml_schema_integer(&'string_literals_map self) -> Result<i64, OnlyOneError<ParseIntError>>
	{
		Self::xml_schema_result(self.get_xml_schema_integers())
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn xml_schema_date_time(&'string_literals_map self) -> Result<DateTime<FixedOffset>, OnlyOneError<ParseDateTimeError>>
	{
		Self::xml_schema_result(self.get_xml_schema_date_time())
	}
	
	#[inline(always)]
	fn xml_schema_result<O, E: error::Error>(iterator: StringLiteralsMapValuesIterator<'a, 'string_literals_map, Result<O, E>, impl Copy + FnOnce(&'string_literals_map str) -> Result<O, E>>) -> Result<O, OnlyOneError<E>>
	{
		match iterator.only_one::<E>()
		{
			Ok(Ok(value)) => Ok(value),
			
			Ok(Err(parse_error)) => Err(OnlyOneError::from(parse_error)),
			
			Err(error) => Err(error)
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn optional_xml_schema_string(&'string_literals_map self) -> Result<Option<&'string_literals_map str>, ZeroOrOneError<Infallible>>
	{
		self.get_xml_schema_strings().zero_or_one::<Infallible>()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn optional_xml_schema_boolean(&'string_literals_map self) -> Result<Option<bool>, ZeroOrOneError<ParseBoolError>>
	{
		Self::optional_xml_schema_result(self.get_xml_schema_booleans())
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn optional_xml_schema_integer(&'string_literals_map self) -> Result<Option<i64>, ZeroOrOneError<ParseIntError>>
	{
		Self::optional_xml_schema_result(self.get_xml_schema_integers())
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn optional_xml_schema_date_time(&'string_literals_map self) -> Result<Option<DateTime<FixedOffset>>, ZeroOrOneError<ParseDateTimeError>>
	{
		Self::optional_xml_schema_result(self.get_xml_schema_date_time())
	}
	
	#[inline(always)]
	fn optional_xml_schema_result<O, E: error::Error>(iterator: StringLiteralsMapValuesIterator<'a, 'string_literals_map, Result<O, E>, impl Copy + FnOnce(&'string_literals_map str) -> Result<O, E>>) -> Result<Option<O>, ZeroOrOneError<E>>
	{
		match iterator.zero_or_one()
		{
			Ok(None) => Ok(None),
			
			Ok(Some(Ok(ok))) => Ok(Some(ok)),
			
			Ok(Some(Err(error))) => Err(ZeroOrOneError::Parse(error)),
			
			Err(error) => Err(error),
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_xml_schema_strings(&'string_literals_map self) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, &'string_literals_map str, impl Copy + FnOnce(&'string_literals_map str) -> &'string_literals_map str>
	{
		self.get_inner(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaString, |value| value)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_xml_schema_booleans(&'string_literals_map self) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, Result<bool, ParseBoolError>, impl Copy + FnOnce(&'string_literals_map str) -> Result<bool, ParseBoolError>>
	{
		self.get_inner(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaBoolean, bool::from_str)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_xml_schema_integers(&'string_literals_map self) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, Result<i64, ParseIntError>, impl Copy + FnOnce(&'string_literals_map str) -> Result<i64, ParseIntError>>
	{
		self.get_inner(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaInteger, i64::from_str)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_xml_schema_date_time(&'string_literals_map self) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, Result<DateTime<FixedOffset>, ParseDateTimeError>, impl Copy + FnOnce(&'string_literals_map str) -> Result<DateTime<FixedOffset>, ParseDateTimeError>>
	{
		self.get_inner(AbsoluteInternationalizedResourceIdentifier::<PathDepth>::XmlSchemaDateTime, DateTime::parse_from_rfc3339)
	}
	
	#[inline(always)]
	fn get_inner<Item, Parser: Copy + FnOnce(&'string_literals_map str) -> Item>(&'string_literals_map self, key: AbsoluteInternationalizedResourceIdentifier<'static, PathDepth>, parser: Parser) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, Item, Parser>
	{
		const Empty: &'static [Cow<'static, str>] = &[];
		
		match self.get(unsafe { transmute(&key) })
		{
			None => StringLiteralsMapValuesIterator(Empty, parser),
			
			Some(non_empty) => StringLiteralsMapValuesIterator(non_empty.as_slice(), parser),
		}
	}
}
