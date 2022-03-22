// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
pub struct XmpElement<'a, 'name, 'namespace, 'local_name>
{
	xml_element: &'a XmlElement,
	
	path: XmpElementPath<'name, 'namespace, 'local_name>,
}

impl<'a, 'name, 'namespace, 'local_name> XmpElement<'a, 'name, 'namespace, 'local_name>
{
	/// This logic works because the elements are static references and hence never dropped and thus intrinsically implement the 'Copy' trait.
	///
	/// In effect, it is an optimized 'memcpy()'.
	#[inline(always)]
	fn path(&self) -> XmpElementPath<'name, 'namespace, 'local_name>
	{
		let length = self.path.len();
		let mut copy = ArrayVec::new_const();
		let source_pointer = self.path.as_ptr();
		unsafe
		{
			copy_nonoverlapping(source_pointer, copy.as_mut_ptr(), length);
			copy.set_len(length)
		}
		copy
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn root(xml_document: &'a XmlDocument, element_name: &'name XmlName<'namespace, 'local_name>) -> Result<Self, XmpValidationError<'name, 'namespace, 'local_name>>
	{
		let path = ArrayVec::new_const();
		Self::child_(element_name, xml_document.root(), path)
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn child(&self, element_name: &'name XmlName<'namespace, 'local_name>) -> Result<Self, XmpValidationError<'name, 'namespace, 'local_name>>
	{
		let path = self.path.clone();
		Self::child_(element_name, self.xml_element, path)
	}
	
	#[inline(always)]
	fn child_(element_name: &'name XmlName<'namespace, 'local_name>, xml_element: &'a XmlElement, mut path: XmpElementPath<'name, 'namespace, 'local_name>) -> Result<Self, XmpValidationError<'name, 'namespace, 'local_name>>
	{
		path.push(element_name);
		
		let child_path = unsafe { from_raw_parts(element_name as *const XmlName<'namespace, 'local_name>, 1) };
		match xml_element.get_only_element(child_path)
		{
			Ok(xml_element) => Ok
			(
				Self
				{
					xml_element,
					path,
				}
			),
			
			Err(cause) => Err(XmpValidationError::MissingOnlyElement { path, cause }),
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn has_attribute_with_expected_value<XAV: XmpAttributeValue<'a> + Eq>(&self, attribute_name: &'name XmlName<'namespace, 'local_name>, expected: XAV) -> Result<(), XmpValidationError<'name, 'namespace, 'local_name>>
	{
		if self.get_attribute_or_error::<XAV>(attribute_name)? != expected
		{
			Err(XmpValidationError::AttributeDoesNotHaveExpectedValue { path: self.path(), attribute_name })
		}
		else
		{
			Ok(())
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn has_attribute_with_any_value<XAV: XmpAttributeValue<'a> + Eq>(&self, attribute_name: &'name XmlName<'namespace, 'local_name>) -> Result<(), XmpValidationError<'name, 'namespace, 'local_name>>
	{
		self.get_attribute_or_error::<XAV>(attribute_name).map(|_| ())
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_attribute_or_error<XAV: XmpAttributeValue<'a>>(&self, attribute_name: &'name XmlName<'namespace, 'local_name>) -> Result<XAV, XmpValidationError<'name, 'namespace, 'local_name>>
	{
		match self.get_attribute::<XAV>(attribute_name)?
		{
			None => Err(XmpValidationError::MissingAttribute { path: self.path(), attribute_name }),
			
			Some(attribute_value) => Ok(attribute_value)
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_attribute_str(&self, attribute_name: &'name XmlName<'namespace, 'local_name>) -> Option<&'a str>
	{
		let attribute_value = self.get_attribute::<&str>(attribute_name);
		unsafe { attribute_value.unwrap_unchecked() }
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn does_not_have_any_attributes_in_namespace<const namespace_prefix: &'static str>(&self, namespace_uniform_resource_identifier: Option<&str>) -> Result<(), XmpValidationError<'name, 'namespace, 'local_name>>
	{
		if self.xml_element.has_no_attributes_in_namespace(namespace_uniform_resource_identifier)
		{
			Ok(())
		}
		else
		{
			Err(XmpValidationError::HasAttributesInNamespace { path: self.path(), namespace_prefix })
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn does_not_have_attribute_which_is_obsolete<XAV: XmpAttributeValue<'a>>(&self, attribute_name: &'name XmlName<'namespace, 'local_name>) -> Result<(), XmpValidationError<'name, 'namespace, 'local_name>>
	{
		if let Some(_) = self.get_attribute::<XAV>(attribute_name)?
		{
			Err(XmpValidationError::HasAttributeWhichIsObsolete { path: self.path(), attribute_name })
		}
		else
		{
			Ok(())
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn does_not_have_attribute(&self, attribute_name: &'name XmlName<'namespace, 'local_name>) -> Result<(), XmpValidationError<'name, 'namespace, 'local_name>>
	{
		if self.get_raw_attribute(attribute_name).is_none()
		{
			Ok(())
		}
		else
		{
			Err(XmpValidationError::HasAttributeWhichShouldNotBePresent { path: self.path(), attribute_name })
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_attribute<XAV: XmpAttributeValue<'a>>(&self, attribute_name: &'name XmlName<'namespace, 'local_name>) -> Result<Option<XAV>, XmpValidationError<'name, 'namespace, 'local_name>>
	{
		let raw_attribute_value = match self.get_raw_attribute(attribute_name)
		{
			None => return Ok(None),
			
			Some(raw_attribute_value) => raw_attribute_value
		};
		
		match XAV::parse(raw_attribute_value)
		{
			Ok(attribute_value) => Ok(Some(attribute_value)),
			
			Err(cause) => Err(XmpValidationError::CouldNotParseAttribute { path: self.path(), attribute_name, cause: XAV::into_xmp_attribute_value_parse_error(cause) })
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn get_raw_attribute(&self, attribute_name: &'name XmlName<'namespace, 'local_name>) -> Option<&'a str>
	{
		self.xml_element.get_attribute(attribute_name)
	}
}
