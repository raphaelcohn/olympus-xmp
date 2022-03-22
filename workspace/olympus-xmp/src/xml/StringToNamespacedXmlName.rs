// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Converts a String to a namespaced XmlName.
pub trait StringToNamespacedXmlName<'namespace>
{
	/// Converts a String to a namespaced XmlName.
	fn with_local_name<'local_name>(&'namespace self, local_name: &'local_name str) -> XmlName<'namespace, 'local_name>;
}

impl<'namespace> const StringToNamespacedXmlName<'namespace> for &'namespace str
{
	#[inline(always)]
	fn with_local_name<'local_name>(&'namespace self, local_name: &'local_name str) -> XmlName<'namespace, 'local_name>
	{
		XmlName::namespaced(self, local_name)
	}
}
