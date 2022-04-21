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

impl<'a> Display for Authority<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		if let Some(ref user_information) = self.user_information
		{
			write!(f, "{}@", user_information)?
		}
		write!(f, "{}", self.host)?;
		if let Some(port) = self.port
		{
			write!(f, ":{}", port)
		}
		else
		{
			Ok(())
		}
	}
}

impl<'a> TryToOwnInPlace for Authority<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		self.user_information.try_to_own_in_place()?;
		self.host.try_to_own_in_place()
	}
}

impl<'a> TryToOwn for Authority<'a>
{
	type TryToOwned = Authority<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Self::TryToOwned, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl<'a> const FromUnchecked<&'a str> for Authority<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host_name: &'a str) -> Self
	{
		Self::new_for_host(Host::from_unchecked(host_name))
	}
}

impl<'a> const FromUnchecked<HostName<'a>> for Authority<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host_name: HostName<'a>) -> Self
	{
		Self::new_for_host_name(host_name)
	}
}

impl<'a> const From<HostName<'a>> for Authority<'a>
{
	#[inline(always)]
	fn from(host_name: HostName<'a>) -> Self
	{
		Self::new_for_host_name(host_name)
	}
}

impl<'a> const FromUnchecked<Host<'a>> for Authority<'a>
{
	#[inline(always)]
	unsafe fn from_unchecked(host: Host<'a>) -> Self
	{
		Self::new_for_host(host)
	}
}

impl<'a> const From<Host<'a>> for Authority<'a>
{
	#[inline(always)]
	fn from(host: Host<'a>) -> Self
	{
		Self::new_for_host(host)
	}
}

impl<'a> Authority<'a>
{
	/// New instance.
	#[inline(always)]
	pub const fn new_for_host_name(host_name: HostName<'a>) -> Self
	{
		Self::new_for_host(Host::from(host_name))
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new_for_host_name_and_port(host_name: HostName<'a>, port: NonZeroU16) -> Self
	{
		Self::new_for_host_and_port(Host::from(host_name), port)
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new_for_host(host: Host<'a>) -> Self
	{
		Self::new(None, host, None)
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new_for_host_and_port(host: Host<'a>, port: NonZeroU16) -> Self
	{
		Self::new(None, host, Some(port))
	}
	
	/// New instance.
	#[inline(always)]
	pub const fn new(user_information: Option<UserInformation<'a>>, host: Host<'a>, port: Option<NonZeroU16>) -> Self
	{
		Self
		{
			user_information,
			host,
			port
		}
	}
	
	/// New hierarchy.
	#[inline(always)]
	pub const fn with<const PathDepth: usize>(self, path_segments: PathSegments<'a, PathDepth>) -> Hierarchy<'a, PathDepth>
	{
		Hierarchy::AuthorityAndAbsolutePath
		{
			authority: self,
			absolute_path: path_segments,
		}
	}
	
	/// `iauthority = [ iuserinfo "@" ] ihost [ ":" port ]`.
	#[inline(always)]
	fn parse(has_authority_and_absolute_path_with_dns_host_name: bool, authority_bytes: &'a [u8]) -> Result<Self, AuthorityParseError>
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
		
		let (host, port_bytes_including_colon) = Host::parse(has_authority_and_absolute_path_with_dns_host_name, ihost_and_port_bytes)?;
		
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
			debug_assert!(count <= (Authority::MaximumPortLengthInBytesWithoutColon as u32));
			debug_assert!(index < count);
			
			let byte = port_bytes_including_colon.get_unchecked_value_safe(index + 1);
			let value = match byte
			{
				_0 ..= _9 => (byte - _0) as u32,
				
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
