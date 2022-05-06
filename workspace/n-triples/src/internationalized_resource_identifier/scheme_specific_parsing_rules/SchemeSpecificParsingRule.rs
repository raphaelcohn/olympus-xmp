// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct SchemeSpecificParsingRule
{
	hierachy_variant_rule: HierarchyVariantRule,
	
	query_allowed: bool,
	
	hash_fragment_allowed: bool,
	
	authority_rule: AuthorityRule,
}

impl SchemeSpecificParsingRule
{
	const HttpDefaultPort: u16 = 80;
	
	const HttpsDefaultPort: u16 = 443;
	
	pub(super) const coap: Self = Self::web_socket_like(5683);
	
	pub(super) const coaps: Self = Self::web_socket_like(5684);
	
	pub(super) const dns: Self = Self::web_socket_like(53);
	
	pub(super) const git: Self = Self::web_socket_like(443);
	
	pub(super) const file: Self = Self::new(HierarchyVariantRule::AuthorityAndAbsolutePathOrAbsolutePathOnly, false, false, AuthorityRule::new(true, EmptyHostNameRule::ConvertsToLocalhost, PortParsingRule::Denied));
	
	pub(super) const ftp: Self = Self::new(HierarchyVariantRule::AuthorityAndAbsolutePathOnly, false, false, AuthorityRule::new(true, EmptyHostNameRule::Denied, PortParsingRule::allowed(21)));
	
	pub(super) const http: Self = Self::http_like(Self::HttpDefaultPort);
	
	pub(super) const https: Self = Self::http_like(Self::HttpsDefaultPort);
	
	// TODO: This variant can never have a host or a port, so the authority_rule is irrelevant.
	pub(super) const mailto: Self = Self::new(HierarchyVariantRule::RootlessOnly, true, false, AuthorityRule::Denied);
	
	pub(super) const ws: Self = Self::web_socket_like(Self::HttpDefaultPort);
	
	pub(super) const wss: Self = Self::web_socket_like(Self::HttpsDefaultPort);
	
	pub(super) const Unknown: Self = Self::new(HierarchyVariantRule::Unknown, true, true, AuthorityRule::new(true, EmptyHostNameRule::Unknown, PortParsingRule::Unknown));
	
	#[inline(always)]
	const fn http_like(default_port: u16) -> Self
	{
		Self::http_or_web_socket_like(default_port, true, true)
	}
	
	#[inline(always)]
	const fn web_socket_like(default_port: u16) -> Self
	{
		Self::http_or_web_socket_like(default_port, false, false)
	}
	
	#[inline(always)]
	const fn http_or_web_socket_like(default_port: u16, hash_fragment_allowed: bool, user_information_allowed: bool) -> Self
	{
		Self::new(HierarchyVariantRule::AuthorityAndAbsolutePathOnly, true, hash_fragment_allowed, AuthorityRule::new(user_information_allowed, EmptyHostNameRule::Denied, PortParsingRule::allowed(default_port)))
	}
	
	#[inline(always)]
	const fn new(hierachy_variant_rule: HierarchyVariantRule, query_allowed: bool, hash_fragment_allowed: bool, authority_rule: AuthorityRule) -> Self
	{
		Self
		{
			hierachy_variant_rule,
			authority_rule,
			query_allowed,
			hash_fragment_allowed,
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
	pub(super) const fn query_should_not_be_present(&self, remaining_string: &str) -> bool
	{
		!self.query_allowed && !remaining_string.is_empty()
	}
	
	#[inline(always)]
	pub(super) const fn user_information_should_not_be_present(&self) -> bool
	{
		!self.authority_rule.user_information_allowed
	}
	
	#[inline(always)]
	pub(super) const fn hash_fragment_should_not_be_present(&self, remaining_string: &str) -> bool
	{
		!self.hash_fragment_allowed && !remaining_string.is_empty()
	}
	
	#[inline(always)]
	pub(super) const fn port_rule(&self) -> PortParsingRule
	{
		self.authority_rule.port_rule
	}
	
	#[inline(always)]
	pub(super) const fn empty_host_name_rule(&self) -> EmptyHostNameRule
	{
		self.authority_rule.empty_host_name_rule
	}
}
