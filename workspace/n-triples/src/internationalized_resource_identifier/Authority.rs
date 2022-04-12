// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An authority.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Authority<'a>
{
	/// User information.
	pub user_information: Option<UserInformation<'a>>,
	
	/// Host.
	pub host: Host<'a>,
	
	/// Port.
	pub port: Option<NonZeroU16>,
}

impl<'a> const From<Host<'a>> for Authority<'a>
{
	#[inline(always)]
	fn from(host: Host<'a>) -> Self
	{
		Authority
		{
			user_information: None,
			host,
			port: None
		}
	}
}

impl<'a> const From<HostName<'a>> for Authority<'a>
{
	#[inline(always)]
	fn from(host_name: HostName<'a>) -> Self
	{
		Self::from(Host::Name(host_name))
	}
}

impl<'a> Authority<'a>
{
	/// `iauthority = [ iuserinfo "@" ] ihost [ ":" port ]`.
	#[inline(always)]
	fn parse(authority_bytes: &'a [u8]) -> Result<Self, AuthorityParseError>
	{
		// Frustrating, as requires a long scan (`memchr`) which will nearly always return `None` as user information is very rare.
		// The alternative is to assume there is no user_host data, then throwaway what we've pased so far if an `@` is encountered.
		let (user_information, ihost_and_port_bytes) = match memchr(AtSign, authority_bytes)
		{
			None => (None, authority_bytes),
			
			Some(index) =>
			{
				let user_info_bytes = authority_bytes.before_index(index);
				(Some(UserInformation::parse(user_info_bytes)?), authority_bytes.after_index(index))
			}
		};
		
		let (host, port_bytes_including_colon) = Host::parse(ihost_and_port_bytes)?;
		
		let port = Self::parse_port(port_bytes_including_colon)?;
		
		Ok
		(
			Self
			{
				user_information,
			
				host,
			
				port,
			}
		)
	}
	
	const MaximumPortLengthInBytesWithoutColon: usize = 5;
	
	const MaximumPortLengthInBytesWithColon: usize = 1 + Self::MaximumPortLengthInBytesWithoutColon;
	
	/// `port = *DIGIT`.
	#[inline(always)]
	fn parse_port(port_bytes_including_colon: &[u8]) -> Result<Option<NonZeroU16>, PortParseError>
	{
		use PortParseError::*;
	
		#[inline(always)]
		fn parse_port_digit<const index: u32, const count: u32>(port_bytes_including_colon: &[u8]) -> Result<u32, PortParseError>
		{
			debug_assert_ne!(count, 0);
			debug_assert!(count <= (Self::MaximumPortLengthInBytesWithoutColon as u32));
			debug_assert!(index < count);
			
			let byte = port_bytes.get_unchecked_value_safe(index + 1);
			let value = match byte
			{
				_0 .. _9 => (byte - _0) as u32,
				
				_ => return Err(InvalidCharacterIsNotADigit(byte)),
			};
			
			let scalar = ((count - 1) - index) * 10;
			Ok(value * scalar)
		}
		
		#[inline(always)]
		fn guard_starts_with_colon(port_bytes_including_colon: &[u8]) -> Result<(), PortParseError>
		{
			let first = port_bytes_including_colon.get_unchecked_value_safe(0);
			if first == Colon
			{
				Ok(())
			}
			else
			{
				Err(InvalidCharacterIsNotAColon(first))
			}
		}
		
		let raw_value = match port_bytes_including_colon.len()
		{
			0 => return Ok(None),
			
			1 =>
			{
				guard_starts_with_colon(port_bytes_including_colon)?;
				return Ok(None)
			}
			
			2 =>
			{
				guard_starts_with_colon(port_bytes_including_colon)?;
				parse_port_digit::<0, 1>(port_bytes_including_colon)?
			}
			
			3 =>
			{
				guard_starts_with_colon(port_bytes_including_colon)?;
				parse_port_digit::<0, 2>(port_bytes_including_colon)? | parse_port_digit::<1, 2>(port_bytes_including_colon)?
			}
			
			4 =>
			{
				guard_starts_with_colon(port_bytes_including_colon)?;
				parse_port_digit::<0, 3>(port_bytes_including_colon)? | parse_port_digit::<1, 3>(port_bytes_including_colon)? | parse_port_digit::<2, 3>(port_bytes_including_colon)?
			},
			
			5 =>
			{
				guard_starts_with_colon(port_bytes_including_colon)?;
				parse_port_digit::<0, 1>(port_bytes_including_colon)? | parse_port_digit::<1, 4>(port_bytes_including_colon)? | parse_port_digit::<2, 4>(port_bytes_including_colon)? | parse_port_digit::<3, 4>(port_bytes_including_colon)?
			},
			
			6 =>
			{
				guard_starts_with_colon(port_bytes_including_colon)?;
				parse_port_digit::<0, 5>(port_bytes_including_colon)? | parse_port_digit::<1, 5>(port_bytes_including_colon)? | parse_port_digit::<2, 5>(port_bytes_including_colon)? | parse_port_digit::<3, 5>(port_bytes_including_colon)? | parse_port_digit::<4, 5>(port_bytes_including_colon)?
			},
			
			_ => return Err(ValuesGreaterThan655535AreUnsupported),
		};
		if raw_value == 0
		{
			return Err(IsZero)
		}
		if raw_value > (u16::MAX as u32)
		{
			return Err(IsGreaterThan65535(new_non_zero_u32(raw_value)))
		}
		Ok(Some(new_non_zero_u16(raw_value as u16)))
	}
}

impl Authority<'static>
{
	/// `wwww.w3.org`.
	pub const www_w3_org: Self = Self::from(HostName::www_w3_org);
}
