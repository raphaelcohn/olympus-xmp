// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Trait common to XmlDocument and XmlElement
pub trait XmlDocumentOrXmlElement
{
	/// Get attribute.
	fn get_attribute<'a, 'namespace, 'local_name>(&'a self, attribute_name: &'a XmlName<'namespace, 'local_name>) -> Option<&'a str>
	where 'a: 'namespace, 'a: 'local_name;
	
	/// Get only element.
	fn get_only_element<'a>(&'a self, path: &[XmlName]) -> Result<&'a XmlElement, NotExactlyOneElementError>;
	
	/// Get elements.
	fn get_elements<'a>(&'a self, path: &[XmlName]) -> Vec<&'a XmlElement>;
	
	/// Has no attributes in namespace.
	fn has_no_attributes_in_namespace(&self, namespace_uniform_resource_identifier: Option<&str>) -> bool;
}
