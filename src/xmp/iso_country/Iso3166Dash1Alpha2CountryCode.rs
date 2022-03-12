// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An ISO 3166-1 Alpha-2 country code.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum Iso3166Dash1Alpha2CountryCode
{
	/// User assigned.
	AA = Self::letters_to_token(b"AA"),
	
	/// TODO.
	AB = Self::letters_to_token(b"AB"),
	
	/// Exceptional reservation.
	///
	/// Ascension Island.
	AC = Self::letters_to_token(b"AC"),
	
	/// TODO.
	AD = Self::letters_to_token(b"AD"),
	
	/// TODO.
	AE = Self::letters_to_token(b"AE"),
	
	/// Afghanistan.
	AF = Self::letters_to_token(b"AF"),
	
	/// TODO.
	AG = Self::letters_to_token(b"AG"),
	
	/// TODO.
	AH = Self::letters_to_token(b"AH"),
	
	/// Anguilla.
	///
	/// Formally was code for French Afars and Issas before the code was deleted and reassigned to Anguilla.
	AI = Self::letters_to_token(b"AI"),
	
	/// TODO.
	AJ = Self::letters_to_token(b"AJ"),
	
	/// TODO.
	AK = Self::letters_to_token(b"AK"),
	
	/// TODO.
	AL = Self::letters_to_token(b"AL"),
	
	/// TODO.
	AM = Self::letters_to_token(b"AM"),
	
	/// TODO.
	AN = Self::letters_to_token(b"AN"),
	
	/// TODO.
	AO = Self::letters_to_token(b"AO"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// African Regional Industrial Property Organization (ARIPO).
	AP = Self::letters_to_token(b"AP"),
	
	/// TODO.
	AQ = Self::letters_to_token(b"AQ"),
	
	/// TODO.
	AR = Self::letters_to_token(b"AR"),
	
	/// TODO.
	AS = Self::letters_to_token(b"AS"),
	
	/// TODO.
	AT = Self::letters_to_token(b"AT"),
	
	/// TODO.
	AU = Self::letters_to_token(b"AU"),
	
	/// TODO.
	AV = Self::letters_to_token(b"AV"),
	
	/// TODO.
	AW = Self::letters_to_token(b"AW"),
	
	/// TODO.
	AX = Self::letters_to_token(b"AX"),
	
	/// TODO.
	AY = Self::letters_to_token(b"AY"),
	
	/// TODO.
	AZ = Self::letters_to_token(b"AZ"),
	
	/// TODO.
	BA = Self::letters_to_token(b"BA"),
	
	/// TODO.
	BB = Self::letters_to_token(b"BB"),
	
	/// TODO.
	BC = Self::letters_to_token(b"BC"),
	
	/// TODO.
	BD = Self::letters_to_token(b"BD"),
	
	/// TODO.
	BE = Self::letters_to_token(b"BE"),
	
	/// TODO.
	BF = Self::letters_to_token(b"BF"),
	
	/// TODO.
	BG = Self::letters_to_token(b"BG"),
	
	/// TODO.
	BH = Self::letters_to_token(b"BH"),
	
	/// TODO.
	BI = Self::letters_to_token(b"BI"),
	
	/// TODO.
	BJ = Self::letters_to_token(b"BJ"),
	
	/// TODO.
	BK = Self::letters_to_token(b"BK"),
	
	/// TODO.
	BL = Self::letters_to_token(b"BL"),
	
	/// TODO.
	BM = Self::letters_to_token(b"BM"),
	
	/// TODO.
	BN = Self::letters_to_token(b"BN"),
	
	/// TODO.
	BO = Self::letters_to_token(b"BO"),
	
	/// TODO.
	BP = Self::letters_to_token(b"BP"),
	
	/// Bonaire, Sint Eustatius and Saba.
	///
	/// Formally was code for British Antarctic Territory before the code was deleted and reassigned to Bonaire, Sint Eustatius and Saba.
	BQ = Self::letters_to_token(b"BQ"),
	
	/// TODO.
	BR = Self::letters_to_token(b"BR"),
	
	/// TODO.
	BS = Self::letters_to_token(b"BS"),
	
	/// TODO.
	BT = Self::letters_to_token(b"BT"),
	
	/// TODO.
	BU = Self::letters_to_token(b"BU"),
	
	/// TODO.
	BV = Self::letters_to_token(b"BV"),
	
	/// TODO.
	BW = Self::letters_to_token(b"BW"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// Benelux Trademarks and Designs Office (BOIP).
	BX = Self::letters_to_token(b"BX"),
	
	/// TODO.
	BY = Self::letters_to_token(b"BY"),
	
	/// TODO.
	BZ = Self::letters_to_token(b"BZ"),
	
	/// TODO.
	CA = Self::letters_to_token(b"CA"),
	
	/// TODO.
	CB = Self::letters_to_token(b"CB"),
	
	/// TODO.
	CC = Self::letters_to_token(b"CC"),
	
	/// TODO.
	CD = Self::letters_to_token(b"CD"),
	
	/// TODO.
	CE = Self::letters_to_token(b"CE"),
	
	/// TODO.
	CF = Self::letters_to_token(b"CF"),
	
	/// TODO.
	CG = Self::letters_to_token(b"CG"),
	
	/// TODO.
	CH = Self::letters_to_token(b"CH"),
	
	/// TODO.
	CI = Self::letters_to_token(b"CI"),
	
	/// TODO.
	CJ = Self::letters_to_token(b"CJ"),
	
	/// TODO.
	CK = Self::letters_to_token(b"CK"),
	
	/// TODO.
	CL = Self::letters_to_token(b"CL"),
	
	/// TODO.
	CM = Self::letters_to_token(b"CM"),
	
	/// TODO.
	CN = Self::letters_to_token(b"CN"),
	
	/// TODO.
	CO = Self::letters_to_token(b"CO"),
	
	/// Exceptional reservation.
	///
	/// Clipperton Island.
	CP = Self::letters_to_token(b"CP"),
	
	/// Exceptional reservation.
	///
	/// Island of Sark.
	CQ = Self::letters_to_token(b"CQ"),
	
	/// TODO.
	CR = Self::letters_to_token(b"CR"),
	
	/// Deleted.
	///
	/// Was Canton and Enderbury Islands.
	/// Merged into Kiribati (`KI`).
	CS = Self::letters_to_token(b"CS"),
	
	/// TODO.
	CT = Self::letters_to_token(b"CT"),
	
	/// TODO.
	CU = Self::letters_to_token(b"CU"),
	
	/// TODO.
	CV = Self::letters_to_token(b"CV"),
	
	/// TODO.
	CW = Self::letters_to_token(b"CW"),
	
	/// TODO.
	CX = Self::letters_to_token(b"CX"),
	
	/// TODO.
	CY = Self::letters_to_token(b"CY"),
	
	/// TODO.
	CZ = Self::letters_to_token(b"CZ"),
	
	/// TODO.
	DA = Self::letters_to_token(b"DA"),
	
	/// TODO.
	DB = Self::letters_to_token(b"DB"),
	
	/// TODO.
	DC = Self::letters_to_token(b"DC"),
	
	/// Deleted.
	///
	/// Was German Democratic Republic.
	/// Merged into Germany (`DE`).
	DD = Self::letters_to_token(b"DD"),
	
	/// TODO.
	DE = Self::letters_to_token(b"DE"),
	
	/// TODO.
	DF = Self::letters_to_token(b"DF"),
	
	/// Exceptional reservation.
	///
	/// Diego Garcia.
	DG = Self::letters_to_token(b"DG"),
	
	/// TODO.
	DH = Self::letters_to_token(b"DH"),
	
	/// TODO.
	DI = Self::letters_to_token(b"DI"),
	
	/// TODO.
	DJ = Self::letters_to_token(b"DJ"),
	
	/// TODO.
	DK = Self::letters_to_token(b"DK"),
	
	/// TODO.
	DL = Self::letters_to_token(b"DL"),
	
	/// TODO.
	DM = Self::letters_to_token(b"DM"),
	
	/// TODO.
	DN = Self::letters_to_token(b"DN"),
	
	/// TODO.
	DO = Self::letters_to_token(b"DO"),
	
	/// TODO.
	DP = Self::letters_to_token(b"DP"),
	
	/// TODO.
	DQ = Self::letters_to_token(b"DQ"),
	
	/// TODO.
	DR = Self::letters_to_token(b"DR"),
	
	/// TODO.
	DS = Self::letters_to_token(b"DS"),
	
	/// TODO.
	DT = Self::letters_to_token(b"DT"),
	
	/// TODO.
	DU = Self::letters_to_token(b"DU"),
	
	/// TODO.
	DV = Self::letters_to_token(b"DV"),
	
	/// TODO.
	DW = Self::letters_to_token(b"DW"),
	
	/// TODO.
	DX = Self::letters_to_token(b"DX"),
	
	/// TODO.
	DY = Self::letters_to_token(b"DY"),
	
	/// TODO.
	DZ = Self::letters_to_token(b"DZ"),
	
	/// Exceptional reservation.
	///
	/// Ceuta, Melilla.
	///
	///	World Intellectual Property Organization (WIPO) uses this in its own standards instead of `EV`.
	EA = Self::letters_to_token(b"EA"),
	
	/// TODO.
	EB = Self::letters_to_token(b"EB"),
	
	/// TODO.
	EC = Self::letters_to_token(b"EC"),
	
	/// TODO.
	ED = Self::letters_to_token(b"ED"),
	
	/// TODO.
	EE = Self::letters_to_token(b"EE"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// Union of Countries under the European Community Patent Convention.
	EF = Self::letters_to_token(b"EF"),
	
	/// TODO.
	EG = Self::letters_to_token(b"EG"),
	
	/// TODO.
	EH = Self::letters_to_token(b"EH"),
	
	/// TODO.
	EI = Self::letters_to_token(b"EI"),
	
	/// TODO.
	EJ = Self::letters_to_token(b"EJ"),
	
	/// TODO.
	EK = Self::letters_to_token(b"EK"),
	
	/// TODO.
	EL = Self::letters_to_token(b"EL"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// European Trademark Office (EUIPO).
	EM = Self::letters_to_token(b"EM"),
	
	/// TODO.
	EN = Self::letters_to_token(b"EN"),
	
	/// TODO.
	EO = Self::letters_to_token(b"EO"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// European Patent Organization (EPOrg), ie union of countries under the European Patent Convention (EPC).
	EP = Self::letters_to_token(b"EP"),
	
	/// TODO.
	EQ = Self::letters_to_token(b"EQ"),
	
	/// TODO.
	ER = Self::letters_to_token(b"ER"),
	
	/// Spain.
	ES = Self::letters_to_token(b"ES"),
	
	/// TODO.
	ET = Self::letters_to_token(b"ET"),
	
	/// Exceptional reservation.
	///
	/// European Union.
	EU = Self::letters_to_token(b"EU"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// Eurasian Patent Organization (EAPO).
	/// Note: WIPO prefers to use `EA` in its own standards.
	EV = Self::letters_to_token(b"EV"),
	
	/// TODO.
	EW = Self::letters_to_token(b"EW"),
	
	/// TODO.
	EX = Self::letters_to_token(b"EX"),
	
	/// TODO.
	EY = Self::letters_to_token(b"EY"),
	
	/// Exceptional reservation.
	///
	/// Eurozone.
	EZ = Self::letters_to_token(b"EZ"),
	
	/// TODO.
	FA = Self::letters_to_token(b"FA"),
	
	/// TODO.
	FB = Self::letters_to_token(b"FB"),
	
	/// TODO.
	FC = Self::letters_to_token(b"FC"),
	
	/// TODO.
	FD = Self::letters_to_token(b"FD"),
	
	/// TODO.
	FE = Self::letters_to_token(b"FE"),
	
	/// TODO.
	FF = Self::letters_to_token(b"FF"),
	
	/// TODO.
	FG = Self::letters_to_token(b"FG"),
	
	/// TODO.
	FH = Self::letters_to_token(b"FH"),
	
	/// TODO.
	FI = Self::letters_to_token(b"FI"),
	
	/// TODO.
	FJ = Self::letters_to_token(b"FJ"),
	
	/// TODO.
	FK = Self::letters_to_token(b"FK"),
	
	/// TODO.
	FL = Self::letters_to_token(b"FL"),
	
	/// TODO.
	FM = Self::letters_to_token(b"FM"),
	
	/// TODO.
	FN = Self::letters_to_token(b"FN"),
	
	/// TODO.
	FO = Self::letters_to_token(b"FO"),
	
	/// TODO.
	FP = Self::letters_to_token(b"FP"),
	
	/// TODO.
	FQ = Self::letters_to_token(b"FQ"),
	
	/// France.
	FR = Self::letters_to_token(b"FR"),
	
	/// TODO.
	FS = Self::letters_to_token(b"FS"),
	
	/// TODO.
	FT = Self::letters_to_token(b"FT"),
	
	/// TODO.
	FU = Self::letters_to_token(b"FU"),
	
	/// TODO.
	FV = Self::letters_to_token(b"FV"),
	
	/// TODO.
	FW = Self::letters_to_token(b"FW"),
	
	/// Exceptional reservation.
	///
	/// France, Metropolitan.
	FX = Self::letters_to_token(b"FX"),
	
	/// TODO.
	FY = Self::letters_to_token(b"FY"),
	
	/// TODO.
	FZ = Self::letters_to_token(b"FZ"),
	
	/// TODO.
	GA = Self::letters_to_token(b"GA"),
	
	/// United Kingdom of Great Britain and Northern Ireland (the).
	GB = Self::letters_to_token(b"GB"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// Patent Office of the Cooperation Council for the Arab States of the Gulf (GCCPO).
	GC = Self::letters_to_token(b"GC"),
	
	/// TODO.
	GD = Self::letters_to_token(b"GD"),
	
	/// TODO.
	GE = Self::letters_to_token(b"GE"),
	
	/// TODO.
	GF = Self::letters_to_token(b"GF"),
	
	/// Guernsey
	GG = Self::letters_to_token(b"GG"),
	
	/// TODO.
	GH = Self::letters_to_token(b"GH"),
	
	/// TODO.
	GI = Self::letters_to_token(b"GI"),
	
	/// TODO.
	GJ = Self::letters_to_token(b"GJ"),
	
	/// TODO.
	GK = Self::letters_to_token(b"GK"),
	
	/// TODO.
	GL = Self::letters_to_token(b"GL"),
	
	/// TODO.
	GM = Self::letters_to_token(b"GM"),
	
	/// TODO.
	GN = Self::letters_to_token(b"GN"),
	
	/// TODO.
	GO = Self::letters_to_token(b"GO"),
	
	/// TODO.
	GP = Self::letters_to_token(b"GP"),
	
	/// TODO.
	GQ = Self::letters_to_token(b"GQ"),
	
	/// TODO.
	GR = Self::letters_to_token(b"GR"),
	
	/// TODO.
	GS = Self::letters_to_token(b"GS"),
	
	/// TODO.
	GT = Self::letters_to_token(b"GT"),
	
	/// TODO.
	GU = Self::letters_to_token(b"GU"),
	
	/// TODO.
	GV = Self::letters_to_token(b"GV"),
	
	/// TODO.
	GW = Self::letters_to_token(b"GW"),
	
	/// TODO.
	GX = Self::letters_to_token(b"GX"),
	
	/// TODO.
	GY = Self::letters_to_token(b"GY"),
	
	/// TODO.
	GZ = Self::letters_to_token(b"GZ"),
	
	/// TODO.
	HA = Self::letters_to_token(b"HA"),
	
	/// TODO.
	HB = Self::letters_to_token(b"HB"),
	
	/// TODO.
	HC = Self::letters_to_token(b"HC"),
	
	/// TODO.
	HD = Self::letters_to_token(b"HD"),
	
	/// TODO.
	HE = Self::letters_to_token(b"HE"),
	
	/// TODO.
	HF = Self::letters_to_token(b"HF"),
	
	/// TODO.
	HG = Self::letters_to_token(b"HG"),
	
	/// TODO.
	HH = Self::letters_to_token(b"HH"),
	
	/// TODO.
	HI = Self::letters_to_token(b"HI"),
	
	/// TODO.
	HJ = Self::letters_to_token(b"HJ"),
	
	/// TODO.
	HK = Self::letters_to_token(b"HK"),
	
	/// TODO.
	HL = Self::letters_to_token(b"HL"),
	
	/// TODO.
	HM = Self::letters_to_token(b"HM"),
	
	/// TODO.
	HN = Self::letters_to_token(b"HN"),
	
	/// TODO.
	HO = Self::letters_to_token(b"HO"),
	
	/// TODO.
	HP = Self::letters_to_token(b"HP"),
	
	/// TODO.
	HQ = Self::letters_to_token(b"HQ"),
	
	/// TODO.
	HR = Self::letters_to_token(b"HR"),
	
	/// TODO.
	HS = Self::letters_to_token(b"HS"),
	
	/// TODO.
	HT = Self::letters_to_token(b"HT"),
	
	/// TODO.
	HU = Self::letters_to_token(b"HU"),
	
	/// TODO.
	HV = Self::letters_to_token(b"HV"),
	
	/// TODO.
	HW = Self::letters_to_token(b"HW"),
	
	/// TODO.
	HX = Self::letters_to_token(b"HX"),
	
	/// TODO.
	HY = Self::letters_to_token(b"HY"),
	
	/// TODO.
	HZ = Self::letters_to_token(b"HZ"),
	
	/// TODO.
	IA = Self::letters_to_token(b"IA"),
	
	/// TODO.
	IB = Self::letters_to_token(b"IB"),
	
	/// Exceptional reservation.
	///
	/// Canary Islands.
	IC = Self::letters_to_token(b"IC"),
	
	/// TODO.
	ID = Self::letters_to_token(b"ID"),
	
	/// TODO.
	IE = Self::letters_to_token(b"IE"),
	
	/// TODO.
	IF = Self::letters_to_token(b"IF"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// International Bureau of WIPO.
	IG = Self::letters_to_token(b"IG"),
	
	/// TODO.
	IH = Self::letters_to_token(b"IH"),
	
	/// TODO.
	II = Self::letters_to_token(b"II"),
	
	/// TODO.
	IJ = Self::letters_to_token(b"IJ"),
	
	/// TODO.
	IK = Self::letters_to_token(b"IK"),
	
	/// TODO.
	IL = Self::letters_to_token(b"IL"),
	
	/// Isle of Man.
	IM = Self::letters_to_token(b"IM"),
	
	/// TODO.
	IN = Self::letters_to_token(b"IN"),
	
	/// British Indian Ocean Territory.
	IO = Self::letters_to_token(b"IO"),
	
	/// TODO.
	IP = Self::letters_to_token(b"IP"),
	
	/// TODO.
	IQ = Self::letters_to_token(b"IQ"),
	
	/// TODO.
	IR = Self::letters_to_token(b"IR"),
	
	/// TODO.
	IS = Self::letters_to_token(b"IS"),
	
	/// TODO.
	IT = Self::letters_to_token(b"IT"),
	
	/// TODO.
	IU = Self::letters_to_token(b"IU"),
	
	/// TODO.
	IV = Self::letters_to_token(b"IV"),
	
	/// TODO.
	IW = Self::letters_to_token(b"IW"),
	
	/// TODO.
	IX = Self::letters_to_token(b"IX"),
	
	/// TODO.
	IY = Self::letters_to_token(b"IY"),
	
	/// TODO.
	IZ = Self::letters_to_token(b"IZ"),
	
	/// TODO.
	JA = Self::letters_to_token(b"JA"),
	
	/// TODO.
	JB = Self::letters_to_token(b"JB"),
	
	/// TODO.
	JC = Self::letters_to_token(b"JC"),
	
	/// TODO.
	JD = Self::letters_to_token(b"JD"),
	
	/// Jersey.
	JE = Self::letters_to_token(b"JE"),
	
	/// TODO.
	JF = Self::letters_to_token(b"JF"),
	
	/// TODO.
	JG = Self::letters_to_token(b"JG"),
	
	/// TODO.
	JH = Self::letters_to_token(b"JH"),
	
	/// TODO.
	JI = Self::letters_to_token(b"JI"),
	
	/// TODO.
	JJ = Self::letters_to_token(b"JJ"),
	
	/// TODO.
	JK = Self::letters_to_token(b"JK"),
	
	/// TODO.
	JL = Self::letters_to_token(b"JL"),
	
	/// TODO.
	JM = Self::letters_to_token(b"JM"),
	
	/// TODO.
	JN = Self::letters_to_token(b"JN"),
	
	/// TODO.
	JO = Self::letters_to_token(b"JO"),
	
	/// TODO.
	JP = Self::letters_to_token(b"JP"),
	
	/// TODO.
	JQ = Self::letters_to_token(b"JQ"),
	
	/// TODO.
	JR = Self::letters_to_token(b"JR"),
	
	/// TODO.
	JS = Self::letters_to_token(b"JS"),
	
	/// TODO.
	JT = Self::letters_to_token(b"JT"),
	
	/// TODO.
	JU = Self::letters_to_token(b"JU"),
	
	/// TODO.
	JV = Self::letters_to_token(b"JV"),
	
	/// TODO.
	JW = Self::letters_to_token(b"JW"),
	
	/// TODO.
	JX = Self::letters_to_token(b"JX"),
	
	/// TODO.
	JY = Self::letters_to_token(b"JY"),
	
	/// TODO.
	JZ = Self::letters_to_token(b"JZ"),
	
	/// TODO.
	KA = Self::letters_to_token(b"KA"),
	
	/// TODO.
	KB = Self::letters_to_token(b"KB"),
	
	/// TODO.
	KC = Self::letters_to_token(b"KC"),
	
	/// TODO.
	KD = Self::letters_to_token(b"KD"),
	
	/// TODO.
	KE = Self::letters_to_token(b"KE"),
	
	/// TODO.
	KF = Self::letters_to_token(b"KF"),
	
	/// TODO.
	KG = Self::letters_to_token(b"KG"),
	
	/// TODO.
	KH = Self::letters_to_token(b"KH"),
	
	/// TODO.
	KI = Self::letters_to_token(b"KI"),
	
	/// TODO.
	KJ = Self::letters_to_token(b"KJ"),
	
	/// TODO.
	KK = Self::letters_to_token(b"KK"),
	
	/// TODO.
	KL = Self::letters_to_token(b"KL"),
	
	/// TODO.
	KM = Self::letters_to_token(b"KM"),
	
	/// TODO.
	KN = Self::letters_to_token(b"KN"),
	
	/// TODO.
	KO = Self::letters_to_token(b"KO"),
	
	/// TODO.
	KP = Self::letters_to_token(b"KP"),
	
	/// TODO.
	KQ = Self::letters_to_token(b"KQ"),
	
	/// TODO.
	KR = Self::letters_to_token(b"KR"),
	
	/// TODO.
	KS = Self::letters_to_token(b"KS"),
	
	/// TODO.
	KT = Self::letters_to_token(b"KT"),
	
	/// TODO.
	KU = Self::letters_to_token(b"KU"),
	
	/// TODO.
	KV = Self::letters_to_token(b"KV"),
	
	/// TODO.
	KW = Self::letters_to_token(b"KW"),
	
	/// TODO.
	KX = Self::letters_to_token(b"KX"),
	
	/// TODO.
	KY = Self::letters_to_token(b"KY"),
	
	/// TODO.
	KZ = Self::letters_to_token(b"KZ"),
	
	/// TODO.
	LA = Self::letters_to_token(b"LA"),
	
	/// TODO.
	LB = Self::letters_to_token(b"LB"),
	
	/// TODO.
	LC = Self::letters_to_token(b"LC"),
	
	/// TODO.
	LD = Self::letters_to_token(b"LD"),
	
	/// TODO.
	LE = Self::letters_to_token(b"LE"),
	
	/// TODO.
	LF = Self::letters_to_token(b"LF"),
	
	/// TODO.
	LG = Self::letters_to_token(b"LG"),
	
	/// TODO.
	LH = Self::letters_to_token(b"LH"),
	
	/// TODO.
	LI = Self::letters_to_token(b"LI"),
	
	/// TODO.
	LJ = Self::letters_to_token(b"LJ"),
	
	/// TODO.
	LK = Self::letters_to_token(b"LK"),
	
	/// TODO.
	LL = Self::letters_to_token(b"LL"),
	
	/// TODO.
	LM = Self::letters_to_token(b"LM"),
	
	/// TODO.
	LN = Self::letters_to_token(b"LN"),
	
	/// TODO.
	LO = Self::letters_to_token(b"LO"),
	
	/// TODO.
	LP = Self::letters_to_token(b"LP"),
	
	/// TODO.
	LQ = Self::letters_to_token(b"LQ"),
	
	/// TODO.
	LR = Self::letters_to_token(b"LR"),
	
	/// TODO.
	LS = Self::letters_to_token(b"LS"),
	
	/// TODO.
	LT = Self::letters_to_token(b"LT"),
	
	/// TODO.
	LU = Self::letters_to_token(b"LU"),
	
	/// TODO.
	LV = Self::letters_to_token(b"LV"),
	
	/// TODO.
	LW = Self::letters_to_token(b"LW"),
	
	/// TODO.
	LX = Self::letters_to_token(b"LX"),
	
	/// TODO.
	LY = Self::letters_to_token(b"LY"),
	
	/// TODO.
	LZ = Self::letters_to_token(b"LZ"),
	
	/// TODO.
	MA = Self::letters_to_token(b"MA"),
	
	/// TODO.
	MB = Self::letters_to_token(b"MB"),
	
	/// TODO.
	MC = Self::letters_to_token(b"MC"),
	
	/// TODO.
	MD = Self::letters_to_token(b"MD"),
	
	/// TODO.
	ME = Self::letters_to_token(b"ME"),
	
	/// TODO.
	MF = Self::letters_to_token(b"MF"),
	
	/// TODO.
	MG = Self::letters_to_token(b"MG"),
	
	/// TODO.
	MH = Self::letters_to_token(b"MH"),
	
	/// TODO.
	MI = Self::letters_to_token(b"MI"),
	
	/// TODO.
	MJ = Self::letters_to_token(b"MJ"),
	
	/// TODO.
	MK = Self::letters_to_token(b"MK"),
	
	/// TODO.
	ML = Self::letters_to_token(b"ML"),
	
	/// TODO.
	MM = Self::letters_to_token(b"MM"),
	
	/// TODO.
	MN = Self::letters_to_token(b"MN"),
	
	/// TODO.
	MO = Self::letters_to_token(b"MO"),
	
	/// TODO.
	MP = Self::letters_to_token(b"MP"),
	
	/// TODO.
	MQ = Self::letters_to_token(b"MQ"),
	
	/// TODO.
	MR = Self::letters_to_token(b"MR"),
	
	/// TODO.
	MS = Self::letters_to_token(b"MS"),
	
	/// TODO.
	MT = Self::letters_to_token(b"MT"),
	
	/// TODO.
	MU = Self::letters_to_token(b"MU"),
	
	/// TODO.
	MV = Self::letters_to_token(b"MV"),
	
	/// TODO.
	MW = Self::letters_to_token(b"MW"),
	
	/// TODO.
	MX = Self::letters_to_token(b"MX"),
	
	/// TODO.
	MY = Self::letters_to_token(b"MY"),
	
	/// TODO.
	MZ = Self::letters_to_token(b"MZ"),
	
	/// TODO.
	NA = Self::letters_to_token(b"NA"),
	
	/// TODO.
	NB = Self::letters_to_token(b"NB"),
	
	/// TODO.
	NC = Self::letters_to_token(b"NC"),
	
	/// TODO.
	ND = Self::letters_to_token(b"ND"),
	
	/// TODO.
	NE = Self::letters_to_token(b"NE"),
	
	/// Norfolk Island.
	NF = Self::letters_to_token(b"NF"),
	
	/// TODO.
	NG = Self::letters_to_token(b"NG"),
	
	/// TODO.
	NH = Self::letters_to_token(b"NH"),
	
	/// TODO.
	NI = Self::letters_to_token(b"NI"),
	
	/// TODO.
	NJ = Self::letters_to_token(b"NJ"),
	
	/// TODO.
	NK = Self::letters_to_token(b"NK"),
	
	/// TODO.
	NL = Self::letters_to_token(b"NL"),
	
	/// TODO.
	NM = Self::letters_to_token(b"NM"),
	
	/// TODO.
	NN = Self::letters_to_token(b"NN"),
	
	/// TODO.
	NO = Self::letters_to_token(b"NO"),
	
	/// TODO.
	NP = Self::letters_to_token(b"NP"),
	
	/// TODO.
	NQ = Self::letters_to_token(b"NQ"),
	
	/// TODO.
	NR = Self::letters_to_token(b"NR"),
	
	/// TODO.
	NS = Self::letters_to_token(b"NS"),
	
	/// TODO.
	NT = Self::letters_to_token(b"NT"),
	
	/// TODO.
	NU = Self::letters_to_token(b"NU"),
	
	/// TODO.
	NV = Self::letters_to_token(b"NV"),
	
	/// TODO.
	NW = Self::letters_to_token(b"NW"),
	
	/// TODO.
	NX = Self::letters_to_token(b"NX"),
	
	/// TODO.
	NY = Self::letters_to_token(b"NY"),
	
	/// TODO.
	NZ = Self::letters_to_token(b"NZ"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	/// African Intellectual Property Organization (OAPI).
	OA = Self::letters_to_token(b"OA"),
	
	/// TODO.
	OB = Self::letters_to_token(b"OB"),
	
	/// TODO.
	OC = Self::letters_to_token(b"OC"),
	
	/// TODO.
	OD = Self::letters_to_token(b"OD"),
	
	/// TODO.
	OE = Self::letters_to_token(b"OE"),
	
	/// TODO.
	OF = Self::letters_to_token(b"OF"),
	
	/// TODO.
	OG = Self::letters_to_token(b"OG"),
	
	/// TODO.
	OH = Self::letters_to_token(b"OH"),
	
	/// TODO.
	OI = Self::letters_to_token(b"OI"),
	
	/// TODO.
	OJ = Self::letters_to_token(b"OJ"),
	
	/// TODO.
	OK = Self::letters_to_token(b"OK"),
	
	/// TODO.
	OL = Self::letters_to_token(b"OL"),
	
	/// TODO.
	OM = Self::letters_to_token(b"OM"),
	
	/// TODO.
	ON = Self::letters_to_token(b"ON"),
	
	/// Escape code.
	OO = Self::letters_to_token(b"OO"),
	
	/// TODO.
	OP = Self::letters_to_token(b"OP"),
	
	/// TODO.
	OQ = Self::letters_to_token(b"OQ"),
	
	/// TODO.
	OR = Self::letters_to_token(b"OR"),
	
	/// TODO.
	OS = Self::letters_to_token(b"OS"),
	
	/// TODO.
	OT = Self::letters_to_token(b"OT"),
	
	/// TODO.
	OU = Self::letters_to_token(b"OU"),
	
	/// TODO.
	OV = Self::letters_to_token(b"OV"),
	
	/// TODO.
	OW = Self::letters_to_token(b"OW"),
	
	/// TODO.
	OX = Self::letters_to_token(b"OX"),
	
	/// TODO.
	OY = Self::letters_to_token(b"OY"),
	
	/// TODO.
	OZ = Self::letters_to_token(b"OZ"),
	
	/// TODO.
	PA = Self::letters_to_token(b"PA"),
	
	/// TODO.
	PB = Self::letters_to_token(b"PB"),
	
	/// TODO.
	PC = Self::letters_to_token(b"PC"),
	
	/// TODO.
	PD = Self::letters_to_token(b"PD"),
	
	/// TODO.
	PE = Self::letters_to_token(b"PE"),
	
	/// TODO.
	PF = Self::letters_to_token(b"PF"),
	
	/// TODO.
	PG = Self::letters_to_token(b"PG"),
	
	/// TODO.
	PH = Self::letters_to_token(b"PH"),
	
	/// TODO.
	PI = Self::letters_to_token(b"PI"),
	
	/// TODO.
	PJ = Self::letters_to_token(b"PJ"),
	
	/// TODO.
	PK = Self::letters_to_token(b"PK"),
	
	/// TODO.
	PL = Self::letters_to_token(b"PL"),
	
	/// TODO.
	PM = Self::letters_to_token(b"PM"),
	
	/// TODO.
	PN = Self::letters_to_token(b"PN"),
	
	/// TODO.
	PO = Self::letters_to_token(b"PO"),
	
	/// TODO.
	PP = Self::letters_to_token(b"PP"),
	
	/// TODO.
	PQ = Self::letters_to_token(b"PQ"),
	
	/// TODO.
	PR = Self::letters_to_token(b"PR"),
	
	/// TODO.
	PS = Self::letters_to_token(b"PS"),
	
	/// TODO.
	PT = Self::letters_to_token(b"PT"),
	
	/// TODO.
	PU = Self::letters_to_token(b"PU"),
	
	/// TODO.
	PV = Self::letters_to_token(b"PV"),
	
	/// TODO.
	PW = Self::letters_to_token(b"PW"),
	
	/// TODO.
	PX = Self::letters_to_token(b"PX"),
	
	/// TODO.
	PY = Self::letters_to_token(b"PY"),
	
	/// TODO.
	PZ = Self::letters_to_token(b"PZ"),
	
	/// TODO.
	QA = Self::letters_to_token(b"QA"),
	
	/// TODO.
	QB = Self::letters_to_token(b"QB"),
	
	/// TODO.
	QC = Self::letters_to_token(b"QC"),
	
	/// TODO.
	QD = Self::letters_to_token(b"QD"),
	
	/// TODO.
	QE = Self::letters_to_token(b"QE"),
	
	/// TODO.
	QF = Self::letters_to_token(b"QF"),
	
	/// TODO.
	QG = Self::letters_to_token(b"QG"),
	
	/// TODO.
	QH = Self::letters_to_token(b"QH"),
	
	/// TODO.
	QI = Self::letters_to_token(b"QI"),
	
	/// TODO.
	QJ = Self::letters_to_token(b"QJ"),
	
	/// TODO.
	QK = Self::letters_to_token(b"QK"),
	
	/// TODO.
	QL = Self::letters_to_token(b"QL"),
	
	/// User assigned.
	///
	/// Used by the International Standard Recording Code (ISRC) as a second country code for the United States.
	QM = Self::letters_to_token(b"QM"),
	
	/// User assigned.
	QN = Self::letters_to_token(b"QN"),
	
	/// User assigned.
	///
	/// Used by the Unicode Common Locale Data Repository (CLDR) to represent Outlying Oceania (a multi-territory region containing Antarctica, Bouvet Island, the Cocos (Keeling) Islands, Christmas Island, South Georgia and the South Sandwich Islands, Heard Island and McDonald Islands, the British Indian Ocean Territory, the French Southern Territories, and the United States Minor Outlying Islands).
	QO = Self::letters_to_token(b"QO"),
	
	/// User assigned.
	///
	///	Before the adoption of the macroregion code EU by ISO, the Unicode Common Locale Data Repository (CLDR) used QU to represent the European Union.
	QP = Self::letters_to_token(b"QP"),
	
	/// User assigned.
	QQ = Self::letters_to_token(b"QQ"),
	
	/// User assigned.
	QR = Self::letters_to_token(b"QR"),
	
	/// User assigned.
	QS = Self::letters_to_token(b"QS"),
	
	/// User assigned.
	QT = Self::letters_to_token(b"QT"),
	
	/// User assigned.
	QU = Self::letters_to_token(b"QU"),
	
	/// User assigned.
	QV = Self::letters_to_token(b"QV"),
	
	/// User assigned.
	QW = Self::letters_to_token(b"QW"),
	
	/// User assigned.
	QX = Self::letters_to_token(b"QX"),
	
	/// User assigned.
	QY = Self::letters_to_token(b"QY"),
	
	/// User assigned.
	///
	/// Used by the World Intellectual Property Organization (WIPO) as an indicator for the Community Plant Variety Office.
	QZ = Self::letters_to_token(b"QZ"),
	
	/// TODO.
	RA = Self::letters_to_token(b"RA"),
	
	/// TODO.
	RB = Self::letters_to_token(b"RB"),
	
	/// TODO.
	RC = Self::letters_to_token(b"RC"),
	
	/// TODO.
	RD = Self::letters_to_token(b"RD"),
	
	/// TODO.
	RE = Self::letters_to_token(b"RE"),
	
	/// TODO.
	RF = Self::letters_to_token(b"RF"),
	
	/// TODO.
	RG = Self::letters_to_token(b"RG"),
	
	/// TODO.
	RH = Self::letters_to_token(b"RH"),
	
	/// TODO.
	RI = Self::letters_to_token(b"RI"),
	
	/// TODO.
	RJ = Self::letters_to_token(b"RJ"),
	
	/// TODO.
	RK = Self::letters_to_token(b"RK"),
	
	/// TODO.
	RL = Self::letters_to_token(b"RL"),
	
	/// TODO.
	RM = Self::letters_to_token(b"RM"),
	
	/// TODO.
	RN = Self::letters_to_token(b"RN"),
	
	/// TODO.
	RO = Self::letters_to_token(b"RO"),
	
	/// TODO.
	RP = Self::letters_to_token(b"RP"),
	
	/// TODO.
	RQ = Self::letters_to_token(b"RQ"),
	
	/// TODO.
	RR = Self::letters_to_token(b"RR"),
	
	/// TODO.
	RS = Self::letters_to_token(b"RS"),
	
	/// TODO.
	RT = Self::letters_to_token(b"RT"),
	
	/// TODO.
	RU = Self::letters_to_token(b"RU"),
	
	/// TODO.
	RV = Self::letters_to_token(b"RV"),
	
	/// TODO.
	RW = Self::letters_to_token(b"RW"),
	
	/// TODO.
	RX = Self::letters_to_token(b"RX"),
	
	/// TODO.
	RY = Self::letters_to_token(b"RY"),
	
	/// TODO.
	RZ = Self::letters_to_token(b"RZ"),
	
	/// TODO.
	SA = Self::letters_to_token(b"SA"),
	
	/// TODO.
	SB = Self::letters_to_token(b"SB"),
	
	/// TODO.
	SC = Self::letters_to_token(b"SC"),
	
	/// TODO.
	SD = Self::letters_to_token(b"SD"),
	
	/// TODO.
	SE = Self::letters_to_token(b"SE"),
	
	/// TODO.
	SF = Self::letters_to_token(b"SF"),
	
	/// TODO.
	SG = Self::letters_to_token(b"SG"),
	
	/// Saint Helena, Ascension and Tristan da Cunha.
	SH = Self::letters_to_token(b"SH"),
	
	/// TODO.
	SI = Self::letters_to_token(b"SI"),
	
	/// TODO.
	SJ = Self::letters_to_token(b"SJ"),
	
	/// TODO.
	SK = Self::letters_to_token(b"SK"),
	
	/// TODO.
	SL = Self::letters_to_token(b"SL"),
	
	/// TODO.
	SM = Self::letters_to_token(b"SM"),
	
	/// TODO.
	SN = Self::letters_to_token(b"SN"),
	
	/// TODO.
	SO = Self::letters_to_token(b"SO"),
	
	/// TODO.
	SP = Self::letters_to_token(b"SP"),
	
	/// TODO.
	SQ = Self::letters_to_token(b"SQ"),
	
	/// TODO.
	SR = Self::letters_to_token(b"SR"),
	
	/// TODO.
	SS = Self::letters_to_token(b"SS"),
	
	/// TODO.
	ST = Self::letters_to_token(b"ST"),
	
	/// Exceptional reservation.
	///
	/// Soviet Union.
	SU = Self::letters_to_token(b"SU"),
	
	/// TODO.
	SV = Self::letters_to_token(b"SV"),
	
	/// TODO.
	SW = Self::letters_to_token(b"SW"),
	
	/// TODO.
	SX = Self::letters_to_token(b"SX"),
	
	/// TODO.
	SY = Self::letters_to_token(b"SY"),
	
	/// TODO.
	SZ = Self::letters_to_token(b"SZ"),
	
	/// Exceptional reservation.
	///
	/// Tristan da Cunha.
	TA = Self::letters_to_token(b"TA"),
	
	/// TODO.
	TB = Self::letters_to_token(b"TB"),
	
	/// TODO.
	TC = Self::letters_to_token(b"TC"),
	
	/// TODO.
	TD = Self::letters_to_token(b"TD"),
	
	/// TODO.
	TE = Self::letters_to_token(b"TE"),
	
	/// TODO.
	TF = Self::letters_to_token(b"TF"),
	
	/// TODO.
	TG = Self::letters_to_token(b"TG"),
	
	/// TODO.
	TH = Self::letters_to_token(b"TH"),
	
	/// TODO.
	TI = Self::letters_to_token(b"TI"),
	
	/// TODO.
	TJ = Self::letters_to_token(b"TJ"),
	
	/// TODO.
	TK = Self::letters_to_token(b"TK"),
	
	/// TODO.
	TL = Self::letters_to_token(b"TL"),
	
	/// TODO.
	TM = Self::letters_to_token(b"TM"),
	
	/// TODO.
	TN = Self::letters_to_token(b"TN"),
	
	/// TODO.
	TO = Self::letters_to_token(b"TO"),
	
	/// TODO.
	TP = Self::letters_to_token(b"TP"),
	
	/// TODO.
	TQ = Self::letters_to_token(b"TQ"),
	
	/// TODO.
	TR = Self::letters_to_token(b"TR"),
	
	/// TODO.
	TS = Self::letters_to_token(b"TS"),
	
	/// TODO.
	TT = Self::letters_to_token(b"TT"),
	
	/// TODO.
	TU = Self::letters_to_token(b"TU"),
	
	/// TODO.
	TV = Self::letters_to_token(b"TV"),
	
	/// TODO.
	TW = Self::letters_to_token(b"TW"),
	
	/// TODO.
	TX = Self::letters_to_token(b"TX"),
	
	/// TODO.
	TY = Self::letters_to_token(b"TY"),
	
	/// TODO.
	TZ = Self::letters_to_token(b"TZ"),
	
	/// TODO.
	UA = Self::letters_to_token(b"UA"),
	
	/// TODO.
	UB = Self::letters_to_token(b"UB"),
	
	/// TODO.
	UC = Self::letters_to_token(b"UC"),
	
	/// TODO.
	UD = Self::letters_to_token(b"UD"),
	
	/// TODO.
	UE = Self::letters_to_token(b"UE"),
	
	/// TODO.
	UF = Self::letters_to_token(b"UF"),
	
	/// TODO.
	UG = Self::letters_to_token(b"UG"),
	
	/// TODO.
	UH = Self::letters_to_token(b"UH"),
	
	/// TODO.
	UI = Self::letters_to_token(b"UI"),
	
	/// TODO.
	UJ = Self::letters_to_token(b"UJ"),
	
	/// Exceptional reservation.
	///
	/// United Kingdom of Great Britain and Northern Ireland (the).
	/// The code `GB` should be used, but this value is used for the DNS top-level country domain and by the European Commission.
	UK = Self::letters_to_token(b"UK"),
	
	/// TODO.
	UL = Self::letters_to_token(b"UL"),
	
	/// TODO.
	UM = Self::letters_to_token(b"UM"),
	
	/// Exceptional reservation.
	///
	/// United Nations.
	UN = Self::letters_to_token(b"UN"),
	
	/// TODO.
	UO = Self::letters_to_token(b"UO"),
	
	/// TODO.
	UP = Self::letters_to_token(b"UP"),
	
	/// TODO.
	UQ = Self::letters_to_token(b"UQ"),
	
	/// TODO.
	UR = Self::letters_to_token(b"UR"),
	
	/// TODO.
	US = Self::letters_to_token(b"US"),
	
	/// TODO.
	UT = Self::letters_to_token(b"UT"),
	
	/// TODO.
	UU = Self::letters_to_token(b"UU"),
	
	/// TODO.
	UV = Self::letters_to_token(b"UV"),
	
	/// TODO.
	UW = Self::letters_to_token(b"UW"),
	
	/// TODO.
	UX = Self::letters_to_token(b"UX"),
	
	/// TODO.
	UY = Self::letters_to_token(b"UY"),
	
	/// TODO.
	UZ = Self::letters_to_token(b"UZ"),
	
	/// TODO.
	VA = Self::letters_to_token(b"VA"),
	
	/// TODO.
	VB = Self::letters_to_token(b"VB"),
	
	/// TODO.
	VC = Self::letters_to_token(b"VC"),
	
	/// TODO.
	VD = Self::letters_to_token(b"VD"),
	
	/// TODO.
	VE = Self::letters_to_token(b"VE"),
	
	/// TODO.
	VF = Self::letters_to_token(b"VF"),
	
	/// TODO.
	VG = Self::letters_to_token(b"VG"),
	
	/// TODO.
	VH = Self::letters_to_token(b"VH"),
	
	/// TODO.
	VI = Self::letters_to_token(b"VI"),
	
	/// TODO.
	VJ = Self::letters_to_token(b"VJ"),
	
	/// TODO.
	VK = Self::letters_to_token(b"VK"),
	
	/// TODO.
	VL = Self::letters_to_token(b"VL"),
	
	/// TODO.
	VM = Self::letters_to_token(b"VM"),
	
	/// TODO.
	VN = Self::letters_to_token(b"VN"),
	
	/// TODO.
	VO = Self::letters_to_token(b"VO"),
	
	/// TODO.
	VP = Self::letters_to_token(b"VP"),
	
	/// TODO.
	VQ = Self::letters_to_token(b"VQ"),
	
	/// TODO.
	VR = Self::letters_to_token(b"VR"),
	
	/// TODO.
	VS = Self::letters_to_token(b"VS"),
	
	/// TODO.
	VT = Self::letters_to_token(b"VT"),
	
	/// TODO.
	VU = Self::letters_to_token(b"VU"),
	
	/// TODO.
	VV = Self::letters_to_token(b"VV"),
	
	/// TODO.
	VW = Self::letters_to_token(b"VW"),
	
	/// TODO.
	VX = Self::letters_to_token(b"VX"),
	
	/// TODO.
	VY = Self::letters_to_token(b"VY"),
	
	/// TODO.
	VZ = Self::letters_to_token(b"VZ"),
	
	/// TODO.
	WA = Self::letters_to_token(b"WA"),
	
	/// TODO.
	WB = Self::letters_to_token(b"WB"),
	
	/// TODO.
	WC = Self::letters_to_token(b"WC"),
	
	/// TODO.
	WD = Self::letters_to_token(b"WD"),
	
	/// TODO.
	WE = Self::letters_to_token(b"WE"),
	
	/// TODO.
	WF = Self::letters_to_token(b"WF"),
	
	/// TODO.
	WG = Self::letters_to_token(b"WG"),
	
	/// TODO.
	WH = Self::letters_to_token(b"WH"),
	
	/// TODO.
	WI = Self::letters_to_token(b"WI"),
	
	/// TODO.
	WJ = Self::letters_to_token(b"WJ"),
	
	/// TODO.
	WK = Self::letters_to_token(b"WK"),
	
	/// TODO.
	WL = Self::letters_to_token(b"WL"),
	
	/// TODO.
	WM = Self::letters_to_token(b"WM"),
	
	/// TODO.
	WN = Self::letters_to_token(b"WN"),
	
	/// Unassigned for use by World Intellectual Property Organisation Standard ST.3.
	///
	///  	World Intellectual Property Organization (WIPO).
	WO = Self::letters_to_token(b"WO"),
	
	/// TODO.
	WP = Self::letters_to_token(b"WP"),
	
	/// TODO.
	WQ = Self::letters_to_token(b"WQ"),
	
	/// TODO.
	WR = Self::letters_to_token(b"WR"),
	
	/// TODO.
	WS = Self::letters_to_token(b"WS"),
	
	/// TODO.
	WT = Self::letters_to_token(b"WT"),
	
	/// TODO.
	WU = Self::letters_to_token(b"WU"),
	
	/// TODO.
	WV = Self::letters_to_token(b"WV"),
	
	/// TODO.
	WW = Self::letters_to_token(b"WW"),
	
	/// TODO.
	WX = Self::letters_to_token(b"WX"),
	
	/// TODO.
	WY = Self::letters_to_token(b"WY"),
	
	/// TODO.
	WZ = Self::letters_to_token(b"WZ"),
	
	/// User assigned.
	///
	/// Used by Switzerland as a country code for the Canary Islands although `IC` is already reserved for that purpose.
	XA = Self::letters_to_token(b"XA"),

	/// User assigned.
	XB = Self::letters_to_token(b"XB"),

	/// User assigned.
	XC = Self::letters_to_token(b"XC"),

	/// User assigned.
	XD = Self::letters_to_token(b"XD"),

	/// User assigned.
	XE = Self::letters_to_token(b"XE"),

	/// User assigned.
	XF = Self::letters_to_token(b"XF"),

	/// User assigned.
	XG = Self::letters_to_token(b"XG"),

	/// User assigned.
	XH = Self::letters_to_token(b"XH"),

	/// User assigned.
	///
	/// Used by the UK Government as an EORI number country code prefix for Northern Ireland.
	/// Use by the European Union for Value Added Tax (VAT) reports with trade with Northern Ireland.
	XI = Self::letters_to_token(b"XI"),

	/// User assigned.
	XJ = Self::letters_to_token(b"XJ"),

	/// User assigned.
	///
	/// Used by the European Commission, IMF, SWIFT, the Unicode Common Locale Data Repository (CLDR) and other organizations as a temporary country code for Kosovo.
	XK = Self::letters_to_token(b"XK"),

	/// User assigned.
	XL = Self::letters_to_token(b"XL"),
		
	/// User assigned.
	XM = Self::letters_to_token(b"XM"),
	
	/// User assigned.
	///
	/// Used by the World Intellectual Property Organization (WIPO) for the Nordic Patent Institute, an international organization common to Denmark, Iceland, Norway and Sweden.
	XN = Self::letters_to_token(b"XN"),
	
	/// User assigned.
	XO = Self::letters_to_token(b"XO"),
	
	/// User assigned.
	XP = Self::letters_to_token(b"XP"),
	
	/// User assigned.
	XQ = Self::letters_to_token(b"XQ"),
	
	/// User assigned.
	XR = Self::letters_to_token(b"XR"),
	
	/// User assigned.
	XS = Self::letters_to_token(b"XS"),
	
	/// User assigned.
	XT = Self::letters_to_token(b"XT"),
	
	/// User assigned.
	///
	/// Used by the World Intellectual Property Organization (WIPO) as an indicator for unknown states, other entities or organizations.
	XU = Self::letters_to_token(b"XU"),
	
	/// User assigned.
	///
	/// Used by the World Intellectual Property Organization (WIPO) as an indicator for the Visegrad Patent Institute.
	XV = Self::letters_to_token(b"XV"),
	
	/// User assigned.
	XW = Self::letters_to_token(b"XW"),
	
	/// User assigned.
	///
	/// Used by the World Intellectual Property Organization (WIPO) as an indicator for unknown states, other entities or organizations.
	XX = Self::letters_to_token(b"XX"),
	
	/// User assigned.
	XY = Self::letters_to_token(b"XY"),
	
	/// User assigned.
	///
	/// Used by UN/LOCODE to represent installations in international waters.
	XZ = Self::letters_to_token(b"XZ"),
	
	/// TODO.
	YA = Self::letters_to_token(b"YA"),
	
	/// TODO.
	YB = Self::letters_to_token(b"YB"),
	
	/// TODO.
	YC = Self::letters_to_token(b"YC"),
	
	/// TODO.
	YD = Self::letters_to_token(b"YD"),
	
	/// TODO.
	YE = Self::letters_to_token(b"YE"),
	
	/// TODO.
	YF = Self::letters_to_token(b"YF"),
	
	/// TODO.
	YG = Self::letters_to_token(b"YG"),
	
	/// TODO.
	YH = Self::letters_to_token(b"YH"),
	
	/// TODO.
	YI = Self::letters_to_token(b"YI"),
	
	/// TODO.
	YJ = Self::letters_to_token(b"YJ"),
	
	/// TODO.
	YK = Self::letters_to_token(b"YK"),
	
	/// TODO.
	YL = Self::letters_to_token(b"YL"),
	
	/// TODO.
	YM = Self::letters_to_token(b"YM"),
	
	/// TODO.
	YN = Self::letters_to_token(b"YN"),
	
	/// TODO.
	YO = Self::letters_to_token(b"YO"),
	
	/// TODO.
	YP = Self::letters_to_token(b"YP"),
	
	/// TODO.
	YQ = Self::letters_to_token(b"YQ"),
	
	/// TODO.
	YR = Self::letters_to_token(b"YR"),
	
	/// TODO.
	YS = Self::letters_to_token(b"YS"),
	
	/// TODO.
	YT = Self::letters_to_token(b"YT"),
	
	/// TODO.
	YU = Self::letters_to_token(b"YU"),
	
	/// TODO.
	YV = Self::letters_to_token(b"YV"),
	
	/// TODO.
	YW = Self::letters_to_token(b"YW"),
	
	/// TODO.
	YX = Self::letters_to_token(b"YX"),
	
	/// TODO.
	YY = Self::letters_to_token(b"YY"),
	
	/// TODO.
	YZ = Self::letters_to_token(b"YZ"),
	
	/// TODO.
	ZA = Self::letters_to_token(b"ZA"),
	
	/// TODO.
	ZB = Self::letters_to_token(b"ZB"),
	
	/// TODO.
	ZC = Self::letters_to_token(b"ZC"),
	
	/// TODO.
	ZD = Self::letters_to_token(b"ZD"),
	
	/// TODO.
	ZE = Self::letters_to_token(b"ZE"),
	
	/// TODO.
	ZF = Self::letters_to_token(b"ZF"),
	
	/// TODO.
	ZG = Self::letters_to_token(b"ZG"),
	
	/// TODO.
	ZH = Self::letters_to_token(b"ZH"),
	
	/// TODO.
	ZI = Self::letters_to_token(b"ZI"),
	
	/// TODO.
	ZJ = Self::letters_to_token(b"ZJ"),
	
	/// TODO.
	ZK = Self::letters_to_token(b"ZK"),
	
	/// TODO.
	ZL = Self::letters_to_token(b"ZL"),
	
	/// TODO.
	ZM = Self::letters_to_token(b"ZM"),
	
	/// TODO.
	ZN = Self::letters_to_token(b"ZN"),
	
	/// TODO.
	ZO = Self::letters_to_token(b"ZO"),
	
	/// TODO.
	ZP = Self::letters_to_token(b"ZP"),
	
	/// TODO.
	ZQ = Self::letters_to_token(b"ZQ"),
	
	/// TODO.
	ZR = Self::letters_to_token(b"ZR"),
	
	/// TODO.
	ZS = Self::letters_to_token(b"ZS"),
	
	/// TODO.
	ZT = Self::letters_to_token(b"ZT"),
	
	/// TODO.
	ZU = Self::letters_to_token(b"ZU"),
	
	/// TODO.
	ZV = Self::letters_to_token(b"ZV"),
	
	/// TODO.
	ZW = Self::letters_to_token(b"ZW"),
	
	/// TODO.
	ZX = Self::letters_to_token(b"ZX"),
	
	/// TODO.
	ZY = Self::letters_to_token(b"ZY"),
	
	/// User assigned.
	///
	/// Used by the International Standard Recording Code (ISRC)for some registrants.
	/// Used by the Unicode Common Locale Data Repository (CLDR) to represent "Unknown or Invalid Territory".
	ZZ = Self::letters_to_token(b"ZZ"),
}

impl Iso3166Dash1Alpha2CountryCode
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn unassigned_for_use_by_world_intellectual_property_organisation_standard_st_3(self) -> bool
	{
		match self
		{
			AP => true,
			
			BX => true,
			
			EF => true,
			
			EM => true,
			
			EP => true,
			
			EV => true,
			
			GC => true,
			
			IB => true,
			
			OA => true,
			
			WO => true,
			
			_ => false,
		}
	}
	
	/// Returns if exceptionally reserved and if has been deleted.
	#[inline(always)]
	pub const fn is_exceptionally_reserved(self) -> (bool, bool)
	{
		match self
		{
			AC => (true, false),
			
			CP => (true, false),
			
			CQ => (true, false),
			
			DG => (true, false),
			
			EA => (true, false),
			
			EU => (true, false),
			
			EZ => (true, false),
			
			FX => (true, true),
			
			IC => (true, false),
			
			SU => (true, true),
			
			TA => (true, false),
			
			UK => (true, false),
			
			UN => (true, false),
			
			_ => (false, false)
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn transnational(self, include_SU: bool, include_XZ: bool) -> bool
	{
		match self
		{
			EU => true,
			
			EZ => true,
			
			SU => include_SU,
			
			UN => true,
			
			XZ => include_XZ,
			
			_ => false,
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_escape_code(self) -> bool
	{
		(self as u16) == OO as u16
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn is_user_assigned(self) -> bool
	{
		match self
		{
			AA => true,
			
			QM => true,
			
			QN => true,
			
			QO => true,
			
			QP => true,
			
			QQ => true,
			
			QR => true,
			
			QS => true,
			
			QT => true,
			
			QU => true,
			
			QV => true,
			
			QW => true,
			
			QX => true,
			
			QY => true,
			
			QZ => true,
			
			XA => true,
			
			XB => true,
			
			XC => true,
			
			XD => true,
			
			XE => true,
			
			XF => true,
			
			XG => true,
			
			XH => true,
			
			XI => true,
			
			XJ => true,
			
			XK => true,
			
			XL => true,
			
			XM => true,
			
			XN => true,
			
			XO => true,
			
			XP => true,
			
			XQ => true,
			
			XR => true,
			
			XS => true,
			
			XT => true,
			
			XU => true,
			
			XV => true,
			
			XW => true,
			
			XX => true,
			
			XY => true,
			
			XZ => true,
			
			ZZ => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	const fn letters_to_token(letters: &[u8; 2]) -> u16
	{
		Iso3166Dash1AlphaCountryCode::letter_to_number_unchecked::<0, _>(letters) + Iso3166Dash1AlphaCountryCode::letter_to_number_unchecked::<1, _>(letters)
	}
}
