// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An ISO 3166-1 numeric country code.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum Iso3166Dash1NumericCountryCode
{
	/// Afghanistan.
	_004 = 4,
	
	/// British Indian Ocean Territory.
	_086 = 86,
	
	/// Exceptional reservation.
	///
	/// France, Metropolitan.
	/// Deleted.
	_249 = 249,
	
	/// France.
	_250 = 250,
	
	/// Saint Helena, Ascension and Tristan da Cunha.
	_654 = 654,
	
	/// Norfolk Island.
	_574 = 574,
	
	/// Spain.
	_724 = 724,
	
	/// USSR, Union of Soviet Socialist Republics.
	_810 = 810,
	
	/// United Kingdom of Great Britain and Northern Ireland (the).
	_826 = 826,
	
	/// Guernsey.
	_831 = 831,
	
	/// Jersey.
	_832 = 832,
	
	/// Isle of Man.
	_833 = 833,
}
