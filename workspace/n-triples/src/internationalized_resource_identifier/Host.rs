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
	fn parse(ihost_and_port_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), HostParseError>
	{
		let length = ihost_and_port_bytes.len();
		
		if length == 0
		{
			return Ok((Host::Name(HostName::Empty), b""))
		}
		
		if ihost_and_port_bytes.get_unchecked_value_safe(0) == OpenSquareBracket
		{
			return Self::parse_internet_protocol_literal(ihost_and_port_bytes)
		}
		
		// Either a name or an IPv4 address.
		// How do we tell the difference? Check for the presence of '.' in the first 4 bytes.
		if InternetProtocolVersion4AddressParser::could_be_an_internet_protocol_version_4_address(length)
		{
			let (possible_octet0_byte1, is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_1(ihost_and_port_bytes);
			if is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name
			{
				return Self::construct_internet_protocol_version_4_address(InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_0_to_9(ihost_and_port_bytes))
			}
			
			let (possible_octet0_byte2, is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name) = InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_2(ihost_and_port_bytes);
			if is_period_so_is_either_an_internet_protocol_version_4_address_or_invalid_ireg_name
			{
				return Self::construct_internet_protocol_version_4_address(InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_10_to_99(ihost_and_port_bytes, possible_octet0_byte1))
			}
			
			if InternetProtocolVersion4AddressParser::get_byte_and_check_if_it_is_period_index_3(ihost_and_port_bytes)
			{
				return Self::construct_internet_protocol_version_4_address(InternetProtocolVersion4AddressParser::parse_knowing_octet_1_is_100_to_255(ihost_and_port_bytes, possible_octet0_byte1, possible_octet0_byte2))
			}
		}
		
		Self::parse_name(ihost_and_port_bytes)
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
	fn construct_internet_protocol_version_4_address(result: Result<(Ipv4Addr, &'a [u8]), InternetProtocolVersion4AddressParseError>) -> Result<(Self, &'a [u8]), HostParseError>
	{
		match result
		{
			Ok((internet_protocol_version_4_address, port_bytes)) => Ok((Host::InternetProtocolVersion4Address(internet_protocol_version_4_address), port_bytes)),
			
			Err(error) => Err(HostParseError::InternetProtocolVersion4AddressParse(error))
		}
	}
}
