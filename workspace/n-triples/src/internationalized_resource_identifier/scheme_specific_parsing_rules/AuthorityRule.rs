// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct AuthorityRule
{
	user_information_allowed: bool,
	
	pub(super) empty_host_name_rule: EmptyHostNameRule,
	
	pub(super) port_rule: PortParsingRule,
}

impl AuthorityRule
{
	const Denied: Self = Self::new(false, EmptyHostNameRule::Denied, PortParsingRule::Denied);
	
	#[inline(always)]
	const fn new(user_information_allowed: bool, empty_host_name_rule: EmptyHostNameRule, port_rule: PortParsingRule) -> Self
	{
		Self
		{
			user_information_allowed,
			empty_host_name_rule,
			port_rule
		}
	}
}
