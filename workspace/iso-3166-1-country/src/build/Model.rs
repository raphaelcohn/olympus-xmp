// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// From `EFSRCA.xlsx` downloaded from <https://unterm.un.org/unterm/country>.
struct UnTermCountryName
{
	short: &'static str,
	
	long: &'static str,
}

struct UnM49CountryName
{
	short: &'static str,
	
	/// Burma.
	/// Democratic Kampuchea.
	/// Zaire.
	/// Hong Kong ???
	/// Ivory Coast.
	/// Upper Volta.
	/// Faroes (need to check which spelling),
	historic_short: Option<&'static str>,
}

struct CountryName
{
	unsd: UnM49CountryName,
	
	unterm: Option<UnTermCountryName>,
}

/// `A-Z`, space, comma, hyphen and period.
struct UnM49UpperCaseTwelveCharacterEnglishLanguageAbbreviation(ArrayVec<u8, 12>);

struct Names<N, X>
{
	arabic: N,
	
	chinese: N,
	
	english: N,
	
	french: N,
	
	russian: N,
	
	spanish: N,
	
	un_m49_upper_case_twelve_character: X,
}

/*
	Alpha2 / Alpha3 / M49 perms
	
	M49, Alpha2, Alpha3
	M49, Alpha2, None	(Sark)
	None, Alpha2, None	(Canary Islands)
 */

struct Country2
{
	names: Names<CountryName, UnM49UpperCaseTwelveCharacterEnglishLanguageAbbreviation>,
	
	m49: M49Code2,
	
	iso_alpha_2: [u8; 2],
	
	iso_alpha_3: Option<[u8; 3]>,
	
	parent_intermediate_region: Option<&'static IntermediateRegion>,
	
	parent_sub_region: Option<&'static SubRegion>,
	
	/// Only ever `None` for Antartica.
	parent_region: Option<&'static Region>,
	
	/// If not empty then obsolete.
	replacements: &'static [Country2],
}

// This could be an enum, too; there's a lot to be said for that.
struct M49Code2(u16);
static Map: std::collections::HashMap<M49Code2, M49Code2Variant> = AAAA;


struct World
{
	names: Names<UnM49CountryName, ()>,
}

struct Region
{
	names: Names<UnM49CountryName, ()>,
	
	parent: (M49Code2, &'static World),
}

struct SubRegion
{
	names: &'static Names<UnM49CountryName, ()>,
	
	// TODO: Rarely, can have a supra-region parent.
	
	parent: (M49Code2, &'static Region),
}

// TODO: There is a UnM49UpperCaseTwelveCharacterEnglishLanguageAbbreviation for Channel Islands, ?but this is because it was formerly a country?
struct IntermediateRegion
{
	names: &'static Names<UnM49CountryName, ()>,
	
	parent: (M49Code2, &'static SubRegion),
}

struct EconomicAndTradeGrouping
{
	names: &'static Names<UnM49CountryName, ()>,
	
	// TODO: This is problematic as it will require a circular set of references.
	constituent_countries: &'static [(M49Code2, &'static Country2)],
}

struct DevelopedAndDeveloping
{
	names: &'static Names<UnM49CountryName, ()>,
	
	// TODO: This is problematic as it will require a circular set of references.
	constituent_countries: &'static [(M49Code2, &'static Country2)],
}

enum M49Code2Variant
{
	// TODO: Developed and developing regions.
	
	// TODO: 896 / 898.
	
	/// Always `001`.
	World(&'static World),
	
	Region(Region),
	
	/// For South-Central Asia (now obsolete); formerly a SubRegion.
	/// The continent of North America (003) comprises Northern America (021), Caribbean (029) and Central America (013).
	SupraRegion
	{
		
		replacements: xxxx
	},
	
	SubRegion(SubRegion),
	
	IntermediateRegion(IntermediateRegion),
	
	Country(&'static Country2),
	
	// eg Associatation of Southeast Asian Nations (ASEAN)
	// eg Commonwealth of Independent States (CIS)
	// eg Southern African Customs Union (SACU).
	// eg Belgium-Luxembourg
	// eg Italy-San Marino-Holy See
	// eg France -Monaco
	// eg United States including Puerto Rico
	// eg United States including Puerto Rico and United States Virgin Islands (overlap)
	// Could also include the least developed countries (LDCS).
	EconomicAndTradeGrouping
	{
		names: &'static Names<UnM49CountryName, ()>,
		
		constituent_countries: &'static [(M49Code2, &'static Country2)],
	},
	
	DevelopedAndDeveloping(DevelopedAndDeveloping),
}
