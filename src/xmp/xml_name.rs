// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Wrapper to produce a static reference to a XmlName.
#[macro_export]
macro_rules! xml_name
{
	($namespace: ident, $local_name: expr) =>
	{
		{
			use $crate::xml::XmlName;
			static Name: XmlName<'static, 'static> = XmlName::namespaced($namespace, $local_name);
			&Name
		}
	}
}
