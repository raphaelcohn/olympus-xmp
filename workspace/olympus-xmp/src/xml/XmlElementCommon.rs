// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Data common across XmlElement and parse errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlElementCommon
{
	namespace: Namespace,
	
	name: OwnedName,
	
	attributes: HashMap<XmlName<'static, 'static>, (String, Option<String>)>
}

impl XmlElementCommon
{
	#[inline(always)]
	fn new(namespace: Namespace, name: OwnedName, attributes: Vec<OwnedAttribute>, text_position: TextPosition) -> Result<Self, ParseError>
	{
		Ok
		(
			Self
			{
				namespace,
				name,
				attributes:
				{
					let mut attributes_map = HashMap::with_capacity(attributes.len());
					for attribute in attributes
					{
						let attribute_name = attribute.name;
						let xml_name = XmlName
						{
							namespace_uniform_resource_identifier: attribute_name.namespace.map(Owned),
							local_name: Owned(attribute_name.local_name),
						};
						
						let duplicate = attributes_map.insert(xml_name, (attribute.value, attribute_name.prefix)).is_some();
						if duplicate
						{
							return Err(ParseError::DuplicateAttribute(text_position))
						}
					}
					attributes_map
				},
			}
		)
	}
	
	#[inline(always)]
	fn has_no_attributes_in_namespace(&self, namespace_uniform_resource_identifier: Option<&str>) -> bool
	{
		for name in self.attributes.keys()
		{
			if name.has_namespace(namespace_uniform_resource_identifier)
			{
				return false
			}
		}
		true
	}
	
	#[inline(always)]
	fn get_attribute<'a>(&'a self, attribute_name: &XmlName) -> Option<&'a str>
	{
		let option = self.attributes.get(attribute_name);
		match option
		{
			Some((value, _prefix)) =>
			{
				let value = value.as_str();
				// Hack to break lifetime dependency on attribute_name.
				Some(unsafe { & * (value as *const str) })
			},
			
			None => None,
		}
	}
	
	#[inline(always)]
	fn has_name(&self, element_name: &XmlName) -> bool
	{
		element_name.matches_owned_name(&self.name)
	}
	
	#[inline(always)]
	fn does_not_have_name(&self, end_element_name: &OwnedName) -> bool
	{
		&self.name != end_element_name
	}
	
	#[inline(always)]
	fn write(&self, event_writer: &mut EventWriter<impl Write>) -> Result<Name, io::Error>
	{
		let name = self.name.borrow();
		event_writer.write_robustly(XmlWriteEvent::StartElement
		{
			name,
			attributes:
			{
				let mut attributes = Vec::with_capacity(self.attributes.len());
				for (xml_name, (value, prefix)) in &self.attributes
				{
					attributes.push(Attribute
					{
						name: xml_name.into_unowned(prefix.as_ref().map(|v| v.as_str())),
						value: value.as_str(),
					});
				}
				Owned(attributes)
			},
			// Could probably be recreated.
			namespace: Borrowed(&self.namespace)
		})?;
		Ok(name)
	}
}
