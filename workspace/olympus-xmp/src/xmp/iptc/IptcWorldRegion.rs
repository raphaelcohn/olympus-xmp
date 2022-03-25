// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// IPTC world region.
///
/// One of the values listed in <https://cv.iptc.org/newscodes/worldregion/>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum IptcWorldRegion
{
	#[allow(missing_docs)]
	World,
	
	#[allow(missing_docs)]
	Africa,
	
	#[allow(missing_docs)]
	SouthAmerica,
	
	#[allow(missing_docs)]
	Oceania,
	
	#[allow(missing_docs)]
	NorthAmerica,
	
	#[allow(missing_docs)]
	Asia,
	
	#[allow(missing_docs)]
	Europe,
	
	#[allow(missing_docs)]
	Antarctica,
}

impl IptcWorldRegion
{
	#[inline(always)]
	fn for_country(country: Iso3166Dash1AlphaCountryCode, interpret_eurasian_as_european_not_asian: bool, interpret_user_assigned: bool, interpret_foreign_territories_of_european_countries_as_european: bool, interpret_world_intellectual_property_organisation: bool) -> Option<Self>
	{
		use IptcWorldRegion::*;
		use Iso3166Dash1AlphaCountryCode::*;
		use Iso3166Dash1Alpha2CountryCode::*;
		use Iso3166Dash1Alpha3CountryCode::*;
		
		const World: Option<IptcWorldRegion> = Some(IptcWorldRegion::World);
		const Africa: Option<IptcWorldRegion> = Some(IptcWorldRegion::Africa);
		const SouthAmerica: Option<IptcWorldRegion> = Some(IptcWorldRegion::SouthAmerica);
		const Oceania: Option<IptcWorldRegion> = Some(IptcWorldRegion::Oceania);
		const NorthAmerica: Option<IptcWorldRegion> = Some(IptcWorldRegion::NorthAmerica);
		const Asia: Option<IptcWorldRegion> = Some(IptcWorldRegion::Asia);
		const Europe: Option<IptcWorldRegion> = Some(IptcWorldRegion::Europe);
		const Antarctica: Option<IptcWorldRegion> = Some(IptcWorldRegion::Antarctica);
		
		const Carribbean: Option<IptcWorldRegion> = NorthAmerica;
		const UserAssigned: Option<IptcWorldRegion> = None;
		const EscapeCode: Option<IptcWorldRegion> = None;
		const Unknown: Option<IptcWorldRegion> = World;
		
		#[inline(always)]
		const fn Eurasia(interpret_eurasian_as_european_not_asian: bool) -> Option<IptcWorldRegion>
		{
			if interpret_eurasian_as_european_not_asian
			{
				Europe
			}
			else
			{
				Asia
			}
		}
		
		// `interpret_world_intellectual_property_organisation` is subsidary to `interpret_user_assigned`, ie if there are World Intellectual Property Organisation (WIPO) user assigned usages, these are only interpeted if `interpret_user_assigned` is `true`.
		#[inline(always)]
		const fn MaybeEuropean(foreign_territory_region: Option<IptcWorldRegion>, interpret_foreign_territories_of_european_countries_as_european: bool) -> Option<IptcWorldRegion>
		{
			if interpret_foreign_territories_of_european_countries_as_european
			{
				Europe
			}
			else
			{
				foreign_territory_region
			}
		}
		
		match country
		{
			Alpha2(alpha_2) => match alpha_2
			{
				AA => UserAssigned,
				
				AC => MaybeEuropean(SouthAmerica, interpret_foreign_territories_of_european_countries_as_european),
				
				AF => Asia,
				
				// TODO: Formally was code for French Afars and Issas before the code was deleted and reassigned to Anguilla.
				AI => Carribbean,
				
				AP if interpret_world_intellectual_property_organisation => Africa,
				
				// TODO: Formerly British Antartica.
				BQ => Carribbean,
				
				BX if interpret_world_intellectual_property_organisation => Europe,
				
				CP => MaybeEuropean(SouthAmerica, interpret_foreign_territories_of_european_countries_as_european)a,
				
				CQ => Europe,
				
				// Was Canton and Enderbury Islands.
				// Merged into Kiribati (`KI`).
				CS => DELETED,
				
				// Was East Germany.
				DD => DELETED,
				
				DG => MaybeEuropean(Asia, interpret_foreign_territories_of_european_countries_as_european),
				
				// Eurasian Patent Organization.
				EA if interpret_world_intellectual_property_organisation => Eurasia(interpret_eurasian_as_european_not_asian),
				
				EA => MaybeEuropean(Africa, interpret_foreign_territories_of_european_countries_as_european),
				
				EF if interpret_world_intellectual_property_organisation => Europe,
				
				EM if interpret_world_intellectual_property_organisation => Europe,
				
				EP if interpret_world_intellectual_property_organisation => Europe,
				
				ES => Europe,
				
				EU => Europe,
				
				// Eurasian Patent Organization.
				EV if interpret_world_intellectual_property_organisation=> Eurasia(interpret_eurasian_as_european_not_asian),
				
				EZ => Europe,
				
				FR => Europe,
				
				FX => Europe,
				
				GB => Europe,
				
				GC if interpret_world_intellectual_property_organisation => Asia,
				
				IC => MaybeEuropean(Africa, interpret_foreign_territories_of_european_countries_as_european),
				
				// International Bureau of WIPO.
				IG if interpret_world_intellectual_property_organisation => World,
				
				IM => Europe,
				
				IO => MaybeEuropean(Asia, interpret_foreign_territories_of_european_countries_as_european),
				
				OA if interpret_world_intellectual_property_organisation => Africa,
				
				OO => EscapeCode,
				
				// Used by the International Standard Recording Code (ISRC) as a second country code for the United States.
				QM if interpret_user_assigned => NorthAmerica,
				
				// Used by the Unicode Common Locale Data Repository (CLDR) to represent Outlying Oceania (a multi-territory region containing Antarctica, Bouvet Island, the Cocos (Keeling) Islands, Christmas Island, South Georgia and the South Sandwich Islands, Heard Island and McDonald Islands, the British Indian Ocean Territory, the French Southern Territories, and the United States Minor Outlying Islands).
				QO if interpret_user_assigned => Oceania,
				
				// Used by the Unicode Common Locale Data Repository (CLDR) historically before `EU` was assigned.
				QP if interpret_user_assigned => Europe,
				
				// Used by the World Intellectual Property Organization (WIPO) as an indicator for the Community Plant Variety Office, an EU institution.
				QZ if interpret_user_assigned && interpret_world_intellectual_property_organisation => Europe,
				
				QM | QN | QP | QQ | QR | QS | QT | QU | QV | QX | QY | QZ => UserAssigned,
				
				SH => MaybeEuropean(SouthAmerica, interpret_foreign_territories_of_european_countries_as_european),
				
				// Soviet Union.
				SU => Eurasia(interpret_eurasian_as_european_not_asian),
				
				TA => MaybeEuropean(SouthAmerica, interpret_foreign_territories_of_european_countries_as_european),
				
				UK => Europe,
				
				UN => World,
				
				// World Intellectual Property Organization (WIPO).
				WO if interpret_world_intellectual_property_organisation => World,
				
				// Used by Switzerland as a country code for the Canary Islands although `IC` is already reserved for that purpose.
				XA if interpret_user_assigned => MaybeEuropean(Africa, interpret_foreign_territories_of_european_countries_as_european),
				
				// Used by the UK Government as an EORI number country code prefix for Northern Ireland.
				// Use by the European Union for Value Added Tax (VAT) reports with trade with Northern Ireland.
				XI if interpret_user_assigned => Europe,
				
				// Used by the European Commission, IMF, SWIFT, the Unicode Common Locale Data Repository (CLDR) and other organizations as a temporary country code for Kosovo.
				XK if interpret_user_assigned => Europe,
				
				// Used by the World Intellectual Property Organization (WIPO) for the Nordic Patent Institute, an international organization common to Denmark, Iceland, Norway and Sweden.
				XN if interpret_user_assigned && interpret_world_intellectual_property_organisation => Europe,
				
				// Used by the World Intellectual Property Organization (WIPO) as an indicator for unknown states, other entities or organizations.
				XU if interpret_user_assigned && interpret_world_intellectual_property_organisation=> Unknown,
				
				// Used by the World Intellectual Property Organization (WIPO) as an indicator for the Visegrad Patent Institute.
				XV if interpret_user_assigned && interpret_world_intellectual_property_organisation=> Europe,
				
				// Used by the World Intellectual Property Organization (WIPO) as an indicator for unknown states, other entities or organizations.
				XX if interpret_user_assigned && interpret_world_intellectual_property_organisation=> Unknown,
				
				// Used by UN/LOCODE to represent installations in international waters.
				XZ if interpret_user_assigned => World,
				
				// Used by the World Intellectual Property Organization (WIPO) for the Nordic Patent Institute, an international organization common to Denmark, Iceland, Norway and Sweden.
				XA | XB | XC | XD | XE | XF | XG | XH | XI | XJ | XK | XL | XM | XN | XO | XP | XQ | XR | XS | XT | XU | XV | XW | XX | XY | XZ => UserAssigned,
				
				// Used by the International Standard Recording Code (ISRC) for some registrants.
				// Used by the Unicode Common Locale Data Repository (CLDR) to represent "Unknown or Invalid Territory".
				ZZ if interpret_user_assigned => Unknown,
				
				_ => (),
			},
			
			Alpha3(alpha_3) => match alpha_3
			{
				FRA => (),
				
				ESP => (),
			}
		}
	}
}

impl_xmp_attribute_value_parse_str!
(
	IptcWorldRegion, IptcWorldRegion,

	"World" => World,
	
	"Africa" => Africa,
	
	"South America" => SouthAmerica,
	
	"Oceania" => Oceania,
	
	"North America" => NorthAmerica,
	
	"Asia" => Asia,
	
	"Europe" => Europe,
	
	"Antarctica" => Antarctica,
);
