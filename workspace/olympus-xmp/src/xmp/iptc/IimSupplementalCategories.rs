// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Obsolete as of IIM version 4.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IimSupplementalCategories(ArrayVec<IimCategoryCode, {IimSupplementalCategories::Maximum}>);

impl<'a> XmpAttributeValue<'a> for IimSupplementalCategories
{
	type Error = IimSupplementalCategoriesParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		use IimSupplementalCategoriesParseError::*;
		
		const Space: char = ' ';
		let mut array_vec = ArrayVec::new_const();
		let mut iterator = raw.split(Space);
		for index in 0 .. Self::Maximum
		{
			match iterator.next()
			{
				None => return Ok(Self(array_vec)),
				
				Some(potential) =>
				{
					let category_code = IimCategoryCode::parse(potential).map_err(|cause| InvalidSupplementalCategory { index, cause })?;
					unsafe { array_vec.push_unchecked(category_code) };
				}
			}
		}
		
		if iterator.next().is_some()
		{
			Err(MoreThanEightSupplementalCategories)
		}
		else
		{
			Ok(Self(array_vec))
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::IimSupplementalCategories(error)
	}
}

impl IimSupplementalCategories
{
	const Maximum: usize = 8;
}
