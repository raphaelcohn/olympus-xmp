// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT.


/// An XML element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlElement
{
	common: XmlElementCommon,
	
	children: Vec<XmlNode>,
}

impl XmlDocumentOrXmlElement for XmlElement
{
	#[inline(always)]
	fn get_attribute<'a, 'namespace, 'local_name>(&'a self, attribute_name: &'a XmlName<'namespace, 'local_name>) -> Option<&'a str>
	where 'a: 'namespace, 'a: 'local_name
	{
		self.common.get_attribute(attribute_name)
	}
	
	#[inline(always)]
	fn get_only_element<'a>(&'a self, path: &[XmlName]) -> Result<&'a XmlElement, NotExactlyOneElementError>
	{
		let mut only_element = None;
		self.get_only_element_inner(path, &mut only_element)?;
		only_element.ok_or(NotExactlyOneElementError::NoElementsForName)
	}
	
	#[inline(always)]
	fn get_elements<'a>(&'a self, path: &[XmlName]) -> Vec<&'a XmlElement>
	{
		let mut results = Vec::new();
		self.get_elements_inner(path, &mut results);
		results
	}
	
	#[inline(always)]
	fn has_no_attributes_in_namespace(&self, namespace_uniform_resource_identifier: Option<&str>) -> bool
	{
		self.common.has_no_attributes_in_namespace(namespace_uniform_resource_identifier)
	}
}

impl XmlElement
{
	/// Get attribute.
	#[inline(always)]
	pub fn get_attribute<'a>(&'a self, attribute_name: &XmlName) -> Option<&'a str>
	{
		self.common.get_attribute(attribute_name)
	}
	
	fn get_elements_inner<'a>(&'a self, path: &[XmlName], results: &mut Vec<&'a Self>)
	{
		debug_assert_ne!(path.len(), 0, "Empty paths are not acceptable");
		
		if self.does_not_have_name(&path[0])
		{
			return
		}
		
		if path.len() == 1
		{
			return results.push(self)
		}
		
		let path = &path[1 .. ];
		for child_node in &self.children
		{
			use XmlNode::Element;
			match child_node
			{
				Element(child_element) => child_element.get_elements_inner(path, results),
				
				_ => continue,
			}
		}
	}
	
	fn get_only_element_inner<'a>(&'a self, path: &[XmlName], only_element: &mut Option<&'a Self>) -> Result<(), NotExactlyOneElementError>
	{
		debug_assert_ne!(path.len(), 0, "Empty paths are not acceptable");
		
		if self.does_not_have_name(&path[0])
		{
			return Ok(())
		}
		
		if path.len() == 1
		{
			if only_element.is_some()
			{
				return Err(NotExactlyOneElementError::MoreThanOneElementForName)
			}
			*only_element = Some(self);
			return Ok(())
		}
		
		let path = &path[1 .. ];
		for child_node in &self.children
		{
			use XmlNode::Element;
			match child_node
			{
				Element(child_element) => child_element.get_only_element_inner(path, only_element)?,
				
				_ => continue,
			}
		}
		
		Ok(())
	}
	
	/// Get text.
	#[inline(always)]
	pub fn get_only_text<'a>(&'a self) -> Result<&'a str, NotExactlyOneTextError>
	{
		use NotExactlyOneTextError::*;
		
		let mut only_text = None;
		for child_node in &self.children
		{
			use XmlNode::Text;
			match child_node
			{
				Text(text) =>
				{
					if only_text.is_none()
					{
						only_text = Some(text.as_ref());
					}
					else
					{
						return Err(MoreThanOneText)
					}
				}
				
				_ => continue,
			}
		}
		only_text.ok_or(NoTexts)
	}
	
	/// Has name.
	#[inline(always)]
	pub fn does_not_have_name(&self, xml_element_name: &XmlName) -> bool
	{
		!self.has_name(xml_element_name)
	}
	
	/// Has name.
	#[inline(always)]
	pub fn has_name(&self, element_name: &XmlName) -> bool
	{
		self.common.has_name(element_name)
	}
	
	fn write(&self, event_writer: &mut EventWriter<impl Write>) -> Result<(), io::Error>
	{
		let name = self.common.write(event_writer)?;
		
		for child in &self.children
		{
			child.write(event_writer)?;
		}
		
		event_writer.write_robustly(XmlWriteEvent::EndElement { name: Some(name) })
	}
	
	fn parse(event_reader: &mut EventReader<impl Read>, namespace: Namespace, name: OwnedName, attributes: Vec<OwnedAttribute>) -> Result<Self, ParseError>
	{
		let mut children = Vec::new();
		let common = XmlElementCommon::new(namespace, name, attributes, event_reader.position())?;
		loop
		{
			match event_reader.next()?
			{
				// TODO: This uses recursion, which isn't ideal.
				StartElement { name, attributes, namespace } => children.push(XmlNode::Element(Self::parse(event_reader, namespace, name, attributes)?)),
				
				ProcessingInstruction { name, data } => children.push(XmlNode::processing_instruction(name, data)),
				
				Characters(text) => children.push(XmlNode::Text(text)),
				
				EndElement { name: end_element_name } => if common.does_not_have_name(&end_element_name)
				{
					return Err(ParseError::MismatchedEndElementName(event_reader.position(), common, end_element_name))
				}
				else
				{
					break
				},
				
				EndDocument => return Err(ParseError::EndDocumentBeforeEndElement(event_reader.position())),
				
				StartDocument { .. } => unreachable_start_document(),
				
				CData(_) | Comment(_) | Whitespace(_) => unreachable_cdata_comments_or_whitespace(),
			}
		}
		
		Ok
		(
			Self
			{
				common,
				
				children,
			}
		)
	}
}
