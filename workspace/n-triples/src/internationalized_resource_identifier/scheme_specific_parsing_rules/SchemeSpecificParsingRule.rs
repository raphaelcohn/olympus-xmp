// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct SchemeSpecificParsingRule
{
	hierachy_variant_rule: HierarchyVariantRule,
	
	query_allowed: bool,
	
	hash_fragment_allowed: bool,
	
	pub(super) empty_host_name_rule: EmptyHostNameRule,
	
	pub(super) port_rule: PortParsingRule,
}

impl SchemeSpecificParsingRule
{
	const HttpDefaultPort: u16 = 80;
	
	const HttpsDefaultPort: u16 = 443;
	
	pub(super) const file: Self = Self::may_have_authority(HierarchyVariantRule::AuthorityAndAbsolutePathOrAbsolutePathOnly, false, false, EmptyHostNameRule::ConvertsToLocalhost, PortParsingRule::Denied);
	
	pub(super) const ftp: Self = Self::authority_and_absolute_path_only(21, false, false);
	
	pub(super) const http: Self = Self::http_like(Self::HttpDefaultPort);
	
	pub(super) const https: Self = Self::http_like(Self::HttpsDefaultPort);
	
	pub(super) const ws: Self = Self::web_socket_like(Self::HttpDefaultPort);
	
	pub(super) const wss: Self = Self::web_socket_like(Self::HttpsDefaultPort);
	
	pub(super) const Unknown: Self = Self::new(HierarchyVariantRule::Unknown, true, true, EmptyHostNameRule::Unknown, PortParsingRule::Unknown);
	
	#[inline(always)]
	const fn http_like(default_port: u16) -> Self
	{
		Self::authority_and_absolute_path_only(default_port, true, true)
	}
	
	#[inline(always)]
	const fn web_socket_like(default_port: u16) -> Self
	{
		Self::authority_and_absolute_path_only(default_port, true, false)
	}
	
	#[inline(always)]
	const fn authority_and_absolute_path_only(default_port: u16, query_allowed: bool, hash_fragment_allowed: bool) -> Self
	{
		Self::may_have_authority(HierarchyVariantRule::AuthorityAndAbsolutePathOnly, query_allowed, hash_fragment_allowed, EmptyHostNameRule::Denied, PortParsingRule::Allowed { default_port: new_non_zero_u16(default_port) })
	}
	
	#[inline(always)]
	const fn may_have_authority(hierachy_variant_rule: HierarchyVariantRule, query_allowed: bool, hash_fragment_allowed: bool, empty_host_name_rule: EmptyHostNameRule, port_rule: PortParsingRule) -> Self
	{
		Self::new(hierachy_variant_rule, query_allowed, hash_fragment_allowed, empty_host_name_rule, port_rule)
	}
	
	#[inline(always)]
	const fn new(hierachy_variant_rule: HierarchyVariantRule, query_allowed: bool, hash_fragment_allowed: bool, empty_host_name_rule: EmptyHostNameRule, port_rule: PortParsingRule) -> Self
	{
		Self
		{
			hierachy_variant_rule,
			query_allowed,
			hash_fragment_allowed,
			empty_host_name_rule,
			port_rule
		}
	}
	
	#[inline(always)]
	pub(super) const fn hierarchy_starts_with_slash(&self) -> bool
	{
		self.hierachy_variant_rule.hierarchy_starts_with_slash()
	}
	
	#[inline(always)]
	pub(super) const fn hierarchy_is_authority_and_absolute_path(&self) -> bool
	{
		self.hierachy_variant_rule.hierarchy_is_authority_and_absolute_path()
	}
	
	#[inline(always)]
	pub(super) const fn authority_host_name_must_be_suitable_for_domain_name_system(&self) -> bool
	{
		self.hierarchy_starts_with_slash()
	}
	
	#[inline(always)]
	pub(super) const fn query_should_not_be_present(&self, remaining_utf8_bytes: &[u8]) -> bool
	{
		!self.query_allowed && !remaining_utf8_bytes.is_empty()
	}
	
	#[inline(always)]
	pub(super) const fn hash_fragment_should_not_be_present(&self, remaining_utf8_bytes: &[u8]) -> bool
	{
		!self.hash_fragment_allowed && !remaining_utf8_bytes.is_empty()
	}
}
