// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Variant.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Variant
{
	BigEndian0,
	
	BigEndian1,
	
	MixedEndian,
}

impl Variant
{
	#[inline(always)]
	fn parse(variant: u8) -> Result<Self, VariantParseError>
	{
		use VariantParseError::*;
		
		debug_assert!(variant < 8);
		
		use Variant::*;
		Ok
		(
			match variant
			{
				0b000 | 0b001 | 0b010 | 0b011 => return Err(ReservedApolloNetworkComputingSystem),
				
				0b100 => BigEndian0,
				
				0b101 => BigEndian1,
				
				0b110 => MixedEndian,
				
				0b111 => return Err(Reserved),
				
				_ => unreachable_code_const("3-bit variant"),
			}
		)
	}
	
	#[inline(always)]
	fn u16_from_bytes(self, bytes: [u8; 2]) -> u16
	{
		use Variant::*;
		match self
		{
			BigEndian0 | BigEndian1=> u16::from_be_bytes(bytes),
			
			MixedEndian => u16::from_le_bytes(bytes),
		}
	}
	
	#[inline(always)]
	fn u32_from_bytes(self, bytes: [u8; 4]) -> u32
	{
		use Variant::*;
		match self
		{
			BigEndian0 | BigEndian1=> u32::from_be_bytes(bytes),
			
			MixedEndian => u32::from_le_bytes(bytes),
		}
	}
}
