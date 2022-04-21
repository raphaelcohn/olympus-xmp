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
	pub fn optional_xml_schema_value_as_domain_type<SP: StrParser<'string_literals_map>, DomainType: TryFrom<SP::Item, Error=TryFromError>, TryFromError: 'static + error::Error>(&'string_literals_map self, parser: XmlSchemaValueParser<'string_literals_map, SP, PathDepth>) -> Result<Option<DomainType>, OptionalXmlSchemaValueError<StringLiteralToDomainTypeParseError<SP::Error, TryFromError>>>
	where <SP as StrParser<'string_literals_map>>::Error: 'static,
	{
		self.xml_schema_values_as_domain_type(parser).zero_or_one()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn only_one_xml_schema_value_as_domain_type<SP: StrParser<'string_literals_map>, DomainType: TryFrom<SP::Item, Error=TryFromError>, TryFromError: 'static + error::Error>(&'string_literals_map self, parser: XmlSchemaValueParser<'string_literals_map, SP, PathDepth>) -> Result<DomainType, OnlyOneXmlSchemaValueError<StringLiteralToDomainTypeParseError<SP::Error, TryFromError>>>
	where <SP as StrParser<'string_literals_map>>::Error: 'static,
	{
		self.xml_schema_values_as_domain_type(parser).only_one()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn xml_schema_values_as_domain_type<SP: StrParser<'string_literals_map>, DomainType: TryFrom<SP::Item, Error=TryFromError>, TryFromError: error::Error>(&'string_literals_map self, parser: XmlSchemaValueParser<'string_literals_map, SP, PathDepth>) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, TryFromStrParser<'string_literals_map, SP, DomainType, TryFromError>>
	{
		parser.parse_domain_type(self)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn optional_xml_schema_value<SP: StrParser<'string_literals_map>>(&'string_literals_map self, parser: XmlSchemaValueParser<'string_literals_map, SP, PathDepth>) -> Result<Option<SP::Item>, OptionalXmlSchemaValueError<SP::Error>>
	{
		self.xml_schema_values(parser).zero_or_one()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn only_one_xml_schema_value<SP: StrParser<'string_literals_map>>(&'string_literals_map self, parser: XmlSchemaValueParser<'string_literals_map, SP, PathDepth>) -> Result<SP::Item, OnlyOneXmlSchemaValueError<SP::Error>>
	{
		self.xml_schema_values(parser).only_one()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn xml_schema_values<SP: StrParser<'string_literals_map>>(&'string_literals_map self, parser: XmlSchemaValueParser<'string_literals_map, SP, PathDepth>) -> StringLiteralsMapValuesIterator<'a, 'string_literals_map, SP>
	{
		parser.parse_from_str_like(self)
	}
}
