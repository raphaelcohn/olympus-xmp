// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XML name.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct XmlName<'namespace, 'local_name>
{
	/// A namespace URI, e.g. `http://www.w3.org/2000/xmlns/`.
	pub namespace_uniform_resource_identifier: Option<Cow<'namespace, str>>,
	
	/// A local name, e.g. `string` in `xsi:string`.
	pub local_name: Cow<'local_name, str>,
}

impl<'local_name> const From<&'local_name str> for XmlName<'static, 'local_name>
{
	#[inline(always)]
	fn from(local_name: &'local_name str) -> Self
	{
		Self
		{
			namespace_uniform_resource_identifier: None,
		
			local_name: Borrowed(local_name),
		}
	}
}

impl<'namespace, 'local_name> Display for XmlName<'namespace, 'local_name>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		match self.namespace_uniform_resource_identifier
		{
			None => write!(f, "{}", self.local_name.as_ref()),
			
			Some(ref namespace_uniform_resource_identifier) => write!(f, "{}:{}", namespace_uniform_resource_identifier.as_ref(), self.local_name.as_ref())
		}
	}
}

impl<'namespace, 'local_name> XmlName<'namespace, 'local_name>
{
	/// Construct a namespaced instance.
	#[inline(always)]
	pub const fn namespaced(namespace_uniform_resource_identifier: &'namespace str, local_name: &'local_name str) -> Self
	{
		Self
		{
			namespace_uniform_resource_identifier: Some(Borrowed(namespace_uniform_resource_identifier)),
			
			local_name: Borrowed(local_name),
		}
	}
	
	#[inline(always)]
	fn has_namespace(&self, namespace_uniform_resource_identifier: Option<&str>) -> bool
	{
		self.namespace_uniform_resource_identifier.as_ref().map(|x| x.deref()) == namespace_uniform_resource_identifier
	}
	
	#[inline(always)]
	fn matches_owned_name(&self, name: &OwnedName) -> bool
	{
		self.namespace_uniform_resource_identifier.as_ref().map(|x| x.deref()) == name.namespace.as_ref().map(|x| x.as_str()) && self.local_name.deref() == name.local_name.as_str()
	}
	
	#[inline(always)]
	fn into_unowned<'prefix, 'name, 'b>(&'b self, prefix: Option<&'prefix str>) -> Name<'name>
	where 'namespace: 'name, 'local_name: 'name, 'prefix: 'name, 'b: 'namespace, 'b: 'local_name
	{
		Name
		{
			local_name: self.local_name.deref(),
			namespace: self.namespace_uniform_resource_identifier.as_ref().map(|v| v.deref()),
			prefix
		}
	}
}
