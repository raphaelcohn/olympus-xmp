// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A host.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Host<'a>
{
	#[allow(missing_docs)]
	Name(HostName<'a>),
	
	#[allow(missing_docs)]
	InternetProtocolVersion4Address(Ipv4Addr),
	
	#[allow(missing_docs)]
	InternetProtocolVersion6Address(Ipv6Addr),
}

impl<'a> Display for Host<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		use Host::*;
		
		match self
		{
			Name(host_name) => write!(f, "{}", host_name),
			
			InternetProtocolVersion4Address(internet_protocol_version_4_address) => write!(f, "{}", internet_protocol_version_4_address),
			
			InternetProtocolVersion6Address(internet_protocol_version_6_address) => write!(f, "[{}]", internet_protocol_version_6_address),
		}
	}
}

impl<'a> const FromUnchecked<Cow<'a, str>> for Host<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host_name: Cow<'a, str>) -> Self
	{
		Self::from(HostName::from_unchecked(host_name))
	}
}

impl<'a> const FromUnchecked<&'a str> for Host<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host_name: &'a str) -> Self
	{
		Self::from(HostName::from_unchecked(host_name))
	}
}

impl<'a> const FromUnchecked<String> for Host<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host_name: String) -> Self
	{
		Self::from(HostName::from_unchecked(host_name))
	}
}

impl<'a> const FromUnchecked<&'a [u8]> for Host<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host_name: &'a [u8]) -> Self
	{
		Self::from(HostName::from_unchecked(host_name))
	}
}

impl<'a, const Count: usize> const FromUnchecked<&'a [u8; Count]> for Host<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host_name: &'a [u8; Count]) -> Self
	{
		Self::from(HostName::from_unchecked(host_name))
	}
}

impl<'a> const From<HostName<'a>> for Host<'a>
{
	#[inline(always)]
	fn from(host_name: HostName<'a>) -> Self
	{
		Host::Name(host_name)
	}
}

impl<'a> const From<Ipv4Addr> for Host<'a>
{
	#[inline(always)]
	fn from(internet_protocol_version_4_address: Ipv4Addr) -> Self
	{
		Host::InternetProtocolVersion4Address(internet_protocol_version_4_address)
	}
}

impl<'a> const From<Ipv6Addr> for Host<'a>
{
	#[inline(always)]
	fn from(internet_protocol_version_6_address: Ipv6Addr) -> Self
	{
		Host::InternetProtocolVersion6Address(internet_protocol_version_6_address)
	}
}

impl<'a> TryToOwnInPlace for Host<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		if let Host::Name(host_name) = self
		{
			host_name.try_to_own_in_place()
		}
		else
		{
			Ok(())
		}
	}
}

impl<'a> TryToOwn for Host<'a>
{
	type TryToOwned = Host<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> Host<'a>
{
	/// `ihost          = IP-literal / IPv4address / ireg-name`
	/// `ireg-name      = *( iunreserved / pct-encoded / sub-delims )`
	/// `IP-literal     = "[" ( IPv6address / IPvFuture  ) "]"`
	/// `IPv4address    = dec-octet "." dec-octet "." dec-octet "." dec-octet`
	/// `IPv6address    = 6( h16 ":" ) ls32 / "::" 5( h16 ":" ) ls32 / [ h16 ] "::" 4( h16 ":" ) ls32 / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32 / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32 / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32 / [ *4( h16 ":" ) h16 ] "::" ls32 / [ *5( h16 ":" ) h16 ] "::" h16 / [ *6( h16 ":" ) h16 ] "::"`
	/// `IPvFuture      = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )`.
	/// `h16            = 1*4HEXDIG`
	/// `ls32           = ( h16 ":" h16 ) / IPv4address`
	/// `dec-octet      = DIGIT  / %x31-39 DIGIT / "1" 2DIGIT / "2" %x30-34 DIGIT / "25" %x30-35` (ie `0-9 | 10-99 | 100-199 | 200-249 | 250-255`).
	/// `iunreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~" / ucschar`.
	#[inline(always)]
	fn parse(scheme_specific_parsing_rule: &SchemeSpecificParsingRule, ihost_and_port_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), HostParseError>
	{
		use HostParseError::*;
		
		let length = ihost_and_port_bytes.len();
		
		if length == 0
		{
			#[inline(always)]
			const fn empty(host_name: HostName<'static>) -> Result<(Host<'static>, &'static [u8]), HostParseError>
			{
				Ok((Host::Name(host_name), b""))
			}
			
			use EmptyHostNameRule::*;
			return match scheme_specific_parsing_rule.authority_rule.empty_host_name_rule
			{
				ConvertsToLocalhost => empty(HostName::localhost),
				
				Unknown => empty(HostName::Empty),
				
				Denied => Err(AnEmptyHostNameIsNotPermittedForThisScheme),
			}
		}
		
		if ihost_and_port_bytes.get_unchecked_value_safe(0) == OpenSquareBracket
		{
			return Self::parse_internet_protocol_literal(ihost_and_port_bytes)
		}
		
		// Either a name or an IPv4 address.
		if scheme_specific_parsing_rule.authority_host_name_must_be_suitable_for_domain_name_system()
		{
			return match ihost_and_port_bytes.get_unchecked_value_safe(0)
			{
				_0 ..= _9 =>
				{
					if InternetProtocolVersion4AddressParser::could_be_an_internet_protocol_version_4_address(length)
					{
						let (possible_octet0_byte1, is_period) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_1(ihost_and_port_bytes);
						if is_period
						{
							Self::construct_internet_protocol_version_4_address(InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_0_to_9(ihost_and_port_bytes))
						}
						else
						{
							let (possible_octet0_byte2, is_period) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_2(ihost_and_port_bytes);
							if is_period
							{
								Self::construct_internet_protocol_version_4_address(InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_10_to_99(ihost_and_port_bytes, possible_octet0_byte1))
							}
							else if InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_3(ihost_and_port_bytes)
							{
								Self::construct_internet_protocol_version_4_address(InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_100_to_255(ihost_and_port_bytes, possible_octet0_byte1, possible_octet0_byte2))
							}
							else
							{
								Err(StartsWithADigitButIsNotAnInternetProtocolVersion4Address)
							}
						}
					}
					else
					{
						Err(StartsWithADigitButIsNotAnInternetProtocolVersion4Address)
					}
				}
				
				_ => Self::parse_name_lower_case(ihost_and_port_bytes),
			}
		}
		
		// Either a name or an IPv4 address.
		// How do we tell the difference? Check for the presence of '.' in the first 4 bytes, try to parse as an IPv4 address then fallback to host name parsing.
		// NOTE: host names do not have to be legal domain names (DNS ANAME or CNAME), otherwise we could simply check if the first byte is a digit!
		if !InternetProtocolVersion4AddressParser::could_be_an_internet_protocol_version_4_address(length)
		{
			let (possible_octet0_byte1, is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_1(ihost_and_port_bytes);
			if is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name
			{
				return Self::construct_internet_protocol_version_4_address_or_fallback_to_parse_name(ihost_and_port_bytes, InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_0_to_9(ihost_and_port_bytes))
			}
			
			let (possible_octet0_byte2, is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_2(ihost_and_port_bytes);
			if is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name
			{
				return Self::construct_internet_protocol_version_4_address_or_fallback_to_parse_name(ihost_and_port_bytes, InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_10_to_99(ihost_and_port_bytes, possible_octet0_byte1))
			}
			
			if InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_3(ihost_and_port_bytes)
			{
				return Self::construct_internet_protocol_version_4_address_or_fallback_to_parse_name(ihost_and_port_bytes, InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_100_to_255(ihost_and_port_bytes, possible_octet0_byte1, possible_octet0_byte2))
			}
		}
		
		Self::parse_name(ihost_and_port_bytes)
	}
	
	#[inline(always)]
	fn parse_name_lower_case(ihost_and_port_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), HostParseError>
	{
		let (host_name, port_bytes_including_colon) = HostName::parse_name_lower_case(ihost_and_port_bytes)?;
		Ok((Host::Name(host_name), port_bytes_including_colon))
	}
	
	#[inline(always)]
	fn parse_name(ihost_and_port_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), HostParseError>
	{
		let (host_name, port_bytes_including_colon) = HostName::parse(ihost_and_port_bytes)?;
		Ok((Host::Name(host_name), port_bytes_including_colon))
	}
	
	#[inline(always)]
	fn parse_internet_protocol_literal(ihost_and_port_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), HostParseError>
	{
		const MinimumIPLiteralLength: usize = 2;
		
		let length = ihost_and_port_bytes.len();
		if length < MinimumIPLiteralLength
		{
			return Err(HostParseError::IpLiteralIsNotClosedBySquareBracket)
		}
		
		let second = ihost_and_port_bytes.get_unchecked_value_safe(1);
		if second == v
		{
			const AdditionalOffsetDueToFutureLiteralIpLiteralV: usize = 1;
			if length < MinimumIPLiteralLength + AdditionalOffsetDueToFutureLiteralIpLiteralV
			{
				return Err(HostParseError::IpLiteralIsNotClosedBySquareBracket)
			}
			let (ihost_bytes, port_bytes_including_colon) = Self::parse_end_of_ip_literal::<AdditionalOffsetDueToFutureLiteralIpLiteralV>(ihost_and_port_bytes)?;
			Self::parse_future_internet_protocol_address(ihost_bytes, port_bytes_including_colon)
		}
		else
		{
			let (ihost_bytes, port_bytes_including_colon) = Self::parse_end_of_ip_literal::<0>(ihost_and_port_bytes)?;
			Self::parse_internet_protocol_version_6_address(ihost_bytes, port_bytes_including_colon)
		}
	}
	
	// Find the trailing ']:'.
	// Limits the scan to the last 6 bytes for efficiency.
	#[inline(always)]
	fn parse_end_of_ip_literal<const additional_offset_due_to_future_ip_literal_v: usize>(ihost_and_port_bytes: &'a [u8]) -> Result<(&'a [u8], &'a [u8]), HostParseError>
	{
		debug_assert!(additional_offset_due_to_future_ip_literal_v <= 1);
		
		let length = ihost_and_port_bytes.len();
		
		let slice_length = min(length, Authority::MaximumPortLengthInBytesWithColon);
		let from_index = length - slice_length;
		let slice = ihost_and_port_bytes.get_unchecked_range_safe(from_index .. );
		
		match memrchr(CloseSquareBracket, slice)
		{
			None => return Err(HostParseError::IpLiteralIsNotClosedBySquareBracket),
			
			Some(relative_index) =>
			{
				let index = from_index + relative_index;
				let ihost_bytes = ihost_and_port_bytes.get_unchecked_range_safe((1 + additional_offset_due_to_future_ip_literal_v)..index);
				let port_bytes_including_colon = ihost_and_port_bytes.after_index(index);
				Ok((ihost_bytes, port_bytes_including_colon))
			}
		}
	}
	
	#[inline(always)]
	fn parse_internet_protocol_version_6_address(ihost_bytes: &'a [u8], port_bytes_including_colon: &'a [u8]) -> Result<(Self, &'a [u8]), HostParseError>
	{
		const MaximumInternetProtocolVersion6AddressLength: usize = "[0000:0000:0000:0000:0000:ffff:255.255.255.255]".len();
		const MaximumLength: usize = MaximumInternetProtocolVersion6AddressLength;
		
		use HostParseError::*;
		
		let length = ihost_bytes.len();
		if length > MaximumLength
		{
			return Err(HostParseError::InternetProtocolVersion6AddressIsTooLong)
		}
		
		match Ipv6Addr::from_str(unsafe { from_utf8_unchecked(ihost_bytes) })
		{
			Ok(address) => Ok((Host::InternetProtocolVersion6Address(address), port_bytes_including_colon)),
			
			Err(_) => Err(CouldNotParseInternetProtocolVersion6Address),
		}
	}
	
	#[inline(always)]
	fn parse_future_internet_protocol_address(_ihost_bytes: &'a [u8], _port_bytes_including_colon: &'a [u8]) -> Result<(Self, &'a [u8]), HostParseError>
	{
		return Err(HostParseError::FutureInternetProtocolAddressParsingIsUnsupported)
	}
	
	#[inline(always)]
	fn construct_internet_protocol_version_4_address_or_fallback_to_parse_name(ihost_and_port_bytes: &'a [u8], result: Result<(Ipv4Addr, &'a [u8]), InternetProtocolVersion4AddressParseError>) -> Result<(Self, &'a [u8]), HostParseError>
	{
		match result
		{
			Ok(inner) => Self::construct_internet_protocol_version_4_address_common(inner),
			
			Err(_) => Self::parse_name(ihost_and_port_bytes)
		}
	}
	
	#[inline(always)]
	fn construct_internet_protocol_version_4_address(result: Result<(Ipv4Addr, &'a [u8]), InternetProtocolVersion4AddressParseError>) -> Result<(Self, &'a [u8]), HostParseError>
	{
		match result
		{
			Ok(inner) => Self::construct_internet_protocol_version_4_address_common(inner),
			
			Err(error) => Err(HostParseError::InternetProtocolVersion4AddressParse(error))
		}
	}
	
	#[inline(always)]
	fn construct_internet_protocol_version_4_address_common((internet_protocol_version_4_address, port_bytes): (Ipv4Addr, &'a [u8])) -> Result<(Self, &'a [u8]), HostParseError>
	{
		Ok((Host::InternetProtocolVersion4Address(internet_protocol_version_4_address), port_bytes))
	}
}
