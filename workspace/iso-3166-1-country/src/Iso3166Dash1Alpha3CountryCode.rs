// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An ISO 3166-1 Alpha-3 country code.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum Iso3166Dash1Alpha3CountryCode
{
	/// User assigned.
	AAA = Self::letters_to_token(b"AAA"),
	
	/// User assigned.
	AAB = Self::letters_to_token(b"AAB"),
	
	/// User assigned.
	AAC = Self::letters_to_token(b"AAC"),
	
	/// User assigned.
	AAD = Self::letters_to_token(b"AAD"),
	
	/// User assigned.
	AAE = Self::letters_to_token(b"AAE"),
	
	/// User assigned.
	AAF = Self::letters_to_token(b"AAF"),
	
	/// User assigned.
	AAG = Self::letters_to_token(b"AAG"),
	
	/// User assigned.
	AAH = Self::letters_to_token(b"AAH"),
	
	/// User assigned.
	AAI = Self::letters_to_token(b"AAI"),
	
	/// User assigned.
	AAJ = Self::letters_to_token(b"AAJ"),
	
	/// User assigned.
	AAK = Self::letters_to_token(b"AAK"),
	
	/// User assigned.
	AAL = Self::letters_to_token(b"AAL"),
	
	/// User assigned.
	AAM = Self::letters_to_token(b"AAM"),
	
	/// User assigned.
	AAN = Self::letters_to_token(b"AAN"),
	
	/// User assigned.
	AAO = Self::letters_to_token(b"AAO"),
	
	/// User assigned.
	AAP = Self::letters_to_token(b"AAP"),
	
	/// User assigned.
	AAQ = Self::letters_to_token(b"AAQ"),
	
	/// User assigned.
	AAR = Self::letters_to_token(b"AAR"),
	
	/// User assigned.
	AAS = Self::letters_to_token(b"AAS"),
	
	/// User assigned.
	AAT = Self::letters_to_token(b"AAT"),
	
	/// User assigned.
	AAU = Self::letters_to_token(b"AAU"),
	
	/// User assigned.
	AAV = Self::letters_to_token(b"AAV"),
	
	/// User assigned.
	AAW = Self::letters_to_token(b"AAW"),
	
	/// User assigned.
	AAX = Self::letters_to_token(b"AAX"),
	
	/// User assigned.
	AAY = Self::letters_to_token(b"AAY"),
	
	/// User assigned.
	AAZ = Self::letters_to_token(b"AAZ"),
	
	/// TODO.
	ABA = Self::letters_to_token(b"ABA"),
	
	/// Unofficial
	///
	/// Asia (NATO STANAG 1059 INT).
	ABB = Self::letters_to_token(b"ABB"),
	
	/// TODO.
	ABC = Self::letters_to_token(b"ABC"),
	
	/// TODO.
	ABD = Self::letters_to_token(b"ABD"),
	
	/// TODO.
	ABE = Self::letters_to_token(b"ABE"),
	
	/// TODO.
	ABF = Self::letters_to_token(b"ABF"),
	
	/// TODO.
	ABG = Self::letters_to_token(b"ABG"),
	
	/// TODO.
	ABH = Self::letters_to_token(b"ABH"),
	
	/// TODO.
	ABI = Self::letters_to_token(b"ABI"),
	
	/// TODO.
	ABJ = Self::letters_to_token(b"ABJ"),
	
	/// TODO.
	ABK = Self::letters_to_token(b"ABK"),
	
	/// TODO.
	ABL = Self::letters_to_token(b"ABL"),
	
	/// TODO.
	ABM = Self::letters_to_token(b"ABM"),
	
	/// TODO.
	ABN = Self::letters_to_token(b"ABN"),
	
	/// TODO.
	ABO = Self::letters_to_token(b"ABO"),
	
	/// TODO.
	ABP = Self::letters_to_token(b"ABP"),
	
	/// TODO.
	ABQ = Self::letters_to_token(b"ABQ"),
	
	/// TODO.
	ABR = Self::letters_to_token(b"ABR"),
	
	/// TODO.
	ABS = Self::letters_to_token(b"ABS"),
	
	/// TODO.
	ABT = Self::letters_to_token(b"ABT"),
	
	/// TODO.
	ABU = Self::letters_to_token(b"ABU"),
	
	/// TODO.
	ABV = Self::letters_to_token(b"ABV"),
	
	/// TODO.
	ABW = Self::letters_to_token(b"ABW"),
	
	/// TODO.
	ABX = Self::letters_to_token(b"ABX"),
	
	/// TODO.
	ABY = Self::letters_to_token(b"ABY"),
	
	/// TODO.
	ABZ = Self::letters_to_token(b"ABZ"),
	
	/// TODO.
	ACA = Self::letters_to_token(b"ACA"),
	
	/// TODO.
	ACB = Self::letters_to_token(b"ACB"),
	
	/// TODO.
	ACC = Self::letters_to_token(b"ACC"),
	
	/// TODO.
	ACD = Self::letters_to_token(b"ACD"),
	
	/// TODO.
	ACE = Self::letters_to_token(b"ACE"),
	
	/// TODO.
	ACF = Self::letters_to_token(b"ACF"),
	
	/// TODO.
	ACG = Self::letters_to_token(b"ACG"),
	
	/// TODO.
	ACH = Self::letters_to_token(b"ACH"),
	
	/// TODO.
	ACI = Self::letters_to_token(b"ACI"),
	
	/// TODO.
	ACJ = Self::letters_to_token(b"ACJ"),
	
	/// TODO.
	ACK = Self::letters_to_token(b"ACK"),
	
	/// TODO.
	ACL = Self::letters_to_token(b"ACL"),
	
	/// TODO.
	ACM = Self::letters_to_token(b"ACM"),
	
	/// TODO.
	ACN = Self::letters_to_token(b"ACN"),
	
	/// TODO.
	ACO = Self::letters_to_token(b"ACO"),
	
	/// TODO.
	ACP = Self::letters_to_token(b"ACP"),
	
	/// TODO.
	ACQ = Self::letters_to_token(b"ACQ"),
	
	/// TODO.
	ACR = Self::letters_to_token(b"ACR"),
	
	/// TODO.
	ACS = Self::letters_to_token(b"ACS"),
	
	/// TODO.
	ACT = Self::letters_to_token(b"ACT"),
	
	/// TODO.
	ACU = Self::letters_to_token(b"ACU"),
	
	/// TODO.
	ACV = Self::letters_to_token(b"ACV"),
	
	/// TODO.
	ACW = Self::letters_to_token(b"ACW"),
	
	/// TODO.
	ACX = Self::letters_to_token(b"ACX"),
	
	/// TODO.
	ACY = Self::letters_to_token(b"ACY"),
	
	/// TODO.
	ACZ = Self::letters_to_token(b"ACZ"),
	
	/// TODO.
	ADA = Self::letters_to_token(b"ADA"),
	
	/// TODO.
	ADB = Self::letters_to_token(b"ADB"),
	
	/// TODO.
	ADC = Self::letters_to_token(b"ADC"),
	
	/// TODO.
	ADD = Self::letters_to_token(b"ADD"),
	
	/// TODO.
	ADE = Self::letters_to_token(b"ADE"),
	
	/// TODO.
	ADF = Self::letters_to_token(b"ADF"),
	
	/// TODO.
	ADG = Self::letters_to_token(b"ADG"),
	
	/// TODO.
	ADH = Self::letters_to_token(b"ADH"),
	
	/// TODO.
	ADI = Self::letters_to_token(b"ADI"),
	
	/// TODO.
	ADJ = Self::letters_to_token(b"ADJ"),
	
	/// TODO.
	ADK = Self::letters_to_token(b"ADK"),
	
	/// TODO.
	ADL = Self::letters_to_token(b"ADL"),
	
	/// TODO.
	ADM = Self::letters_to_token(b"ADM"),
	
	/// TODO.
	ADN = Self::letters_to_token(b"ADN"),
	
	/// TODO.
	ADO = Self::letters_to_token(b"ADO"),
	
	/// TODO.
	ADP = Self::letters_to_token(b"ADP"),
	
	/// TODO.
	ADQ = Self::letters_to_token(b"ADQ"),
	
	/// TODO.
	ADR = Self::letters_to_token(b"ADR"),
	
	/// TODO.
	ADS = Self::letters_to_token(b"ADS"),
	
	/// TODO.
	ADT = Self::letters_to_token(b"ADT"),
	
	/// TODO.
	ADU = Self::letters_to_token(b"ADU"),
	
	/// TODO.
	ADV = Self::letters_to_token(b"ADV"),
	
	/// TODO.
	ADW = Self::letters_to_token(b"ADW"),
	
	/// TODO.
	ADX = Self::letters_to_token(b"ADX"),
	
	/// TODO.
	ADY = Self::letters_to_token(b"ADY"),
	
	/// TODO.
	ADZ = Self::letters_to_token(b"ADZ"),
	
	/// TODO.
	AEA = Self::letters_to_token(b"AEA"),
	
	/// TODO.
	AEB = Self::letters_to_token(b"AEB"),
	
	/// TODO.
	AEC = Self::letters_to_token(b"AEC"),
	
	/// TODO.
	AED = Self::letters_to_token(b"AED"),
	
	/// TODO.
	AEE = Self::letters_to_token(b"AEE"),
	
	/// TODO.
	AEF = Self::letters_to_token(b"AEF"),
	
	/// TODO.
	AEG = Self::letters_to_token(b"AEG"),
	
	/// TODO.
	AEH = Self::letters_to_token(b"AEH"),
	
	/// TODO.
	AEI = Self::letters_to_token(b"AEI"),
	
	/// TODO.
	AEJ = Self::letters_to_token(b"AEJ"),
	
	/// TODO.
	AEK = Self::letters_to_token(b"AEK"),
	
	/// TODO.
	AEL = Self::letters_to_token(b"AEL"),
	
	/// TODO.
	AEM = Self::letters_to_token(b"AEM"),
	
	/// TODO.
	AEN = Self::letters_to_token(b"AEN"),
	
	/// TODO.
	AEO = Self::letters_to_token(b"AEO"),
	
	/// TODO.
	AEP = Self::letters_to_token(b"AEP"),
	
	/// TODO.
	AEQ = Self::letters_to_token(b"AEQ"),
	
	/// TODO.
	AER = Self::letters_to_token(b"AER"),
	
	/// TODO.
	AES = Self::letters_to_token(b"AES"),
	
	/// TODO.
	AET = Self::letters_to_token(b"AET"),
	
	/// TODO.
	AEU = Self::letters_to_token(b"AEU"),
	
	/// TODO.
	AEV = Self::letters_to_token(b"AEV"),
	
	/// TODO.
	AEW = Self::letters_to_token(b"AEW"),
	
	/// TODO.
	AEX = Self::letters_to_token(b"AEX"),
	
	/// TODO.
	AEY = Self::letters_to_token(b"AEY"),
	
	/// TODO.
	AEZ = Self::letters_to_token(b"AEZ"),
	
	/// TODO.
	AFA = Self::letters_to_token(b"AFA"),
	
	/// TODO.
	AFB = Self::letters_to_token(b"AFB"),
	
	/// TODO.
	AFC = Self::letters_to_token(b"AFC"),
	
	/// TODO.
	AFD = Self::letters_to_token(b"AFD"),
	
	/// TODO.
	AFE = Self::letters_to_token(b"AFE"),
	
	/// TODO.
	AFF = Self::letters_to_token(b"AFF"),
	
	/// Afghanistan.
	AFG = Self::letters_to_token(b"AFG"),
	
	/// TODO.
	AFH = Self::letters_to_token(b"AFH"),
	
	/// TODO.
	AFI = Self::letters_to_token(b"AFI"),
	
	/// TODO.
	AFJ = Self::letters_to_token(b"AFJ"),
	
	/// TODO.
	AFK = Self::letters_to_token(b"AFK"),
	
	/// TODO.
	AFL = Self::letters_to_token(b"AFL"),
	
	/// TODO.
	AFM = Self::letters_to_token(b"AFM"),
	
	/// TODO.
	AFN = Self::letters_to_token(b"AFN"),
	
	/// TODO.
	AFO = Self::letters_to_token(b"AFO"),
	
	/// TODO.
	AFP = Self::letters_to_token(b"AFP"),
	
	/// TODO.
	AFQ = Self::letters_to_token(b"AFQ"),
	
	/// TODO.
	AFR = Self::letters_to_token(b"AFR"),
	
	/// TODO.
	AFS = Self::letters_to_token(b"AFS"),
	
	/// TODO.
	AFT = Self::letters_to_token(b"AFT"),
	
	/// TODO.
	AFU = Self::letters_to_token(b"AFU"),
	
	/// TODO.
	AFV = Self::letters_to_token(b"AFV"),
	
	/// TODO.
	AFW = Self::letters_to_token(b"AFW"),
	
	/// TODO.
	AFX = Self::letters_to_token(b"AFX"),
	
	/// TODO.
	AFY = Self::letters_to_token(b"AFY"),
	
	/// TODO.
	AFZ = Self::letters_to_token(b"AFZ"),
	
	/// TODO.
	AGA = Self::letters_to_token(b"AGA"),
	
	/// TODO.
	AGB = Self::letters_to_token(b"AGB"),
	
	/// TODO.
	AGC = Self::letters_to_token(b"AGC"),
	
	/// TODO.
	AGD = Self::letters_to_token(b"AGD"),
	
	/// TODO.
	AGE = Self::letters_to_token(b"AGE"),
	
	/// TODO.
	AGF = Self::letters_to_token(b"AGF"),
	
	/// TODO.
	AGG = Self::letters_to_token(b"AGG"),
	
	/// TODO.
	AGH = Self::letters_to_token(b"AGH"),
	
	/// TODO.
	AGI = Self::letters_to_token(b"AGI"),
	
	/// TODO.
	AGJ = Self::letters_to_token(b"AGJ"),
	
	/// TODO.
	AGK = Self::letters_to_token(b"AGK"),
	
	/// TODO.
	AGL = Self::letters_to_token(b"AGL"),
	
	/// TODO.
	AGM = Self::letters_to_token(b"AGM"),
	
	/// TODO.
	AGN = Self::letters_to_token(b"AGN"),
	
	/// TODO.
	AGO = Self::letters_to_token(b"AGO"),
	
	/// TODO.
	AGP = Self::letters_to_token(b"AGP"),
	
	/// TODO.
	AGQ = Self::letters_to_token(b"AGQ"),
	
	/// TODO.
	AGR = Self::letters_to_token(b"AGR"),
	
	/// TODO.
	AGS = Self::letters_to_token(b"AGS"),
	
	/// TODO.
	AGT = Self::letters_to_token(b"AGT"),
	
	/// TODO.
	AGU = Self::letters_to_token(b"AGU"),
	
	/// TODO.
	AGV = Self::letters_to_token(b"AGV"),
	
	/// TODO.
	AGW = Self::letters_to_token(b"AGW"),
	
	/// TODO.
	AGX = Self::letters_to_token(b"AGX"),
	
	/// TODO.
	AGY = Self::letters_to_token(b"AGY"),
	
	/// TODO.
	AGZ = Self::letters_to_token(b"AGZ"),
	
	/// TODO.
	AHA = Self::letters_to_token(b"AHA"),
	
	/// TODO.
	AHB = Self::letters_to_token(b"AHB"),
	
	/// TODO.
	AHC = Self::letters_to_token(b"AHC"),
	
	/// TODO.
	AHD = Self::letters_to_token(b"AHD"),
	
	/// TODO.
	AHE = Self::letters_to_token(b"AHE"),
	
	/// TODO.
	AHF = Self::letters_to_token(b"AHF"),
	
	/// TODO.
	AHG = Self::letters_to_token(b"AHG"),
	
	/// TODO.
	AHH = Self::letters_to_token(b"AHH"),
	
	/// TODO.
	AHI = Self::letters_to_token(b"AHI"),
	
	/// TODO.
	AHJ = Self::letters_to_token(b"AHJ"),
	
	/// TODO.
	AHK = Self::letters_to_token(b"AHK"),
	
	/// TODO.
	AHL = Self::letters_to_token(b"AHL"),
	
	/// TODO.
	AHM = Self::letters_to_token(b"AHM"),
	
	/// TODO.
	AHN = Self::letters_to_token(b"AHN"),
	
	/// TODO.
	AHO = Self::letters_to_token(b"AHO"),
	
	/// TODO.
	AHP = Self::letters_to_token(b"AHP"),
	
	/// TODO.
	AHQ = Self::letters_to_token(b"AHQ"),
	
	/// TODO.
	AHR = Self::letters_to_token(b"AHR"),
	
	/// TODO.
	AHS = Self::letters_to_token(b"AHS"),
	
	/// TODO.
	AHT = Self::letters_to_token(b"AHT"),
	
	/// TODO.
	AHU = Self::letters_to_token(b"AHU"),
	
	/// TODO.
	AHV = Self::letters_to_token(b"AHV"),
	
	/// TODO.
	AHW = Self::letters_to_token(b"AHW"),
	
	/// TODO.
	AHX = Self::letters_to_token(b"AHX"),
	
	/// TODO.
	AHY = Self::letters_to_token(b"AHY"),
	
	/// TODO.
	AHZ = Self::letters_to_token(b"AHZ"),
	
	/// TODO.
	AIA = Self::letters_to_token(b"AIA"),
	
	/// TODO.
	AIB = Self::letters_to_token(b"AIB"),
	
	/// TODO.
	AIC = Self::letters_to_token(b"AIC"),
	
	/// TODO.
	AID = Self::letters_to_token(b"AID"),
	
	/// TODO.
	AIE = Self::letters_to_token(b"AIE"),
	
	/// TODO.
	AIF = Self::letters_to_token(b"AIF"),
	
	/// TODO.
	AIG = Self::letters_to_token(b"AIG"),
	
	/// TODO.
	AIH = Self::letters_to_token(b"AIH"),
	
	/// TODO.
	AII = Self::letters_to_token(b"AII"),
	
	/// TODO.
	AIJ = Self::letters_to_token(b"AIJ"),
	
	/// TODO.
	AIK = Self::letters_to_token(b"AIK"),
	
	/// TODO.
	AIL = Self::letters_to_token(b"AIL"),
	
	/// TODO.
	AIM = Self::letters_to_token(b"AIM"),
	
	/// TODO.
	AIN = Self::letters_to_token(b"AIN"),
	
	/// TODO.
	AIO = Self::letters_to_token(b"AIO"),
	
	/// TODO.
	AIP = Self::letters_to_token(b"AIP"),
	
	/// TODO.
	AIQ = Self::letters_to_token(b"AIQ"),
	
	/// TODO.
	AIR = Self::letters_to_token(b"AIR"),
	
	/// TODO.
	AIS = Self::letters_to_token(b"AIS"),
	
	/// TODO.
	AIT = Self::letters_to_token(b"AIT"),
	
	/// TODO.
	AIU = Self::letters_to_token(b"AIU"),
	
	/// TODO.
	AIV = Self::letters_to_token(b"AIV"),
	
	/// TODO.
	AIW = Self::letters_to_token(b"AIW"),
	
	/// TODO.
	AIX = Self::letters_to_token(b"AIX"),
	
	/// TODO.
	AIY = Self::letters_to_token(b"AIY"),
	
	/// TODO.
	AIZ = Self::letters_to_token(b"AIZ"),
	
	/// TODO.
	AJA = Self::letters_to_token(b"AJA"),
	
	/// TODO.
	AJB = Self::letters_to_token(b"AJB"),
	
	/// TODO.
	AJC = Self::letters_to_token(b"AJC"),
	
	/// TODO.
	AJD = Self::letters_to_token(b"AJD"),
	
	/// TODO.
	AJE = Self::letters_to_token(b"AJE"),
	
	/// TODO.
	AJF = Self::letters_to_token(b"AJF"),
	
	/// TODO.
	AJG = Self::letters_to_token(b"AJG"),
	
	/// TODO.
	AJH = Self::letters_to_token(b"AJH"),
	
	/// TODO.
	AJI = Self::letters_to_token(b"AJI"),
	
	/// TODO.
	AJJ = Self::letters_to_token(b"AJJ"),
	
	/// TODO.
	AJK = Self::letters_to_token(b"AJK"),
	
	/// TODO.
	AJL = Self::letters_to_token(b"AJL"),
	
	/// TODO.
	AJM = Self::letters_to_token(b"AJM"),
	
	/// TODO.
	AJN = Self::letters_to_token(b"AJN"),
	
	/// TODO.
	AJO = Self::letters_to_token(b"AJO"),
	
	/// TODO.
	AJP = Self::letters_to_token(b"AJP"),
	
	/// TODO.
	AJQ = Self::letters_to_token(b"AJQ"),
	
	/// TODO.
	AJR = Self::letters_to_token(b"AJR"),
	
	/// TODO.
	AJS = Self::letters_to_token(b"AJS"),
	
	/// TODO.
	AJT = Self::letters_to_token(b"AJT"),
	
	/// TODO.
	AJU = Self::letters_to_token(b"AJU"),
	
	/// TODO.
	AJV = Self::letters_to_token(b"AJV"),
	
	/// TODO.
	AJW = Self::letters_to_token(b"AJW"),
	
	/// TODO.
	AJX = Self::letters_to_token(b"AJX"),
	
	/// TODO.
	AJY = Self::letters_to_token(b"AJY"),
	
	/// TODO.
	AJZ = Self::letters_to_token(b"AJZ"),
	
	/// TODO.
	AKA = Self::letters_to_token(b"AKA"),
	
	/// TODO.
	AKB = Self::letters_to_token(b"AKB"),
	
	/// TODO.
	AKC = Self::letters_to_token(b"AKC"),
	
	/// TODO.
	AKD = Self::letters_to_token(b"AKD"),
	
	/// TODO.
	AKE = Self::letters_to_token(b"AKE"),
	
	/// TODO.
	AKF = Self::letters_to_token(b"AKF"),
	
	/// TODO.
	AKG = Self::letters_to_token(b"AKG"),
	
	/// TODO.
	AKH = Self::letters_to_token(b"AKH"),
	
	/// TODO.
	AKI = Self::letters_to_token(b"AKI"),
	
	/// TODO.
	AKJ = Self::letters_to_token(b"AKJ"),
	
	/// TODO.
	AKK = Self::letters_to_token(b"AKK"),
	
	/// TODO.
	AKL = Self::letters_to_token(b"AKL"),
	
	/// TODO.
	AKM = Self::letters_to_token(b"AKM"),
	
	/// TODO.
	AKN = Self::letters_to_token(b"AKN"),
	
	/// TODO.
	AKO = Self::letters_to_token(b"AKO"),
	
	/// TODO.
	AKP = Self::letters_to_token(b"AKP"),
	
	/// TODO.
	AKQ = Self::letters_to_token(b"AKQ"),
	
	/// TODO.
	AKR = Self::letters_to_token(b"AKR"),
	
	/// TODO.
	AKS = Self::letters_to_token(b"AKS"),
	
	/// TODO.
	AKT = Self::letters_to_token(b"AKT"),
	
	/// TODO.
	AKU = Self::letters_to_token(b"AKU"),
	
	/// TODO.
	AKV = Self::letters_to_token(b"AKV"),
	
	/// TODO.
	AKW = Self::letters_to_token(b"AKW"),
	
	/// TODO.
	AKX = Self::letters_to_token(b"AKX"),
	
	/// TODO.
	AKY = Self::letters_to_token(b"AKY"),
	
	/// TODO.
	AKZ = Self::letters_to_token(b"AKZ"),
	
	/// TODO.
	ALA = Self::letters_to_token(b"ALA"),
	
	/// TODO.
	ALB = Self::letters_to_token(b"ALB"),
	
	/// TODO.
	ALC = Self::letters_to_token(b"ALC"),
	
	/// TODO.
	ALD = Self::letters_to_token(b"ALD"),
	
	/// TODO.
	ALE = Self::letters_to_token(b"ALE"),
	
	/// TODO.
	ALF = Self::letters_to_token(b"ALF"),
	
	/// TODO.
	ALG = Self::letters_to_token(b"ALG"),
	
	/// TODO.
	ALH = Self::letters_to_token(b"ALH"),
	
	/// TODO.
	ALI = Self::letters_to_token(b"ALI"),
	
	/// TODO.
	ALJ = Self::letters_to_token(b"ALJ"),
	
	/// TODO.
	ALK = Self::letters_to_token(b"ALK"),
	
	/// TODO.
	ALL = Self::letters_to_token(b"ALL"),
	
	/// TODO.
	ALM = Self::letters_to_token(b"ALM"),
	
	/// TODO.
	ALN = Self::letters_to_token(b"ALN"),
	
	/// TODO.
	ALO = Self::letters_to_token(b"ALO"),
	
	/// TODO.
	ALP = Self::letters_to_token(b"ALP"),
	
	/// TODO.
	ALQ = Self::letters_to_token(b"ALQ"),
	
	/// TODO.
	ALR = Self::letters_to_token(b"ALR"),
	
	/// TODO.
	ALS = Self::letters_to_token(b"ALS"),
	
	/// TODO.
	ALT = Self::letters_to_token(b"ALT"),
	
	/// TODO.
	ALU = Self::letters_to_token(b"ALU"),
	
	/// TODO.
	ALV = Self::letters_to_token(b"ALV"),
	
	/// TODO.
	ALW = Self::letters_to_token(b"ALW"),
	
	/// TODO.
	ALX = Self::letters_to_token(b"ALX"),
	
	/// TODO.
	ALY = Self::letters_to_token(b"ALY"),
	
	/// TODO.
	ALZ = Self::letters_to_token(b"ALZ"),
	
	/// TODO.
	AMA = Self::letters_to_token(b"AMA"),
	
	/// TODO.
	AMB = Self::letters_to_token(b"AMB"),
	
	/// TODO.
	AMC = Self::letters_to_token(b"AMC"),
	
	/// TODO.
	AMD = Self::letters_to_token(b"AMD"),
	
	/// TODO.
	AME = Self::letters_to_token(b"AME"),
	
	/// TODO.
	AMF = Self::letters_to_token(b"AMF"),
	
	/// TODO.
	AMG = Self::letters_to_token(b"AMG"),
	
	/// TODO.
	AMH = Self::letters_to_token(b"AMH"),
	
	/// TODO.
	AMI = Self::letters_to_token(b"AMI"),
	
	/// TODO.
	AMJ = Self::letters_to_token(b"AMJ"),
	
	/// TODO.
	AMK = Self::letters_to_token(b"AMK"),
	
	/// TODO.
	AML = Self::letters_to_token(b"AML"),
	
	/// TODO.
	AMM = Self::letters_to_token(b"AMM"),
	
	/// TODO.
	AMN = Self::letters_to_token(b"AMN"),
	
	/// TODO.
	AMO = Self::letters_to_token(b"AMO"),
	
	/// TODO.
	AMP = Self::letters_to_token(b"AMP"),
	
	/// TODO.
	AMQ = Self::letters_to_token(b"AMQ"),
	
	/// TODO.
	AMR = Self::letters_to_token(b"AMR"),
	
	/// TODO.
	AMS = Self::letters_to_token(b"AMS"),
	
	/// TODO.
	AMT = Self::letters_to_token(b"AMT"),
	
	/// TODO.
	AMU = Self::letters_to_token(b"AMU"),
	
	/// TODO.
	AMV = Self::letters_to_token(b"AMV"),
	
	/// TODO.
	AMW = Self::letters_to_token(b"AMW"),
	
	/// TODO.
	AMX = Self::letters_to_token(b"AMX"),
	
	/// TODO.
	AMY = Self::letters_to_token(b"AMY"),
	
	/// TODO.
	AMZ = Self::letters_to_token(b"AMZ"),
	
	/// TODO.
	ANA = Self::letters_to_token(b"ANA"),
	
	/// TODO.
	ANB = Self::letters_to_token(b"ANB"),
	
	/// TODO.
	ANC = Self::letters_to_token(b"ANC"),
	
	/// TODO.
	AND = Self::letters_to_token(b"AND"),
	
	/// TODO.
	ANE = Self::letters_to_token(b"ANE"),
	
	/// TODO.
	ANF = Self::letters_to_token(b"ANF"),
	
	/// TODO.
	ANG = Self::letters_to_token(b"ANG"),
	
	/// TODO.
	ANH = Self::letters_to_token(b"ANH"),
	
	/// TODO.
	ANI = Self::letters_to_token(b"ANI"),
	
	/// TODO.
	ANJ = Self::letters_to_token(b"ANJ"),
	
	/// TODO.
	ANK = Self::letters_to_token(b"ANK"),
	
	/// TODO.
	ANL = Self::letters_to_token(b"ANL"),
	
	/// TODO.
	ANM = Self::letters_to_token(b"ANM"),
	
	/// TODO.
	ANN = Self::letters_to_token(b"ANN"),
	
	/// TODO.
	ANO = Self::letters_to_token(b"ANO"),
	
	/// TODO.
	ANP = Self::letters_to_token(b"ANP"),
	
	/// TODO.
	ANQ = Self::letters_to_token(b"ANQ"),
	
	/// TODO.
	ANR = Self::letters_to_token(b"ANR"),
	
	/// TODO.
	ANS = Self::letters_to_token(b"ANS"),
	
	/// TODO.
	ANT = Self::letters_to_token(b"ANT"),
	
	/// TODO.
	ANU = Self::letters_to_token(b"ANU"),
	
	/// TODO.
	ANV = Self::letters_to_token(b"ANV"),
	
	/// TODO.
	ANW = Self::letters_to_token(b"ANW"),
	
	/// TODO.
	ANX = Self::letters_to_token(b"ANX"),
	
	/// TODO.
	ANY = Self::letters_to_token(b"ANY"),
	
	/// TODO.
	ANZ = Self::letters_to_token(b"ANZ"),
	
	/// TODO.
	AOA = Self::letters_to_token(b"AOA"),
	
	/// TODO.
	AOB = Self::letters_to_token(b"AOB"),
	
	/// TODO.
	AOC = Self::letters_to_token(b"AOC"),
	
	/// TODO.
	AOD = Self::letters_to_token(b"AOD"),
	
	/// TODO.
	AOE = Self::letters_to_token(b"AOE"),
	
	/// TODO.
	AOF = Self::letters_to_token(b"AOF"),
	
	/// TODO.
	AOG = Self::letters_to_token(b"AOG"),
	
	/// TODO.
	AOH = Self::letters_to_token(b"AOH"),
	
	/// TODO.
	AOI = Self::letters_to_token(b"AOI"),
	
	/// TODO.
	AOJ = Self::letters_to_token(b"AOJ"),
	
	/// TODO.
	AOK = Self::letters_to_token(b"AOK"),
	
	/// TODO.
	AOL = Self::letters_to_token(b"AOL"),
	
	/// TODO.
	AOM = Self::letters_to_token(b"AOM"),
	
	/// TODO.
	AON = Self::letters_to_token(b"AON"),
	
	/// TODO.
	AOO = Self::letters_to_token(b"AOO"),
	
	/// TODO.
	AOP = Self::letters_to_token(b"AOP"),
	
	/// TODO.
	AOQ = Self::letters_to_token(b"AOQ"),
	
	/// TODO.
	AOR = Self::letters_to_token(b"AOR"),
	
	/// TODO.
	AOS = Self::letters_to_token(b"AOS"),
	
	/// TODO.
	AOT = Self::letters_to_token(b"AOT"),
	
	/// TODO.
	AOU = Self::letters_to_token(b"AOU"),
	
	/// TODO.
	AOV = Self::letters_to_token(b"AOV"),
	
	/// TODO.
	AOW = Self::letters_to_token(b"AOW"),
	
	/// TODO.
	AOX = Self::letters_to_token(b"AOX"),
	
	/// TODO.
	AOY = Self::letters_to_token(b"AOY"),
	
	/// TODO.
	AOZ = Self::letters_to_token(b"AOZ"),
	
	/// TODO.
	APA = Self::letters_to_token(b"APA"),
	
	/// TODO.
	APB = Self::letters_to_token(b"APB"),
	
	/// TODO.
	APC = Self::letters_to_token(b"APC"),
	
	/// TODO.
	APD = Self::letters_to_token(b"APD"),
	
	/// TODO.
	APE = Self::letters_to_token(b"APE"),
	
	/// TODO.
	APF = Self::letters_to_token(b"APF"),
	
	/// TODO.
	APG = Self::letters_to_token(b"APG"),
	
	/// TODO.
	APH = Self::letters_to_token(b"APH"),
	
	/// TODO.
	API = Self::letters_to_token(b"API"),
	
	/// TODO.
	APJ = Self::letters_to_token(b"APJ"),
	
	/// TODO.
	APK = Self::letters_to_token(b"APK"),
	
	/// TODO.
	APL = Self::letters_to_token(b"APL"),
	
	/// TODO.
	APM = Self::letters_to_token(b"APM"),
	
	/// TODO.
	APN = Self::letters_to_token(b"APN"),
	
	/// TODO.
	APO = Self::letters_to_token(b"APO"),
	
	/// TODO.
	APP = Self::letters_to_token(b"APP"),
	
	/// TODO.
	APQ = Self::letters_to_token(b"APQ"),
	
	/// TODO.
	APR = Self::letters_to_token(b"APR"),
	
	/// TODO.
	APS = Self::letters_to_token(b"APS"),
	
	/// TODO.
	APT = Self::letters_to_token(b"APT"),
	
	/// TODO.
	APU = Self::letters_to_token(b"APU"),
	
	/// TODO.
	APV = Self::letters_to_token(b"APV"),
	
	/// TODO.
	APW = Self::letters_to_token(b"APW"),
	
	/// TODO.
	APX = Self::letters_to_token(b"APX"),
	
	/// TODO.
	APY = Self::letters_to_token(b"APY"),
	
	/// TODO.
	APZ = Self::letters_to_token(b"APZ"),
	
	/// TODO.
	AQA = Self::letters_to_token(b"AQA"),
	
	/// TODO.
	AQB = Self::letters_to_token(b"AQB"),
	
	/// TODO.
	AQC = Self::letters_to_token(b"AQC"),
	
	/// TODO.
	AQD = Self::letters_to_token(b"AQD"),
	
	/// TODO.
	AQE = Self::letters_to_token(b"AQE"),
	
	/// TODO.
	AQF = Self::letters_to_token(b"AQF"),
	
	/// TODO.
	AQG = Self::letters_to_token(b"AQG"),
	
	/// TODO.
	AQH = Self::letters_to_token(b"AQH"),
	
	/// TODO.
	AQI = Self::letters_to_token(b"AQI"),
	
	/// TODO.
	AQJ = Self::letters_to_token(b"AQJ"),
	
	/// TODO.
	AQK = Self::letters_to_token(b"AQK"),
	
	/// TODO.
	AQL = Self::letters_to_token(b"AQL"),
	
	/// TODO.
	AQM = Self::letters_to_token(b"AQM"),
	
	/// TODO.
	AQN = Self::letters_to_token(b"AQN"),
	
	/// TODO.
	AQO = Self::letters_to_token(b"AQO"),
	
	/// TODO.
	AQP = Self::letters_to_token(b"AQP"),
	
	/// TODO.
	AQQ = Self::letters_to_token(b"AQQ"),
	
	/// TODO.
	AQR = Self::letters_to_token(b"AQR"),
	
	/// TODO.
	AQS = Self::letters_to_token(b"AQS"),
	
	/// TODO.
	AQT = Self::letters_to_token(b"AQT"),
	
	/// TODO.
	AQU = Self::letters_to_token(b"AQU"),
	
	/// TODO.
	AQV = Self::letters_to_token(b"AQV"),
	
	/// TODO.
	AQW = Self::letters_to_token(b"AQW"),
	
	/// TODO.
	AQX = Self::letters_to_token(b"AQX"),
	
	/// TODO.
	AQY = Self::letters_to_token(b"AQY"),
	
	/// TODO.
	AQZ = Self::letters_to_token(b"AQZ"),
	
	/// TODO.
	ARA = Self::letters_to_token(b"ARA"),
	
	/// TODO.
	ARB = Self::letters_to_token(b"ARB"),
	
	/// TODO.
	ARC = Self::letters_to_token(b"ARC"),
	
	/// TODO.
	ARD = Self::letters_to_token(b"ARD"),
	
	/// TODO.
	ARE = Self::letters_to_token(b"ARE"),
	
	/// TODO.
	ARF = Self::letters_to_token(b"ARF"),
	
	/// TODO.
	ARG = Self::letters_to_token(b"ARG"),
	
	/// TODO.
	ARH = Self::letters_to_token(b"ARH"),
	
	/// TODO.
	ARI = Self::letters_to_token(b"ARI"),
	
	/// TODO.
	ARJ = Self::letters_to_token(b"ARJ"),
	
	/// TODO.
	ARK = Self::letters_to_token(b"ARK"),
	
	/// TODO.
	ARL = Self::letters_to_token(b"ARL"),
	
	/// TODO.
	ARM = Self::letters_to_token(b"ARM"),
	
	/// TODO.
	ARN = Self::letters_to_token(b"ARN"),
	
	/// TODO.
	ARO = Self::letters_to_token(b"ARO"),
	
	/// TODO.
	ARP = Self::letters_to_token(b"ARP"),
	
	/// TODO.
	ARQ = Self::letters_to_token(b"ARQ"),
	
	/// TODO.
	ARR = Self::letters_to_token(b"ARR"),
	
	/// TODO.
	ARS = Self::letters_to_token(b"ARS"),
	
	/// TODO.
	ART = Self::letters_to_token(b"ART"),
	
	/// TODO.
	ARU = Self::letters_to_token(b"ARU"),
	
	/// TODO.
	ARV = Self::letters_to_token(b"ARV"),
	
	/// TODO.
	ARW = Self::letters_to_token(b"ARW"),
	
	/// TODO.
	ARX = Self::letters_to_token(b"ARX"),
	
	/// TODO.
	ARY = Self::letters_to_token(b"ARY"),
	
	/// TODO.
	ARZ = Self::letters_to_token(b"ARZ"),
	
	/// TODO.
	ASA = Self::letters_to_token(b"ASA"),
	
	/// TODO.
	ASB = Self::letters_to_token(b"ASB"),
	
	/// Exceptional reservation.
	///
	/// Ascension Island.
	ASC = Self::letters_to_token(b"ASC"),
	
	/// TODO.
	ASD = Self::letters_to_token(b"ASD"),
	
	/// TODO.
	ASE = Self::letters_to_token(b"ASE"),
	
	/// TODO.
	ASF = Self::letters_to_token(b"ASF"),
	
	/// TODO.
	ASG = Self::letters_to_token(b"ASG"),
	
	/// TODO.
	ASH = Self::letters_to_token(b"ASH"),
	
	/// TODO.
	ASI = Self::letters_to_token(b"ASI"),
	
	/// TODO.
	ASJ = Self::letters_to_token(b"ASJ"),
	
	/// TODO.
	ASK = Self::letters_to_token(b"ASK"),
	
	/// TODO.
	ASL = Self::letters_to_token(b"ASL"),
	
	/// TODO.
	ASM = Self::letters_to_token(b"ASM"),
	
	/// TODO.
	ASN = Self::letters_to_token(b"ASN"),
	
	/// TODO.
	ASO = Self::letters_to_token(b"ASO"),
	
	/// TODO.
	ASP = Self::letters_to_token(b"ASP"),
	
	/// TODO.
	ASQ = Self::letters_to_token(b"ASQ"),
	
	/// TODO.
	ASR = Self::letters_to_token(b"ASR"),
	
	/// TODO.
	ASS = Self::letters_to_token(b"ASS"),
	
	/// TODO.
	AST = Self::letters_to_token(b"AST"),
	
	/// TODO.
	ASU = Self::letters_to_token(b"ASU"),
	
	/// TODO.
	ASV = Self::letters_to_token(b"ASV"),
	
	/// TODO.
	ASW = Self::letters_to_token(b"ASW"),
	
	/// TODO.
	ASX = Self::letters_to_token(b"ASX"),
	
	/// TODO.
	ASY = Self::letters_to_token(b"ASY"),
	
	/// TODO.
	ASZ = Self::letters_to_token(b"ASZ"),
	
	/// TODO.
	ATA = Self::letters_to_token(b"ATA"),
	
	/// TODO.
	ATB = Self::letters_to_token(b"ATB"),
	
	/// TODO.
	ATC = Self::letters_to_token(b"ATC"),
	
	/// TODO.
	ATD = Self::letters_to_token(b"ATD"),
	
	/// TODO.
	ATE = Self::letters_to_token(b"ATE"),
	
	/// TODO.
	ATF = Self::letters_to_token(b"ATF"),
	
	/// TODO.
	ATG = Self::letters_to_token(b"ATG"),
	
	/// TODO.
	ATH = Self::letters_to_token(b"ATH"),
	
	/// TODO.
	ATI = Self::letters_to_token(b"ATI"),
	
	/// TODO.
	ATJ = Self::letters_to_token(b"ATJ"),
	
	/// TODO.
	ATK = Self::letters_to_token(b"ATK"),
	
	/// TODO.
	ATL = Self::letters_to_token(b"ATL"),
	
	/// TODO.
	ATM = Self::letters_to_token(b"ATM"),
	
	/// TODO.
	ATN = Self::letters_to_token(b"ATN"),
	
	/// TODO.
	ATO = Self::letters_to_token(b"ATO"),
	
	/// TODO.
	ATP = Self::letters_to_token(b"ATP"),
	
	/// TODO.
	ATQ = Self::letters_to_token(b"ATQ"),
	
	/// TODO.
	ATR = Self::letters_to_token(b"ATR"),
	
	/// TODO.
	ATS = Self::letters_to_token(b"ATS"),
	
	/// TODO.
	ATT = Self::letters_to_token(b"ATT"),
	
	/// TODO.
	ATU = Self::letters_to_token(b"ATU"),
	
	/// TODO.
	ATV = Self::letters_to_token(b"ATV"),
	
	/// TODO.
	ATW = Self::letters_to_token(b"ATW"),
	
	/// TODO.
	ATX = Self::letters_to_token(b"ATX"),
	
	/// TODO.
	ATY = Self::letters_to_token(b"ATY"),
	
	/// TODO.
	ATZ = Self::letters_to_token(b"ATZ"),
	
	/// TODO.
	AUA = Self::letters_to_token(b"AUA"),
	
	/// TODO.
	AUB = Self::letters_to_token(b"AUB"),
	
	/// TODO.
	AUC = Self::letters_to_token(b"AUC"),
	
	/// TODO.
	AUD = Self::letters_to_token(b"AUD"),
	
	/// TODO.
	AUE = Self::letters_to_token(b"AUE"),
	
	/// TODO.
	AUF = Self::letters_to_token(b"AUF"),
	
	/// TODO.
	AUG = Self::letters_to_token(b"AUG"),
	
	/// TODO.
	AUH = Self::letters_to_token(b"AUH"),
	
	/// TODO.
	AUI = Self::letters_to_token(b"AUI"),
	
	/// TODO.
	AUJ = Self::letters_to_token(b"AUJ"),
	
	/// TODO.
	AUK = Self::letters_to_token(b"AUK"),
	
	/// TODO.
	AUL = Self::letters_to_token(b"AUL"),
	
	/// TODO.
	AUM = Self::letters_to_token(b"AUM"),
	
	/// TODO.
	AUN = Self::letters_to_token(b"AUN"),
	
	/// TODO.
	AUO = Self::letters_to_token(b"AUO"),
	
	/// TODO.
	AUP = Self::letters_to_token(b"AUP"),
	
	/// TODO.
	AUQ = Self::letters_to_token(b"AUQ"),
	
	/// TODO.
	AUR = Self::letters_to_token(b"AUR"),
	
	/// TODO.
	AUS = Self::letters_to_token(b"AUS"),
	
	/// TODO.
	AUT = Self::letters_to_token(b"AUT"),
	
	/// TODO.
	AUU = Self::letters_to_token(b"AUU"),
	
	/// TODO.
	AUV = Self::letters_to_token(b"AUV"),
	
	/// TODO.
	AUW = Self::letters_to_token(b"AUW"),
	
	/// TODO.
	AUX = Self::letters_to_token(b"AUX"),
	
	/// TODO.
	AUY = Self::letters_to_token(b"AUY"),
	
	/// TODO.
	AUZ = Self::letters_to_token(b"AUZ"),
	
	/// TODO.
	AVA = Self::letters_to_token(b"AVA"),
	
	/// TODO.
	AVB = Self::letters_to_token(b"AVB"),
	
	/// TODO.
	AVC = Self::letters_to_token(b"AVC"),
	
	/// TODO.
	AVD = Self::letters_to_token(b"AVD"),
	
	/// TODO.
	AVE = Self::letters_to_token(b"AVE"),
	
	/// TODO.
	AVF = Self::letters_to_token(b"AVF"),
	
	/// TODO.
	AVG = Self::letters_to_token(b"AVG"),
	
	/// TODO.
	AVH = Self::letters_to_token(b"AVH"),
	
	/// TODO.
	AVI = Self::letters_to_token(b"AVI"),
	
	/// TODO.
	AVJ = Self::letters_to_token(b"AVJ"),
	
	/// TODO.
	AVK = Self::letters_to_token(b"AVK"),
	
	/// TODO.
	AVL = Self::letters_to_token(b"AVL"),
	
	/// TODO.
	AVM = Self::letters_to_token(b"AVM"),
	
	/// TODO.
	AVN = Self::letters_to_token(b"AVN"),
	
	/// TODO.
	AVO = Self::letters_to_token(b"AVO"),
	
	/// TODO.
	AVP = Self::letters_to_token(b"AVP"),
	
	/// TODO.
	AVQ = Self::letters_to_token(b"AVQ"),
	
	/// TODO.
	AVR = Self::letters_to_token(b"AVR"),
	
	/// TODO.
	AVS = Self::letters_to_token(b"AVS"),
	
	/// TODO.
	AVT = Self::letters_to_token(b"AVT"),
	
	/// TODO.
	AVU = Self::letters_to_token(b"AVU"),
	
	/// TODO.
	AVV = Self::letters_to_token(b"AVV"),
	
	/// TODO.
	AVW = Self::letters_to_token(b"AVW"),
	
	/// TODO.
	AVX = Self::letters_to_token(b"AVX"),
	
	/// TODO.
	AVY = Self::letters_to_token(b"AVY"),
	
	/// TODO.
	AVZ = Self::letters_to_token(b"AVZ"),
	
	/// TODO.
	AWA = Self::letters_to_token(b"AWA"),
	
	/// TODO.
	AWB = Self::letters_to_token(b"AWB"),
	
	/// TODO.
	AWC = Self::letters_to_token(b"AWC"),
	
	/// TODO.
	AWD = Self::letters_to_token(b"AWD"),
	
	/// TODO.
	AWE = Self::letters_to_token(b"AWE"),
	
	/// TODO.
	AWF = Self::letters_to_token(b"AWF"),
	
	/// TODO.
	AWG = Self::letters_to_token(b"AWG"),
	
	/// TODO.
	AWH = Self::letters_to_token(b"AWH"),
	
	/// TODO.
	AWI = Self::letters_to_token(b"AWI"),
	
	/// TODO.
	AWJ = Self::letters_to_token(b"AWJ"),
	
	/// TODO.
	AWK = Self::letters_to_token(b"AWK"),
	
	/// TODO.
	AWL = Self::letters_to_token(b"AWL"),
	
	/// TODO.
	AWM = Self::letters_to_token(b"AWM"),
	
	/// TODO.
	AWN = Self::letters_to_token(b"AWN"),
	
	/// TODO.
	AWO = Self::letters_to_token(b"AWO"),
	
	/// TODO.
	AWP = Self::letters_to_token(b"AWP"),
	
	/// TODO.
	AWQ = Self::letters_to_token(b"AWQ"),
	
	/// TODO.
	AWR = Self::letters_to_token(b"AWR"),
	
	/// TODO.
	AWS = Self::letters_to_token(b"AWS"),
	
	/// TODO.
	AWT = Self::letters_to_token(b"AWT"),
	
	/// TODO.
	AWU = Self::letters_to_token(b"AWU"),
	
	/// TODO.
	AWV = Self::letters_to_token(b"AWV"),
	
	/// TODO.
	AWW = Self::letters_to_token(b"AWW"),
	
	/// TODO.
	AWX = Self::letters_to_token(b"AWX"),
	
	/// TODO.
	AWY = Self::letters_to_token(b"AWY"),
	
	/// TODO.
	AWZ = Self::letters_to_token(b"AWZ"),
	
	/// TODO.
	AXA = Self::letters_to_token(b"AXA"),
	
	/// TODO.
	AXB = Self::letters_to_token(b"AXB"),
	
	/// TODO.
	AXC = Self::letters_to_token(b"AXC"),
	
	/// TODO.
	AXD = Self::letters_to_token(b"AXD"),
	
	/// TODO.
	AXE = Self::letters_to_token(b"AXE"),
	
	/// TODO.
	AXF = Self::letters_to_token(b"AXF"),
	
	/// TODO.
	AXG = Self::letters_to_token(b"AXG"),
	
	/// TODO.
	AXH = Self::letters_to_token(b"AXH"),
	
	/// TODO.
	AXI = Self::letters_to_token(b"AXI"),
	
	/// TODO.
	AXJ = Self::letters_to_token(b"AXJ"),
	
	/// TODO.
	AXK = Self::letters_to_token(b"AXK"),
	
	/// TODO.
	AXL = Self::letters_to_token(b"AXL"),
	
	/// TODO.
	AXM = Self::letters_to_token(b"AXM"),
	
	/// TODO.
	AXN = Self::letters_to_token(b"AXN"),
	
	/// TODO.
	AXO = Self::letters_to_token(b"AXO"),
	
	/// TODO.
	AXP = Self::letters_to_token(b"AXP"),
	
	/// TODO.
	AXQ = Self::letters_to_token(b"AXQ"),
	
	/// TODO.
	AXR = Self::letters_to_token(b"AXR"),
	
	/// TODO.
	AXS = Self::letters_to_token(b"AXS"),
	
	/// TODO.
	AXT = Self::letters_to_token(b"AXT"),
	
	/// TODO.
	AXU = Self::letters_to_token(b"AXU"),
	
	/// TODO.
	AXV = Self::letters_to_token(b"AXV"),
	
	/// TODO.
	AXW = Self::letters_to_token(b"AXW"),
	
	/// TODO.
	AXX = Self::letters_to_token(b"AXX"),
	
	/// TODO.
	AXY = Self::letters_to_token(b"AXY"),
	
	/// TODO.
	AXZ = Self::letters_to_token(b"AXZ"),
	
	/// TODO.
	AYA = Self::letters_to_token(b"AYA"),
	
	/// TODO.
	AYB = Self::letters_to_token(b"AYB"),
	
	/// TODO.
	AYC = Self::letters_to_token(b"AYC"),
	
	/// TODO.
	AYD = Self::letters_to_token(b"AYD"),
	
	/// TODO.
	AYE = Self::letters_to_token(b"AYE"),
	
	/// TODO.
	AYF = Self::letters_to_token(b"AYF"),
	
	/// TODO.
	AYG = Self::letters_to_token(b"AYG"),
	
	/// TODO.
	AYH = Self::letters_to_token(b"AYH"),
	
	/// TODO.
	AYI = Self::letters_to_token(b"AYI"),
	
	/// TODO.
	AYJ = Self::letters_to_token(b"AYJ"),
	
	/// TODO.
	AYK = Self::letters_to_token(b"AYK"),
	
	/// TODO.
	AYL = Self::letters_to_token(b"AYL"),
	
	/// TODO.
	AYM = Self::letters_to_token(b"AYM"),
	
	/// TODO.
	AYN = Self::letters_to_token(b"AYN"),
	
	/// TODO.
	AYO = Self::letters_to_token(b"AYO"),
	
	/// TODO.
	AYP = Self::letters_to_token(b"AYP"),
	
	/// TODO.
	AYQ = Self::letters_to_token(b"AYQ"),
	
	/// TODO.
	AYR = Self::letters_to_token(b"AYR"),
	
	/// TODO.
	AYS = Self::letters_to_token(b"AYS"),
	
	/// TODO.
	AYT = Self::letters_to_token(b"AYT"),
	
	/// TODO.
	AYU = Self::letters_to_token(b"AYU"),
	
	/// TODO.
	AYV = Self::letters_to_token(b"AYV"),
	
	/// TODO.
	AYW = Self::letters_to_token(b"AYW"),
	
	/// TODO.
	AYX = Self::letters_to_token(b"AYX"),
	
	/// TODO.
	AYY = Self::letters_to_token(b"AYY"),
	
	/// TODO.
	AYZ = Self::letters_to_token(b"AYZ"),
	
	/// TODO.
	AZA = Self::letters_to_token(b"AZA"),
	
	/// TODO.
	AZB = Self::letters_to_token(b"AZB"),
	
	/// TODO.
	AZC = Self::letters_to_token(b"AZC"),
	
	/// TODO.
	AZD = Self::letters_to_token(b"AZD"),
	
	/// TODO.
	AZE = Self::letters_to_token(b"AZE"),
	
	/// TODO.
	AZF = Self::letters_to_token(b"AZF"),
	
	/// TODO.
	AZG = Self::letters_to_token(b"AZG"),
	
	/// TODO.
	AZH = Self::letters_to_token(b"AZH"),
	
	/// TODO.
	AZI = Self::letters_to_token(b"AZI"),
	
	/// TODO.
	AZJ = Self::letters_to_token(b"AZJ"),
	
	/// TODO.
	AZK = Self::letters_to_token(b"AZK"),
	
	/// TODO.
	AZL = Self::letters_to_token(b"AZL"),
	
	/// TODO.
	AZM = Self::letters_to_token(b"AZM"),
	
	/// TODO.
	AZN = Self::letters_to_token(b"AZN"),
	
	/// TODO.
	AZO = Self::letters_to_token(b"AZO"),
	
	/// TODO.
	AZP = Self::letters_to_token(b"AZP"),
	
	/// TODO.
	AZQ = Self::letters_to_token(b"AZQ"),
	
	/// TODO.
	AZR = Self::letters_to_token(b"AZR"),
	
	/// TODO.
	AZS = Self::letters_to_token(b"AZS"),
	
	/// TODO.
	AZT = Self::letters_to_token(b"AZT"),
	
	/// TODO.
	AZU = Self::letters_to_token(b"AZU"),
	
	/// TODO.
	AZV = Self::letters_to_token(b"AZV"),
	
	/// TODO.
	AZW = Self::letters_to_token(b"AZW"),
	
	/// TODO.
	AZX = Self::letters_to_token(b"AZX"),
	
	/// TODO.
	AZY = Self::letters_to_token(b"AZY"),
	
	/// TODO.
	AZZ = Self::letters_to_token(b"AZZ"),
	
	/// TODO.
	BAA = Self::letters_to_token(b"BAA"),
	
	/// TODO.
	BAB = Self::letters_to_token(b"BAB"),
	
	/// TODO.
	BAC = Self::letters_to_token(b"BAC"),
	
	/// TODO.
	BAD = Self::letters_to_token(b"BAD"),
	
	/// TODO.
	BAE = Self::letters_to_token(b"BAE"),
	
	/// TODO.
	BAF = Self::letters_to_token(b"BAF"),
	
	/// TODO.
	BAG = Self::letters_to_token(b"BAG"),
	
	/// TODO.
	BAH = Self::letters_to_token(b"BAH"),
	
	/// TODO.
	BAI = Self::letters_to_token(b"BAI"),
	
	/// TODO.
	BAJ = Self::letters_to_token(b"BAJ"),
	
	/// TODO.
	BAK = Self::letters_to_token(b"BAK"),
	
	/// TODO.
	BAL = Self::letters_to_token(b"BAL"),
	
	/// TODO.
	BAM = Self::letters_to_token(b"BAM"),
	
	/// TODO.
	BAN = Self::letters_to_token(b"BAN"),
	
	/// TODO.
	BAO = Self::letters_to_token(b"BAO"),
	
	/// TODO.
	BAP = Self::letters_to_token(b"BAP"),
	
	/// TODO.
	BAQ = Self::letters_to_token(b"BAQ"),
	
	/// TODO.
	BAR = Self::letters_to_token(b"BAR"),
	
	/// TODO.
	BAS = Self::letters_to_token(b"BAS"),
	
	/// TODO.
	BAT = Self::letters_to_token(b"BAT"),
	
	/// TODO.
	BAU = Self::letters_to_token(b"BAU"),
	
	/// TODO.
	BAV = Self::letters_to_token(b"BAV"),
	
	/// TODO.
	BAW = Self::letters_to_token(b"BAW"),
	
	/// TODO.
	BAX = Self::letters_to_token(b"BAX"),
	
	/// TODO.
	BAY = Self::letters_to_token(b"BAY"),
	
	/// TODO.
	BAZ = Self::letters_to_token(b"BAZ"),
	
	/// TODO.
	BBA = Self::letters_to_token(b"BBA"),
	
	/// TODO.
	BBB = Self::letters_to_token(b"BBB"),
	
	/// TODO.
	BBC = Self::letters_to_token(b"BBC"),
	
	/// TODO.
	BBD = Self::letters_to_token(b"BBD"),
	
	/// TODO.
	BBE = Self::letters_to_token(b"BBE"),
	
	/// TODO.
	BBF = Self::letters_to_token(b"BBF"),
	
	/// TODO.
	BBG = Self::letters_to_token(b"BBG"),
	
	/// TODO.
	BBH = Self::letters_to_token(b"BBH"),
	
	/// TODO.
	BBI = Self::letters_to_token(b"BBI"),
	
	/// TODO.
	BBJ = Self::letters_to_token(b"BBJ"),
	
	/// TODO.
	BBK = Self::letters_to_token(b"BBK"),
	
	/// TODO.
	BBL = Self::letters_to_token(b"BBL"),
	
	/// TODO.
	BBM = Self::letters_to_token(b"BBM"),
	
	/// TODO.
	BBN = Self::letters_to_token(b"BBN"),
	
	/// TODO.
	BBO = Self::letters_to_token(b"BBO"),
	
	/// TODO.
	BBP = Self::letters_to_token(b"BBP"),
	
	/// TODO.
	BBQ = Self::letters_to_token(b"BBQ"),
	
	/// TODO.
	BBR = Self::letters_to_token(b"BBR"),
	
	/// TODO.
	BBS = Self::letters_to_token(b"BBS"),
	
	/// TODO.
	BBT = Self::letters_to_token(b"BBT"),
	
	/// TODO.
	BBU = Self::letters_to_token(b"BBU"),
	
	/// TODO.
	BBV = Self::letters_to_token(b"BBV"),
	
	/// TODO.
	BBW = Self::letters_to_token(b"BBW"),
	
	/// TODO.
	BBX = Self::letters_to_token(b"BBX"),
	
	/// TODO.
	BBY = Self::letters_to_token(b"BBY"),
	
	/// TODO.
	BBZ = Self::letters_to_token(b"BBZ"),
	
	/// TODO.
	BCA = Self::letters_to_token(b"BCA"),
	
	/// TODO.
	BCB = Self::letters_to_token(b"BCB"),
	
	/// TODO.
	BCC = Self::letters_to_token(b"BCC"),
	
	/// TODO.
	BCD = Self::letters_to_token(b"BCD"),
	
	/// TODO.
	BCE = Self::letters_to_token(b"BCE"),
	
	/// TODO.
	BCF = Self::letters_to_token(b"BCF"),
	
	/// TODO.
	BCG = Self::letters_to_token(b"BCG"),
	
	/// TODO.
	BCH = Self::letters_to_token(b"BCH"),
	
	/// TODO.
	BCI = Self::letters_to_token(b"BCI"),
	
	/// TODO.
	BCJ = Self::letters_to_token(b"BCJ"),
	
	/// TODO.
	BCK = Self::letters_to_token(b"BCK"),
	
	/// TODO.
	BCL = Self::letters_to_token(b"BCL"),
	
	/// TODO.
	BCM = Self::letters_to_token(b"BCM"),
	
	/// TODO.
	BCN = Self::letters_to_token(b"BCN"),
	
	/// TODO.
	BCO = Self::letters_to_token(b"BCO"),
	
	/// TODO.
	BCP = Self::letters_to_token(b"BCP"),
	
	/// TODO.
	BCQ = Self::letters_to_token(b"BCQ"),
	
	/// TODO.
	BCR = Self::letters_to_token(b"BCR"),
	
	/// TODO.
	BCS = Self::letters_to_token(b"BCS"),
	
	/// TODO.
	BCT = Self::letters_to_token(b"BCT"),
	
	/// TODO.
	BCU = Self::letters_to_token(b"BCU"),
	
	/// TODO.
	BCV = Self::letters_to_token(b"BCV"),
	
	/// TODO.
	BCW = Self::letters_to_token(b"BCW"),
	
	/// TODO.
	BCX = Self::letters_to_token(b"BCX"),
	
	/// TODO.
	BCY = Self::letters_to_token(b"BCY"),
	
	/// TODO.
	BCZ = Self::letters_to_token(b"BCZ"),
	
	/// TODO.
	BDA = Self::letters_to_token(b"BDA"),
	
	/// TODO.
	BDB = Self::letters_to_token(b"BDB"),
	
	/// TODO.
	BDC = Self::letters_to_token(b"BDC"),
	
	/// TODO.
	BDD = Self::letters_to_token(b"BDD"),
	
	/// TODO.
	BDE = Self::letters_to_token(b"BDE"),
	
	/// TODO.
	BDF = Self::letters_to_token(b"BDF"),
	
	/// TODO.
	BDG = Self::letters_to_token(b"BDG"),
	
	/// TODO.
	BDH = Self::letters_to_token(b"BDH"),
	
	/// TODO.
	BDI = Self::letters_to_token(b"BDI"),
	
	/// TODO.
	BDJ = Self::letters_to_token(b"BDJ"),
	
	/// TODO.
	BDK = Self::letters_to_token(b"BDK"),
	
	/// TODO.
	BDL = Self::letters_to_token(b"BDL"),
	
	/// TODO.
	BDM = Self::letters_to_token(b"BDM"),
	
	/// TODO.
	BDN = Self::letters_to_token(b"BDN"),
	
	/// TODO.
	BDO = Self::letters_to_token(b"BDO"),
	
	/// TODO.
	BDP = Self::letters_to_token(b"BDP"),
	
	/// TODO.
	BDQ = Self::letters_to_token(b"BDQ"),
	
	/// TODO.
	BDR = Self::letters_to_token(b"BDR"),
	
	/// TODO.
	BDS = Self::letters_to_token(b"BDS"),
	
	/// TODO.
	BDT = Self::letters_to_token(b"BDT"),
	
	/// TODO.
	BDU = Self::letters_to_token(b"BDU"),
	
	/// TODO.
	BDV = Self::letters_to_token(b"BDV"),
	
	/// TODO.
	BDW = Self::letters_to_token(b"BDW"),
	
	/// TODO.
	BDX = Self::letters_to_token(b"BDX"),
	
	/// TODO.
	BDY = Self::letters_to_token(b"BDY"),
	
	/// TODO.
	BDZ = Self::letters_to_token(b"BDZ"),
	
	/// TODO.
	BEA = Self::letters_to_token(b"BEA"),
	
	/// TODO.
	BEB = Self::letters_to_token(b"BEB"),
	
	/// TODO.
	BEC = Self::letters_to_token(b"BEC"),
	
	/// TODO.
	BED = Self::letters_to_token(b"BED"),
	
	/// TODO.
	BEE = Self::letters_to_token(b"BEE"),
	
	/// TODO.
	BEF = Self::letters_to_token(b"BEF"),
	
	/// TODO.
	BEG = Self::letters_to_token(b"BEG"),
	
	/// TODO.
	BEH = Self::letters_to_token(b"BEH"),
	
	/// TODO.
	BEI = Self::letters_to_token(b"BEI"),
	
	/// TODO.
	BEJ = Self::letters_to_token(b"BEJ"),
	
	/// TODO.
	BEK = Self::letters_to_token(b"BEK"),
	
	/// TODO.
	BEL = Self::letters_to_token(b"BEL"),
	
	/// TODO.
	BEM = Self::letters_to_token(b"BEM"),
	
	/// TODO.
	BEN = Self::letters_to_token(b"BEN"),
	
	/// TODO.
	BEO = Self::letters_to_token(b"BEO"),
	
	/// TODO.
	BEP = Self::letters_to_token(b"BEP"),
	
	/// TODO.
	BEQ = Self::letters_to_token(b"BEQ"),
	
	/// TODO.
	BER = Self::letters_to_token(b"BER"),
	
	/// TODO.
	BES = Self::letters_to_token(b"BES"),
	
	/// TODO.
	BET = Self::letters_to_token(b"BET"),
	
	/// TODO.
	BEU = Self::letters_to_token(b"BEU"),
	
	/// TODO.
	BEV = Self::letters_to_token(b"BEV"),
	
	/// TODO.
	BEW = Self::letters_to_token(b"BEW"),
	
	/// TODO.
	BEX = Self::letters_to_token(b"BEX"),
	
	/// TODO.
	BEY = Self::letters_to_token(b"BEY"),
	
	/// TODO.
	BEZ = Self::letters_to_token(b"BEZ"),
	
	/// TODO.
	BFA = Self::letters_to_token(b"BFA"),
	
	/// TODO.
	BFB = Self::letters_to_token(b"BFB"),
	
	/// TODO.
	BFC = Self::letters_to_token(b"BFC"),
	
	/// TODO.
	BFD = Self::letters_to_token(b"BFD"),
	
	/// TODO.
	BFE = Self::letters_to_token(b"BFE"),
	
	/// TODO.
	BFF = Self::letters_to_token(b"BFF"),
	
	/// TODO.
	BFG = Self::letters_to_token(b"BFG"),
	
	/// TODO.
	BFH = Self::letters_to_token(b"BFH"),
	
	/// TODO.
	BFI = Self::letters_to_token(b"BFI"),
	
	/// TODO.
	BFJ = Self::letters_to_token(b"BFJ"),
	
	/// TODO.
	BFK = Self::letters_to_token(b"BFK"),
	
	/// TODO.
	BFL = Self::letters_to_token(b"BFL"),
	
	/// TODO.
	BFM = Self::letters_to_token(b"BFM"),
	
	/// TODO.
	BFN = Self::letters_to_token(b"BFN"),
	
	/// TODO.
	BFO = Self::letters_to_token(b"BFO"),
	
	/// TODO.
	BFP = Self::letters_to_token(b"BFP"),
	
	/// TODO.
	BFQ = Self::letters_to_token(b"BFQ"),
	
	/// TODO.
	BFR = Self::letters_to_token(b"BFR"),
	
	/// TODO.
	BFS = Self::letters_to_token(b"BFS"),
	
	/// TODO.
	BFT = Self::letters_to_token(b"BFT"),
	
	/// TODO.
	BFU = Self::letters_to_token(b"BFU"),
	
	/// TODO.
	BFV = Self::letters_to_token(b"BFV"),
	
	/// TODO.
	BFW = Self::letters_to_token(b"BFW"),
	
	/// TODO.
	BFX = Self::letters_to_token(b"BFX"),
	
	/// TODO.
	BFY = Self::letters_to_token(b"BFY"),
	
	/// TODO.
	BFZ = Self::letters_to_token(b"BFZ"),
	
	/// TODO.
	BGA = Self::letters_to_token(b"BGA"),
	
	/// TODO.
	BGB = Self::letters_to_token(b"BGB"),
	
	/// TODO.
	BGC = Self::letters_to_token(b"BGC"),
	
	/// TODO.
	BGD = Self::letters_to_token(b"BGD"),
	
	/// TODO.
	BGE = Self::letters_to_token(b"BGE"),
	
	/// TODO.
	BGF = Self::letters_to_token(b"BGF"),
	
	/// TODO.
	BGG = Self::letters_to_token(b"BGG"),
	
	/// TODO.
	BGH = Self::letters_to_token(b"BGH"),
	
	/// TODO.
	BGI = Self::letters_to_token(b"BGI"),
	
	/// TODO.
	BGJ = Self::letters_to_token(b"BGJ"),
	
	/// TODO.
	BGK = Self::letters_to_token(b"BGK"),
	
	/// TODO.
	BGL = Self::letters_to_token(b"BGL"),
	
	/// TODO.
	BGM = Self::letters_to_token(b"BGM"),
	
	/// TODO.
	BGN = Self::letters_to_token(b"BGN"),
	
	/// TODO.
	BGO = Self::letters_to_token(b"BGO"),
	
	/// TODO.
	BGP = Self::letters_to_token(b"BGP"),
	
	/// TODO.
	BGQ = Self::letters_to_token(b"BGQ"),
	
	/// TODO.
	BGR = Self::letters_to_token(b"BGR"),
	
	/// TODO.
	BGS = Self::letters_to_token(b"BGS"),
	
	/// TODO.
	BGT = Self::letters_to_token(b"BGT"),
	
	/// TODO.
	BGU = Self::letters_to_token(b"BGU"),
	
	/// TODO.
	BGV = Self::letters_to_token(b"BGV"),
	
	/// TODO.
	BGW = Self::letters_to_token(b"BGW"),
	
	/// TODO.
	BGX = Self::letters_to_token(b"BGX"),
	
	/// TODO.
	BGY = Self::letters_to_token(b"BGY"),
	
	/// TODO.
	BGZ = Self::letters_to_token(b"BGZ"),
	
	/// TODO.
	BHA = Self::letters_to_token(b"BHA"),
	
	/// TODO.
	BHB = Self::letters_to_token(b"BHB"),
	
	/// TODO.
	BHC = Self::letters_to_token(b"BHC"),
	
	/// TODO.
	BHD = Self::letters_to_token(b"BHD"),
	
	/// TODO.
	BHE = Self::letters_to_token(b"BHE"),
	
	/// TODO.
	BHF = Self::letters_to_token(b"BHF"),
	
	/// TODO.
	BHG = Self::letters_to_token(b"BHG"),
	
	/// TODO.
	BHH = Self::letters_to_token(b"BHH"),
	
	/// TODO.
	BHI = Self::letters_to_token(b"BHI"),
	
	/// TODO.
	BHJ = Self::letters_to_token(b"BHJ"),
	
	/// TODO.
	BHK = Self::letters_to_token(b"BHK"),
	
	/// TODO.
	BHL = Self::letters_to_token(b"BHL"),
	
	/// TODO.
	BHM = Self::letters_to_token(b"BHM"),
	
	/// TODO.
	BHN = Self::letters_to_token(b"BHN"),
	
	/// TODO.
	BHO = Self::letters_to_token(b"BHO"),
	
	/// TODO.
	BHP = Self::letters_to_token(b"BHP"),
	
	/// TODO.
	BHQ = Self::letters_to_token(b"BHQ"),
	
	/// TODO.
	BHR = Self::letters_to_token(b"BHR"),
	
	/// TODO.
	BHS = Self::letters_to_token(b"BHS"),
	
	/// TODO.
	BHT = Self::letters_to_token(b"BHT"),
	
	/// TODO.
	BHU = Self::letters_to_token(b"BHU"),
	
	/// TODO.
	BHV = Self::letters_to_token(b"BHV"),
	
	/// TODO.
	BHW = Self::letters_to_token(b"BHW"),
	
	/// TODO.
	BHX = Self::letters_to_token(b"BHX"),
	
	/// TODO.
	BHY = Self::letters_to_token(b"BHY"),
	
	/// TODO.
	BHZ = Self::letters_to_token(b"BHZ"),
	
	/// TODO.
	BIA = Self::letters_to_token(b"BIA"),
	
	/// TODO.
	BIB = Self::letters_to_token(b"BIB"),
	
	/// TODO.
	BIC = Self::letters_to_token(b"BIC"),
	
	/// TODO.
	BID = Self::letters_to_token(b"BID"),
	
	/// TODO.
	BIE = Self::letters_to_token(b"BIE"),
	
	/// TODO.
	BIF = Self::letters_to_token(b"BIF"),
	
	/// TODO.
	BIG = Self::letters_to_token(b"BIG"),
	
	/// TODO.
	BIH = Self::letters_to_token(b"BIH"),
	
	/// TODO.
	BII = Self::letters_to_token(b"BII"),
	
	/// TODO.
	BIJ = Self::letters_to_token(b"BIJ"),
	
	/// TODO.
	BIK = Self::letters_to_token(b"BIK"),
	
	/// TODO.
	BIL = Self::letters_to_token(b"BIL"),
	
	/// TODO.
	BIM = Self::letters_to_token(b"BIM"),
	
	/// TODO.
	BIN = Self::letters_to_token(b"BIN"),
	
	/// TODO.
	BIO = Self::letters_to_token(b"BIO"),
	
	/// TODO.
	BIP = Self::letters_to_token(b"BIP"),
	
	/// TODO.
	BIQ = Self::letters_to_token(b"BIQ"),
	
	/// TODO.
	BIR = Self::letters_to_token(b"BIR"),
	
	/// TODO.
	BIS = Self::letters_to_token(b"BIS"),
	
	/// TODO.
	BIT = Self::letters_to_token(b"BIT"),
	
	/// TODO.
	BIU = Self::letters_to_token(b"BIU"),
	
	/// TODO.
	BIV = Self::letters_to_token(b"BIV"),
	
	/// TODO.
	BIW = Self::letters_to_token(b"BIW"),
	
	/// TODO.
	BIX = Self::letters_to_token(b"BIX"),
	
	/// TODO.
	BIY = Self::letters_to_token(b"BIY"),
	
	/// TODO.
	BIZ = Self::letters_to_token(b"BIZ"),
	
	/// TODO.
	BJA = Self::letters_to_token(b"BJA"),
	
	/// TODO.
	BJB = Self::letters_to_token(b"BJB"),
	
	/// TODO.
	BJC = Self::letters_to_token(b"BJC"),
	
	/// TODO.
	BJD = Self::letters_to_token(b"BJD"),
	
	/// TODO.
	BJE = Self::letters_to_token(b"BJE"),
	
	/// TODO.
	BJF = Self::letters_to_token(b"BJF"),
	
	/// TODO.
	BJG = Self::letters_to_token(b"BJG"),
	
	/// TODO.
	BJH = Self::letters_to_token(b"BJH"),
	
	/// TODO.
	BJI = Self::letters_to_token(b"BJI"),
	
	/// TODO.
	BJJ = Self::letters_to_token(b"BJJ"),
	
	/// TODO.
	BJK = Self::letters_to_token(b"BJK"),
	
	/// TODO.
	BJL = Self::letters_to_token(b"BJL"),
	
	/// TODO.
	BJM = Self::letters_to_token(b"BJM"),
	
	/// TODO.
	BJN = Self::letters_to_token(b"BJN"),
	
	/// TODO.
	BJO = Self::letters_to_token(b"BJO"),
	
	/// TODO.
	BJP = Self::letters_to_token(b"BJP"),
	
	/// TODO.
	BJQ = Self::letters_to_token(b"BJQ"),
	
	/// TODO.
	BJR = Self::letters_to_token(b"BJR"),
	
	/// TODO.
	BJS = Self::letters_to_token(b"BJS"),
	
	/// TODO.
	BJT = Self::letters_to_token(b"BJT"),
	
	/// TODO.
	BJU = Self::letters_to_token(b"BJU"),
	
	/// TODO.
	BJV = Self::letters_to_token(b"BJV"),
	
	/// TODO.
	BJW = Self::letters_to_token(b"BJW"),
	
	/// TODO.
	BJX = Self::letters_to_token(b"BJX"),
	
	/// TODO.
	BJY = Self::letters_to_token(b"BJY"),
	
	/// TODO.
	BJZ = Self::letters_to_token(b"BJZ"),
	
	/// TODO.
	BKA = Self::letters_to_token(b"BKA"),
	
	/// TODO.
	BKB = Self::letters_to_token(b"BKB"),
	
	/// TODO.
	BKC = Self::letters_to_token(b"BKC"),
	
	/// TODO.
	BKD = Self::letters_to_token(b"BKD"),
	
	/// TODO.
	BKE = Self::letters_to_token(b"BKE"),
	
	/// TODO.
	BKF = Self::letters_to_token(b"BKF"),
	
	/// TODO.
	BKG = Self::letters_to_token(b"BKG"),
	
	/// TODO.
	BKH = Self::letters_to_token(b"BKH"),
	
	/// TODO.
	BKI = Self::letters_to_token(b"BKI"),
	
	/// TODO.
	BKJ = Self::letters_to_token(b"BKJ"),
	
	/// TODO.
	BKK = Self::letters_to_token(b"BKK"),
	
	/// TODO.
	BKL = Self::letters_to_token(b"BKL"),
	
	/// TODO.
	BKM = Self::letters_to_token(b"BKM"),
	
	/// TODO.
	BKN = Self::letters_to_token(b"BKN"),
	
	/// TODO.
	BKO = Self::letters_to_token(b"BKO"),
	
	/// TODO.
	BKP = Self::letters_to_token(b"BKP"),
	
	/// TODO.
	BKQ = Self::letters_to_token(b"BKQ"),
	
	/// TODO.
	BKR = Self::letters_to_token(b"BKR"),
	
	/// TODO.
	BKS = Self::letters_to_token(b"BKS"),
	
	/// TODO.
	BKT = Self::letters_to_token(b"BKT"),
	
	/// TODO.
	BKU = Self::letters_to_token(b"BKU"),
	
	/// TODO.
	BKV = Self::letters_to_token(b"BKV"),
	
	/// TODO.
	BKW = Self::letters_to_token(b"BKW"),
	
	/// TODO.
	BKX = Self::letters_to_token(b"BKX"),
	
	/// TODO.
	BKY = Self::letters_to_token(b"BKY"),
	
	/// TODO.
	BKZ = Self::letters_to_token(b"BKZ"),
	
	/// TODO.
	BLA = Self::letters_to_token(b"BLA"),
	
	/// TODO.
	BLB = Self::letters_to_token(b"BLB"),
	
	/// TODO.
	BLC = Self::letters_to_token(b"BLC"),
	
	/// TODO.
	BLD = Self::letters_to_token(b"BLD"),
	
	/// TODO.
	BLE = Self::letters_to_token(b"BLE"),
	
	/// TODO.
	BLF = Self::letters_to_token(b"BLF"),
	
	/// TODO.
	BLG = Self::letters_to_token(b"BLG"),
	
	/// TODO.
	BLH = Self::letters_to_token(b"BLH"),
	
	/// TODO.
	BLI = Self::letters_to_token(b"BLI"),
	
	/// TODO.
	BLJ = Self::letters_to_token(b"BLJ"),
	
	/// TODO.
	BLK = Self::letters_to_token(b"BLK"),
	
	/// TODO.
	BLL = Self::letters_to_token(b"BLL"),
	
	/// TODO.
	BLM = Self::letters_to_token(b"BLM"),
	
	/// TODO.
	BLN = Self::letters_to_token(b"BLN"),
	
	/// TODO.
	BLO = Self::letters_to_token(b"BLO"),
	
	/// TODO.
	BLP = Self::letters_to_token(b"BLP"),
	
	/// TODO.
	BLQ = Self::letters_to_token(b"BLQ"),
	
	/// TODO.
	BLR = Self::letters_to_token(b"BLR"),
	
	/// TODO.
	BLS = Self::letters_to_token(b"BLS"),
	
	/// TODO.
	BLT = Self::letters_to_token(b"BLT"),
	
	/// TODO.
	BLU = Self::letters_to_token(b"BLU"),
	
	/// TODO.
	BLV = Self::letters_to_token(b"BLV"),
	
	/// TODO.
	BLW = Self::letters_to_token(b"BLW"),
	
	/// TODO.
	BLX = Self::letters_to_token(b"BLX"),
	
	/// TODO.
	BLY = Self::letters_to_token(b"BLY"),
	
	/// TODO.
	BLZ = Self::letters_to_token(b"BLZ"),
	
	/// TODO.
	BMA = Self::letters_to_token(b"BMA"),
	
	/// TODO.
	BMB = Self::letters_to_token(b"BMB"),
	
	/// TODO.
	BMC = Self::letters_to_token(b"BMC"),
	
	/// TODO.
	BMD = Self::letters_to_token(b"BMD"),
	
	/// TODO.
	BME = Self::letters_to_token(b"BME"),
	
	/// TODO.
	BMF = Self::letters_to_token(b"BMF"),
	
	/// TODO.
	BMG = Self::letters_to_token(b"BMG"),
	
	/// TODO.
	BMH = Self::letters_to_token(b"BMH"),
	
	/// TODO.
	BMI = Self::letters_to_token(b"BMI"),
	
	/// TODO.
	BMJ = Self::letters_to_token(b"BMJ"),
	
	/// TODO.
	BMK = Self::letters_to_token(b"BMK"),
	
	/// TODO.
	BML = Self::letters_to_token(b"BML"),
	
	/// TODO.
	BMM = Self::letters_to_token(b"BMM"),
	
	/// TODO.
	BMN = Self::letters_to_token(b"BMN"),
	
	/// TODO.
	BMO = Self::letters_to_token(b"BMO"),
	
	/// TODO.
	BMP = Self::letters_to_token(b"BMP"),
	
	/// TODO.
	BMQ = Self::letters_to_token(b"BMQ"),
	
	/// TODO.
	BMR = Self::letters_to_token(b"BMR"),
	
	/// TODO.
	BMS = Self::letters_to_token(b"BMS"),
	
	/// TODO.
	BMT = Self::letters_to_token(b"BMT"),
	
	/// TODO.
	BMU = Self::letters_to_token(b"BMU"),
	
	/// TODO.
	BMV = Self::letters_to_token(b"BMV"),
	
	/// TODO.
	BMW = Self::letters_to_token(b"BMW"),
	
	/// TODO.
	BMX = Self::letters_to_token(b"BMX"),
	
	/// TODO.
	BMY = Self::letters_to_token(b"BMY"),
	
	/// TODO.
	BMZ = Self::letters_to_token(b"BMZ"),
	
	/// TODO.
	BNA = Self::letters_to_token(b"BNA"),
	
	/// TODO.
	BNB = Self::letters_to_token(b"BNB"),
	
	/// TODO.
	BNC = Self::letters_to_token(b"BNC"),
	
	/// TODO.
	BND = Self::letters_to_token(b"BND"),
	
	/// TODO.
	BNE = Self::letters_to_token(b"BNE"),
	
	/// TODO.
	BNF = Self::letters_to_token(b"BNF"),
	
	/// TODO.
	BNG = Self::letters_to_token(b"BNG"),
	
	/// TODO.
	BNH = Self::letters_to_token(b"BNH"),
	
	/// TODO.
	BNI = Self::letters_to_token(b"BNI"),
	
	/// TODO.
	BNJ = Self::letters_to_token(b"BNJ"),
	
	/// TODO.
	BNK = Self::letters_to_token(b"BNK"),
	
	/// TODO.
	BNL = Self::letters_to_token(b"BNL"),
	
	/// TODO.
	BNM = Self::letters_to_token(b"BNM"),
	
	/// TODO.
	BNN = Self::letters_to_token(b"BNN"),
	
	/// TODO.
	BNO = Self::letters_to_token(b"BNO"),
	
	/// TODO.
	BNP = Self::letters_to_token(b"BNP"),
	
	/// TODO.
	BNQ = Self::letters_to_token(b"BNQ"),
	
	/// TODO.
	BNR = Self::letters_to_token(b"BNR"),
	
	/// TODO.
	BNS = Self::letters_to_token(b"BNS"),
	
	/// TODO.
	BNT = Self::letters_to_token(b"BNT"),
	
	/// TODO.
	BNU = Self::letters_to_token(b"BNU"),
	
	/// TODO.
	BNV = Self::letters_to_token(b"BNV"),
	
	/// TODO.
	BNW = Self::letters_to_token(b"BNW"),
	
	/// TODO.
	BNX = Self::letters_to_token(b"BNX"),
	
	/// TODO.
	BNY = Self::letters_to_token(b"BNY"),
	
	/// TODO.
	BNZ = Self::letters_to_token(b"BNZ"),
	
	/// TODO.
	BOA = Self::letters_to_token(b"BOA"),
	
	/// TODO.
	BOB = Self::letters_to_token(b"BOB"),
	
	/// TODO.
	BOC = Self::letters_to_token(b"BOC"),
	
	/// TODO.
	BOD = Self::letters_to_token(b"BOD"),
	
	/// TODO.
	BOE = Self::letters_to_token(b"BOE"),
	
	/// TODO.
	BOF = Self::letters_to_token(b"BOF"),
	
	/// TODO.
	BOG = Self::letters_to_token(b"BOG"),
	
	/// TODO.
	BOH = Self::letters_to_token(b"BOH"),
	
	/// TODO.
	BOI = Self::letters_to_token(b"BOI"),
	
	/// TODO.
	BOJ = Self::letters_to_token(b"BOJ"),
	
	/// TODO.
	BOK = Self::letters_to_token(b"BOK"),
	
	/// TODO.
	BOL = Self::letters_to_token(b"BOL"),
	
	/// TODO.
	BOM = Self::letters_to_token(b"BOM"),
	
	/// TODO.
	BON = Self::letters_to_token(b"BON"),
	
	/// TODO.
	BOO = Self::letters_to_token(b"BOO"),
	
	/// TODO.
	BOP = Self::letters_to_token(b"BOP"),
	
	/// TODO.
	BOQ = Self::letters_to_token(b"BOQ"),
	
	/// TODO.
	BOR = Self::letters_to_token(b"BOR"),
	
	/// TODO.
	BOS = Self::letters_to_token(b"BOS"),
	
	/// TODO.
	BOT = Self::letters_to_token(b"BOT"),
	
	/// TODO.
	BOU = Self::letters_to_token(b"BOU"),
	
	/// TODO.
	BOV = Self::letters_to_token(b"BOV"),
	
	/// TODO.
	BOW = Self::letters_to_token(b"BOW"),
	
	/// TODO.
	BOX = Self::letters_to_token(b"BOX"),
	
	/// TODO.
	BOY = Self::letters_to_token(b"BOY"),
	
	/// TODO.
	BOZ = Self::letters_to_token(b"BOZ"),
	
	/// TODO.
	BPA = Self::letters_to_token(b"BPA"),
	
	/// TODO.
	BPB = Self::letters_to_token(b"BPB"),
	
	/// TODO.
	BPC = Self::letters_to_token(b"BPC"),
	
	/// TODO.
	BPD = Self::letters_to_token(b"BPD"),
	
	/// TODO.
	BPE = Self::letters_to_token(b"BPE"),
	
	/// TODO.
	BPF = Self::letters_to_token(b"BPF"),
	
	/// TODO.
	BPG = Self::letters_to_token(b"BPG"),
	
	/// TODO.
	BPH = Self::letters_to_token(b"BPH"),
	
	/// TODO.
	BPI = Self::letters_to_token(b"BPI"),
	
	/// TODO.
	BPJ = Self::letters_to_token(b"BPJ"),
	
	/// TODO.
	BPK = Self::letters_to_token(b"BPK"),
	
	/// TODO.
	BPL = Self::letters_to_token(b"BPL"),
	
	/// TODO.
	BPM = Self::letters_to_token(b"BPM"),
	
	/// TODO.
	BPN = Self::letters_to_token(b"BPN"),
	
	/// TODO.
	BPO = Self::letters_to_token(b"BPO"),
	
	/// TODO.
	BPP = Self::letters_to_token(b"BPP"),
	
	/// TODO.
	BPQ = Self::letters_to_token(b"BPQ"),
	
	/// TODO.
	BPR = Self::letters_to_token(b"BPR"),
	
	/// TODO.
	BPS = Self::letters_to_token(b"BPS"),
	
	/// TODO.
	BPT = Self::letters_to_token(b"BPT"),
	
	/// TODO.
	BPU = Self::letters_to_token(b"BPU"),
	
	/// TODO.
	BPV = Self::letters_to_token(b"BPV"),
	
	/// TODO.
	BPW = Self::letters_to_token(b"BPW"),
	
	/// TODO.
	BPX = Self::letters_to_token(b"BPX"),
	
	/// TODO.
	BPY = Self::letters_to_token(b"BPY"),
	
	/// TODO.
	BPZ = Self::letters_to_token(b"BPZ"),
	
	/// TODO.
	BQA = Self::letters_to_token(b"BQA"),
	
	/// TODO.
	BQB = Self::letters_to_token(b"BQB"),
	
	/// TODO.
	BQC = Self::letters_to_token(b"BQC"),
	
	/// TODO.
	BQD = Self::letters_to_token(b"BQD"),
	
	/// TODO.
	BQE = Self::letters_to_token(b"BQE"),
	
	/// TODO.
	BQF = Self::letters_to_token(b"BQF"),
	
	/// TODO.
	BQG = Self::letters_to_token(b"BQG"),
	
	/// TODO.
	BQH = Self::letters_to_token(b"BQH"),
	
	/// TODO.
	BQI = Self::letters_to_token(b"BQI"),
	
	/// TODO.
	BQJ = Self::letters_to_token(b"BQJ"),
	
	/// TODO.
	BQK = Self::letters_to_token(b"BQK"),
	
	/// TODO.
	BQL = Self::letters_to_token(b"BQL"),
	
	/// TODO.
	BQM = Self::letters_to_token(b"BQM"),
	
	/// TODO.
	BQN = Self::letters_to_token(b"BQN"),
	
	/// TODO.
	BQO = Self::letters_to_token(b"BQO"),
	
	/// TODO.
	BQP = Self::letters_to_token(b"BQP"),
	
	/// TODO.
	BQQ = Self::letters_to_token(b"BQQ"),
	
	/// TODO.
	BQR = Self::letters_to_token(b"BQR"),
	
	/// TODO.
	BQS = Self::letters_to_token(b"BQS"),
	
	/// TODO.
	BQT = Self::letters_to_token(b"BQT"),
	
	/// TODO.
	BQU = Self::letters_to_token(b"BQU"),
	
	/// TODO.
	BQV = Self::letters_to_token(b"BQV"),
	
	/// TODO.
	BQW = Self::letters_to_token(b"BQW"),
	
	/// TODO.
	BQX = Self::letters_to_token(b"BQX"),
	
	/// TODO.
	BQY = Self::letters_to_token(b"BQY"),
	
	/// TODO.
	BQZ = Self::letters_to_token(b"BQZ"),
	
	/// TODO.
	BRA = Self::letters_to_token(b"BRA"),
	
	/// TODO.
	BRB = Self::letters_to_token(b"BRB"),
	
	/// TODO.
	BRC = Self::letters_to_token(b"BRC"),
	
	/// TODO.
	BRD = Self::letters_to_token(b"BRD"),
	
	/// TODO.
	BRE = Self::letters_to_token(b"BRE"),
	
	/// TODO.
	BRF = Self::letters_to_token(b"BRF"),
	
	/// TODO.
	BRG = Self::letters_to_token(b"BRG"),
	
	/// TODO.
	BRH = Self::letters_to_token(b"BRH"),
	
	/// TODO.
	BRI = Self::letters_to_token(b"BRI"),
	
	/// TODO.
	BRJ = Self::letters_to_token(b"BRJ"),
	
	/// TODO.
	BRK = Self::letters_to_token(b"BRK"),
	
	/// TODO.
	BRL = Self::letters_to_token(b"BRL"),
	
	/// TODO.
	BRM = Self::letters_to_token(b"BRM"),
	
	/// TODO.
	BRN = Self::letters_to_token(b"BRN"),
	
	/// TODO.
	BRO = Self::letters_to_token(b"BRO"),
	
	/// TODO.
	BRP = Self::letters_to_token(b"BRP"),
	
	/// TODO.
	BRQ = Self::letters_to_token(b"BRQ"),
	
	/// TODO.
	BRR = Self::letters_to_token(b"BRR"),
	
	/// TODO.
	BRS = Self::letters_to_token(b"BRS"),
	
	/// TODO.
	BRT = Self::letters_to_token(b"BRT"),
	
	/// TODO.
	BRU = Self::letters_to_token(b"BRU"),
	
	/// TODO.
	BRV = Self::letters_to_token(b"BRV"),
	
	/// TODO.
	BRW = Self::letters_to_token(b"BRW"),
	
	/// TODO.
	BRX = Self::letters_to_token(b"BRX"),
	
	/// TODO.
	BRY = Self::letters_to_token(b"BRY"),
	
	/// TODO.
	BRZ = Self::letters_to_token(b"BRZ"),
	
	/// TODO.
	BSA = Self::letters_to_token(b"BSA"),
	
	/// TODO.
	BSB = Self::letters_to_token(b"BSB"),
	
	/// TODO.
	BSC = Self::letters_to_token(b"BSC"),
	
	/// TODO.
	BSD = Self::letters_to_token(b"BSD"),
	
	/// TODO.
	BSE = Self::letters_to_token(b"BSE"),
	
	/// TODO.
	BSF = Self::letters_to_token(b"BSF"),
	
	/// TODO.
	BSG = Self::letters_to_token(b"BSG"),
	
	/// TODO.
	BSH = Self::letters_to_token(b"BSH"),
	
	/// TODO.
	BSI = Self::letters_to_token(b"BSI"),
	
	/// TODO.
	BSJ = Self::letters_to_token(b"BSJ"),
	
	/// TODO.
	BSK = Self::letters_to_token(b"BSK"),
	
	/// TODO.
	BSL = Self::letters_to_token(b"BSL"),
	
	/// TODO.
	BSM = Self::letters_to_token(b"BSM"),
	
	/// TODO.
	BSN = Self::letters_to_token(b"BSN"),
	
	/// TODO.
	BSO = Self::letters_to_token(b"BSO"),
	
	/// TODO.
	BSP = Self::letters_to_token(b"BSP"),
	
	/// TODO.
	BSQ = Self::letters_to_token(b"BSQ"),
	
	/// TODO.
	BSR = Self::letters_to_token(b"BSR"),
	
	/// TODO.
	BSS = Self::letters_to_token(b"BSS"),
	
	/// TODO.
	BST = Self::letters_to_token(b"BST"),
	
	/// TODO.
	BSU = Self::letters_to_token(b"BSU"),
	
	/// TODO.
	BSV = Self::letters_to_token(b"BSV"),
	
	/// TODO.
	BSW = Self::letters_to_token(b"BSW"),
	
	/// TODO.
	BSX = Self::letters_to_token(b"BSX"),
	
	/// TODO.
	BSY = Self::letters_to_token(b"BSY"),
	
	/// TODO.
	BSZ = Self::letters_to_token(b"BSZ"),
	
	/// TODO.
	BTA = Self::letters_to_token(b"BTA"),
	
	/// TODO.
	BTB = Self::letters_to_token(b"BTB"),
	
	/// TODO.
	BTC = Self::letters_to_token(b"BTC"),
	
	/// TODO.
	BTD = Self::letters_to_token(b"BTD"),
	
	/// TODO.
	BTE = Self::letters_to_token(b"BTE"),
	
	/// TODO.
	BTF = Self::letters_to_token(b"BTF"),
	
	/// TODO.
	BTG = Self::letters_to_token(b"BTG"),
	
	/// TODO.
	BTH = Self::letters_to_token(b"BTH"),
	
	/// TODO.
	BTI = Self::letters_to_token(b"BTI"),
	
	/// TODO.
	BTJ = Self::letters_to_token(b"BTJ"),
	
	/// TODO.
	BTK = Self::letters_to_token(b"BTK"),
	
	/// TODO.
	BTL = Self::letters_to_token(b"BTL"),
	
	/// TODO.
	BTM = Self::letters_to_token(b"BTM"),
	
	/// TODO.
	BTN = Self::letters_to_token(b"BTN"),
	
	/// TODO.
	BTO = Self::letters_to_token(b"BTO"),
	
	/// TODO.
	BTP = Self::letters_to_token(b"BTP"),
	
	/// TODO.
	BTQ = Self::letters_to_token(b"BTQ"),
	
	/// TODO.
	BTR = Self::letters_to_token(b"BTR"),
	
	/// TODO.
	BTS = Self::letters_to_token(b"BTS"),
	
	/// TODO.
	BTT = Self::letters_to_token(b"BTT"),
	
	/// TODO.
	BTU = Self::letters_to_token(b"BTU"),
	
	/// TODO.
	BTV = Self::letters_to_token(b"BTV"),
	
	/// TODO.
	BTW = Self::letters_to_token(b"BTW"),
	
	/// TODO.
	BTX = Self::letters_to_token(b"BTX"),
	
	/// TODO.
	BTY = Self::letters_to_token(b"BTY"),
	
	/// TODO.
	BTZ = Self::letters_to_token(b"BTZ"),
	
	/// TODO.
	BUA = Self::letters_to_token(b"BUA"),
	
	/// TODO.
	BUB = Self::letters_to_token(b"BUB"),
	
	/// TODO.
	BUC = Self::letters_to_token(b"BUC"),
	
	/// TODO.
	BUD = Self::letters_to_token(b"BUD"),
	
	/// TODO.
	BUE = Self::letters_to_token(b"BUE"),
	
	/// TODO.
	BUF = Self::letters_to_token(b"BUF"),
	
	/// TODO.
	BUG = Self::letters_to_token(b"BUG"),
	
	/// TODO.
	BUH = Self::letters_to_token(b"BUH"),
	
	/// TODO.
	BUI = Self::letters_to_token(b"BUI"),
	
	/// TODO.
	BUJ = Self::letters_to_token(b"BUJ"),
	
	/// TODO.
	BUK = Self::letters_to_token(b"BUK"),
	
	/// TODO.
	BUL = Self::letters_to_token(b"BUL"),
	
	/// TODO.
	BUM = Self::letters_to_token(b"BUM"),
	
	/// TODO.
	BUN = Self::letters_to_token(b"BUN"),
	
	/// TODO.
	BUO = Self::letters_to_token(b"BUO"),
	
	/// TODO.
	BUP = Self::letters_to_token(b"BUP"),
	
	/// TODO.
	BUQ = Self::letters_to_token(b"BUQ"),
	
	/// TODO.
	BUR = Self::letters_to_token(b"BUR"),
	
	/// TODO.
	BUS = Self::letters_to_token(b"BUS"),
	
	/// TODO.
	BUT = Self::letters_to_token(b"BUT"),
	
	/// TODO.
	BUU = Self::letters_to_token(b"BUU"),
	
	/// TODO.
	BUV = Self::letters_to_token(b"BUV"),
	
	/// TODO.
	BUW = Self::letters_to_token(b"BUW"),
	
	/// TODO.
	BUX = Self::letters_to_token(b"BUX"),
	
	/// TODO.
	BUY = Self::letters_to_token(b"BUY"),
	
	/// TODO.
	BUZ = Self::letters_to_token(b"BUZ"),
	
	/// TODO.
	BVA = Self::letters_to_token(b"BVA"),
	
	/// TODO.
	BVB = Self::letters_to_token(b"BVB"),
	
	/// TODO.
	BVC = Self::letters_to_token(b"BVC"),
	
	/// TODO.
	BVD = Self::letters_to_token(b"BVD"),
	
	/// TODO.
	BVE = Self::letters_to_token(b"BVE"),
	
	/// TODO.
	BVF = Self::letters_to_token(b"BVF"),
	
	/// TODO.
	BVG = Self::letters_to_token(b"BVG"),
	
	/// TODO.
	BVH = Self::letters_to_token(b"BVH"),
	
	/// TODO.
	BVI = Self::letters_to_token(b"BVI"),
	
	/// TODO.
	BVJ = Self::letters_to_token(b"BVJ"),
	
	/// TODO.
	BVK = Self::letters_to_token(b"BVK"),
	
	/// TODO.
	BVL = Self::letters_to_token(b"BVL"),
	
	/// TODO.
	BVM = Self::letters_to_token(b"BVM"),
	
	/// TODO.
	BVN = Self::letters_to_token(b"BVN"),
	
	/// TODO.
	BVO = Self::letters_to_token(b"BVO"),
	
	/// TODO.
	BVP = Self::letters_to_token(b"BVP"),
	
	/// TODO.
	BVQ = Self::letters_to_token(b"BVQ"),
	
	/// TODO.
	BVR = Self::letters_to_token(b"BVR"),
	
	/// TODO.
	BVS = Self::letters_to_token(b"BVS"),
	
	/// TODO.
	BVT = Self::letters_to_token(b"BVT"),
	
	/// TODO.
	BVU = Self::letters_to_token(b"BVU"),
	
	/// TODO.
	BVV = Self::letters_to_token(b"BVV"),
	
	/// TODO.
	BVW = Self::letters_to_token(b"BVW"),
	
	/// TODO.
	BVX = Self::letters_to_token(b"BVX"),
	
	/// TODO.
	BVY = Self::letters_to_token(b"BVY"),
	
	/// TODO.
	BVZ = Self::letters_to_token(b"BVZ"),
	
	/// TODO.
	BWA = Self::letters_to_token(b"BWA"),
	
	/// TODO.
	BWB = Self::letters_to_token(b"BWB"),
	
	/// TODO.
	BWC = Self::letters_to_token(b"BWC"),
	
	/// TODO.
	BWD = Self::letters_to_token(b"BWD"),
	
	/// TODO.
	BWE = Self::letters_to_token(b"BWE"),
	
	/// TODO.
	BWF = Self::letters_to_token(b"BWF"),
	
	/// TODO.
	BWG = Self::letters_to_token(b"BWG"),
	
	/// TODO.
	BWH = Self::letters_to_token(b"BWH"),
	
	/// TODO.
	BWI = Self::letters_to_token(b"BWI"),
	
	/// TODO.
	BWJ = Self::letters_to_token(b"BWJ"),
	
	/// TODO.
	BWK = Self::letters_to_token(b"BWK"),
	
	/// TODO.
	BWL = Self::letters_to_token(b"BWL"),
	
	/// TODO.
	BWM = Self::letters_to_token(b"BWM"),
	
	/// TODO.
	BWN = Self::letters_to_token(b"BWN"),
	
	/// TODO.
	BWO = Self::letters_to_token(b"BWO"),
	
	/// TODO.
	BWP = Self::letters_to_token(b"BWP"),
	
	/// TODO.
	BWQ = Self::letters_to_token(b"BWQ"),
	
	/// TODO.
	BWR = Self::letters_to_token(b"BWR"),
	
	/// TODO.
	BWS = Self::letters_to_token(b"BWS"),
	
	/// TODO.
	BWT = Self::letters_to_token(b"BWT"),
	
	/// TODO.
	BWU = Self::letters_to_token(b"BWU"),
	
	/// TODO.
	BWV = Self::letters_to_token(b"BWV"),
	
	/// TODO.
	BWW = Self::letters_to_token(b"BWW"),
	
	/// TODO.
	BWX = Self::letters_to_token(b"BWX"),
	
	/// TODO.
	BWY = Self::letters_to_token(b"BWY"),
	
	/// TODO.
	BWZ = Self::letters_to_token(b"BWZ"),
	
	/// TODO.
	BXA = Self::letters_to_token(b"BXA"),
	
	/// TODO.
	BXB = Self::letters_to_token(b"BXB"),
	
	/// TODO.
	BXC = Self::letters_to_token(b"BXC"),
	
	/// TODO.
	BXD = Self::letters_to_token(b"BXD"),
	
	/// TODO.
	BXE = Self::letters_to_token(b"BXE"),
	
	/// TODO.
	BXF = Self::letters_to_token(b"BXF"),
	
	/// TODO.
	BXG = Self::letters_to_token(b"BXG"),
	
	/// TODO.
	BXH = Self::letters_to_token(b"BXH"),
	
	/// TODO.
	BXI = Self::letters_to_token(b"BXI"),
	
	/// TODO.
	BXJ = Self::letters_to_token(b"BXJ"),
	
	/// TODO.
	BXK = Self::letters_to_token(b"BXK"),
	
	/// TODO.
	BXL = Self::letters_to_token(b"BXL"),
	
	/// TODO.
	BXM = Self::letters_to_token(b"BXM"),
	
	/// TODO.
	BXN = Self::letters_to_token(b"BXN"),
	
	/// TODO.
	BXO = Self::letters_to_token(b"BXO"),
	
	/// TODO.
	BXP = Self::letters_to_token(b"BXP"),
	
	/// TODO.
	BXQ = Self::letters_to_token(b"BXQ"),
	
	/// TODO.
	BXR = Self::letters_to_token(b"BXR"),
	
	/// TODO.
	BXS = Self::letters_to_token(b"BXS"),
	
	/// TODO.
	BXT = Self::letters_to_token(b"BXT"),
	
	/// TODO.
	BXU = Self::letters_to_token(b"BXU"),
	
	/// TODO.
	BXV = Self::letters_to_token(b"BXV"),
	
	/// TODO.
	BXW = Self::letters_to_token(b"BXW"),
	
	/// TODO.
	BXX = Self::letters_to_token(b"BXX"),
	
	/// TODO.
	BXY = Self::letters_to_token(b"BXY"),
	
	/// TODO.
	BXZ = Self::letters_to_token(b"BXZ"),
	
	/// TODO.
	BYA = Self::letters_to_token(b"BYA"),
	
	/// TODO.
	BYB = Self::letters_to_token(b"BYB"),
	
	/// TODO.
	BYC = Self::letters_to_token(b"BYC"),
	
	/// TODO.
	BYD = Self::letters_to_token(b"BYD"),
	
	/// TODO.
	BYE = Self::letters_to_token(b"BYE"),
	
	/// TODO.
	BYF = Self::letters_to_token(b"BYF"),
	
	/// TODO.
	BYG = Self::letters_to_token(b"BYG"),
	
	/// TODO.
	BYH = Self::letters_to_token(b"BYH"),
	
	/// TODO.
	BYI = Self::letters_to_token(b"BYI"),
	
	/// TODO.
	BYJ = Self::letters_to_token(b"BYJ"),
	
	/// TODO.
	BYK = Self::letters_to_token(b"BYK"),
	
	/// TODO.
	BYL = Self::letters_to_token(b"BYL"),
	
	/// TODO.
	BYM = Self::letters_to_token(b"BYM"),
	
	/// TODO.
	BYN = Self::letters_to_token(b"BYN"),
	
	/// TODO.
	BYO = Self::letters_to_token(b"BYO"),
	
	/// TODO.
	BYP = Self::letters_to_token(b"BYP"),
	
	/// TODO.
	BYQ = Self::letters_to_token(b"BYQ"),
	
	/// TODO.
	BYR = Self::letters_to_token(b"BYR"),
	
	/// TODO.
	BYS = Self::letters_to_token(b"BYS"),
	
	/// TODO.
	BYT = Self::letters_to_token(b"BYT"),
	
	/// TODO.
	BYU = Self::letters_to_token(b"BYU"),
	
	/// TODO.
	BYV = Self::letters_to_token(b"BYV"),
	
	/// TODO.
	BYW = Self::letters_to_token(b"BYW"),
	
	/// TODO.
	BYX = Self::letters_to_token(b"BYX"),
	
	/// TODO.
	BYY = Self::letters_to_token(b"BYY"),
	
	/// TODO.
	BYZ = Self::letters_to_token(b"BYZ"),
	
	/// TODO.
	BZA = Self::letters_to_token(b"BZA"),
	
	/// TODO.
	BZB = Self::letters_to_token(b"BZB"),
	
	/// TODO.
	BZC = Self::letters_to_token(b"BZC"),
	
	/// TODO.
	BZD = Self::letters_to_token(b"BZD"),
	
	/// TODO.
	BZE = Self::letters_to_token(b"BZE"),
	
	/// TODO.
	BZF = Self::letters_to_token(b"BZF"),
	
	/// TODO.
	BZG = Self::letters_to_token(b"BZG"),
	
	/// TODO.
	BZH = Self::letters_to_token(b"BZH"),
	
	/// TODO.
	BZI = Self::letters_to_token(b"BZI"),
	
	/// TODO.
	BZJ = Self::letters_to_token(b"BZJ"),
	
	/// TODO.
	BZK = Self::letters_to_token(b"BZK"),
	
	/// TODO.
	BZL = Self::letters_to_token(b"BZL"),
	
	/// TODO.
	BZM = Self::letters_to_token(b"BZM"),
	
	/// TODO.
	BZN = Self::letters_to_token(b"BZN"),
	
	/// TODO.
	BZO = Self::letters_to_token(b"BZO"),
	
	/// TODO.
	BZP = Self::letters_to_token(b"BZP"),
	
	/// TODO.
	BZQ = Self::letters_to_token(b"BZQ"),
	
	/// TODO.
	BZR = Self::letters_to_token(b"BZR"),
	
	/// TODO.
	BZS = Self::letters_to_token(b"BZS"),
	
	/// TODO.
	BZT = Self::letters_to_token(b"BZT"),
	
	/// TODO.
	BZU = Self::letters_to_token(b"BZU"),
	
	/// TODO.
	BZV = Self::letters_to_token(b"BZV"),
	
	/// TODO.
	BZW = Self::letters_to_token(b"BZW"),
	
	/// TODO.
	BZX = Self::letters_to_token(b"BZX"),
	
	/// TODO.
	BZY = Self::letters_to_token(b"BZY"),
	
	/// TODO.
	BZZ = Self::letters_to_token(b"BZZ"),
	
	/// TODO.
	CAA = Self::letters_to_token(b"CAA"),
	
	/// TODO.
	CAB = Self::letters_to_token(b"CAB"),
	
	/// TODO.
	CAC = Self::letters_to_token(b"CAC"),
	
	/// TODO.
	CAD = Self::letters_to_token(b"CAD"),
	
	/// TODO.
	CAE = Self::letters_to_token(b"CAE"),
	
	/// TODO.
	CAF = Self::letters_to_token(b"CAF"),
	
	/// TODO.
	CAG = Self::letters_to_token(b"CAG"),
	
	/// TODO.
	CAH = Self::letters_to_token(b"CAH"),
	
	/// TODO.
	CAI = Self::letters_to_token(b"CAI"),
	
	/// TODO.
	CAJ = Self::letters_to_token(b"CAJ"),
	
	/// TODO.
	CAK = Self::letters_to_token(b"CAK"),
	
	/// TODO.
	CAL = Self::letters_to_token(b"CAL"),
	
	/// TODO.
	CAM = Self::letters_to_token(b"CAM"),
	
	/// TODO.
	CAN = Self::letters_to_token(b"CAN"),
	
	/// TODO.
	CAO = Self::letters_to_token(b"CAO"),
	
	/// TODO.
	CAP = Self::letters_to_token(b"CAP"),
	
	/// TODO.
	CAQ = Self::letters_to_token(b"CAQ"),
	
	/// TODO.
	CAR = Self::letters_to_token(b"CAR"),
	
	/// TODO.
	CAS = Self::letters_to_token(b"CAS"),
	
	/// TODO.
	CAT = Self::letters_to_token(b"CAT"),
	
	/// TODO.
	CAU = Self::letters_to_token(b"CAU"),
	
	/// TODO.
	CAV = Self::letters_to_token(b"CAV"),
	
	/// TODO.
	CAW = Self::letters_to_token(b"CAW"),
	
	/// TODO.
	CAX = Self::letters_to_token(b"CAX"),
	
	/// TODO.
	CAY = Self::letters_to_token(b"CAY"),
	
	/// TODO.
	CAZ = Self::letters_to_token(b"CAZ"),
	
	/// TODO.
	CBA = Self::letters_to_token(b"CBA"),
	
	/// TODO.
	CBB = Self::letters_to_token(b"CBB"),
	
	/// TODO.
	CBC = Self::letters_to_token(b"CBC"),
	
	/// TODO.
	CBD = Self::letters_to_token(b"CBD"),
	
	/// TODO.
	CBE = Self::letters_to_token(b"CBE"),
	
	/// TODO.
	CBF = Self::letters_to_token(b"CBF"),
	
	/// TODO.
	CBG = Self::letters_to_token(b"CBG"),
	
	/// TODO.
	CBH = Self::letters_to_token(b"CBH"),
	
	/// TODO.
	CBI = Self::letters_to_token(b"CBI"),
	
	/// TODO.
	CBJ = Self::letters_to_token(b"CBJ"),
	
	/// TODO.
	CBK = Self::letters_to_token(b"CBK"),
	
	/// TODO.
	CBL = Self::letters_to_token(b"CBL"),
	
	/// TODO.
	CBM = Self::letters_to_token(b"CBM"),
	
	/// TODO.
	CBN = Self::letters_to_token(b"CBN"),
	
	/// TODO.
	CBO = Self::letters_to_token(b"CBO"),
	
	/// TODO.
	CBP = Self::letters_to_token(b"CBP"),
	
	/// TODO.
	CBQ = Self::letters_to_token(b"CBQ"),
	
	/// TODO.
	CBR = Self::letters_to_token(b"CBR"),
	
	/// TODO.
	CBS = Self::letters_to_token(b"CBS"),
	
	/// TODO.
	CBT = Self::letters_to_token(b"CBT"),
	
	/// TODO.
	CBU = Self::letters_to_token(b"CBU"),
	
	/// TODO.
	CBV = Self::letters_to_token(b"CBV"),
	
	/// TODO.
	CBW = Self::letters_to_token(b"CBW"),
	
	/// TODO.
	CBX = Self::letters_to_token(b"CBX"),
	
	/// TODO.
	CBY = Self::letters_to_token(b"CBY"),
	
	/// TODO.
	CBZ = Self::letters_to_token(b"CBZ"),
	
	/// TODO.
	CCA = Self::letters_to_token(b"CCA"),
	
	/// TODO.
	CCB = Self::letters_to_token(b"CCB"),
	
	/// TODO.
	CCC = Self::letters_to_token(b"CCC"),
	
	/// TODO.
	CCD = Self::letters_to_token(b"CCD"),
	
	/// TODO.
	CCE = Self::letters_to_token(b"CCE"),
	
	/// TODO.
	CCF = Self::letters_to_token(b"CCF"),
	
	/// TODO.
	CCG = Self::letters_to_token(b"CCG"),
	
	/// TODO.
	CCH = Self::letters_to_token(b"CCH"),
	
	/// TODO.
	CCI = Self::letters_to_token(b"CCI"),
	
	/// TODO.
	CCJ = Self::letters_to_token(b"CCJ"),
	
	/// TODO.
	CCK = Self::letters_to_token(b"CCK"),
	
	/// TODO.
	CCL = Self::letters_to_token(b"CCL"),
	
	/// TODO.
	CCM = Self::letters_to_token(b"CCM"),
	
	/// TODO.
	CCN = Self::letters_to_token(b"CCN"),
	
	/// TODO.
	CCO = Self::letters_to_token(b"CCO"),
	
	/// TODO.
	CCP = Self::letters_to_token(b"CCP"),
	
	/// TODO.
	CCQ = Self::letters_to_token(b"CCQ"),
	
	/// TODO.
	CCR = Self::letters_to_token(b"CCR"),
	
	/// TODO.
	CCS = Self::letters_to_token(b"CCS"),
	
	/// TODO.
	CCT = Self::letters_to_token(b"CCT"),
	
	/// TODO.
	CCU = Self::letters_to_token(b"CCU"),
	
	/// TODO.
	CCV = Self::letters_to_token(b"CCV"),
	
	/// TODO.
	CCW = Self::letters_to_token(b"CCW"),
	
	/// TODO.
	CCX = Self::letters_to_token(b"CCX"),
	
	/// TODO.
	CCY = Self::letters_to_token(b"CCY"),
	
	/// TODO.
	CCZ = Self::letters_to_token(b"CCZ"),
	
	/// TODO.
	CDA = Self::letters_to_token(b"CDA"),
	
	/// TODO.
	CDB = Self::letters_to_token(b"CDB"),
	
	/// TODO.
	CDC = Self::letters_to_token(b"CDC"),
	
	/// TODO.
	CDD = Self::letters_to_token(b"CDD"),
	
	/// TODO.
	CDE = Self::letters_to_token(b"CDE"),
	
	/// TODO.
	CDF = Self::letters_to_token(b"CDF"),
	
	/// TODO.
	CDG = Self::letters_to_token(b"CDG"),
	
	/// TODO.
	CDH = Self::letters_to_token(b"CDH"),
	
	/// TODO.
	CDI = Self::letters_to_token(b"CDI"),
	
	/// TODO.
	CDJ = Self::letters_to_token(b"CDJ"),
	
	/// TODO.
	CDK = Self::letters_to_token(b"CDK"),
	
	/// TODO.
	CDL = Self::letters_to_token(b"CDL"),
	
	/// TODO.
	CDM = Self::letters_to_token(b"CDM"),
	
	/// TODO.
	CDN = Self::letters_to_token(b"CDN"),
	
	/// TODO.
	CDO = Self::letters_to_token(b"CDO"),
	
	/// TODO.
	CDP = Self::letters_to_token(b"CDP"),
	
	/// TODO.
	CDQ = Self::letters_to_token(b"CDQ"),
	
	/// TODO.
	CDR = Self::letters_to_token(b"CDR"),
	
	/// TODO.
	CDS = Self::letters_to_token(b"CDS"),
	
	/// TODO.
	CDT = Self::letters_to_token(b"CDT"),
	
	/// TODO.
	CDU = Self::letters_to_token(b"CDU"),
	
	/// TODO.
	CDV = Self::letters_to_token(b"CDV"),
	
	/// TODO.
	CDW = Self::letters_to_token(b"CDW"),
	
	/// TODO.
	CDX = Self::letters_to_token(b"CDX"),
	
	/// TODO.
	CDY = Self::letters_to_token(b"CDY"),
	
	/// TODO.
	CDZ = Self::letters_to_token(b"CDZ"),
	
	/// TODO.
	CEA = Self::letters_to_token(b"CEA"),
	
	/// TODO.
	CEB = Self::letters_to_token(b"CEB"),
	
	/// TODO.
	CEC = Self::letters_to_token(b"CEC"),
	
	/// TODO.
	CED = Self::letters_to_token(b"CED"),
	
	/// TODO.
	CEE = Self::letters_to_token(b"CEE"),
	
	/// TODO.
	CEF = Self::letters_to_token(b"CEF"),
	
	/// TODO.
	CEG = Self::letters_to_token(b"CEG"),
	
	/// TODO.
	CEH = Self::letters_to_token(b"CEH"),
	
	/// TODO.
	CEI = Self::letters_to_token(b"CEI"),
	
	/// TODO.
	CEJ = Self::letters_to_token(b"CEJ"),
	
	/// TODO.
	CEK = Self::letters_to_token(b"CEK"),
	
	/// TODO.
	CEL = Self::letters_to_token(b"CEL"),
	
	/// TODO.
	CEM = Self::letters_to_token(b"CEM"),
	
	/// TODO.
	CEN = Self::letters_to_token(b"CEN"),
	
	/// TODO.
	CEO = Self::letters_to_token(b"CEO"),
	
	/// TODO.
	CEP = Self::letters_to_token(b"CEP"),
	
	/// TODO.
	CEQ = Self::letters_to_token(b"CEQ"),
	
	/// TODO.
	CER = Self::letters_to_token(b"CER"),
	
	/// TODO.
	CES = Self::letters_to_token(b"CES"),
	
	/// TODO.
	CET = Self::letters_to_token(b"CET"),
	
	/// TODO.
	CEU = Self::letters_to_token(b"CEU"),
	
	/// TODO.
	CEV = Self::letters_to_token(b"CEV"),
	
	/// TODO.
	CEW = Self::letters_to_token(b"CEW"),
	
	/// TODO.
	CEX = Self::letters_to_token(b"CEX"),
	
	/// TODO.
	CEY = Self::letters_to_token(b"CEY"),
	
	/// TODO.
	CEZ = Self::letters_to_token(b"CEZ"),
	
	/// TODO.
	CFA = Self::letters_to_token(b"CFA"),
	
	/// TODO.
	CFB = Self::letters_to_token(b"CFB"),
	
	/// TODO.
	CFC = Self::letters_to_token(b"CFC"),
	
	/// TODO.
	CFD = Self::letters_to_token(b"CFD"),
	
	/// TODO.
	CFE = Self::letters_to_token(b"CFE"),
	
	/// TODO.
	CFF = Self::letters_to_token(b"CFF"),
	
	/// TODO.
	CFG = Self::letters_to_token(b"CFG"),
	
	/// TODO.
	CFH = Self::letters_to_token(b"CFH"),
	
	/// TODO.
	CFI = Self::letters_to_token(b"CFI"),
	
	/// TODO.
	CFJ = Self::letters_to_token(b"CFJ"),
	
	/// TODO.
	CFK = Self::letters_to_token(b"CFK"),
	
	/// TODO.
	CFL = Self::letters_to_token(b"CFL"),
	
	/// TODO.
	CFM = Self::letters_to_token(b"CFM"),
	
	/// TODO.
	CFN = Self::letters_to_token(b"CFN"),
	
	/// TODO.
	CFO = Self::letters_to_token(b"CFO"),
	
	/// TODO.
	CFP = Self::letters_to_token(b"CFP"),
	
	/// TODO.
	CFQ = Self::letters_to_token(b"CFQ"),
	
	/// TODO.
	CFR = Self::letters_to_token(b"CFR"),
	
	/// TODO.
	CFS = Self::letters_to_token(b"CFS"),
	
	/// TODO.
	CFT = Self::letters_to_token(b"CFT"),
	
	/// TODO.
	CFU = Self::letters_to_token(b"CFU"),
	
	/// TODO.
	CFV = Self::letters_to_token(b"CFV"),
	
	/// TODO.
	CFW = Self::letters_to_token(b"CFW"),
	
	/// TODO.
	CFX = Self::letters_to_token(b"CFX"),
	
	/// TODO.
	CFY = Self::letters_to_token(b"CFY"),
	
	/// TODO.
	CFZ = Self::letters_to_token(b"CFZ"),
	
	/// TODO.
	CGA = Self::letters_to_token(b"CGA"),
	
	/// TODO.
	CGB = Self::letters_to_token(b"CGB"),
	
	/// TODO.
	CGC = Self::letters_to_token(b"CGC"),
	
	/// TODO.
	CGD = Self::letters_to_token(b"CGD"),
	
	/// TODO.
	CGE = Self::letters_to_token(b"CGE"),
	
	/// TODO.
	CGF = Self::letters_to_token(b"CGF"),
	
	/// TODO.
	CGG = Self::letters_to_token(b"CGG"),
	
	/// TODO.
	CGH = Self::letters_to_token(b"CGH"),
	
	/// TODO.
	CGI = Self::letters_to_token(b"CGI"),
	
	/// TODO.
	CGJ = Self::letters_to_token(b"CGJ"),
	
	/// TODO.
	CGK = Self::letters_to_token(b"CGK"),
	
	/// TODO.
	CGL = Self::letters_to_token(b"CGL"),
	
	/// TODO.
	CGM = Self::letters_to_token(b"CGM"),
	
	/// TODO.
	CGN = Self::letters_to_token(b"CGN"),
	
	/// TODO.
	CGO = Self::letters_to_token(b"CGO"),
	
	/// TODO.
	CGP = Self::letters_to_token(b"CGP"),
	
	/// TODO.
	CGQ = Self::letters_to_token(b"CGQ"),
	
	/// TODO.
	CGR = Self::letters_to_token(b"CGR"),
	
	/// TODO.
	CGS = Self::letters_to_token(b"CGS"),
	
	/// TODO.
	CGT = Self::letters_to_token(b"CGT"),
	
	/// TODO.
	CGU = Self::letters_to_token(b"CGU"),
	
	/// TODO.
	CGV = Self::letters_to_token(b"CGV"),
	
	/// TODO.
	CGW = Self::letters_to_token(b"CGW"),
	
	/// TODO.
	CGX = Self::letters_to_token(b"CGX"),
	
	/// TODO.
	CGY = Self::letters_to_token(b"CGY"),
	
	/// TODO.
	CGZ = Self::letters_to_token(b"CGZ"),
	
	/// TODO.
	CHA = Self::letters_to_token(b"CHA"),
	
	/// TODO.
	CHB = Self::letters_to_token(b"CHB"),
	
	/// TODO.
	CHC = Self::letters_to_token(b"CHC"),
	
	/// TODO.
	CHD = Self::letters_to_token(b"CHD"),
	
	/// TODO.
	CHE = Self::letters_to_token(b"CHE"),
	
	/// TODO.
	CHF = Self::letters_to_token(b"CHF"),
	
	/// TODO.
	CHG = Self::letters_to_token(b"CHG"),
	
	/// TODO.
	CHH = Self::letters_to_token(b"CHH"),
	
	/// TODO.
	CHI = Self::letters_to_token(b"CHI"),
	
	/// TODO.
	CHJ = Self::letters_to_token(b"CHJ"),
	
	/// TODO.
	CHK = Self::letters_to_token(b"CHK"),
	
	/// TODO.
	CHL = Self::letters_to_token(b"CHL"),
	
	/// TODO.
	CHM = Self::letters_to_token(b"CHM"),
	
	/// TODO.
	CHN = Self::letters_to_token(b"CHN"),
	
	/// TODO.
	CHO = Self::letters_to_token(b"CHO"),
	
	/// TODO.
	CHP = Self::letters_to_token(b"CHP"),
	
	/// TODO.
	CHQ = Self::letters_to_token(b"CHQ"),
	
	/// TODO.
	CHR = Self::letters_to_token(b"CHR"),
	
	/// TODO.
	CHS = Self::letters_to_token(b"CHS"),
	
	/// TODO.
	CHT = Self::letters_to_token(b"CHT"),
	
	/// TODO.
	CHU = Self::letters_to_token(b"CHU"),
	
	/// TODO.
	CHV = Self::letters_to_token(b"CHV"),
	
	/// TODO.
	CHW = Self::letters_to_token(b"CHW"),
	
	/// TODO.
	CHX = Self::letters_to_token(b"CHX"),
	
	/// TODO.
	CHY = Self::letters_to_token(b"CHY"),
	
	/// TODO.
	CHZ = Self::letters_to_token(b"CHZ"),
	
	/// TODO.
	CIA = Self::letters_to_token(b"CIA"),
	
	/// TODO.
	CIB = Self::letters_to_token(b"CIB"),
	
	/// TODO.
	CIC = Self::letters_to_token(b"CIC"),
	
	/// TODO.
	CID = Self::letters_to_token(b"CID"),
	
	/// TODO.
	CIE = Self::letters_to_token(b"CIE"),
	
	/// TODO.
	CIF = Self::letters_to_token(b"CIF"),
	
	/// TODO.
	CIG = Self::letters_to_token(b"CIG"),
	
	/// TODO.
	CIH = Self::letters_to_token(b"CIH"),
	
	/// TODO.
	CII = Self::letters_to_token(b"CII"),
	
	/// TODO.
	CIJ = Self::letters_to_token(b"CIJ"),
	
	/// TODO.
	CIK = Self::letters_to_token(b"CIK"),
	
	/// TODO.
	CIL = Self::letters_to_token(b"CIL"),
	
	/// TODO.
	CIM = Self::letters_to_token(b"CIM"),
	
	/// TODO.
	CIN = Self::letters_to_token(b"CIN"),
	
	/// TODO.
	CIO = Self::letters_to_token(b"CIO"),
	
	/// TODO.
	CIP = Self::letters_to_token(b"CIP"),
	
	/// TODO.
	CIQ = Self::letters_to_token(b"CIQ"),
	
	/// TODO.
	CIR = Self::letters_to_token(b"CIR"),
	
	/// TODO.
	CIS = Self::letters_to_token(b"CIS"),
	
	/// TODO.
	CIT = Self::letters_to_token(b"CIT"),
	
	/// TODO.
	CIU = Self::letters_to_token(b"CIU"),
	
	/// TODO.
	CIV = Self::letters_to_token(b"CIV"),
	
	/// TODO.
	CIW = Self::letters_to_token(b"CIW"),
	
	/// TODO.
	CIX = Self::letters_to_token(b"CIX"),
	
	/// TODO.
	CIY = Self::letters_to_token(b"CIY"),
	
	/// TODO.
	CIZ = Self::letters_to_token(b"CIZ"),
	
	/// TODO.
	CJA = Self::letters_to_token(b"CJA"),
	
	/// TODO.
	CJB = Self::letters_to_token(b"CJB"),
	
	/// TODO.
	CJC = Self::letters_to_token(b"CJC"),
	
	/// TODO.
	CJD = Self::letters_to_token(b"CJD"),
	
	/// TODO.
	CJE = Self::letters_to_token(b"CJE"),
	
	/// TODO.
	CJF = Self::letters_to_token(b"CJF"),
	
	/// TODO.
	CJG = Self::letters_to_token(b"CJG"),
	
	/// TODO.
	CJH = Self::letters_to_token(b"CJH"),
	
	/// TODO.
	CJI = Self::letters_to_token(b"CJI"),
	
	/// TODO.
	CJJ = Self::letters_to_token(b"CJJ"),
	
	/// TODO.
	CJK = Self::letters_to_token(b"CJK"),
	
	/// TODO.
	CJL = Self::letters_to_token(b"CJL"),
	
	/// TODO.
	CJM = Self::letters_to_token(b"CJM"),
	
	/// TODO.
	CJN = Self::letters_to_token(b"CJN"),
	
	/// TODO.
	CJO = Self::letters_to_token(b"CJO"),
	
	/// TODO.
	CJP = Self::letters_to_token(b"CJP"),
	
	/// TODO.
	CJQ = Self::letters_to_token(b"CJQ"),
	
	/// TODO.
	CJR = Self::letters_to_token(b"CJR"),
	
	/// TODO.
	CJS = Self::letters_to_token(b"CJS"),
	
	/// TODO.
	CJT = Self::letters_to_token(b"CJT"),
	
	/// TODO.
	CJU = Self::letters_to_token(b"CJU"),
	
	/// TODO.
	CJV = Self::letters_to_token(b"CJV"),
	
	/// TODO.
	CJW = Self::letters_to_token(b"CJW"),
	
	/// TODO.
	CJX = Self::letters_to_token(b"CJX"),
	
	/// TODO.
	CJY = Self::letters_to_token(b"CJY"),
	
	/// TODO.
	CJZ = Self::letters_to_token(b"CJZ"),
	
	/// TODO.
	CKA = Self::letters_to_token(b"CKA"),
	
	/// TODO.
	CKB = Self::letters_to_token(b"CKB"),
	
	/// TODO.
	CKC = Self::letters_to_token(b"CKC"),
	
	/// TODO.
	CKD = Self::letters_to_token(b"CKD"),
	
	/// TODO.
	CKE = Self::letters_to_token(b"CKE"),
	
	/// TODO.
	CKF = Self::letters_to_token(b"CKF"),
	
	/// TODO.
	CKG = Self::letters_to_token(b"CKG"),
	
	/// TODO.
	CKH = Self::letters_to_token(b"CKH"),
	
	/// TODO.
	CKI = Self::letters_to_token(b"CKI"),
	
	/// TODO.
	CKJ = Self::letters_to_token(b"CKJ"),
	
	/// TODO.
	CKK = Self::letters_to_token(b"CKK"),
	
	/// TODO.
	CKL = Self::letters_to_token(b"CKL"),
	
	/// TODO.
	CKM = Self::letters_to_token(b"CKM"),
	
	/// TODO.
	CKN = Self::letters_to_token(b"CKN"),
	
	/// TODO.
	CKO = Self::letters_to_token(b"CKO"),
	
	/// TODO.
	CKP = Self::letters_to_token(b"CKP"),
	
	/// TODO.
	CKQ = Self::letters_to_token(b"CKQ"),
	
	/// TODO.
	CKR = Self::letters_to_token(b"CKR"),
	
	/// TODO.
	CKS = Self::letters_to_token(b"CKS"),
	
	/// TODO.
	CKT = Self::letters_to_token(b"CKT"),
	
	/// TODO.
	CKU = Self::letters_to_token(b"CKU"),
	
	/// TODO.
	CKV = Self::letters_to_token(b"CKV"),
	
	/// TODO.
	CKW = Self::letters_to_token(b"CKW"),
	
	/// TODO.
	CKX = Self::letters_to_token(b"CKX"),
	
	/// TODO.
	CKY = Self::letters_to_token(b"CKY"),
	
	/// TODO.
	CKZ = Self::letters_to_token(b"CKZ"),
	
	/// TODO.
	CLA = Self::letters_to_token(b"CLA"),
	
	/// TODO.
	CLB = Self::letters_to_token(b"CLB"),
	
	/// TODO.
	CLC = Self::letters_to_token(b"CLC"),
	
	/// TODO.
	CLD = Self::letters_to_token(b"CLD"),
	
	/// TODO.
	CLE = Self::letters_to_token(b"CLE"),
	
	/// TODO.
	CLF = Self::letters_to_token(b"CLF"),
	
	/// TODO.
	CLG = Self::letters_to_token(b"CLG"),
	
	/// TODO.
	CLH = Self::letters_to_token(b"CLH"),
	
	/// TODO.
	CLI = Self::letters_to_token(b"CLI"),
	
	/// TODO.
	CLJ = Self::letters_to_token(b"CLJ"),
	
	/// TODO.
	CLK = Self::letters_to_token(b"CLK"),
	
	/// TODO.
	CLL = Self::letters_to_token(b"CLL"),
	
	/// TODO.
	CLM = Self::letters_to_token(b"CLM"),
	
	/// TODO.
	CLN = Self::letters_to_token(b"CLN"),
	
	/// TODO.
	CLO = Self::letters_to_token(b"CLO"),
	
	/// TODO.
	CLP = Self::letters_to_token(b"CLP"),
	
	/// TODO.
	CLQ = Self::letters_to_token(b"CLQ"),
	
	/// TODO.
	CLR = Self::letters_to_token(b"CLR"),
	
	/// TODO.
	CLS = Self::letters_to_token(b"CLS"),
	
	/// TODO.
	CLT = Self::letters_to_token(b"CLT"),
	
	/// TODO.
	CLU = Self::letters_to_token(b"CLU"),
	
	/// TODO.
	CLV = Self::letters_to_token(b"CLV"),
	
	/// TODO.
	CLW = Self::letters_to_token(b"CLW"),
	
	/// TODO.
	CLX = Self::letters_to_token(b"CLX"),
	
	/// TODO.
	CLY = Self::letters_to_token(b"CLY"),
	
	/// TODO.
	CLZ = Self::letters_to_token(b"CLZ"),
	
	/// TODO.
	CMA = Self::letters_to_token(b"CMA"),
	
	/// TODO.
	CMB = Self::letters_to_token(b"CMB"),
	
	/// TODO.
	CMC = Self::letters_to_token(b"CMC"),
	
	/// TODO.
	CMD = Self::letters_to_token(b"CMD"),
	
	/// TODO.
	CME = Self::letters_to_token(b"CME"),
	
	/// TODO.
	CMF = Self::letters_to_token(b"CMF"),
	
	/// TODO.
	CMG = Self::letters_to_token(b"CMG"),
	
	/// TODO.
	CMH = Self::letters_to_token(b"CMH"),
	
	/// TODO.
	CMI = Self::letters_to_token(b"CMI"),
	
	/// TODO.
	CMJ = Self::letters_to_token(b"CMJ"),
	
	/// TODO.
	CMK = Self::letters_to_token(b"CMK"),
	
	/// TODO.
	CML = Self::letters_to_token(b"CML"),
	
	/// TODO.
	CMM = Self::letters_to_token(b"CMM"),
	
	/// TODO.
	CMN = Self::letters_to_token(b"CMN"),
	
	/// TODO.
	CMO = Self::letters_to_token(b"CMO"),
	
	/// TODO.
	CMP = Self::letters_to_token(b"CMP"),
	
	/// TODO.
	CMQ = Self::letters_to_token(b"CMQ"),
	
	/// TODO.
	CMR = Self::letters_to_token(b"CMR"),
	
	/// TODO.
	CMS = Self::letters_to_token(b"CMS"),
	
	/// TODO.
	CMT = Self::letters_to_token(b"CMT"),
	
	/// TODO.
	CMU = Self::letters_to_token(b"CMU"),
	
	/// TODO.
	CMV = Self::letters_to_token(b"CMV"),
	
	/// TODO.
	CMW = Self::letters_to_token(b"CMW"),
	
	/// TODO.
	CMX = Self::letters_to_token(b"CMX"),
	
	/// TODO.
	CMY = Self::letters_to_token(b"CMY"),
	
	/// TODO.
	CMZ = Self::letters_to_token(b"CMZ"),
	
	/// TODO.
	CNA = Self::letters_to_token(b"CNA"),
	
	/// TODO.
	CNB = Self::letters_to_token(b"CNB"),
	
	/// TODO.
	CNC = Self::letters_to_token(b"CNC"),
	
	/// TODO.
	CND = Self::letters_to_token(b"CND"),
	
	/// TODO.
	CNE = Self::letters_to_token(b"CNE"),
	
	/// TODO.
	CNF = Self::letters_to_token(b"CNF"),
	
	/// TODO.
	CNG = Self::letters_to_token(b"CNG"),
	
	/// TODO.
	CNH = Self::letters_to_token(b"CNH"),
	
	/// TODO.
	CNI = Self::letters_to_token(b"CNI"),
	
	/// TODO.
	CNJ = Self::letters_to_token(b"CNJ"),
	
	/// TODO.
	CNK = Self::letters_to_token(b"CNK"),
	
	/// TODO.
	CNL = Self::letters_to_token(b"CNL"),
	
	/// TODO.
	CNM = Self::letters_to_token(b"CNM"),
	
	/// TODO.
	CNN = Self::letters_to_token(b"CNN"),
	
	/// TODO.
	CNO = Self::letters_to_token(b"CNO"),
	
	/// TODO.
	CNP = Self::letters_to_token(b"CNP"),
	
	/// TODO.
	CNQ = Self::letters_to_token(b"CNQ"),
	
	/// TODO.
	CNR = Self::letters_to_token(b"CNR"),
	
	/// TODO.
	CNS = Self::letters_to_token(b"CNS"),
	
	/// TODO.
	CNT = Self::letters_to_token(b"CNT"),
	
	/// TODO.
	CNU = Self::letters_to_token(b"CNU"),
	
	/// TODO.
	CNV = Self::letters_to_token(b"CNV"),
	
	/// TODO.
	CNW = Self::letters_to_token(b"CNW"),
	
	/// TODO.
	CNX = Self::letters_to_token(b"CNX"),
	
	/// TODO.
	CNY = Self::letters_to_token(b"CNY"),
	
	/// TODO.
	CNZ = Self::letters_to_token(b"CNZ"),
	
	/// TODO.
	COA = Self::letters_to_token(b"COA"),
	
	/// TODO.
	COB = Self::letters_to_token(b"COB"),
	
	/// TODO.
	COC = Self::letters_to_token(b"COC"),
	
	/// TODO.
	COD = Self::letters_to_token(b"COD"),
	
	/// TODO.
	COE = Self::letters_to_token(b"COE"),
	
	/// TODO.
	COF = Self::letters_to_token(b"COF"),
	
	/// TODO.
	COG = Self::letters_to_token(b"COG"),
	
	/// TODO.
	COH = Self::letters_to_token(b"COH"),
	
	/// TODO.
	COI = Self::letters_to_token(b"COI"),
	
	/// TODO.
	COJ = Self::letters_to_token(b"COJ"),
	
	/// TODO.
	COK = Self::letters_to_token(b"COK"),
	
	/// TODO.
	COL = Self::letters_to_token(b"COL"),
	
	/// TODO.
	COM = Self::letters_to_token(b"COM"),
	
	/// TODO.
	CON = Self::letters_to_token(b"CON"),
	
	/// TODO.
	COO = Self::letters_to_token(b"COO"),
	
	/// TODO.
	COP = Self::letters_to_token(b"COP"),
	
	/// TODO.
	COQ = Self::letters_to_token(b"COQ"),
	
	/// TODO.
	COR = Self::letters_to_token(b"COR"),
	
	/// TODO.
	COS = Self::letters_to_token(b"COS"),
	
	/// TODO.
	COT = Self::letters_to_token(b"COT"),
	
	/// TODO.
	COU = Self::letters_to_token(b"COU"),
	
	/// TODO.
	COV = Self::letters_to_token(b"COV"),
	
	/// TODO.
	COW = Self::letters_to_token(b"COW"),
	
	/// TODO.
	COX = Self::letters_to_token(b"COX"),
	
	/// TODO.
	COY = Self::letters_to_token(b"COY"),
	
	/// TODO.
	COZ = Self::letters_to_token(b"COZ"),
	
	/// TODO.
	CPA = Self::letters_to_token(b"CPA"),
	
	/// TODO.
	CPB = Self::letters_to_token(b"CPB"),
	
	/// TODO.
	CPC = Self::letters_to_token(b"CPC"),
	
	/// TODO.
	CPD = Self::letters_to_token(b"CPD"),
	
	/// TODO.
	CPE = Self::letters_to_token(b"CPE"),
	
	/// TODO.
	CPF = Self::letters_to_token(b"CPF"),
	
	/// TODO.
	CPG = Self::letters_to_token(b"CPG"),
	
	/// TODO.
	CPH = Self::letters_to_token(b"CPH"),
	
	/// TODO.
	CPI = Self::letters_to_token(b"CPI"),
	
	/// TODO.
	CPJ = Self::letters_to_token(b"CPJ"),
	
	/// TODO.
	CPK = Self::letters_to_token(b"CPK"),
	
	/// TODO.
	CPL = Self::letters_to_token(b"CPL"),
	
	/// TODO.
	CPM = Self::letters_to_token(b"CPM"),
	
	/// TODO.
	CPN = Self::letters_to_token(b"CPN"),
	
	/// TODO.
	CPO = Self::letters_to_token(b"CPO"),
	
	/// TODO.
	CPP = Self::letters_to_token(b"CPP"),
	
	/// TODO.
	CPQ = Self::letters_to_token(b"CPQ"),
	
	/// TODO.
	CPR = Self::letters_to_token(b"CPR"),
	
	/// TODO.
	CPS = Self::letters_to_token(b"CPS"),
	
	/// Exceptional reservation.
	///
	/// Clipperton Island.
	CPT = Self::letters_to_token(b"CPT"),
	
	/// TODO.
	CPU = Self::letters_to_token(b"CPU"),
	
	/// TODO.
	CPV = Self::letters_to_token(b"CPV"),
	
	/// TODO.
	CPW = Self::letters_to_token(b"CPW"),
	
	/// TODO.
	CPX = Self::letters_to_token(b"CPX"),
	
	/// TODO.
	CPY = Self::letters_to_token(b"CPY"),
	
	/// TODO.
	CPZ = Self::letters_to_token(b"CPZ"),
	
	/// TODO.
	CQA = Self::letters_to_token(b"CQA"),
	
	/// TODO.
	CQB = Self::letters_to_token(b"CQB"),
	
	/// TODO.
	CQC = Self::letters_to_token(b"CQC"),
	
	/// TODO.
	CQD = Self::letters_to_token(b"CQD"),
	
	/// TODO.
	CQE = Self::letters_to_token(b"CQE"),
	
	/// TODO.
	CQF = Self::letters_to_token(b"CQF"),
	
	/// TODO.
	CQG = Self::letters_to_token(b"CQG"),
	
	/// TODO.
	CQH = Self::letters_to_token(b"CQH"),
	
	/// TODO.
	CQI = Self::letters_to_token(b"CQI"),
	
	/// TODO.
	CQJ = Self::letters_to_token(b"CQJ"),
	
	/// TODO.
	CQK = Self::letters_to_token(b"CQK"),
	
	/// TODO.
	CQL = Self::letters_to_token(b"CQL"),
	
	/// TODO.
	CQM = Self::letters_to_token(b"CQM"),
	
	/// TODO.
	CQN = Self::letters_to_token(b"CQN"),
	
	/// TODO.
	CQO = Self::letters_to_token(b"CQO"),
	
	/// TODO.
	CQP = Self::letters_to_token(b"CQP"),
	
	/// TODO.
	CQQ = Self::letters_to_token(b"CQQ"),
	
	/// TODO.
	CQR = Self::letters_to_token(b"CQR"),
	
	/// TODO.
	CQS = Self::letters_to_token(b"CQS"),
	
	/// TODO.
	CQT = Self::letters_to_token(b"CQT"),
	
	/// TODO.
	CQU = Self::letters_to_token(b"CQU"),
	
	/// TODO.
	CQV = Self::letters_to_token(b"CQV"),
	
	/// TODO.
	CQW = Self::letters_to_token(b"CQW"),
	
	/// TODO.
	CQX = Self::letters_to_token(b"CQX"),
	
	/// TODO.
	CQY = Self::letters_to_token(b"CQY"),
	
	/// TODO.
	CQZ = Self::letters_to_token(b"CQZ"),
	
	/// TODO.
	CRA = Self::letters_to_token(b"CRA"),
	
	/// TODO.
	CRB = Self::letters_to_token(b"CRB"),
	
	/// TODO.
	CRC = Self::letters_to_token(b"CRC"),
	
	/// TODO.
	CRD = Self::letters_to_token(b"CRD"),
	
	/// TODO.
	CRE = Self::letters_to_token(b"CRE"),
	
	/// TODO.
	CRF = Self::letters_to_token(b"CRF"),
	
	/// TODO.
	CRG = Self::letters_to_token(b"CRG"),
	
	/// TODO.
	CRH = Self::letters_to_token(b"CRH"),
	
	/// TODO.
	CRI = Self::letters_to_token(b"CRI"),
	
	/// TODO.
	CRJ = Self::letters_to_token(b"CRJ"),
	
	/// TODO.
	CRK = Self::letters_to_token(b"CRK"),
	
	/// TODO.
	CRL = Self::letters_to_token(b"CRL"),
	
	/// TODO.
	CRM = Self::letters_to_token(b"CRM"),
	
	/// TODO.
	CRN = Self::letters_to_token(b"CRN"),
	
	/// TODO.
	CRO = Self::letters_to_token(b"CRO"),
	
	/// TODO.
	CRP = Self::letters_to_token(b"CRP"),
	
	/// TODO.
	CRQ = Self::letters_to_token(b"CRQ"),
	
	/// TODO.
	CRR = Self::letters_to_token(b"CRR"),
	
	/// TODO.
	CRS = Self::letters_to_token(b"CRS"),
	
	/// TODO.
	CRT = Self::letters_to_token(b"CRT"),
	
	/// TODO.
	CRU = Self::letters_to_token(b"CRU"),
	
	/// TODO.
	CRV = Self::letters_to_token(b"CRV"),
	
	/// TODO.
	CRW = Self::letters_to_token(b"CRW"),
	
	/// TODO.
	CRX = Self::letters_to_token(b"CRX"),
	
	/// TODO.
	CRY = Self::letters_to_token(b"CRY"),
	
	/// TODO.
	CRZ = Self::letters_to_token(b"CRZ"),
	
	/// TODO.
	CSA = Self::letters_to_token(b"CSA"),
	
	/// TODO.
	CSB = Self::letters_to_token(b"CSB"),
	
	/// TODO.
	CSC = Self::letters_to_token(b"CSC"),
	
	/// TODO.
	CSD = Self::letters_to_token(b"CSD"),
	
	/// TODO.
	CSE = Self::letters_to_token(b"CSE"),
	
	/// TODO.
	CSF = Self::letters_to_token(b"CSF"),
	
	/// TODO.
	CSG = Self::letters_to_token(b"CSG"),
	
	/// TODO.
	CSH = Self::letters_to_token(b"CSH"),
	
	/// TODO.
	CSI = Self::letters_to_token(b"CSI"),
	
	/// TODO.
	CSJ = Self::letters_to_token(b"CSJ"),
	
	/// TODO.
	CSK = Self::letters_to_token(b"CSK"),
	
	/// TODO.
	CSL = Self::letters_to_token(b"CSL"),
	
	/// TODO.
	CSM = Self::letters_to_token(b"CSM"),
	
	/// TODO.
	CSN = Self::letters_to_token(b"CSN"),
	
	/// TODO.
	CSO = Self::letters_to_token(b"CSO"),
	
	/// TODO.
	CSP = Self::letters_to_token(b"CSP"),
	
	/// TODO.
	CSQ = Self::letters_to_token(b"CSQ"),
	
	/// TODO.
	CSR = Self::letters_to_token(b"CSR"),
	
	/// TODO.
	CSS = Self::letters_to_token(b"CSS"),
	
	/// TODO.
	CST = Self::letters_to_token(b"CST"),
	
	/// TODO.
	CSU = Self::letters_to_token(b"CSU"),
	
	/// TODO.
	CSV = Self::letters_to_token(b"CSV"),
	
	/// TODO.
	CSW = Self::letters_to_token(b"CSW"),
	
	/// TODO.
	CSX = Self::letters_to_token(b"CSX"),
	
	/// TODO.
	CSY = Self::letters_to_token(b"CSY"),
	
	/// TODO.
	CSZ = Self::letters_to_token(b"CSZ"),
	
	/// TODO.
	CTA = Self::letters_to_token(b"CTA"),
	
	/// TODO.
	CTB = Self::letters_to_token(b"CTB"),
	
	/// TODO.
	CTC = Self::letters_to_token(b"CTC"),
	
	/// TODO.
	CTD = Self::letters_to_token(b"CTD"),
	
	/// TODO.
	CTE = Self::letters_to_token(b"CTE"),
	
	/// TODO.
	CTF = Self::letters_to_token(b"CTF"),
	
	/// TODO.
	CTG = Self::letters_to_token(b"CTG"),
	
	/// TODO.
	CTH = Self::letters_to_token(b"CTH"),
	
	/// TODO.
	CTI = Self::letters_to_token(b"CTI"),
	
	/// TODO.
	CTJ = Self::letters_to_token(b"CTJ"),
	
	/// TODO.
	CTK = Self::letters_to_token(b"CTK"),
	
	/// TODO.
	CTL = Self::letters_to_token(b"CTL"),
	
	/// TODO.
	CTM = Self::letters_to_token(b"CTM"),
	
	/// TODO.
	CTN = Self::letters_to_token(b"CTN"),
	
	/// TODO.
	CTO = Self::letters_to_token(b"CTO"),
	
	/// TODO.
	CTP = Self::letters_to_token(b"CTP"),
	
	/// TODO.
	CTQ = Self::letters_to_token(b"CTQ"),
	
	/// TODO.
	CTR = Self::letters_to_token(b"CTR"),
	
	/// TODO.
	CTS = Self::letters_to_token(b"CTS"),
	
	/// TODO.
	CTT = Self::letters_to_token(b"CTT"),
	
	/// TODO.
	CTU = Self::letters_to_token(b"CTU"),
	
	/// TODO.
	CTV = Self::letters_to_token(b"CTV"),
	
	/// TODO.
	CTW = Self::letters_to_token(b"CTW"),
	
	/// TODO.
	CTX = Self::letters_to_token(b"CTX"),
	
	/// TODO.
	CTY = Self::letters_to_token(b"CTY"),
	
	/// TODO.
	CTZ = Self::letters_to_token(b"CTZ"),
	
	/// TODO.
	CUA = Self::letters_to_token(b"CUA"),
	
	/// TODO.
	CUB = Self::letters_to_token(b"CUB"),
	
	/// TODO.
	CUC = Self::letters_to_token(b"CUC"),
	
	/// TODO.
	CUD = Self::letters_to_token(b"CUD"),
	
	/// TODO.
	CUE = Self::letters_to_token(b"CUE"),
	
	/// TODO.
	CUF = Self::letters_to_token(b"CUF"),
	
	/// TODO.
	CUG = Self::letters_to_token(b"CUG"),
	
	/// TODO.
	CUH = Self::letters_to_token(b"CUH"),
	
	/// TODO.
	CUI = Self::letters_to_token(b"CUI"),
	
	/// TODO.
	CUJ = Self::letters_to_token(b"CUJ"),
	
	/// TODO.
	CUK = Self::letters_to_token(b"CUK"),
	
	/// TODO.
	CUL = Self::letters_to_token(b"CUL"),
	
	/// TODO.
	CUM = Self::letters_to_token(b"CUM"),
	
	/// TODO.
	CUN = Self::letters_to_token(b"CUN"),
	
	/// TODO.
	CUO = Self::letters_to_token(b"CUO"),
	
	/// TODO.
	CUP = Self::letters_to_token(b"CUP"),
	
	/// TODO.
	CUQ = Self::letters_to_token(b"CUQ"),
	
	/// TODO.
	CUR = Self::letters_to_token(b"CUR"),
	
	/// TODO.
	CUS = Self::letters_to_token(b"CUS"),
	
	/// TODO.
	CUT = Self::letters_to_token(b"CUT"),
	
	/// TODO.
	CUU = Self::letters_to_token(b"CUU"),
	
	/// TODO.
	CUV = Self::letters_to_token(b"CUV"),
	
	/// TODO.
	CUW = Self::letters_to_token(b"CUW"),
	
	/// TODO.
	CUX = Self::letters_to_token(b"CUX"),
	
	/// TODO.
	CUY = Self::letters_to_token(b"CUY"),
	
	/// TODO.
	CUZ = Self::letters_to_token(b"CUZ"),
	
	/// TODO.
	CVA = Self::letters_to_token(b"CVA"),
	
	/// TODO.
	CVB = Self::letters_to_token(b"CVB"),
	
	/// TODO.
	CVC = Self::letters_to_token(b"CVC"),
	
	/// TODO.
	CVD = Self::letters_to_token(b"CVD"),
	
	/// TODO.
	CVE = Self::letters_to_token(b"CVE"),
	
	/// TODO.
	CVF = Self::letters_to_token(b"CVF"),
	
	/// TODO.
	CVG = Self::letters_to_token(b"CVG"),
	
	/// TODO.
	CVH = Self::letters_to_token(b"CVH"),
	
	/// TODO.
	CVI = Self::letters_to_token(b"CVI"),
	
	/// TODO.
	CVJ = Self::letters_to_token(b"CVJ"),
	
	/// TODO.
	CVK = Self::letters_to_token(b"CVK"),
	
	/// TODO.
	CVL = Self::letters_to_token(b"CVL"),
	
	/// TODO.
	CVM = Self::letters_to_token(b"CVM"),
	
	/// TODO.
	CVN = Self::letters_to_token(b"CVN"),
	
	/// TODO.
	CVO = Self::letters_to_token(b"CVO"),
	
	/// TODO.
	CVP = Self::letters_to_token(b"CVP"),
	
	/// TODO.
	CVQ = Self::letters_to_token(b"CVQ"),
	
	/// TODO.
	CVR = Self::letters_to_token(b"CVR"),
	
	/// TODO.
	CVS = Self::letters_to_token(b"CVS"),
	
	/// TODO.
	CVT = Self::letters_to_token(b"CVT"),
	
	/// TODO.
	CVU = Self::letters_to_token(b"CVU"),
	
	/// TODO.
	CVV = Self::letters_to_token(b"CVV"),
	
	/// TODO.
	CVW = Self::letters_to_token(b"CVW"),
	
	/// TODO.
	CVX = Self::letters_to_token(b"CVX"),
	
	/// TODO.
	CVY = Self::letters_to_token(b"CVY"),
	
	/// TODO.
	CVZ = Self::letters_to_token(b"CVZ"),
	
	/// TODO.
	CWA = Self::letters_to_token(b"CWA"),
	
	/// TODO.
	CWB = Self::letters_to_token(b"CWB"),
	
	/// TODO.
	CWC = Self::letters_to_token(b"CWC"),
	
	/// TODO.
	CWD = Self::letters_to_token(b"CWD"),
	
	/// TODO.
	CWE = Self::letters_to_token(b"CWE"),
	
	/// TODO.
	CWF = Self::letters_to_token(b"CWF"),
	
	/// TODO.
	CWG = Self::letters_to_token(b"CWG"),
	
	/// TODO.
	CWH = Self::letters_to_token(b"CWH"),
	
	/// TODO.
	CWI = Self::letters_to_token(b"CWI"),
	
	/// TODO.
	CWJ = Self::letters_to_token(b"CWJ"),
	
	/// TODO.
	CWK = Self::letters_to_token(b"CWK"),
	
	/// TODO.
	CWL = Self::letters_to_token(b"CWL"),
	
	/// TODO.
	CWM = Self::letters_to_token(b"CWM"),
	
	/// TODO.
	CWN = Self::letters_to_token(b"CWN"),
	
	/// TODO.
	CWO = Self::letters_to_token(b"CWO"),
	
	/// TODO.
	CWP = Self::letters_to_token(b"CWP"),
	
	/// TODO.
	CWQ = Self::letters_to_token(b"CWQ"),
	
	/// TODO.
	CWR = Self::letters_to_token(b"CWR"),
	
	/// TODO.
	CWS = Self::letters_to_token(b"CWS"),
	
	/// TODO.
	CWT = Self::letters_to_token(b"CWT"),
	
	/// TODO.
	CWU = Self::letters_to_token(b"CWU"),
	
	/// TODO.
	CWV = Self::letters_to_token(b"CWV"),
	
	/// TODO.
	CWW = Self::letters_to_token(b"CWW"),
	
	/// TODO.
	CWX = Self::letters_to_token(b"CWX"),
	
	/// TODO.
	CWY = Self::letters_to_token(b"CWY"),
	
	/// TODO.
	CWZ = Self::letters_to_token(b"CWZ"),
	
	/// TODO.
	CXA = Self::letters_to_token(b"CXA"),
	
	/// TODO.
	CXB = Self::letters_to_token(b"CXB"),
	
	/// TODO.
	CXC = Self::letters_to_token(b"CXC"),
	
	/// TODO.
	CXD = Self::letters_to_token(b"CXD"),
	
	/// TODO.
	CXE = Self::letters_to_token(b"CXE"),
	
	/// TODO.
	CXF = Self::letters_to_token(b"CXF"),
	
	/// TODO.
	CXG = Self::letters_to_token(b"CXG"),
	
	/// TODO.
	CXH = Self::letters_to_token(b"CXH"),
	
	/// TODO.
	CXI = Self::letters_to_token(b"CXI"),
	
	/// TODO.
	CXJ = Self::letters_to_token(b"CXJ"),
	
	/// TODO.
	CXK = Self::letters_to_token(b"CXK"),
	
	/// TODO.
	CXL = Self::letters_to_token(b"CXL"),
	
	/// TODO.
	CXM = Self::letters_to_token(b"CXM"),
	
	/// TODO.
	CXN = Self::letters_to_token(b"CXN"),
	
	/// TODO.
	CXO = Self::letters_to_token(b"CXO"),
	
	/// TODO.
	CXP = Self::letters_to_token(b"CXP"),
	
	/// TODO.
	CXQ = Self::letters_to_token(b"CXQ"),
	
	/// TODO.
	CXR = Self::letters_to_token(b"CXR"),
	
	/// TODO.
	CXS = Self::letters_to_token(b"CXS"),
	
	/// TODO.
	CXT = Self::letters_to_token(b"CXT"),
	
	/// TODO.
	CXU = Self::letters_to_token(b"CXU"),
	
	/// TODO.
	CXV = Self::letters_to_token(b"CXV"),
	
	/// TODO.
	CXW = Self::letters_to_token(b"CXW"),
	
	/// TODO.
	CXX = Self::letters_to_token(b"CXX"),
	
	/// TODO.
	CXY = Self::letters_to_token(b"CXY"),
	
	/// TODO.
	CXZ = Self::letters_to_token(b"CXZ"),
	
	/// TODO.
	CYA = Self::letters_to_token(b"CYA"),
	
	/// TODO.
	CYB = Self::letters_to_token(b"CYB"),
	
	/// TODO.
	CYC = Self::letters_to_token(b"CYC"),
	
	/// TODO.
	CYD = Self::letters_to_token(b"CYD"),
	
	/// TODO.
	CYE = Self::letters_to_token(b"CYE"),
	
	/// TODO.
	CYF = Self::letters_to_token(b"CYF"),
	
	/// TODO.
	CYG = Self::letters_to_token(b"CYG"),
	
	/// TODO.
	CYH = Self::letters_to_token(b"CYH"),
	
	/// TODO.
	CYI = Self::letters_to_token(b"CYI"),
	
	/// TODO.
	CYJ = Self::letters_to_token(b"CYJ"),
	
	/// TODO.
	CYK = Self::letters_to_token(b"CYK"),
	
	/// TODO.
	CYL = Self::letters_to_token(b"CYL"),
	
	/// TODO.
	CYM = Self::letters_to_token(b"CYM"),
	
	/// TODO.
	CYN = Self::letters_to_token(b"CYN"),
	
	/// TODO.
	CYO = Self::letters_to_token(b"CYO"),
	
	/// TODO.
	CYP = Self::letters_to_token(b"CYP"),
	
	/// TODO.
	CYQ = Self::letters_to_token(b"CYQ"),
	
	/// TODO.
	CYR = Self::letters_to_token(b"CYR"),
	
	/// TODO.
	CYS = Self::letters_to_token(b"CYS"),
	
	/// TODO.
	CYT = Self::letters_to_token(b"CYT"),
	
	/// TODO.
	CYU = Self::letters_to_token(b"CYU"),
	
	/// TODO.
	CYV = Self::letters_to_token(b"CYV"),
	
	/// TODO.
	CYW = Self::letters_to_token(b"CYW"),
	
	/// TODO.
	CYX = Self::letters_to_token(b"CYX"),
	
	/// TODO.
	CYY = Self::letters_to_token(b"CYY"),
	
	/// TODO.
	CYZ = Self::letters_to_token(b"CYZ"),
	
	/// TODO.
	CZA = Self::letters_to_token(b"CZA"),
	
	/// TODO.
	CZB = Self::letters_to_token(b"CZB"),
	
	/// TODO.
	CZC = Self::letters_to_token(b"CZC"),
	
	/// TODO.
	CZD = Self::letters_to_token(b"CZD"),
	
	/// TODO.
	CZE = Self::letters_to_token(b"CZE"),
	
	/// TODO.
	CZF = Self::letters_to_token(b"CZF"),
	
	/// TODO.
	CZG = Self::letters_to_token(b"CZG"),
	
	/// TODO.
	CZH = Self::letters_to_token(b"CZH"),
	
	/// TODO.
	CZI = Self::letters_to_token(b"CZI"),
	
	/// TODO.
	CZJ = Self::letters_to_token(b"CZJ"),
	
	/// TODO.
	CZK = Self::letters_to_token(b"CZK"),
	
	/// TODO.
	CZL = Self::letters_to_token(b"CZL"),
	
	/// TODO.
	CZM = Self::letters_to_token(b"CZM"),
	
	/// TODO.
	CZN = Self::letters_to_token(b"CZN"),
	
	/// TODO.
	CZO = Self::letters_to_token(b"CZO"),
	
	/// TODO.
	CZP = Self::letters_to_token(b"CZP"),
	
	/// TODO.
	CZQ = Self::letters_to_token(b"CZQ"),
	
	/// TODO.
	CZR = Self::letters_to_token(b"CZR"),
	
	/// TODO.
	CZS = Self::letters_to_token(b"CZS"),
	
	/// TODO.
	CZT = Self::letters_to_token(b"CZT"),
	
	/// TODO.
	CZU = Self::letters_to_token(b"CZU"),
	
	/// TODO.
	CZV = Self::letters_to_token(b"CZV"),
	
	/// TODO.
	CZW = Self::letters_to_token(b"CZW"),
	
	/// TODO.
	CZX = Self::letters_to_token(b"CZX"),
	
	/// TODO.
	CZY = Self::letters_to_token(b"CZY"),
	
	/// TODO.
	CZZ = Self::letters_to_token(b"CZZ"),
	
	/// TODO.
	DAA = Self::letters_to_token(b"DAA"),
	
	/// TODO.
	DAB = Self::letters_to_token(b"DAB"),
	
	/// TODO.
	DAC = Self::letters_to_token(b"DAC"),
	
	/// TODO.
	DAD = Self::letters_to_token(b"DAD"),
	
	/// TODO.
	DAE = Self::letters_to_token(b"DAE"),
	
	/// TODO.
	DAF = Self::letters_to_token(b"DAF"),
	
	/// TODO.
	DAG = Self::letters_to_token(b"DAG"),
	
	/// TODO.
	DAH = Self::letters_to_token(b"DAH"),
	
	/// TODO.
	DAI = Self::letters_to_token(b"DAI"),
	
	/// TODO.
	DAJ = Self::letters_to_token(b"DAJ"),
	
	/// TODO.
	DAK = Self::letters_to_token(b"DAK"),
	
	/// TODO.
	DAL = Self::letters_to_token(b"DAL"),
	
	/// TODO.
	DAM = Self::letters_to_token(b"DAM"),
	
	/// TODO.
	DAN = Self::letters_to_token(b"DAN"),
	
	/// TODO.
	DAO = Self::letters_to_token(b"DAO"),
	
	/// TODO.
	DAP = Self::letters_to_token(b"DAP"),
	
	/// TODO.
	DAQ = Self::letters_to_token(b"DAQ"),
	
	/// TODO.
	DAR = Self::letters_to_token(b"DAR"),
	
	/// TODO.
	DAS = Self::letters_to_token(b"DAS"),
	
	/// TODO.
	DAT = Self::letters_to_token(b"DAT"),
	
	/// TODO.
	DAU = Self::letters_to_token(b"DAU"),
	
	/// TODO.
	DAV = Self::letters_to_token(b"DAV"),
	
	/// TODO.
	DAW = Self::letters_to_token(b"DAW"),
	
	/// TODO.
	DAX = Self::letters_to_token(b"DAX"),
	
	/// TODO.
	DAY = Self::letters_to_token(b"DAY"),
	
	/// TODO.
	DAZ = Self::letters_to_token(b"DAZ"),
	
	/// TODO.
	DBA = Self::letters_to_token(b"DBA"),
	
	/// TODO.
	DBB = Self::letters_to_token(b"DBB"),
	
	/// TODO.
	DBC = Self::letters_to_token(b"DBC"),
	
	/// TODO.
	DBD = Self::letters_to_token(b"DBD"),
	
	/// TODO.
	DBE = Self::letters_to_token(b"DBE"),
	
	/// TODO.
	DBF = Self::letters_to_token(b"DBF"),
	
	/// TODO.
	DBG = Self::letters_to_token(b"DBG"),
	
	/// TODO.
	DBH = Self::letters_to_token(b"DBH"),
	
	/// TODO.
	DBI = Self::letters_to_token(b"DBI"),
	
	/// TODO.
	DBJ = Self::letters_to_token(b"DBJ"),
	
	/// TODO.
	DBK = Self::letters_to_token(b"DBK"),
	
	/// TODO.
	DBL = Self::letters_to_token(b"DBL"),
	
	/// TODO.
	DBM = Self::letters_to_token(b"DBM"),
	
	/// TODO.
	DBN = Self::letters_to_token(b"DBN"),
	
	/// TODO.
	DBO = Self::letters_to_token(b"DBO"),
	
	/// TODO.
	DBP = Self::letters_to_token(b"DBP"),
	
	/// TODO.
	DBQ = Self::letters_to_token(b"DBQ"),
	
	/// TODO.
	DBR = Self::letters_to_token(b"DBR"),
	
	/// TODO.
	DBS = Self::letters_to_token(b"DBS"),
	
	/// TODO.
	DBT = Self::letters_to_token(b"DBT"),
	
	/// TODO.
	DBU = Self::letters_to_token(b"DBU"),
	
	/// TODO.
	DBV = Self::letters_to_token(b"DBV"),
	
	/// TODO.
	DBW = Self::letters_to_token(b"DBW"),
	
	/// TODO.
	DBX = Self::letters_to_token(b"DBX"),
	
	/// TODO.
	DBY = Self::letters_to_token(b"DBY"),
	
	/// TODO.
	DBZ = Self::letters_to_token(b"DBZ"),
	
	/// TODO.
	DCA = Self::letters_to_token(b"DCA"),
	
	/// TODO.
	DCB = Self::letters_to_token(b"DCB"),
	
	/// TODO.
	DCC = Self::letters_to_token(b"DCC"),
	
	/// TODO.
	DCD = Self::letters_to_token(b"DCD"),
	
	/// TODO.
	DCE = Self::letters_to_token(b"DCE"),
	
	/// TODO.
	DCF = Self::letters_to_token(b"DCF"),
	
	/// TODO.
	DCG = Self::letters_to_token(b"DCG"),
	
	/// TODO.
	DCH = Self::letters_to_token(b"DCH"),
	
	/// TODO.
	DCI = Self::letters_to_token(b"DCI"),
	
	/// TODO.
	DCJ = Self::letters_to_token(b"DCJ"),
	
	/// TODO.
	DCK = Self::letters_to_token(b"DCK"),
	
	/// TODO.
	DCL = Self::letters_to_token(b"DCL"),
	
	/// TODO.
	DCM = Self::letters_to_token(b"DCM"),
	
	/// TODO.
	DCN = Self::letters_to_token(b"DCN"),
	
	/// TODO.
	DCO = Self::letters_to_token(b"DCO"),
	
	/// TODO.
	DCP = Self::letters_to_token(b"DCP"),
	
	/// TODO.
	DCQ = Self::letters_to_token(b"DCQ"),
	
	/// TODO.
	DCR = Self::letters_to_token(b"DCR"),
	
	/// TODO.
	DCS = Self::letters_to_token(b"DCS"),
	
	/// TODO.
	DCT = Self::letters_to_token(b"DCT"),
	
	/// TODO.
	DCU = Self::letters_to_token(b"DCU"),
	
	/// TODO.
	DCV = Self::letters_to_token(b"DCV"),
	
	/// TODO.
	DCW = Self::letters_to_token(b"DCW"),
	
	/// TODO.
	DCX = Self::letters_to_token(b"DCX"),
	
	/// TODO.
	DCY = Self::letters_to_token(b"DCY"),
	
	/// TODO.
	DCZ = Self::letters_to_token(b"DCZ"),
	
	/// TODO.
	DDA = Self::letters_to_token(b"DDA"),
	
	/// TODO.
	DDB = Self::letters_to_token(b"DDB"),
	
	/// TODO.
	DDC = Self::letters_to_token(b"DDC"),
	
	/// TODO.
	DDD = Self::letters_to_token(b"DDD"),
	
	/// TODO.
	DDE = Self::letters_to_token(b"DDE"),
	
	/// TODO.
	DDF = Self::letters_to_token(b"DDF"),
	
	/// TODO.
	DDG = Self::letters_to_token(b"DDG"),
	
	/// TODO.
	DDH = Self::letters_to_token(b"DDH"),
	
	/// TODO.
	DDI = Self::letters_to_token(b"DDI"),
	
	/// TODO.
	DDJ = Self::letters_to_token(b"DDJ"),
	
	/// TODO.
	DDK = Self::letters_to_token(b"DDK"),
	
	/// TODO.
	DDL = Self::letters_to_token(b"DDL"),
	
	/// TODO.
	DDM = Self::letters_to_token(b"DDM"),
	
	/// TODO.
	DDN = Self::letters_to_token(b"DDN"),
	
	/// TODO.
	DDO = Self::letters_to_token(b"DDO"),
	
	/// TODO.
	DDP = Self::letters_to_token(b"DDP"),
	
	/// TODO.
	DDQ = Self::letters_to_token(b"DDQ"),
	
	/// TODO.
	DDR = Self::letters_to_token(b"DDR"),
	
	/// TODO.
	DDS = Self::letters_to_token(b"DDS"),
	
	/// TODO.
	DDT = Self::letters_to_token(b"DDT"),
	
	/// TODO.
	DDU = Self::letters_to_token(b"DDU"),
	
	/// TODO.
	DDV = Self::letters_to_token(b"DDV"),
	
	/// TODO.
	DDW = Self::letters_to_token(b"DDW"),
	
	/// TODO.
	DDX = Self::letters_to_token(b"DDX"),
	
	/// TODO.
	DDY = Self::letters_to_token(b"DDY"),
	
	/// TODO.
	DDZ = Self::letters_to_token(b"DDZ"),
	
	/// TODO.
	DEA = Self::letters_to_token(b"DEA"),
	
	/// TODO.
	DEB = Self::letters_to_token(b"DEB"),
	
	/// TODO.
	DEC = Self::letters_to_token(b"DEC"),
	
	/// TODO.
	DED = Self::letters_to_token(b"DED"),
	
	/// TODO.
	DEE = Self::letters_to_token(b"DEE"),
	
	/// TODO.
	DEF = Self::letters_to_token(b"DEF"),
	
	/// TODO.
	DEG = Self::letters_to_token(b"DEG"),
	
	/// TODO.
	DEH = Self::letters_to_token(b"DEH"),
	
	/// TODO.
	DEI = Self::letters_to_token(b"DEI"),
	
	/// TODO.
	DEJ = Self::letters_to_token(b"DEJ"),
	
	/// TODO.
	DEK = Self::letters_to_token(b"DEK"),
	
	/// TODO.
	DEL = Self::letters_to_token(b"DEL"),
	
	/// TODO.
	DEM = Self::letters_to_token(b"DEM"),
	
	/// TODO.
	DEN = Self::letters_to_token(b"DEN"),
	
	/// TODO.
	DEO = Self::letters_to_token(b"DEO"),
	
	/// TODO.
	DEP = Self::letters_to_token(b"DEP"),
	
	/// TODO.
	DEQ = Self::letters_to_token(b"DEQ"),
	
	/// TODO.
	DER = Self::letters_to_token(b"DER"),
	
	/// TODO.
	DES = Self::letters_to_token(b"DES"),
	
	/// TODO.
	DET = Self::letters_to_token(b"DET"),
	
	/// TODO.
	DEU = Self::letters_to_token(b"DEU"),
	
	/// TODO.
	DEV = Self::letters_to_token(b"DEV"),
	
	/// TODO.
	DEW = Self::letters_to_token(b"DEW"),
	
	/// TODO.
	DEX = Self::letters_to_token(b"DEX"),
	
	/// TODO.
	DEY = Self::letters_to_token(b"DEY"),
	
	/// TODO.
	DEZ = Self::letters_to_token(b"DEZ"),
	
	/// TODO.
	DFA = Self::letters_to_token(b"DFA"),
	
	/// TODO.
	DFB = Self::letters_to_token(b"DFB"),
	
	/// TODO.
	DFC = Self::letters_to_token(b"DFC"),
	
	/// TODO.
	DFD = Self::letters_to_token(b"DFD"),
	
	/// TODO.
	DFE = Self::letters_to_token(b"DFE"),
	
	/// TODO.
	DFF = Self::letters_to_token(b"DFF"),
	
	/// TODO.
	DFG = Self::letters_to_token(b"DFG"),
	
	/// TODO.
	DFH = Self::letters_to_token(b"DFH"),
	
	/// TODO.
	DFI = Self::letters_to_token(b"DFI"),
	
	/// TODO.
	DFJ = Self::letters_to_token(b"DFJ"),
	
	/// TODO.
	DFK = Self::letters_to_token(b"DFK"),
	
	/// TODO.
	DFL = Self::letters_to_token(b"DFL"),
	
	/// TODO.
	DFM = Self::letters_to_token(b"DFM"),
	
	/// TODO.
	DFN = Self::letters_to_token(b"DFN"),
	
	/// TODO.
	DFO = Self::letters_to_token(b"DFO"),
	
	/// TODO.
	DFP = Self::letters_to_token(b"DFP"),
	
	/// TODO.
	DFQ = Self::letters_to_token(b"DFQ"),
	
	/// TODO.
	DFR = Self::letters_to_token(b"DFR"),
	
	/// TODO.
	DFS = Self::letters_to_token(b"DFS"),
	
	/// TODO.
	DFT = Self::letters_to_token(b"DFT"),
	
	/// TODO.
	DFU = Self::letters_to_token(b"DFU"),
	
	/// TODO.
	DFV = Self::letters_to_token(b"DFV"),
	
	/// TODO.
	DFW = Self::letters_to_token(b"DFW"),
	
	/// TODO.
	DFX = Self::letters_to_token(b"DFX"),
	
	/// TODO.
	DFY = Self::letters_to_token(b"DFY"),
	
	/// TODO.
	DFZ = Self::letters_to_token(b"DFZ"),
	
	/// Exceptional reservation.
	///
	/// Diego Garcia.
	DGA = Self::letters_to_token(b"DGA"),
	
	/// TODO.
	DGB = Self::letters_to_token(b"DGB"),
	
	/// TODO.
	DGC = Self::letters_to_token(b"DGC"),
	
	/// TODO.
	DGD = Self::letters_to_token(b"DGD"),
	
	/// TODO.
	DGE = Self::letters_to_token(b"DGE"),
	
	/// TODO.
	DGF = Self::letters_to_token(b"DGF"),
	
	/// TODO.
	DGG = Self::letters_to_token(b"DGG"),
	
	/// TODO.
	DGH = Self::letters_to_token(b"DGH"),
	
	/// TODO.
	DGI = Self::letters_to_token(b"DGI"),
	
	/// TODO.
	DGJ = Self::letters_to_token(b"DGJ"),
	
	/// TODO.
	DGK = Self::letters_to_token(b"DGK"),
	
	/// TODO.
	DGL = Self::letters_to_token(b"DGL"),
	
	/// TODO.
	DGM = Self::letters_to_token(b"DGM"),
	
	/// TODO.
	DGN = Self::letters_to_token(b"DGN"),
	
	/// TODO.
	DGO = Self::letters_to_token(b"DGO"),
	
	/// TODO.
	DGP = Self::letters_to_token(b"DGP"),
	
	/// TODO.
	DGQ = Self::letters_to_token(b"DGQ"),
	
	/// TODO.
	DGR = Self::letters_to_token(b"DGR"),
	
	/// TODO.
	DGS = Self::letters_to_token(b"DGS"),
	
	/// TODO.
	DGT = Self::letters_to_token(b"DGT"),
	
	/// TODO.
	DGU = Self::letters_to_token(b"DGU"),
	
	/// TODO.
	DGV = Self::letters_to_token(b"DGV"),
	
	/// TODO.
	DGW = Self::letters_to_token(b"DGW"),
	
	/// TODO.
	DGX = Self::letters_to_token(b"DGX"),
	
	/// TODO.
	DGY = Self::letters_to_token(b"DGY"),
	
	/// TODO.
	DGZ = Self::letters_to_token(b"DGZ"),
	
	/// TODO.
	DHA = Self::letters_to_token(b"DHA"),
	
	/// TODO.
	DHB = Self::letters_to_token(b"DHB"),
	
	/// TODO.
	DHC = Self::letters_to_token(b"DHC"),
	
	/// TODO.
	DHD = Self::letters_to_token(b"DHD"),
	
	/// TODO.
	DHE = Self::letters_to_token(b"DHE"),
	
	/// TODO.
	DHF = Self::letters_to_token(b"DHF"),
	
	/// TODO.
	DHG = Self::letters_to_token(b"DHG"),
	
	/// TODO.
	DHH = Self::letters_to_token(b"DHH"),
	
	/// TODO.
	DHI = Self::letters_to_token(b"DHI"),
	
	/// TODO.
	DHJ = Self::letters_to_token(b"DHJ"),
	
	/// TODO.
	DHK = Self::letters_to_token(b"DHK"),
	
	/// TODO.
	DHL = Self::letters_to_token(b"DHL"),
	
	/// TODO.
	DHM = Self::letters_to_token(b"DHM"),
	
	/// TODO.
	DHN = Self::letters_to_token(b"DHN"),
	
	/// TODO.
	DHO = Self::letters_to_token(b"DHO"),
	
	/// TODO.
	DHP = Self::letters_to_token(b"DHP"),
	
	/// TODO.
	DHQ = Self::letters_to_token(b"DHQ"),
	
	/// TODO.
	DHR = Self::letters_to_token(b"DHR"),
	
	/// TODO.
	DHS = Self::letters_to_token(b"DHS"),
	
	/// TODO.
	DHT = Self::letters_to_token(b"DHT"),
	
	/// TODO.
	DHU = Self::letters_to_token(b"DHU"),
	
	/// TODO.
	DHV = Self::letters_to_token(b"DHV"),
	
	/// TODO.
	DHW = Self::letters_to_token(b"DHW"),
	
	/// TODO.
	DHX = Self::letters_to_token(b"DHX"),
	
	/// TODO.
	DHY = Self::letters_to_token(b"DHY"),
	
	/// TODO.
	DHZ = Self::letters_to_token(b"DHZ"),
	
	/// TODO.
	DIA = Self::letters_to_token(b"DIA"),
	
	/// TODO.
	DIB = Self::letters_to_token(b"DIB"),
	
	/// TODO.
	DIC = Self::letters_to_token(b"DIC"),
	
	/// TODO.
	DID = Self::letters_to_token(b"DID"),
	
	/// TODO.
	DIE = Self::letters_to_token(b"DIE"),
	
	/// TODO.
	DIF = Self::letters_to_token(b"DIF"),
	
	/// TODO.
	DIG = Self::letters_to_token(b"DIG"),
	
	/// TODO.
	DIH = Self::letters_to_token(b"DIH"),
	
	/// TODO.
	DII = Self::letters_to_token(b"DII"),
	
	/// TODO.
	DIJ = Self::letters_to_token(b"DIJ"),
	
	/// TODO.
	DIK = Self::letters_to_token(b"DIK"),
	
	/// TODO.
	DIL = Self::letters_to_token(b"DIL"),
	
	/// TODO.
	DIM = Self::letters_to_token(b"DIM"),
	
	/// TODO.
	DIN = Self::letters_to_token(b"DIN"),
	
	/// TODO.
	DIO = Self::letters_to_token(b"DIO"),
	
	/// TODO.
	DIP = Self::letters_to_token(b"DIP"),
	
	/// TODO.
	DIQ = Self::letters_to_token(b"DIQ"),
	
	/// TODO.
	DIR = Self::letters_to_token(b"DIR"),
	
	/// TODO.
	DIS = Self::letters_to_token(b"DIS"),
	
	/// TODO.
	DIT = Self::letters_to_token(b"DIT"),
	
	/// TODO.
	DIU = Self::letters_to_token(b"DIU"),
	
	/// TODO.
	DIV = Self::letters_to_token(b"DIV"),
	
	/// TODO.
	DIW = Self::letters_to_token(b"DIW"),
	
	/// TODO.
	DIX = Self::letters_to_token(b"DIX"),
	
	/// TODO.
	DIY = Self::letters_to_token(b"DIY"),
	
	/// TODO.
	DIZ = Self::letters_to_token(b"DIZ"),
	
	/// TODO.
	DJA = Self::letters_to_token(b"DJA"),
	
	/// TODO.
	DJB = Self::letters_to_token(b"DJB"),
	
	/// TODO.
	DJC = Self::letters_to_token(b"DJC"),
	
	/// TODO.
	DJD = Self::letters_to_token(b"DJD"),
	
	/// TODO.
	DJE = Self::letters_to_token(b"DJE"),
	
	/// TODO.
	DJF = Self::letters_to_token(b"DJF"),
	
	/// TODO.
	DJG = Self::letters_to_token(b"DJG"),
	
	/// TODO.
	DJH = Self::letters_to_token(b"DJH"),
	
	/// TODO.
	DJI = Self::letters_to_token(b"DJI"),
	
	/// TODO.
	DJJ = Self::letters_to_token(b"DJJ"),
	
	/// TODO.
	DJK = Self::letters_to_token(b"DJK"),
	
	/// TODO.
	DJL = Self::letters_to_token(b"DJL"),
	
	/// TODO.
	DJM = Self::letters_to_token(b"DJM"),
	
	/// TODO.
	DJN = Self::letters_to_token(b"DJN"),
	
	/// TODO.
	DJO = Self::letters_to_token(b"DJO"),
	
	/// TODO.
	DJP = Self::letters_to_token(b"DJP"),
	
	/// TODO.
	DJQ = Self::letters_to_token(b"DJQ"),
	
	/// TODO.
	DJR = Self::letters_to_token(b"DJR"),
	
	/// TODO.
	DJS = Self::letters_to_token(b"DJS"),
	
	/// TODO.
	DJT = Self::letters_to_token(b"DJT"),
	
	/// TODO.
	DJU = Self::letters_to_token(b"DJU"),
	
	/// TODO.
	DJV = Self::letters_to_token(b"DJV"),
	
	/// TODO.
	DJW = Self::letters_to_token(b"DJW"),
	
	/// TODO.
	DJX = Self::letters_to_token(b"DJX"),
	
	/// TODO.
	DJY = Self::letters_to_token(b"DJY"),
	
	/// TODO.
	DJZ = Self::letters_to_token(b"DJZ"),
	
	/// TODO.
	DKA = Self::letters_to_token(b"DKA"),
	
	/// TODO.
	DKB = Self::letters_to_token(b"DKB"),
	
	/// TODO.
	DKC = Self::letters_to_token(b"DKC"),
	
	/// TODO.
	DKD = Self::letters_to_token(b"DKD"),
	
	/// TODO.
	DKE = Self::letters_to_token(b"DKE"),
	
	/// TODO.
	DKF = Self::letters_to_token(b"DKF"),
	
	/// TODO.
	DKG = Self::letters_to_token(b"DKG"),
	
	/// TODO.
	DKH = Self::letters_to_token(b"DKH"),
	
	/// TODO.
	DKI = Self::letters_to_token(b"DKI"),
	
	/// TODO.
	DKJ = Self::letters_to_token(b"DKJ"),
	
	/// TODO.
	DKK = Self::letters_to_token(b"DKK"),
	
	/// TODO.
	DKL = Self::letters_to_token(b"DKL"),
	
	/// TODO.
	DKM = Self::letters_to_token(b"DKM"),
	
	/// TODO.
	DKN = Self::letters_to_token(b"DKN"),
	
	/// TODO.
	DKO = Self::letters_to_token(b"DKO"),
	
	/// TODO.
	DKP = Self::letters_to_token(b"DKP"),
	
	/// TODO.
	DKQ = Self::letters_to_token(b"DKQ"),
	
	/// TODO.
	DKR = Self::letters_to_token(b"DKR"),
	
	/// TODO.
	DKS = Self::letters_to_token(b"DKS"),
	
	/// TODO.
	DKT = Self::letters_to_token(b"DKT"),
	
	/// TODO.
	DKU = Self::letters_to_token(b"DKU"),
	
	/// TODO.
	DKV = Self::letters_to_token(b"DKV"),
	
	/// TODO.
	DKW = Self::letters_to_token(b"DKW"),
	
	/// TODO.
	DKX = Self::letters_to_token(b"DKX"),
	
	/// TODO.
	DKY = Self::letters_to_token(b"DKY"),
	
	/// TODO.
	DKZ = Self::letters_to_token(b"DKZ"),
	
	/// TODO.
	DLA = Self::letters_to_token(b"DLA"),
	
	/// TODO.
	DLB = Self::letters_to_token(b"DLB"),
	
	/// TODO.
	DLC = Self::letters_to_token(b"DLC"),
	
	/// TODO.
	DLD = Self::letters_to_token(b"DLD"),
	
	/// TODO.
	DLE = Self::letters_to_token(b"DLE"),
	
	/// TODO.
	DLF = Self::letters_to_token(b"DLF"),
	
	/// TODO.
	DLG = Self::letters_to_token(b"DLG"),
	
	/// TODO.
	DLH = Self::letters_to_token(b"DLH"),
	
	/// TODO.
	DLI = Self::letters_to_token(b"DLI"),
	
	/// TODO.
	DLJ = Self::letters_to_token(b"DLJ"),
	
	/// TODO.
	DLK = Self::letters_to_token(b"DLK"),
	
	/// TODO.
	DLL = Self::letters_to_token(b"DLL"),
	
	/// TODO.
	DLM = Self::letters_to_token(b"DLM"),
	
	/// TODO.
	DLN = Self::letters_to_token(b"DLN"),
	
	/// TODO.
	DLO = Self::letters_to_token(b"DLO"),
	
	/// TODO.
	DLP = Self::letters_to_token(b"DLP"),
	
	/// TODO.
	DLQ = Self::letters_to_token(b"DLQ"),
	
	/// TODO.
	DLR = Self::letters_to_token(b"DLR"),
	
	/// TODO.
	DLS = Self::letters_to_token(b"DLS"),
	
	/// TODO.
	DLT = Self::letters_to_token(b"DLT"),
	
	/// TODO.
	DLU = Self::letters_to_token(b"DLU"),
	
	/// TODO.
	DLV = Self::letters_to_token(b"DLV"),
	
	/// TODO.
	DLW = Self::letters_to_token(b"DLW"),
	
	/// TODO.
	DLX = Self::letters_to_token(b"DLX"),
	
	/// TODO.
	DLY = Self::letters_to_token(b"DLY"),
	
	/// TODO.
	DLZ = Self::letters_to_token(b"DLZ"),
	
	/// TODO.
	DMA = Self::letters_to_token(b"DMA"),
	
	/// TODO.
	DMB = Self::letters_to_token(b"DMB"),
	
	/// TODO.
	DMC = Self::letters_to_token(b"DMC"),
	
	/// TODO.
	DMD = Self::letters_to_token(b"DMD"),
	
	/// TODO.
	DME = Self::letters_to_token(b"DME"),
	
	/// TODO.
	DMF = Self::letters_to_token(b"DMF"),
	
	/// TODO.
	DMG = Self::letters_to_token(b"DMG"),
	
	/// TODO.
	DMH = Self::letters_to_token(b"DMH"),
	
	/// TODO.
	DMI = Self::letters_to_token(b"DMI"),
	
	/// TODO.
	DMJ = Self::letters_to_token(b"DMJ"),
	
	/// TODO.
	DMK = Self::letters_to_token(b"DMK"),
	
	/// TODO.
	DML = Self::letters_to_token(b"DML"),
	
	/// TODO.
	DMM = Self::letters_to_token(b"DMM"),
	
	/// TODO.
	DMN = Self::letters_to_token(b"DMN"),
	
	/// TODO.
	DMO = Self::letters_to_token(b"DMO"),
	
	/// TODO.
	DMP = Self::letters_to_token(b"DMP"),
	
	/// TODO.
	DMQ = Self::letters_to_token(b"DMQ"),
	
	/// TODO.
	DMR = Self::letters_to_token(b"DMR"),
	
	/// TODO.
	DMS = Self::letters_to_token(b"DMS"),
	
	/// TODO.
	DMT = Self::letters_to_token(b"DMT"),
	
	/// TODO.
	DMU = Self::letters_to_token(b"DMU"),
	
	/// TODO.
	DMV = Self::letters_to_token(b"DMV"),
	
	/// TODO.
	DMW = Self::letters_to_token(b"DMW"),
	
	/// TODO.
	DMX = Self::letters_to_token(b"DMX"),
	
	/// TODO.
	DMY = Self::letters_to_token(b"DMY"),
	
	/// TODO.
	DMZ = Self::letters_to_token(b"DMZ"),
	
	/// TODO.
	DNA = Self::letters_to_token(b"DNA"),
	
	/// TODO.
	DNB = Self::letters_to_token(b"DNB"),
	
	/// TODO.
	DNC = Self::letters_to_token(b"DNC"),
	
	/// TODO.
	DND = Self::letters_to_token(b"DND"),
	
	/// TODO.
	DNE = Self::letters_to_token(b"DNE"),
	
	/// TODO.
	DNF = Self::letters_to_token(b"DNF"),
	
	/// TODO.
	DNG = Self::letters_to_token(b"DNG"),
	
	/// TODO.
	DNH = Self::letters_to_token(b"DNH"),
	
	/// TODO.
	DNI = Self::letters_to_token(b"DNI"),
	
	/// TODO.
	DNJ = Self::letters_to_token(b"DNJ"),
	
	/// TODO.
	DNK = Self::letters_to_token(b"DNK"),
	
	/// TODO.
	DNL = Self::letters_to_token(b"DNL"),
	
	/// TODO.
	DNM = Self::letters_to_token(b"DNM"),
	
	/// TODO.
	DNN = Self::letters_to_token(b"DNN"),
	
	/// TODO.
	DNO = Self::letters_to_token(b"DNO"),
	
	/// TODO.
	DNP = Self::letters_to_token(b"DNP"),
	
	/// TODO.
	DNQ = Self::letters_to_token(b"DNQ"),
	
	/// TODO.
	DNR = Self::letters_to_token(b"DNR"),
	
	/// TODO.
	DNS = Self::letters_to_token(b"DNS"),
	
	/// TODO.
	DNT = Self::letters_to_token(b"DNT"),
	
	/// TODO.
	DNU = Self::letters_to_token(b"DNU"),
	
	/// TODO.
	DNV = Self::letters_to_token(b"DNV"),
	
	/// TODO.
	DNW = Self::letters_to_token(b"DNW"),
	
	/// TODO.
	DNX = Self::letters_to_token(b"DNX"),
	
	/// TODO.
	DNY = Self::letters_to_token(b"DNY"),
	
	/// TODO.
	DNZ = Self::letters_to_token(b"DNZ"),
	
	/// TODO.
	DOA = Self::letters_to_token(b"DOA"),
	
	/// TODO.
	DOB = Self::letters_to_token(b"DOB"),
	
	/// TODO.
	DOC = Self::letters_to_token(b"DOC"),
	
	/// TODO.
	DOD = Self::letters_to_token(b"DOD"),
	
	/// TODO.
	DOE = Self::letters_to_token(b"DOE"),
	
	/// TODO.
	DOF = Self::letters_to_token(b"DOF"),
	
	/// TODO.
	DOG = Self::letters_to_token(b"DOG"),
	
	/// TODO.
	DOH = Self::letters_to_token(b"DOH"),
	
	/// TODO.
	DOI = Self::letters_to_token(b"DOI"),
	
	/// TODO.
	DOJ = Self::letters_to_token(b"DOJ"),
	
	/// TODO.
	DOK = Self::letters_to_token(b"DOK"),
	
	/// TODO.
	DOL = Self::letters_to_token(b"DOL"),
	
	/// TODO.
	DOM = Self::letters_to_token(b"DOM"),
	
	/// TODO.
	DON = Self::letters_to_token(b"DON"),
	
	/// TODO.
	DOO = Self::letters_to_token(b"DOO"),
	
	/// TODO.
	DOP = Self::letters_to_token(b"DOP"),
	
	/// TODO.
	DOQ = Self::letters_to_token(b"DOQ"),
	
	/// TODO.
	DOR = Self::letters_to_token(b"DOR"),
	
	/// TODO.
	DOS = Self::letters_to_token(b"DOS"),
	
	/// TODO.
	DOT = Self::letters_to_token(b"DOT"),
	
	/// TODO.
	DOU = Self::letters_to_token(b"DOU"),
	
	/// TODO.
	DOV = Self::letters_to_token(b"DOV"),
	
	/// TODO.
	DOW = Self::letters_to_token(b"DOW"),
	
	/// TODO.
	DOX = Self::letters_to_token(b"DOX"),
	
	/// TODO.
	DOY = Self::letters_to_token(b"DOY"),
	
	/// TODO.
	DOZ = Self::letters_to_token(b"DOZ"),
	
	/// TODO.
	DPA = Self::letters_to_token(b"DPA"),
	
	/// TODO.
	DPB = Self::letters_to_token(b"DPB"),
	
	/// TODO.
	DPC = Self::letters_to_token(b"DPC"),
	
	/// TODO.
	DPD = Self::letters_to_token(b"DPD"),
	
	/// TODO.
	DPE = Self::letters_to_token(b"DPE"),
	
	/// TODO.
	DPF = Self::letters_to_token(b"DPF"),
	
	/// TODO.
	DPG = Self::letters_to_token(b"DPG"),
	
	/// TODO.
	DPH = Self::letters_to_token(b"DPH"),
	
	/// TODO.
	DPI = Self::letters_to_token(b"DPI"),
	
	/// TODO.
	DPJ = Self::letters_to_token(b"DPJ"),
	
	/// TODO.
	DPK = Self::letters_to_token(b"DPK"),
	
	/// TODO.
	DPL = Self::letters_to_token(b"DPL"),
	
	/// TODO.
	DPM = Self::letters_to_token(b"DPM"),
	
	/// TODO.
	DPN = Self::letters_to_token(b"DPN"),
	
	/// TODO.
	DPO = Self::letters_to_token(b"DPO"),
	
	/// TODO.
	DPP = Self::letters_to_token(b"DPP"),
	
	/// TODO.
	DPQ = Self::letters_to_token(b"DPQ"),
	
	/// TODO.
	DPR = Self::letters_to_token(b"DPR"),
	
	/// TODO.
	DPS = Self::letters_to_token(b"DPS"),
	
	/// TODO.
	DPT = Self::letters_to_token(b"DPT"),
	
	/// TODO.
	DPU = Self::letters_to_token(b"DPU"),
	
	/// TODO.
	DPV = Self::letters_to_token(b"DPV"),
	
	/// TODO.
	DPW = Self::letters_to_token(b"DPW"),
	
	/// TODO.
	DPX = Self::letters_to_token(b"DPX"),
	
	/// TODO.
	DPY = Self::letters_to_token(b"DPY"),
	
	/// TODO.
	DPZ = Self::letters_to_token(b"DPZ"),
	
	/// TODO.
	DQA = Self::letters_to_token(b"DQA"),
	
	/// TODO.
	DQB = Self::letters_to_token(b"DQB"),
	
	/// TODO.
	DQC = Self::letters_to_token(b"DQC"),
	
	/// TODO.
	DQD = Self::letters_to_token(b"DQD"),
	
	/// TODO.
	DQE = Self::letters_to_token(b"DQE"),
	
	/// TODO.
	DQF = Self::letters_to_token(b"DQF"),
	
	/// TODO.
	DQG = Self::letters_to_token(b"DQG"),
	
	/// TODO.
	DQH = Self::letters_to_token(b"DQH"),
	
	/// TODO.
	DQI = Self::letters_to_token(b"DQI"),
	
	/// TODO.
	DQJ = Self::letters_to_token(b"DQJ"),
	
	/// TODO.
	DQK = Self::letters_to_token(b"DQK"),
	
	/// TODO.
	DQL = Self::letters_to_token(b"DQL"),
	
	/// TODO.
	DQM = Self::letters_to_token(b"DQM"),
	
	/// TODO.
	DQN = Self::letters_to_token(b"DQN"),
	
	/// TODO.
	DQO = Self::letters_to_token(b"DQO"),
	
	/// TODO.
	DQP = Self::letters_to_token(b"DQP"),
	
	/// TODO.
	DQQ = Self::letters_to_token(b"DQQ"),
	
	/// TODO.
	DQR = Self::letters_to_token(b"DQR"),
	
	/// TODO.
	DQS = Self::letters_to_token(b"DQS"),
	
	/// TODO.
	DQT = Self::letters_to_token(b"DQT"),
	
	/// TODO.
	DQU = Self::letters_to_token(b"DQU"),
	
	/// TODO.
	DQV = Self::letters_to_token(b"DQV"),
	
	/// TODO.
	DQW = Self::letters_to_token(b"DQW"),
	
	/// TODO.
	DQX = Self::letters_to_token(b"DQX"),
	
	/// TODO.
	DQY = Self::letters_to_token(b"DQY"),
	
	/// TODO.
	DQZ = Self::letters_to_token(b"DQZ"),
	
	/// TODO.
	DRA = Self::letters_to_token(b"DRA"),
	
	/// TODO.
	DRB = Self::letters_to_token(b"DRB"),
	
	/// TODO.
	DRC = Self::letters_to_token(b"DRC"),
	
	/// TODO.
	DRD = Self::letters_to_token(b"DRD"),
	
	/// TODO.
	DRE = Self::letters_to_token(b"DRE"),
	
	/// TODO.
	DRF = Self::letters_to_token(b"DRF"),
	
	/// TODO.
	DRG = Self::letters_to_token(b"DRG"),
	
	/// TODO.
	DRH = Self::letters_to_token(b"DRH"),
	
	/// TODO.
	DRI = Self::letters_to_token(b"DRI"),
	
	/// TODO.
	DRJ = Self::letters_to_token(b"DRJ"),
	
	/// TODO.
	DRK = Self::letters_to_token(b"DRK"),
	
	/// TODO.
	DRL = Self::letters_to_token(b"DRL"),
	
	/// TODO.
	DRM = Self::letters_to_token(b"DRM"),
	
	/// TODO.
	DRN = Self::letters_to_token(b"DRN"),
	
	/// TODO.
	DRO = Self::letters_to_token(b"DRO"),
	
	/// TODO.
	DRP = Self::letters_to_token(b"DRP"),
	
	/// TODO.
	DRQ = Self::letters_to_token(b"DRQ"),
	
	/// TODO.
	DRR = Self::letters_to_token(b"DRR"),
	
	/// TODO.
	DRS = Self::letters_to_token(b"DRS"),
	
	/// TODO.
	DRT = Self::letters_to_token(b"DRT"),
	
	/// TODO.
	DRU = Self::letters_to_token(b"DRU"),
	
	/// TODO.
	DRV = Self::letters_to_token(b"DRV"),
	
	/// TODO.
	DRW = Self::letters_to_token(b"DRW"),
	
	/// TODO.
	DRX = Self::letters_to_token(b"DRX"),
	
	/// TODO.
	DRY = Self::letters_to_token(b"DRY"),
	
	/// TODO.
	DRZ = Self::letters_to_token(b"DRZ"),
	
	/// TODO.
	DSA = Self::letters_to_token(b"DSA"),
	
	/// TODO.
	DSB = Self::letters_to_token(b"DSB"),
	
	/// TODO.
	DSC = Self::letters_to_token(b"DSC"),
	
	/// TODO.
	DSD = Self::letters_to_token(b"DSD"),
	
	/// TODO.
	DSE = Self::letters_to_token(b"DSE"),
	
	/// TODO.
	DSF = Self::letters_to_token(b"DSF"),
	
	/// TODO.
	DSG = Self::letters_to_token(b"DSG"),
	
	/// TODO.
	DSH = Self::letters_to_token(b"DSH"),
	
	/// TODO.
	DSI = Self::letters_to_token(b"DSI"),
	
	/// TODO.
	DSJ = Self::letters_to_token(b"DSJ"),
	
	/// TODO.
	DSK = Self::letters_to_token(b"DSK"),
	
	/// TODO.
	DSL = Self::letters_to_token(b"DSL"),
	
	/// TODO.
	DSM = Self::letters_to_token(b"DSM"),
	
	/// TODO.
	DSN = Self::letters_to_token(b"DSN"),
	
	/// TODO.
	DSO = Self::letters_to_token(b"DSO"),
	
	/// TODO.
	DSP = Self::letters_to_token(b"DSP"),
	
	/// TODO.
	DSQ = Self::letters_to_token(b"DSQ"),
	
	/// TODO.
	DSR = Self::letters_to_token(b"DSR"),
	
	/// TODO.
	DSS = Self::letters_to_token(b"DSS"),
	
	/// TODO.
	DST = Self::letters_to_token(b"DST"),
	
	/// TODO.
	DSU = Self::letters_to_token(b"DSU"),
	
	/// TODO.
	DSV = Self::letters_to_token(b"DSV"),
	
	/// TODO.
	DSW = Self::letters_to_token(b"DSW"),
	
	/// TODO.
	DSX = Self::letters_to_token(b"DSX"),
	
	/// TODO.
	DSY = Self::letters_to_token(b"DSY"),
	
	/// TODO.
	DSZ = Self::letters_to_token(b"DSZ"),
	
	/// TODO.
	DTA = Self::letters_to_token(b"DTA"),
	
	/// TODO.
	DTB = Self::letters_to_token(b"DTB"),
	
	/// TODO.
	DTC = Self::letters_to_token(b"DTC"),
	
	/// TODO.
	DTD = Self::letters_to_token(b"DTD"),
	
	/// TODO.
	DTE = Self::letters_to_token(b"DTE"),
	
	/// TODO.
	DTF = Self::letters_to_token(b"DTF"),
	
	/// TODO.
	DTG = Self::letters_to_token(b"DTG"),
	
	/// TODO.
	DTH = Self::letters_to_token(b"DTH"),
	
	/// TODO.
	DTI = Self::letters_to_token(b"DTI"),
	
	/// TODO.
	DTJ = Self::letters_to_token(b"DTJ"),
	
	/// TODO.
	DTK = Self::letters_to_token(b"DTK"),
	
	/// TODO.
	DTL = Self::letters_to_token(b"DTL"),
	
	/// TODO.
	DTM = Self::letters_to_token(b"DTM"),
	
	/// TODO.
	DTN = Self::letters_to_token(b"DTN"),
	
	/// TODO.
	DTO = Self::letters_to_token(b"DTO"),
	
	/// TODO.
	DTP = Self::letters_to_token(b"DTP"),
	
	/// TODO.
	DTQ = Self::letters_to_token(b"DTQ"),
	
	/// TODO.
	DTR = Self::letters_to_token(b"DTR"),
	
	/// TODO.
	DTS = Self::letters_to_token(b"DTS"),
	
	/// TODO.
	DTT = Self::letters_to_token(b"DTT"),
	
	/// TODO.
	DTU = Self::letters_to_token(b"DTU"),
	
	/// TODO.
	DTV = Self::letters_to_token(b"DTV"),
	
	/// TODO.
	DTW = Self::letters_to_token(b"DTW"),
	
	/// TODO.
	DTX = Self::letters_to_token(b"DTX"),
	
	/// TODO.
	DTY = Self::letters_to_token(b"DTY"),
	
	/// TODO.
	DTZ = Self::letters_to_token(b"DTZ"),
	
	/// TODO.
	DUA = Self::letters_to_token(b"DUA"),
	
	/// TODO.
	DUB = Self::letters_to_token(b"DUB"),
	
	/// TODO.
	DUC = Self::letters_to_token(b"DUC"),
	
	/// TODO.
	DUD = Self::letters_to_token(b"DUD"),
	
	/// TODO.
	DUE = Self::letters_to_token(b"DUE"),
	
	/// TODO.
	DUF = Self::letters_to_token(b"DUF"),
	
	/// TODO.
	DUG = Self::letters_to_token(b"DUG"),
	
	/// TODO.
	DUH = Self::letters_to_token(b"DUH"),
	
	/// TODO.
	DUI = Self::letters_to_token(b"DUI"),
	
	/// TODO.
	DUJ = Self::letters_to_token(b"DUJ"),
	
	/// TODO.
	DUK = Self::letters_to_token(b"DUK"),
	
	/// TODO.
	DUL = Self::letters_to_token(b"DUL"),
	
	/// TODO.
	DUM = Self::letters_to_token(b"DUM"),
	
	/// TODO.
	DUN = Self::letters_to_token(b"DUN"),
	
	/// TODO.
	DUO = Self::letters_to_token(b"DUO"),
	
	/// TODO.
	DUP = Self::letters_to_token(b"DUP"),
	
	/// TODO.
	DUQ = Self::letters_to_token(b"DUQ"),
	
	/// TODO.
	DUR = Self::letters_to_token(b"DUR"),
	
	/// TODO.
	DUS = Self::letters_to_token(b"DUS"),
	
	/// TODO.
	DUT = Self::letters_to_token(b"DUT"),
	
	/// TODO.
	DUU = Self::letters_to_token(b"DUU"),
	
	/// TODO.
	DUV = Self::letters_to_token(b"DUV"),
	
	/// TODO.
	DUW = Self::letters_to_token(b"DUW"),
	
	/// TODO.
	DUX = Self::letters_to_token(b"DUX"),
	
	/// TODO.
	DUY = Self::letters_to_token(b"DUY"),
	
	/// TODO.
	DUZ = Self::letters_to_token(b"DUZ"),
	
	/// TODO.
	DVA = Self::letters_to_token(b"DVA"),
	
	/// TODO.
	DVB = Self::letters_to_token(b"DVB"),
	
	/// TODO.
	DVC = Self::letters_to_token(b"DVC"),
	
	/// TODO.
	DVD = Self::letters_to_token(b"DVD"),
	
	/// TODO.
	DVE = Self::letters_to_token(b"DVE"),
	
	/// TODO.
	DVF = Self::letters_to_token(b"DVF"),
	
	/// TODO.
	DVG = Self::letters_to_token(b"DVG"),
	
	/// TODO.
	DVH = Self::letters_to_token(b"DVH"),
	
	/// TODO.
	DVI = Self::letters_to_token(b"DVI"),
	
	/// TODO.
	DVJ = Self::letters_to_token(b"DVJ"),
	
	/// TODO.
	DVK = Self::letters_to_token(b"DVK"),
	
	/// TODO.
	DVL = Self::letters_to_token(b"DVL"),
	
	/// TODO.
	DVM = Self::letters_to_token(b"DVM"),
	
	/// TODO.
	DVN = Self::letters_to_token(b"DVN"),
	
	/// TODO.
	DVO = Self::letters_to_token(b"DVO"),
	
	/// TODO.
	DVP = Self::letters_to_token(b"DVP"),
	
	/// TODO.
	DVQ = Self::letters_to_token(b"DVQ"),
	
	/// TODO.
	DVR = Self::letters_to_token(b"DVR"),
	
	/// TODO.
	DVS = Self::letters_to_token(b"DVS"),
	
	/// TODO.
	DVT = Self::letters_to_token(b"DVT"),
	
	/// TODO.
	DVU = Self::letters_to_token(b"DVU"),
	
	/// TODO.
	DVV = Self::letters_to_token(b"DVV"),
	
	/// TODO.
	DVW = Self::letters_to_token(b"DVW"),
	
	/// TODO.
	DVX = Self::letters_to_token(b"DVX"),
	
	/// TODO.
	DVY = Self::letters_to_token(b"DVY"),
	
	/// TODO.
	DVZ = Self::letters_to_token(b"DVZ"),
	
	/// TODO.
	DWA = Self::letters_to_token(b"DWA"),
	
	/// TODO.
	DWB = Self::letters_to_token(b"DWB"),
	
	/// TODO.
	DWC = Self::letters_to_token(b"DWC"),
	
	/// TODO.
	DWD = Self::letters_to_token(b"DWD"),
	
	/// TODO.
	DWE = Self::letters_to_token(b"DWE"),
	
	/// TODO.
	DWF = Self::letters_to_token(b"DWF"),
	
	/// TODO.
	DWG = Self::letters_to_token(b"DWG"),
	
	/// TODO.
	DWH = Self::letters_to_token(b"DWH"),
	
	/// TODO.
	DWI = Self::letters_to_token(b"DWI"),
	
	/// TODO.
	DWJ = Self::letters_to_token(b"DWJ"),
	
	/// TODO.
	DWK = Self::letters_to_token(b"DWK"),
	
	/// TODO.
	DWL = Self::letters_to_token(b"DWL"),
	
	/// TODO.
	DWM = Self::letters_to_token(b"DWM"),
	
	/// TODO.
	DWN = Self::letters_to_token(b"DWN"),
	
	/// TODO.
	DWO = Self::letters_to_token(b"DWO"),
	
	/// TODO.
	DWP = Self::letters_to_token(b"DWP"),
	
	/// TODO.
	DWQ = Self::letters_to_token(b"DWQ"),
	
	/// TODO.
	DWR = Self::letters_to_token(b"DWR"),
	
	/// TODO.
	DWS = Self::letters_to_token(b"DWS"),
	
	/// TODO.
	DWT = Self::letters_to_token(b"DWT"),
	
	/// TODO.
	DWU = Self::letters_to_token(b"DWU"),
	
	/// TODO.
	DWV = Self::letters_to_token(b"DWV"),
	
	/// TODO.
	DWW = Self::letters_to_token(b"DWW"),
	
	/// TODO.
	DWX = Self::letters_to_token(b"DWX"),
	
	/// TODO.
	DWY = Self::letters_to_token(b"DWY"),
	
	/// TODO.
	DWZ = Self::letters_to_token(b"DWZ"),
	
	/// TODO.
	DXA = Self::letters_to_token(b"DXA"),
	
	/// TODO.
	DXB = Self::letters_to_token(b"DXB"),
	
	/// TODO.
	DXC = Self::letters_to_token(b"DXC"),
	
	/// TODO.
	DXD = Self::letters_to_token(b"DXD"),
	
	/// TODO.
	DXE = Self::letters_to_token(b"DXE"),
	
	/// TODO.
	DXF = Self::letters_to_token(b"DXF"),
	
	/// TODO.
	DXG = Self::letters_to_token(b"DXG"),
	
	/// TODO.
	DXH = Self::letters_to_token(b"DXH"),
	
	/// TODO.
	DXI = Self::letters_to_token(b"DXI"),
	
	/// TODO.
	DXJ = Self::letters_to_token(b"DXJ"),
	
	/// TODO.
	DXK = Self::letters_to_token(b"DXK"),
	
	/// TODO.
	DXL = Self::letters_to_token(b"DXL"),
	
	/// TODO.
	DXM = Self::letters_to_token(b"DXM"),
	
	/// TODO.
	DXN = Self::letters_to_token(b"DXN"),
	
	/// TODO.
	DXO = Self::letters_to_token(b"DXO"),
	
	/// TODO.
	DXP = Self::letters_to_token(b"DXP"),
	
	/// TODO.
	DXQ = Self::letters_to_token(b"DXQ"),
	
	/// TODO.
	DXR = Self::letters_to_token(b"DXR"),
	
	/// TODO.
	DXS = Self::letters_to_token(b"DXS"),
	
	/// TODO.
	DXT = Self::letters_to_token(b"DXT"),
	
	/// TODO.
	DXU = Self::letters_to_token(b"DXU"),
	
	/// TODO.
	DXV = Self::letters_to_token(b"DXV"),
	
	/// TODO.
	DXW = Self::letters_to_token(b"DXW"),
	
	/// TODO.
	DXX = Self::letters_to_token(b"DXX"),
	
	/// TODO.
	DXY = Self::letters_to_token(b"DXY"),
	
	/// TODO.
	DXZ = Self::letters_to_token(b"DXZ"),
	
	/// TODO.
	DYA = Self::letters_to_token(b"DYA"),
	
	/// TODO.
	DYB = Self::letters_to_token(b"DYB"),
	
	/// TODO.
	DYC = Self::letters_to_token(b"DYC"),
	
	/// TODO.
	DYD = Self::letters_to_token(b"DYD"),
	
	/// TODO.
	DYE = Self::letters_to_token(b"DYE"),
	
	/// TODO.
	DYF = Self::letters_to_token(b"DYF"),
	
	/// TODO.
	DYG = Self::letters_to_token(b"DYG"),
	
	/// TODO.
	DYH = Self::letters_to_token(b"DYH"),
	
	/// TODO.
	DYI = Self::letters_to_token(b"DYI"),
	
	/// TODO.
	DYJ = Self::letters_to_token(b"DYJ"),
	
	/// TODO.
	DYK = Self::letters_to_token(b"DYK"),
	
	/// TODO.
	DYL = Self::letters_to_token(b"DYL"),
	
	/// TODO.
	DYM = Self::letters_to_token(b"DYM"),
	
	/// TODO.
	DYN = Self::letters_to_token(b"DYN"),
	
	/// TODO.
	DYO = Self::letters_to_token(b"DYO"),
	
	/// TODO.
	DYP = Self::letters_to_token(b"DYP"),
	
	/// TODO.
	DYQ = Self::letters_to_token(b"DYQ"),
	
	/// TODO.
	DYR = Self::letters_to_token(b"DYR"),
	
	/// TODO.
	DYS = Self::letters_to_token(b"DYS"),
	
	/// TODO.
	DYT = Self::letters_to_token(b"DYT"),
	
	/// TODO.
	DYU = Self::letters_to_token(b"DYU"),
	
	/// TODO.
	DYV = Self::letters_to_token(b"DYV"),
	
	/// TODO.
	DYW = Self::letters_to_token(b"DYW"),
	
	/// TODO.
	DYX = Self::letters_to_token(b"DYX"),
	
	/// TODO.
	DYY = Self::letters_to_token(b"DYY"),
	
	/// TODO.
	DYZ = Self::letters_to_token(b"DYZ"),
	
	/// TODO.
	DZA = Self::letters_to_token(b"DZA"),
	
	/// TODO.
	DZB = Self::letters_to_token(b"DZB"),
	
	/// TODO.
	DZC = Self::letters_to_token(b"DZC"),
	
	/// TODO.
	DZD = Self::letters_to_token(b"DZD"),
	
	/// TODO.
	DZE = Self::letters_to_token(b"DZE"),
	
	/// TODO.
	DZF = Self::letters_to_token(b"DZF"),
	
	/// TODO.
	DZG = Self::letters_to_token(b"DZG"),
	
	/// TODO.
	DZH = Self::letters_to_token(b"DZH"),
	
	/// TODO.
	DZI = Self::letters_to_token(b"DZI"),
	
	/// TODO.
	DZJ = Self::letters_to_token(b"DZJ"),
	
	/// TODO.
	DZK = Self::letters_to_token(b"DZK"),
	
	/// TODO.
	DZL = Self::letters_to_token(b"DZL"),
	
	/// TODO.
	DZM = Self::letters_to_token(b"DZM"),
	
	/// TODO.
	DZN = Self::letters_to_token(b"DZN"),
	
	/// TODO.
	DZO = Self::letters_to_token(b"DZO"),
	
	/// TODO.
	DZP = Self::letters_to_token(b"DZP"),
	
	/// TODO.
	DZQ = Self::letters_to_token(b"DZQ"),
	
	/// TODO.
	DZR = Self::letters_to_token(b"DZR"),
	
	/// TODO.
	DZS = Self::letters_to_token(b"DZS"),
	
	/// TODO.
	DZT = Self::letters_to_token(b"DZT"),
	
	/// TODO.
	DZU = Self::letters_to_token(b"DZU"),
	
	/// TODO.
	DZV = Self::letters_to_token(b"DZV"),
	
	/// TODO.
	DZW = Self::letters_to_token(b"DZW"),
	
	/// TODO.
	DZX = Self::letters_to_token(b"DZX"),
	
	/// TODO.
	DZY = Self::letters_to_token(b"DZY"),
	
	/// TODO.
	DZZ = Self::letters_to_token(b"DZZ"),
	
	/// TODO.
	EAA = Self::letters_to_token(b"EAA"),
	
	/// TODO.
	EAB = Self::letters_to_token(b"EAB"),
	
	/// TODO.
	EAC = Self::letters_to_token(b"EAC"),
	
	/// TODO.
	EAD = Self::letters_to_token(b"EAD"),
	
	/// TODO.
	EAE = Self::letters_to_token(b"EAE"),
	
	/// TODO.
	EAF = Self::letters_to_token(b"EAF"),
	
	/// TODO.
	EAG = Self::letters_to_token(b"EAG"),
	
	/// TODO.
	EAH = Self::letters_to_token(b"EAH"),
	
	/// TODO.
	EAI = Self::letters_to_token(b"EAI"),
	
	/// TODO.
	EAJ = Self::letters_to_token(b"EAJ"),
	
	/// TODO.
	EAK = Self::letters_to_token(b"EAK"),
	
	/// TODO.
	EAL = Self::letters_to_token(b"EAL"),
	
	/// TODO.
	EAM = Self::letters_to_token(b"EAM"),
	
	/// TODO.
	EAN = Self::letters_to_token(b"EAN"),
	
	/// TODO.
	EAO = Self::letters_to_token(b"EAO"),
	
	/// TODO.
	EAP = Self::letters_to_token(b"EAP"),
	
	/// TODO.
	EAQ = Self::letters_to_token(b"EAQ"),
	
	/// TODO.
	EAR = Self::letters_to_token(b"EAR"),
	
	/// TODO.
	EAS = Self::letters_to_token(b"EAS"),
	
	/// TODO.
	EAT = Self::letters_to_token(b"EAT"),
	
	/// TODO.
	EAU = Self::letters_to_token(b"EAU"),
	
	/// TODO.
	EAV = Self::letters_to_token(b"EAV"),
	
	/// TODO.
	EAW = Self::letters_to_token(b"EAW"),
	
	/// TODO.
	EAX = Self::letters_to_token(b"EAX"),
	
	/// TODO.
	EAY = Self::letters_to_token(b"EAY"),
	
	/// TODO.
	EAZ = Self::letters_to_token(b"EAZ"),
	
	/// TODO.
	EBA = Self::letters_to_token(b"EBA"),
	
	/// TODO.
	EBB = Self::letters_to_token(b"EBB"),
	
	/// TODO.
	EBC = Self::letters_to_token(b"EBC"),
	
	/// TODO.
	EBD = Self::letters_to_token(b"EBD"),
	
	/// TODO.
	EBE = Self::letters_to_token(b"EBE"),
	
	/// TODO.
	EBF = Self::letters_to_token(b"EBF"),
	
	/// TODO.
	EBG = Self::letters_to_token(b"EBG"),
	
	/// TODO.
	EBH = Self::letters_to_token(b"EBH"),
	
	/// TODO.
	EBI = Self::letters_to_token(b"EBI"),
	
	/// TODO.
	EBJ = Self::letters_to_token(b"EBJ"),
	
	/// TODO.
	EBK = Self::letters_to_token(b"EBK"),
	
	/// TODO.
	EBL = Self::letters_to_token(b"EBL"),
	
	/// TODO.
	EBM = Self::letters_to_token(b"EBM"),
	
	/// TODO.
	EBN = Self::letters_to_token(b"EBN"),
	
	/// TODO.
	EBO = Self::letters_to_token(b"EBO"),
	
	/// TODO.
	EBP = Self::letters_to_token(b"EBP"),
	
	/// TODO.
	EBQ = Self::letters_to_token(b"EBQ"),
	
	/// TODO.
	EBR = Self::letters_to_token(b"EBR"),
	
	/// TODO.
	EBS = Self::letters_to_token(b"EBS"),
	
	/// TODO.
	EBT = Self::letters_to_token(b"EBT"),
	
	/// TODO.
	EBU = Self::letters_to_token(b"EBU"),
	
	/// TODO.
	EBV = Self::letters_to_token(b"EBV"),
	
	/// TODO.
	EBW = Self::letters_to_token(b"EBW"),
	
	/// TODO.
	EBX = Self::letters_to_token(b"EBX"),
	
	/// TODO.
	EBY = Self::letters_to_token(b"EBY"),
	
	/// TODO.
	EBZ = Self::letters_to_token(b"EBZ"),
	
	/// TODO.
	ECA = Self::letters_to_token(b"ECA"),
	
	/// TODO.
	ECB = Self::letters_to_token(b"ECB"),
	
	/// TODO.
	ECC = Self::letters_to_token(b"ECC"),
	
	/// TODO.
	ECD = Self::letters_to_token(b"ECD"),
	
	/// TODO.
	ECE = Self::letters_to_token(b"ECE"),
	
	/// TODO.
	ECF = Self::letters_to_token(b"ECF"),
	
	/// TODO.
	ECG = Self::letters_to_token(b"ECG"),
	
	/// TODO.
	ECH = Self::letters_to_token(b"ECH"),
	
	/// TODO.
	ECI = Self::letters_to_token(b"ECI"),
	
	/// TODO.
	ECJ = Self::letters_to_token(b"ECJ"),
	
	/// TODO.
	ECK = Self::letters_to_token(b"ECK"),
	
	/// TODO.
	ECL = Self::letters_to_token(b"ECL"),
	
	/// TODO.
	ECM = Self::letters_to_token(b"ECM"),
	
	/// TODO.
	ECN = Self::letters_to_token(b"ECN"),
	
	/// TODO.
	ECO = Self::letters_to_token(b"ECO"),
	
	/// TODO.
	ECP = Self::letters_to_token(b"ECP"),
	
	/// TODO.
	ECQ = Self::letters_to_token(b"ECQ"),
	
	/// TODO.
	ECR = Self::letters_to_token(b"ECR"),
	
	/// TODO.
	ECS = Self::letters_to_token(b"ECS"),
	
	/// TODO.
	ECT = Self::letters_to_token(b"ECT"),
	
	/// TODO.
	ECU = Self::letters_to_token(b"ECU"),
	
	/// TODO.
	ECV = Self::letters_to_token(b"ECV"),
	
	/// TODO.
	ECW = Self::letters_to_token(b"ECW"),
	
	/// TODO.
	ECX = Self::letters_to_token(b"ECX"),
	
	/// TODO.
	ECY = Self::letters_to_token(b"ECY"),
	
	/// TODO.
	ECZ = Self::letters_to_token(b"ECZ"),
	
	/// TODO.
	EDA = Self::letters_to_token(b"EDA"),
	
	/// TODO.
	EDB = Self::letters_to_token(b"EDB"),
	
	/// TODO.
	EDC = Self::letters_to_token(b"EDC"),
	
	/// TODO.
	EDD = Self::letters_to_token(b"EDD"),
	
	/// TODO.
	EDE = Self::letters_to_token(b"EDE"),
	
	/// TODO.
	EDF = Self::letters_to_token(b"EDF"),
	
	/// TODO.
	EDG = Self::letters_to_token(b"EDG"),
	
	/// TODO.
	EDH = Self::letters_to_token(b"EDH"),
	
	/// TODO.
	EDI = Self::letters_to_token(b"EDI"),
	
	/// TODO.
	EDJ = Self::letters_to_token(b"EDJ"),
	
	/// TODO.
	EDK = Self::letters_to_token(b"EDK"),
	
	/// TODO.
	EDL = Self::letters_to_token(b"EDL"),
	
	/// TODO.
	EDM = Self::letters_to_token(b"EDM"),
	
	/// TODO.
	EDN = Self::letters_to_token(b"EDN"),
	
	/// TODO.
	EDO = Self::letters_to_token(b"EDO"),
	
	/// TODO.
	EDP = Self::letters_to_token(b"EDP"),
	
	/// TODO.
	EDQ = Self::letters_to_token(b"EDQ"),
	
	/// TODO.
	EDR = Self::letters_to_token(b"EDR"),
	
	/// TODO.
	EDS = Self::letters_to_token(b"EDS"),
	
	/// TODO.
	EDT = Self::letters_to_token(b"EDT"),
	
	/// TODO.
	EDU = Self::letters_to_token(b"EDU"),
	
	/// TODO.
	EDV = Self::letters_to_token(b"EDV"),
	
	/// TODO.
	EDW = Self::letters_to_token(b"EDW"),
	
	/// TODO.
	EDX = Self::letters_to_token(b"EDX"),
	
	/// TODO.
	EDY = Self::letters_to_token(b"EDY"),
	
	/// TODO.
	EDZ = Self::letters_to_token(b"EDZ"),
	
	/// TODO.
	EEA = Self::letters_to_token(b"EEA"),
	
	/// TODO.
	EEB = Self::letters_to_token(b"EEB"),
	
	/// TODO.
	EEC = Self::letters_to_token(b"EEC"),
	
	/// TODO.
	EED = Self::letters_to_token(b"EED"),
	
	/// Unoffical.
	///
	/// Europe (NATO STANAG 1059 INT).
	EEE = Self::letters_to_token(b"EEE"),
	
	/// TODO.
	EEF = Self::letters_to_token(b"EEF"),
	
	/// TODO.
	EEG = Self::letters_to_token(b"EEG"),
	
	/// TODO.
	EEH = Self::letters_to_token(b"EEH"),
	
	/// TODO.
	EEI = Self::letters_to_token(b"EEI"),
	
	/// TODO.
	EEJ = Self::letters_to_token(b"EEJ"),
	
	/// TODO.
	EEK = Self::letters_to_token(b"EEK"),
	
	/// TODO.
	EEL = Self::letters_to_token(b"EEL"),
	
	/// TODO.
	EEM = Self::letters_to_token(b"EEM"),
	
	/// TODO.
	EEN = Self::letters_to_token(b"EEN"),
	
	/// TODO.
	EEO = Self::letters_to_token(b"EEO"),
	
	/// TODO.
	EEP = Self::letters_to_token(b"EEP"),
	
	/// TODO.
	EEQ = Self::letters_to_token(b"EEQ"),
	
	/// TODO.
	EER = Self::letters_to_token(b"EER"),
	
	/// TODO.
	EES = Self::letters_to_token(b"EES"),
	
	/// TODO.
	EET = Self::letters_to_token(b"EET"),
	
	/// TODO.
	EEU = Self::letters_to_token(b"EEU"),
	
	/// TODO.
	EEV = Self::letters_to_token(b"EEV"),
	
	/// TODO.
	EEW = Self::letters_to_token(b"EEW"),
	
	/// TODO.
	EEX = Self::letters_to_token(b"EEX"),
	
	/// TODO.
	EEY = Self::letters_to_token(b"EEY"),
	
	/// TODO.
	EEZ = Self::letters_to_token(b"EEZ"),
	
	/// TODO.
	EFA = Self::letters_to_token(b"EFA"),
	
	/// TODO.
	EFB = Self::letters_to_token(b"EFB"),
	
	/// TODO.
	EFC = Self::letters_to_token(b"EFC"),
	
	/// TODO.
	EFD = Self::letters_to_token(b"EFD"),
	
	/// TODO.
	EFE = Self::letters_to_token(b"EFE"),
	
	/// TODO.
	EFF = Self::letters_to_token(b"EFF"),
	
	/// TODO.
	EFG = Self::letters_to_token(b"EFG"),
	
	/// TODO.
	EFH = Self::letters_to_token(b"EFH"),
	
	/// TODO.
	EFI = Self::letters_to_token(b"EFI"),
	
	/// TODO.
	EFJ = Self::letters_to_token(b"EFJ"),
	
	/// TODO.
	EFK = Self::letters_to_token(b"EFK"),
	
	/// TODO.
	EFL = Self::letters_to_token(b"EFL"),
	
	/// TODO.
	EFM = Self::letters_to_token(b"EFM"),
	
	/// TODO.
	EFN = Self::letters_to_token(b"EFN"),
	
	/// TODO.
	EFO = Self::letters_to_token(b"EFO"),
	
	/// TODO.
	EFP = Self::letters_to_token(b"EFP"),
	
	/// TODO.
	EFQ = Self::letters_to_token(b"EFQ"),
	
	/// TODO.
	EFR = Self::letters_to_token(b"EFR"),
	
	/// TODO.
	EFS = Self::letters_to_token(b"EFS"),
	
	/// TODO.
	EFT = Self::letters_to_token(b"EFT"),
	
	/// TODO.
	EFU = Self::letters_to_token(b"EFU"),
	
	/// TODO.
	EFV = Self::letters_to_token(b"EFV"),
	
	/// TODO.
	EFW = Self::letters_to_token(b"EFW"),
	
	/// TODO.
	EFX = Self::letters_to_token(b"EFX"),
	
	/// TODO.
	EFY = Self::letters_to_token(b"EFY"),
	
	/// TODO.
	EFZ = Self::letters_to_token(b"EFZ"),
	
	/// TODO.
	EGA = Self::letters_to_token(b"EGA"),
	
	/// TODO.
	EGB = Self::letters_to_token(b"EGB"),
	
	/// TODO.
	EGC = Self::letters_to_token(b"EGC"),
	
	/// TODO.
	EGD = Self::letters_to_token(b"EGD"),
	
	/// TODO.
	EGE = Self::letters_to_token(b"EGE"),
	
	/// TODO.
	EGF = Self::letters_to_token(b"EGF"),
	
	/// TODO.
	EGG = Self::letters_to_token(b"EGG"),
	
	/// TODO.
	EGH = Self::letters_to_token(b"EGH"),
	
	/// TODO.
	EGI = Self::letters_to_token(b"EGI"),
	
	/// TODO.
	EGJ = Self::letters_to_token(b"EGJ"),
	
	/// TODO.
	EGK = Self::letters_to_token(b"EGK"),
	
	/// TODO.
	EGL = Self::letters_to_token(b"EGL"),
	
	/// TODO.
	EGM = Self::letters_to_token(b"EGM"),
	
	/// TODO.
	EGN = Self::letters_to_token(b"EGN"),
	
	/// TODO.
	EGO = Self::letters_to_token(b"EGO"),
	
	/// TODO.
	EGP = Self::letters_to_token(b"EGP"),
	
	/// TODO.
	EGQ = Self::letters_to_token(b"EGQ"),
	
	/// TODO.
	EGR = Self::letters_to_token(b"EGR"),
	
	/// TODO.
	EGS = Self::letters_to_token(b"EGS"),
	
	/// TODO.
	EGT = Self::letters_to_token(b"EGT"),
	
	/// TODO.
	EGU = Self::letters_to_token(b"EGU"),
	
	/// TODO.
	EGV = Self::letters_to_token(b"EGV"),
	
	/// TODO.
	EGW = Self::letters_to_token(b"EGW"),
	
	/// TODO.
	EGX = Self::letters_to_token(b"EGX"),
	
	/// TODO.
	EGY = Self::letters_to_token(b"EGY"),
	
	/// TODO.
	EGZ = Self::letters_to_token(b"EGZ"),
	
	/// TODO.
	EHA = Self::letters_to_token(b"EHA"),
	
	/// TODO.
	EHB = Self::letters_to_token(b"EHB"),
	
	/// TODO.
	EHC = Self::letters_to_token(b"EHC"),
	
	/// TODO.
	EHD = Self::letters_to_token(b"EHD"),
	
	/// TODO.
	EHE = Self::letters_to_token(b"EHE"),
	
	/// TODO.
	EHF = Self::letters_to_token(b"EHF"),
	
	/// TODO.
	EHG = Self::letters_to_token(b"EHG"),
	
	/// TODO.
	EHH = Self::letters_to_token(b"EHH"),
	
	/// TODO.
	EHI = Self::letters_to_token(b"EHI"),
	
	/// TODO.
	EHJ = Self::letters_to_token(b"EHJ"),
	
	/// TODO.
	EHK = Self::letters_to_token(b"EHK"),
	
	/// TODO.
	EHL = Self::letters_to_token(b"EHL"),
	
	/// TODO.
	EHM = Self::letters_to_token(b"EHM"),
	
	/// TODO.
	EHN = Self::letters_to_token(b"EHN"),
	
	/// TODO.
	EHO = Self::letters_to_token(b"EHO"),
	
	/// TODO.
	EHP = Self::letters_to_token(b"EHP"),
	
	/// TODO.
	EHQ = Self::letters_to_token(b"EHQ"),
	
	/// TODO.
	EHR = Self::letters_to_token(b"EHR"),
	
	/// TODO.
	EHS = Self::letters_to_token(b"EHS"),
	
	/// TODO.
	EHT = Self::letters_to_token(b"EHT"),
	
	/// TODO.
	EHU = Self::letters_to_token(b"EHU"),
	
	/// TODO.
	EHV = Self::letters_to_token(b"EHV"),
	
	/// TODO.
	EHW = Self::letters_to_token(b"EHW"),
	
	/// TODO.
	EHX = Self::letters_to_token(b"EHX"),
	
	/// TODO.
	EHY = Self::letters_to_token(b"EHY"),
	
	/// TODO.
	EHZ = Self::letters_to_token(b"EHZ"),
	
	/// TODO.
	EIA = Self::letters_to_token(b"EIA"),
	
	/// TODO.
	EIB = Self::letters_to_token(b"EIB"),
	
	/// TODO.
	EIC = Self::letters_to_token(b"EIC"),
	
	/// TODO.
	EID = Self::letters_to_token(b"EID"),
	
	/// TODO.
	EIE = Self::letters_to_token(b"EIE"),
	
	/// TODO.
	EIF = Self::letters_to_token(b"EIF"),
	
	/// TODO.
	EIG = Self::letters_to_token(b"EIG"),
	
	/// TODO.
	EIH = Self::letters_to_token(b"EIH"),
	
	/// TODO.
	EII = Self::letters_to_token(b"EII"),
	
	/// TODO.
	EIJ = Self::letters_to_token(b"EIJ"),
	
	/// TODO.
	EIK = Self::letters_to_token(b"EIK"),
	
	/// TODO.
	EIL = Self::letters_to_token(b"EIL"),
	
	/// TODO.
	EIM = Self::letters_to_token(b"EIM"),
	
	/// TODO.
	EIN = Self::letters_to_token(b"EIN"),
	
	/// TODO.
	EIO = Self::letters_to_token(b"EIO"),
	
	/// TODO.
	EIP = Self::letters_to_token(b"EIP"),
	
	/// TODO.
	EIQ = Self::letters_to_token(b"EIQ"),
	
	/// TODO.
	EIR = Self::letters_to_token(b"EIR"),
	
	/// TODO.
	EIS = Self::letters_to_token(b"EIS"),
	
	/// TODO.
	EIT = Self::letters_to_token(b"EIT"),
	
	/// TODO.
	EIU = Self::letters_to_token(b"EIU"),
	
	/// TODO.
	EIV = Self::letters_to_token(b"EIV"),
	
	/// TODO.
	EIW = Self::letters_to_token(b"EIW"),
	
	/// TODO.
	EIX = Self::letters_to_token(b"EIX"),
	
	/// TODO.
	EIY = Self::letters_to_token(b"EIY"),
	
	/// TODO.
	EIZ = Self::letters_to_token(b"EIZ"),
	
	/// TODO.
	EJA = Self::letters_to_token(b"EJA"),
	
	/// TODO.
	EJB = Self::letters_to_token(b"EJB"),
	
	/// TODO.
	EJC = Self::letters_to_token(b"EJC"),
	
	/// TODO.
	EJD = Self::letters_to_token(b"EJD"),
	
	/// TODO.
	EJE = Self::letters_to_token(b"EJE"),
	
	/// TODO.
	EJF = Self::letters_to_token(b"EJF"),
	
	/// TODO.
	EJG = Self::letters_to_token(b"EJG"),
	
	/// TODO.
	EJH = Self::letters_to_token(b"EJH"),
	
	/// TODO.
	EJI = Self::letters_to_token(b"EJI"),
	
	/// TODO.
	EJJ = Self::letters_to_token(b"EJJ"),
	
	/// TODO.
	EJK = Self::letters_to_token(b"EJK"),
	
	/// TODO.
	EJL = Self::letters_to_token(b"EJL"),
	
	/// TODO.
	EJM = Self::letters_to_token(b"EJM"),
	
	/// TODO.
	EJN = Self::letters_to_token(b"EJN"),
	
	/// TODO.
	EJO = Self::letters_to_token(b"EJO"),
	
	/// TODO.
	EJP = Self::letters_to_token(b"EJP"),
	
	/// TODO.
	EJQ = Self::letters_to_token(b"EJQ"),
	
	/// TODO.
	EJR = Self::letters_to_token(b"EJR"),
	
	/// TODO.
	EJS = Self::letters_to_token(b"EJS"),
	
	/// TODO.
	EJT = Self::letters_to_token(b"EJT"),
	
	/// TODO.
	EJU = Self::letters_to_token(b"EJU"),
	
	/// TODO.
	EJV = Self::letters_to_token(b"EJV"),
	
	/// TODO.
	EJW = Self::letters_to_token(b"EJW"),
	
	/// TODO.
	EJX = Self::letters_to_token(b"EJX"),
	
	/// TODO.
	EJY = Self::letters_to_token(b"EJY"),
	
	/// TODO.
	EJZ = Self::letters_to_token(b"EJZ"),
	
	/// TODO.
	EKA = Self::letters_to_token(b"EKA"),
	
	/// TODO.
	EKB = Self::letters_to_token(b"EKB"),
	
	/// TODO.
	EKC = Self::letters_to_token(b"EKC"),
	
	/// TODO.
	EKD = Self::letters_to_token(b"EKD"),
	
	/// TODO.
	EKE = Self::letters_to_token(b"EKE"),
	
	/// TODO.
	EKF = Self::letters_to_token(b"EKF"),
	
	/// TODO.
	EKG = Self::letters_to_token(b"EKG"),
	
	/// TODO.
	EKH = Self::letters_to_token(b"EKH"),
	
	/// TODO.
	EKI = Self::letters_to_token(b"EKI"),
	
	/// TODO.
	EKJ = Self::letters_to_token(b"EKJ"),
	
	/// TODO.
	EKK = Self::letters_to_token(b"EKK"),
	
	/// TODO.
	EKL = Self::letters_to_token(b"EKL"),
	
	/// TODO.
	EKM = Self::letters_to_token(b"EKM"),
	
	/// TODO.
	EKN = Self::letters_to_token(b"EKN"),
	
	/// TODO.
	EKO = Self::letters_to_token(b"EKO"),
	
	/// TODO.
	EKP = Self::letters_to_token(b"EKP"),
	
	/// TODO.
	EKQ = Self::letters_to_token(b"EKQ"),
	
	/// TODO.
	EKR = Self::letters_to_token(b"EKR"),
	
	/// TODO.
	EKS = Self::letters_to_token(b"EKS"),
	
	/// TODO.
	EKT = Self::letters_to_token(b"EKT"),
	
	/// TODO.
	EKU = Self::letters_to_token(b"EKU"),
	
	/// TODO.
	EKV = Self::letters_to_token(b"EKV"),
	
	/// TODO.
	EKW = Self::letters_to_token(b"EKW"),
	
	/// TODO.
	EKX = Self::letters_to_token(b"EKX"),
	
	/// TODO.
	EKY = Self::letters_to_token(b"EKY"),
	
	/// TODO.
	EKZ = Self::letters_to_token(b"EKZ"),
	
	/// TODO.
	ELA = Self::letters_to_token(b"ELA"),
	
	/// TODO.
	ELB = Self::letters_to_token(b"ELB"),
	
	/// TODO.
	ELC = Self::letters_to_token(b"ELC"),
	
	/// TODO.
	ELD = Self::letters_to_token(b"ELD"),
	
	/// TODO.
	ELE = Self::letters_to_token(b"ELE"),
	
	/// TODO.
	ELF = Self::letters_to_token(b"ELF"),
	
	/// TODO.
	ELG = Self::letters_to_token(b"ELG"),
	
	/// TODO.
	ELH = Self::letters_to_token(b"ELH"),
	
	/// TODO.
	ELI = Self::letters_to_token(b"ELI"),
	
	/// TODO.
	ELJ = Self::letters_to_token(b"ELJ"),
	
	/// TODO.
	ELK = Self::letters_to_token(b"ELK"),
	
	/// TODO.
	ELL = Self::letters_to_token(b"ELL"),
	
	/// TODO.
	ELM = Self::letters_to_token(b"ELM"),
	
	/// TODO.
	ELN = Self::letters_to_token(b"ELN"),
	
	/// TODO.
	ELO = Self::letters_to_token(b"ELO"),
	
	/// TODO.
	ELP = Self::letters_to_token(b"ELP"),
	
	/// TODO.
	ELQ = Self::letters_to_token(b"ELQ"),
	
	/// TODO.
	ELR = Self::letters_to_token(b"ELR"),
	
	/// TODO.
	ELS = Self::letters_to_token(b"ELS"),
	
	/// TODO.
	ELT = Self::letters_to_token(b"ELT"),
	
	/// TODO.
	ELU = Self::letters_to_token(b"ELU"),
	
	/// TODO.
	ELV = Self::letters_to_token(b"ELV"),
	
	/// TODO.
	ELW = Self::letters_to_token(b"ELW"),
	
	/// TODO.
	ELX = Self::letters_to_token(b"ELX"),
	
	/// TODO.
	ELY = Self::letters_to_token(b"ELY"),
	
	/// TODO.
	ELZ = Self::letters_to_token(b"ELZ"),
	
	/// TODO.
	EMA = Self::letters_to_token(b"EMA"),
	
	/// TODO.
	EMB = Self::letters_to_token(b"EMB"),
	
	/// TODO.
	EMC = Self::letters_to_token(b"EMC"),
	
	/// TODO.
	EMD = Self::letters_to_token(b"EMD"),
	
	/// TODO.
	EME = Self::letters_to_token(b"EME"),
	
	/// TODO.
	EMF = Self::letters_to_token(b"EMF"),
	
	/// TODO.
	EMG = Self::letters_to_token(b"EMG"),
	
	/// TODO.
	EMH = Self::letters_to_token(b"EMH"),
	
	/// TODO.
	EMI = Self::letters_to_token(b"EMI"),
	
	/// TODO.
	EMJ = Self::letters_to_token(b"EMJ"),
	
	/// TODO.
	EMK = Self::letters_to_token(b"EMK"),
	
	/// TODO.
	EML = Self::letters_to_token(b"EML"),
	
	/// TODO.
	EMM = Self::letters_to_token(b"EMM"),
	
	/// TODO.
	EMN = Self::letters_to_token(b"EMN"),
	
	/// TODO.
	EMO = Self::letters_to_token(b"EMO"),
	
	/// TODO.
	EMP = Self::letters_to_token(b"EMP"),
	
	/// TODO.
	EMQ = Self::letters_to_token(b"EMQ"),
	
	/// TODO.
	EMR = Self::letters_to_token(b"EMR"),
	
	/// TODO.
	EMS = Self::letters_to_token(b"EMS"),
	
	/// TODO.
	EMT = Self::letters_to_token(b"EMT"),
	
	/// TODO.
	EMU = Self::letters_to_token(b"EMU"),
	
	/// TODO.
	EMV = Self::letters_to_token(b"EMV"),
	
	/// TODO.
	EMW = Self::letters_to_token(b"EMW"),
	
	/// TODO.
	EMX = Self::letters_to_token(b"EMX"),
	
	/// TODO.
	EMY = Self::letters_to_token(b"EMY"),
	
	/// TODO.
	EMZ = Self::letters_to_token(b"EMZ"),
	
	/// TODO.
	ENA = Self::letters_to_token(b"ENA"),
	
	/// TODO.
	ENB = Self::letters_to_token(b"ENB"),
	
	/// TODO.
	ENC = Self::letters_to_token(b"ENC"),
	
	/// TODO.
	END = Self::letters_to_token(b"END"),
	
	/// TODO.
	ENE = Self::letters_to_token(b"ENE"),
	
	/// TODO.
	ENF = Self::letters_to_token(b"ENF"),
	
	/// TODO.
	ENG = Self::letters_to_token(b"ENG"),
	
	/// TODO.
	ENH = Self::letters_to_token(b"ENH"),
	
	/// TODO.
	ENI = Self::letters_to_token(b"ENI"),
	
	/// TODO.
	ENJ = Self::letters_to_token(b"ENJ"),
	
	/// TODO.
	ENK = Self::letters_to_token(b"ENK"),
	
	/// TODO.
	ENL = Self::letters_to_token(b"ENL"),
	
	/// TODO.
	ENM = Self::letters_to_token(b"ENM"),
	
	/// TODO.
	ENN = Self::letters_to_token(b"ENN"),
	
	/// TODO.
	ENO = Self::letters_to_token(b"ENO"),
	
	/// TODO.
	ENP = Self::letters_to_token(b"ENP"),
	
	/// TODO.
	ENQ = Self::letters_to_token(b"ENQ"),
	
	/// TODO.
	ENR = Self::letters_to_token(b"ENR"),
	
	/// TODO.
	ENS = Self::letters_to_token(b"ENS"),
	
	/// TODO.
	ENT = Self::letters_to_token(b"ENT"),
	
	/// TODO.
	ENU = Self::letters_to_token(b"ENU"),
	
	/// TODO.
	ENV = Self::letters_to_token(b"ENV"),
	
	/// TODO.
	ENW = Self::letters_to_token(b"ENW"),
	
	/// TODO.
	ENX = Self::letters_to_token(b"ENX"),
	
	/// TODO.
	ENY = Self::letters_to_token(b"ENY"),
	
	/// TODO.
	ENZ = Self::letters_to_token(b"ENZ"),
	
	/// TODO.
	EOA = Self::letters_to_token(b"EOA"),
	
	/// TODO.
	EOB = Self::letters_to_token(b"EOB"),
	
	/// TODO.
	EOC = Self::letters_to_token(b"EOC"),
	
	/// TODO.
	EOD = Self::letters_to_token(b"EOD"),
	
	/// TODO.
	EOE = Self::letters_to_token(b"EOE"),
	
	/// TODO.
	EOF = Self::letters_to_token(b"EOF"),
	
	/// TODO.
	EOG = Self::letters_to_token(b"EOG"),
	
	/// TODO.
	EOH = Self::letters_to_token(b"EOH"),
	
	/// TODO.
	EOI = Self::letters_to_token(b"EOI"),
	
	/// TODO.
	EOJ = Self::letters_to_token(b"EOJ"),
	
	/// TODO.
	EOK = Self::letters_to_token(b"EOK"),
	
	/// TODO.
	EOL = Self::letters_to_token(b"EOL"),
	
	/// TODO.
	EOM = Self::letters_to_token(b"EOM"),
	
	/// TODO.
	EON = Self::letters_to_token(b"EON"),
	
	/// TODO.
	EOO = Self::letters_to_token(b"EOO"),
	
	/// TODO.
	EOP = Self::letters_to_token(b"EOP"),
	
	/// TODO.
	EOQ = Self::letters_to_token(b"EOQ"),
	
	/// TODO.
	EOR = Self::letters_to_token(b"EOR"),
	
	/// TODO.
	EOS = Self::letters_to_token(b"EOS"),
	
	/// TODO.
	EOT = Self::letters_to_token(b"EOT"),
	
	/// TODO.
	EOU = Self::letters_to_token(b"EOU"),
	
	/// TODO.
	EOV = Self::letters_to_token(b"EOV"),
	
	/// TODO.
	EOW = Self::letters_to_token(b"EOW"),
	
	/// TODO.
	EOX = Self::letters_to_token(b"EOX"),
	
	/// TODO.
	EOY = Self::letters_to_token(b"EOY"),
	
	/// TODO.
	EOZ = Self::letters_to_token(b"EOZ"),
	
	/// TODO.
	EPA = Self::letters_to_token(b"EPA"),
	
	/// TODO.
	EPB = Self::letters_to_token(b"EPB"),
	
	/// TODO.
	EPC = Self::letters_to_token(b"EPC"),
	
	/// TODO.
	EPD = Self::letters_to_token(b"EPD"),
	
	/// TODO.
	EPE = Self::letters_to_token(b"EPE"),
	
	/// TODO.
	EPF = Self::letters_to_token(b"EPF"),
	
	/// TODO.
	EPG = Self::letters_to_token(b"EPG"),
	
	/// TODO.
	EPH = Self::letters_to_token(b"EPH"),
	
	/// TODO.
	EPI = Self::letters_to_token(b"EPI"),
	
	/// TODO.
	EPJ = Self::letters_to_token(b"EPJ"),
	
	/// TODO.
	EPK = Self::letters_to_token(b"EPK"),
	
	/// TODO.
	EPL = Self::letters_to_token(b"EPL"),
	
	/// TODO.
	EPM = Self::letters_to_token(b"EPM"),
	
	/// TODO.
	EPN = Self::letters_to_token(b"EPN"),
	
	/// TODO.
	EPO = Self::letters_to_token(b"EPO"),
	
	/// TODO.
	EPP = Self::letters_to_token(b"EPP"),
	
	/// TODO.
	EPQ = Self::letters_to_token(b"EPQ"),
	
	/// TODO.
	EPR = Self::letters_to_token(b"EPR"),
	
	/// TODO.
	EPS = Self::letters_to_token(b"EPS"),
	
	/// TODO.
	EPT = Self::letters_to_token(b"EPT"),
	
	/// TODO.
	EPU = Self::letters_to_token(b"EPU"),
	
	/// TODO.
	EPV = Self::letters_to_token(b"EPV"),
	
	/// TODO.
	EPW = Self::letters_to_token(b"EPW"),
	
	/// TODO.
	EPX = Self::letters_to_token(b"EPX"),
	
	/// TODO.
	EPY = Self::letters_to_token(b"EPY"),
	
	/// TODO.
	EPZ = Self::letters_to_token(b"EPZ"),
	
	/// TODO.
	EQA = Self::letters_to_token(b"EQA"),
	
	/// TODO.
	EQB = Self::letters_to_token(b"EQB"),
	
	/// TODO.
	EQC = Self::letters_to_token(b"EQC"),
	
	/// TODO.
	EQD = Self::letters_to_token(b"EQD"),
	
	/// TODO.
	EQE = Self::letters_to_token(b"EQE"),
	
	/// TODO.
	EQF = Self::letters_to_token(b"EQF"),
	
	/// TODO.
	EQG = Self::letters_to_token(b"EQG"),
	
	/// TODO.
	EQH = Self::letters_to_token(b"EQH"),
	
	/// TODO.
	EQI = Self::letters_to_token(b"EQI"),
	
	/// TODO.
	EQJ = Self::letters_to_token(b"EQJ"),
	
	/// TODO.
	EQK = Self::letters_to_token(b"EQK"),
	
	/// TODO.
	EQL = Self::letters_to_token(b"EQL"),
	
	/// TODO.
	EQM = Self::letters_to_token(b"EQM"),
	
	/// TODO.
	EQN = Self::letters_to_token(b"EQN"),
	
	/// TODO.
	EQO = Self::letters_to_token(b"EQO"),
	
	/// TODO.
	EQP = Self::letters_to_token(b"EQP"),
	
	/// TODO.
	EQQ = Self::letters_to_token(b"EQQ"),
	
	/// TODO.
	EQR = Self::letters_to_token(b"EQR"),
	
	/// TODO.
	EQS = Self::letters_to_token(b"EQS"),
	
	/// TODO.
	EQT = Self::letters_to_token(b"EQT"),
	
	/// TODO.
	EQU = Self::letters_to_token(b"EQU"),
	
	/// TODO.
	EQV = Self::letters_to_token(b"EQV"),
	
	/// TODO.
	EQW = Self::letters_to_token(b"EQW"),
	
	/// TODO.
	EQX = Self::letters_to_token(b"EQX"),
	
	/// TODO.
	EQY = Self::letters_to_token(b"EQY"),
	
	/// TODO.
	EQZ = Self::letters_to_token(b"EQZ"),
	
	/// TODO.
	ERA = Self::letters_to_token(b"ERA"),
	
	/// TODO.
	ERB = Self::letters_to_token(b"ERB"),
	
	/// TODO.
	ERC = Self::letters_to_token(b"ERC"),
	
	/// TODO.
	ERD = Self::letters_to_token(b"ERD"),
	
	/// TODO.
	ERE = Self::letters_to_token(b"ERE"),
	
	/// TODO.
	ERF = Self::letters_to_token(b"ERF"),
	
	/// TODO.
	ERG = Self::letters_to_token(b"ERG"),
	
	/// TODO.
	ERH = Self::letters_to_token(b"ERH"),
	
	/// TODO.
	ERI = Self::letters_to_token(b"ERI"),
	
	/// TODO.
	ERJ = Self::letters_to_token(b"ERJ"),
	
	/// TODO.
	ERK = Self::letters_to_token(b"ERK"),
	
	/// TODO.
	ERL = Self::letters_to_token(b"ERL"),
	
	/// TODO.
	ERM = Self::letters_to_token(b"ERM"),
	
	/// TODO.
	ERN = Self::letters_to_token(b"ERN"),
	
	/// TODO.
	ERO = Self::letters_to_token(b"ERO"),
	
	/// TODO.
	ERP = Self::letters_to_token(b"ERP"),
	
	/// TODO.
	ERQ = Self::letters_to_token(b"ERQ"),
	
	/// TODO.
	ERR = Self::letters_to_token(b"ERR"),
	
	/// TODO.
	ERS = Self::letters_to_token(b"ERS"),
	
	/// TODO.
	ERT = Self::letters_to_token(b"ERT"),
	
	/// TODO.
	ERU = Self::letters_to_token(b"ERU"),
	
	/// TODO.
	ERV = Self::letters_to_token(b"ERV"),
	
	/// TODO.
	ERW = Self::letters_to_token(b"ERW"),
	
	/// TODO.
	ERX = Self::letters_to_token(b"ERX"),
	
	/// TODO.
	ERY = Self::letters_to_token(b"ERY"),
	
	/// TODO.
	ERZ = Self::letters_to_token(b"ERZ"),
	
	/// TODO.
	ESA = Self::letters_to_token(b"ESA"),
	
	/// TODO.
	ESB = Self::letters_to_token(b"ESB"),
	
	/// TODO.
	ESC = Self::letters_to_token(b"ESC"),
	
	/// TODO.
	ESD = Self::letters_to_token(b"ESD"),
	
	/// TODO.
	ESE = Self::letters_to_token(b"ESE"),
	
	/// TODO.
	ESF = Self::letters_to_token(b"ESF"),
	
	/// TODO.
	ESG = Self::letters_to_token(b"ESG"),
	
	/// TODO.
	ESH = Self::letters_to_token(b"ESH"),
	
	/// TODO.
	ESI = Self::letters_to_token(b"ESI"),
	
	/// TODO.
	ESJ = Self::letters_to_token(b"ESJ"),
	
	/// TODO.
	ESK = Self::letters_to_token(b"ESK"),
	
	/// TODO.
	ESL = Self::letters_to_token(b"ESL"),
	
	/// TODO.
	ESM = Self::letters_to_token(b"ESM"),
	
	/// TODO.
	ESN = Self::letters_to_token(b"ESN"),
	
	/// TODO.
	ESO = Self::letters_to_token(b"ESO"),
	
	/// Spain.
	ESP = Self::letters_to_token(b"ESP"),
	
	/// TODO.
	ESQ = Self::letters_to_token(b"ESQ"),
	
	/// TODO.
	ESR = Self::letters_to_token(b"ESR"),
	
	/// TODO.
	ESS = Self::letters_to_token(b"ESS"),
	
	/// TODO.
	EST = Self::letters_to_token(b"EST"),
	
	/// TODO.
	ESU = Self::letters_to_token(b"ESU"),
	
	/// TODO.
	ESV = Self::letters_to_token(b"ESV"),
	
	/// TODO.
	ESW = Self::letters_to_token(b"ESW"),
	
	/// TODO.
	ESX = Self::letters_to_token(b"ESX"),
	
	/// TODO.
	ESY = Self::letters_to_token(b"ESY"),
	
	/// TODO.
	ESZ = Self::letters_to_token(b"ESZ"),
	
	/// TODO.
	ETA = Self::letters_to_token(b"ETA"),
	
	/// TODO.
	ETB = Self::letters_to_token(b"ETB"),
	
	/// TODO.
	ETC = Self::letters_to_token(b"ETC"),
	
	/// TODO.
	ETD = Self::letters_to_token(b"ETD"),
	
	/// TODO.
	ETE = Self::letters_to_token(b"ETE"),
	
	/// TODO.
	ETF = Self::letters_to_token(b"ETF"),
	
	/// TODO.
	ETG = Self::letters_to_token(b"ETG"),
	
	/// TODO.
	ETH = Self::letters_to_token(b"ETH"),
	
	/// TODO.
	ETI = Self::letters_to_token(b"ETI"),
	
	/// TODO.
	ETJ = Self::letters_to_token(b"ETJ"),
	
	/// TODO.
	ETK = Self::letters_to_token(b"ETK"),
	
	/// TODO.
	ETL = Self::letters_to_token(b"ETL"),
	
	/// TODO.
	ETM = Self::letters_to_token(b"ETM"),
	
	/// TODO.
	ETN = Self::letters_to_token(b"ETN"),
	
	/// TODO.
	ETO = Self::letters_to_token(b"ETO"),
	
	/// TODO.
	ETP = Self::letters_to_token(b"ETP"),
	
	/// TODO.
	ETQ = Self::letters_to_token(b"ETQ"),
	
	/// TODO.
	ETR = Self::letters_to_token(b"ETR"),
	
	/// TODO.
	ETS = Self::letters_to_token(b"ETS"),
	
	/// TODO.
	ETT = Self::letters_to_token(b"ETT"),
	
	/// TODO.
	ETU = Self::letters_to_token(b"ETU"),
	
	/// TODO.
	ETV = Self::letters_to_token(b"ETV"),
	
	/// TODO.
	ETW = Self::letters_to_token(b"ETW"),
	
	/// TODO.
	ETX = Self::letters_to_token(b"ETX"),
	
	/// TODO.
	ETY = Self::letters_to_token(b"ETY"),
	
	/// TODO.
	ETZ = Self::letters_to_token(b"ETZ"),
	
	/// TODO.
	EUA = Self::letters_to_token(b"EUA"),
	
	/// TODO.
	EUB = Self::letters_to_token(b"EUB"),
	
	/// TODO.
	EUC = Self::letters_to_token(b"EUC"),
	
	/// TODO.
	EUD = Self::letters_to_token(b"EUD"),
	
	/// Unoffical.
	///
	/// European Union laissez-passer 'passport' code.
	EUE = Self::letters_to_token(b"EUE"),
	
	/// TODO.
	EUF = Self::letters_to_token(b"EUF"),
	
	/// TODO.
	EUG = Self::letters_to_token(b"EUG"),
	
	/// TODO.
	EUH = Self::letters_to_token(b"EUH"),
	
	/// TODO.
	EUI = Self::letters_to_token(b"EUI"),
	
	/// TODO.
	EUJ = Self::letters_to_token(b"EUJ"),
	
	/// TODO.
	EUK = Self::letters_to_token(b"EUK"),
	
	/// TODO.
	EUL = Self::letters_to_token(b"EUL"),
	
	/// TODO.
	EUM = Self::letters_to_token(b"EUM"),
	
	/// TODO.
	EUN = Self::letters_to_token(b"EUN"),
	
	/// TODO.
	EUO = Self::letters_to_token(b"EUO"),
	
	/// TODO.
	EUP = Self::letters_to_token(b"EUP"),
	
	/// TODO.
	EUQ = Self::letters_to_token(b"EUQ"),
	
	/// TODO.
	EUR = Self::letters_to_token(b"EUR"),
	
	/// TODO.
	EUS = Self::letters_to_token(b"EUS"),
	
	/// TODO.
	EUT = Self::letters_to_token(b"EUT"),
	
	/// TODO.
	EUU = Self::letters_to_token(b"EUU"),
	
	/// TODO.
	EUV = Self::letters_to_token(b"EUV"),
	
	/// TODO.
	EUW = Self::letters_to_token(b"EUW"),
	
	/// TODO.
	EUX = Self::letters_to_token(b"EUX"),
	
	/// TODO.
	EUY = Self::letters_to_token(b"EUY"),
	
	/// TODO.
	EUZ = Self::letters_to_token(b"EUZ"),
	
	/// TODO.
	EVA = Self::letters_to_token(b"EVA"),
	
	/// TODO.
	EVB = Self::letters_to_token(b"EVB"),
	
	/// TODO.
	EVC = Self::letters_to_token(b"EVC"),
	
	/// TODO.
	EVD = Self::letters_to_token(b"EVD"),
	
	/// TODO.
	EVE = Self::letters_to_token(b"EVE"),
	
	/// TODO.
	EVF = Self::letters_to_token(b"EVF"),
	
	/// TODO.
	EVG = Self::letters_to_token(b"EVG"),
	
	/// TODO.
	EVH = Self::letters_to_token(b"EVH"),
	
	/// TODO.
	EVI = Self::letters_to_token(b"EVI"),
	
	/// TODO.
	EVJ = Self::letters_to_token(b"EVJ"),
	
	/// TODO.
	EVK = Self::letters_to_token(b"EVK"),
	
	/// TODO.
	EVL = Self::letters_to_token(b"EVL"),
	
	/// TODO.
	EVM = Self::letters_to_token(b"EVM"),
	
	/// TODO.
	EVN = Self::letters_to_token(b"EVN"),
	
	/// TODO.
	EVO = Self::letters_to_token(b"EVO"),
	
	/// TODO.
	EVP = Self::letters_to_token(b"EVP"),
	
	/// TODO.
	EVQ = Self::letters_to_token(b"EVQ"),
	
	/// TODO.
	EVR = Self::letters_to_token(b"EVR"),
	
	/// TODO.
	EVS = Self::letters_to_token(b"EVS"),
	
	/// TODO.
	EVT = Self::letters_to_token(b"EVT"),
	
	/// TODO.
	EVU = Self::letters_to_token(b"EVU"),
	
	/// TODO.
	EVV = Self::letters_to_token(b"EVV"),
	
	/// TODO.
	EVW = Self::letters_to_token(b"EVW"),
	
	/// TODO.
	EVX = Self::letters_to_token(b"EVX"),
	
	/// TODO.
	EVY = Self::letters_to_token(b"EVY"),
	
	/// TODO.
	EVZ = Self::letters_to_token(b"EVZ"),
	
	/// TODO.
	EWA = Self::letters_to_token(b"EWA"),
	
	/// TODO.
	EWB = Self::letters_to_token(b"EWB"),
	
	/// TODO.
	EWC = Self::letters_to_token(b"EWC"),
	
	/// TODO.
	EWD = Self::letters_to_token(b"EWD"),
	
	/// TODO.
	EWE = Self::letters_to_token(b"EWE"),
	
	/// TODO.
	EWF = Self::letters_to_token(b"EWF"),
	
	/// TODO.
	EWG = Self::letters_to_token(b"EWG"),
	
	/// TODO.
	EWH = Self::letters_to_token(b"EWH"),
	
	/// TODO.
	EWI = Self::letters_to_token(b"EWI"),
	
	/// TODO.
	EWJ = Self::letters_to_token(b"EWJ"),
	
	/// TODO.
	EWK = Self::letters_to_token(b"EWK"),
	
	/// TODO.
	EWL = Self::letters_to_token(b"EWL"),
	
	/// TODO.
	EWM = Self::letters_to_token(b"EWM"),
	
	/// TODO.
	EWN = Self::letters_to_token(b"EWN"),
	
	/// TODO.
	EWO = Self::letters_to_token(b"EWO"),
	
	/// TODO.
	EWP = Self::letters_to_token(b"EWP"),
	
	/// TODO.
	EWQ = Self::letters_to_token(b"EWQ"),
	
	/// TODO.
	EWR = Self::letters_to_token(b"EWR"),
	
	/// TODO.
	EWS = Self::letters_to_token(b"EWS"),
	
	/// TODO.
	EWT = Self::letters_to_token(b"EWT"),
	
	/// TODO.
	EWU = Self::letters_to_token(b"EWU"),
	
	/// TODO.
	EWV = Self::letters_to_token(b"EWV"),
	
	/// TODO.
	EWW = Self::letters_to_token(b"EWW"),
	
	/// TODO.
	EWX = Self::letters_to_token(b"EWX"),
	
	/// TODO.
	EWY = Self::letters_to_token(b"EWY"),
	
	/// TODO.
	EWZ = Self::letters_to_token(b"EWZ"),
	
	/// TODO.
	EXA = Self::letters_to_token(b"EXA"),
	
	/// TODO.
	EXB = Self::letters_to_token(b"EXB"),
	
	/// TODO.
	EXC = Self::letters_to_token(b"EXC"),
	
	/// TODO.
	EXD = Self::letters_to_token(b"EXD"),
	
	/// TODO.
	EXE = Self::letters_to_token(b"EXE"),
	
	/// TODO.
	EXF = Self::letters_to_token(b"EXF"),
	
	/// TODO.
	EXG = Self::letters_to_token(b"EXG"),
	
	/// TODO.
	EXH = Self::letters_to_token(b"EXH"),
	
	/// TODO.
	EXI = Self::letters_to_token(b"EXI"),
	
	/// TODO.
	EXJ = Self::letters_to_token(b"EXJ"),
	
	/// TODO.
	EXK = Self::letters_to_token(b"EXK"),
	
	/// TODO.
	EXL = Self::letters_to_token(b"EXL"),
	
	/// TODO.
	EXM = Self::letters_to_token(b"EXM"),
	
	/// TODO.
	EXN = Self::letters_to_token(b"EXN"),
	
	/// TODO.
	EXO = Self::letters_to_token(b"EXO"),
	
	/// TODO.
	EXP = Self::letters_to_token(b"EXP"),
	
	/// TODO.
	EXQ = Self::letters_to_token(b"EXQ"),
	
	/// TODO.
	EXR = Self::letters_to_token(b"EXR"),
	
	/// TODO.
	EXS = Self::letters_to_token(b"EXS"),
	
	/// TODO.
	EXT = Self::letters_to_token(b"EXT"),
	
	/// TODO.
	EXU = Self::letters_to_token(b"EXU"),
	
	/// TODO.
	EXV = Self::letters_to_token(b"EXV"),
	
	/// TODO.
	EXW = Self::letters_to_token(b"EXW"),
	
	/// TODO.
	EXX = Self::letters_to_token(b"EXX"),
	
	/// TODO.
	EXY = Self::letters_to_token(b"EXY"),
	
	/// TODO.
	EXZ = Self::letters_to_token(b"EXZ"),
	
	/// TODO.
	EYA = Self::letters_to_token(b"EYA"),
	
	/// TODO.
	EYB = Self::letters_to_token(b"EYB"),
	
	/// TODO.
	EYC = Self::letters_to_token(b"EYC"),
	
	/// TODO.
	EYD = Self::letters_to_token(b"EYD"),
	
	/// TODO.
	EYE = Self::letters_to_token(b"EYE"),
	
	/// TODO.
	EYF = Self::letters_to_token(b"EYF"),
	
	/// TODO.
	EYG = Self::letters_to_token(b"EYG"),
	
	/// TODO.
	EYH = Self::letters_to_token(b"EYH"),
	
	/// TODO.
	EYI = Self::letters_to_token(b"EYI"),
	
	/// TODO.
	EYJ = Self::letters_to_token(b"EYJ"),
	
	/// TODO.
	EYK = Self::letters_to_token(b"EYK"),
	
	/// TODO.
	EYL = Self::letters_to_token(b"EYL"),
	
	/// TODO.
	EYM = Self::letters_to_token(b"EYM"),
	
	/// TODO.
	EYN = Self::letters_to_token(b"EYN"),
	
	/// TODO.
	EYO = Self::letters_to_token(b"EYO"),
	
	/// TODO.
	EYP = Self::letters_to_token(b"EYP"),
	
	/// TODO.
	EYQ = Self::letters_to_token(b"EYQ"),
	
	/// TODO.
	EYR = Self::letters_to_token(b"EYR"),
	
	/// TODO.
	EYS = Self::letters_to_token(b"EYS"),
	
	/// TODO.
	EYT = Self::letters_to_token(b"EYT"),
	
	/// TODO.
	EYU = Self::letters_to_token(b"EYU"),
	
	/// TODO.
	EYV = Self::letters_to_token(b"EYV"),
	
	/// TODO.
	EYW = Self::letters_to_token(b"EYW"),
	
	/// TODO.
	EYX = Self::letters_to_token(b"EYX"),
	
	/// TODO.
	EYY = Self::letters_to_token(b"EYY"),
	
	/// TODO.
	EYZ = Self::letters_to_token(b"EYZ"),
	
	/// TODO.
	EZA = Self::letters_to_token(b"EZA"),
	
	/// TODO.
	EZB = Self::letters_to_token(b"EZB"),
	
	/// TODO.
	EZC = Self::letters_to_token(b"EZC"),
	
	/// TODO.
	EZD = Self::letters_to_token(b"EZD"),
	
	/// TODO.
	EZE = Self::letters_to_token(b"EZE"),
	
	/// TODO.
	EZF = Self::letters_to_token(b"EZF"),
	
	/// TODO.
	EZG = Self::letters_to_token(b"EZG"),
	
	/// TODO.
	EZH = Self::letters_to_token(b"EZH"),
	
	/// TODO.
	EZI = Self::letters_to_token(b"EZI"),
	
	/// TODO.
	EZJ = Self::letters_to_token(b"EZJ"),
	
	/// TODO.
	EZK = Self::letters_to_token(b"EZK"),
	
	/// TODO.
	EZL = Self::letters_to_token(b"EZL"),
	
	/// TODO.
	EZM = Self::letters_to_token(b"EZM"),
	
	/// TODO.
	EZN = Self::letters_to_token(b"EZN"),
	
	/// TODO.
	EZO = Self::letters_to_token(b"EZO"),
	
	/// TODO.
	EZP = Self::letters_to_token(b"EZP"),
	
	/// TODO.
	EZQ = Self::letters_to_token(b"EZQ"),
	
	/// TODO.
	EZR = Self::letters_to_token(b"EZR"),
	
	/// TODO.
	EZS = Self::letters_to_token(b"EZS"),
	
	/// TODO.
	EZT = Self::letters_to_token(b"EZT"),
	
	/// TODO.
	EZU = Self::letters_to_token(b"EZU"),
	
	/// TODO.
	EZV = Self::letters_to_token(b"EZV"),
	
	/// TODO.
	EZW = Self::letters_to_token(b"EZW"),
	
	/// TODO.
	EZX = Self::letters_to_token(b"EZX"),
	
	/// TODO.
	EZY = Self::letters_to_token(b"EZY"),
	
	/// TODO.
	EZZ = Self::letters_to_token(b"EZZ"),
	
	/// TODO.
	FAA = Self::letters_to_token(b"FAA"),
	
	/// TODO.
	FAB = Self::letters_to_token(b"FAB"),
	
	/// TODO.
	FAC = Self::letters_to_token(b"FAC"),
	
	/// TODO.
	FAD = Self::letters_to_token(b"FAD"),
	
	/// TODO.
	FAE = Self::letters_to_token(b"FAE"),
	
	/// TODO.
	FAF = Self::letters_to_token(b"FAF"),
	
	/// TODO.
	FAG = Self::letters_to_token(b"FAG"),
	
	/// TODO.
	FAH = Self::letters_to_token(b"FAH"),
	
	/// TODO.
	FAI = Self::letters_to_token(b"FAI"),
	
	/// TODO.
	FAJ = Self::letters_to_token(b"FAJ"),
	
	/// TODO.
	FAK = Self::letters_to_token(b"FAK"),
	
	/// TODO.
	FAL = Self::letters_to_token(b"FAL"),
	
	/// TODO.
	FAM = Self::letters_to_token(b"FAM"),
	
	/// TODO.
	FAN = Self::letters_to_token(b"FAN"),
	
	/// TODO.
	FAO = Self::letters_to_token(b"FAO"),
	
	/// TODO.
	FAP = Self::letters_to_token(b"FAP"),
	
	/// TODO.
	FAQ = Self::letters_to_token(b"FAQ"),
	
	/// TODO.
	FAR = Self::letters_to_token(b"FAR"),
	
	/// TODO.
	FAS = Self::letters_to_token(b"FAS"),
	
	/// TODO.
	FAT = Self::letters_to_token(b"FAT"),
	
	/// TODO.
	FAU = Self::letters_to_token(b"FAU"),
	
	/// TODO.
	FAV = Self::letters_to_token(b"FAV"),
	
	/// TODO.
	FAW = Self::letters_to_token(b"FAW"),
	
	/// TODO.
	FAX = Self::letters_to_token(b"FAX"),
	
	/// TODO.
	FAY = Self::letters_to_token(b"FAY"),
	
	/// TODO.
	FAZ = Self::letters_to_token(b"FAZ"),
	
	/// TODO.
	FBA = Self::letters_to_token(b"FBA"),
	
	/// TODO.
	FBB = Self::letters_to_token(b"FBB"),
	
	/// TODO.
	FBC = Self::letters_to_token(b"FBC"),
	
	/// TODO.
	FBD = Self::letters_to_token(b"FBD"),
	
	/// TODO.
	FBE = Self::letters_to_token(b"FBE"),
	
	/// TODO.
	FBF = Self::letters_to_token(b"FBF"),
	
	/// TODO.
	FBG = Self::letters_to_token(b"FBG"),
	
	/// TODO.
	FBH = Self::letters_to_token(b"FBH"),
	
	/// TODO.
	FBI = Self::letters_to_token(b"FBI"),
	
	/// TODO.
	FBJ = Self::letters_to_token(b"FBJ"),
	
	/// TODO.
	FBK = Self::letters_to_token(b"FBK"),
	
	/// TODO.
	FBL = Self::letters_to_token(b"FBL"),
	
	/// TODO.
	FBM = Self::letters_to_token(b"FBM"),
	
	/// TODO.
	FBN = Self::letters_to_token(b"FBN"),
	
	/// TODO.
	FBO = Self::letters_to_token(b"FBO"),
	
	/// TODO.
	FBP = Self::letters_to_token(b"FBP"),
	
	/// TODO.
	FBQ = Self::letters_to_token(b"FBQ"),
	
	/// TODO.
	FBR = Self::letters_to_token(b"FBR"),
	
	/// TODO.
	FBS = Self::letters_to_token(b"FBS"),
	
	/// TODO.
	FBT = Self::letters_to_token(b"FBT"),
	
	/// TODO.
	FBU = Self::letters_to_token(b"FBU"),
	
	/// TODO.
	FBV = Self::letters_to_token(b"FBV"),
	
	/// TODO.
	FBW = Self::letters_to_token(b"FBW"),
	
	/// TODO.
	FBX = Self::letters_to_token(b"FBX"),
	
	/// TODO.
	FBY = Self::letters_to_token(b"FBY"),
	
	/// TODO.
	FBZ = Self::letters_to_token(b"FBZ"),
	
	/// TODO.
	FCA = Self::letters_to_token(b"FCA"),
	
	/// TODO.
	FCB = Self::letters_to_token(b"FCB"),
	
	/// TODO.
	FCC = Self::letters_to_token(b"FCC"),
	
	/// TODO.
	FCD = Self::letters_to_token(b"FCD"),
	
	/// TODO.
	FCE = Self::letters_to_token(b"FCE"),
	
	/// TODO.
	FCF = Self::letters_to_token(b"FCF"),
	
	/// TODO.
	FCG = Self::letters_to_token(b"FCG"),
	
	/// TODO.
	FCH = Self::letters_to_token(b"FCH"),
	
	/// TODO.
	FCI = Self::letters_to_token(b"FCI"),
	
	/// TODO.
	FCJ = Self::letters_to_token(b"FCJ"),
	
	/// TODO.
	FCK = Self::letters_to_token(b"FCK"),
	
	/// TODO.
	FCL = Self::letters_to_token(b"FCL"),
	
	/// TODO.
	FCM = Self::letters_to_token(b"FCM"),
	
	/// TODO.
	FCN = Self::letters_to_token(b"FCN"),
	
	/// TODO.
	FCO = Self::letters_to_token(b"FCO"),
	
	/// TODO.
	FCP = Self::letters_to_token(b"FCP"),
	
	/// TODO.
	FCQ = Self::letters_to_token(b"FCQ"),
	
	/// TODO.
	FCR = Self::letters_to_token(b"FCR"),
	
	/// TODO.
	FCS = Self::letters_to_token(b"FCS"),
	
	/// TODO.
	FCT = Self::letters_to_token(b"FCT"),
	
	/// TODO.
	FCU = Self::letters_to_token(b"FCU"),
	
	/// TODO.
	FCV = Self::letters_to_token(b"FCV"),
	
	/// TODO.
	FCW = Self::letters_to_token(b"FCW"),
	
	/// TODO.
	FCX = Self::letters_to_token(b"FCX"),
	
	/// TODO.
	FCY = Self::letters_to_token(b"FCY"),
	
	/// TODO.
	FCZ = Self::letters_to_token(b"FCZ"),
	
	/// TODO.
	FDA = Self::letters_to_token(b"FDA"),
	
	/// TODO.
	FDB = Self::letters_to_token(b"FDB"),
	
	/// TODO.
	FDC = Self::letters_to_token(b"FDC"),
	
	/// TODO.
	FDD = Self::letters_to_token(b"FDD"),
	
	/// TODO.
	FDE = Self::letters_to_token(b"FDE"),
	
	/// TODO.
	FDF = Self::letters_to_token(b"FDF"),
	
	/// TODO.
	FDG = Self::letters_to_token(b"FDG"),
	
	/// TODO.
	FDH = Self::letters_to_token(b"FDH"),
	
	/// TODO.
	FDI = Self::letters_to_token(b"FDI"),
	
	/// TODO.
	FDJ = Self::letters_to_token(b"FDJ"),
	
	/// TODO.
	FDK = Self::letters_to_token(b"FDK"),
	
	/// TODO.
	FDL = Self::letters_to_token(b"FDL"),
	
	/// TODO.
	FDM = Self::letters_to_token(b"FDM"),
	
	/// TODO.
	FDN = Self::letters_to_token(b"FDN"),
	
	/// TODO.
	FDO = Self::letters_to_token(b"FDO"),
	
	/// TODO.
	FDP = Self::letters_to_token(b"FDP"),
	
	/// TODO.
	FDQ = Self::letters_to_token(b"FDQ"),
	
	/// TODO.
	FDR = Self::letters_to_token(b"FDR"),
	
	/// TODO.
	FDS = Self::letters_to_token(b"FDS"),
	
	/// TODO.
	FDT = Self::letters_to_token(b"FDT"),
	
	/// TODO.
	FDU = Self::letters_to_token(b"FDU"),
	
	/// TODO.
	FDV = Self::letters_to_token(b"FDV"),
	
	/// TODO.
	FDW = Self::letters_to_token(b"FDW"),
	
	/// TODO.
	FDX = Self::letters_to_token(b"FDX"),
	
	/// TODO.
	FDY = Self::letters_to_token(b"FDY"),
	
	/// TODO.
	FDZ = Self::letters_to_token(b"FDZ"),
	
	/// TODO.
	FEA = Self::letters_to_token(b"FEA"),
	
	/// TODO.
	FEB = Self::letters_to_token(b"FEB"),
	
	/// TODO.
	FEC = Self::letters_to_token(b"FEC"),
	
	/// TODO.
	FED = Self::letters_to_token(b"FED"),
	
	/// TODO.
	FEE = Self::letters_to_token(b"FEE"),
	
	/// TODO.
	FEF = Self::letters_to_token(b"FEF"),
	
	/// TODO.
	FEG = Self::letters_to_token(b"FEG"),
	
	/// TODO.
	FEH = Self::letters_to_token(b"FEH"),
	
	/// TODO.
	FEI = Self::letters_to_token(b"FEI"),
	
	/// TODO.
	FEJ = Self::letters_to_token(b"FEJ"),
	
	/// TODO.
	FEK = Self::letters_to_token(b"FEK"),
	
	/// TODO.
	FEL = Self::letters_to_token(b"FEL"),
	
	/// TODO.
	FEM = Self::letters_to_token(b"FEM"),
	
	/// TODO.
	FEN = Self::letters_to_token(b"FEN"),
	
	/// TODO.
	FEO = Self::letters_to_token(b"FEO"),
	
	/// TODO.
	FEP = Self::letters_to_token(b"FEP"),
	
	/// TODO.
	FEQ = Self::letters_to_token(b"FEQ"),
	
	/// TODO.
	FER = Self::letters_to_token(b"FER"),
	
	/// TODO.
	FES = Self::letters_to_token(b"FES"),
	
	/// TODO.
	FET = Self::letters_to_token(b"FET"),
	
	/// TODO.
	FEU = Self::letters_to_token(b"FEU"),
	
	/// TODO.
	FEV = Self::letters_to_token(b"FEV"),
	
	/// TODO.
	FEW = Self::letters_to_token(b"FEW"),
	
	/// TODO.
	FEX = Self::letters_to_token(b"FEX"),
	
	/// TODO.
	FEY = Self::letters_to_token(b"FEY"),
	
	/// TODO.
	FEZ = Self::letters_to_token(b"FEZ"),
	
	/// TODO.
	FFA = Self::letters_to_token(b"FFA"),
	
	/// TODO.
	FFB = Self::letters_to_token(b"FFB"),
	
	/// TODO.
	FFC = Self::letters_to_token(b"FFC"),
	
	/// TODO.
	FFD = Self::letters_to_token(b"FFD"),
	
	/// TODO.
	FFE = Self::letters_to_token(b"FFE"),
	
	/// Unoffical.
	///
	/// Africa (NATO STANAG 1059 INT).
	FFF = Self::letters_to_token(b"FFF"),
	
	/// TODO.
	FFG = Self::letters_to_token(b"FFG"),
	
	/// TODO.
	FFH = Self::letters_to_token(b"FFH"),
	
	/// TODO.
	FFI = Self::letters_to_token(b"FFI"),
	
	/// TODO.
	FFJ = Self::letters_to_token(b"FFJ"),
	
	/// TODO.
	FFK = Self::letters_to_token(b"FFK"),
	
	/// TODO.
	FFL = Self::letters_to_token(b"FFL"),
	
	/// TODO.
	FFM = Self::letters_to_token(b"FFM"),
	
	/// TODO.
	FFN = Self::letters_to_token(b"FFN"),
	
	/// TODO.
	FFO = Self::letters_to_token(b"FFO"),
	
	/// TODO.
	FFP = Self::letters_to_token(b"FFP"),
	
	/// TODO.
	FFQ = Self::letters_to_token(b"FFQ"),
	
	/// TODO.
	FFR = Self::letters_to_token(b"FFR"),
	
	/// TODO.
	FFS = Self::letters_to_token(b"FFS"),
	
	/// TODO.
	FFT = Self::letters_to_token(b"FFT"),
	
	/// TODO.
	FFU = Self::letters_to_token(b"FFU"),
	
	/// TODO.
	FFV = Self::letters_to_token(b"FFV"),
	
	/// TODO.
	FFW = Self::letters_to_token(b"FFW"),
	
	/// TODO.
	FFX = Self::letters_to_token(b"FFX"),
	
	/// TODO.
	FFY = Self::letters_to_token(b"FFY"),
	
	/// TODO.
	FFZ = Self::letters_to_token(b"FFZ"),
	
	/// TODO.
	FGA = Self::letters_to_token(b"FGA"),
	
	/// TODO.
	FGB = Self::letters_to_token(b"FGB"),
	
	/// TODO.
	FGC = Self::letters_to_token(b"FGC"),
	
	/// TODO.
	FGD = Self::letters_to_token(b"FGD"),
	
	/// TODO.
	FGE = Self::letters_to_token(b"FGE"),
	
	/// TODO.
	FGF = Self::letters_to_token(b"FGF"),
	
	/// TODO.
	FGG = Self::letters_to_token(b"FGG"),
	
	/// TODO.
	FGH = Self::letters_to_token(b"FGH"),
	
	/// TODO.
	FGI = Self::letters_to_token(b"FGI"),
	
	/// TODO.
	FGJ = Self::letters_to_token(b"FGJ"),
	
	/// TODO.
	FGK = Self::letters_to_token(b"FGK"),
	
	/// TODO.
	FGL = Self::letters_to_token(b"FGL"),
	
	/// TODO.
	FGM = Self::letters_to_token(b"FGM"),
	
	/// TODO.
	FGN = Self::letters_to_token(b"FGN"),
	
	/// TODO.
	FGO = Self::letters_to_token(b"FGO"),
	
	/// TODO.
	FGP = Self::letters_to_token(b"FGP"),
	
	/// TODO.
	FGQ = Self::letters_to_token(b"FGQ"),
	
	/// TODO.
	FGR = Self::letters_to_token(b"FGR"),
	
	/// TODO.
	FGS = Self::letters_to_token(b"FGS"),
	
	/// TODO.
	FGT = Self::letters_to_token(b"FGT"),
	
	/// TODO.
	FGU = Self::letters_to_token(b"FGU"),
	
	/// TODO.
	FGV = Self::letters_to_token(b"FGV"),
	
	/// TODO.
	FGW = Self::letters_to_token(b"FGW"),
	
	/// TODO.
	FGX = Self::letters_to_token(b"FGX"),
	
	/// TODO.
	FGY = Self::letters_to_token(b"FGY"),
	
	/// TODO.
	FGZ = Self::letters_to_token(b"FGZ"),
	
	/// TODO.
	FHA = Self::letters_to_token(b"FHA"),
	
	/// TODO.
	FHB = Self::letters_to_token(b"FHB"),
	
	/// TODO.
	FHC = Self::letters_to_token(b"FHC"),
	
	/// TODO.
	FHD = Self::letters_to_token(b"FHD"),
	
	/// TODO.
	FHE = Self::letters_to_token(b"FHE"),
	
	/// TODO.
	FHF = Self::letters_to_token(b"FHF"),
	
	/// TODO.
	FHG = Self::letters_to_token(b"FHG"),
	
	/// TODO.
	FHH = Self::letters_to_token(b"FHH"),
	
	/// TODO.
	FHI = Self::letters_to_token(b"FHI"),
	
	/// TODO.
	FHJ = Self::letters_to_token(b"FHJ"),
	
	/// TODO.
	FHK = Self::letters_to_token(b"FHK"),
	
	/// TODO.
	FHL = Self::letters_to_token(b"FHL"),
	
	/// TODO.
	FHM = Self::letters_to_token(b"FHM"),
	
	/// TODO.
	FHN = Self::letters_to_token(b"FHN"),
	
	/// TODO.
	FHO = Self::letters_to_token(b"FHO"),
	
	/// TODO.
	FHP = Self::letters_to_token(b"FHP"),
	
	/// TODO.
	FHQ = Self::letters_to_token(b"FHQ"),
	
	/// TODO.
	FHR = Self::letters_to_token(b"FHR"),
	
	/// TODO.
	FHS = Self::letters_to_token(b"FHS"),
	
	/// TODO.
	FHT = Self::letters_to_token(b"FHT"),
	
	/// TODO.
	FHU = Self::letters_to_token(b"FHU"),
	
	/// TODO.
	FHV = Self::letters_to_token(b"FHV"),
	
	/// TODO.
	FHW = Self::letters_to_token(b"FHW"),
	
	/// TODO.
	FHX = Self::letters_to_token(b"FHX"),
	
	/// TODO.
	FHY = Self::letters_to_token(b"FHY"),
	
	/// TODO.
	FHZ = Self::letters_to_token(b"FHZ"),
	
	/// TODO.
	FIA = Self::letters_to_token(b"FIA"),
	
	/// TODO.
	FIB = Self::letters_to_token(b"FIB"),
	
	/// TODO.
	FIC = Self::letters_to_token(b"FIC"),
	
	/// TODO.
	FID = Self::letters_to_token(b"FID"),
	
	/// TODO.
	FIE = Self::letters_to_token(b"FIE"),
	
	/// TODO.
	FIF = Self::letters_to_token(b"FIF"),
	
	/// TODO.
	FIG = Self::letters_to_token(b"FIG"),
	
	/// TODO.
	FIH = Self::letters_to_token(b"FIH"),
	
	/// TODO.
	FII = Self::letters_to_token(b"FII"),
	
	/// TODO.
	FIJ = Self::letters_to_token(b"FIJ"),
	
	/// TODO.
	FIK = Self::letters_to_token(b"FIK"),
	
	/// TODO.
	FIL = Self::letters_to_token(b"FIL"),
	
	/// TODO.
	FIM = Self::letters_to_token(b"FIM"),
	
	/// TODO.
	FIN = Self::letters_to_token(b"FIN"),
	
	/// TODO.
	FIO = Self::letters_to_token(b"FIO"),
	
	/// TODO.
	FIP = Self::letters_to_token(b"FIP"),
	
	/// TODO.
	FIQ = Self::letters_to_token(b"FIQ"),
	
	/// TODO.
	FIR = Self::letters_to_token(b"FIR"),
	
	/// TODO.
	FIS = Self::letters_to_token(b"FIS"),
	
	/// TODO.
	FIT = Self::letters_to_token(b"FIT"),
	
	/// TODO.
	FIU = Self::letters_to_token(b"FIU"),
	
	/// TODO.
	FIV = Self::letters_to_token(b"FIV"),
	
	/// TODO.
	FIW = Self::letters_to_token(b"FIW"),
	
	/// TODO.
	FIX = Self::letters_to_token(b"FIX"),
	
	/// TODO.
	FIY = Self::letters_to_token(b"FIY"),
	
	/// TODO.
	FIZ = Self::letters_to_token(b"FIZ"),
	
	/// TODO.
	FJA = Self::letters_to_token(b"FJA"),
	
	/// TODO.
	FJB = Self::letters_to_token(b"FJB"),
	
	/// TODO.
	FJC = Self::letters_to_token(b"FJC"),
	
	/// TODO.
	FJD = Self::letters_to_token(b"FJD"),
	
	/// TODO.
	FJE = Self::letters_to_token(b"FJE"),
	
	/// TODO.
	FJF = Self::letters_to_token(b"FJF"),
	
	/// TODO.
	FJG = Self::letters_to_token(b"FJG"),
	
	/// TODO.
	FJH = Self::letters_to_token(b"FJH"),
	
	/// TODO.
	FJI = Self::letters_to_token(b"FJI"),
	
	/// TODO.
	FJJ = Self::letters_to_token(b"FJJ"),
	
	/// TODO.
	FJK = Self::letters_to_token(b"FJK"),
	
	/// TODO.
	FJL = Self::letters_to_token(b"FJL"),
	
	/// TODO.
	FJM = Self::letters_to_token(b"FJM"),
	
	/// TODO.
	FJN = Self::letters_to_token(b"FJN"),
	
	/// TODO.
	FJO = Self::letters_to_token(b"FJO"),
	
	/// TODO.
	FJP = Self::letters_to_token(b"FJP"),
	
	/// TODO.
	FJQ = Self::letters_to_token(b"FJQ"),
	
	/// TODO.
	FJR = Self::letters_to_token(b"FJR"),
	
	/// TODO.
	FJS = Self::letters_to_token(b"FJS"),
	
	/// TODO.
	FJT = Self::letters_to_token(b"FJT"),
	
	/// TODO.
	FJU = Self::letters_to_token(b"FJU"),
	
	/// TODO.
	FJV = Self::letters_to_token(b"FJV"),
	
	/// TODO.
	FJW = Self::letters_to_token(b"FJW"),
	
	/// TODO.
	FJX = Self::letters_to_token(b"FJX"),
	
	/// TODO.
	FJY = Self::letters_to_token(b"FJY"),
	
	/// TODO.
	FJZ = Self::letters_to_token(b"FJZ"),
	
	/// TODO.
	FKA = Self::letters_to_token(b"FKA"),
	
	/// TODO.
	FKB = Self::letters_to_token(b"FKB"),
	
	/// TODO.
	FKC = Self::letters_to_token(b"FKC"),
	
	/// TODO.
	FKD = Self::letters_to_token(b"FKD"),
	
	/// TODO.
	FKE = Self::letters_to_token(b"FKE"),
	
	/// TODO.
	FKF = Self::letters_to_token(b"FKF"),
	
	/// TODO.
	FKG = Self::letters_to_token(b"FKG"),
	
	/// TODO.
	FKH = Self::letters_to_token(b"FKH"),
	
	/// TODO.
	FKI = Self::letters_to_token(b"FKI"),
	
	/// TODO.
	FKJ = Self::letters_to_token(b"FKJ"),
	
	/// TODO.
	FKK = Self::letters_to_token(b"FKK"),
	
	/// TODO.
	FKL = Self::letters_to_token(b"FKL"),
	
	/// TODO.
	FKM = Self::letters_to_token(b"FKM"),
	
	/// TODO.
	FKN = Self::letters_to_token(b"FKN"),
	
	/// TODO.
	FKO = Self::letters_to_token(b"FKO"),
	
	/// TODO.
	FKP = Self::letters_to_token(b"FKP"),
	
	/// TODO.
	FKQ = Self::letters_to_token(b"FKQ"),
	
	/// TODO.
	FKR = Self::letters_to_token(b"FKR"),
	
	/// TODO.
	FKS = Self::letters_to_token(b"FKS"),
	
	/// TODO.
	FKT = Self::letters_to_token(b"FKT"),
	
	/// TODO.
	FKU = Self::letters_to_token(b"FKU"),
	
	/// TODO.
	FKV = Self::letters_to_token(b"FKV"),
	
	/// TODO.
	FKW = Self::letters_to_token(b"FKW"),
	
	/// TODO.
	FKX = Self::letters_to_token(b"FKX"),
	
	/// TODO.
	FKY = Self::letters_to_token(b"FKY"),
	
	/// TODO.
	FKZ = Self::letters_to_token(b"FKZ"),
	
	/// TODO.
	FLA = Self::letters_to_token(b"FLA"),
	
	/// TODO.
	FLB = Self::letters_to_token(b"FLB"),
	
	/// TODO.
	FLC = Self::letters_to_token(b"FLC"),
	
	/// TODO.
	FLD = Self::letters_to_token(b"FLD"),
	
	/// TODO.
	FLE = Self::letters_to_token(b"FLE"),
	
	/// TODO.
	FLF = Self::letters_to_token(b"FLF"),
	
	/// TODO.
	FLG = Self::letters_to_token(b"FLG"),
	
	/// TODO.
	FLH = Self::letters_to_token(b"FLH"),
	
	/// TODO.
	FLI = Self::letters_to_token(b"FLI"),
	
	/// TODO.
	FLJ = Self::letters_to_token(b"FLJ"),
	
	/// TODO.
	FLK = Self::letters_to_token(b"FLK"),
	
	/// TODO.
	FLL = Self::letters_to_token(b"FLL"),
	
	/// TODO.
	FLM = Self::letters_to_token(b"FLM"),
	
	/// TODO.
	FLN = Self::letters_to_token(b"FLN"),
	
	/// TODO.
	FLO = Self::letters_to_token(b"FLO"),
	
	/// TODO.
	FLP = Self::letters_to_token(b"FLP"),
	
	/// TODO.
	FLQ = Self::letters_to_token(b"FLQ"),
	
	/// TODO.
	FLR = Self::letters_to_token(b"FLR"),
	
	/// TODO.
	FLS = Self::letters_to_token(b"FLS"),
	
	/// TODO.
	FLT = Self::letters_to_token(b"FLT"),
	
	/// TODO.
	FLU = Self::letters_to_token(b"FLU"),
	
	/// TODO.
	FLV = Self::letters_to_token(b"FLV"),
	
	/// TODO.
	FLW = Self::letters_to_token(b"FLW"),
	
	/// TODO.
	FLX = Self::letters_to_token(b"FLX"),
	
	/// TODO.
	FLY = Self::letters_to_token(b"FLY"),
	
	/// TODO.
	FLZ = Self::letters_to_token(b"FLZ"),
	
	/// TODO.
	FMA = Self::letters_to_token(b"FMA"),
	
	/// TODO.
	FMB = Self::letters_to_token(b"FMB"),
	
	/// TODO.
	FMC = Self::letters_to_token(b"FMC"),
	
	/// TODO.
	FMD = Self::letters_to_token(b"FMD"),
	
	/// TODO.
	FME = Self::letters_to_token(b"FME"),
	
	/// TODO.
	FMF = Self::letters_to_token(b"FMF"),
	
	/// TODO.
	FMG = Self::letters_to_token(b"FMG"),
	
	/// TODO.
	FMH = Self::letters_to_token(b"FMH"),
	
	/// TODO.
	FMI = Self::letters_to_token(b"FMI"),
	
	/// TODO.
	FMJ = Self::letters_to_token(b"FMJ"),
	
	/// TODO.
	FMK = Self::letters_to_token(b"FMK"),
	
	/// TODO.
	FML = Self::letters_to_token(b"FML"),
	
	/// TODO.
	FMM = Self::letters_to_token(b"FMM"),
	
	/// TODO.
	FMN = Self::letters_to_token(b"FMN"),
	
	/// TODO.
	FMO = Self::letters_to_token(b"FMO"),
	
	/// TODO.
	FMP = Self::letters_to_token(b"FMP"),
	
	/// TODO.
	FMQ = Self::letters_to_token(b"FMQ"),
	
	/// TODO.
	FMR = Self::letters_to_token(b"FMR"),
	
	/// TODO.
	FMS = Self::letters_to_token(b"FMS"),
	
	/// TODO.
	FMT = Self::letters_to_token(b"FMT"),
	
	/// TODO.
	FMU = Self::letters_to_token(b"FMU"),
	
	/// TODO.
	FMV = Self::letters_to_token(b"FMV"),
	
	/// TODO.
	FMW = Self::letters_to_token(b"FMW"),
	
	/// TODO.
	FMX = Self::letters_to_token(b"FMX"),
	
	/// TODO.
	FMY = Self::letters_to_token(b"FMY"),
	
	/// TODO.
	FMZ = Self::letters_to_token(b"FMZ"),
	
	/// TODO.
	FNA = Self::letters_to_token(b"FNA"),
	
	/// TODO.
	FNB = Self::letters_to_token(b"FNB"),
	
	/// TODO.
	FNC = Self::letters_to_token(b"FNC"),
	
	/// TODO.
	FND = Self::letters_to_token(b"FND"),
	
	/// TODO.
	FNE = Self::letters_to_token(b"FNE"),
	
	/// TODO.
	FNF = Self::letters_to_token(b"FNF"),
	
	/// TODO.
	FNG = Self::letters_to_token(b"FNG"),
	
	/// TODO.
	FNH = Self::letters_to_token(b"FNH"),
	
	/// TODO.
	FNI = Self::letters_to_token(b"FNI"),
	
	/// TODO.
	FNJ = Self::letters_to_token(b"FNJ"),
	
	/// TODO.
	FNK = Self::letters_to_token(b"FNK"),
	
	/// TODO.
	FNL = Self::letters_to_token(b"FNL"),
	
	/// TODO.
	FNM = Self::letters_to_token(b"FNM"),
	
	/// TODO.
	FNN = Self::letters_to_token(b"FNN"),
	
	/// TODO.
	FNO = Self::letters_to_token(b"FNO"),
	
	/// TODO.
	FNP = Self::letters_to_token(b"FNP"),
	
	/// TODO.
	FNQ = Self::letters_to_token(b"FNQ"),
	
	/// TODO.
	FNR = Self::letters_to_token(b"FNR"),
	
	/// TODO.
	FNS = Self::letters_to_token(b"FNS"),
	
	/// TODO.
	FNT = Self::letters_to_token(b"FNT"),
	
	/// TODO.
	FNU = Self::letters_to_token(b"FNU"),
	
	/// TODO.
	FNV = Self::letters_to_token(b"FNV"),
	
	/// TODO.
	FNW = Self::letters_to_token(b"FNW"),
	
	/// TODO.
	FNX = Self::letters_to_token(b"FNX"),
	
	/// TODO.
	FNY = Self::letters_to_token(b"FNY"),
	
	/// TODO.
	FNZ = Self::letters_to_token(b"FNZ"),
	
	/// TODO.
	FOA = Self::letters_to_token(b"FOA"),
	
	/// TODO.
	FOB = Self::letters_to_token(b"FOB"),
	
	/// TODO.
	FOC = Self::letters_to_token(b"FOC"),
	
	/// TODO.
	FOD = Self::letters_to_token(b"FOD"),
	
	/// TODO.
	FOE = Self::letters_to_token(b"FOE"),
	
	/// TODO.
	FOF = Self::letters_to_token(b"FOF"),
	
	/// TODO.
	FOG = Self::letters_to_token(b"FOG"),
	
	/// TODO.
	FOH = Self::letters_to_token(b"FOH"),
	
	/// TODO.
	FOI = Self::letters_to_token(b"FOI"),
	
	/// TODO.
	FOJ = Self::letters_to_token(b"FOJ"),
	
	/// TODO.
	FOK = Self::letters_to_token(b"FOK"),
	
	/// TODO.
	FOL = Self::letters_to_token(b"FOL"),
	
	/// TODO.
	FOM = Self::letters_to_token(b"FOM"),
	
	/// TODO.
	FON = Self::letters_to_token(b"FON"),
	
	/// TODO.
	FOO = Self::letters_to_token(b"FOO"),
	
	/// TODO.
	FOP = Self::letters_to_token(b"FOP"),
	
	/// TODO.
	FOQ = Self::letters_to_token(b"FOQ"),
	
	/// TODO.
	FOR = Self::letters_to_token(b"FOR"),
	
	/// TODO.
	FOS = Self::letters_to_token(b"FOS"),
	
	/// TODO.
	FOT = Self::letters_to_token(b"FOT"),
	
	/// TODO.
	FOU = Self::letters_to_token(b"FOU"),
	
	/// TODO.
	FOV = Self::letters_to_token(b"FOV"),
	
	/// TODO.
	FOW = Self::letters_to_token(b"FOW"),
	
	/// TODO.
	FOX = Self::letters_to_token(b"FOX"),
	
	/// TODO.
	FOY = Self::letters_to_token(b"FOY"),
	
	/// TODO.
	FOZ = Self::letters_to_token(b"FOZ"),
	
	/// TODO.
	FPA = Self::letters_to_token(b"FPA"),
	
	/// TODO.
	FPB = Self::letters_to_token(b"FPB"),
	
	/// TODO.
	FPC = Self::letters_to_token(b"FPC"),
	
	/// TODO.
	FPD = Self::letters_to_token(b"FPD"),
	
	/// TODO.
	FPE = Self::letters_to_token(b"FPE"),
	
	/// TODO.
	FPF = Self::letters_to_token(b"FPF"),
	
	/// TODO.
	FPG = Self::letters_to_token(b"FPG"),
	
	/// TODO.
	FPH = Self::letters_to_token(b"FPH"),
	
	/// TODO.
	FPI = Self::letters_to_token(b"FPI"),
	
	/// TODO.
	FPJ = Self::letters_to_token(b"FPJ"),
	
	/// TODO.
	FPK = Self::letters_to_token(b"FPK"),
	
	/// TODO.
	FPL = Self::letters_to_token(b"FPL"),
	
	/// TODO.
	FPM = Self::letters_to_token(b"FPM"),
	
	/// TODO.
	FPN = Self::letters_to_token(b"FPN"),
	
	/// TODO.
	FPO = Self::letters_to_token(b"FPO"),
	
	/// TODO.
	FPP = Self::letters_to_token(b"FPP"),
	
	/// TODO.
	FPQ = Self::letters_to_token(b"FPQ"),
	
	/// TODO.
	FPR = Self::letters_to_token(b"FPR"),
	
	/// TODO.
	FPS = Self::letters_to_token(b"FPS"),
	
	/// TODO.
	FPT = Self::letters_to_token(b"FPT"),
	
	/// TODO.
	FPU = Self::letters_to_token(b"FPU"),
	
	/// TODO.
	FPV = Self::letters_to_token(b"FPV"),
	
	/// TODO.
	FPW = Self::letters_to_token(b"FPW"),
	
	/// TODO.
	FPX = Self::letters_to_token(b"FPX"),
	
	/// TODO.
	FPY = Self::letters_to_token(b"FPY"),
	
	/// TODO.
	FPZ = Self::letters_to_token(b"FPZ"),
	
	/// TODO.
	FQA = Self::letters_to_token(b"FQA"),
	
	/// TODO.
	FQB = Self::letters_to_token(b"FQB"),
	
	/// TODO.
	FQC = Self::letters_to_token(b"FQC"),
	
	/// TODO.
	FQD = Self::letters_to_token(b"FQD"),
	
	/// TODO.
	FQE = Self::letters_to_token(b"FQE"),
	
	/// TODO.
	FQF = Self::letters_to_token(b"FQF"),
	
	/// TODO.
	FQG = Self::letters_to_token(b"FQG"),
	
	/// TODO.
	FQH = Self::letters_to_token(b"FQH"),
	
	/// TODO.
	FQI = Self::letters_to_token(b"FQI"),
	
	/// TODO.
	FQJ = Self::letters_to_token(b"FQJ"),
	
	/// TODO.
	FQK = Self::letters_to_token(b"FQK"),
	
	/// TODO.
	FQL = Self::letters_to_token(b"FQL"),
	
	/// TODO.
	FQM = Self::letters_to_token(b"FQM"),
	
	/// TODO.
	FQN = Self::letters_to_token(b"FQN"),
	
	/// TODO.
	FQO = Self::letters_to_token(b"FQO"),
	
	/// TODO.
	FQP = Self::letters_to_token(b"FQP"),
	
	/// TODO.
	FQQ = Self::letters_to_token(b"FQQ"),
	
	/// TODO.
	FQR = Self::letters_to_token(b"FQR"),
	
	/// TODO.
	FQS = Self::letters_to_token(b"FQS"),
	
	/// TODO.
	FQT = Self::letters_to_token(b"FQT"),
	
	/// TODO.
	FQU = Self::letters_to_token(b"FQU"),
	
	/// TODO.
	FQV = Self::letters_to_token(b"FQV"),
	
	/// TODO.
	FQW = Self::letters_to_token(b"FQW"),
	
	/// TODO.
	FQX = Self::letters_to_token(b"FQX"),
	
	/// TODO.
	FQY = Self::letters_to_token(b"FQY"),
	
	/// TODO.
	FQZ = Self::letters_to_token(b"FQZ"),
	
	/// France.
	FRA = Self::letters_to_token(b"FRA"),
	
	/// TODO.
	FRB = Self::letters_to_token(b"FRB"),
	
	/// TODO.
	FRC = Self::letters_to_token(b"FRC"),
	
	/// TODO.
	FRD = Self::letters_to_token(b"FRD"),
	
	/// TODO.
	FRE = Self::letters_to_token(b"FRE"),
	
	/// TODO.
	FRF = Self::letters_to_token(b"FRF"),
	
	/// TODO.
	FRG = Self::letters_to_token(b"FRG"),
	
	/// TODO.
	FRH = Self::letters_to_token(b"FRH"),
	
	/// TODO.
	FRI = Self::letters_to_token(b"FRI"),
	
	/// TODO.
	FRJ = Self::letters_to_token(b"FRJ"),
	
	/// TODO.
	FRK = Self::letters_to_token(b"FRK"),
	
	/// TODO.
	FRL = Self::letters_to_token(b"FRL"),
	
	/// TODO.
	FRM = Self::letters_to_token(b"FRM"),
	
	/// TODO.
	FRN = Self::letters_to_token(b"FRN"),
	
	/// TODO.
	FRO = Self::letters_to_token(b"FRO"),
	
	/// TODO.
	FRP = Self::letters_to_token(b"FRP"),
	
	/// TODO.
	FRQ = Self::letters_to_token(b"FRQ"),
	
	/// TODO.
	FRR = Self::letters_to_token(b"FRR"),
	
	/// TODO.
	FRS = Self::letters_to_token(b"FRS"),
	
	/// TODO.
	FRT = Self::letters_to_token(b"FRT"),
	
	/// TODO.
	FRU = Self::letters_to_token(b"FRU"),
	
	/// TODO.
	FRV = Self::letters_to_token(b"FRV"),
	
	/// TODO.
	FRW = Self::letters_to_token(b"FRW"),
	
	/// TODO.
	FRX = Self::letters_to_token(b"FRX"),
	
	/// TODO.
	FRY = Self::letters_to_token(b"FRY"),
	
	/// TODO.
	FRZ = Self::letters_to_token(b"FRZ"),
	
	/// TODO.
	FSA = Self::letters_to_token(b"FSA"),
	
	/// TODO.
	FSB = Self::letters_to_token(b"FSB"),
	
	/// TODO.
	FSC = Self::letters_to_token(b"FSC"),
	
	/// TODO.
	FSD = Self::letters_to_token(b"FSD"),
	
	/// TODO.
	FSE = Self::letters_to_token(b"FSE"),
	
	/// TODO.
	FSF = Self::letters_to_token(b"FSF"),
	
	/// TODO.
	FSG = Self::letters_to_token(b"FSG"),
	
	/// TODO.
	FSH = Self::letters_to_token(b"FSH"),
	
	/// TODO.
	FSI = Self::letters_to_token(b"FSI"),
	
	/// TODO.
	FSJ = Self::letters_to_token(b"FSJ"),
	
	/// TODO.
	FSK = Self::letters_to_token(b"FSK"),
	
	/// TODO.
	FSL = Self::letters_to_token(b"FSL"),
	
	/// TODO.
	FSM = Self::letters_to_token(b"FSM"),
	
	/// TODO.
	FSN = Self::letters_to_token(b"FSN"),
	
	/// TODO.
	FSO = Self::letters_to_token(b"FSO"),
	
	/// TODO.
	FSP = Self::letters_to_token(b"FSP"),
	
	/// TODO.
	FSQ = Self::letters_to_token(b"FSQ"),
	
	/// TODO.
	FSR = Self::letters_to_token(b"FSR"),
	
	/// TODO.
	FSS = Self::letters_to_token(b"FSS"),
	
	/// TODO.
	FST = Self::letters_to_token(b"FST"),
	
	/// TODO.
	FSU = Self::letters_to_token(b"FSU"),
	
	/// TODO.
	FSV = Self::letters_to_token(b"FSV"),
	
	/// TODO.
	FSW = Self::letters_to_token(b"FSW"),
	
	/// TODO.
	FSX = Self::letters_to_token(b"FSX"),
	
	/// TODO.
	FSY = Self::letters_to_token(b"FSY"),
	
	/// TODO.
	FSZ = Self::letters_to_token(b"FSZ"),
	
	/// TODO.
	FTA = Self::letters_to_token(b"FTA"),
	
	/// TODO.
	FTB = Self::letters_to_token(b"FTB"),
	
	/// TODO.
	FTC = Self::letters_to_token(b"FTC"),
	
	/// TODO.
	FTD = Self::letters_to_token(b"FTD"),
	
	/// TODO.
	FTE = Self::letters_to_token(b"FTE"),
	
	/// TODO.
	FTF = Self::letters_to_token(b"FTF"),
	
	/// TODO.
	FTG = Self::letters_to_token(b"FTG"),
	
	/// TODO.
	FTH = Self::letters_to_token(b"FTH"),
	
	/// TODO.
	FTI = Self::letters_to_token(b"FTI"),
	
	/// TODO.
	FTJ = Self::letters_to_token(b"FTJ"),
	
	/// TODO.
	FTK = Self::letters_to_token(b"FTK"),
	
	/// TODO.
	FTL = Self::letters_to_token(b"FTL"),
	
	/// TODO.
	FTM = Self::letters_to_token(b"FTM"),
	
	/// TODO.
	FTN = Self::letters_to_token(b"FTN"),
	
	/// TODO.
	FTO = Self::letters_to_token(b"FTO"),
	
	/// TODO.
	FTP = Self::letters_to_token(b"FTP"),
	
	/// TODO.
	FTQ = Self::letters_to_token(b"FTQ"),
	
	/// TODO.
	FTR = Self::letters_to_token(b"FTR"),
	
	/// TODO.
	FTS = Self::letters_to_token(b"FTS"),
	
	/// TODO.
	FTT = Self::letters_to_token(b"FTT"),
	
	/// TODO.
	FTU = Self::letters_to_token(b"FTU"),
	
	/// TODO.
	FTV = Self::letters_to_token(b"FTV"),
	
	/// TODO.
	FTW = Self::letters_to_token(b"FTW"),
	
	/// TODO.
	FTX = Self::letters_to_token(b"FTX"),
	
	/// TODO.
	FTY = Self::letters_to_token(b"FTY"),
	
	/// TODO.
	FTZ = Self::letters_to_token(b"FTZ"),
	
	/// TODO.
	FUA = Self::letters_to_token(b"FUA"),
	
	/// TODO.
	FUB = Self::letters_to_token(b"FUB"),
	
	/// TODO.
	FUC = Self::letters_to_token(b"FUC"),
	
	/// TODO.
	FUD = Self::letters_to_token(b"FUD"),
	
	/// TODO.
	FUE = Self::letters_to_token(b"FUE"),
	
	/// TODO.
	FUF = Self::letters_to_token(b"FUF"),
	
	/// TODO.
	FUG = Self::letters_to_token(b"FUG"),
	
	/// TODO.
	FUH = Self::letters_to_token(b"FUH"),
	
	/// TODO.
	FUI = Self::letters_to_token(b"FUI"),
	
	/// TODO.
	FUJ = Self::letters_to_token(b"FUJ"),
	
	/// TODO.
	FUK = Self::letters_to_token(b"FUK"),
	
	/// TODO.
	FUL = Self::letters_to_token(b"FUL"),
	
	/// TODO.
	FUM = Self::letters_to_token(b"FUM"),
	
	/// TODO.
	FUN = Self::letters_to_token(b"FUN"),
	
	/// TODO.
	FUO = Self::letters_to_token(b"FUO"),
	
	/// TODO.
	FUP = Self::letters_to_token(b"FUP"),
	
	/// TODO.
	FUQ = Self::letters_to_token(b"FUQ"),
	
	/// TODO.
	FUR = Self::letters_to_token(b"FUR"),
	
	/// TODO.
	FUS = Self::letters_to_token(b"FUS"),
	
	/// TODO.
	FUT = Self::letters_to_token(b"FUT"),
	
	/// TODO.
	FUU = Self::letters_to_token(b"FUU"),
	
	/// TODO.
	FUV = Self::letters_to_token(b"FUV"),
	
	/// TODO.
	FUW = Self::letters_to_token(b"FUW"),
	
	/// TODO.
	FUX = Self::letters_to_token(b"FUX"),
	
	/// TODO.
	FUY = Self::letters_to_token(b"FUY"),
	
	/// TODO.
	FUZ = Self::letters_to_token(b"FUZ"),
	
	/// TODO.
	FVA = Self::letters_to_token(b"FVA"),
	
	/// TODO.
	FVB = Self::letters_to_token(b"FVB"),
	
	/// TODO.
	FVC = Self::letters_to_token(b"FVC"),
	
	/// TODO.
	FVD = Self::letters_to_token(b"FVD"),
	
	/// TODO.
	FVE = Self::letters_to_token(b"FVE"),
	
	/// TODO.
	FVF = Self::letters_to_token(b"FVF"),
	
	/// TODO.
	FVG = Self::letters_to_token(b"FVG"),
	
	/// TODO.
	FVH = Self::letters_to_token(b"FVH"),
	
	/// TODO.
	FVI = Self::letters_to_token(b"FVI"),
	
	/// TODO.
	FVJ = Self::letters_to_token(b"FVJ"),
	
	/// TODO.
	FVK = Self::letters_to_token(b"FVK"),
	
	/// TODO.
	FVL = Self::letters_to_token(b"FVL"),
	
	/// TODO.
	FVM = Self::letters_to_token(b"FVM"),
	
	/// TODO.
	FVN = Self::letters_to_token(b"FVN"),
	
	/// TODO.
	FVO = Self::letters_to_token(b"FVO"),
	
	/// TODO.
	FVP = Self::letters_to_token(b"FVP"),
	
	/// TODO.
	FVQ = Self::letters_to_token(b"FVQ"),
	
	/// TODO.
	FVR = Self::letters_to_token(b"FVR"),
	
	/// TODO.
	FVS = Self::letters_to_token(b"FVS"),
	
	/// TODO.
	FVT = Self::letters_to_token(b"FVT"),
	
	/// TODO.
	FVU = Self::letters_to_token(b"FVU"),
	
	/// TODO.
	FVV = Self::letters_to_token(b"FVV"),
	
	/// TODO.
	FVW = Self::letters_to_token(b"FVW"),
	
	/// TODO.
	FVX = Self::letters_to_token(b"FVX"),
	
	/// TODO.
	FVY = Self::letters_to_token(b"FVY"),
	
	/// TODO.
	FVZ = Self::letters_to_token(b"FVZ"),
	
	/// TODO.
	FWA = Self::letters_to_token(b"FWA"),
	
	/// TODO.
	FWB = Self::letters_to_token(b"FWB"),
	
	/// TODO.
	FWC = Self::letters_to_token(b"FWC"),
	
	/// TODO.
	FWD = Self::letters_to_token(b"FWD"),
	
	/// TODO.
	FWE = Self::letters_to_token(b"FWE"),
	
	/// TODO.
	FWF = Self::letters_to_token(b"FWF"),
	
	/// TODO.
	FWG = Self::letters_to_token(b"FWG"),
	
	/// TODO.
	FWH = Self::letters_to_token(b"FWH"),
	
	/// TODO.
	FWI = Self::letters_to_token(b"FWI"),
	
	/// TODO.
	FWJ = Self::letters_to_token(b"FWJ"),
	
	/// TODO.
	FWK = Self::letters_to_token(b"FWK"),
	
	/// TODO.
	FWL = Self::letters_to_token(b"FWL"),
	
	/// TODO.
	FWM = Self::letters_to_token(b"FWM"),
	
	/// TODO.
	FWN = Self::letters_to_token(b"FWN"),
	
	/// TODO.
	FWO = Self::letters_to_token(b"FWO"),
	
	/// TODO.
	FWP = Self::letters_to_token(b"FWP"),
	
	/// TODO.
	FWQ = Self::letters_to_token(b"FWQ"),
	
	/// TODO.
	FWR = Self::letters_to_token(b"FWR"),
	
	/// TODO.
	FWS = Self::letters_to_token(b"FWS"),
	
	/// TODO.
	FWT = Self::letters_to_token(b"FWT"),
	
	/// TODO.
	FWU = Self::letters_to_token(b"FWU"),
	
	/// TODO.
	FWV = Self::letters_to_token(b"FWV"),
	
	/// TODO.
	FWW = Self::letters_to_token(b"FWW"),
	
	/// TODO.
	FWX = Self::letters_to_token(b"FWX"),
	
	/// TODO.
	FWY = Self::letters_to_token(b"FWY"),
	
	/// TODO.
	FWZ = Self::letters_to_token(b"FWZ"),
	
	/// TODO.
	FXA = Self::letters_to_token(b"FXA"),
	
	/// TODO.
	FXB = Self::letters_to_token(b"FXB"),
	
	/// TODO.
	FXC = Self::letters_to_token(b"FXC"),
	
	/// TODO.
	FXD = Self::letters_to_token(b"FXD"),
	
	/// TODO.
	FXE = Self::letters_to_token(b"FXE"),
	
	/// TODO.
	FXF = Self::letters_to_token(b"FXF"),
	
	/// TODO.
	FXG = Self::letters_to_token(b"FXG"),
	
	/// TODO.
	FXH = Self::letters_to_token(b"FXH"),
	
	/// TODO.
	FXI = Self::letters_to_token(b"FXI"),
	
	/// TODO.
	FXJ = Self::letters_to_token(b"FXJ"),
	
	/// TODO.
	FXK = Self::letters_to_token(b"FXK"),
	
	/// TODO.
	FXL = Self::letters_to_token(b"FXL"),
	
	/// TODO.
	FXM = Self::letters_to_token(b"FXM"),
	
	/// TODO.
	FXN = Self::letters_to_token(b"FXN"),
	
	/// TODO.
	FXO = Self::letters_to_token(b"FXO"),
	
	/// TODO.
	FXP = Self::letters_to_token(b"FXP"),
	
	/// TODO.
	FXQ = Self::letters_to_token(b"FXQ"),
	
	/// TODO.
	FXR = Self::letters_to_token(b"FXR"),
	
	/// TODO.
	FXS = Self::letters_to_token(b"FXS"),
	
	/// TODO.
	FXT = Self::letters_to_token(b"FXT"),
	
	/// TODO.
	FXU = Self::letters_to_token(b"FXU"),
	
	/// TODO.
	FXV = Self::letters_to_token(b"FXV"),
	
	/// TODO.
	FXW = Self::letters_to_token(b"FXW"),
	
	/// Exceptional reservation.
	///
	/// France, Metropolitan.
	FXX = Self::letters_to_token(b"FXX"),
	
	/// TODO.
	FXY = Self::letters_to_token(b"FXY"),
	
	/// TODO.
	FXZ = Self::letters_to_token(b"FXZ"),
	
	/// TODO.
	FYA = Self::letters_to_token(b"FYA"),
	
	/// TODO.
	FYB = Self::letters_to_token(b"FYB"),
	
	/// TODO.
	FYC = Self::letters_to_token(b"FYC"),
	
	/// TODO.
	FYD = Self::letters_to_token(b"FYD"),
	
	/// TODO.
	FYE = Self::letters_to_token(b"FYE"),
	
	/// TODO.
	FYF = Self::letters_to_token(b"FYF"),
	
	/// TODO.
	FYG = Self::letters_to_token(b"FYG"),
	
	/// TODO.
	FYH = Self::letters_to_token(b"FYH"),
	
	/// TODO.
	FYI = Self::letters_to_token(b"FYI"),
	
	/// TODO.
	FYJ = Self::letters_to_token(b"FYJ"),
	
	/// TODO.
	FYK = Self::letters_to_token(b"FYK"),
	
	/// TODO.
	FYL = Self::letters_to_token(b"FYL"),
	
	/// TODO.
	FYM = Self::letters_to_token(b"FYM"),
	
	/// TODO.
	FYN = Self::letters_to_token(b"FYN"),
	
	/// TODO.
	FYO = Self::letters_to_token(b"FYO"),
	
	/// TODO.
	FYP = Self::letters_to_token(b"FYP"),
	
	/// TODO.
	FYQ = Self::letters_to_token(b"FYQ"),
	
	/// TODO.
	FYR = Self::letters_to_token(b"FYR"),
	
	/// TODO.
	FYS = Self::letters_to_token(b"FYS"),
	
	/// TODO.
	FYT = Self::letters_to_token(b"FYT"),
	
	/// TODO.
	FYU = Self::letters_to_token(b"FYU"),
	
	/// TODO.
	FYV = Self::letters_to_token(b"FYV"),
	
	/// TODO.
	FYW = Self::letters_to_token(b"FYW"),
	
	/// TODO.
	FYX = Self::letters_to_token(b"FYX"),
	
	/// TODO.
	FYY = Self::letters_to_token(b"FYY"),
	
	/// TODO.
	FYZ = Self::letters_to_token(b"FYZ"),
	
	/// TODO.
	FZA = Self::letters_to_token(b"FZA"),
	
	/// TODO.
	FZB = Self::letters_to_token(b"FZB"),
	
	/// TODO.
	FZC = Self::letters_to_token(b"FZC"),
	
	/// TODO.
	FZD = Self::letters_to_token(b"FZD"),
	
	/// TODO.
	FZE = Self::letters_to_token(b"FZE"),
	
	/// TODO.
	FZF = Self::letters_to_token(b"FZF"),
	
	/// TODO.
	FZG = Self::letters_to_token(b"FZG"),
	
	/// TODO.
	FZH = Self::letters_to_token(b"FZH"),
	
	/// TODO.
	FZI = Self::letters_to_token(b"FZI"),
	
	/// TODO.
	FZJ = Self::letters_to_token(b"FZJ"),
	
	/// TODO.
	FZK = Self::letters_to_token(b"FZK"),
	
	/// TODO.
	FZL = Self::letters_to_token(b"FZL"),
	
	/// TODO.
	FZM = Self::letters_to_token(b"FZM"),
	
	/// TODO.
	FZN = Self::letters_to_token(b"FZN"),
	
	/// TODO.
	FZO = Self::letters_to_token(b"FZO"),
	
	/// TODO.
	FZP = Self::letters_to_token(b"FZP"),
	
	/// TODO.
	FZQ = Self::letters_to_token(b"FZQ"),
	
	/// TODO.
	FZR = Self::letters_to_token(b"FZR"),
	
	/// TODO.
	FZS = Self::letters_to_token(b"FZS"),
	
	/// TODO.
	FZT = Self::letters_to_token(b"FZT"),
	
	/// TODO.
	FZU = Self::letters_to_token(b"FZU"),
	
	/// TODO.
	FZV = Self::letters_to_token(b"FZV"),
	
	/// TODO.
	FZW = Self::letters_to_token(b"FZW"),
	
	/// TODO.
	FZX = Self::letters_to_token(b"FZX"),
	
	/// TODO.
	FZY = Self::letters_to_token(b"FZY"),
	
	/// TODO.
	FZZ = Self::letters_to_token(b"FZZ"),
	
	/// TODO.
	GAA = Self::letters_to_token(b"GAA"),
	
	/// TODO.
	GAB = Self::letters_to_token(b"GAB"),
	
	/// TODO.
	GAC = Self::letters_to_token(b"GAC"),
	
	/// TODO.
	GAD = Self::letters_to_token(b"GAD"),
	
	/// TODO.
	GAE = Self::letters_to_token(b"GAE"),
	
	/// TODO.
	GAF = Self::letters_to_token(b"GAF"),
	
	/// TODO.
	GAG = Self::letters_to_token(b"GAG"),
	
	/// TODO.
	GAH = Self::letters_to_token(b"GAH"),
	
	/// TODO.
	GAI = Self::letters_to_token(b"GAI"),
	
	/// TODO.
	GAJ = Self::letters_to_token(b"GAJ"),
	
	/// TODO.
	GAK = Self::letters_to_token(b"GAK"),
	
	/// TODO.
	GAL = Self::letters_to_token(b"GAL"),
	
	/// TODO.
	GAM = Self::letters_to_token(b"GAM"),
	
	/// TODO.
	GAN = Self::letters_to_token(b"GAN"),
	
	/// TODO.
	GAO = Self::letters_to_token(b"GAO"),
	
	/// TODO.
	GAP = Self::letters_to_token(b"GAP"),
	
	/// TODO.
	GAQ = Self::letters_to_token(b"GAQ"),
	
	/// TODO.
	GAR = Self::letters_to_token(b"GAR"),
	
	/// TODO.
	GAS = Self::letters_to_token(b"GAS"),
	
	/// TODO.
	GAT = Self::letters_to_token(b"GAT"),
	
	/// TODO.
	GAU = Self::letters_to_token(b"GAU"),
	
	/// TODO.
	GAV = Self::letters_to_token(b"GAV"),
	
	/// TODO.
	GAW = Self::letters_to_token(b"GAW"),
	
	/// TODO.
	GAX = Self::letters_to_token(b"GAX"),
	
	/// TODO.
	GAY = Self::letters_to_token(b"GAY"),
	
	/// TODO.
	GAZ = Self::letters_to_token(b"GAZ"),
	
	/// TODO.
	GBA = Self::letters_to_token(b"GBA"),
	
	/// TODO.
	GBB = Self::letters_to_token(b"GBB"),
	
	/// TODO.
	GBC = Self::letters_to_token(b"GBC"),
	
	/// TODO.
	GBD = Self::letters_to_token(b"GBD"),
	
	/// TODO.
	GBE = Self::letters_to_token(b"GBE"),
	
	/// TODO.
	GBF = Self::letters_to_token(b"GBF"),
	
	/// TODO.
	GBG = Self::letters_to_token(b"GBG"),
	
	/// TODO.
	GBH = Self::letters_to_token(b"GBH"),
	
	/// TODO.
	GBI = Self::letters_to_token(b"GBI"),
	
	/// TODO.
	GBJ = Self::letters_to_token(b"GBJ"),
	
	/// TODO.
	GBK = Self::letters_to_token(b"GBK"),
	
	/// TODO.
	GBL = Self::letters_to_token(b"GBL"),
	
	/// TODO.
	GBM = Self::letters_to_token(b"GBM"),
	
	/// TODO.
	GBN = Self::letters_to_token(b"GBN"),
	
	/// TODO.
	GBO = Self::letters_to_token(b"GBO"),
	
	/// TODO.
	GBP = Self::letters_to_token(b"GBP"),
	
	/// TODO.
	GBQ = Self::letters_to_token(b"GBQ"),
	
	/// United Kingdom of Great Britain and Northern Ireland (the).
	GBR = Self::letters_to_token(b"GBR"),
	
	/// TODO.
	GBS = Self::letters_to_token(b"GBS"),
	
	/// TODO.
	GBT = Self::letters_to_token(b"GBT"),
	
	/// TODO.
	GBU = Self::letters_to_token(b"GBU"),
	
	/// TODO.
	GBV = Self::letters_to_token(b"GBV"),
	
	/// TODO.
	GBW = Self::letters_to_token(b"GBW"),
	
	/// TODO.
	GBX = Self::letters_to_token(b"GBX"),
	
	/// TODO.
	GBY = Self::letters_to_token(b"GBY"),
	
	/// TODO.
	GBZ = Self::letters_to_token(b"GBZ"),
	
	/// TODO.
	GCA = Self::letters_to_token(b"GCA"),
	
	/// TODO.
	GCB = Self::letters_to_token(b"GCB"),
	
	/// TODO.
	GCC = Self::letters_to_token(b"GCC"),
	
	/// TODO.
	GCD = Self::letters_to_token(b"GCD"),
	
	/// TODO.
	GCE = Self::letters_to_token(b"GCE"),
	
	/// TODO.
	GCF = Self::letters_to_token(b"GCF"),
	
	/// TODO.
	GCG = Self::letters_to_token(b"GCG"),
	
	/// TODO.
	GCH = Self::letters_to_token(b"GCH"),
	
	/// TODO.
	GCI = Self::letters_to_token(b"GCI"),
	
	/// TODO.
	GCJ = Self::letters_to_token(b"GCJ"),
	
	/// TODO.
	GCK = Self::letters_to_token(b"GCK"),
	
	/// TODO.
	GCL = Self::letters_to_token(b"GCL"),
	
	/// TODO.
	GCM = Self::letters_to_token(b"GCM"),
	
	/// TODO.
	GCN = Self::letters_to_token(b"GCN"),
	
	/// TODO.
	GCO = Self::letters_to_token(b"GCO"),
	
	/// TODO.
	GCP = Self::letters_to_token(b"GCP"),
	
	/// TODO.
	GCQ = Self::letters_to_token(b"GCQ"),
	
	/// TODO.
	GCR = Self::letters_to_token(b"GCR"),
	
	/// TODO.
	GCS = Self::letters_to_token(b"GCS"),
	
	/// TODO.
	GCT = Self::letters_to_token(b"GCT"),
	
	/// TODO.
	GCU = Self::letters_to_token(b"GCU"),
	
	/// TODO.
	GCV = Self::letters_to_token(b"GCV"),
	
	/// TODO.
	GCW = Self::letters_to_token(b"GCW"),
	
	/// TODO.
	GCX = Self::letters_to_token(b"GCX"),
	
	/// TODO.
	GCY = Self::letters_to_token(b"GCY"),
	
	/// TODO.
	GCZ = Self::letters_to_token(b"GCZ"),
	
	/// TODO.
	GDA = Self::letters_to_token(b"GDA"),
	
	/// TODO.
	GDB = Self::letters_to_token(b"GDB"),
	
	/// TODO.
	GDC = Self::letters_to_token(b"GDC"),
	
	/// TODO.
	GDD = Self::letters_to_token(b"GDD"),
	
	/// TODO.
	GDE = Self::letters_to_token(b"GDE"),
	
	/// TODO.
	GDF = Self::letters_to_token(b"GDF"),
	
	/// TODO.
	GDG = Self::letters_to_token(b"GDG"),
	
	/// TODO.
	GDH = Self::letters_to_token(b"GDH"),
	
	/// TODO.
	GDI = Self::letters_to_token(b"GDI"),
	
	/// TODO.
	GDJ = Self::letters_to_token(b"GDJ"),
	
	/// TODO.
	GDK = Self::letters_to_token(b"GDK"),
	
	/// TODO.
	GDL = Self::letters_to_token(b"GDL"),
	
	/// TODO.
	GDM = Self::letters_to_token(b"GDM"),
	
	/// TODO.
	GDN = Self::letters_to_token(b"GDN"),
	
	/// TODO.
	GDO = Self::letters_to_token(b"GDO"),
	
	/// TODO.
	GDP = Self::letters_to_token(b"GDP"),
	
	/// TODO.
	GDQ = Self::letters_to_token(b"GDQ"),
	
	/// TODO.
	GDR = Self::letters_to_token(b"GDR"),
	
	/// TODO.
	GDS = Self::letters_to_token(b"GDS"),
	
	/// TODO.
	GDT = Self::letters_to_token(b"GDT"),
	
	/// TODO.
	GDU = Self::letters_to_token(b"GDU"),
	
	/// TODO.
	GDV = Self::letters_to_token(b"GDV"),
	
	/// TODO.
	GDW = Self::letters_to_token(b"GDW"),
	
	/// TODO.
	GDX = Self::letters_to_token(b"GDX"),
	
	/// TODO.
	GDY = Self::letters_to_token(b"GDY"),
	
	/// TODO.
	GDZ = Self::letters_to_token(b"GDZ"),
	
	/// TODO.
	GEA = Self::letters_to_token(b"GEA"),
	
	/// TODO.
	GEB = Self::letters_to_token(b"GEB"),
	
	/// TODO.
	GEC = Self::letters_to_token(b"GEC"),
	
	/// TODO.
	GED = Self::letters_to_token(b"GED"),
	
	/// TODO.
	GEE = Self::letters_to_token(b"GEE"),
	
	/// TODO.
	GEF = Self::letters_to_token(b"GEF"),
	
	/// TODO.
	GEG = Self::letters_to_token(b"GEG"),
	
	/// TODO.
	GEH = Self::letters_to_token(b"GEH"),
	
	/// TODO.
	GEI = Self::letters_to_token(b"GEI"),
	
	/// TODO.
	GEJ = Self::letters_to_token(b"GEJ"),
	
	/// TODO.
	GEK = Self::letters_to_token(b"GEK"),
	
	/// TODO.
	GEL = Self::letters_to_token(b"GEL"),
	
	/// TODO.
	GEM = Self::letters_to_token(b"GEM"),
	
	/// TODO.
	GEN = Self::letters_to_token(b"GEN"),
	
	/// TODO.
	GEO = Self::letters_to_token(b"GEO"),
	
	/// TODO.
	GEP = Self::letters_to_token(b"GEP"),
	
	/// TODO.
	GEQ = Self::letters_to_token(b"GEQ"),
	
	/// TODO.
	GER = Self::letters_to_token(b"GER"),
	
	/// TODO.
	GES = Self::letters_to_token(b"GES"),
	
	/// TODO.
	GET = Self::letters_to_token(b"GET"),
	
	/// TODO.
	GEU = Self::letters_to_token(b"GEU"),
	
	/// TODO.
	GEV = Self::letters_to_token(b"GEV"),
	
	/// TODO.
	GEW = Self::letters_to_token(b"GEW"),
	
	/// TODO.
	GEX = Self::letters_to_token(b"GEX"),
	
	/// TODO.
	GEY = Self::letters_to_token(b"GEY"),
	
	/// TODO.
	GEZ = Self::letters_to_token(b"GEZ"),
	
	/// TODO.
	GFA = Self::letters_to_token(b"GFA"),
	
	/// TODO.
	GFB = Self::letters_to_token(b"GFB"),
	
	/// TODO.
	GFC = Self::letters_to_token(b"GFC"),
	
	/// TODO.
	GFD = Self::letters_to_token(b"GFD"),
	
	/// TODO.
	GFE = Self::letters_to_token(b"GFE"),
	
	/// TODO.
	GFF = Self::letters_to_token(b"GFF"),
	
	/// TODO.
	GFG = Self::letters_to_token(b"GFG"),
	
	/// TODO.
	GFH = Self::letters_to_token(b"GFH"),
	
	/// TODO.
	GFI = Self::letters_to_token(b"GFI"),
	
	/// TODO.
	GFJ = Self::letters_to_token(b"GFJ"),
	
	/// TODO.
	GFK = Self::letters_to_token(b"GFK"),
	
	/// TODO.
	GFL = Self::letters_to_token(b"GFL"),
	
	/// TODO.
	GFM = Self::letters_to_token(b"GFM"),
	
	/// TODO.
	GFN = Self::letters_to_token(b"GFN"),
	
	/// TODO.
	GFO = Self::letters_to_token(b"GFO"),
	
	/// TODO.
	GFP = Self::letters_to_token(b"GFP"),
	
	/// TODO.
	GFQ = Self::letters_to_token(b"GFQ"),
	
	/// TODO.
	GFR = Self::letters_to_token(b"GFR"),
	
	/// TODO.
	GFS = Self::letters_to_token(b"GFS"),
	
	/// TODO.
	GFT = Self::letters_to_token(b"GFT"),
	
	/// TODO.
	GFU = Self::letters_to_token(b"GFU"),
	
	/// TODO.
	GFV = Self::letters_to_token(b"GFV"),
	
	/// TODO.
	GFW = Self::letters_to_token(b"GFW"),
	
	/// TODO.
	GFX = Self::letters_to_token(b"GFX"),
	
	/// TODO.
	GFY = Self::letters_to_token(b"GFY"),
	
	/// TODO.
	GFZ = Self::letters_to_token(b"GFZ"),
	
	/// TODO.
	GGA = Self::letters_to_token(b"GGA"),
	
	/// TODO.
	GGB = Self::letters_to_token(b"GGB"),
	
	/// TODO.
	GGC = Self::letters_to_token(b"GGC"),
	
	/// TODO.
	GGD = Self::letters_to_token(b"GGD"),
	
	/// TODO.
	GGE = Self::letters_to_token(b"GGE"),
	
	/// TODO.
	GGF = Self::letters_to_token(b"GGF"),
	
	/// TODO.
	GGG = Self::letters_to_token(b"GGG"),
	
	/// TODO.
	GGH = Self::letters_to_token(b"GGH"),
	
	/// TODO.
	GGI = Self::letters_to_token(b"GGI"),
	
	/// TODO.
	GGJ = Self::letters_to_token(b"GGJ"),
	
	/// TODO.
	GGK = Self::letters_to_token(b"GGK"),
	
	/// TODO.
	GGL = Self::letters_to_token(b"GGL"),
	
	/// TODO.
	GGM = Self::letters_to_token(b"GGM"),
	
	/// TODO.
	GGN = Self::letters_to_token(b"GGN"),
	
	/// TODO.
	GGO = Self::letters_to_token(b"GGO"),
	
	/// TODO.
	GGP = Self::letters_to_token(b"GGP"),
	
	/// TODO.
	GGQ = Self::letters_to_token(b"GGQ"),
	
	/// TODO.
	GGR = Self::letters_to_token(b"GGR"),
	
	/// TODO.
	GGS = Self::letters_to_token(b"GGS"),
	
	/// TODO.
	GGT = Self::letters_to_token(b"GGT"),
	
	/// TODO.
	GGU = Self::letters_to_token(b"GGU"),
	
	/// TODO.
	GGV = Self::letters_to_token(b"GGV"),
	
	/// TODO.
	GGW = Self::letters_to_token(b"GGW"),
	
	/// TODO.
	GGX = Self::letters_to_token(b"GGX"),
	
	/// Guernsey.
	GGY = Self::letters_to_token(b"GGY"),
	
	/// TODO.
	GGZ = Self::letters_to_token(b"GGZ"),
	
	/// TODO.
	GHA = Self::letters_to_token(b"GHA"),
	
	/// TODO.
	GHB = Self::letters_to_token(b"GHB"),
	
	/// TODO.
	GHC = Self::letters_to_token(b"GHC"),
	
	/// TODO.
	GHD = Self::letters_to_token(b"GHD"),
	
	/// TODO.
	GHE = Self::letters_to_token(b"GHE"),
	
	/// TODO.
	GHF = Self::letters_to_token(b"GHF"),
	
	/// TODO.
	GHG = Self::letters_to_token(b"GHG"),
	
	/// TODO.
	GHH = Self::letters_to_token(b"GHH"),
	
	/// TODO.
	GHI = Self::letters_to_token(b"GHI"),
	
	/// TODO.
	GHJ = Self::letters_to_token(b"GHJ"),
	
	/// TODO.
	GHK = Self::letters_to_token(b"GHK"),
	
	/// TODO.
	GHL = Self::letters_to_token(b"GHL"),
	
	/// TODO.
	GHM = Self::letters_to_token(b"GHM"),
	
	/// TODO.
	GHN = Self::letters_to_token(b"GHN"),
	
	/// TODO.
	GHO = Self::letters_to_token(b"GHO"),
	
	/// TODO.
	GHP = Self::letters_to_token(b"GHP"),
	
	/// TODO.
	GHQ = Self::letters_to_token(b"GHQ"),
	
	/// TODO.
	GHR = Self::letters_to_token(b"GHR"),
	
	/// TODO.
	GHS = Self::letters_to_token(b"GHS"),
	
	/// TODO.
	GHT = Self::letters_to_token(b"GHT"),
	
	/// TODO.
	GHU = Self::letters_to_token(b"GHU"),
	
	/// TODO.
	GHV = Self::letters_to_token(b"GHV"),
	
	/// TODO.
	GHW = Self::letters_to_token(b"GHW"),
	
	/// TODO.
	GHX = Self::letters_to_token(b"GHX"),
	
	/// TODO.
	GHY = Self::letters_to_token(b"GHY"),
	
	/// TODO.
	GHZ = Self::letters_to_token(b"GHZ"),
	
	/// TODO.
	GIA = Self::letters_to_token(b"GIA"),
	
	/// TODO.
	GIB = Self::letters_to_token(b"GIB"),
	
	/// TODO.
	GIC = Self::letters_to_token(b"GIC"),
	
	/// TODO.
	GID = Self::letters_to_token(b"GID"),
	
	/// TODO.
	GIE = Self::letters_to_token(b"GIE"),
	
	/// TODO.
	GIF = Self::letters_to_token(b"GIF"),
	
	/// TODO.
	GIG = Self::letters_to_token(b"GIG"),
	
	/// TODO.
	GIH = Self::letters_to_token(b"GIH"),
	
	/// TODO.
	GII = Self::letters_to_token(b"GII"),
	
	/// TODO.
	GIJ = Self::letters_to_token(b"GIJ"),
	
	/// TODO.
	GIK = Self::letters_to_token(b"GIK"),
	
	/// TODO.
	GIL = Self::letters_to_token(b"GIL"),
	
	/// TODO.
	GIM = Self::letters_to_token(b"GIM"),
	
	/// TODO.
	GIN = Self::letters_to_token(b"GIN"),
	
	/// TODO.
	GIO = Self::letters_to_token(b"GIO"),
	
	/// TODO.
	GIP = Self::letters_to_token(b"GIP"),
	
	/// TODO.
	GIQ = Self::letters_to_token(b"GIQ"),
	
	/// TODO.
	GIR = Self::letters_to_token(b"GIR"),
	
	/// TODO.
	GIS = Self::letters_to_token(b"GIS"),
	
	/// TODO.
	GIT = Self::letters_to_token(b"GIT"),
	
	/// TODO.
	GIU = Self::letters_to_token(b"GIU"),
	
	/// TODO.
	GIV = Self::letters_to_token(b"GIV"),
	
	/// TODO.
	GIW = Self::letters_to_token(b"GIW"),
	
	/// TODO.
	GIX = Self::letters_to_token(b"GIX"),
	
	/// TODO.
	GIY = Self::letters_to_token(b"GIY"),
	
	/// TODO.
	GIZ = Self::letters_to_token(b"GIZ"),
	
	/// TODO.
	GJA = Self::letters_to_token(b"GJA"),
	
	/// TODO.
	GJB = Self::letters_to_token(b"GJB"),
	
	/// TODO.
	GJC = Self::letters_to_token(b"GJC"),
	
	/// TODO.
	GJD = Self::letters_to_token(b"GJD"),
	
	/// TODO.
	GJE = Self::letters_to_token(b"GJE"),
	
	/// TODO.
	GJF = Self::letters_to_token(b"GJF"),
	
	/// TODO.
	GJG = Self::letters_to_token(b"GJG"),
	
	/// TODO.
	GJH = Self::letters_to_token(b"GJH"),
	
	/// TODO.
	GJI = Self::letters_to_token(b"GJI"),
	
	/// TODO.
	GJJ = Self::letters_to_token(b"GJJ"),
	
	/// TODO.
	GJK = Self::letters_to_token(b"GJK"),
	
	/// TODO.
	GJL = Self::letters_to_token(b"GJL"),
	
	/// TODO.
	GJM = Self::letters_to_token(b"GJM"),
	
	/// TODO.
	GJN = Self::letters_to_token(b"GJN"),
	
	/// TODO.
	GJO = Self::letters_to_token(b"GJO"),
	
	/// TODO.
	GJP = Self::letters_to_token(b"GJP"),
	
	/// TODO.
	GJQ = Self::letters_to_token(b"GJQ"),
	
	/// TODO.
	GJR = Self::letters_to_token(b"GJR"),
	
	/// TODO.
	GJS = Self::letters_to_token(b"GJS"),
	
	/// TODO.
	GJT = Self::letters_to_token(b"GJT"),
	
	/// TODO.
	GJU = Self::letters_to_token(b"GJU"),
	
	/// TODO.
	GJV = Self::letters_to_token(b"GJV"),
	
	/// TODO.
	GJW = Self::letters_to_token(b"GJW"),
	
	/// TODO.
	GJX = Self::letters_to_token(b"GJX"),
	
	/// TODO.
	GJY = Self::letters_to_token(b"GJY"),
	
	/// TODO.
	GJZ = Self::letters_to_token(b"GJZ"),
	
	/// TODO.
	GKA = Self::letters_to_token(b"GKA"),
	
	/// TODO.
	GKB = Self::letters_to_token(b"GKB"),
	
	/// TODO.
	GKC = Self::letters_to_token(b"GKC"),
	
	/// TODO.
	GKD = Self::letters_to_token(b"GKD"),
	
	/// TODO.
	GKE = Self::letters_to_token(b"GKE"),
	
	/// TODO.
	GKF = Self::letters_to_token(b"GKF"),
	
	/// TODO.
	GKG = Self::letters_to_token(b"GKG"),
	
	/// TODO.
	GKH = Self::letters_to_token(b"GKH"),
	
	/// TODO.
	GKI = Self::letters_to_token(b"GKI"),
	
	/// TODO.
	GKJ = Self::letters_to_token(b"GKJ"),
	
	/// TODO.
	GKK = Self::letters_to_token(b"GKK"),
	
	/// TODO.
	GKL = Self::letters_to_token(b"GKL"),
	
	/// TODO.
	GKM = Self::letters_to_token(b"GKM"),
	
	/// TODO.
	GKN = Self::letters_to_token(b"GKN"),
	
	/// TODO.
	GKO = Self::letters_to_token(b"GKO"),
	
	/// TODO.
	GKP = Self::letters_to_token(b"GKP"),
	
	/// TODO.
	GKQ = Self::letters_to_token(b"GKQ"),
	
	/// TODO.
	GKR = Self::letters_to_token(b"GKR"),
	
	/// TODO.
	GKS = Self::letters_to_token(b"GKS"),
	
	/// TODO.
	GKT = Self::letters_to_token(b"GKT"),
	
	/// TODO.
	GKU = Self::letters_to_token(b"GKU"),
	
	/// TODO.
	GKV = Self::letters_to_token(b"GKV"),
	
	/// TODO.
	GKW = Self::letters_to_token(b"GKW"),
	
	/// TODO.
	GKX = Self::letters_to_token(b"GKX"),
	
	/// TODO.
	GKY = Self::letters_to_token(b"GKY"),
	
	/// TODO.
	GKZ = Self::letters_to_token(b"GKZ"),
	
	/// TODO.
	GLA = Self::letters_to_token(b"GLA"),
	
	/// TODO.
	GLB = Self::letters_to_token(b"GLB"),
	
	/// TODO.
	GLC = Self::letters_to_token(b"GLC"),
	
	/// TODO.
	GLD = Self::letters_to_token(b"GLD"),
	
	/// TODO.
	GLE = Self::letters_to_token(b"GLE"),
	
	/// TODO.
	GLF = Self::letters_to_token(b"GLF"),
	
	/// TODO.
	GLG = Self::letters_to_token(b"GLG"),
	
	/// TODO.
	GLH = Self::letters_to_token(b"GLH"),
	
	/// TODO.
	GLI = Self::letters_to_token(b"GLI"),
	
	/// TODO.
	GLJ = Self::letters_to_token(b"GLJ"),
	
	/// TODO.
	GLK = Self::letters_to_token(b"GLK"),
	
	/// TODO.
	GLL = Self::letters_to_token(b"GLL"),
	
	/// TODO.
	GLM = Self::letters_to_token(b"GLM"),
	
	/// TODO.
	GLN = Self::letters_to_token(b"GLN"),
	
	/// TODO.
	GLO = Self::letters_to_token(b"GLO"),
	
	/// TODO.
	GLP = Self::letters_to_token(b"GLP"),
	
	/// TODO.
	GLQ = Self::letters_to_token(b"GLQ"),
	
	/// TODO.
	GLR = Self::letters_to_token(b"GLR"),
	
	/// TODO.
	GLS = Self::letters_to_token(b"GLS"),
	
	/// TODO.
	GLT = Self::letters_to_token(b"GLT"),
	
	/// TODO.
	GLU = Self::letters_to_token(b"GLU"),
	
	/// TODO.
	GLV = Self::letters_to_token(b"GLV"),
	
	/// TODO.
	GLW = Self::letters_to_token(b"GLW"),
	
	/// TODO.
	GLX = Self::letters_to_token(b"GLX"),
	
	/// TODO.
	GLY = Self::letters_to_token(b"GLY"),
	
	/// TODO.
	GLZ = Self::letters_to_token(b"GLZ"),
	
	/// TODO.
	GMA = Self::letters_to_token(b"GMA"),
	
	/// TODO.
	GMB = Self::letters_to_token(b"GMB"),
	
	/// TODO.
	GMC = Self::letters_to_token(b"GMC"),
	
	/// TODO.
	GMD = Self::letters_to_token(b"GMD"),
	
	/// TODO.
	GME = Self::letters_to_token(b"GME"),
	
	/// TODO.
	GMF = Self::letters_to_token(b"GMF"),
	
	/// TODO.
	GMG = Self::letters_to_token(b"GMG"),
	
	/// TODO.
	GMH = Self::letters_to_token(b"GMH"),
	
	/// TODO.
	GMI = Self::letters_to_token(b"GMI"),
	
	/// TODO.
	GMJ = Self::letters_to_token(b"GMJ"),
	
	/// TODO.
	GMK = Self::letters_to_token(b"GMK"),
	
	/// TODO.
	GML = Self::letters_to_token(b"GML"),
	
	/// TODO.
	GMM = Self::letters_to_token(b"GMM"),
	
	/// TODO.
	GMN = Self::letters_to_token(b"GMN"),
	
	/// TODO.
	GMO = Self::letters_to_token(b"GMO"),
	
	/// TODO.
	GMP = Self::letters_to_token(b"GMP"),
	
	/// TODO.
	GMQ = Self::letters_to_token(b"GMQ"),
	
	/// TODO.
	GMR = Self::letters_to_token(b"GMR"),
	
	/// TODO.
	GMS = Self::letters_to_token(b"GMS"),
	
	/// TODO.
	GMT = Self::letters_to_token(b"GMT"),
	
	/// TODO.
	GMU = Self::letters_to_token(b"GMU"),
	
	/// TODO.
	GMV = Self::letters_to_token(b"GMV"),
	
	/// TODO.
	GMW = Self::letters_to_token(b"GMW"),
	
	/// TODO.
	GMX = Self::letters_to_token(b"GMX"),
	
	/// TODO.
	GMY = Self::letters_to_token(b"GMY"),
	
	/// TODO.
	GMZ = Self::letters_to_token(b"GMZ"),
	
	/// TODO.
	GNA = Self::letters_to_token(b"GNA"),
	
	/// TODO.
	GNB = Self::letters_to_token(b"GNB"),
	
	/// TODO.
	GNC = Self::letters_to_token(b"GNC"),
	
	/// TODO.
	GND = Self::letters_to_token(b"GND"),
	
	/// TODO.
	GNE = Self::letters_to_token(b"GNE"),
	
	/// TODO.
	GNF = Self::letters_to_token(b"GNF"),
	
	/// TODO.
	GNG = Self::letters_to_token(b"GNG"),
	
	/// TODO.
	GNH = Self::letters_to_token(b"GNH"),
	
	/// TODO.
	GNI = Self::letters_to_token(b"GNI"),
	
	/// TODO.
	GNJ = Self::letters_to_token(b"GNJ"),
	
	/// TODO.
	GNK = Self::letters_to_token(b"GNK"),
	
	/// TODO.
	GNL = Self::letters_to_token(b"GNL"),
	
	/// TODO.
	GNM = Self::letters_to_token(b"GNM"),
	
	/// TODO.
	GNN = Self::letters_to_token(b"GNN"),
	
	/// TODO.
	GNO = Self::letters_to_token(b"GNO"),
	
	/// TODO.
	GNP = Self::letters_to_token(b"GNP"),
	
	/// TODO.
	GNQ = Self::letters_to_token(b"GNQ"),
	
	/// TODO.
	GNR = Self::letters_to_token(b"GNR"),
	
	/// TODO.
	GNS = Self::letters_to_token(b"GNS"),
	
	/// TODO.
	GNT = Self::letters_to_token(b"GNT"),
	
	/// TODO.
	GNU = Self::letters_to_token(b"GNU"),
	
	/// TODO.
	GNV = Self::letters_to_token(b"GNV"),
	
	/// TODO.
	GNW = Self::letters_to_token(b"GNW"),
	
	/// TODO.
	GNX = Self::letters_to_token(b"GNX"),
	
	/// TODO.
	GNY = Self::letters_to_token(b"GNY"),
	
	/// TODO.
	GNZ = Self::letters_to_token(b"GNZ"),
	
	/// TODO.
	GOA = Self::letters_to_token(b"GOA"),
	
	/// TODO.
	GOB = Self::letters_to_token(b"GOB"),
	
	/// TODO.
	GOC = Self::letters_to_token(b"GOC"),
	
	/// TODO.
	GOD = Self::letters_to_token(b"GOD"),
	
	/// TODO.
	GOE = Self::letters_to_token(b"GOE"),
	
	/// TODO.
	GOF = Self::letters_to_token(b"GOF"),
	
	/// TODO.
	GOG = Self::letters_to_token(b"GOG"),
	
	/// TODO.
	GOH = Self::letters_to_token(b"GOH"),
	
	/// TODO.
	GOI = Self::letters_to_token(b"GOI"),
	
	/// TODO.
	GOJ = Self::letters_to_token(b"GOJ"),
	
	/// TODO.
	GOK = Self::letters_to_token(b"GOK"),
	
	/// TODO.
	GOL = Self::letters_to_token(b"GOL"),
	
	/// TODO.
	GOM = Self::letters_to_token(b"GOM"),
	
	/// TODO.
	GON = Self::letters_to_token(b"GON"),
	
	/// TODO.
	GOO = Self::letters_to_token(b"GOO"),
	
	/// TODO.
	GOP = Self::letters_to_token(b"GOP"),
	
	/// TODO.
	GOQ = Self::letters_to_token(b"GOQ"),
	
	/// TODO.
	GOR = Self::letters_to_token(b"GOR"),
	
	/// TODO.
	GOS = Self::letters_to_token(b"GOS"),
	
	/// TODO.
	GOT = Self::letters_to_token(b"GOT"),
	
	/// TODO.
	GOU = Self::letters_to_token(b"GOU"),
	
	/// TODO.
	GOV = Self::letters_to_token(b"GOV"),
	
	/// TODO.
	GOW = Self::letters_to_token(b"GOW"),
	
	/// TODO.
	GOX = Self::letters_to_token(b"GOX"),
	
	/// TODO.
	GOY = Self::letters_to_token(b"GOY"),
	
	/// TODO.
	GOZ = Self::letters_to_token(b"GOZ"),
	
	/// TODO.
	GPA = Self::letters_to_token(b"GPA"),
	
	/// TODO.
	GPB = Self::letters_to_token(b"GPB"),
	
	/// TODO.
	GPC = Self::letters_to_token(b"GPC"),
	
	/// TODO.
	GPD = Self::letters_to_token(b"GPD"),
	
	/// TODO.
	GPE = Self::letters_to_token(b"GPE"),
	
	/// TODO.
	GPF = Self::letters_to_token(b"GPF"),
	
	/// TODO.
	GPG = Self::letters_to_token(b"GPG"),
	
	/// TODO.
	GPH = Self::letters_to_token(b"GPH"),
	
	/// TODO.
	GPI = Self::letters_to_token(b"GPI"),
	
	/// TODO.
	GPJ = Self::letters_to_token(b"GPJ"),
	
	/// TODO.
	GPK = Self::letters_to_token(b"GPK"),
	
	/// TODO.
	GPL = Self::letters_to_token(b"GPL"),
	
	/// TODO.
	GPM = Self::letters_to_token(b"GPM"),
	
	/// TODO.
	GPN = Self::letters_to_token(b"GPN"),
	
	/// TODO.
	GPO = Self::letters_to_token(b"GPO"),
	
	/// TODO.
	GPP = Self::letters_to_token(b"GPP"),
	
	/// TODO.
	GPQ = Self::letters_to_token(b"GPQ"),
	
	/// TODO.
	GPR = Self::letters_to_token(b"GPR"),
	
	/// TODO.
	GPS = Self::letters_to_token(b"GPS"),
	
	/// TODO.
	GPT = Self::letters_to_token(b"GPT"),
	
	/// TODO.
	GPU = Self::letters_to_token(b"GPU"),
	
	/// TODO.
	GPV = Self::letters_to_token(b"GPV"),
	
	/// TODO.
	GPW = Self::letters_to_token(b"GPW"),
	
	/// TODO.
	GPX = Self::letters_to_token(b"GPX"),
	
	/// TODO.
	GPY = Self::letters_to_token(b"GPY"),
	
	/// TODO.
	GPZ = Self::letters_to_token(b"GPZ"),
	
	/// TODO.
	GQA = Self::letters_to_token(b"GQA"),
	
	/// TODO.
	GQB = Self::letters_to_token(b"GQB"),
	
	/// TODO.
	GQC = Self::letters_to_token(b"GQC"),
	
	/// TODO.
	GQD = Self::letters_to_token(b"GQD"),
	
	/// TODO.
	GQE = Self::letters_to_token(b"GQE"),
	
	/// TODO.
	GQF = Self::letters_to_token(b"GQF"),
	
	/// TODO.
	GQG = Self::letters_to_token(b"GQG"),
	
	/// TODO.
	GQH = Self::letters_to_token(b"GQH"),
	
	/// TODO.
	GQI = Self::letters_to_token(b"GQI"),
	
	/// TODO.
	GQJ = Self::letters_to_token(b"GQJ"),
	
	/// TODO.
	GQK = Self::letters_to_token(b"GQK"),
	
	/// TODO.
	GQL = Self::letters_to_token(b"GQL"),
	
	/// TODO.
	GQM = Self::letters_to_token(b"GQM"),
	
	/// TODO.
	GQN = Self::letters_to_token(b"GQN"),
	
	/// TODO.
	GQO = Self::letters_to_token(b"GQO"),
	
	/// TODO.
	GQP = Self::letters_to_token(b"GQP"),
	
	/// TODO.
	GQQ = Self::letters_to_token(b"GQQ"),
	
	/// TODO.
	GQR = Self::letters_to_token(b"GQR"),
	
	/// TODO.
	GQS = Self::letters_to_token(b"GQS"),
	
	/// TODO.
	GQT = Self::letters_to_token(b"GQT"),
	
	/// TODO.
	GQU = Self::letters_to_token(b"GQU"),
	
	/// TODO.
	GQV = Self::letters_to_token(b"GQV"),
	
	/// TODO.
	GQW = Self::letters_to_token(b"GQW"),
	
	/// TODO.
	GQX = Self::letters_to_token(b"GQX"),
	
	/// TODO.
	GQY = Self::letters_to_token(b"GQY"),
	
	/// TODO.
	GQZ = Self::letters_to_token(b"GQZ"),
	
	/// TODO.
	GRA = Self::letters_to_token(b"GRA"),
	
	/// TODO.
	GRB = Self::letters_to_token(b"GRB"),
	
	/// TODO.
	GRC = Self::letters_to_token(b"GRC"),
	
	/// TODO.
	GRD = Self::letters_to_token(b"GRD"),
	
	/// TODO.
	GRE = Self::letters_to_token(b"GRE"),
	
	/// TODO.
	GRF = Self::letters_to_token(b"GRF"),
	
	/// TODO.
	GRG = Self::letters_to_token(b"GRG"),
	
	/// TODO.
	GRH = Self::letters_to_token(b"GRH"),
	
	/// TODO.
	GRI = Self::letters_to_token(b"GRI"),
	
	/// TODO.
	GRJ = Self::letters_to_token(b"GRJ"),
	
	/// TODO.
	GRK = Self::letters_to_token(b"GRK"),
	
	/// TODO.
	GRL = Self::letters_to_token(b"GRL"),
	
	/// TODO.
	GRM = Self::letters_to_token(b"GRM"),
	
	/// TODO.
	GRN = Self::letters_to_token(b"GRN"),
	
	/// TODO.
	GRO = Self::letters_to_token(b"GRO"),
	
	/// TODO.
	GRP = Self::letters_to_token(b"GRP"),
	
	/// TODO.
	GRQ = Self::letters_to_token(b"GRQ"),
	
	/// TODO.
	GRR = Self::letters_to_token(b"GRR"),
	
	/// TODO.
	GRS = Self::letters_to_token(b"GRS"),
	
	/// TODO.
	GRT = Self::letters_to_token(b"GRT"),
	
	/// TODO.
	GRU = Self::letters_to_token(b"GRU"),
	
	/// TODO.
	GRV = Self::letters_to_token(b"GRV"),
	
	/// TODO.
	GRW = Self::letters_to_token(b"GRW"),
	
	/// TODO.
	GRX = Self::letters_to_token(b"GRX"),
	
	/// TODO.
	GRY = Self::letters_to_token(b"GRY"),
	
	/// TODO.
	GRZ = Self::letters_to_token(b"GRZ"),
	
	/// TODO.
	GSA = Self::letters_to_token(b"GSA"),
	
	/// TODO.
	GSB = Self::letters_to_token(b"GSB"),
	
	/// TODO.
	GSC = Self::letters_to_token(b"GSC"),
	
	/// TODO.
	GSD = Self::letters_to_token(b"GSD"),
	
	/// TODO.
	GSE = Self::letters_to_token(b"GSE"),
	
	/// TODO.
	GSF = Self::letters_to_token(b"GSF"),
	
	/// TODO.
	GSG = Self::letters_to_token(b"GSG"),
	
	/// TODO.
	GSH = Self::letters_to_token(b"GSH"),
	
	/// TODO.
	GSI = Self::letters_to_token(b"GSI"),
	
	/// TODO.
	GSJ = Self::letters_to_token(b"GSJ"),
	
	/// TODO.
	GSK = Self::letters_to_token(b"GSK"),
	
	/// TODO.
	GSL = Self::letters_to_token(b"GSL"),
	
	/// TODO.
	GSM = Self::letters_to_token(b"GSM"),
	
	/// TODO.
	GSN = Self::letters_to_token(b"GSN"),
	
	/// TODO.
	GSO = Self::letters_to_token(b"GSO"),
	
	/// TODO.
	GSP = Self::letters_to_token(b"GSP"),
	
	/// TODO.
	GSQ = Self::letters_to_token(b"GSQ"),
	
	/// TODO.
	GSR = Self::letters_to_token(b"GSR"),
	
	/// TODO.
	GSS = Self::letters_to_token(b"GSS"),
	
	/// TODO.
	GST = Self::letters_to_token(b"GST"),
	
	/// TODO.
	GSU = Self::letters_to_token(b"GSU"),
	
	/// TODO.
	GSV = Self::letters_to_token(b"GSV"),
	
	/// TODO.
	GSW = Self::letters_to_token(b"GSW"),
	
	/// TODO.
	GSX = Self::letters_to_token(b"GSX"),
	
	/// TODO.
	GSY = Self::letters_to_token(b"GSY"),
	
	/// TODO.
	GSZ = Self::letters_to_token(b"GSZ"),
	
	/// TODO.
	GTA = Self::letters_to_token(b"GTA"),
	
	/// TODO.
	GTB = Self::letters_to_token(b"GTB"),
	
	/// TODO.
	GTC = Self::letters_to_token(b"GTC"),
	
	/// TODO.
	GTD = Self::letters_to_token(b"GTD"),
	
	/// TODO.
	GTE = Self::letters_to_token(b"GTE"),
	
	/// TODO.
	GTF = Self::letters_to_token(b"GTF"),
	
	/// TODO.
	GTG = Self::letters_to_token(b"GTG"),
	
	/// TODO.
	GTH = Self::letters_to_token(b"GTH"),
	
	/// TODO.
	GTI = Self::letters_to_token(b"GTI"),
	
	/// TODO.
	GTJ = Self::letters_to_token(b"GTJ"),
	
	/// TODO.
	GTK = Self::letters_to_token(b"GTK"),
	
	/// TODO.
	GTL = Self::letters_to_token(b"GTL"),
	
	/// TODO.
	GTM = Self::letters_to_token(b"GTM"),
	
	/// TODO.
	GTN = Self::letters_to_token(b"GTN"),
	
	/// TODO.
	GTO = Self::letters_to_token(b"GTO"),
	
	/// TODO.
	GTP = Self::letters_to_token(b"GTP"),
	
	/// TODO.
	GTQ = Self::letters_to_token(b"GTQ"),
	
	/// TODO.
	GTR = Self::letters_to_token(b"GTR"),
	
	/// TODO.
	GTS = Self::letters_to_token(b"GTS"),
	
	/// TODO.
	GTT = Self::letters_to_token(b"GTT"),
	
	/// TODO.
	GTU = Self::letters_to_token(b"GTU"),
	
	/// TODO.
	GTV = Self::letters_to_token(b"GTV"),
	
	/// TODO.
	GTW = Self::letters_to_token(b"GTW"),
	
	/// TODO.
	GTX = Self::letters_to_token(b"GTX"),
	
	/// TODO.
	GTY = Self::letters_to_token(b"GTY"),
	
	/// TODO.
	GTZ = Self::letters_to_token(b"GTZ"),
	
	/// TODO.
	GUA = Self::letters_to_token(b"GUA"),
	
	/// TODO.
	GUB = Self::letters_to_token(b"GUB"),
	
	/// TODO.
	GUC = Self::letters_to_token(b"GUC"),
	
	/// TODO.
	GUD = Self::letters_to_token(b"GUD"),
	
	/// TODO.
	GUE = Self::letters_to_token(b"GUE"),
	
	/// TODO.
	GUF = Self::letters_to_token(b"GUF"),
	
	/// TODO.
	GUG = Self::letters_to_token(b"GUG"),
	
	/// TODO.
	GUH = Self::letters_to_token(b"GUH"),
	
	/// TODO.
	GUI = Self::letters_to_token(b"GUI"),
	
	/// TODO.
	GUJ = Self::letters_to_token(b"GUJ"),
	
	/// TODO.
	GUK = Self::letters_to_token(b"GUK"),
	
	/// TODO.
	GUL = Self::letters_to_token(b"GUL"),
	
	/// TODO.
	GUM = Self::letters_to_token(b"GUM"),
	
	/// TODO.
	GUN = Self::letters_to_token(b"GUN"),
	
	/// TODO.
	GUO = Self::letters_to_token(b"GUO"),
	
	/// TODO.
	GUP = Self::letters_to_token(b"GUP"),
	
	/// TODO.
	GUQ = Self::letters_to_token(b"GUQ"),
	
	/// TODO.
	GUR = Self::letters_to_token(b"GUR"),
	
	/// TODO.
	GUS = Self::letters_to_token(b"GUS"),
	
	/// TODO.
	GUT = Self::letters_to_token(b"GUT"),
	
	/// TODO.
	GUU = Self::letters_to_token(b"GUU"),
	
	/// TODO.
	GUV = Self::letters_to_token(b"GUV"),
	
	/// TODO.
	GUW = Self::letters_to_token(b"GUW"),
	
	/// TODO.
	GUX = Self::letters_to_token(b"GUX"),
	
	/// TODO.
	GUY = Self::letters_to_token(b"GUY"),
	
	/// TODO.
	GUZ = Self::letters_to_token(b"GUZ"),
	
	/// TODO.
	GVA = Self::letters_to_token(b"GVA"),
	
	/// TODO.
	GVB = Self::letters_to_token(b"GVB"),
	
	/// TODO.
	GVC = Self::letters_to_token(b"GVC"),
	
	/// TODO.
	GVD = Self::letters_to_token(b"GVD"),
	
	/// TODO.
	GVE = Self::letters_to_token(b"GVE"),
	
	/// TODO.
	GVF = Self::letters_to_token(b"GVF"),
	
	/// TODO.
	GVG = Self::letters_to_token(b"GVG"),
	
	/// TODO.
	GVH = Self::letters_to_token(b"GVH"),
	
	/// TODO.
	GVI = Self::letters_to_token(b"GVI"),
	
	/// TODO.
	GVJ = Self::letters_to_token(b"GVJ"),
	
	/// TODO.
	GVK = Self::letters_to_token(b"GVK"),
	
	/// TODO.
	GVL = Self::letters_to_token(b"GVL"),
	
	/// TODO.
	GVM = Self::letters_to_token(b"GVM"),
	
	/// TODO.
	GVN = Self::letters_to_token(b"GVN"),
	
	/// TODO.
	GVO = Self::letters_to_token(b"GVO"),
	
	/// TODO.
	GVP = Self::letters_to_token(b"GVP"),
	
	/// TODO.
	GVQ = Self::letters_to_token(b"GVQ"),
	
	/// TODO.
	GVR = Self::letters_to_token(b"GVR"),
	
	/// TODO.
	GVS = Self::letters_to_token(b"GVS"),
	
	/// TODO.
	GVT = Self::letters_to_token(b"GVT"),
	
	/// TODO.
	GVU = Self::letters_to_token(b"GVU"),
	
	/// TODO.
	GVV = Self::letters_to_token(b"GVV"),
	
	/// TODO.
	GVW = Self::letters_to_token(b"GVW"),
	
	/// TODO.
	GVX = Self::letters_to_token(b"GVX"),
	
	/// TODO.
	GVY = Self::letters_to_token(b"GVY"),
	
	/// TODO.
	GVZ = Self::letters_to_token(b"GVZ"),
	
	/// TODO.
	GWA = Self::letters_to_token(b"GWA"),
	
	/// TODO.
	GWB = Self::letters_to_token(b"GWB"),
	
	/// TODO.
	GWC = Self::letters_to_token(b"GWC"),
	
	/// TODO.
	GWD = Self::letters_to_token(b"GWD"),
	
	/// TODO.
	GWE = Self::letters_to_token(b"GWE"),
	
	/// TODO.
	GWF = Self::letters_to_token(b"GWF"),
	
	/// TODO.
	GWG = Self::letters_to_token(b"GWG"),
	
	/// TODO.
	GWH = Self::letters_to_token(b"GWH"),
	
	/// TODO.
	GWI = Self::letters_to_token(b"GWI"),
	
	/// TODO.
	GWJ = Self::letters_to_token(b"GWJ"),
	
	/// TODO.
	GWK = Self::letters_to_token(b"GWK"),
	
	/// TODO.
	GWL = Self::letters_to_token(b"GWL"),
	
	/// TODO.
	GWM = Self::letters_to_token(b"GWM"),
	
	/// TODO.
	GWN = Self::letters_to_token(b"GWN"),
	
	/// TODO.
	GWO = Self::letters_to_token(b"GWO"),
	
	/// TODO.
	GWP = Self::letters_to_token(b"GWP"),
	
	/// TODO.
	GWQ = Self::letters_to_token(b"GWQ"),
	
	/// TODO.
	GWR = Self::letters_to_token(b"GWR"),
	
	/// TODO.
	GWS = Self::letters_to_token(b"GWS"),
	
	/// TODO.
	GWT = Self::letters_to_token(b"GWT"),
	
	/// TODO.
	GWU = Self::letters_to_token(b"GWU"),
	
	/// TODO.
	GWV = Self::letters_to_token(b"GWV"),
	
	/// TODO.
	GWW = Self::letters_to_token(b"GWW"),
	
	/// TODO.
	GWX = Self::letters_to_token(b"GWX"),
	
	/// TODO.
	GWY = Self::letters_to_token(b"GWY"),
	
	/// TODO.
	GWZ = Self::letters_to_token(b"GWZ"),
	
	/// TODO.
	GXA = Self::letters_to_token(b"GXA"),
	
	/// TODO.
	GXB = Self::letters_to_token(b"GXB"),
	
	/// TODO.
	GXC = Self::letters_to_token(b"GXC"),
	
	/// TODO.
	GXD = Self::letters_to_token(b"GXD"),
	
	/// TODO.
	GXE = Self::letters_to_token(b"GXE"),
	
	/// TODO.
	GXF = Self::letters_to_token(b"GXF"),
	
	/// TODO.
	GXG = Self::letters_to_token(b"GXG"),
	
	/// TODO.
	GXH = Self::letters_to_token(b"GXH"),
	
	/// TODO.
	GXI = Self::letters_to_token(b"GXI"),
	
	/// TODO.
	GXJ = Self::letters_to_token(b"GXJ"),
	
	/// TODO.
	GXK = Self::letters_to_token(b"GXK"),
	
	/// TODO.
	GXL = Self::letters_to_token(b"GXL"),
	
	/// TODO.
	GXM = Self::letters_to_token(b"GXM"),
	
	/// TODO.
	GXN = Self::letters_to_token(b"GXN"),
	
	/// TODO.
	GXO = Self::letters_to_token(b"GXO"),
	
	/// TODO.
	GXP = Self::letters_to_token(b"GXP"),
	
	/// TODO.
	GXQ = Self::letters_to_token(b"GXQ"),
	
	/// TODO.
	GXR = Self::letters_to_token(b"GXR"),
	
	/// TODO.
	GXS = Self::letters_to_token(b"GXS"),
	
	/// TODO.
	GXT = Self::letters_to_token(b"GXT"),
	
	/// TODO.
	GXU = Self::letters_to_token(b"GXU"),
	
	/// TODO.
	GXV = Self::letters_to_token(b"GXV"),
	
	/// TODO.
	GXW = Self::letters_to_token(b"GXW"),
	
	/// TODO.
	GXX = Self::letters_to_token(b"GXX"),
	
	/// TODO.
	GXY = Self::letters_to_token(b"GXY"),
	
	/// TODO.
	GXZ = Self::letters_to_token(b"GXZ"),
	
	/// TODO.
	GYA = Self::letters_to_token(b"GYA"),
	
	/// TODO.
	GYB = Self::letters_to_token(b"GYB"),
	
	/// TODO.
	GYC = Self::letters_to_token(b"GYC"),
	
	/// TODO.
	GYD = Self::letters_to_token(b"GYD"),
	
	/// TODO.
	GYE = Self::letters_to_token(b"GYE"),
	
	/// TODO.
	GYF = Self::letters_to_token(b"GYF"),
	
	/// TODO.
	GYG = Self::letters_to_token(b"GYG"),
	
	/// TODO.
	GYH = Self::letters_to_token(b"GYH"),
	
	/// TODO.
	GYI = Self::letters_to_token(b"GYI"),
	
	/// TODO.
	GYJ = Self::letters_to_token(b"GYJ"),
	
	/// TODO.
	GYK = Self::letters_to_token(b"GYK"),
	
	/// TODO.
	GYL = Self::letters_to_token(b"GYL"),
	
	/// TODO.
	GYM = Self::letters_to_token(b"GYM"),
	
	/// TODO.
	GYN = Self::letters_to_token(b"GYN"),
	
	/// TODO.
	GYO = Self::letters_to_token(b"GYO"),
	
	/// TODO.
	GYP = Self::letters_to_token(b"GYP"),
	
	/// TODO.
	GYQ = Self::letters_to_token(b"GYQ"),
	
	/// TODO.
	GYR = Self::letters_to_token(b"GYR"),
	
	/// TODO.
	GYS = Self::letters_to_token(b"GYS"),
	
	/// TODO.
	GYT = Self::letters_to_token(b"GYT"),
	
	/// TODO.
	GYU = Self::letters_to_token(b"GYU"),
	
	/// TODO.
	GYV = Self::letters_to_token(b"GYV"),
	
	/// TODO.
	GYW = Self::letters_to_token(b"GYW"),
	
	/// TODO.
	GYX = Self::letters_to_token(b"GYX"),
	
	/// TODO.
	GYY = Self::letters_to_token(b"GYY"),
	
	/// TODO.
	GYZ = Self::letters_to_token(b"GYZ"),
	
	/// TODO.
	GZA = Self::letters_to_token(b"GZA"),
	
	/// TODO.
	GZB = Self::letters_to_token(b"GZB"),
	
	/// TODO.
	GZC = Self::letters_to_token(b"GZC"),
	
	/// TODO.
	GZD = Self::letters_to_token(b"GZD"),
	
	/// TODO.
	GZE = Self::letters_to_token(b"GZE"),
	
	/// TODO.
	GZF = Self::letters_to_token(b"GZF"),
	
	/// TODO.
	GZG = Self::letters_to_token(b"GZG"),
	
	/// TODO.
	GZH = Self::letters_to_token(b"GZH"),
	
	/// TODO.
	GZI = Self::letters_to_token(b"GZI"),
	
	/// TODO.
	GZJ = Self::letters_to_token(b"GZJ"),
	
	/// TODO.
	GZK = Self::letters_to_token(b"GZK"),
	
	/// TODO.
	GZL = Self::letters_to_token(b"GZL"),
	
	/// TODO.
	GZM = Self::letters_to_token(b"GZM"),
	
	/// TODO.
	GZN = Self::letters_to_token(b"GZN"),
	
	/// TODO.
	GZO = Self::letters_to_token(b"GZO"),
	
	/// TODO.
	GZP = Self::letters_to_token(b"GZP"),
	
	/// TODO.
	GZQ = Self::letters_to_token(b"GZQ"),
	
	/// TODO.
	GZR = Self::letters_to_token(b"GZR"),
	
	/// TODO.
	GZS = Self::letters_to_token(b"GZS"),
	
	/// TODO.
	GZT = Self::letters_to_token(b"GZT"),
	
	/// TODO.
	GZU = Self::letters_to_token(b"GZU"),
	
	/// TODO.
	GZV = Self::letters_to_token(b"GZV"),
	
	/// TODO.
	GZW = Self::letters_to_token(b"GZW"),
	
	/// TODO.
	GZX = Self::letters_to_token(b"GZX"),
	
	/// TODO.
	GZY = Self::letters_to_token(b"GZY"),
	
	/// TODO.
	GZZ = Self::letters_to_token(b"GZZ"),
	
	/// TODO.
	HAA = Self::letters_to_token(b"HAA"),
	
	/// TODO.
	HAB = Self::letters_to_token(b"HAB"),
	
	/// TODO.
	HAC = Self::letters_to_token(b"HAC"),
	
	/// TODO.
	HAD = Self::letters_to_token(b"HAD"),
	
	/// TODO.
	HAE = Self::letters_to_token(b"HAE"),
	
	/// TODO.
	HAF = Self::letters_to_token(b"HAF"),
	
	/// TODO.
	HAG = Self::letters_to_token(b"HAG"),
	
	/// TODO.
	HAH = Self::letters_to_token(b"HAH"),
	
	/// TODO.
	HAI = Self::letters_to_token(b"HAI"),
	
	/// TODO.
	HAJ = Self::letters_to_token(b"HAJ"),
	
	/// TODO.
	HAK = Self::letters_to_token(b"HAK"),
	
	/// TODO.
	HAL = Self::letters_to_token(b"HAL"),
	
	/// TODO.
	HAM = Self::letters_to_token(b"HAM"),
	
	/// TODO.
	HAN = Self::letters_to_token(b"HAN"),
	
	/// TODO.
	HAO = Self::letters_to_token(b"HAO"),
	
	/// TODO.
	HAP = Self::letters_to_token(b"HAP"),
	
	/// TODO.
	HAQ = Self::letters_to_token(b"HAQ"),
	
	/// TODO.
	HAR = Self::letters_to_token(b"HAR"),
	
	/// TODO.
	HAS = Self::letters_to_token(b"HAS"),
	
	/// TODO.
	HAT = Self::letters_to_token(b"HAT"),
	
	/// TODO.
	HAU = Self::letters_to_token(b"HAU"),
	
	/// TODO.
	HAV = Self::letters_to_token(b"HAV"),
	
	/// TODO.
	HAW = Self::letters_to_token(b"HAW"),
	
	/// TODO.
	HAX = Self::letters_to_token(b"HAX"),
	
	/// TODO.
	HAY = Self::letters_to_token(b"HAY"),
	
	/// TODO.
	HAZ = Self::letters_to_token(b"HAZ"),
	
	/// TODO.
	HBA = Self::letters_to_token(b"HBA"),
	
	/// TODO.
	HBB = Self::letters_to_token(b"HBB"),
	
	/// TODO.
	HBC = Self::letters_to_token(b"HBC"),
	
	/// TODO.
	HBD = Self::letters_to_token(b"HBD"),
	
	/// TODO.
	HBE = Self::letters_to_token(b"HBE"),
	
	/// TODO.
	HBF = Self::letters_to_token(b"HBF"),
	
	/// TODO.
	HBG = Self::letters_to_token(b"HBG"),
	
	/// TODO.
	HBH = Self::letters_to_token(b"HBH"),
	
	/// TODO.
	HBI = Self::letters_to_token(b"HBI"),
	
	/// TODO.
	HBJ = Self::letters_to_token(b"HBJ"),
	
	/// TODO.
	HBK = Self::letters_to_token(b"HBK"),
	
	/// TODO.
	HBL = Self::letters_to_token(b"HBL"),
	
	/// TODO.
	HBM = Self::letters_to_token(b"HBM"),
	
	/// TODO.
	HBN = Self::letters_to_token(b"HBN"),
	
	/// TODO.
	HBO = Self::letters_to_token(b"HBO"),
	
	/// TODO.
	HBP = Self::letters_to_token(b"HBP"),
	
	/// TODO.
	HBQ = Self::letters_to_token(b"HBQ"),
	
	/// TODO.
	HBR = Self::letters_to_token(b"HBR"),
	
	/// TODO.
	HBS = Self::letters_to_token(b"HBS"),
	
	/// TODO.
	HBT = Self::letters_to_token(b"HBT"),
	
	/// TODO.
	HBU = Self::letters_to_token(b"HBU"),
	
	/// TODO.
	HBV = Self::letters_to_token(b"HBV"),
	
	/// TODO.
	HBW = Self::letters_to_token(b"HBW"),
	
	/// TODO.
	HBX = Self::letters_to_token(b"HBX"),
	
	/// TODO.
	HBY = Self::letters_to_token(b"HBY"),
	
	/// TODO.
	HBZ = Self::letters_to_token(b"HBZ"),
	
	/// TODO.
	HCA = Self::letters_to_token(b"HCA"),
	
	/// TODO.
	HCB = Self::letters_to_token(b"HCB"),
	
	/// TODO.
	HCC = Self::letters_to_token(b"HCC"),
	
	/// TODO.
	HCD = Self::letters_to_token(b"HCD"),
	
	/// TODO.
	HCE = Self::letters_to_token(b"HCE"),
	
	/// TODO.
	HCF = Self::letters_to_token(b"HCF"),
	
	/// TODO.
	HCG = Self::letters_to_token(b"HCG"),
	
	/// TODO.
	HCH = Self::letters_to_token(b"HCH"),
	
	/// TODO.
	HCI = Self::letters_to_token(b"HCI"),
	
	/// TODO.
	HCJ = Self::letters_to_token(b"HCJ"),
	
	/// TODO.
	HCK = Self::letters_to_token(b"HCK"),
	
	/// TODO.
	HCL = Self::letters_to_token(b"HCL"),
	
	/// TODO.
	HCM = Self::letters_to_token(b"HCM"),
	
	/// TODO.
	HCN = Self::letters_to_token(b"HCN"),
	
	/// TODO.
	HCO = Self::letters_to_token(b"HCO"),
	
	/// TODO.
	HCP = Self::letters_to_token(b"HCP"),
	
	/// TODO.
	HCQ = Self::letters_to_token(b"HCQ"),
	
	/// TODO.
	HCR = Self::letters_to_token(b"HCR"),
	
	/// TODO.
	HCS = Self::letters_to_token(b"HCS"),
	
	/// TODO.
	HCT = Self::letters_to_token(b"HCT"),
	
	/// TODO.
	HCU = Self::letters_to_token(b"HCU"),
	
	/// TODO.
	HCV = Self::letters_to_token(b"HCV"),
	
	/// TODO.
	HCW = Self::letters_to_token(b"HCW"),
	
	/// TODO.
	HCX = Self::letters_to_token(b"HCX"),
	
	/// TODO.
	HCY = Self::letters_to_token(b"HCY"),
	
	/// TODO.
	HCZ = Self::letters_to_token(b"HCZ"),
	
	/// TODO.
	HDA = Self::letters_to_token(b"HDA"),
	
	/// TODO.
	HDB = Self::letters_to_token(b"HDB"),
	
	/// TODO.
	HDC = Self::letters_to_token(b"HDC"),
	
	/// TODO.
	HDD = Self::letters_to_token(b"HDD"),
	
	/// TODO.
	HDE = Self::letters_to_token(b"HDE"),
	
	/// TODO.
	HDF = Self::letters_to_token(b"HDF"),
	
	/// TODO.
	HDG = Self::letters_to_token(b"HDG"),
	
	/// TODO.
	HDH = Self::letters_to_token(b"HDH"),
	
	/// TODO.
	HDI = Self::letters_to_token(b"HDI"),
	
	/// TODO.
	HDJ = Self::letters_to_token(b"HDJ"),
	
	/// TODO.
	HDK = Self::letters_to_token(b"HDK"),
	
	/// TODO.
	HDL = Self::letters_to_token(b"HDL"),
	
	/// TODO.
	HDM = Self::letters_to_token(b"HDM"),
	
	/// TODO.
	HDN = Self::letters_to_token(b"HDN"),
	
	/// TODO.
	HDO = Self::letters_to_token(b"HDO"),
	
	/// TODO.
	HDP = Self::letters_to_token(b"HDP"),
	
	/// TODO.
	HDQ = Self::letters_to_token(b"HDQ"),
	
	/// TODO.
	HDR = Self::letters_to_token(b"HDR"),
	
	/// TODO.
	HDS = Self::letters_to_token(b"HDS"),
	
	/// TODO.
	HDT = Self::letters_to_token(b"HDT"),
	
	/// TODO.
	HDU = Self::letters_to_token(b"HDU"),
	
	/// TODO.
	HDV = Self::letters_to_token(b"HDV"),
	
	/// TODO.
	HDW = Self::letters_to_token(b"HDW"),
	
	/// TODO.
	HDX = Self::letters_to_token(b"HDX"),
	
	/// TODO.
	HDY = Self::letters_to_token(b"HDY"),
	
	/// TODO.
	HDZ = Self::letters_to_token(b"HDZ"),
	
	/// TODO.
	HEA = Self::letters_to_token(b"HEA"),
	
	/// TODO.
	HEB = Self::letters_to_token(b"HEB"),
	
	/// TODO.
	HEC = Self::letters_to_token(b"HEC"),
	
	/// TODO.
	HED = Self::letters_to_token(b"HED"),
	
	/// TODO.
	HEE = Self::letters_to_token(b"HEE"),
	
	/// TODO.
	HEF = Self::letters_to_token(b"HEF"),
	
	/// TODO.
	HEG = Self::letters_to_token(b"HEG"),
	
	/// TODO.
	HEH = Self::letters_to_token(b"HEH"),
	
	/// TODO.
	HEI = Self::letters_to_token(b"HEI"),
	
	/// TODO.
	HEJ = Self::letters_to_token(b"HEJ"),
	
	/// TODO.
	HEK = Self::letters_to_token(b"HEK"),
	
	/// TODO.
	HEL = Self::letters_to_token(b"HEL"),
	
	/// TODO.
	HEM = Self::letters_to_token(b"HEM"),
	
	/// TODO.
	HEN = Self::letters_to_token(b"HEN"),
	
	/// TODO.
	HEO = Self::letters_to_token(b"HEO"),
	
	/// TODO.
	HEP = Self::letters_to_token(b"HEP"),
	
	/// TODO.
	HEQ = Self::letters_to_token(b"HEQ"),
	
	/// TODO.
	HER = Self::letters_to_token(b"HER"),
	
	/// TODO.
	HES = Self::letters_to_token(b"HES"),
	
	/// TODO.
	HET = Self::letters_to_token(b"HET"),
	
	/// TODO.
	HEU = Self::letters_to_token(b"HEU"),
	
	/// TODO.
	HEV = Self::letters_to_token(b"HEV"),
	
	/// TODO.
	HEW = Self::letters_to_token(b"HEW"),
	
	/// TODO.
	HEX = Self::letters_to_token(b"HEX"),
	
	/// TODO.
	HEY = Self::letters_to_token(b"HEY"),
	
	/// TODO.
	HEZ = Self::letters_to_token(b"HEZ"),
	
	/// TODO.
	HFA = Self::letters_to_token(b"HFA"),
	
	/// TODO.
	HFB = Self::letters_to_token(b"HFB"),
	
	/// TODO.
	HFC = Self::letters_to_token(b"HFC"),
	
	/// TODO.
	HFD = Self::letters_to_token(b"HFD"),
	
	/// TODO.
	HFE = Self::letters_to_token(b"HFE"),
	
	/// TODO.
	HFF = Self::letters_to_token(b"HFF"),
	
	/// TODO.
	HFG = Self::letters_to_token(b"HFG"),
	
	/// TODO.
	HFH = Self::letters_to_token(b"HFH"),
	
	/// TODO.
	HFI = Self::letters_to_token(b"HFI"),
	
	/// TODO.
	HFJ = Self::letters_to_token(b"HFJ"),
	
	/// TODO.
	HFK = Self::letters_to_token(b"HFK"),
	
	/// TODO.
	HFL = Self::letters_to_token(b"HFL"),
	
	/// TODO.
	HFM = Self::letters_to_token(b"HFM"),
	
	/// TODO.
	HFN = Self::letters_to_token(b"HFN"),
	
	/// TODO.
	HFO = Self::letters_to_token(b"HFO"),
	
	/// TODO.
	HFP = Self::letters_to_token(b"HFP"),
	
	/// TODO.
	HFQ = Self::letters_to_token(b"HFQ"),
	
	/// TODO.
	HFR = Self::letters_to_token(b"HFR"),
	
	/// TODO.
	HFS = Self::letters_to_token(b"HFS"),
	
	/// TODO.
	HFT = Self::letters_to_token(b"HFT"),
	
	/// TODO.
	HFU = Self::letters_to_token(b"HFU"),
	
	/// TODO.
	HFV = Self::letters_to_token(b"HFV"),
	
	/// TODO.
	HFW = Self::letters_to_token(b"HFW"),
	
	/// TODO.
	HFX = Self::letters_to_token(b"HFX"),
	
	/// TODO.
	HFY = Self::letters_to_token(b"HFY"),
	
	/// TODO.
	HFZ = Self::letters_to_token(b"HFZ"),
	
	/// TODO.
	HGA = Self::letters_to_token(b"HGA"),
	
	/// TODO.
	HGB = Self::letters_to_token(b"HGB"),
	
	/// TODO.
	HGC = Self::letters_to_token(b"HGC"),
	
	/// TODO.
	HGD = Self::letters_to_token(b"HGD"),
	
	/// TODO.
	HGE = Self::letters_to_token(b"HGE"),
	
	/// TODO.
	HGF = Self::letters_to_token(b"HGF"),
	
	/// TODO.
	HGG = Self::letters_to_token(b"HGG"),
	
	/// TODO.
	HGH = Self::letters_to_token(b"HGH"),
	
	/// TODO.
	HGI = Self::letters_to_token(b"HGI"),
	
	/// TODO.
	HGJ = Self::letters_to_token(b"HGJ"),
	
	/// TODO.
	HGK = Self::letters_to_token(b"HGK"),
	
	/// TODO.
	HGL = Self::letters_to_token(b"HGL"),
	
	/// TODO.
	HGM = Self::letters_to_token(b"HGM"),
	
	/// TODO.
	HGN = Self::letters_to_token(b"HGN"),
	
	/// TODO.
	HGO = Self::letters_to_token(b"HGO"),
	
	/// TODO.
	HGP = Self::letters_to_token(b"HGP"),
	
	/// TODO.
	HGQ = Self::letters_to_token(b"HGQ"),
	
	/// TODO.
	HGR = Self::letters_to_token(b"HGR"),
	
	/// TODO.
	HGS = Self::letters_to_token(b"HGS"),
	
	/// TODO.
	HGT = Self::letters_to_token(b"HGT"),
	
	/// TODO.
	HGU = Self::letters_to_token(b"HGU"),
	
	/// TODO.
	HGV = Self::letters_to_token(b"HGV"),
	
	/// TODO.
	HGW = Self::letters_to_token(b"HGW"),
	
	/// TODO.
	HGX = Self::letters_to_token(b"HGX"),
	
	/// TODO.
	HGY = Self::letters_to_token(b"HGY"),
	
	/// TODO.
	HGZ = Self::letters_to_token(b"HGZ"),
	
	/// TODO.
	HHA = Self::letters_to_token(b"HHA"),
	
	/// TODO.
	HHB = Self::letters_to_token(b"HHB"),
	
	/// TODO.
	HHC = Self::letters_to_token(b"HHC"),
	
	/// TODO.
	HHD = Self::letters_to_token(b"HHD"),
	
	/// TODO.
	HHE = Self::letters_to_token(b"HHE"),
	
	/// TODO.
	HHF = Self::letters_to_token(b"HHF"),
	
	/// TODO.
	HHG = Self::letters_to_token(b"HHG"),
	
	/// TODO.
	HHH = Self::letters_to_token(b"HHH"),
	
	/// TODO.
	HHI = Self::letters_to_token(b"HHI"),
	
	/// TODO.
	HHJ = Self::letters_to_token(b"HHJ"),
	
	/// TODO.
	HHK = Self::letters_to_token(b"HHK"),
	
	/// TODO.
	HHL = Self::letters_to_token(b"HHL"),
	
	/// TODO.
	HHM = Self::letters_to_token(b"HHM"),
	
	/// TODO.
	HHN = Self::letters_to_token(b"HHN"),
	
	/// TODO.
	HHO = Self::letters_to_token(b"HHO"),
	
	/// TODO.
	HHP = Self::letters_to_token(b"HHP"),
	
	/// TODO.
	HHQ = Self::letters_to_token(b"HHQ"),
	
	/// TODO.
	HHR = Self::letters_to_token(b"HHR"),
	
	/// TODO.
	HHS = Self::letters_to_token(b"HHS"),
	
	/// TODO.
	HHT = Self::letters_to_token(b"HHT"),
	
	/// TODO.
	HHU = Self::letters_to_token(b"HHU"),
	
	/// TODO.
	HHV = Self::letters_to_token(b"HHV"),
	
	/// TODO.
	HHW = Self::letters_to_token(b"HHW"),
	
	/// TODO.
	HHX = Self::letters_to_token(b"HHX"),
	
	/// TODO.
	HHY = Self::letters_to_token(b"HHY"),
	
	/// TODO.
	HHZ = Self::letters_to_token(b"HHZ"),
	
	/// TODO.
	HIA = Self::letters_to_token(b"HIA"),
	
	/// TODO.
	HIB = Self::letters_to_token(b"HIB"),
	
	/// TODO.
	HIC = Self::letters_to_token(b"HIC"),
	
	/// TODO.
	HID = Self::letters_to_token(b"HID"),
	
	/// TODO.
	HIE = Self::letters_to_token(b"HIE"),
	
	/// TODO.
	HIF = Self::letters_to_token(b"HIF"),
	
	/// TODO.
	HIG = Self::letters_to_token(b"HIG"),
	
	/// TODO.
	HIH = Self::letters_to_token(b"HIH"),
	
	/// TODO.
	HII = Self::letters_to_token(b"HII"),
	
	/// TODO.
	HIJ = Self::letters_to_token(b"HIJ"),
	
	/// TODO.
	HIK = Self::letters_to_token(b"HIK"),
	
	/// TODO.
	HIL = Self::letters_to_token(b"HIL"),
	
	/// TODO.
	HIM = Self::letters_to_token(b"HIM"),
	
	/// TODO.
	HIN = Self::letters_to_token(b"HIN"),
	
	/// TODO.
	HIO = Self::letters_to_token(b"HIO"),
	
	/// TODO.
	HIP = Self::letters_to_token(b"HIP"),
	
	/// TODO.
	HIQ = Self::letters_to_token(b"HIQ"),
	
	/// TODO.
	HIR = Self::letters_to_token(b"HIR"),
	
	/// TODO.
	HIS = Self::letters_to_token(b"HIS"),
	
	/// TODO.
	HIT = Self::letters_to_token(b"HIT"),
	
	/// TODO.
	HIU = Self::letters_to_token(b"HIU"),
	
	/// TODO.
	HIV = Self::letters_to_token(b"HIV"),
	
	/// TODO.
	HIW = Self::letters_to_token(b"HIW"),
	
	/// TODO.
	HIX = Self::letters_to_token(b"HIX"),
	
	/// TODO.
	HIY = Self::letters_to_token(b"HIY"),
	
	/// TODO.
	HIZ = Self::letters_to_token(b"HIZ"),
	
	/// TODO.
	HJA = Self::letters_to_token(b"HJA"),
	
	/// TODO.
	HJB = Self::letters_to_token(b"HJB"),
	
	/// TODO.
	HJC = Self::letters_to_token(b"HJC"),
	
	/// TODO.
	HJD = Self::letters_to_token(b"HJD"),
	
	/// TODO.
	HJE = Self::letters_to_token(b"HJE"),
	
	/// TODO.
	HJF = Self::letters_to_token(b"HJF"),
	
	/// TODO.
	HJG = Self::letters_to_token(b"HJG"),
	
	/// TODO.
	HJH = Self::letters_to_token(b"HJH"),
	
	/// TODO.
	HJI = Self::letters_to_token(b"HJI"),
	
	/// TODO.
	HJJ = Self::letters_to_token(b"HJJ"),
	
	/// TODO.
	HJK = Self::letters_to_token(b"HJK"),
	
	/// TODO.
	HJL = Self::letters_to_token(b"HJL"),
	
	/// TODO.
	HJM = Self::letters_to_token(b"HJM"),
	
	/// TODO.
	HJN = Self::letters_to_token(b"HJN"),
	
	/// TODO.
	HJO = Self::letters_to_token(b"HJO"),
	
	/// TODO.
	HJP = Self::letters_to_token(b"HJP"),
	
	/// TODO.
	HJQ = Self::letters_to_token(b"HJQ"),
	
	/// TODO.
	HJR = Self::letters_to_token(b"HJR"),
	
	/// TODO.
	HJS = Self::letters_to_token(b"HJS"),
	
	/// TODO.
	HJT = Self::letters_to_token(b"HJT"),
	
	/// TODO.
	HJU = Self::letters_to_token(b"HJU"),
	
	/// TODO.
	HJV = Self::letters_to_token(b"HJV"),
	
	/// TODO.
	HJW = Self::letters_to_token(b"HJW"),
	
	/// TODO.
	HJX = Self::letters_to_token(b"HJX"),
	
	/// TODO.
	HJY = Self::letters_to_token(b"HJY"),
	
	/// TODO.
	HJZ = Self::letters_to_token(b"HJZ"),
	
	/// TODO.
	HKA = Self::letters_to_token(b"HKA"),
	
	/// TODO.
	HKB = Self::letters_to_token(b"HKB"),
	
	/// TODO.
	HKC = Self::letters_to_token(b"HKC"),
	
	/// TODO.
	HKD = Self::letters_to_token(b"HKD"),
	
	/// TODO.
	HKE = Self::letters_to_token(b"HKE"),
	
	/// TODO.
	HKF = Self::letters_to_token(b"HKF"),
	
	/// TODO.
	HKG = Self::letters_to_token(b"HKG"),
	
	/// TODO.
	HKH = Self::letters_to_token(b"HKH"),
	
	/// TODO.
	HKI = Self::letters_to_token(b"HKI"),
	
	/// TODO.
	HKJ = Self::letters_to_token(b"HKJ"),
	
	/// TODO.
	HKK = Self::letters_to_token(b"HKK"),
	
	/// TODO.
	HKL = Self::letters_to_token(b"HKL"),
	
	/// TODO.
	HKM = Self::letters_to_token(b"HKM"),
	
	/// TODO.
	HKN = Self::letters_to_token(b"HKN"),
	
	/// TODO.
	HKO = Self::letters_to_token(b"HKO"),
	
	/// TODO.
	HKP = Self::letters_to_token(b"HKP"),
	
	/// TODO.
	HKQ = Self::letters_to_token(b"HKQ"),
	
	/// TODO.
	HKR = Self::letters_to_token(b"HKR"),
	
	/// TODO.
	HKS = Self::letters_to_token(b"HKS"),
	
	/// TODO.
	HKT = Self::letters_to_token(b"HKT"),
	
	/// TODO.
	HKU = Self::letters_to_token(b"HKU"),
	
	/// TODO.
	HKV = Self::letters_to_token(b"HKV"),
	
	/// TODO.
	HKW = Self::letters_to_token(b"HKW"),
	
	/// TODO.
	HKX = Self::letters_to_token(b"HKX"),
	
	/// TODO.
	HKY = Self::letters_to_token(b"HKY"),
	
	/// TODO.
	HKZ = Self::letters_to_token(b"HKZ"),
	
	/// TODO.
	HLA = Self::letters_to_token(b"HLA"),
	
	/// TODO.
	HLB = Self::letters_to_token(b"HLB"),
	
	/// TODO.
	HLC = Self::letters_to_token(b"HLC"),
	
	/// TODO.
	HLD = Self::letters_to_token(b"HLD"),
	
	/// TODO.
	HLE = Self::letters_to_token(b"HLE"),
	
	/// TODO.
	HLF = Self::letters_to_token(b"HLF"),
	
	/// TODO.
	HLG = Self::letters_to_token(b"HLG"),
	
	/// TODO.
	HLH = Self::letters_to_token(b"HLH"),
	
	/// TODO.
	HLI = Self::letters_to_token(b"HLI"),
	
	/// TODO.
	HLJ = Self::letters_to_token(b"HLJ"),
	
	/// TODO.
	HLK = Self::letters_to_token(b"HLK"),
	
	/// TODO.
	HLL = Self::letters_to_token(b"HLL"),
	
	/// TODO.
	HLM = Self::letters_to_token(b"HLM"),
	
	/// TODO.
	HLN = Self::letters_to_token(b"HLN"),
	
	/// TODO.
	HLO = Self::letters_to_token(b"HLO"),
	
	/// TODO.
	HLP = Self::letters_to_token(b"HLP"),
	
	/// TODO.
	HLQ = Self::letters_to_token(b"HLQ"),
	
	/// TODO.
	HLR = Self::letters_to_token(b"HLR"),
	
	/// TODO.
	HLS = Self::letters_to_token(b"HLS"),
	
	/// TODO.
	HLT = Self::letters_to_token(b"HLT"),
	
	/// TODO.
	HLU = Self::letters_to_token(b"HLU"),
	
	/// TODO.
	HLV = Self::letters_to_token(b"HLV"),
	
	/// TODO.
	HLW = Self::letters_to_token(b"HLW"),
	
	/// TODO.
	HLX = Self::letters_to_token(b"HLX"),
	
	/// TODO.
	HLY = Self::letters_to_token(b"HLY"),
	
	/// TODO.
	HLZ = Self::letters_to_token(b"HLZ"),
	
	/// TODO.
	HMA = Self::letters_to_token(b"HMA"),
	
	/// TODO.
	HMB = Self::letters_to_token(b"HMB"),
	
	/// TODO.
	HMC = Self::letters_to_token(b"HMC"),
	
	/// TODO.
	HMD = Self::letters_to_token(b"HMD"),
	
	/// TODO.
	HME = Self::letters_to_token(b"HME"),
	
	/// TODO.
	HMF = Self::letters_to_token(b"HMF"),
	
	/// TODO.
	HMG = Self::letters_to_token(b"HMG"),
	
	/// TODO.
	HMH = Self::letters_to_token(b"HMH"),
	
	/// TODO.
	HMI = Self::letters_to_token(b"HMI"),
	
	/// TODO.
	HMJ = Self::letters_to_token(b"HMJ"),
	
	/// TODO.
	HMK = Self::letters_to_token(b"HMK"),
	
	/// TODO.
	HML = Self::letters_to_token(b"HML"),
	
	/// TODO.
	HMM = Self::letters_to_token(b"HMM"),
	
	/// TODO.
	HMN = Self::letters_to_token(b"HMN"),
	
	/// TODO.
	HMO = Self::letters_to_token(b"HMO"),
	
	/// TODO.
	HMP = Self::letters_to_token(b"HMP"),
	
	/// TODO.
	HMQ = Self::letters_to_token(b"HMQ"),
	
	/// TODO.
	HMR = Self::letters_to_token(b"HMR"),
	
	/// TODO.
	HMS = Self::letters_to_token(b"HMS"),
	
	/// TODO.
	HMT = Self::letters_to_token(b"HMT"),
	
	/// TODO.
	HMU = Self::letters_to_token(b"HMU"),
	
	/// TODO.
	HMV = Self::letters_to_token(b"HMV"),
	
	/// TODO.
	HMW = Self::letters_to_token(b"HMW"),
	
	/// TODO.
	HMX = Self::letters_to_token(b"HMX"),
	
	/// TODO.
	HMY = Self::letters_to_token(b"HMY"),
	
	/// TODO.
	HMZ = Self::letters_to_token(b"HMZ"),
	
	/// TODO.
	HNA = Self::letters_to_token(b"HNA"),
	
	/// TODO.
	HNB = Self::letters_to_token(b"HNB"),
	
	/// TODO.
	HNC = Self::letters_to_token(b"HNC"),
	
	/// TODO.
	HND = Self::letters_to_token(b"HND"),
	
	/// TODO.
	HNE = Self::letters_to_token(b"HNE"),
	
	/// TODO.
	HNF = Self::letters_to_token(b"HNF"),
	
	/// TODO.
	HNG = Self::letters_to_token(b"HNG"),
	
	/// TODO.
	HNH = Self::letters_to_token(b"HNH"),
	
	/// TODO.
	HNI = Self::letters_to_token(b"HNI"),
	
	/// TODO.
	HNJ = Self::letters_to_token(b"HNJ"),
	
	/// TODO.
	HNK = Self::letters_to_token(b"HNK"),
	
	/// TODO.
	HNL = Self::letters_to_token(b"HNL"),
	
	/// TODO.
	HNM = Self::letters_to_token(b"HNM"),
	
	/// TODO.
	HNN = Self::letters_to_token(b"HNN"),
	
	/// TODO.
	HNO = Self::letters_to_token(b"HNO"),
	
	/// TODO.
	HNP = Self::letters_to_token(b"HNP"),
	
	/// TODO.
	HNQ = Self::letters_to_token(b"HNQ"),
	
	/// TODO.
	HNR = Self::letters_to_token(b"HNR"),
	
	/// TODO.
	HNS = Self::letters_to_token(b"HNS"),
	
	/// TODO.
	HNT = Self::letters_to_token(b"HNT"),
	
	/// TODO.
	HNU = Self::letters_to_token(b"HNU"),
	
	/// TODO.
	HNV = Self::letters_to_token(b"HNV"),
	
	/// TODO.
	HNW = Self::letters_to_token(b"HNW"),
	
	/// TODO.
	HNX = Self::letters_to_token(b"HNX"),
	
	/// TODO.
	HNY = Self::letters_to_token(b"HNY"),
	
	/// TODO.
	HNZ = Self::letters_to_token(b"HNZ"),
	
	/// TODO.
	HOA = Self::letters_to_token(b"HOA"),
	
	/// TODO.
	HOB = Self::letters_to_token(b"HOB"),
	
	/// TODO.
	HOC = Self::letters_to_token(b"HOC"),
	
	/// TODO.
	HOD = Self::letters_to_token(b"HOD"),
	
	/// TODO.
	HOE = Self::letters_to_token(b"HOE"),
	
	/// TODO.
	HOF = Self::letters_to_token(b"HOF"),
	
	/// TODO.
	HOG = Self::letters_to_token(b"HOG"),
	
	/// TODO.
	HOH = Self::letters_to_token(b"HOH"),
	
	/// TODO.
	HOI = Self::letters_to_token(b"HOI"),
	
	/// TODO.
	HOJ = Self::letters_to_token(b"HOJ"),
	
	/// TODO.
	HOK = Self::letters_to_token(b"HOK"),
	
	/// TODO.
	HOL = Self::letters_to_token(b"HOL"),
	
	/// TODO.
	HOM = Self::letters_to_token(b"HOM"),
	
	/// TODO.
	HON = Self::letters_to_token(b"HON"),
	
	/// TODO.
	HOO = Self::letters_to_token(b"HOO"),
	
	/// TODO.
	HOP = Self::letters_to_token(b"HOP"),
	
	/// TODO.
	HOQ = Self::letters_to_token(b"HOQ"),
	
	/// TODO.
	HOR = Self::letters_to_token(b"HOR"),
	
	/// TODO.
	HOS = Self::letters_to_token(b"HOS"),
	
	/// TODO.
	HOT = Self::letters_to_token(b"HOT"),
	
	/// TODO.
	HOU = Self::letters_to_token(b"HOU"),
	
	/// TODO.
	HOV = Self::letters_to_token(b"HOV"),
	
	/// TODO.
	HOW = Self::letters_to_token(b"HOW"),
	
	/// TODO.
	HOX = Self::letters_to_token(b"HOX"),
	
	/// TODO.
	HOY = Self::letters_to_token(b"HOY"),
	
	/// TODO.
	HOZ = Self::letters_to_token(b"HOZ"),
	
	/// TODO.
	HPA = Self::letters_to_token(b"HPA"),
	
	/// TODO.
	HPB = Self::letters_to_token(b"HPB"),
	
	/// TODO.
	HPC = Self::letters_to_token(b"HPC"),
	
	/// TODO.
	HPD = Self::letters_to_token(b"HPD"),
	
	/// TODO.
	HPE = Self::letters_to_token(b"HPE"),
	
	/// TODO.
	HPF = Self::letters_to_token(b"HPF"),
	
	/// TODO.
	HPG = Self::letters_to_token(b"HPG"),
	
	/// TODO.
	HPH = Self::letters_to_token(b"HPH"),
	
	/// TODO.
	HPI = Self::letters_to_token(b"HPI"),
	
	/// TODO.
	HPJ = Self::letters_to_token(b"HPJ"),
	
	/// TODO.
	HPK = Self::letters_to_token(b"HPK"),
	
	/// TODO.
	HPL = Self::letters_to_token(b"HPL"),
	
	/// TODO.
	HPM = Self::letters_to_token(b"HPM"),
	
	/// TODO.
	HPN = Self::letters_to_token(b"HPN"),
	
	/// TODO.
	HPO = Self::letters_to_token(b"HPO"),
	
	/// TODO.
	HPP = Self::letters_to_token(b"HPP"),
	
	/// TODO.
	HPQ = Self::letters_to_token(b"HPQ"),
	
	/// TODO.
	HPR = Self::letters_to_token(b"HPR"),
	
	/// TODO.
	HPS = Self::letters_to_token(b"HPS"),
	
	/// TODO.
	HPT = Self::letters_to_token(b"HPT"),
	
	/// TODO.
	HPU = Self::letters_to_token(b"HPU"),
	
	/// TODO.
	HPV = Self::letters_to_token(b"HPV"),
	
	/// TODO.
	HPW = Self::letters_to_token(b"HPW"),
	
	/// TODO.
	HPX = Self::letters_to_token(b"HPX"),
	
	/// TODO.
	HPY = Self::letters_to_token(b"HPY"),
	
	/// TODO.
	HPZ = Self::letters_to_token(b"HPZ"),
	
	/// TODO.
	HQA = Self::letters_to_token(b"HQA"),
	
	/// TODO.
	HQB = Self::letters_to_token(b"HQB"),
	
	/// TODO.
	HQC = Self::letters_to_token(b"HQC"),
	
	/// TODO.
	HQD = Self::letters_to_token(b"HQD"),
	
	/// TODO.
	HQE = Self::letters_to_token(b"HQE"),
	
	/// TODO.
	HQF = Self::letters_to_token(b"HQF"),
	
	/// TODO.
	HQG = Self::letters_to_token(b"HQG"),
	
	/// TODO.
	HQH = Self::letters_to_token(b"HQH"),
	
	/// TODO.
	HQI = Self::letters_to_token(b"HQI"),
	
	/// TODO.
	HQJ = Self::letters_to_token(b"HQJ"),
	
	/// TODO.
	HQK = Self::letters_to_token(b"HQK"),
	
	/// TODO.
	HQL = Self::letters_to_token(b"HQL"),
	
	/// TODO.
	HQM = Self::letters_to_token(b"HQM"),
	
	/// TODO.
	HQN = Self::letters_to_token(b"HQN"),
	
	/// TODO.
	HQO = Self::letters_to_token(b"HQO"),
	
	/// TODO.
	HQP = Self::letters_to_token(b"HQP"),
	
	/// TODO.
	HQQ = Self::letters_to_token(b"HQQ"),
	
	/// TODO.
	HQR = Self::letters_to_token(b"HQR"),
	
	/// TODO.
	HQS = Self::letters_to_token(b"HQS"),
	
	/// TODO.
	HQT = Self::letters_to_token(b"HQT"),
	
	/// TODO.
	HQU = Self::letters_to_token(b"HQU"),
	
	/// TODO.
	HQV = Self::letters_to_token(b"HQV"),
	
	/// TODO.
	HQW = Self::letters_to_token(b"HQW"),
	
	/// TODO.
	HQX = Self::letters_to_token(b"HQX"),
	
	/// TODO.
	HQY = Self::letters_to_token(b"HQY"),
	
	/// TODO.
	HQZ = Self::letters_to_token(b"HQZ"),
	
	/// TODO.
	HRA = Self::letters_to_token(b"HRA"),
	
	/// TODO.
	HRB = Self::letters_to_token(b"HRB"),
	
	/// TODO.
	HRC = Self::letters_to_token(b"HRC"),
	
	/// TODO.
	HRD = Self::letters_to_token(b"HRD"),
	
	/// TODO.
	HRE = Self::letters_to_token(b"HRE"),
	
	/// TODO.
	HRF = Self::letters_to_token(b"HRF"),
	
	/// TODO.
	HRG = Self::letters_to_token(b"HRG"),
	
	/// TODO.
	HRH = Self::letters_to_token(b"HRH"),
	
	/// TODO.
	HRI = Self::letters_to_token(b"HRI"),
	
	/// TODO.
	HRJ = Self::letters_to_token(b"HRJ"),
	
	/// TODO.
	HRK = Self::letters_to_token(b"HRK"),
	
	/// TODO.
	HRL = Self::letters_to_token(b"HRL"),
	
	/// TODO.
	HRM = Self::letters_to_token(b"HRM"),
	
	/// TODO.
	HRN = Self::letters_to_token(b"HRN"),
	
	/// TODO.
	HRO = Self::letters_to_token(b"HRO"),
	
	/// TODO.
	HRP = Self::letters_to_token(b"HRP"),
	
	/// TODO.
	HRQ = Self::letters_to_token(b"HRQ"),
	
	/// TODO.
	HRR = Self::letters_to_token(b"HRR"),
	
	/// TODO.
	HRS = Self::letters_to_token(b"HRS"),
	
	/// TODO.
	HRT = Self::letters_to_token(b"HRT"),
	
	/// TODO.
	HRU = Self::letters_to_token(b"HRU"),
	
	/// TODO.
	HRV = Self::letters_to_token(b"HRV"),
	
	/// TODO.
	HRW = Self::letters_to_token(b"HRW"),
	
	/// TODO.
	HRX = Self::letters_to_token(b"HRX"),
	
	/// TODO.
	HRY = Self::letters_to_token(b"HRY"),
	
	/// TODO.
	HRZ = Self::letters_to_token(b"HRZ"),
	
	/// TODO.
	HSA = Self::letters_to_token(b"HSA"),
	
	/// TODO.
	HSB = Self::letters_to_token(b"HSB"),
	
	/// TODO.
	HSC = Self::letters_to_token(b"HSC"),
	
	/// TODO.
	HSD = Self::letters_to_token(b"HSD"),
	
	/// TODO.
	HSE = Self::letters_to_token(b"HSE"),
	
	/// TODO.
	HSF = Self::letters_to_token(b"HSF"),
	
	/// TODO.
	HSG = Self::letters_to_token(b"HSG"),
	
	/// TODO.
	HSH = Self::letters_to_token(b"HSH"),
	
	/// TODO.
	HSI = Self::letters_to_token(b"HSI"),
	
	/// TODO.
	HSJ = Self::letters_to_token(b"HSJ"),
	
	/// TODO.
	HSK = Self::letters_to_token(b"HSK"),
	
	/// TODO.
	HSL = Self::letters_to_token(b"HSL"),
	
	/// TODO.
	HSM = Self::letters_to_token(b"HSM"),
	
	/// TODO.
	HSN = Self::letters_to_token(b"HSN"),
	
	/// TODO.
	HSO = Self::letters_to_token(b"HSO"),
	
	/// TODO.
	HSP = Self::letters_to_token(b"HSP"),
	
	/// TODO.
	HSQ = Self::letters_to_token(b"HSQ"),
	
	/// TODO.
	HSR = Self::letters_to_token(b"HSR"),
	
	/// TODO.
	HSS = Self::letters_to_token(b"HSS"),
	
	/// TODO.
	HST = Self::letters_to_token(b"HST"),
	
	/// TODO.
	HSU = Self::letters_to_token(b"HSU"),
	
	/// TODO.
	HSV = Self::letters_to_token(b"HSV"),
	
	/// TODO.
	HSW = Self::letters_to_token(b"HSW"),
	
	/// TODO.
	HSX = Self::letters_to_token(b"HSX"),
	
	/// TODO.
	HSY = Self::letters_to_token(b"HSY"),
	
	/// TODO.
	HSZ = Self::letters_to_token(b"HSZ"),
	
	/// TODO.
	HTA = Self::letters_to_token(b"HTA"),
	
	/// TODO.
	HTB = Self::letters_to_token(b"HTB"),
	
	/// TODO.
	HTC = Self::letters_to_token(b"HTC"),
	
	/// TODO.
	HTD = Self::letters_to_token(b"HTD"),
	
	/// TODO.
	HTE = Self::letters_to_token(b"HTE"),
	
	/// TODO.
	HTF = Self::letters_to_token(b"HTF"),
	
	/// TODO.
	HTG = Self::letters_to_token(b"HTG"),
	
	/// TODO.
	HTH = Self::letters_to_token(b"HTH"),
	
	/// TODO.
	HTI = Self::letters_to_token(b"HTI"),
	
	/// TODO.
	HTJ = Self::letters_to_token(b"HTJ"),
	
	/// TODO.
	HTK = Self::letters_to_token(b"HTK"),
	
	/// TODO.
	HTL = Self::letters_to_token(b"HTL"),
	
	/// TODO.
	HTM = Self::letters_to_token(b"HTM"),
	
	/// TODO.
	HTN = Self::letters_to_token(b"HTN"),
	
	/// TODO.
	HTO = Self::letters_to_token(b"HTO"),
	
	/// TODO.
	HTP = Self::letters_to_token(b"HTP"),
	
	/// TODO.
	HTQ = Self::letters_to_token(b"HTQ"),
	
	/// TODO.
	HTR = Self::letters_to_token(b"HTR"),
	
	/// TODO.
	HTS = Self::letters_to_token(b"HTS"),
	
	/// TODO.
	HTT = Self::letters_to_token(b"HTT"),
	
	/// TODO.
	HTU = Self::letters_to_token(b"HTU"),
	
	/// TODO.
	HTV = Self::letters_to_token(b"HTV"),
	
	/// TODO.
	HTW = Self::letters_to_token(b"HTW"),
	
	/// TODO.
	HTX = Self::letters_to_token(b"HTX"),
	
	/// TODO.
	HTY = Self::letters_to_token(b"HTY"),
	
	/// TODO.
	HTZ = Self::letters_to_token(b"HTZ"),
	
	/// TODO.
	HUA = Self::letters_to_token(b"HUA"),
	
	/// TODO.
	HUB = Self::letters_to_token(b"HUB"),
	
	/// TODO.
	HUC = Self::letters_to_token(b"HUC"),
	
	/// TODO.
	HUD = Self::letters_to_token(b"HUD"),
	
	/// TODO.
	HUE = Self::letters_to_token(b"HUE"),
	
	/// TODO.
	HUF = Self::letters_to_token(b"HUF"),
	
	/// TODO.
	HUG = Self::letters_to_token(b"HUG"),
	
	/// TODO.
	HUH = Self::letters_to_token(b"HUH"),
	
	/// TODO.
	HUI = Self::letters_to_token(b"HUI"),
	
	/// TODO.
	HUJ = Self::letters_to_token(b"HUJ"),
	
	/// TODO.
	HUK = Self::letters_to_token(b"HUK"),
	
	/// TODO.
	HUL = Self::letters_to_token(b"HUL"),
	
	/// TODO.
	HUM = Self::letters_to_token(b"HUM"),
	
	/// TODO.
	HUN = Self::letters_to_token(b"HUN"),
	
	/// TODO.
	HUO = Self::letters_to_token(b"HUO"),
	
	/// TODO.
	HUP = Self::letters_to_token(b"HUP"),
	
	/// TODO.
	HUQ = Self::letters_to_token(b"HUQ"),
	
	/// TODO.
	HUR = Self::letters_to_token(b"HUR"),
	
	/// TODO.
	HUS = Self::letters_to_token(b"HUS"),
	
	/// TODO.
	HUT = Self::letters_to_token(b"HUT"),
	
	/// TODO.
	HUU = Self::letters_to_token(b"HUU"),
	
	/// TODO.
	HUV = Self::letters_to_token(b"HUV"),
	
	/// TODO.
	HUW = Self::letters_to_token(b"HUW"),
	
	/// TODO.
	HUX = Self::letters_to_token(b"HUX"),
	
	/// TODO.
	HUY = Self::letters_to_token(b"HUY"),
	
	/// TODO.
	HUZ = Self::letters_to_token(b"HUZ"),
	
	/// TODO.
	HVA = Self::letters_to_token(b"HVA"),
	
	/// TODO.
	HVB = Self::letters_to_token(b"HVB"),
	
	/// TODO.
	HVC = Self::letters_to_token(b"HVC"),
	
	/// TODO.
	HVD = Self::letters_to_token(b"HVD"),
	
	/// TODO.
	HVE = Self::letters_to_token(b"HVE"),
	
	/// TODO.
	HVF = Self::letters_to_token(b"HVF"),
	
	/// TODO.
	HVG = Self::letters_to_token(b"HVG"),
	
	/// TODO.
	HVH = Self::letters_to_token(b"HVH"),
	
	/// TODO.
	HVI = Self::letters_to_token(b"HVI"),
	
	/// TODO.
	HVJ = Self::letters_to_token(b"HVJ"),
	
	/// TODO.
	HVK = Self::letters_to_token(b"HVK"),
	
	/// TODO.
	HVL = Self::letters_to_token(b"HVL"),
	
	/// TODO.
	HVM = Self::letters_to_token(b"HVM"),
	
	/// TODO.
	HVN = Self::letters_to_token(b"HVN"),
	
	/// TODO.
	HVO = Self::letters_to_token(b"HVO"),
	
	/// TODO.
	HVP = Self::letters_to_token(b"HVP"),
	
	/// TODO.
	HVQ = Self::letters_to_token(b"HVQ"),
	
	/// TODO.
	HVR = Self::letters_to_token(b"HVR"),
	
	/// TODO.
	HVS = Self::letters_to_token(b"HVS"),
	
	/// TODO.
	HVT = Self::letters_to_token(b"HVT"),
	
	/// TODO.
	HVU = Self::letters_to_token(b"HVU"),
	
	/// TODO.
	HVV = Self::letters_to_token(b"HVV"),
	
	/// TODO.
	HVW = Self::letters_to_token(b"HVW"),
	
	/// TODO.
	HVX = Self::letters_to_token(b"HVX"),
	
	/// TODO.
	HVY = Self::letters_to_token(b"HVY"),
	
	/// TODO.
	HVZ = Self::letters_to_token(b"HVZ"),
	
	/// TODO.
	HWA = Self::letters_to_token(b"HWA"),
	
	/// TODO.
	HWB = Self::letters_to_token(b"HWB"),
	
	/// TODO.
	HWC = Self::letters_to_token(b"HWC"),
	
	/// TODO.
	HWD = Self::letters_to_token(b"HWD"),
	
	/// TODO.
	HWE = Self::letters_to_token(b"HWE"),
	
	/// TODO.
	HWF = Self::letters_to_token(b"HWF"),
	
	/// TODO.
	HWG = Self::letters_to_token(b"HWG"),
	
	/// TODO.
	HWH = Self::letters_to_token(b"HWH"),
	
	/// TODO.
	HWI = Self::letters_to_token(b"HWI"),
	
	/// TODO.
	HWJ = Self::letters_to_token(b"HWJ"),
	
	/// TODO.
	HWK = Self::letters_to_token(b"HWK"),
	
	/// TODO.
	HWL = Self::letters_to_token(b"HWL"),
	
	/// TODO.
	HWM = Self::letters_to_token(b"HWM"),
	
	/// TODO.
	HWN = Self::letters_to_token(b"HWN"),
	
	/// TODO.
	HWO = Self::letters_to_token(b"HWO"),
	
	/// TODO.
	HWP = Self::letters_to_token(b"HWP"),
	
	/// TODO.
	HWQ = Self::letters_to_token(b"HWQ"),
	
	/// TODO.
	HWR = Self::letters_to_token(b"HWR"),
	
	/// TODO.
	HWS = Self::letters_to_token(b"HWS"),
	
	/// TODO.
	HWT = Self::letters_to_token(b"HWT"),
	
	/// TODO.
	HWU = Self::letters_to_token(b"HWU"),
	
	/// TODO.
	HWV = Self::letters_to_token(b"HWV"),
	
	/// TODO.
	HWW = Self::letters_to_token(b"HWW"),
	
	/// TODO.
	HWX = Self::letters_to_token(b"HWX"),
	
	/// TODO.
	HWY = Self::letters_to_token(b"HWY"),
	
	/// TODO.
	HWZ = Self::letters_to_token(b"HWZ"),
	
	/// TODO.
	HXA = Self::letters_to_token(b"HXA"),
	
	/// TODO.
	HXB = Self::letters_to_token(b"HXB"),
	
	/// TODO.
	HXC = Self::letters_to_token(b"HXC"),
	
	/// TODO.
	HXD = Self::letters_to_token(b"HXD"),
	
	/// TODO.
	HXE = Self::letters_to_token(b"HXE"),
	
	/// TODO.
	HXF = Self::letters_to_token(b"HXF"),
	
	/// TODO.
	HXG = Self::letters_to_token(b"HXG"),
	
	/// TODO.
	HXH = Self::letters_to_token(b"HXH"),
	
	/// TODO.
	HXI = Self::letters_to_token(b"HXI"),
	
	/// TODO.
	HXJ = Self::letters_to_token(b"HXJ"),
	
	/// TODO.
	HXK = Self::letters_to_token(b"HXK"),
	
	/// TODO.
	HXL = Self::letters_to_token(b"HXL"),
	
	/// TODO.
	HXM = Self::letters_to_token(b"HXM"),
	
	/// TODO.
	HXN = Self::letters_to_token(b"HXN"),
	
	/// TODO.
	HXO = Self::letters_to_token(b"HXO"),
	
	/// TODO.
	HXP = Self::letters_to_token(b"HXP"),
	
	/// TODO.
	HXQ = Self::letters_to_token(b"HXQ"),
	
	/// TODO.
	HXR = Self::letters_to_token(b"HXR"),
	
	/// TODO.
	HXS = Self::letters_to_token(b"HXS"),
	
	/// TODO.
	HXT = Self::letters_to_token(b"HXT"),
	
	/// TODO.
	HXU = Self::letters_to_token(b"HXU"),
	
	/// TODO.
	HXV = Self::letters_to_token(b"HXV"),
	
	/// TODO.
	HXW = Self::letters_to_token(b"HXW"),
	
	/// TODO.
	HXX = Self::letters_to_token(b"HXX"),
	
	/// TODO.
	HXY = Self::letters_to_token(b"HXY"),
	
	/// TODO.
	HXZ = Self::letters_to_token(b"HXZ"),
	
	/// TODO.
	HYA = Self::letters_to_token(b"HYA"),
	
	/// TODO.
	HYB = Self::letters_to_token(b"HYB"),
	
	/// TODO.
	HYC = Self::letters_to_token(b"HYC"),
	
	/// TODO.
	HYD = Self::letters_to_token(b"HYD"),
	
	/// TODO.
	HYE = Self::letters_to_token(b"HYE"),
	
	/// TODO.
	HYF = Self::letters_to_token(b"HYF"),
	
	/// TODO.
	HYG = Self::letters_to_token(b"HYG"),
	
	/// TODO.
	HYH = Self::letters_to_token(b"HYH"),
	
	/// TODO.
	HYI = Self::letters_to_token(b"HYI"),
	
	/// TODO.
	HYJ = Self::letters_to_token(b"HYJ"),
	
	/// TODO.
	HYK = Self::letters_to_token(b"HYK"),
	
	/// TODO.
	HYL = Self::letters_to_token(b"HYL"),
	
	/// TODO.
	HYM = Self::letters_to_token(b"HYM"),
	
	/// TODO.
	HYN = Self::letters_to_token(b"HYN"),
	
	/// TODO.
	HYO = Self::letters_to_token(b"HYO"),
	
	/// TODO.
	HYP = Self::letters_to_token(b"HYP"),
	
	/// TODO.
	HYQ = Self::letters_to_token(b"HYQ"),
	
	/// TODO.
	HYR = Self::letters_to_token(b"HYR"),
	
	/// TODO.
	HYS = Self::letters_to_token(b"HYS"),
	
	/// TODO.
	HYT = Self::letters_to_token(b"HYT"),
	
	/// TODO.
	HYU = Self::letters_to_token(b"HYU"),
	
	/// TODO.
	HYV = Self::letters_to_token(b"HYV"),
	
	/// TODO.
	HYW = Self::letters_to_token(b"HYW"),
	
	/// TODO.
	HYX = Self::letters_to_token(b"HYX"),
	
	/// TODO.
	HYY = Self::letters_to_token(b"HYY"),
	
	/// TODO.
	HYZ = Self::letters_to_token(b"HYZ"),
	
	/// TODO.
	HZA = Self::letters_to_token(b"HZA"),
	
	/// TODO.
	HZB = Self::letters_to_token(b"HZB"),
	
	/// TODO.
	HZC = Self::letters_to_token(b"HZC"),
	
	/// TODO.
	HZD = Self::letters_to_token(b"HZD"),
	
	/// TODO.
	HZE = Self::letters_to_token(b"HZE"),
	
	/// TODO.
	HZF = Self::letters_to_token(b"HZF"),
	
	/// TODO.
	HZG = Self::letters_to_token(b"HZG"),
	
	/// TODO.
	HZH = Self::letters_to_token(b"HZH"),
	
	/// TODO.
	HZI = Self::letters_to_token(b"HZI"),
	
	/// TODO.
	HZJ = Self::letters_to_token(b"HZJ"),
	
	/// TODO.
	HZK = Self::letters_to_token(b"HZK"),
	
	/// TODO.
	HZL = Self::letters_to_token(b"HZL"),
	
	/// TODO.
	HZM = Self::letters_to_token(b"HZM"),
	
	/// TODO.
	HZN = Self::letters_to_token(b"HZN"),
	
	/// TODO.
	HZO = Self::letters_to_token(b"HZO"),
	
	/// TODO.
	HZP = Self::letters_to_token(b"HZP"),
	
	/// TODO.
	HZQ = Self::letters_to_token(b"HZQ"),
	
	/// TODO.
	HZR = Self::letters_to_token(b"HZR"),
	
	/// TODO.
	HZS = Self::letters_to_token(b"HZS"),
	
	/// TODO.
	HZT = Self::letters_to_token(b"HZT"),
	
	/// TODO.
	HZU = Self::letters_to_token(b"HZU"),
	
	/// TODO.
	HZV = Self::letters_to_token(b"HZV"),
	
	/// TODO.
	HZW = Self::letters_to_token(b"HZW"),
	
	/// TODO.
	HZX = Self::letters_to_token(b"HZX"),
	
	/// TODO.
	HZY = Self::letters_to_token(b"HZY"),
	
	/// TODO.
	HZZ = Self::letters_to_token(b"HZZ"),
	
	/// TODO.
	IAA = Self::letters_to_token(b"IAA"),
	
	/// TODO.
	IAB = Self::letters_to_token(b"IAB"),
	
	/// TODO.
	IAC = Self::letters_to_token(b"IAC"),
	
	/// TODO.
	IAD = Self::letters_to_token(b"IAD"),
	
	/// TODO.
	IAE = Self::letters_to_token(b"IAE"),
	
	/// TODO.
	IAF = Self::letters_to_token(b"IAF"),
	
	/// TODO.
	IAG = Self::letters_to_token(b"IAG"),
	
	/// TODO.
	IAH = Self::letters_to_token(b"IAH"),
	
	/// TODO.
	IAI = Self::letters_to_token(b"IAI"),
	
	/// TODO.
	IAJ = Self::letters_to_token(b"IAJ"),
	
	/// TODO.
	IAK = Self::letters_to_token(b"IAK"),
	
	/// TODO.
	IAL = Self::letters_to_token(b"IAL"),
	
	/// TODO.
	IAM = Self::letters_to_token(b"IAM"),
	
	/// TODO.
	IAN = Self::letters_to_token(b"IAN"),
	
	/// TODO.
	IAO = Self::letters_to_token(b"IAO"),
	
	/// TODO.
	IAP = Self::letters_to_token(b"IAP"),
	
	/// TODO.
	IAQ = Self::letters_to_token(b"IAQ"),
	
	/// TODO.
	IAR = Self::letters_to_token(b"IAR"),
	
	/// TODO.
	IAS = Self::letters_to_token(b"IAS"),
	
	/// TODO.
	IAT = Self::letters_to_token(b"IAT"),
	
	/// TODO.
	IAU = Self::letters_to_token(b"IAU"),
	
	/// TODO.
	IAV = Self::letters_to_token(b"IAV"),
	
	/// TODO.
	IAW = Self::letters_to_token(b"IAW"),
	
	/// TODO.
	IAX = Self::letters_to_token(b"IAX"),
	
	/// TODO.
	IAY = Self::letters_to_token(b"IAY"),
	
	/// TODO.
	IAZ = Self::letters_to_token(b"IAZ"),
	
	/// TODO.
	IBA = Self::letters_to_token(b"IBA"),
	
	/// TODO.
	IBB = Self::letters_to_token(b"IBB"),
	
	/// TODO.
	IBC = Self::letters_to_token(b"IBC"),
	
	/// TODO.
	IBD = Self::letters_to_token(b"IBD"),
	
	/// TODO.
	IBE = Self::letters_to_token(b"IBE"),
	
	/// TODO.
	IBF = Self::letters_to_token(b"IBF"),
	
	/// TODO.
	IBG = Self::letters_to_token(b"IBG"),
	
	/// TODO.
	IBH = Self::letters_to_token(b"IBH"),
	
	/// TODO.
	IBI = Self::letters_to_token(b"IBI"),
	
	/// TODO.
	IBJ = Self::letters_to_token(b"IBJ"),
	
	/// TODO.
	IBK = Self::letters_to_token(b"IBK"),
	
	/// TODO.
	IBL = Self::letters_to_token(b"IBL"),
	
	/// TODO.
	IBM = Self::letters_to_token(b"IBM"),
	
	/// TODO.
	IBN = Self::letters_to_token(b"IBN"),
	
	/// TODO.
	IBO = Self::letters_to_token(b"IBO"),
	
	/// TODO.
	IBP = Self::letters_to_token(b"IBP"),
	
	/// TODO.
	IBQ = Self::letters_to_token(b"IBQ"),
	
	/// TODO.
	IBR = Self::letters_to_token(b"IBR"),
	
	/// TODO.
	IBS = Self::letters_to_token(b"IBS"),
	
	/// TODO.
	IBT = Self::letters_to_token(b"IBT"),
	
	/// TODO.
	IBU = Self::letters_to_token(b"IBU"),
	
	/// TODO.
	IBV = Self::letters_to_token(b"IBV"),
	
	/// TODO.
	IBW = Self::letters_to_token(b"IBW"),
	
	/// TODO.
	IBX = Self::letters_to_token(b"IBX"),
	
	/// TODO.
	IBY = Self::letters_to_token(b"IBY"),
	
	/// TODO.
	IBZ = Self::letters_to_token(b"IBZ"),
	
	/// TODO.
	ICA = Self::letters_to_token(b"ICA"),
	
	/// TODO.
	ICB = Self::letters_to_token(b"ICB"),
	
	/// TODO.
	ICC = Self::letters_to_token(b"ICC"),
	
	/// TODO.
	ICD = Self::letters_to_token(b"ICD"),
	
	/// TODO.
	ICE = Self::letters_to_token(b"ICE"),
	
	/// TODO.
	ICF = Self::letters_to_token(b"ICF"),
	
	/// TODO.
	ICG = Self::letters_to_token(b"ICG"),
	
	/// TODO.
	ICH = Self::letters_to_token(b"ICH"),
	
	/// TODO.
	ICI = Self::letters_to_token(b"ICI"),
	
	/// TODO.
	ICJ = Self::letters_to_token(b"ICJ"),
	
	/// TODO.
	ICK = Self::letters_to_token(b"ICK"),
	
	/// TODO.
	ICL = Self::letters_to_token(b"ICL"),
	
	/// TODO.
	ICM = Self::letters_to_token(b"ICM"),
	
	/// TODO.
	ICN = Self::letters_to_token(b"ICN"),
	
	/// TODO.
	ICO = Self::letters_to_token(b"ICO"),
	
	/// TODO.
	ICP = Self::letters_to_token(b"ICP"),
	
	/// TODO.
	ICQ = Self::letters_to_token(b"ICQ"),
	
	/// TODO.
	ICR = Self::letters_to_token(b"ICR"),
	
	/// TODO.
	ICS = Self::letters_to_token(b"ICS"),
	
	/// TODO.
	ICT = Self::letters_to_token(b"ICT"),
	
	/// TODO.
	ICU = Self::letters_to_token(b"ICU"),
	
	/// TODO.
	ICV = Self::letters_to_token(b"ICV"),
	
	/// TODO.
	ICW = Self::letters_to_token(b"ICW"),
	
	/// TODO.
	ICX = Self::letters_to_token(b"ICX"),
	
	/// TODO.
	ICY = Self::letters_to_token(b"ICY"),
	
	/// TODO.
	ICZ = Self::letters_to_token(b"ICZ"),
	
	/// TODO.
	IDA = Self::letters_to_token(b"IDA"),
	
	/// TODO.
	IDB = Self::letters_to_token(b"IDB"),
	
	/// TODO.
	IDC = Self::letters_to_token(b"IDC"),
	
	/// TODO.
	IDD = Self::letters_to_token(b"IDD"),
	
	/// TODO.
	IDE = Self::letters_to_token(b"IDE"),
	
	/// TODO.
	IDF = Self::letters_to_token(b"IDF"),
	
	/// TODO.
	IDG = Self::letters_to_token(b"IDG"),
	
	/// TODO.
	IDH = Self::letters_to_token(b"IDH"),
	
	/// TODO.
	IDI = Self::letters_to_token(b"IDI"),
	
	/// TODO.
	IDJ = Self::letters_to_token(b"IDJ"),
	
	/// TODO.
	IDK = Self::letters_to_token(b"IDK"),
	
	/// TODO.
	IDL = Self::letters_to_token(b"IDL"),
	
	/// TODO.
	IDM = Self::letters_to_token(b"IDM"),
	
	/// TODO.
	IDN = Self::letters_to_token(b"IDN"),
	
	/// TODO.
	IDO = Self::letters_to_token(b"IDO"),
	
	/// TODO.
	IDP = Self::letters_to_token(b"IDP"),
	
	/// TODO.
	IDQ = Self::letters_to_token(b"IDQ"),
	
	/// TODO.
	IDR = Self::letters_to_token(b"IDR"),
	
	/// TODO.
	IDS = Self::letters_to_token(b"IDS"),
	
	/// TODO.
	IDT = Self::letters_to_token(b"IDT"),
	
	/// TODO.
	IDU = Self::letters_to_token(b"IDU"),
	
	/// TODO.
	IDV = Self::letters_to_token(b"IDV"),
	
	/// TODO.
	IDW = Self::letters_to_token(b"IDW"),
	
	/// TODO.
	IDX = Self::letters_to_token(b"IDX"),
	
	/// TODO.
	IDY = Self::letters_to_token(b"IDY"),
	
	/// TODO.
	IDZ = Self::letters_to_token(b"IDZ"),
	
	/// TODO.
	IEA = Self::letters_to_token(b"IEA"),
	
	/// TODO.
	IEB = Self::letters_to_token(b"IEB"),
	
	/// TODO.
	IEC = Self::letters_to_token(b"IEC"),
	
	/// TODO.
	IED = Self::letters_to_token(b"IED"),
	
	/// TODO.
	IEE = Self::letters_to_token(b"IEE"),
	
	/// TODO.
	IEF = Self::letters_to_token(b"IEF"),
	
	/// TODO.
	IEG = Self::letters_to_token(b"IEG"),
	
	/// TODO.
	IEH = Self::letters_to_token(b"IEH"),
	
	/// TODO.
	IEI = Self::letters_to_token(b"IEI"),
	
	/// TODO.
	IEJ = Self::letters_to_token(b"IEJ"),
	
	/// TODO.
	IEK = Self::letters_to_token(b"IEK"),
	
	/// TODO.
	IEL = Self::letters_to_token(b"IEL"),
	
	/// TODO.
	IEM = Self::letters_to_token(b"IEM"),
	
	/// TODO.
	IEN = Self::letters_to_token(b"IEN"),
	
	/// TODO.
	IEO = Self::letters_to_token(b"IEO"),
	
	/// TODO.
	IEP = Self::letters_to_token(b"IEP"),
	
	/// TODO.
	IEQ = Self::letters_to_token(b"IEQ"),
	
	/// TODO.
	IER = Self::letters_to_token(b"IER"),
	
	/// TODO.
	IES = Self::letters_to_token(b"IES"),
	
	/// TODO.
	IET = Self::letters_to_token(b"IET"),
	
	/// TODO.
	IEU = Self::letters_to_token(b"IEU"),
	
	/// TODO.
	IEV = Self::letters_to_token(b"IEV"),
	
	/// TODO.
	IEW = Self::letters_to_token(b"IEW"),
	
	/// TODO.
	IEX = Self::letters_to_token(b"IEX"),
	
	/// TODO.
	IEY = Self::letters_to_token(b"IEY"),
	
	/// TODO.
	IEZ = Self::letters_to_token(b"IEZ"),
	
	/// TODO.
	IFA = Self::letters_to_token(b"IFA"),
	
	/// TODO.
	IFB = Self::letters_to_token(b"IFB"),
	
	/// TODO.
	IFC = Self::letters_to_token(b"IFC"),
	
	/// TODO.
	IFD = Self::letters_to_token(b"IFD"),
	
	/// TODO.
	IFE = Self::letters_to_token(b"IFE"),
	
	/// TODO.
	IFF = Self::letters_to_token(b"IFF"),
	
	/// TODO.
	IFG = Self::letters_to_token(b"IFG"),
	
	/// TODO.
	IFH = Self::letters_to_token(b"IFH"),
	
	/// TODO.
	IFI = Self::letters_to_token(b"IFI"),
	
	/// TODO.
	IFJ = Self::letters_to_token(b"IFJ"),
	
	/// TODO.
	IFK = Self::letters_to_token(b"IFK"),
	
	/// TODO.
	IFL = Self::letters_to_token(b"IFL"),
	
	/// TODO.
	IFM = Self::letters_to_token(b"IFM"),
	
	/// TODO.
	IFN = Self::letters_to_token(b"IFN"),
	
	/// TODO.
	IFO = Self::letters_to_token(b"IFO"),
	
	/// TODO.
	IFP = Self::letters_to_token(b"IFP"),
	
	/// TODO.
	IFQ = Self::letters_to_token(b"IFQ"),
	
	/// TODO.
	IFR = Self::letters_to_token(b"IFR"),
	
	/// TODO.
	IFS = Self::letters_to_token(b"IFS"),
	
	/// TODO.
	IFT = Self::letters_to_token(b"IFT"),
	
	/// TODO.
	IFU = Self::letters_to_token(b"IFU"),
	
	/// TODO.
	IFV = Self::letters_to_token(b"IFV"),
	
	/// TODO.
	IFW = Self::letters_to_token(b"IFW"),
	
	/// TODO.
	IFX = Self::letters_to_token(b"IFX"),
	
	/// TODO.
	IFY = Self::letters_to_token(b"IFY"),
	
	/// TODO.
	IFZ = Self::letters_to_token(b"IFZ"),
	
	/// TODO.
	IGA = Self::letters_to_token(b"IGA"),
	
	/// TODO.
	IGB = Self::letters_to_token(b"IGB"),
	
	/// TODO.
	IGC = Self::letters_to_token(b"IGC"),
	
	/// TODO.
	IGD = Self::letters_to_token(b"IGD"),
	
	/// TODO.
	IGE = Self::letters_to_token(b"IGE"),
	
	/// TODO.
	IGF = Self::letters_to_token(b"IGF"),
	
	/// TODO.
	IGG = Self::letters_to_token(b"IGG"),
	
	/// TODO.
	IGH = Self::letters_to_token(b"IGH"),
	
	/// TODO.
	IGI = Self::letters_to_token(b"IGI"),
	
	/// TODO.
	IGJ = Self::letters_to_token(b"IGJ"),
	
	/// TODO.
	IGK = Self::letters_to_token(b"IGK"),
	
	/// TODO.
	IGL = Self::letters_to_token(b"IGL"),
	
	/// TODO.
	IGM = Self::letters_to_token(b"IGM"),
	
	/// TODO.
	IGN = Self::letters_to_token(b"IGN"),
	
	/// TODO.
	IGO = Self::letters_to_token(b"IGO"),
	
	/// TODO.
	IGP = Self::letters_to_token(b"IGP"),
	
	/// TODO.
	IGQ = Self::letters_to_token(b"IGQ"),
	
	/// TODO.
	IGR = Self::letters_to_token(b"IGR"),
	
	/// TODO.
	IGS = Self::letters_to_token(b"IGS"),
	
	/// TODO.
	IGT = Self::letters_to_token(b"IGT"),
	
	/// TODO.
	IGU = Self::letters_to_token(b"IGU"),
	
	/// TODO.
	IGV = Self::letters_to_token(b"IGV"),
	
	/// TODO.
	IGW = Self::letters_to_token(b"IGW"),
	
	/// TODO.
	IGX = Self::letters_to_token(b"IGX"),
	
	/// TODO.
	IGY = Self::letters_to_token(b"IGY"),
	
	/// TODO.
	IGZ = Self::letters_to_token(b"IGZ"),
	
	/// TODO.
	IHA = Self::letters_to_token(b"IHA"),
	
	/// TODO.
	IHB = Self::letters_to_token(b"IHB"),
	
	/// TODO.
	IHC = Self::letters_to_token(b"IHC"),
	
	/// TODO.
	IHD = Self::letters_to_token(b"IHD"),
	
	/// TODO.
	IHE = Self::letters_to_token(b"IHE"),
	
	/// TODO.
	IHF = Self::letters_to_token(b"IHF"),
	
	/// TODO.
	IHG = Self::letters_to_token(b"IHG"),
	
	/// TODO.
	IHH = Self::letters_to_token(b"IHH"),
	
	/// TODO.
	IHI = Self::letters_to_token(b"IHI"),
	
	/// TODO.
	IHJ = Self::letters_to_token(b"IHJ"),
	
	/// TODO.
	IHK = Self::letters_to_token(b"IHK"),
	
	/// TODO.
	IHL = Self::letters_to_token(b"IHL"),
	
	/// TODO.
	IHM = Self::letters_to_token(b"IHM"),
	
	/// TODO.
	IHN = Self::letters_to_token(b"IHN"),
	
	/// TODO.
	IHO = Self::letters_to_token(b"IHO"),
	
	/// TODO.
	IHP = Self::letters_to_token(b"IHP"),
	
	/// TODO.
	IHQ = Self::letters_to_token(b"IHQ"),
	
	/// TODO.
	IHR = Self::letters_to_token(b"IHR"),
	
	/// TODO.
	IHS = Self::letters_to_token(b"IHS"),
	
	/// TODO.
	IHT = Self::letters_to_token(b"IHT"),
	
	/// TODO.
	IHU = Self::letters_to_token(b"IHU"),
	
	/// TODO.
	IHV = Self::letters_to_token(b"IHV"),
	
	/// TODO.
	IHW = Self::letters_to_token(b"IHW"),
	
	/// TODO.
	IHX = Self::letters_to_token(b"IHX"),
	
	/// TODO.
	IHY = Self::letters_to_token(b"IHY"),
	
	/// TODO.
	IHZ = Self::letters_to_token(b"IHZ"),
	
	/// TODO.
	IIA = Self::letters_to_token(b"IIA"),
	
	/// TODO.
	IIB = Self::letters_to_token(b"IIB"),
	
	/// TODO.
	IIC = Self::letters_to_token(b"IIC"),
	
	/// TODO.
	IID = Self::letters_to_token(b"IID"),
	
	/// TODO.
	IIE = Self::letters_to_token(b"IIE"),
	
	/// TODO.
	IIF = Self::letters_to_token(b"IIF"),
	
	/// TODO.
	IIG = Self::letters_to_token(b"IIG"),
	
	/// TODO.
	IIH = Self::letters_to_token(b"IIH"),
	
	/// TODO.
	III = Self::letters_to_token(b"III"),
	
	/// TODO.
	IIJ = Self::letters_to_token(b"IIJ"),
	
	/// TODO.
	IIK = Self::letters_to_token(b"IIK"),
	
	/// TODO.
	IIL = Self::letters_to_token(b"IIL"),
	
	/// TODO.
	IIM = Self::letters_to_token(b"IIM"),
	
	/// TODO.
	IIN = Self::letters_to_token(b"IIN"),
	
	/// TODO.
	IIO = Self::letters_to_token(b"IIO"),
	
	/// TODO.
	IIP = Self::letters_to_token(b"IIP"),
	
	/// TODO.
	IIQ = Self::letters_to_token(b"IIQ"),
	
	/// TODO.
	IIR = Self::letters_to_token(b"IIR"),
	
	/// TODO.
	IIS = Self::letters_to_token(b"IIS"),
	
	/// TODO.
	IIT = Self::letters_to_token(b"IIT"),
	
	/// TODO.
	IIU = Self::letters_to_token(b"IIU"),
	
	/// TODO.
	IIV = Self::letters_to_token(b"IIV"),
	
	/// TODO.
	IIW = Self::letters_to_token(b"IIW"),
	
	/// TODO.
	IIX = Self::letters_to_token(b"IIX"),
	
	/// TODO.
	IIY = Self::letters_to_token(b"IIY"),
	
	/// TODO.
	IIZ = Self::letters_to_token(b"IIZ"),
	
	/// TODO.
	IJA = Self::letters_to_token(b"IJA"),
	
	/// TODO.
	IJB = Self::letters_to_token(b"IJB"),
	
	/// TODO.
	IJC = Self::letters_to_token(b"IJC"),
	
	/// TODO.
	IJD = Self::letters_to_token(b"IJD"),
	
	/// TODO.
	IJE = Self::letters_to_token(b"IJE"),
	
	/// TODO.
	IJF = Self::letters_to_token(b"IJF"),
	
	/// TODO.
	IJG = Self::letters_to_token(b"IJG"),
	
	/// TODO.
	IJH = Self::letters_to_token(b"IJH"),
	
	/// TODO.
	IJI = Self::letters_to_token(b"IJI"),
	
	/// TODO.
	IJJ = Self::letters_to_token(b"IJJ"),
	
	/// TODO.
	IJK = Self::letters_to_token(b"IJK"),
	
	/// TODO.
	IJL = Self::letters_to_token(b"IJL"),
	
	/// TODO.
	IJM = Self::letters_to_token(b"IJM"),
	
	/// TODO.
	IJN = Self::letters_to_token(b"IJN"),
	
	/// TODO.
	IJO = Self::letters_to_token(b"IJO"),
	
	/// TODO.
	IJP = Self::letters_to_token(b"IJP"),
	
	/// TODO.
	IJQ = Self::letters_to_token(b"IJQ"),
	
	/// TODO.
	IJR = Self::letters_to_token(b"IJR"),
	
	/// TODO.
	IJS = Self::letters_to_token(b"IJS"),
	
	/// TODO.
	IJT = Self::letters_to_token(b"IJT"),
	
	/// TODO.
	IJU = Self::letters_to_token(b"IJU"),
	
	/// TODO.
	IJV = Self::letters_to_token(b"IJV"),
	
	/// TODO.
	IJW = Self::letters_to_token(b"IJW"),
	
	/// TODO.
	IJX = Self::letters_to_token(b"IJX"),
	
	/// TODO.
	IJY = Self::letters_to_token(b"IJY"),
	
	/// TODO.
	IJZ = Self::letters_to_token(b"IJZ"),
	
	/// TODO.
	IKA = Self::letters_to_token(b"IKA"),
	
	/// TODO.
	IKB = Self::letters_to_token(b"IKB"),
	
	/// TODO.
	IKC = Self::letters_to_token(b"IKC"),
	
	/// TODO.
	IKD = Self::letters_to_token(b"IKD"),
	
	/// TODO.
	IKE = Self::letters_to_token(b"IKE"),
	
	/// TODO.
	IKF = Self::letters_to_token(b"IKF"),
	
	/// TODO.
	IKG = Self::letters_to_token(b"IKG"),
	
	/// TODO.
	IKH = Self::letters_to_token(b"IKH"),
	
	/// TODO.
	IKI = Self::letters_to_token(b"IKI"),
	
	/// TODO.
	IKJ = Self::letters_to_token(b"IKJ"),
	
	/// TODO.
	IKK = Self::letters_to_token(b"IKK"),
	
	/// TODO.
	IKL = Self::letters_to_token(b"IKL"),
	
	/// TODO.
	IKM = Self::letters_to_token(b"IKM"),
	
	/// TODO.
	IKN = Self::letters_to_token(b"IKN"),
	
	/// TODO.
	IKO = Self::letters_to_token(b"IKO"),
	
	/// TODO.
	IKP = Self::letters_to_token(b"IKP"),
	
	/// TODO.
	IKQ = Self::letters_to_token(b"IKQ"),
	
	/// TODO.
	IKR = Self::letters_to_token(b"IKR"),
	
	/// TODO.
	IKS = Self::letters_to_token(b"IKS"),
	
	/// TODO.
	IKT = Self::letters_to_token(b"IKT"),
	
	/// TODO.
	IKU = Self::letters_to_token(b"IKU"),
	
	/// TODO.
	IKV = Self::letters_to_token(b"IKV"),
	
	/// TODO.
	IKW = Self::letters_to_token(b"IKW"),
	
	/// TODO.
	IKX = Self::letters_to_token(b"IKX"),
	
	/// TODO.
	IKY = Self::letters_to_token(b"IKY"),
	
	/// TODO.
	IKZ = Self::letters_to_token(b"IKZ"),
	
	/// TODO.
	ILA = Self::letters_to_token(b"ILA"),
	
	/// TODO.
	ILB = Self::letters_to_token(b"ILB"),
	
	/// TODO.
	ILC = Self::letters_to_token(b"ILC"),
	
	/// TODO.
	ILD = Self::letters_to_token(b"ILD"),
	
	/// TODO.
	ILE = Self::letters_to_token(b"ILE"),
	
	/// TODO.
	ILF = Self::letters_to_token(b"ILF"),
	
	/// TODO.
	ILG = Self::letters_to_token(b"ILG"),
	
	/// TODO.
	ILH = Self::letters_to_token(b"ILH"),
	
	/// TODO.
	ILI = Self::letters_to_token(b"ILI"),
	
	/// TODO.
	ILJ = Self::letters_to_token(b"ILJ"),
	
	/// TODO.
	ILK = Self::letters_to_token(b"ILK"),
	
	/// TODO.
	ILL = Self::letters_to_token(b"ILL"),
	
	/// TODO.
	ILM = Self::letters_to_token(b"ILM"),
	
	/// TODO.
	ILN = Self::letters_to_token(b"ILN"),
	
	/// TODO.
	ILO = Self::letters_to_token(b"ILO"),
	
	/// TODO.
	ILP = Self::letters_to_token(b"ILP"),
	
	/// TODO.
	ILQ = Self::letters_to_token(b"ILQ"),
	
	/// TODO.
	ILR = Self::letters_to_token(b"ILR"),
	
	/// TODO.
	ILS = Self::letters_to_token(b"ILS"),
	
	/// TODO.
	ILT = Self::letters_to_token(b"ILT"),
	
	/// TODO.
	ILU = Self::letters_to_token(b"ILU"),
	
	/// TODO.
	ILV = Self::letters_to_token(b"ILV"),
	
	/// TODO.
	ILW = Self::letters_to_token(b"ILW"),
	
	/// TODO.
	ILX = Self::letters_to_token(b"ILX"),
	
	/// TODO.
	ILY = Self::letters_to_token(b"ILY"),
	
	/// TODO.
	ILZ = Self::letters_to_token(b"ILZ"),
	
	/// TODO.
	IMA = Self::letters_to_token(b"IMA"),
	
	/// TODO.
	IMB = Self::letters_to_token(b"IMB"),
	
	/// TODO.
	IMC = Self::letters_to_token(b"IMC"),
	
	/// TODO.
	IMD = Self::letters_to_token(b"IMD"),
	
	/// TODO.
	IME = Self::letters_to_token(b"IME"),
	
	/// TODO.
	IMF = Self::letters_to_token(b"IMF"),
	
	/// TODO.
	IMG = Self::letters_to_token(b"IMG"),
	
	/// TODO.
	IMH = Self::letters_to_token(b"IMH"),
	
	/// TODO.
	IMI = Self::letters_to_token(b"IMI"),
	
	/// TODO.
	IMJ = Self::letters_to_token(b"IMJ"),
	
	/// TODO.
	IMK = Self::letters_to_token(b"IMK"),
	
	/// TODO.
	IML = Self::letters_to_token(b"IML"),
	
	/// TODO.
	IMM = Self::letters_to_token(b"IMM"),
	
	/// Isle of Man.
	IMN = Self::letters_to_token(b"IMN"),
	
	/// TODO.
	IMO = Self::letters_to_token(b"IMO"),
	
	/// TODO.
	IMP = Self::letters_to_token(b"IMP"),
	
	/// TODO.
	IMQ = Self::letters_to_token(b"IMQ"),
	
	/// TODO.
	IMR = Self::letters_to_token(b"IMR"),
	
	/// TODO.
	IMS = Self::letters_to_token(b"IMS"),
	
	/// TODO.
	IMT = Self::letters_to_token(b"IMT"),
	
	/// TODO.
	IMU = Self::letters_to_token(b"IMU"),
	
	/// TODO.
	IMV = Self::letters_to_token(b"IMV"),
	
	/// TODO.
	IMW = Self::letters_to_token(b"IMW"),
	
	/// TODO.
	IMX = Self::letters_to_token(b"IMX"),
	
	/// TODO.
	IMY = Self::letters_to_token(b"IMY"),
	
	/// TODO.
	IMZ = Self::letters_to_token(b"IMZ"),
	
	/// TODO.
	INA = Self::letters_to_token(b"INA"),
	
	/// TODO.
	INB = Self::letters_to_token(b"INB"),
	
	/// TODO.
	INC = Self::letters_to_token(b"INC"),
	
	/// TODO.
	IND = Self::letters_to_token(b"IND"),
	
	/// TODO.
	INE = Self::letters_to_token(b"INE"),
	
	/// TODO.
	INF = Self::letters_to_token(b"INF"),
	
	/// TODO.
	ING = Self::letters_to_token(b"ING"),
	
	/// TODO.
	INH = Self::letters_to_token(b"INH"),
	
	/// TODO.
	INI = Self::letters_to_token(b"INI"),
	
	/// TODO.
	INJ = Self::letters_to_token(b"INJ"),
	
	/// TODO.
	INK = Self::letters_to_token(b"INK"),
	
	/// TODO.
	INL = Self::letters_to_token(b"INL"),
	
	/// TODO.
	INM = Self::letters_to_token(b"INM"),
	
	/// TODO.
	INN = Self::letters_to_token(b"INN"),
	
	/// TODO.
	INO = Self::letters_to_token(b"INO"),
	
	/// TODO.
	INP = Self::letters_to_token(b"INP"),
	
	/// TODO.
	INQ = Self::letters_to_token(b"INQ"),
	
	/// TODO.
	INR = Self::letters_to_token(b"INR"),
	
	/// TODO.
	INS = Self::letters_to_token(b"INS"),
	
	/// TODO.
	INT = Self::letters_to_token(b"INT"),
	
	/// TODO.
	INU = Self::letters_to_token(b"INU"),
	
	/// TODO.
	INV = Self::letters_to_token(b"INV"),
	
	/// TODO.
	INW = Self::letters_to_token(b"INW"),
	
	/// TODO.
	INX = Self::letters_to_token(b"INX"),
	
	/// TODO.
	INY = Self::letters_to_token(b"INY"),
	
	/// TODO.
	INZ = Self::letters_to_token(b"INZ"),
	
	/// TODO.
	IOA = Self::letters_to_token(b"IOA"),
	
	/// TODO.
	IOB = Self::letters_to_token(b"IOB"),
	
	/// TODO.
	IOC = Self::letters_to_token(b"IOC"),
	
	/// TODO.
	IOD = Self::letters_to_token(b"IOD"),
	
	/// TODO.
	IOE = Self::letters_to_token(b"IOE"),
	
	/// TODO.
	IOF = Self::letters_to_token(b"IOF"),
	
	/// TODO.
	IOG = Self::letters_to_token(b"IOG"),
	
	/// TODO.
	IOH = Self::letters_to_token(b"IOH"),
	
	/// TODO.
	IOI = Self::letters_to_token(b"IOI"),
	
	/// TODO.
	IOJ = Self::letters_to_token(b"IOJ"),
	
	/// TODO.
	IOK = Self::letters_to_token(b"IOK"),
	
	/// TODO.
	IOL = Self::letters_to_token(b"IOL"),
	
	/// TODO.
	IOM = Self::letters_to_token(b"IOM"),
	
	/// TODO.
	ION = Self::letters_to_token(b"ION"),
	
	/// TODO.
	IOO = Self::letters_to_token(b"IOO"),
	
	/// TODO.
	IOP = Self::letters_to_token(b"IOP"),
	
	/// TODO.
	IOQ = Self::letters_to_token(b"IOQ"),
	
	/// TODO.
	IOR = Self::letters_to_token(b"IOR"),
	
	/// TODO.
	IOS = Self::letters_to_token(b"IOS"),
	
	/// British Indian Ocean Territory.
	IOT = Self::letters_to_token(b"IOT"),
	
	/// TODO.
	IOU = Self::letters_to_token(b"IOU"),
	
	/// TODO.
	IOV = Self::letters_to_token(b"IOV"),
	
	/// TODO.
	IOW = Self::letters_to_token(b"IOW"),
	
	/// TODO.
	IOX = Self::letters_to_token(b"IOX"),
	
	/// TODO.
	IOY = Self::letters_to_token(b"IOY"),
	
	/// TODO.
	IOZ = Self::letters_to_token(b"IOZ"),
	
	/// TODO.
	IPA = Self::letters_to_token(b"IPA"),
	
	/// TODO.
	IPB = Self::letters_to_token(b"IPB"),
	
	/// TODO.
	IPC = Self::letters_to_token(b"IPC"),
	
	/// TODO.
	IPD = Self::letters_to_token(b"IPD"),
	
	/// TODO.
	IPE = Self::letters_to_token(b"IPE"),
	
	/// TODO.
	IPF = Self::letters_to_token(b"IPF"),
	
	/// TODO.
	IPG = Self::letters_to_token(b"IPG"),
	
	/// TODO.
	IPH = Self::letters_to_token(b"IPH"),
	
	/// TODO.
	IPI = Self::letters_to_token(b"IPI"),
	
	/// TODO.
	IPJ = Self::letters_to_token(b"IPJ"),
	
	/// TODO.
	IPK = Self::letters_to_token(b"IPK"),
	
	/// TODO.
	IPL = Self::letters_to_token(b"IPL"),
	
	/// TODO.
	IPM = Self::letters_to_token(b"IPM"),
	
	/// TODO.
	IPN = Self::letters_to_token(b"IPN"),
	
	/// TODO.
	IPO = Self::letters_to_token(b"IPO"),
	
	/// TODO.
	IPP = Self::letters_to_token(b"IPP"),
	
	/// TODO.
	IPQ = Self::letters_to_token(b"IPQ"),
	
	/// TODO.
	IPR = Self::letters_to_token(b"IPR"),
	
	/// TODO.
	IPS = Self::letters_to_token(b"IPS"),
	
	/// TODO.
	IPT = Self::letters_to_token(b"IPT"),
	
	/// TODO.
	IPU = Self::letters_to_token(b"IPU"),
	
	/// TODO.
	IPV = Self::letters_to_token(b"IPV"),
	
	/// TODO.
	IPW = Self::letters_to_token(b"IPW"),
	
	/// TODO.
	IPX = Self::letters_to_token(b"IPX"),
	
	/// TODO.
	IPY = Self::letters_to_token(b"IPY"),
	
	/// TODO.
	IPZ = Self::letters_to_token(b"IPZ"),
	
	/// TODO.
	IQA = Self::letters_to_token(b"IQA"),
	
	/// TODO.
	IQB = Self::letters_to_token(b"IQB"),
	
	/// TODO.
	IQC = Self::letters_to_token(b"IQC"),
	
	/// TODO.
	IQD = Self::letters_to_token(b"IQD"),
	
	/// TODO.
	IQE = Self::letters_to_token(b"IQE"),
	
	/// TODO.
	IQF = Self::letters_to_token(b"IQF"),
	
	/// TODO.
	IQG = Self::letters_to_token(b"IQG"),
	
	/// TODO.
	IQH = Self::letters_to_token(b"IQH"),
	
	/// TODO.
	IQI = Self::letters_to_token(b"IQI"),
	
	/// TODO.
	IQJ = Self::letters_to_token(b"IQJ"),
	
	/// TODO.
	IQK = Self::letters_to_token(b"IQK"),
	
	/// TODO.
	IQL = Self::letters_to_token(b"IQL"),
	
	/// TODO.
	IQM = Self::letters_to_token(b"IQM"),
	
	/// TODO.
	IQN = Self::letters_to_token(b"IQN"),
	
	/// TODO.
	IQO = Self::letters_to_token(b"IQO"),
	
	/// TODO.
	IQP = Self::letters_to_token(b"IQP"),
	
	/// TODO.
	IQQ = Self::letters_to_token(b"IQQ"),
	
	/// TODO.
	IQR = Self::letters_to_token(b"IQR"),
	
	/// TODO.
	IQS = Self::letters_to_token(b"IQS"),
	
	/// TODO.
	IQT = Self::letters_to_token(b"IQT"),
	
	/// TODO.
	IQU = Self::letters_to_token(b"IQU"),
	
	/// TODO.
	IQV = Self::letters_to_token(b"IQV"),
	
	/// TODO.
	IQW = Self::letters_to_token(b"IQW"),
	
	/// TODO.
	IQX = Self::letters_to_token(b"IQX"),
	
	/// TODO.
	IQY = Self::letters_to_token(b"IQY"),
	
	/// TODO.
	IQZ = Self::letters_to_token(b"IQZ"),
	
	/// TODO.
	IRA = Self::letters_to_token(b"IRA"),
	
	/// TODO.
	IRB = Self::letters_to_token(b"IRB"),
	
	/// TODO.
	IRC = Self::letters_to_token(b"IRC"),
	
	/// TODO.
	IRD = Self::letters_to_token(b"IRD"),
	
	/// TODO.
	IRE = Self::letters_to_token(b"IRE"),
	
	/// TODO.
	IRF = Self::letters_to_token(b"IRF"),
	
	/// TODO.
	IRG = Self::letters_to_token(b"IRG"),
	
	/// TODO.
	IRH = Self::letters_to_token(b"IRH"),
	
	/// TODO.
	IRI = Self::letters_to_token(b"IRI"),
	
	/// TODO.
	IRJ = Self::letters_to_token(b"IRJ"),
	
	/// TODO.
	IRK = Self::letters_to_token(b"IRK"),
	
	/// TODO.
	IRL = Self::letters_to_token(b"IRL"),
	
	/// TODO.
	IRM = Self::letters_to_token(b"IRM"),
	
	/// TODO.
	IRN = Self::letters_to_token(b"IRN"),
	
	/// TODO.
	IRO = Self::letters_to_token(b"IRO"),
	
	/// TODO.
	IRP = Self::letters_to_token(b"IRP"),
	
	/// TODO.
	IRQ = Self::letters_to_token(b"IRQ"),
	
	/// TODO.
	IRR = Self::letters_to_token(b"IRR"),
	
	/// TODO.
	IRS = Self::letters_to_token(b"IRS"),
	
	/// TODO.
	IRT = Self::letters_to_token(b"IRT"),
	
	/// TODO.
	IRU = Self::letters_to_token(b"IRU"),
	
	/// TODO.
	IRV = Self::letters_to_token(b"IRV"),
	
	/// TODO.
	IRW = Self::letters_to_token(b"IRW"),
	
	/// TODO.
	IRX = Self::letters_to_token(b"IRX"),
	
	/// TODO.
	IRY = Self::letters_to_token(b"IRY"),
	
	/// TODO.
	IRZ = Self::letters_to_token(b"IRZ"),
	
	/// TODO.
	ISA = Self::letters_to_token(b"ISA"),
	
	/// TODO.
	ISB = Self::letters_to_token(b"ISB"),
	
	/// TODO.
	ISC = Self::letters_to_token(b"ISC"),
	
	/// TODO.
	ISD = Self::letters_to_token(b"ISD"),
	
	/// TODO.
	ISE = Self::letters_to_token(b"ISE"),
	
	/// TODO.
	ISF = Self::letters_to_token(b"ISF"),
	
	/// TODO.
	ISG = Self::letters_to_token(b"ISG"),
	
	/// TODO.
	ISH = Self::letters_to_token(b"ISH"),
	
	/// TODO.
	ISI = Self::letters_to_token(b"ISI"),
	
	/// TODO.
	ISJ = Self::letters_to_token(b"ISJ"),
	
	/// TODO.
	ISK = Self::letters_to_token(b"ISK"),
	
	/// TODO.
	ISL = Self::letters_to_token(b"ISL"),
	
	/// TODO.
	ISM = Self::letters_to_token(b"ISM"),
	
	/// TODO.
	ISN = Self::letters_to_token(b"ISN"),
	
	/// TODO.
	ISO = Self::letters_to_token(b"ISO"),
	
	/// TODO.
	ISP = Self::letters_to_token(b"ISP"),
	
	/// TODO.
	ISQ = Self::letters_to_token(b"ISQ"),
	
	/// TODO.
	ISR = Self::letters_to_token(b"ISR"),
	
	/// TODO.
	ISS = Self::letters_to_token(b"ISS"),
	
	/// TODO.
	IST = Self::letters_to_token(b"IST"),
	
	/// TODO.
	ISU = Self::letters_to_token(b"ISU"),
	
	/// TODO.
	ISV = Self::letters_to_token(b"ISV"),
	
	/// TODO.
	ISW = Self::letters_to_token(b"ISW"),
	
	/// TODO.
	ISX = Self::letters_to_token(b"ISX"),
	
	/// TODO.
	ISY = Self::letters_to_token(b"ISY"),
	
	/// TODO.
	ISZ = Self::letters_to_token(b"ISZ"),
	
	/// TODO.
	ITA = Self::letters_to_token(b"ITA"),
	
	/// TODO.
	ITB = Self::letters_to_token(b"ITB"),
	
	/// TODO.
	ITC = Self::letters_to_token(b"ITC"),
	
	/// TODO.
	ITD = Self::letters_to_token(b"ITD"),
	
	/// TODO.
	ITE = Self::letters_to_token(b"ITE"),
	
	/// TODO.
	ITF = Self::letters_to_token(b"ITF"),
	
	/// TODO.
	ITG = Self::letters_to_token(b"ITG"),
	
	/// TODO.
	ITH = Self::letters_to_token(b"ITH"),
	
	/// TODO.
	ITI = Self::letters_to_token(b"ITI"),
	
	/// TODO.
	ITJ = Self::letters_to_token(b"ITJ"),
	
	/// TODO.
	ITK = Self::letters_to_token(b"ITK"),
	
	/// TODO.
	ITL = Self::letters_to_token(b"ITL"),
	
	/// TODO.
	ITM = Self::letters_to_token(b"ITM"),
	
	/// TODO.
	ITN = Self::letters_to_token(b"ITN"),
	
	/// TODO.
	ITO = Self::letters_to_token(b"ITO"),
	
	/// TODO.
	ITP = Self::letters_to_token(b"ITP"),
	
	/// TODO.
	ITQ = Self::letters_to_token(b"ITQ"),
	
	/// TODO.
	ITR = Self::letters_to_token(b"ITR"),
	
	/// TODO.
	ITS = Self::letters_to_token(b"ITS"),
	
	/// TODO.
	ITT = Self::letters_to_token(b"ITT"),
	
	/// TODO.
	ITU = Self::letters_to_token(b"ITU"),
	
	/// TODO.
	ITV = Self::letters_to_token(b"ITV"),
	
	/// TODO.
	ITW = Self::letters_to_token(b"ITW"),
	
	/// TODO.
	ITX = Self::letters_to_token(b"ITX"),
	
	/// TODO.
	ITY = Self::letters_to_token(b"ITY"),
	
	/// TODO.
	ITZ = Self::letters_to_token(b"ITZ"),
	
	/// TODO.
	IUA = Self::letters_to_token(b"IUA"),
	
	/// TODO.
	IUB = Self::letters_to_token(b"IUB"),
	
	/// TODO.
	IUC = Self::letters_to_token(b"IUC"),
	
	/// TODO.
	IUD = Self::letters_to_token(b"IUD"),
	
	/// TODO.
	IUE = Self::letters_to_token(b"IUE"),
	
	/// TODO.
	IUF = Self::letters_to_token(b"IUF"),
	
	/// TODO.
	IUG = Self::letters_to_token(b"IUG"),
	
	/// TODO.
	IUH = Self::letters_to_token(b"IUH"),
	
	/// TODO.
	IUI = Self::letters_to_token(b"IUI"),
	
	/// TODO.
	IUJ = Self::letters_to_token(b"IUJ"),
	
	/// TODO.
	IUK = Self::letters_to_token(b"IUK"),
	
	/// TODO.
	IUL = Self::letters_to_token(b"IUL"),
	
	/// TODO.
	IUM = Self::letters_to_token(b"IUM"),
	
	/// TODO.
	IUN = Self::letters_to_token(b"IUN"),
	
	/// TODO.
	IUO = Self::letters_to_token(b"IUO"),
	
	/// TODO.
	IUP = Self::letters_to_token(b"IUP"),
	
	/// TODO.
	IUQ = Self::letters_to_token(b"IUQ"),
	
	/// TODO.
	IUR = Self::letters_to_token(b"IUR"),
	
	/// TODO.
	IUS = Self::letters_to_token(b"IUS"),
	
	/// TODO.
	IUT = Self::letters_to_token(b"IUT"),
	
	/// TODO.
	IUU = Self::letters_to_token(b"IUU"),
	
	/// TODO.
	IUV = Self::letters_to_token(b"IUV"),
	
	/// TODO.
	IUW = Self::letters_to_token(b"IUW"),
	
	/// TODO.
	IUX = Self::letters_to_token(b"IUX"),
	
	/// TODO.
	IUY = Self::letters_to_token(b"IUY"),
	
	/// TODO.
	IUZ = Self::letters_to_token(b"IUZ"),
	
	/// TODO.
	IVA = Self::letters_to_token(b"IVA"),
	
	/// TODO.
	IVB = Self::letters_to_token(b"IVB"),
	
	/// TODO.
	IVC = Self::letters_to_token(b"IVC"),
	
	/// TODO.
	IVD = Self::letters_to_token(b"IVD"),
	
	/// TODO.
	IVE = Self::letters_to_token(b"IVE"),
	
	/// TODO.
	IVF = Self::letters_to_token(b"IVF"),
	
	/// TODO.
	IVG = Self::letters_to_token(b"IVG"),
	
	/// TODO.
	IVH = Self::letters_to_token(b"IVH"),
	
	/// TODO.
	IVI = Self::letters_to_token(b"IVI"),
	
	/// TODO.
	IVJ = Self::letters_to_token(b"IVJ"),
	
	/// TODO.
	IVK = Self::letters_to_token(b"IVK"),
	
	/// TODO.
	IVL = Self::letters_to_token(b"IVL"),
	
	/// TODO.
	IVM = Self::letters_to_token(b"IVM"),
	
	/// TODO.
	IVN = Self::letters_to_token(b"IVN"),
	
	/// TODO.
	IVO = Self::letters_to_token(b"IVO"),
	
	/// TODO.
	IVP = Self::letters_to_token(b"IVP"),
	
	/// TODO.
	IVQ = Self::letters_to_token(b"IVQ"),
	
	/// TODO.
	IVR = Self::letters_to_token(b"IVR"),
	
	/// TODO.
	IVS = Self::letters_to_token(b"IVS"),
	
	/// TODO.
	IVT = Self::letters_to_token(b"IVT"),
	
	/// TODO.
	IVU = Self::letters_to_token(b"IVU"),
	
	/// TODO.
	IVV = Self::letters_to_token(b"IVV"),
	
	/// TODO.
	IVW = Self::letters_to_token(b"IVW"),
	
	/// TODO.
	IVX = Self::letters_to_token(b"IVX"),
	
	/// TODO.
	IVY = Self::letters_to_token(b"IVY"),
	
	/// TODO.
	IVZ = Self::letters_to_token(b"IVZ"),
	
	/// TODO.
	IWA = Self::letters_to_token(b"IWA"),
	
	/// TODO.
	IWB = Self::letters_to_token(b"IWB"),
	
	/// TODO.
	IWC = Self::letters_to_token(b"IWC"),
	
	/// TODO.
	IWD = Self::letters_to_token(b"IWD"),
	
	/// TODO.
	IWE = Self::letters_to_token(b"IWE"),
	
	/// TODO.
	IWF = Self::letters_to_token(b"IWF"),
	
	/// TODO.
	IWG = Self::letters_to_token(b"IWG"),
	
	/// TODO.
	IWH = Self::letters_to_token(b"IWH"),
	
	/// TODO.
	IWI = Self::letters_to_token(b"IWI"),
	
	/// TODO.
	IWJ = Self::letters_to_token(b"IWJ"),
	
	/// TODO.
	IWK = Self::letters_to_token(b"IWK"),
	
	/// TODO.
	IWL = Self::letters_to_token(b"IWL"),
	
	/// TODO.
	IWM = Self::letters_to_token(b"IWM"),
	
	/// TODO.
	IWN = Self::letters_to_token(b"IWN"),
	
	/// TODO.
	IWO = Self::letters_to_token(b"IWO"),
	
	/// TODO.
	IWP = Self::letters_to_token(b"IWP"),
	
	/// TODO.
	IWQ = Self::letters_to_token(b"IWQ"),
	
	/// TODO.
	IWR = Self::letters_to_token(b"IWR"),
	
	/// TODO.
	IWS = Self::letters_to_token(b"IWS"),
	
	/// TODO.
	IWT = Self::letters_to_token(b"IWT"),
	
	/// TODO.
	IWU = Self::letters_to_token(b"IWU"),
	
	/// TODO.
	IWV = Self::letters_to_token(b"IWV"),
	
	/// TODO.
	IWW = Self::letters_to_token(b"IWW"),
	
	/// TODO.
	IWX = Self::letters_to_token(b"IWX"),
	
	/// TODO.
	IWY = Self::letters_to_token(b"IWY"),
	
	/// TODO.
	IWZ = Self::letters_to_token(b"IWZ"),
	
	/// TODO.
	IXA = Self::letters_to_token(b"IXA"),
	
	/// TODO.
	IXB = Self::letters_to_token(b"IXB"),
	
	/// TODO.
	IXC = Self::letters_to_token(b"IXC"),
	
	/// TODO.
	IXD = Self::letters_to_token(b"IXD"),
	
	/// TODO.
	IXE = Self::letters_to_token(b"IXE"),
	
	/// TODO.
	IXF = Self::letters_to_token(b"IXF"),
	
	/// TODO.
	IXG = Self::letters_to_token(b"IXG"),
	
	/// TODO.
	IXH = Self::letters_to_token(b"IXH"),
	
	/// TODO.
	IXI = Self::letters_to_token(b"IXI"),
	
	/// TODO.
	IXJ = Self::letters_to_token(b"IXJ"),
	
	/// TODO.
	IXK = Self::letters_to_token(b"IXK"),
	
	/// TODO.
	IXL = Self::letters_to_token(b"IXL"),
	
	/// TODO.
	IXM = Self::letters_to_token(b"IXM"),
	
	/// TODO.
	IXN = Self::letters_to_token(b"IXN"),
	
	/// TODO.
	IXO = Self::letters_to_token(b"IXO"),
	
	/// TODO.
	IXP = Self::letters_to_token(b"IXP"),
	
	/// TODO.
	IXQ = Self::letters_to_token(b"IXQ"),
	
	/// TODO.
	IXR = Self::letters_to_token(b"IXR"),
	
	/// TODO.
	IXS = Self::letters_to_token(b"IXS"),
	
	/// TODO.
	IXT = Self::letters_to_token(b"IXT"),
	
	/// TODO.
	IXU = Self::letters_to_token(b"IXU"),
	
	/// TODO.
	IXV = Self::letters_to_token(b"IXV"),
	
	/// TODO.
	IXW = Self::letters_to_token(b"IXW"),
	
	/// TODO.
	IXX = Self::letters_to_token(b"IXX"),
	
	/// TODO.
	IXY = Self::letters_to_token(b"IXY"),
	
	/// TODO.
	IXZ = Self::letters_to_token(b"IXZ"),
	
	/// TODO.
	IYA = Self::letters_to_token(b"IYA"),
	
	/// TODO.
	IYB = Self::letters_to_token(b"IYB"),
	
	/// TODO.
	IYC = Self::letters_to_token(b"IYC"),
	
	/// TODO.
	IYD = Self::letters_to_token(b"IYD"),
	
	/// TODO.
	IYE = Self::letters_to_token(b"IYE"),
	
	/// TODO.
	IYF = Self::letters_to_token(b"IYF"),
	
	/// TODO.
	IYG = Self::letters_to_token(b"IYG"),
	
	/// TODO.
	IYH = Self::letters_to_token(b"IYH"),
	
	/// TODO.
	IYI = Self::letters_to_token(b"IYI"),
	
	/// TODO.
	IYJ = Self::letters_to_token(b"IYJ"),
	
	/// TODO.
	IYK = Self::letters_to_token(b"IYK"),
	
	/// TODO.
	IYL = Self::letters_to_token(b"IYL"),
	
	/// TODO.
	IYM = Self::letters_to_token(b"IYM"),
	
	/// TODO.
	IYN = Self::letters_to_token(b"IYN"),
	
	/// TODO.
	IYO = Self::letters_to_token(b"IYO"),
	
	/// TODO.
	IYP = Self::letters_to_token(b"IYP"),
	
	/// TODO.
	IYQ = Self::letters_to_token(b"IYQ"),
	
	/// TODO.
	IYR = Self::letters_to_token(b"IYR"),
	
	/// TODO.
	IYS = Self::letters_to_token(b"IYS"),
	
	/// TODO.
	IYT = Self::letters_to_token(b"IYT"),
	
	/// TODO.
	IYU = Self::letters_to_token(b"IYU"),
	
	/// TODO.
	IYV = Self::letters_to_token(b"IYV"),
	
	/// TODO.
	IYW = Self::letters_to_token(b"IYW"),
	
	/// TODO.
	IYX = Self::letters_to_token(b"IYX"),
	
	/// TODO.
	IYY = Self::letters_to_token(b"IYY"),
	
	/// TODO.
	IYZ = Self::letters_to_token(b"IYZ"),
	
	/// TODO.
	IZA = Self::letters_to_token(b"IZA"),
	
	/// TODO.
	IZB = Self::letters_to_token(b"IZB"),
	
	/// TODO.
	IZC = Self::letters_to_token(b"IZC"),
	
	/// TODO.
	IZD = Self::letters_to_token(b"IZD"),
	
	/// TODO.
	IZE = Self::letters_to_token(b"IZE"),
	
	/// TODO.
	IZF = Self::letters_to_token(b"IZF"),
	
	/// TODO.
	IZG = Self::letters_to_token(b"IZG"),
	
	/// TODO.
	IZH = Self::letters_to_token(b"IZH"),
	
	/// TODO.
	IZI = Self::letters_to_token(b"IZI"),
	
	/// TODO.
	IZJ = Self::letters_to_token(b"IZJ"),
	
	/// TODO.
	IZK = Self::letters_to_token(b"IZK"),
	
	/// TODO.
	IZL = Self::letters_to_token(b"IZL"),
	
	/// TODO.
	IZM = Self::letters_to_token(b"IZM"),
	
	/// TODO.
	IZN = Self::letters_to_token(b"IZN"),
	
	/// TODO.
	IZO = Self::letters_to_token(b"IZO"),
	
	/// TODO.
	IZP = Self::letters_to_token(b"IZP"),
	
	/// TODO.
	IZQ = Self::letters_to_token(b"IZQ"),
	
	/// TODO.
	IZR = Self::letters_to_token(b"IZR"),
	
	/// TODO.
	IZS = Self::letters_to_token(b"IZS"),
	
	/// TODO.
	IZT = Self::letters_to_token(b"IZT"),
	
	/// TODO.
	IZU = Self::letters_to_token(b"IZU"),
	
	/// TODO.
	IZV = Self::letters_to_token(b"IZV"),
	
	/// TODO.
	IZW = Self::letters_to_token(b"IZW"),
	
	/// TODO.
	IZX = Self::letters_to_token(b"IZX"),
	
	/// TODO.
	IZY = Self::letters_to_token(b"IZY"),
	
	/// TODO.
	IZZ = Self::letters_to_token(b"IZZ"),
	
	/// TODO.
	JAA = Self::letters_to_token(b"JAA"),
	
	/// TODO.
	JAB = Self::letters_to_token(b"JAB"),
	
	/// TODO.
	JAC = Self::letters_to_token(b"JAC"),
	
	/// TODO.
	JAD = Self::letters_to_token(b"JAD"),
	
	/// TODO.
	JAE = Self::letters_to_token(b"JAE"),
	
	/// TODO.
	JAF = Self::letters_to_token(b"JAF"),
	
	/// TODO.
	JAG = Self::letters_to_token(b"JAG"),
	
	/// TODO.
	JAH = Self::letters_to_token(b"JAH"),
	
	/// TODO.
	JAI = Self::letters_to_token(b"JAI"),
	
	/// TODO.
	JAJ = Self::letters_to_token(b"JAJ"),
	
	/// TODO.
	JAK = Self::letters_to_token(b"JAK"),
	
	/// TODO.
	JAL = Self::letters_to_token(b"JAL"),
	
	/// TODO.
	JAM = Self::letters_to_token(b"JAM"),
	
	/// TODO.
	JAN = Self::letters_to_token(b"JAN"),
	
	/// TODO.
	JAO = Self::letters_to_token(b"JAO"),
	
	/// TODO.
	JAP = Self::letters_to_token(b"JAP"),
	
	/// TODO.
	JAQ = Self::letters_to_token(b"JAQ"),
	
	/// TODO.
	JAR = Self::letters_to_token(b"JAR"),
	
	/// TODO.
	JAS = Self::letters_to_token(b"JAS"),
	
	/// TODO.
	JAT = Self::letters_to_token(b"JAT"),
	
	/// TODO.
	JAU = Self::letters_to_token(b"JAU"),
	
	/// TODO.
	JAV = Self::letters_to_token(b"JAV"),
	
	/// TODO.
	JAW = Self::letters_to_token(b"JAW"),
	
	/// TODO.
	JAX = Self::letters_to_token(b"JAX"),
	
	/// TODO.
	JAY = Self::letters_to_token(b"JAY"),
	
	/// TODO.
	JAZ = Self::letters_to_token(b"JAZ"),
	
	/// TODO.
	JBA = Self::letters_to_token(b"JBA"),
	
	/// TODO.
	JBB = Self::letters_to_token(b"JBB"),
	
	/// TODO.
	JBC = Self::letters_to_token(b"JBC"),
	
	/// TODO.
	JBD = Self::letters_to_token(b"JBD"),
	
	/// TODO.
	JBE = Self::letters_to_token(b"JBE"),
	
	/// TODO.
	JBF = Self::letters_to_token(b"JBF"),
	
	/// TODO.
	JBG = Self::letters_to_token(b"JBG"),
	
	/// TODO.
	JBH = Self::letters_to_token(b"JBH"),
	
	/// TODO.
	JBI = Self::letters_to_token(b"JBI"),
	
	/// TODO.
	JBJ = Self::letters_to_token(b"JBJ"),
	
	/// TODO.
	JBK = Self::letters_to_token(b"JBK"),
	
	/// TODO.
	JBL = Self::letters_to_token(b"JBL"),
	
	/// TODO.
	JBM = Self::letters_to_token(b"JBM"),
	
	/// TODO.
	JBN = Self::letters_to_token(b"JBN"),
	
	/// TODO.
	JBO = Self::letters_to_token(b"JBO"),
	
	/// TODO.
	JBP = Self::letters_to_token(b"JBP"),
	
	/// TODO.
	JBQ = Self::letters_to_token(b"JBQ"),
	
	/// TODO.
	JBR = Self::letters_to_token(b"JBR"),
	
	/// TODO.
	JBS = Self::letters_to_token(b"JBS"),
	
	/// TODO.
	JBT = Self::letters_to_token(b"JBT"),
	
	/// TODO.
	JBU = Self::letters_to_token(b"JBU"),
	
	/// TODO.
	JBV = Self::letters_to_token(b"JBV"),
	
	/// TODO.
	JBW = Self::letters_to_token(b"JBW"),
	
	/// TODO.
	JBX = Self::letters_to_token(b"JBX"),
	
	/// TODO.
	JBY = Self::letters_to_token(b"JBY"),
	
	/// TODO.
	JBZ = Self::letters_to_token(b"JBZ"),
	
	/// TODO.
	JCA = Self::letters_to_token(b"JCA"),
	
	/// TODO.
	JCB = Self::letters_to_token(b"JCB"),
	
	/// TODO.
	JCC = Self::letters_to_token(b"JCC"),
	
	/// TODO.
	JCD = Self::letters_to_token(b"JCD"),
	
	/// TODO.
	JCE = Self::letters_to_token(b"JCE"),
	
	/// TODO.
	JCF = Self::letters_to_token(b"JCF"),
	
	/// TODO.
	JCG = Self::letters_to_token(b"JCG"),
	
	/// TODO.
	JCH = Self::letters_to_token(b"JCH"),
	
	/// TODO.
	JCI = Self::letters_to_token(b"JCI"),
	
	/// TODO.
	JCJ = Self::letters_to_token(b"JCJ"),
	
	/// TODO.
	JCK = Self::letters_to_token(b"JCK"),
	
	/// TODO.
	JCL = Self::letters_to_token(b"JCL"),
	
	/// TODO.
	JCM = Self::letters_to_token(b"JCM"),
	
	/// TODO.
	JCN = Self::letters_to_token(b"JCN"),
	
	/// TODO.
	JCO = Self::letters_to_token(b"JCO"),
	
	/// TODO.
	JCP = Self::letters_to_token(b"JCP"),
	
	/// TODO.
	JCQ = Self::letters_to_token(b"JCQ"),
	
	/// TODO.
	JCR = Self::letters_to_token(b"JCR"),
	
	/// TODO.
	JCS = Self::letters_to_token(b"JCS"),
	
	/// TODO.
	JCT = Self::letters_to_token(b"JCT"),
	
	/// TODO.
	JCU = Self::letters_to_token(b"JCU"),
	
	/// TODO.
	JCV = Self::letters_to_token(b"JCV"),
	
	/// TODO.
	JCW = Self::letters_to_token(b"JCW"),
	
	/// TODO.
	JCX = Self::letters_to_token(b"JCX"),
	
	/// TODO.
	JCY = Self::letters_to_token(b"JCY"),
	
	/// TODO.
	JCZ = Self::letters_to_token(b"JCZ"),
	
	/// TODO.
	JDA = Self::letters_to_token(b"JDA"),
	
	/// TODO.
	JDB = Self::letters_to_token(b"JDB"),
	
	/// TODO.
	JDC = Self::letters_to_token(b"JDC"),
	
	/// TODO.
	JDD = Self::letters_to_token(b"JDD"),
	
	/// TODO.
	JDE = Self::letters_to_token(b"JDE"),
	
	/// TODO.
	JDF = Self::letters_to_token(b"JDF"),
	
	/// TODO.
	JDG = Self::letters_to_token(b"JDG"),
	
	/// TODO.
	JDH = Self::letters_to_token(b"JDH"),
	
	/// TODO.
	JDI = Self::letters_to_token(b"JDI"),
	
	/// TODO.
	JDJ = Self::letters_to_token(b"JDJ"),
	
	/// TODO.
	JDK = Self::letters_to_token(b"JDK"),
	
	/// TODO.
	JDL = Self::letters_to_token(b"JDL"),
	
	/// TODO.
	JDM = Self::letters_to_token(b"JDM"),
	
	/// TODO.
	JDN = Self::letters_to_token(b"JDN"),
	
	/// TODO.
	JDO = Self::letters_to_token(b"JDO"),
	
	/// TODO.
	JDP = Self::letters_to_token(b"JDP"),
	
	/// TODO.
	JDQ = Self::letters_to_token(b"JDQ"),
	
	/// TODO.
	JDR = Self::letters_to_token(b"JDR"),
	
	/// TODO.
	JDS = Self::letters_to_token(b"JDS"),
	
	/// TODO.
	JDT = Self::letters_to_token(b"JDT"),
	
	/// TODO.
	JDU = Self::letters_to_token(b"JDU"),
	
	/// TODO.
	JDV = Self::letters_to_token(b"JDV"),
	
	/// TODO.
	JDW = Self::letters_to_token(b"JDW"),
	
	/// TODO.
	JDX = Self::letters_to_token(b"JDX"),
	
	/// TODO.
	JDY = Self::letters_to_token(b"JDY"),
	
	/// TODO.
	JDZ = Self::letters_to_token(b"JDZ"),
	
	/// TODO.
	JEA = Self::letters_to_token(b"JEA"),
	
	/// TODO.
	JEB = Self::letters_to_token(b"JEB"),
	
	/// TODO.
	JEC = Self::letters_to_token(b"JEC"),
	
	/// TODO.
	JED = Self::letters_to_token(b"JED"),
	
	/// TODO.
	JEE = Self::letters_to_token(b"JEE"),
	
	/// TODO.
	JEF = Self::letters_to_token(b"JEF"),
	
	/// TODO.
	JEG = Self::letters_to_token(b"JEG"),
	
	/// TODO.
	JEH = Self::letters_to_token(b"JEH"),
	
	/// TODO.
	JEI = Self::letters_to_token(b"JEI"),
	
	/// TODO.
	JEJ = Self::letters_to_token(b"JEJ"),
	
	/// TODO.
	JEK = Self::letters_to_token(b"JEK"),
	
	/// TODO.
	JEL = Self::letters_to_token(b"JEL"),
	
	/// TODO.
	JEM = Self::letters_to_token(b"JEM"),
	
	/// TODO.
	JEN = Self::letters_to_token(b"JEN"),
	
	/// TODO.
	JEO = Self::letters_to_token(b"JEO"),
	
	/// TODO.
	JEP = Self::letters_to_token(b"JEP"),
	
	/// TODO.
	JEQ = Self::letters_to_token(b"JEQ"),
	
	/// TODO.
	JER = Self::letters_to_token(b"JER"),
	
	/// TODO.
	JES = Self::letters_to_token(b"JES"),
	
	/// TODO.
	JET = Self::letters_to_token(b"JET"),
	
	/// TODO.
	JEU = Self::letters_to_token(b"JEU"),
	
	/// TODO.
	JEV = Self::letters_to_token(b"JEV"),
	
	/// TODO.
	JEW = Self::letters_to_token(b"JEW"),
	
	/// TODO.
	JEX = Self::letters_to_token(b"JEX"),
	
	/// Jersey.
	JEY = Self::letters_to_token(b"JEY"),
	
	/// TODO.
	JEZ = Self::letters_to_token(b"JEZ"),
	
	/// TODO.
	JFA = Self::letters_to_token(b"JFA"),
	
	/// TODO.
	JFB = Self::letters_to_token(b"JFB"),
	
	/// TODO.
	JFC = Self::letters_to_token(b"JFC"),
	
	/// TODO.
	JFD = Self::letters_to_token(b"JFD"),
	
	/// TODO.
	JFE = Self::letters_to_token(b"JFE"),
	
	/// TODO.
	JFF = Self::letters_to_token(b"JFF"),
	
	/// TODO.
	JFG = Self::letters_to_token(b"JFG"),
	
	/// TODO.
	JFH = Self::letters_to_token(b"JFH"),
	
	/// TODO.
	JFI = Self::letters_to_token(b"JFI"),
	
	/// TODO.
	JFJ = Self::letters_to_token(b"JFJ"),
	
	/// TODO.
	JFK = Self::letters_to_token(b"JFK"),
	
	/// TODO.
	JFL = Self::letters_to_token(b"JFL"),
	
	/// TODO.
	JFM = Self::letters_to_token(b"JFM"),
	
	/// TODO.
	JFN = Self::letters_to_token(b"JFN"),
	
	/// TODO.
	JFO = Self::letters_to_token(b"JFO"),
	
	/// TODO.
	JFP = Self::letters_to_token(b"JFP"),
	
	/// TODO.
	JFQ = Self::letters_to_token(b"JFQ"),
	
	/// TODO.
	JFR = Self::letters_to_token(b"JFR"),
	
	/// TODO.
	JFS = Self::letters_to_token(b"JFS"),
	
	/// TODO.
	JFT = Self::letters_to_token(b"JFT"),
	
	/// TODO.
	JFU = Self::letters_to_token(b"JFU"),
	
	/// TODO.
	JFV = Self::letters_to_token(b"JFV"),
	
	/// TODO.
	JFW = Self::letters_to_token(b"JFW"),
	
	/// TODO.
	JFX = Self::letters_to_token(b"JFX"),
	
	/// TODO.
	JFY = Self::letters_to_token(b"JFY"),
	
	/// TODO.
	JFZ = Self::letters_to_token(b"JFZ"),
	
	/// TODO.
	JGA = Self::letters_to_token(b"JGA"),
	
	/// TODO.
	JGB = Self::letters_to_token(b"JGB"),
	
	/// TODO.
	JGC = Self::letters_to_token(b"JGC"),
	
	/// TODO.
	JGD = Self::letters_to_token(b"JGD"),
	
	/// TODO.
	JGE = Self::letters_to_token(b"JGE"),
	
	/// TODO.
	JGF = Self::letters_to_token(b"JGF"),
	
	/// TODO.
	JGG = Self::letters_to_token(b"JGG"),
	
	/// TODO.
	JGH = Self::letters_to_token(b"JGH"),
	
	/// TODO.
	JGI = Self::letters_to_token(b"JGI"),
	
	/// TODO.
	JGJ = Self::letters_to_token(b"JGJ"),
	
	/// TODO.
	JGK = Self::letters_to_token(b"JGK"),
	
	/// TODO.
	JGL = Self::letters_to_token(b"JGL"),
	
	/// TODO.
	JGM = Self::letters_to_token(b"JGM"),
	
	/// TODO.
	JGN = Self::letters_to_token(b"JGN"),
	
	/// TODO.
	JGO = Self::letters_to_token(b"JGO"),
	
	/// TODO.
	JGP = Self::letters_to_token(b"JGP"),
	
	/// TODO.
	JGQ = Self::letters_to_token(b"JGQ"),
	
	/// TODO.
	JGR = Self::letters_to_token(b"JGR"),
	
	/// TODO.
	JGS = Self::letters_to_token(b"JGS"),
	
	/// TODO.
	JGT = Self::letters_to_token(b"JGT"),
	
	/// TODO.
	JGU = Self::letters_to_token(b"JGU"),
	
	/// TODO.
	JGV = Self::letters_to_token(b"JGV"),
	
	/// TODO.
	JGW = Self::letters_to_token(b"JGW"),
	
	/// TODO.
	JGX = Self::letters_to_token(b"JGX"),
	
	/// TODO.
	JGY = Self::letters_to_token(b"JGY"),
	
	/// TODO.
	JGZ = Self::letters_to_token(b"JGZ"),
	
	/// TODO.
	JHA = Self::letters_to_token(b"JHA"),
	
	/// TODO.
	JHB = Self::letters_to_token(b"JHB"),
	
	/// TODO.
	JHC = Self::letters_to_token(b"JHC"),
	
	/// TODO.
	JHD = Self::letters_to_token(b"JHD"),
	
	/// TODO.
	JHE = Self::letters_to_token(b"JHE"),
	
	/// TODO.
	JHF = Self::letters_to_token(b"JHF"),
	
	/// TODO.
	JHG = Self::letters_to_token(b"JHG"),
	
	/// TODO.
	JHH = Self::letters_to_token(b"JHH"),
	
	/// TODO.
	JHI = Self::letters_to_token(b"JHI"),
	
	/// TODO.
	JHJ = Self::letters_to_token(b"JHJ"),
	
	/// TODO.
	JHK = Self::letters_to_token(b"JHK"),
	
	/// TODO.
	JHL = Self::letters_to_token(b"JHL"),
	
	/// TODO.
	JHM = Self::letters_to_token(b"JHM"),
	
	/// TODO.
	JHN = Self::letters_to_token(b"JHN"),
	
	/// TODO.
	JHO = Self::letters_to_token(b"JHO"),
	
	/// TODO.
	JHP = Self::letters_to_token(b"JHP"),
	
	/// TODO.
	JHQ = Self::letters_to_token(b"JHQ"),
	
	/// TODO.
	JHR = Self::letters_to_token(b"JHR"),
	
	/// TODO.
	JHS = Self::letters_to_token(b"JHS"),
	
	/// TODO.
	JHT = Self::letters_to_token(b"JHT"),
	
	/// TODO.
	JHU = Self::letters_to_token(b"JHU"),
	
	/// TODO.
	JHV = Self::letters_to_token(b"JHV"),
	
	/// TODO.
	JHW = Self::letters_to_token(b"JHW"),
	
	/// TODO.
	JHX = Self::letters_to_token(b"JHX"),
	
	/// TODO.
	JHY = Self::letters_to_token(b"JHY"),
	
	/// TODO.
	JHZ = Self::letters_to_token(b"JHZ"),
	
	/// TODO.
	JIA = Self::letters_to_token(b"JIA"),
	
	/// TODO.
	JIB = Self::letters_to_token(b"JIB"),
	
	/// TODO.
	JIC = Self::letters_to_token(b"JIC"),
	
	/// TODO.
	JID = Self::letters_to_token(b"JID"),
	
	/// TODO.
	JIE = Self::letters_to_token(b"JIE"),
	
	/// TODO.
	JIF = Self::letters_to_token(b"JIF"),
	
	/// TODO.
	JIG = Self::letters_to_token(b"JIG"),
	
	/// TODO.
	JIH = Self::letters_to_token(b"JIH"),
	
	/// TODO.
	JII = Self::letters_to_token(b"JII"),
	
	/// TODO.
	JIJ = Self::letters_to_token(b"JIJ"),
	
	/// TODO.
	JIK = Self::letters_to_token(b"JIK"),
	
	/// TODO.
	JIL = Self::letters_to_token(b"JIL"),
	
	/// TODO.
	JIM = Self::letters_to_token(b"JIM"),
	
	/// TODO.
	JIN = Self::letters_to_token(b"JIN"),
	
	/// TODO.
	JIO = Self::letters_to_token(b"JIO"),
	
	/// TODO.
	JIP = Self::letters_to_token(b"JIP"),
	
	/// TODO.
	JIQ = Self::letters_to_token(b"JIQ"),
	
	/// TODO.
	JIR = Self::letters_to_token(b"JIR"),
	
	/// TODO.
	JIS = Self::letters_to_token(b"JIS"),
	
	/// TODO.
	JIT = Self::letters_to_token(b"JIT"),
	
	/// TODO.
	JIU = Self::letters_to_token(b"JIU"),
	
	/// TODO.
	JIV = Self::letters_to_token(b"JIV"),
	
	/// TODO.
	JIW = Self::letters_to_token(b"JIW"),
	
	/// TODO.
	JIX = Self::letters_to_token(b"JIX"),
	
	/// TODO.
	JIY = Self::letters_to_token(b"JIY"),
	
	/// TODO.
	JIZ = Self::letters_to_token(b"JIZ"),
	
	/// TODO.
	JJA = Self::letters_to_token(b"JJA"),
	
	/// TODO.
	JJB = Self::letters_to_token(b"JJB"),
	
	/// TODO.
	JJC = Self::letters_to_token(b"JJC"),
	
	/// TODO.
	JJD = Self::letters_to_token(b"JJD"),
	
	/// TODO.
	JJE = Self::letters_to_token(b"JJE"),
	
	/// TODO.
	JJF = Self::letters_to_token(b"JJF"),
	
	/// TODO.
	JJG = Self::letters_to_token(b"JJG"),
	
	/// TODO.
	JJH = Self::letters_to_token(b"JJH"),
	
	/// TODO.
	JJI = Self::letters_to_token(b"JJI"),
	
	/// TODO.
	JJJ = Self::letters_to_token(b"JJJ"),
	
	/// TODO.
	JJK = Self::letters_to_token(b"JJK"),
	
	/// TODO.
	JJL = Self::letters_to_token(b"JJL"),
	
	/// TODO.
	JJM = Self::letters_to_token(b"JJM"),
	
	/// TODO.
	JJN = Self::letters_to_token(b"JJN"),
	
	/// TODO.
	JJO = Self::letters_to_token(b"JJO"),
	
	/// TODO.
	JJP = Self::letters_to_token(b"JJP"),
	
	/// TODO.
	JJQ = Self::letters_to_token(b"JJQ"),
	
	/// TODO.
	JJR = Self::letters_to_token(b"JJR"),
	
	/// TODO.
	JJS = Self::letters_to_token(b"JJS"),
	
	/// TODO.
	JJT = Self::letters_to_token(b"JJT"),
	
	/// TODO.
	JJU = Self::letters_to_token(b"JJU"),
	
	/// TODO.
	JJV = Self::letters_to_token(b"JJV"),
	
	/// TODO.
	JJW = Self::letters_to_token(b"JJW"),
	
	/// TODO.
	JJX = Self::letters_to_token(b"JJX"),
	
	/// TODO.
	JJY = Self::letters_to_token(b"JJY"),
	
	/// TODO.
	JJZ = Self::letters_to_token(b"JJZ"),
	
	/// TODO.
	JKA = Self::letters_to_token(b"JKA"),
	
	/// TODO.
	JKB = Self::letters_to_token(b"JKB"),
	
	/// TODO.
	JKC = Self::letters_to_token(b"JKC"),
	
	/// TODO.
	JKD = Self::letters_to_token(b"JKD"),
	
	/// TODO.
	JKE = Self::letters_to_token(b"JKE"),
	
	/// TODO.
	JKF = Self::letters_to_token(b"JKF"),
	
	/// TODO.
	JKG = Self::letters_to_token(b"JKG"),
	
	/// TODO.
	JKH = Self::letters_to_token(b"JKH"),
	
	/// TODO.
	JKI = Self::letters_to_token(b"JKI"),
	
	/// TODO.
	JKJ = Self::letters_to_token(b"JKJ"),
	
	/// TODO.
	JKK = Self::letters_to_token(b"JKK"),
	
	/// TODO.
	JKL = Self::letters_to_token(b"JKL"),
	
	/// TODO.
	JKM = Self::letters_to_token(b"JKM"),
	
	/// TODO.
	JKN = Self::letters_to_token(b"JKN"),
	
	/// TODO.
	JKO = Self::letters_to_token(b"JKO"),
	
	/// TODO.
	JKP = Self::letters_to_token(b"JKP"),
	
	/// TODO.
	JKQ = Self::letters_to_token(b"JKQ"),
	
	/// TODO.
	JKR = Self::letters_to_token(b"JKR"),
	
	/// TODO.
	JKS = Self::letters_to_token(b"JKS"),
	
	/// TODO.
	JKT = Self::letters_to_token(b"JKT"),
	
	/// TODO.
	JKU = Self::letters_to_token(b"JKU"),
	
	/// TODO.
	JKV = Self::letters_to_token(b"JKV"),
	
	/// TODO.
	JKW = Self::letters_to_token(b"JKW"),
	
	/// TODO.
	JKX = Self::letters_to_token(b"JKX"),
	
	/// TODO.
	JKY = Self::letters_to_token(b"JKY"),
	
	/// TODO.
	JKZ = Self::letters_to_token(b"JKZ"),
	
	/// TODO.
	JLA = Self::letters_to_token(b"JLA"),
	
	/// TODO.
	JLB = Self::letters_to_token(b"JLB"),
	
	/// TODO.
	JLC = Self::letters_to_token(b"JLC"),
	
	/// TODO.
	JLD = Self::letters_to_token(b"JLD"),
	
	/// TODO.
	JLE = Self::letters_to_token(b"JLE"),
	
	/// TODO.
	JLF = Self::letters_to_token(b"JLF"),
	
	/// TODO.
	JLG = Self::letters_to_token(b"JLG"),
	
	/// TODO.
	JLH = Self::letters_to_token(b"JLH"),
	
	/// TODO.
	JLI = Self::letters_to_token(b"JLI"),
	
	/// TODO.
	JLJ = Self::letters_to_token(b"JLJ"),
	
	/// TODO.
	JLK = Self::letters_to_token(b"JLK"),
	
	/// TODO.
	JLL = Self::letters_to_token(b"JLL"),
	
	/// TODO.
	JLM = Self::letters_to_token(b"JLM"),
	
	/// TODO.
	JLN = Self::letters_to_token(b"JLN"),
	
	/// TODO.
	JLO = Self::letters_to_token(b"JLO"),
	
	/// TODO.
	JLP = Self::letters_to_token(b"JLP"),
	
	/// TODO.
	JLQ = Self::letters_to_token(b"JLQ"),
	
	/// TODO.
	JLR = Self::letters_to_token(b"JLR"),
	
	/// TODO.
	JLS = Self::letters_to_token(b"JLS"),
	
	/// TODO.
	JLT = Self::letters_to_token(b"JLT"),
	
	/// TODO.
	JLU = Self::letters_to_token(b"JLU"),
	
	/// TODO.
	JLV = Self::letters_to_token(b"JLV"),
	
	/// TODO.
	JLW = Self::letters_to_token(b"JLW"),
	
	/// TODO.
	JLX = Self::letters_to_token(b"JLX"),
	
	/// TODO.
	JLY = Self::letters_to_token(b"JLY"),
	
	/// TODO.
	JLZ = Self::letters_to_token(b"JLZ"),
	
	/// TODO.
	JMA = Self::letters_to_token(b"JMA"),
	
	/// TODO.
	JMB = Self::letters_to_token(b"JMB"),
	
	/// TODO.
	JMC = Self::letters_to_token(b"JMC"),
	
	/// TODO.
	JMD = Self::letters_to_token(b"JMD"),
	
	/// TODO.
	JME = Self::letters_to_token(b"JME"),
	
	/// TODO.
	JMF = Self::letters_to_token(b"JMF"),
	
	/// TODO.
	JMG = Self::letters_to_token(b"JMG"),
	
	/// TODO.
	JMH = Self::letters_to_token(b"JMH"),
	
	/// TODO.
	JMI = Self::letters_to_token(b"JMI"),
	
	/// TODO.
	JMJ = Self::letters_to_token(b"JMJ"),
	
	/// TODO.
	JMK = Self::letters_to_token(b"JMK"),
	
	/// TODO.
	JML = Self::letters_to_token(b"JML"),
	
	/// TODO.
	JMM = Self::letters_to_token(b"JMM"),
	
	/// TODO.
	JMN = Self::letters_to_token(b"JMN"),
	
	/// TODO.
	JMO = Self::letters_to_token(b"JMO"),
	
	/// TODO.
	JMP = Self::letters_to_token(b"JMP"),
	
	/// TODO.
	JMQ = Self::letters_to_token(b"JMQ"),
	
	/// TODO.
	JMR = Self::letters_to_token(b"JMR"),
	
	/// TODO.
	JMS = Self::letters_to_token(b"JMS"),
	
	/// TODO.
	JMT = Self::letters_to_token(b"JMT"),
	
	/// TODO.
	JMU = Self::letters_to_token(b"JMU"),
	
	/// TODO.
	JMV = Self::letters_to_token(b"JMV"),
	
	/// TODO.
	JMW = Self::letters_to_token(b"JMW"),
	
	/// TODO.
	JMX = Self::letters_to_token(b"JMX"),
	
	/// TODO.
	JMY = Self::letters_to_token(b"JMY"),
	
	/// TODO.
	JMZ = Self::letters_to_token(b"JMZ"),
	
	/// TODO.
	JNA = Self::letters_to_token(b"JNA"),
	
	/// TODO.
	JNB = Self::letters_to_token(b"JNB"),
	
	/// TODO.
	JNC = Self::letters_to_token(b"JNC"),
	
	/// TODO.
	JND = Self::letters_to_token(b"JND"),
	
	/// TODO.
	JNE = Self::letters_to_token(b"JNE"),
	
	/// TODO.
	JNF = Self::letters_to_token(b"JNF"),
	
	/// TODO.
	JNG = Self::letters_to_token(b"JNG"),
	
	/// TODO.
	JNH = Self::letters_to_token(b"JNH"),
	
	/// TODO.
	JNI = Self::letters_to_token(b"JNI"),
	
	/// TODO.
	JNJ = Self::letters_to_token(b"JNJ"),
	
	/// TODO.
	JNK = Self::letters_to_token(b"JNK"),
	
	/// TODO.
	JNL = Self::letters_to_token(b"JNL"),
	
	/// TODO.
	JNM = Self::letters_to_token(b"JNM"),
	
	/// TODO.
	JNN = Self::letters_to_token(b"JNN"),
	
	/// TODO.
	JNO = Self::letters_to_token(b"JNO"),
	
	/// TODO.
	JNP = Self::letters_to_token(b"JNP"),
	
	/// TODO.
	JNQ = Self::letters_to_token(b"JNQ"),
	
	/// TODO.
	JNR = Self::letters_to_token(b"JNR"),
	
	/// TODO.
	JNS = Self::letters_to_token(b"JNS"),
	
	/// TODO.
	JNT = Self::letters_to_token(b"JNT"),
	
	/// TODO.
	JNU = Self::letters_to_token(b"JNU"),
	
	/// TODO.
	JNV = Self::letters_to_token(b"JNV"),
	
	/// TODO.
	JNW = Self::letters_to_token(b"JNW"),
	
	/// TODO.
	JNX = Self::letters_to_token(b"JNX"),
	
	/// TODO.
	JNY = Self::letters_to_token(b"JNY"),
	
	/// TODO.
	JNZ = Self::letters_to_token(b"JNZ"),
	
	/// TODO.
	JOA = Self::letters_to_token(b"JOA"),
	
	/// TODO.
	JOB = Self::letters_to_token(b"JOB"),
	
	/// TODO.
	JOC = Self::letters_to_token(b"JOC"),
	
	/// TODO.
	JOD = Self::letters_to_token(b"JOD"),
	
	/// TODO.
	JOE = Self::letters_to_token(b"JOE"),
	
	/// TODO.
	JOF = Self::letters_to_token(b"JOF"),
	
	/// TODO.
	JOG = Self::letters_to_token(b"JOG"),
	
	/// TODO.
	JOH = Self::letters_to_token(b"JOH"),
	
	/// TODO.
	JOI = Self::letters_to_token(b"JOI"),
	
	/// TODO.
	JOJ = Self::letters_to_token(b"JOJ"),
	
	/// TODO.
	JOK = Self::letters_to_token(b"JOK"),
	
	/// TODO.
	JOL = Self::letters_to_token(b"JOL"),
	
	/// TODO.
	JOM = Self::letters_to_token(b"JOM"),
	
	/// TODO.
	JON = Self::letters_to_token(b"JON"),
	
	/// TODO.
	JOO = Self::letters_to_token(b"JOO"),
	
	/// TODO.
	JOP = Self::letters_to_token(b"JOP"),
	
	/// TODO.
	JOQ = Self::letters_to_token(b"JOQ"),
	
	/// TODO.
	JOR = Self::letters_to_token(b"JOR"),
	
	/// TODO.
	JOS = Self::letters_to_token(b"JOS"),
	
	/// TODO.
	JOT = Self::letters_to_token(b"JOT"),
	
	/// TODO.
	JOU = Self::letters_to_token(b"JOU"),
	
	/// TODO.
	JOV = Self::letters_to_token(b"JOV"),
	
	/// TODO.
	JOW = Self::letters_to_token(b"JOW"),
	
	/// TODO.
	JOX = Self::letters_to_token(b"JOX"),
	
	/// TODO.
	JOY = Self::letters_to_token(b"JOY"),
	
	/// TODO.
	JOZ = Self::letters_to_token(b"JOZ"),
	
	/// TODO.
	JPA = Self::letters_to_token(b"JPA"),
	
	/// TODO.
	JPB = Self::letters_to_token(b"JPB"),
	
	/// TODO.
	JPC = Self::letters_to_token(b"JPC"),
	
	/// TODO.
	JPD = Self::letters_to_token(b"JPD"),
	
	/// TODO.
	JPE = Self::letters_to_token(b"JPE"),
	
	/// TODO.
	JPF = Self::letters_to_token(b"JPF"),
	
	/// TODO.
	JPG = Self::letters_to_token(b"JPG"),
	
	/// TODO.
	JPH = Self::letters_to_token(b"JPH"),
	
	/// TODO.
	JPI = Self::letters_to_token(b"JPI"),
	
	/// TODO.
	JPJ = Self::letters_to_token(b"JPJ"),
	
	/// TODO.
	JPK = Self::letters_to_token(b"JPK"),
	
	/// TODO.
	JPL = Self::letters_to_token(b"JPL"),
	
	/// TODO.
	JPM = Self::letters_to_token(b"JPM"),
	
	/// TODO.
	JPN = Self::letters_to_token(b"JPN"),
	
	/// TODO.
	JPO = Self::letters_to_token(b"JPO"),
	
	/// TODO.
	JPP = Self::letters_to_token(b"JPP"),
	
	/// TODO.
	JPQ = Self::letters_to_token(b"JPQ"),
	
	/// TODO.
	JPR = Self::letters_to_token(b"JPR"),
	
	/// TODO.
	JPS = Self::letters_to_token(b"JPS"),
	
	/// TODO.
	JPT = Self::letters_to_token(b"JPT"),
	
	/// TODO.
	JPU = Self::letters_to_token(b"JPU"),
	
	/// TODO.
	JPV = Self::letters_to_token(b"JPV"),
	
	/// TODO.
	JPW = Self::letters_to_token(b"JPW"),
	
	/// TODO.
	JPX = Self::letters_to_token(b"JPX"),
	
	/// TODO.
	JPY = Self::letters_to_token(b"JPY"),
	
	/// TODO.
	JPZ = Self::letters_to_token(b"JPZ"),
	
	/// TODO.
	JQA = Self::letters_to_token(b"JQA"),
	
	/// TODO.
	JQB = Self::letters_to_token(b"JQB"),
	
	/// TODO.
	JQC = Self::letters_to_token(b"JQC"),
	
	/// TODO.
	JQD = Self::letters_to_token(b"JQD"),
	
	/// TODO.
	JQE = Self::letters_to_token(b"JQE"),
	
	/// TODO.
	JQF = Self::letters_to_token(b"JQF"),
	
	/// TODO.
	JQG = Self::letters_to_token(b"JQG"),
	
	/// TODO.
	JQH = Self::letters_to_token(b"JQH"),
	
	/// TODO.
	JQI = Self::letters_to_token(b"JQI"),
	
	/// TODO.
	JQJ = Self::letters_to_token(b"JQJ"),
	
	/// TODO.
	JQK = Self::letters_to_token(b"JQK"),
	
	/// TODO.
	JQL = Self::letters_to_token(b"JQL"),
	
	/// TODO.
	JQM = Self::letters_to_token(b"JQM"),
	
	/// TODO.
	JQN = Self::letters_to_token(b"JQN"),
	
	/// TODO.
	JQO = Self::letters_to_token(b"JQO"),
	
	/// TODO.
	JQP = Self::letters_to_token(b"JQP"),
	
	/// TODO.
	JQQ = Self::letters_to_token(b"JQQ"),
	
	/// TODO.
	JQR = Self::letters_to_token(b"JQR"),
	
	/// TODO.
	JQS = Self::letters_to_token(b"JQS"),
	
	/// TODO.
	JQT = Self::letters_to_token(b"JQT"),
	
	/// TODO.
	JQU = Self::letters_to_token(b"JQU"),
	
	/// TODO.
	JQV = Self::letters_to_token(b"JQV"),
	
	/// TODO.
	JQW = Self::letters_to_token(b"JQW"),
	
	/// TODO.
	JQX = Self::letters_to_token(b"JQX"),
	
	/// TODO.
	JQY = Self::letters_to_token(b"JQY"),
	
	/// TODO.
	JQZ = Self::letters_to_token(b"JQZ"),
	
	/// TODO.
	JRA = Self::letters_to_token(b"JRA"),
	
	/// TODO.
	JRB = Self::letters_to_token(b"JRB"),
	
	/// TODO.
	JRC = Self::letters_to_token(b"JRC"),
	
	/// TODO.
	JRD = Self::letters_to_token(b"JRD"),
	
	/// TODO.
	JRE = Self::letters_to_token(b"JRE"),
	
	/// TODO.
	JRF = Self::letters_to_token(b"JRF"),
	
	/// TODO.
	JRG = Self::letters_to_token(b"JRG"),
	
	/// TODO.
	JRH = Self::letters_to_token(b"JRH"),
	
	/// TODO.
	JRI = Self::letters_to_token(b"JRI"),
	
	/// TODO.
	JRJ = Self::letters_to_token(b"JRJ"),
	
	/// TODO.
	JRK = Self::letters_to_token(b"JRK"),
	
	/// TODO.
	JRL = Self::letters_to_token(b"JRL"),
	
	/// TODO.
	JRM = Self::letters_to_token(b"JRM"),
	
	/// TODO.
	JRN = Self::letters_to_token(b"JRN"),
	
	/// TODO.
	JRO = Self::letters_to_token(b"JRO"),
	
	/// TODO.
	JRP = Self::letters_to_token(b"JRP"),
	
	/// TODO.
	JRQ = Self::letters_to_token(b"JRQ"),
	
	/// TODO.
	JRR = Self::letters_to_token(b"JRR"),
	
	/// TODO.
	JRS = Self::letters_to_token(b"JRS"),
	
	/// TODO.
	JRT = Self::letters_to_token(b"JRT"),
	
	/// TODO.
	JRU = Self::letters_to_token(b"JRU"),
	
	/// TODO.
	JRV = Self::letters_to_token(b"JRV"),
	
	/// TODO.
	JRW = Self::letters_to_token(b"JRW"),
	
	/// TODO.
	JRX = Self::letters_to_token(b"JRX"),
	
	/// TODO.
	JRY = Self::letters_to_token(b"JRY"),
	
	/// TODO.
	JRZ = Self::letters_to_token(b"JRZ"),
	
	/// TODO.
	JSA = Self::letters_to_token(b"JSA"),
	
	/// TODO.
	JSB = Self::letters_to_token(b"JSB"),
	
	/// TODO.
	JSC = Self::letters_to_token(b"JSC"),
	
	/// TODO.
	JSD = Self::letters_to_token(b"JSD"),
	
	/// TODO.
	JSE = Self::letters_to_token(b"JSE"),
	
	/// TODO.
	JSF = Self::letters_to_token(b"JSF"),
	
	/// TODO.
	JSG = Self::letters_to_token(b"JSG"),
	
	/// TODO.
	JSH = Self::letters_to_token(b"JSH"),
	
	/// TODO.
	JSI = Self::letters_to_token(b"JSI"),
	
	/// TODO.
	JSJ = Self::letters_to_token(b"JSJ"),
	
	/// TODO.
	JSK = Self::letters_to_token(b"JSK"),
	
	/// TODO.
	JSL = Self::letters_to_token(b"JSL"),
	
	/// TODO.
	JSM = Self::letters_to_token(b"JSM"),
	
	/// TODO.
	JSN = Self::letters_to_token(b"JSN"),
	
	/// TODO.
	JSO = Self::letters_to_token(b"JSO"),
	
	/// TODO.
	JSP = Self::letters_to_token(b"JSP"),
	
	/// TODO.
	JSQ = Self::letters_to_token(b"JSQ"),
	
	/// TODO.
	JSR = Self::letters_to_token(b"JSR"),
	
	/// TODO.
	JSS = Self::letters_to_token(b"JSS"),
	
	/// TODO.
	JST = Self::letters_to_token(b"JST"),
	
	/// TODO.
	JSU = Self::letters_to_token(b"JSU"),
	
	/// TODO.
	JSV = Self::letters_to_token(b"JSV"),
	
	/// TODO.
	JSW = Self::letters_to_token(b"JSW"),
	
	/// TODO.
	JSX = Self::letters_to_token(b"JSX"),
	
	/// TODO.
	JSY = Self::letters_to_token(b"JSY"),
	
	/// TODO.
	JSZ = Self::letters_to_token(b"JSZ"),
	
	/// TODO.
	JTA = Self::letters_to_token(b"JTA"),
	
	/// TODO.
	JTB = Self::letters_to_token(b"JTB"),
	
	/// TODO.
	JTC = Self::letters_to_token(b"JTC"),
	
	/// TODO.
	JTD = Self::letters_to_token(b"JTD"),
	
	/// TODO.
	JTE = Self::letters_to_token(b"JTE"),
	
	/// TODO.
	JTF = Self::letters_to_token(b"JTF"),
	
	/// TODO.
	JTG = Self::letters_to_token(b"JTG"),
	
	/// TODO.
	JTH = Self::letters_to_token(b"JTH"),
	
	/// TODO.
	JTI = Self::letters_to_token(b"JTI"),
	
	/// TODO.
	JTJ = Self::letters_to_token(b"JTJ"),
	
	/// TODO.
	JTK = Self::letters_to_token(b"JTK"),
	
	/// TODO.
	JTL = Self::letters_to_token(b"JTL"),
	
	/// TODO.
	JTM = Self::letters_to_token(b"JTM"),
	
	/// TODO.
	JTN = Self::letters_to_token(b"JTN"),
	
	/// TODO.
	JTO = Self::letters_to_token(b"JTO"),
	
	/// TODO.
	JTP = Self::letters_to_token(b"JTP"),
	
	/// TODO.
	JTQ = Self::letters_to_token(b"JTQ"),
	
	/// TODO.
	JTR = Self::letters_to_token(b"JTR"),
	
	/// TODO.
	JTS = Self::letters_to_token(b"JTS"),
	
	/// TODO.
	JTT = Self::letters_to_token(b"JTT"),
	
	/// TODO.
	JTU = Self::letters_to_token(b"JTU"),
	
	/// TODO.
	JTV = Self::letters_to_token(b"JTV"),
	
	/// TODO.
	JTW = Self::letters_to_token(b"JTW"),
	
	/// TODO.
	JTX = Self::letters_to_token(b"JTX"),
	
	/// TODO.
	JTY = Self::letters_to_token(b"JTY"),
	
	/// TODO.
	JTZ = Self::letters_to_token(b"JTZ"),
	
	/// TODO.
	JUA = Self::letters_to_token(b"JUA"),
	
	/// TODO.
	JUB = Self::letters_to_token(b"JUB"),
	
	/// TODO.
	JUC = Self::letters_to_token(b"JUC"),
	
	/// TODO.
	JUD = Self::letters_to_token(b"JUD"),
	
	/// TODO.
	JUE = Self::letters_to_token(b"JUE"),
	
	/// TODO.
	JUF = Self::letters_to_token(b"JUF"),
	
	/// TODO.
	JUG = Self::letters_to_token(b"JUG"),
	
	/// TODO.
	JUH = Self::letters_to_token(b"JUH"),
	
	/// TODO.
	JUI = Self::letters_to_token(b"JUI"),
	
	/// TODO.
	JUJ = Self::letters_to_token(b"JUJ"),
	
	/// TODO.
	JUK = Self::letters_to_token(b"JUK"),
	
	/// TODO.
	JUL = Self::letters_to_token(b"JUL"),
	
	/// TODO.
	JUM = Self::letters_to_token(b"JUM"),
	
	/// TODO.
	JUN = Self::letters_to_token(b"JUN"),
	
	/// TODO.
	JUO = Self::letters_to_token(b"JUO"),
	
	/// TODO.
	JUP = Self::letters_to_token(b"JUP"),
	
	/// TODO.
	JUQ = Self::letters_to_token(b"JUQ"),
	
	/// TODO.
	JUR = Self::letters_to_token(b"JUR"),
	
	/// TODO.
	JUS = Self::letters_to_token(b"JUS"),
	
	/// TODO.
	JUT = Self::letters_to_token(b"JUT"),
	
	/// TODO.
	JUU = Self::letters_to_token(b"JUU"),
	
	/// TODO.
	JUV = Self::letters_to_token(b"JUV"),
	
	/// TODO.
	JUW = Self::letters_to_token(b"JUW"),
	
	/// TODO.
	JUX = Self::letters_to_token(b"JUX"),
	
	/// TODO.
	JUY = Self::letters_to_token(b"JUY"),
	
	/// TODO.
	JUZ = Self::letters_to_token(b"JUZ"),
	
	/// TODO.
	JVA = Self::letters_to_token(b"JVA"),
	
	/// TODO.
	JVB = Self::letters_to_token(b"JVB"),
	
	/// TODO.
	JVC = Self::letters_to_token(b"JVC"),
	
	/// TODO.
	JVD = Self::letters_to_token(b"JVD"),
	
	/// TODO.
	JVE = Self::letters_to_token(b"JVE"),
	
	/// TODO.
	JVF = Self::letters_to_token(b"JVF"),
	
	/// TODO.
	JVG = Self::letters_to_token(b"JVG"),
	
	/// TODO.
	JVH = Self::letters_to_token(b"JVH"),
	
	/// TODO.
	JVI = Self::letters_to_token(b"JVI"),
	
	/// TODO.
	JVJ = Self::letters_to_token(b"JVJ"),
	
	/// TODO.
	JVK = Self::letters_to_token(b"JVK"),
	
	/// TODO.
	JVL = Self::letters_to_token(b"JVL"),
	
	/// TODO.
	JVM = Self::letters_to_token(b"JVM"),
	
	/// TODO.
	JVN = Self::letters_to_token(b"JVN"),
	
	/// TODO.
	JVO = Self::letters_to_token(b"JVO"),
	
	/// TODO.
	JVP = Self::letters_to_token(b"JVP"),
	
	/// TODO.
	JVQ = Self::letters_to_token(b"JVQ"),
	
	/// TODO.
	JVR = Self::letters_to_token(b"JVR"),
	
	/// TODO.
	JVS = Self::letters_to_token(b"JVS"),
	
	/// TODO.
	JVT = Self::letters_to_token(b"JVT"),
	
	/// TODO.
	JVU = Self::letters_to_token(b"JVU"),
	
	/// TODO.
	JVV = Self::letters_to_token(b"JVV"),
	
	/// TODO.
	JVW = Self::letters_to_token(b"JVW"),
	
	/// TODO.
	JVX = Self::letters_to_token(b"JVX"),
	
	/// TODO.
	JVY = Self::letters_to_token(b"JVY"),
	
	/// TODO.
	JVZ = Self::letters_to_token(b"JVZ"),
	
	/// TODO.
	JWA = Self::letters_to_token(b"JWA"),
	
	/// TODO.
	JWB = Self::letters_to_token(b"JWB"),
	
	/// TODO.
	JWC = Self::letters_to_token(b"JWC"),
	
	/// TODO.
	JWD = Self::letters_to_token(b"JWD"),
	
	/// TODO.
	JWE = Self::letters_to_token(b"JWE"),
	
	/// TODO.
	JWF = Self::letters_to_token(b"JWF"),
	
	/// TODO.
	JWG = Self::letters_to_token(b"JWG"),
	
	/// TODO.
	JWH = Self::letters_to_token(b"JWH"),
	
	/// TODO.
	JWI = Self::letters_to_token(b"JWI"),
	
	/// TODO.
	JWJ = Self::letters_to_token(b"JWJ"),
	
	/// TODO.
	JWK = Self::letters_to_token(b"JWK"),
	
	/// TODO.
	JWL = Self::letters_to_token(b"JWL"),
	
	/// TODO.
	JWM = Self::letters_to_token(b"JWM"),
	
	/// TODO.
	JWN = Self::letters_to_token(b"JWN"),
	
	/// TODO.
	JWO = Self::letters_to_token(b"JWO"),
	
	/// TODO.
	JWP = Self::letters_to_token(b"JWP"),
	
	/// TODO.
	JWQ = Self::letters_to_token(b"JWQ"),
	
	/// TODO.
	JWR = Self::letters_to_token(b"JWR"),
	
	/// TODO.
	JWS = Self::letters_to_token(b"JWS"),
	
	/// TODO.
	JWT = Self::letters_to_token(b"JWT"),
	
	/// TODO.
	JWU = Self::letters_to_token(b"JWU"),
	
	/// TODO.
	JWV = Self::letters_to_token(b"JWV"),
	
	/// TODO.
	JWW = Self::letters_to_token(b"JWW"),
	
	/// TODO.
	JWX = Self::letters_to_token(b"JWX"),
	
	/// TODO.
	JWY = Self::letters_to_token(b"JWY"),
	
	/// TODO.
	JWZ = Self::letters_to_token(b"JWZ"),
	
	/// TODO.
	JXA = Self::letters_to_token(b"JXA"),
	
	/// TODO.
	JXB = Self::letters_to_token(b"JXB"),
	
	/// TODO.
	JXC = Self::letters_to_token(b"JXC"),
	
	/// TODO.
	JXD = Self::letters_to_token(b"JXD"),
	
	/// TODO.
	JXE = Self::letters_to_token(b"JXE"),
	
	/// TODO.
	JXF = Self::letters_to_token(b"JXF"),
	
	/// TODO.
	JXG = Self::letters_to_token(b"JXG"),
	
	/// TODO.
	JXH = Self::letters_to_token(b"JXH"),
	
	/// TODO.
	JXI = Self::letters_to_token(b"JXI"),
	
	/// TODO.
	JXJ = Self::letters_to_token(b"JXJ"),
	
	/// TODO.
	JXK = Self::letters_to_token(b"JXK"),
	
	/// TODO.
	JXL = Self::letters_to_token(b"JXL"),
	
	/// TODO.
	JXM = Self::letters_to_token(b"JXM"),
	
	/// TODO.
	JXN = Self::letters_to_token(b"JXN"),
	
	/// TODO.
	JXO = Self::letters_to_token(b"JXO"),
	
	/// TODO.
	JXP = Self::letters_to_token(b"JXP"),
	
	/// TODO.
	JXQ = Self::letters_to_token(b"JXQ"),
	
	/// TODO.
	JXR = Self::letters_to_token(b"JXR"),
	
	/// TODO.
	JXS = Self::letters_to_token(b"JXS"),
	
	/// TODO.
	JXT = Self::letters_to_token(b"JXT"),
	
	/// TODO.
	JXU = Self::letters_to_token(b"JXU"),
	
	/// TODO.
	JXV = Self::letters_to_token(b"JXV"),
	
	/// TODO.
	JXW = Self::letters_to_token(b"JXW"),
	
	/// TODO.
	JXX = Self::letters_to_token(b"JXX"),
	
	/// TODO.
	JXY = Self::letters_to_token(b"JXY"),
	
	/// TODO.
	JXZ = Self::letters_to_token(b"JXZ"),
	
	/// TODO.
	JYA = Self::letters_to_token(b"JYA"),
	
	/// TODO.
	JYB = Self::letters_to_token(b"JYB"),
	
	/// TODO.
	JYC = Self::letters_to_token(b"JYC"),
	
	/// TODO.
	JYD = Self::letters_to_token(b"JYD"),
	
	/// TODO.
	JYE = Self::letters_to_token(b"JYE"),
	
	/// TODO.
	JYF = Self::letters_to_token(b"JYF"),
	
	/// TODO.
	JYG = Self::letters_to_token(b"JYG"),
	
	/// TODO.
	JYH = Self::letters_to_token(b"JYH"),
	
	/// TODO.
	JYI = Self::letters_to_token(b"JYI"),
	
	/// TODO.
	JYJ = Self::letters_to_token(b"JYJ"),
	
	/// TODO.
	JYK = Self::letters_to_token(b"JYK"),
	
	/// TODO.
	JYL = Self::letters_to_token(b"JYL"),
	
	/// TODO.
	JYM = Self::letters_to_token(b"JYM"),
	
	/// TODO.
	JYN = Self::letters_to_token(b"JYN"),
	
	/// TODO.
	JYO = Self::letters_to_token(b"JYO"),
	
	/// TODO.
	JYP = Self::letters_to_token(b"JYP"),
	
	/// TODO.
	JYQ = Self::letters_to_token(b"JYQ"),
	
	/// TODO.
	JYR = Self::letters_to_token(b"JYR"),
	
	/// TODO.
	JYS = Self::letters_to_token(b"JYS"),
	
	/// TODO.
	JYT = Self::letters_to_token(b"JYT"),
	
	/// TODO.
	JYU = Self::letters_to_token(b"JYU"),
	
	/// TODO.
	JYV = Self::letters_to_token(b"JYV"),
	
	/// TODO.
	JYW = Self::letters_to_token(b"JYW"),
	
	/// TODO.
	JYX = Self::letters_to_token(b"JYX"),
	
	/// TODO.
	JYY = Self::letters_to_token(b"JYY"),
	
	/// TODO.
	JYZ = Self::letters_to_token(b"JYZ"),
	
	/// TODO.
	JZA = Self::letters_to_token(b"JZA"),
	
	/// TODO.
	JZB = Self::letters_to_token(b"JZB"),
	
	/// TODO.
	JZC = Self::letters_to_token(b"JZC"),
	
	/// TODO.
	JZD = Self::letters_to_token(b"JZD"),
	
	/// TODO.
	JZE = Self::letters_to_token(b"JZE"),
	
	/// TODO.
	JZF = Self::letters_to_token(b"JZF"),
	
	/// TODO.
	JZG = Self::letters_to_token(b"JZG"),
	
	/// TODO.
	JZH = Self::letters_to_token(b"JZH"),
	
	/// TODO.
	JZI = Self::letters_to_token(b"JZI"),
	
	/// TODO.
	JZJ = Self::letters_to_token(b"JZJ"),
	
	/// TODO.
	JZK = Self::letters_to_token(b"JZK"),
	
	/// TODO.
	JZL = Self::letters_to_token(b"JZL"),
	
	/// TODO.
	JZM = Self::letters_to_token(b"JZM"),
	
	/// TODO.
	JZN = Self::letters_to_token(b"JZN"),
	
	/// TODO.
	JZO = Self::letters_to_token(b"JZO"),
	
	/// TODO.
	JZP = Self::letters_to_token(b"JZP"),
	
	/// TODO.
	JZQ = Self::letters_to_token(b"JZQ"),
	
	/// TODO.
	JZR = Self::letters_to_token(b"JZR"),
	
	/// TODO.
	JZS = Self::letters_to_token(b"JZS"),
	
	/// TODO.
	JZT = Self::letters_to_token(b"JZT"),
	
	/// TODO.
	JZU = Self::letters_to_token(b"JZU"),
	
	/// TODO.
	JZV = Self::letters_to_token(b"JZV"),
	
	/// TODO.
	JZW = Self::letters_to_token(b"JZW"),
	
	/// TODO.
	JZX = Self::letters_to_token(b"JZX"),
	
	/// TODO.
	JZY = Self::letters_to_token(b"JZY"),
	
	/// TODO.
	JZZ = Self::letters_to_token(b"JZZ"),
	
	/// TODO.
	KAA = Self::letters_to_token(b"KAA"),
	
	/// TODO.
	KAB = Self::letters_to_token(b"KAB"),
	
	/// TODO.
	KAC = Self::letters_to_token(b"KAC"),
	
	/// TODO.
	KAD = Self::letters_to_token(b"KAD"),
	
	/// TODO.
	KAE = Self::letters_to_token(b"KAE"),
	
	/// TODO.
	KAF = Self::letters_to_token(b"KAF"),
	
	/// TODO.
	KAG = Self::letters_to_token(b"KAG"),
	
	/// TODO.
	KAH = Self::letters_to_token(b"KAH"),
	
	/// TODO.
	KAI = Self::letters_to_token(b"KAI"),
	
	/// TODO.
	KAJ = Self::letters_to_token(b"KAJ"),
	
	/// TODO.
	KAK = Self::letters_to_token(b"KAK"),
	
	/// TODO.
	KAL = Self::letters_to_token(b"KAL"),
	
	/// TODO.
	KAM = Self::letters_to_token(b"KAM"),
	
	/// TODO.
	KAN = Self::letters_to_token(b"KAN"),
	
	/// TODO.
	KAO = Self::letters_to_token(b"KAO"),
	
	/// TODO.
	KAP = Self::letters_to_token(b"KAP"),
	
	/// TODO.
	KAQ = Self::letters_to_token(b"KAQ"),
	
	/// TODO.
	KAR = Self::letters_to_token(b"KAR"),
	
	/// TODO.
	KAS = Self::letters_to_token(b"KAS"),
	
	/// TODO.
	KAT = Self::letters_to_token(b"KAT"),
	
	/// TODO.
	KAU = Self::letters_to_token(b"KAU"),
	
	/// TODO.
	KAV = Self::letters_to_token(b"KAV"),
	
	/// TODO.
	KAW = Self::letters_to_token(b"KAW"),
	
	/// TODO.
	KAX = Self::letters_to_token(b"KAX"),
	
	/// TODO.
	KAY = Self::letters_to_token(b"KAY"),
	
	/// TODO.
	KAZ = Self::letters_to_token(b"KAZ"),
	
	/// TODO.
	KBA = Self::letters_to_token(b"KBA"),
	
	/// TODO.
	KBB = Self::letters_to_token(b"KBB"),
	
	/// TODO.
	KBC = Self::letters_to_token(b"KBC"),
	
	/// TODO.
	KBD = Self::letters_to_token(b"KBD"),
	
	/// TODO.
	KBE = Self::letters_to_token(b"KBE"),
	
	/// TODO.
	KBF = Self::letters_to_token(b"KBF"),
	
	/// TODO.
	KBG = Self::letters_to_token(b"KBG"),
	
	/// TODO.
	KBH = Self::letters_to_token(b"KBH"),
	
	/// TODO.
	KBI = Self::letters_to_token(b"KBI"),
	
	/// TODO.
	KBJ = Self::letters_to_token(b"KBJ"),
	
	/// TODO.
	KBK = Self::letters_to_token(b"KBK"),
	
	/// TODO.
	KBL = Self::letters_to_token(b"KBL"),
	
	/// TODO.
	KBM = Self::letters_to_token(b"KBM"),
	
	/// TODO.
	KBN = Self::letters_to_token(b"KBN"),
	
	/// TODO.
	KBO = Self::letters_to_token(b"KBO"),
	
	/// TODO.
	KBP = Self::letters_to_token(b"KBP"),
	
	/// TODO.
	KBQ = Self::letters_to_token(b"KBQ"),
	
	/// TODO.
	KBR = Self::letters_to_token(b"KBR"),
	
	/// TODO.
	KBS = Self::letters_to_token(b"KBS"),
	
	/// TODO.
	KBT = Self::letters_to_token(b"KBT"),
	
	/// TODO.
	KBU = Self::letters_to_token(b"KBU"),
	
	/// TODO.
	KBV = Self::letters_to_token(b"KBV"),
	
	/// TODO.
	KBW = Self::letters_to_token(b"KBW"),
	
	/// TODO.
	KBX = Self::letters_to_token(b"KBX"),
	
	/// TODO.
	KBY = Self::letters_to_token(b"KBY"),
	
	/// TODO.
	KBZ = Self::letters_to_token(b"KBZ"),
	
	/// TODO.
	KCA = Self::letters_to_token(b"KCA"),
	
	/// TODO.
	KCB = Self::letters_to_token(b"KCB"),
	
	/// TODO.
	KCC = Self::letters_to_token(b"KCC"),
	
	/// TODO.
	KCD = Self::letters_to_token(b"KCD"),
	
	/// TODO.
	KCE = Self::letters_to_token(b"KCE"),
	
	/// TODO.
	KCF = Self::letters_to_token(b"KCF"),
	
	/// TODO.
	KCG = Self::letters_to_token(b"KCG"),
	
	/// TODO.
	KCH = Self::letters_to_token(b"KCH"),
	
	/// TODO.
	KCI = Self::letters_to_token(b"KCI"),
	
	/// TODO.
	KCJ = Self::letters_to_token(b"KCJ"),
	
	/// TODO.
	KCK = Self::letters_to_token(b"KCK"),
	
	/// TODO.
	KCL = Self::letters_to_token(b"KCL"),
	
	/// TODO.
	KCM = Self::letters_to_token(b"KCM"),
	
	/// TODO.
	KCN = Self::letters_to_token(b"KCN"),
	
	/// TODO.
	KCO = Self::letters_to_token(b"KCO"),
	
	/// TODO.
	KCP = Self::letters_to_token(b"KCP"),
	
	/// TODO.
	KCQ = Self::letters_to_token(b"KCQ"),
	
	/// TODO.
	KCR = Self::letters_to_token(b"KCR"),
	
	/// TODO.
	KCS = Self::letters_to_token(b"KCS"),
	
	/// TODO.
	KCT = Self::letters_to_token(b"KCT"),
	
	/// TODO.
	KCU = Self::letters_to_token(b"KCU"),
	
	/// TODO.
	KCV = Self::letters_to_token(b"KCV"),
	
	/// TODO.
	KCW = Self::letters_to_token(b"KCW"),
	
	/// TODO.
	KCX = Self::letters_to_token(b"KCX"),
	
	/// TODO.
	KCY = Self::letters_to_token(b"KCY"),
	
	/// TODO.
	KCZ = Self::letters_to_token(b"KCZ"),
	
	/// TODO.
	KDA = Self::letters_to_token(b"KDA"),
	
	/// TODO.
	KDB = Self::letters_to_token(b"KDB"),
	
	/// TODO.
	KDC = Self::letters_to_token(b"KDC"),
	
	/// TODO.
	KDD = Self::letters_to_token(b"KDD"),
	
	/// TODO.
	KDE = Self::letters_to_token(b"KDE"),
	
	/// TODO.
	KDF = Self::letters_to_token(b"KDF"),
	
	/// TODO.
	KDG = Self::letters_to_token(b"KDG"),
	
	/// TODO.
	KDH = Self::letters_to_token(b"KDH"),
	
	/// TODO.
	KDI = Self::letters_to_token(b"KDI"),
	
	/// TODO.
	KDJ = Self::letters_to_token(b"KDJ"),
	
	/// TODO.
	KDK = Self::letters_to_token(b"KDK"),
	
	/// TODO.
	KDL = Self::letters_to_token(b"KDL"),
	
	/// TODO.
	KDM = Self::letters_to_token(b"KDM"),
	
	/// TODO.
	KDN = Self::letters_to_token(b"KDN"),
	
	/// TODO.
	KDO = Self::letters_to_token(b"KDO"),
	
	/// TODO.
	KDP = Self::letters_to_token(b"KDP"),
	
	/// TODO.
	KDQ = Self::letters_to_token(b"KDQ"),
	
	/// TODO.
	KDR = Self::letters_to_token(b"KDR"),
	
	/// TODO.
	KDS = Self::letters_to_token(b"KDS"),
	
	/// TODO.
	KDT = Self::letters_to_token(b"KDT"),
	
	/// TODO.
	KDU = Self::letters_to_token(b"KDU"),
	
	/// TODO.
	KDV = Self::letters_to_token(b"KDV"),
	
	/// TODO.
	KDW = Self::letters_to_token(b"KDW"),
	
	/// TODO.
	KDX = Self::letters_to_token(b"KDX"),
	
	/// TODO.
	KDY = Self::letters_to_token(b"KDY"),
	
	/// TODO.
	KDZ = Self::letters_to_token(b"KDZ"),
	
	/// TODO.
	KEA = Self::letters_to_token(b"KEA"),
	
	/// TODO.
	KEB = Self::letters_to_token(b"KEB"),
	
	/// TODO.
	KEC = Self::letters_to_token(b"KEC"),
	
	/// TODO.
	KED = Self::letters_to_token(b"KED"),
	
	/// TODO.
	KEE = Self::letters_to_token(b"KEE"),
	
	/// TODO.
	KEF = Self::letters_to_token(b"KEF"),
	
	/// TODO.
	KEG = Self::letters_to_token(b"KEG"),
	
	/// TODO.
	KEH = Self::letters_to_token(b"KEH"),
	
	/// TODO.
	KEI = Self::letters_to_token(b"KEI"),
	
	/// TODO.
	KEJ = Self::letters_to_token(b"KEJ"),
	
	/// TODO.
	KEK = Self::letters_to_token(b"KEK"),
	
	/// TODO.
	KEL = Self::letters_to_token(b"KEL"),
	
	/// TODO.
	KEM = Self::letters_to_token(b"KEM"),
	
	/// TODO.
	KEN = Self::letters_to_token(b"KEN"),
	
	/// TODO.
	KEO = Self::letters_to_token(b"KEO"),
	
	/// TODO.
	KEP = Self::letters_to_token(b"KEP"),
	
	/// TODO.
	KEQ = Self::letters_to_token(b"KEQ"),
	
	/// TODO.
	KER = Self::letters_to_token(b"KER"),
	
	/// TODO.
	KES = Self::letters_to_token(b"KES"),
	
	/// TODO.
	KET = Self::letters_to_token(b"KET"),
	
	/// TODO.
	KEU = Self::letters_to_token(b"KEU"),
	
	/// TODO.
	KEV = Self::letters_to_token(b"KEV"),
	
	/// TODO.
	KEW = Self::letters_to_token(b"KEW"),
	
	/// TODO.
	KEX = Self::letters_to_token(b"KEX"),
	
	/// TODO.
	KEY = Self::letters_to_token(b"KEY"),
	
	/// TODO.
	KEZ = Self::letters_to_token(b"KEZ"),
	
	/// TODO.
	KFA = Self::letters_to_token(b"KFA"),
	
	/// TODO.
	KFB = Self::letters_to_token(b"KFB"),
	
	/// TODO.
	KFC = Self::letters_to_token(b"KFC"),
	
	/// TODO.
	KFD = Self::letters_to_token(b"KFD"),
	
	/// TODO.
	KFE = Self::letters_to_token(b"KFE"),
	
	/// TODO.
	KFF = Self::letters_to_token(b"KFF"),
	
	/// TODO.
	KFG = Self::letters_to_token(b"KFG"),
	
	/// TODO.
	KFH = Self::letters_to_token(b"KFH"),
	
	/// TODO.
	KFI = Self::letters_to_token(b"KFI"),
	
	/// TODO.
	KFJ = Self::letters_to_token(b"KFJ"),
	
	/// TODO.
	KFK = Self::letters_to_token(b"KFK"),
	
	/// TODO.
	KFL = Self::letters_to_token(b"KFL"),
	
	/// TODO.
	KFM = Self::letters_to_token(b"KFM"),
	
	/// TODO.
	KFN = Self::letters_to_token(b"KFN"),
	
	/// TODO.
	KFO = Self::letters_to_token(b"KFO"),
	
	/// TODO.
	KFP = Self::letters_to_token(b"KFP"),
	
	/// TODO.
	KFQ = Self::letters_to_token(b"KFQ"),
	
	/// TODO.
	KFR = Self::letters_to_token(b"KFR"),
	
	/// TODO.
	KFS = Self::letters_to_token(b"KFS"),
	
	/// TODO.
	KFT = Self::letters_to_token(b"KFT"),
	
	/// TODO.
	KFU = Self::letters_to_token(b"KFU"),
	
	/// TODO.
	KFV = Self::letters_to_token(b"KFV"),
	
	/// TODO.
	KFW = Self::letters_to_token(b"KFW"),
	
	/// TODO.
	KFX = Self::letters_to_token(b"KFX"),
	
	/// TODO.
	KFY = Self::letters_to_token(b"KFY"),
	
	/// TODO.
	KFZ = Self::letters_to_token(b"KFZ"),
	
	/// TODO.
	KGA = Self::letters_to_token(b"KGA"),
	
	/// TODO.
	KGB = Self::letters_to_token(b"KGB"),
	
	/// TODO.
	KGC = Self::letters_to_token(b"KGC"),
	
	/// TODO.
	KGD = Self::letters_to_token(b"KGD"),
	
	/// TODO.
	KGE = Self::letters_to_token(b"KGE"),
	
	/// TODO.
	KGF = Self::letters_to_token(b"KGF"),
	
	/// TODO.
	KGG = Self::letters_to_token(b"KGG"),
	
	/// TODO.
	KGH = Self::letters_to_token(b"KGH"),
	
	/// TODO.
	KGI = Self::letters_to_token(b"KGI"),
	
	/// TODO.
	KGJ = Self::letters_to_token(b"KGJ"),
	
	/// TODO.
	KGK = Self::letters_to_token(b"KGK"),
	
	/// TODO.
	KGL = Self::letters_to_token(b"KGL"),
	
	/// TODO.
	KGM = Self::letters_to_token(b"KGM"),
	
	/// TODO.
	KGN = Self::letters_to_token(b"KGN"),
	
	/// TODO.
	KGO = Self::letters_to_token(b"KGO"),
	
	/// TODO.
	KGP = Self::letters_to_token(b"KGP"),
	
	/// TODO.
	KGQ = Self::letters_to_token(b"KGQ"),
	
	/// TODO.
	KGR = Self::letters_to_token(b"KGR"),
	
	/// TODO.
	KGS = Self::letters_to_token(b"KGS"),
	
	/// TODO.
	KGT = Self::letters_to_token(b"KGT"),
	
	/// TODO.
	KGU = Self::letters_to_token(b"KGU"),
	
	/// TODO.
	KGV = Self::letters_to_token(b"KGV"),
	
	/// TODO.
	KGW = Self::letters_to_token(b"KGW"),
	
	/// TODO.
	KGX = Self::letters_to_token(b"KGX"),
	
	/// TODO.
	KGY = Self::letters_to_token(b"KGY"),
	
	/// TODO.
	KGZ = Self::letters_to_token(b"KGZ"),
	
	/// TODO.
	KHA = Self::letters_to_token(b"KHA"),
	
	/// TODO.
	KHB = Self::letters_to_token(b"KHB"),
	
	/// TODO.
	KHC = Self::letters_to_token(b"KHC"),
	
	/// TODO.
	KHD = Self::letters_to_token(b"KHD"),
	
	/// TODO.
	KHE = Self::letters_to_token(b"KHE"),
	
	/// TODO.
	KHF = Self::letters_to_token(b"KHF"),
	
	/// TODO.
	KHG = Self::letters_to_token(b"KHG"),
	
	/// TODO.
	KHH = Self::letters_to_token(b"KHH"),
	
	/// TODO.
	KHI = Self::letters_to_token(b"KHI"),
	
	/// TODO.
	KHJ = Self::letters_to_token(b"KHJ"),
	
	/// TODO.
	KHK = Self::letters_to_token(b"KHK"),
	
	/// TODO.
	KHL = Self::letters_to_token(b"KHL"),
	
	/// TODO.
	KHM = Self::letters_to_token(b"KHM"),
	
	/// TODO.
	KHN = Self::letters_to_token(b"KHN"),
	
	/// TODO.
	KHO = Self::letters_to_token(b"KHO"),
	
	/// TODO.
	KHP = Self::letters_to_token(b"KHP"),
	
	/// TODO.
	KHQ = Self::letters_to_token(b"KHQ"),
	
	/// TODO.
	KHR = Self::letters_to_token(b"KHR"),
	
	/// TODO.
	KHS = Self::letters_to_token(b"KHS"),
	
	/// TODO.
	KHT = Self::letters_to_token(b"KHT"),
	
	/// TODO.
	KHU = Self::letters_to_token(b"KHU"),
	
	/// TODO.
	KHV = Self::letters_to_token(b"KHV"),
	
	/// TODO.
	KHW = Self::letters_to_token(b"KHW"),
	
	/// TODO.
	KHX = Self::letters_to_token(b"KHX"),
	
	/// TODO.
	KHY = Self::letters_to_token(b"KHY"),
	
	/// TODO.
	KHZ = Self::letters_to_token(b"KHZ"),
	
	/// TODO.
	KIA = Self::letters_to_token(b"KIA"),
	
	/// TODO.
	KIB = Self::letters_to_token(b"KIB"),
	
	/// TODO.
	KIC = Self::letters_to_token(b"KIC"),
	
	/// TODO.
	KID = Self::letters_to_token(b"KID"),
	
	/// TODO.
	KIE = Self::letters_to_token(b"KIE"),
	
	/// TODO.
	KIF = Self::letters_to_token(b"KIF"),
	
	/// TODO.
	KIG = Self::letters_to_token(b"KIG"),
	
	/// TODO.
	KIH = Self::letters_to_token(b"KIH"),
	
	/// TODO.
	KII = Self::letters_to_token(b"KII"),
	
	/// TODO.
	KIJ = Self::letters_to_token(b"KIJ"),
	
	/// TODO.
	KIK = Self::letters_to_token(b"KIK"),
	
	/// TODO.
	KIL = Self::letters_to_token(b"KIL"),
	
	/// TODO.
	KIM = Self::letters_to_token(b"KIM"),
	
	/// TODO.
	KIN = Self::letters_to_token(b"KIN"),
	
	/// TODO.
	KIO = Self::letters_to_token(b"KIO"),
	
	/// TODO.
	KIP = Self::letters_to_token(b"KIP"),
	
	/// TODO.
	KIQ = Self::letters_to_token(b"KIQ"),
	
	/// TODO.
	KIR = Self::letters_to_token(b"KIR"),
	
	/// TODO.
	KIS = Self::letters_to_token(b"KIS"),
	
	/// TODO.
	KIT = Self::letters_to_token(b"KIT"),
	
	/// TODO.
	KIU = Self::letters_to_token(b"KIU"),
	
	/// TODO.
	KIV = Self::letters_to_token(b"KIV"),
	
	/// TODO.
	KIW = Self::letters_to_token(b"KIW"),
	
	/// TODO.
	KIX = Self::letters_to_token(b"KIX"),
	
	/// TODO.
	KIY = Self::letters_to_token(b"KIY"),
	
	/// TODO.
	KIZ = Self::letters_to_token(b"KIZ"),
	
	/// TODO.
	KJA = Self::letters_to_token(b"KJA"),
	
	/// TODO.
	KJB = Self::letters_to_token(b"KJB"),
	
	/// TODO.
	KJC = Self::letters_to_token(b"KJC"),
	
	/// TODO.
	KJD = Self::letters_to_token(b"KJD"),
	
	/// TODO.
	KJE = Self::letters_to_token(b"KJE"),
	
	/// TODO.
	KJF = Self::letters_to_token(b"KJF"),
	
	/// TODO.
	KJG = Self::letters_to_token(b"KJG"),
	
	/// TODO.
	KJH = Self::letters_to_token(b"KJH"),
	
	/// TODO.
	KJI = Self::letters_to_token(b"KJI"),
	
	/// TODO.
	KJJ = Self::letters_to_token(b"KJJ"),
	
	/// TODO.
	KJK = Self::letters_to_token(b"KJK"),
	
	/// TODO.
	KJL = Self::letters_to_token(b"KJL"),
	
	/// TODO.
	KJM = Self::letters_to_token(b"KJM"),
	
	/// TODO.
	KJN = Self::letters_to_token(b"KJN"),
	
	/// TODO.
	KJO = Self::letters_to_token(b"KJO"),
	
	/// TODO.
	KJP = Self::letters_to_token(b"KJP"),
	
	/// TODO.
	KJQ = Self::letters_to_token(b"KJQ"),
	
	/// TODO.
	KJR = Self::letters_to_token(b"KJR"),
	
	/// TODO.
	KJS = Self::letters_to_token(b"KJS"),
	
	/// TODO.
	KJT = Self::letters_to_token(b"KJT"),
	
	/// TODO.
	KJU = Self::letters_to_token(b"KJU"),
	
	/// TODO.
	KJV = Self::letters_to_token(b"KJV"),
	
	/// TODO.
	KJW = Self::letters_to_token(b"KJW"),
	
	/// TODO.
	KJX = Self::letters_to_token(b"KJX"),
	
	/// TODO.
	KJY = Self::letters_to_token(b"KJY"),
	
	/// TODO.
	KJZ = Self::letters_to_token(b"KJZ"),
	
	/// TODO.
	KKA = Self::letters_to_token(b"KKA"),
	
	/// TODO.
	KKB = Self::letters_to_token(b"KKB"),
	
	/// TODO.
	KKC = Self::letters_to_token(b"KKC"),
	
	/// TODO.
	KKD = Self::letters_to_token(b"KKD"),
	
	/// TODO.
	KKE = Self::letters_to_token(b"KKE"),
	
	/// TODO.
	KKF = Self::letters_to_token(b"KKF"),
	
	/// TODO.
	KKG = Self::letters_to_token(b"KKG"),
	
	/// TODO.
	KKH = Self::letters_to_token(b"KKH"),
	
	/// TODO.
	KKI = Self::letters_to_token(b"KKI"),
	
	/// TODO.
	KKJ = Self::letters_to_token(b"KKJ"),
	
	/// TODO.
	KKK = Self::letters_to_token(b"KKK"),
	
	/// TODO.
	KKL = Self::letters_to_token(b"KKL"),
	
	/// TODO.
	KKM = Self::letters_to_token(b"KKM"),
	
	/// TODO.
	KKN = Self::letters_to_token(b"KKN"),
	
	/// TODO.
	KKO = Self::letters_to_token(b"KKO"),
	
	/// TODO.
	KKP = Self::letters_to_token(b"KKP"),
	
	/// TODO.
	KKQ = Self::letters_to_token(b"KKQ"),
	
	/// TODO.
	KKR = Self::letters_to_token(b"KKR"),
	
	/// TODO.
	KKS = Self::letters_to_token(b"KKS"),
	
	/// TODO.
	KKT = Self::letters_to_token(b"KKT"),
	
	/// TODO.
	KKU = Self::letters_to_token(b"KKU"),
	
	/// TODO.
	KKV = Self::letters_to_token(b"KKV"),
	
	/// TODO.
	KKW = Self::letters_to_token(b"KKW"),
	
	/// TODO.
	KKX = Self::letters_to_token(b"KKX"),
	
	/// TODO.
	KKY = Self::letters_to_token(b"KKY"),
	
	/// TODO.
	KKZ = Self::letters_to_token(b"KKZ"),
	
	/// TODO.
	KLA = Self::letters_to_token(b"KLA"),
	
	/// TODO.
	KLB = Self::letters_to_token(b"KLB"),
	
	/// TODO.
	KLC = Self::letters_to_token(b"KLC"),
	
	/// TODO.
	KLD = Self::letters_to_token(b"KLD"),
	
	/// TODO.
	KLE = Self::letters_to_token(b"KLE"),
	
	/// TODO.
	KLF = Self::letters_to_token(b"KLF"),
	
	/// TODO.
	KLG = Self::letters_to_token(b"KLG"),
	
	/// TODO.
	KLH = Self::letters_to_token(b"KLH"),
	
	/// TODO.
	KLI = Self::letters_to_token(b"KLI"),
	
	/// TODO.
	KLJ = Self::letters_to_token(b"KLJ"),
	
	/// TODO.
	KLK = Self::letters_to_token(b"KLK"),
	
	/// TODO.
	KLL = Self::letters_to_token(b"KLL"),
	
	/// TODO.
	KLM = Self::letters_to_token(b"KLM"),
	
	/// TODO.
	KLN = Self::letters_to_token(b"KLN"),
	
	/// TODO.
	KLO = Self::letters_to_token(b"KLO"),
	
	/// TODO.
	KLP = Self::letters_to_token(b"KLP"),
	
	/// TODO.
	KLQ = Self::letters_to_token(b"KLQ"),
	
	/// TODO.
	KLR = Self::letters_to_token(b"KLR"),
	
	/// TODO.
	KLS = Self::letters_to_token(b"KLS"),
	
	/// TODO.
	KLT = Self::letters_to_token(b"KLT"),
	
	/// TODO.
	KLU = Self::letters_to_token(b"KLU"),
	
	/// TODO.
	KLV = Self::letters_to_token(b"KLV"),
	
	/// TODO.
	KLW = Self::letters_to_token(b"KLW"),
	
	/// TODO.
	KLX = Self::letters_to_token(b"KLX"),
	
	/// TODO.
	KLY = Self::letters_to_token(b"KLY"),
	
	/// TODO.
	KLZ = Self::letters_to_token(b"KLZ"),
	
	/// TODO.
	KMA = Self::letters_to_token(b"KMA"),
	
	/// TODO.
	KMB = Self::letters_to_token(b"KMB"),
	
	/// TODO.
	KMC = Self::letters_to_token(b"KMC"),
	
	/// TODO.
	KMD = Self::letters_to_token(b"KMD"),
	
	/// TODO.
	KME = Self::letters_to_token(b"KME"),
	
	/// TODO.
	KMF = Self::letters_to_token(b"KMF"),
	
	/// TODO.
	KMG = Self::letters_to_token(b"KMG"),
	
	/// TODO.
	KMH = Self::letters_to_token(b"KMH"),
	
	/// TODO.
	KMI = Self::letters_to_token(b"KMI"),
	
	/// TODO.
	KMJ = Self::letters_to_token(b"KMJ"),
	
	/// TODO.
	KMK = Self::letters_to_token(b"KMK"),
	
	/// TODO.
	KML = Self::letters_to_token(b"KML"),
	
	/// TODO.
	KMM = Self::letters_to_token(b"KMM"),
	
	/// TODO.
	KMN = Self::letters_to_token(b"KMN"),
	
	/// TODO.
	KMO = Self::letters_to_token(b"KMO"),
	
	/// TODO.
	KMP = Self::letters_to_token(b"KMP"),
	
	/// TODO.
	KMQ = Self::letters_to_token(b"KMQ"),
	
	/// TODO.
	KMR = Self::letters_to_token(b"KMR"),
	
	/// TODO.
	KMS = Self::letters_to_token(b"KMS"),
	
	/// TODO.
	KMT = Self::letters_to_token(b"KMT"),
	
	/// TODO.
	KMU = Self::letters_to_token(b"KMU"),
	
	/// TODO.
	KMV = Self::letters_to_token(b"KMV"),
	
	/// TODO.
	KMW = Self::letters_to_token(b"KMW"),
	
	/// TODO.
	KMX = Self::letters_to_token(b"KMX"),
	
	/// TODO.
	KMY = Self::letters_to_token(b"KMY"),
	
	/// TODO.
	KMZ = Self::letters_to_token(b"KMZ"),
	
	/// TODO.
	KNA = Self::letters_to_token(b"KNA"),
	
	/// TODO.
	KNB = Self::letters_to_token(b"KNB"),
	
	/// TODO.
	KNC = Self::letters_to_token(b"KNC"),
	
	/// TODO.
	KND = Self::letters_to_token(b"KND"),
	
	/// TODO.
	KNE = Self::letters_to_token(b"KNE"),
	
	/// TODO.
	KNF = Self::letters_to_token(b"KNF"),
	
	/// TODO.
	KNG = Self::letters_to_token(b"KNG"),
	
	/// TODO.
	KNH = Self::letters_to_token(b"KNH"),
	
	/// TODO.
	KNI = Self::letters_to_token(b"KNI"),
	
	/// TODO.
	KNJ = Self::letters_to_token(b"KNJ"),
	
	/// TODO.
	KNK = Self::letters_to_token(b"KNK"),
	
	/// TODO.
	KNL = Self::letters_to_token(b"KNL"),
	
	/// TODO.
	KNM = Self::letters_to_token(b"KNM"),
	
	/// TODO.
	KNN = Self::letters_to_token(b"KNN"),
	
	/// TODO.
	KNO = Self::letters_to_token(b"KNO"),
	
	/// TODO.
	KNP = Self::letters_to_token(b"KNP"),
	
	/// TODO.
	KNQ = Self::letters_to_token(b"KNQ"),
	
	/// TODO.
	KNR = Self::letters_to_token(b"KNR"),
	
	/// TODO.
	KNS = Self::letters_to_token(b"KNS"),
	
	/// TODO.
	KNT = Self::letters_to_token(b"KNT"),
	
	/// TODO.
	KNU = Self::letters_to_token(b"KNU"),
	
	/// TODO.
	KNV = Self::letters_to_token(b"KNV"),
	
	/// TODO.
	KNW = Self::letters_to_token(b"KNW"),
	
	/// TODO.
	KNX = Self::letters_to_token(b"KNX"),
	
	/// TODO.
	KNY = Self::letters_to_token(b"KNY"),
	
	/// TODO.
	KNZ = Self::letters_to_token(b"KNZ"),
	
	/// TODO.
	KOA = Self::letters_to_token(b"KOA"),
	
	/// TODO.
	KOB = Self::letters_to_token(b"KOB"),
	
	/// TODO.
	KOC = Self::letters_to_token(b"KOC"),
	
	/// TODO.
	KOD = Self::letters_to_token(b"KOD"),
	
	/// TODO.
	KOE = Self::letters_to_token(b"KOE"),
	
	/// TODO.
	KOF = Self::letters_to_token(b"KOF"),
	
	/// TODO.
	KOG = Self::letters_to_token(b"KOG"),
	
	/// TODO.
	KOH = Self::letters_to_token(b"KOH"),
	
	/// TODO.
	KOI = Self::letters_to_token(b"KOI"),
	
	/// TODO.
	KOJ = Self::letters_to_token(b"KOJ"),
	
	/// TODO.
	KOK = Self::letters_to_token(b"KOK"),
	
	/// TODO.
	KOL = Self::letters_to_token(b"KOL"),
	
	/// TODO.
	KOM = Self::letters_to_token(b"KOM"),
	
	/// TODO.
	KON = Self::letters_to_token(b"KON"),
	
	/// TODO.
	KOO = Self::letters_to_token(b"KOO"),
	
	/// TODO.
	KOP = Self::letters_to_token(b"KOP"),
	
	/// TODO.
	KOQ = Self::letters_to_token(b"KOQ"),
	
	/// TODO.
	KOR = Self::letters_to_token(b"KOR"),
	
	/// TODO.
	KOS = Self::letters_to_token(b"KOS"),
	
	/// TODO.
	KOT = Self::letters_to_token(b"KOT"),
	
	/// TODO.
	KOU = Self::letters_to_token(b"KOU"),
	
	/// TODO.
	KOV = Self::letters_to_token(b"KOV"),
	
	/// TODO.
	KOW = Self::letters_to_token(b"KOW"),
	
	/// TODO.
	KOX = Self::letters_to_token(b"KOX"),
	
	/// TODO.
	KOY = Self::letters_to_token(b"KOY"),
	
	/// TODO.
	KOZ = Self::letters_to_token(b"KOZ"),
	
	/// TODO.
	KPA = Self::letters_to_token(b"KPA"),
	
	/// TODO.
	KPB = Self::letters_to_token(b"KPB"),
	
	/// TODO.
	KPC = Self::letters_to_token(b"KPC"),
	
	/// TODO.
	KPD = Self::letters_to_token(b"KPD"),
	
	/// TODO.
	KPE = Self::letters_to_token(b"KPE"),
	
	/// TODO.
	KPF = Self::letters_to_token(b"KPF"),
	
	/// TODO.
	KPG = Self::letters_to_token(b"KPG"),
	
	/// TODO.
	KPH = Self::letters_to_token(b"KPH"),
	
	/// TODO.
	KPI = Self::letters_to_token(b"KPI"),
	
	/// TODO.
	KPJ = Self::letters_to_token(b"KPJ"),
	
	/// TODO.
	KPK = Self::letters_to_token(b"KPK"),
	
	/// TODO.
	KPL = Self::letters_to_token(b"KPL"),
	
	/// TODO.
	KPM = Self::letters_to_token(b"KPM"),
	
	/// TODO.
	KPN = Self::letters_to_token(b"KPN"),
	
	/// TODO.
	KPO = Self::letters_to_token(b"KPO"),
	
	/// TODO.
	KPP = Self::letters_to_token(b"KPP"),
	
	/// TODO.
	KPQ = Self::letters_to_token(b"KPQ"),
	
	/// TODO.
	KPR = Self::letters_to_token(b"KPR"),
	
	/// TODO.
	KPS = Self::letters_to_token(b"KPS"),
	
	/// TODO.
	KPT = Self::letters_to_token(b"KPT"),
	
	/// TODO.
	KPU = Self::letters_to_token(b"KPU"),
	
	/// TODO.
	KPV = Self::letters_to_token(b"KPV"),
	
	/// TODO.
	KPW = Self::letters_to_token(b"KPW"),
	
	/// TODO.
	KPX = Self::letters_to_token(b"KPX"),
	
	/// TODO.
	KPY = Self::letters_to_token(b"KPY"),
	
	/// TODO.
	KPZ = Self::letters_to_token(b"KPZ"),
	
	/// TODO.
	KQA = Self::letters_to_token(b"KQA"),
	
	/// TODO.
	KQB = Self::letters_to_token(b"KQB"),
	
	/// TODO.
	KQC = Self::letters_to_token(b"KQC"),
	
	/// TODO.
	KQD = Self::letters_to_token(b"KQD"),
	
	/// TODO.
	KQE = Self::letters_to_token(b"KQE"),
	
	/// TODO.
	KQF = Self::letters_to_token(b"KQF"),
	
	/// TODO.
	KQG = Self::letters_to_token(b"KQG"),
	
	/// TODO.
	KQH = Self::letters_to_token(b"KQH"),
	
	/// TODO.
	KQI = Self::letters_to_token(b"KQI"),
	
	/// TODO.
	KQJ = Self::letters_to_token(b"KQJ"),
	
	/// TODO.
	KQK = Self::letters_to_token(b"KQK"),
	
	/// TODO.
	KQL = Self::letters_to_token(b"KQL"),
	
	/// TODO.
	KQM = Self::letters_to_token(b"KQM"),
	
	/// TODO.
	KQN = Self::letters_to_token(b"KQN"),
	
	/// TODO.
	KQO = Self::letters_to_token(b"KQO"),
	
	/// TODO.
	KQP = Self::letters_to_token(b"KQP"),
	
	/// TODO.
	KQQ = Self::letters_to_token(b"KQQ"),
	
	/// TODO.
	KQR = Self::letters_to_token(b"KQR"),
	
	/// TODO.
	KQS = Self::letters_to_token(b"KQS"),
	
	/// TODO.
	KQT = Self::letters_to_token(b"KQT"),
	
	/// TODO.
	KQU = Self::letters_to_token(b"KQU"),
	
	/// TODO.
	KQV = Self::letters_to_token(b"KQV"),
	
	/// TODO.
	KQW = Self::letters_to_token(b"KQW"),
	
	/// TODO.
	KQX = Self::letters_to_token(b"KQX"),
	
	/// TODO.
	KQY = Self::letters_to_token(b"KQY"),
	
	/// TODO.
	KQZ = Self::letters_to_token(b"KQZ"),
	
	/// TODO.
	KRA = Self::letters_to_token(b"KRA"),
	
	/// TODO.
	KRB = Self::letters_to_token(b"KRB"),
	
	/// TODO.
	KRC = Self::letters_to_token(b"KRC"),
	
	/// TODO.
	KRD = Self::letters_to_token(b"KRD"),
	
	/// TODO.
	KRE = Self::letters_to_token(b"KRE"),
	
	/// TODO.
	KRF = Self::letters_to_token(b"KRF"),
	
	/// TODO.
	KRG = Self::letters_to_token(b"KRG"),
	
	/// TODO.
	KRH = Self::letters_to_token(b"KRH"),
	
	/// TODO.
	KRI = Self::letters_to_token(b"KRI"),
	
	/// TODO.
	KRJ = Self::letters_to_token(b"KRJ"),
	
	/// TODO.
	KRK = Self::letters_to_token(b"KRK"),
	
	/// TODO.
	KRL = Self::letters_to_token(b"KRL"),
	
	/// TODO.
	KRM = Self::letters_to_token(b"KRM"),
	
	/// TODO.
	KRN = Self::letters_to_token(b"KRN"),
	
	/// TODO.
	KRO = Self::letters_to_token(b"KRO"),
	
	/// TODO.
	KRP = Self::letters_to_token(b"KRP"),
	
	/// TODO.
	KRQ = Self::letters_to_token(b"KRQ"),
	
	/// TODO.
	KRR = Self::letters_to_token(b"KRR"),
	
	/// TODO.
	KRS = Self::letters_to_token(b"KRS"),
	
	/// TODO.
	KRT = Self::letters_to_token(b"KRT"),
	
	/// TODO.
	KRU = Self::letters_to_token(b"KRU"),
	
	/// TODO.
	KRV = Self::letters_to_token(b"KRV"),
	
	/// TODO.
	KRW = Self::letters_to_token(b"KRW"),
	
	/// TODO.
	KRX = Self::letters_to_token(b"KRX"),
	
	/// TODO.
	KRY = Self::letters_to_token(b"KRY"),
	
	/// TODO.
	KRZ = Self::letters_to_token(b"KRZ"),
	
	/// TODO.
	KSA = Self::letters_to_token(b"KSA"),
	
	/// TODO.
	KSB = Self::letters_to_token(b"KSB"),
	
	/// TODO.
	KSC = Self::letters_to_token(b"KSC"),
	
	/// TODO.
	KSD = Self::letters_to_token(b"KSD"),
	
	/// TODO.
	KSE = Self::letters_to_token(b"KSE"),
	
	/// TODO.
	KSF = Self::letters_to_token(b"KSF"),
	
	/// TODO.
	KSG = Self::letters_to_token(b"KSG"),
	
	/// TODO.
	KSH = Self::letters_to_token(b"KSH"),
	
	/// TODO.
	KSI = Self::letters_to_token(b"KSI"),
	
	/// TODO.
	KSJ = Self::letters_to_token(b"KSJ"),
	
	/// TODO.
	KSK = Self::letters_to_token(b"KSK"),
	
	/// TODO.
	KSL = Self::letters_to_token(b"KSL"),
	
	/// TODO.
	KSM = Self::letters_to_token(b"KSM"),
	
	/// TODO.
	KSN = Self::letters_to_token(b"KSN"),
	
	/// TODO.
	KSO = Self::letters_to_token(b"KSO"),
	
	/// TODO.
	KSP = Self::letters_to_token(b"KSP"),
	
	/// TODO.
	KSQ = Self::letters_to_token(b"KSQ"),
	
	/// TODO.
	KSR = Self::letters_to_token(b"KSR"),
	
	/// TODO.
	KSS = Self::letters_to_token(b"KSS"),
	
	/// TODO.
	KST = Self::letters_to_token(b"KST"),
	
	/// TODO.
	KSU = Self::letters_to_token(b"KSU"),
	
	/// TODO.
	KSV = Self::letters_to_token(b"KSV"),
	
	/// TODO.
	KSW = Self::letters_to_token(b"KSW"),
	
	/// TODO.
	KSX = Self::letters_to_token(b"KSX"),
	
	/// TODO.
	KSY = Self::letters_to_token(b"KSY"),
	
	/// TODO.
	KSZ = Self::letters_to_token(b"KSZ"),
	
	/// TODO.
	KTA = Self::letters_to_token(b"KTA"),
	
	/// TODO.
	KTB = Self::letters_to_token(b"KTB"),
	
	/// TODO.
	KTC = Self::letters_to_token(b"KTC"),
	
	/// TODO.
	KTD = Self::letters_to_token(b"KTD"),
	
	/// TODO.
	KTE = Self::letters_to_token(b"KTE"),
	
	/// TODO.
	KTF = Self::letters_to_token(b"KTF"),
	
	/// TODO.
	KTG = Self::letters_to_token(b"KTG"),
	
	/// TODO.
	KTH = Self::letters_to_token(b"KTH"),
	
	/// TODO.
	KTI = Self::letters_to_token(b"KTI"),
	
	/// TODO.
	KTJ = Self::letters_to_token(b"KTJ"),
	
	/// TODO.
	KTK = Self::letters_to_token(b"KTK"),
	
	/// TODO.
	KTL = Self::letters_to_token(b"KTL"),
	
	/// TODO.
	KTM = Self::letters_to_token(b"KTM"),
	
	/// TODO.
	KTN = Self::letters_to_token(b"KTN"),
	
	/// TODO.
	KTO = Self::letters_to_token(b"KTO"),
	
	/// TODO.
	KTP = Self::letters_to_token(b"KTP"),
	
	/// TODO.
	KTQ = Self::letters_to_token(b"KTQ"),
	
	/// TODO.
	KTR = Self::letters_to_token(b"KTR"),
	
	/// TODO.
	KTS = Self::letters_to_token(b"KTS"),
	
	/// TODO.
	KTT = Self::letters_to_token(b"KTT"),
	
	/// TODO.
	KTU = Self::letters_to_token(b"KTU"),
	
	/// TODO.
	KTV = Self::letters_to_token(b"KTV"),
	
	/// TODO.
	KTW = Self::letters_to_token(b"KTW"),
	
	/// TODO.
	KTX = Self::letters_to_token(b"KTX"),
	
	/// TODO.
	KTY = Self::letters_to_token(b"KTY"),
	
	/// TODO.
	KTZ = Self::letters_to_token(b"KTZ"),
	
	/// TODO.
	KUA = Self::letters_to_token(b"KUA"),
	
	/// TODO.
	KUB = Self::letters_to_token(b"KUB"),
	
	/// TODO.
	KUC = Self::letters_to_token(b"KUC"),
	
	/// TODO.
	KUD = Self::letters_to_token(b"KUD"),
	
	/// TODO.
	KUE = Self::letters_to_token(b"KUE"),
	
	/// TODO.
	KUF = Self::letters_to_token(b"KUF"),
	
	/// TODO.
	KUG = Self::letters_to_token(b"KUG"),
	
	/// TODO.
	KUH = Self::letters_to_token(b"KUH"),
	
	/// TODO.
	KUI = Self::letters_to_token(b"KUI"),
	
	/// TODO.
	KUJ = Self::letters_to_token(b"KUJ"),
	
	/// TODO.
	KUK = Self::letters_to_token(b"KUK"),
	
	/// TODO.
	KUL = Self::letters_to_token(b"KUL"),
	
	/// TODO.
	KUM = Self::letters_to_token(b"KUM"),
	
	/// TODO.
	KUN = Self::letters_to_token(b"KUN"),
	
	/// TODO.
	KUO = Self::letters_to_token(b"KUO"),
	
	/// TODO.
	KUP = Self::letters_to_token(b"KUP"),
	
	/// TODO.
	KUQ = Self::letters_to_token(b"KUQ"),
	
	/// TODO.
	KUR = Self::letters_to_token(b"KUR"),
	
	/// TODO.
	KUS = Self::letters_to_token(b"KUS"),
	
	/// TODO.
	KUT = Self::letters_to_token(b"KUT"),
	
	/// TODO.
	KUU = Self::letters_to_token(b"KUU"),
	
	/// TODO.
	KUV = Self::letters_to_token(b"KUV"),
	
	/// TODO.
	KUW = Self::letters_to_token(b"KUW"),
	
	/// TODO.
	KUX = Self::letters_to_token(b"KUX"),
	
	/// TODO.
	KUY = Self::letters_to_token(b"KUY"),
	
	/// TODO.
	KUZ = Self::letters_to_token(b"KUZ"),
	
	/// TODO.
	KVA = Self::letters_to_token(b"KVA"),
	
	/// TODO.
	KVB = Self::letters_to_token(b"KVB"),
	
	/// TODO.
	KVC = Self::letters_to_token(b"KVC"),
	
	/// TODO.
	KVD = Self::letters_to_token(b"KVD"),
	
	/// TODO.
	KVE = Self::letters_to_token(b"KVE"),
	
	/// TODO.
	KVF = Self::letters_to_token(b"KVF"),
	
	/// TODO.
	KVG = Self::letters_to_token(b"KVG"),
	
	/// TODO.
	KVH = Self::letters_to_token(b"KVH"),
	
	/// TODO.
	KVI = Self::letters_to_token(b"KVI"),
	
	/// TODO.
	KVJ = Self::letters_to_token(b"KVJ"),
	
	/// TODO.
	KVK = Self::letters_to_token(b"KVK"),
	
	/// TODO.
	KVL = Self::letters_to_token(b"KVL"),
	
	/// TODO.
	KVM = Self::letters_to_token(b"KVM"),
	
	/// TODO.
	KVN = Self::letters_to_token(b"KVN"),
	
	/// TODO.
	KVO = Self::letters_to_token(b"KVO"),
	
	/// TODO.
	KVP = Self::letters_to_token(b"KVP"),
	
	/// TODO.
	KVQ = Self::letters_to_token(b"KVQ"),
	
	/// TODO.
	KVR = Self::letters_to_token(b"KVR"),
	
	/// TODO.
	KVS = Self::letters_to_token(b"KVS"),
	
	/// TODO.
	KVT = Self::letters_to_token(b"KVT"),
	
	/// TODO.
	KVU = Self::letters_to_token(b"KVU"),
	
	/// TODO.
	KVV = Self::letters_to_token(b"KVV"),
	
	/// TODO.
	KVW = Self::letters_to_token(b"KVW"),
	
	/// TODO.
	KVX = Self::letters_to_token(b"KVX"),
	
	/// TODO.
	KVY = Self::letters_to_token(b"KVY"),
	
	/// TODO.
	KVZ = Self::letters_to_token(b"KVZ"),
	
	/// TODO.
	KWA = Self::letters_to_token(b"KWA"),
	
	/// TODO.
	KWB = Self::letters_to_token(b"KWB"),
	
	/// TODO.
	KWC = Self::letters_to_token(b"KWC"),
	
	/// TODO.
	KWD = Self::letters_to_token(b"KWD"),
	
	/// TODO.
	KWE = Self::letters_to_token(b"KWE"),
	
	/// TODO.
	KWF = Self::letters_to_token(b"KWF"),
	
	/// TODO.
	KWG = Self::letters_to_token(b"KWG"),
	
	/// TODO.
	KWH = Self::letters_to_token(b"KWH"),
	
	/// TODO.
	KWI = Self::letters_to_token(b"KWI"),
	
	/// TODO.
	KWJ = Self::letters_to_token(b"KWJ"),
	
	/// TODO.
	KWK = Self::letters_to_token(b"KWK"),
	
	/// TODO.
	KWL = Self::letters_to_token(b"KWL"),
	
	/// TODO.
	KWM = Self::letters_to_token(b"KWM"),
	
	/// TODO.
	KWN = Self::letters_to_token(b"KWN"),
	
	/// TODO.
	KWO = Self::letters_to_token(b"KWO"),
	
	/// TODO.
	KWP = Self::letters_to_token(b"KWP"),
	
	/// TODO.
	KWQ = Self::letters_to_token(b"KWQ"),
	
	/// TODO.
	KWR = Self::letters_to_token(b"KWR"),
	
	/// TODO.
	KWS = Self::letters_to_token(b"KWS"),
	
	/// TODO.
	KWT = Self::letters_to_token(b"KWT"),
	
	/// TODO.
	KWU = Self::letters_to_token(b"KWU"),
	
	/// TODO.
	KWV = Self::letters_to_token(b"KWV"),
	
	/// TODO.
	KWW = Self::letters_to_token(b"KWW"),
	
	/// TODO.
	KWX = Self::letters_to_token(b"KWX"),
	
	/// TODO.
	KWY = Self::letters_to_token(b"KWY"),
	
	/// TODO.
	KWZ = Self::letters_to_token(b"KWZ"),
	
	/// TODO.
	KXA = Self::letters_to_token(b"KXA"),
	
	/// TODO.
	KXB = Self::letters_to_token(b"KXB"),
	
	/// TODO.
	KXC = Self::letters_to_token(b"KXC"),
	
	/// TODO.
	KXD = Self::letters_to_token(b"KXD"),
	
	/// TODO.
	KXE = Self::letters_to_token(b"KXE"),
	
	/// TODO.
	KXF = Self::letters_to_token(b"KXF"),
	
	/// TODO.
	KXG = Self::letters_to_token(b"KXG"),
	
	/// TODO.
	KXH = Self::letters_to_token(b"KXH"),
	
	/// TODO.
	KXI = Self::letters_to_token(b"KXI"),
	
	/// TODO.
	KXJ = Self::letters_to_token(b"KXJ"),
	
	/// TODO.
	KXK = Self::letters_to_token(b"KXK"),
	
	/// TODO.
	KXL = Self::letters_to_token(b"KXL"),
	
	/// TODO.
	KXM = Self::letters_to_token(b"KXM"),
	
	/// TODO.
	KXN = Self::letters_to_token(b"KXN"),
	
	/// TODO.
	KXO = Self::letters_to_token(b"KXO"),
	
	/// TODO.
	KXP = Self::letters_to_token(b"KXP"),
	
	/// TODO.
	KXQ = Self::letters_to_token(b"KXQ"),
	
	/// TODO.
	KXR = Self::letters_to_token(b"KXR"),
	
	/// TODO.
	KXS = Self::letters_to_token(b"KXS"),
	
	/// TODO.
	KXT = Self::letters_to_token(b"KXT"),
	
	/// TODO.
	KXU = Self::letters_to_token(b"KXU"),
	
	/// TODO.
	KXV = Self::letters_to_token(b"KXV"),
	
	/// TODO.
	KXW = Self::letters_to_token(b"KXW"),
	
	/// TODO.
	KXX = Self::letters_to_token(b"KXX"),
	
	/// TODO.
	KXY = Self::letters_to_token(b"KXY"),
	
	/// TODO.
	KXZ = Self::letters_to_token(b"KXZ"),
	
	/// TODO.
	KYA = Self::letters_to_token(b"KYA"),
	
	/// TODO.
	KYB = Self::letters_to_token(b"KYB"),
	
	/// TODO.
	KYC = Self::letters_to_token(b"KYC"),
	
	/// TODO.
	KYD = Self::letters_to_token(b"KYD"),
	
	/// TODO.
	KYE = Self::letters_to_token(b"KYE"),
	
	/// TODO.
	KYF = Self::letters_to_token(b"KYF"),
	
	/// TODO.
	KYG = Self::letters_to_token(b"KYG"),
	
	/// TODO.
	KYH = Self::letters_to_token(b"KYH"),
	
	/// TODO.
	KYI = Self::letters_to_token(b"KYI"),
	
	/// TODO.
	KYJ = Self::letters_to_token(b"KYJ"),
	
	/// TODO.
	KYK = Self::letters_to_token(b"KYK"),
	
	/// TODO.
	KYL = Self::letters_to_token(b"KYL"),
	
	/// TODO.
	KYM = Self::letters_to_token(b"KYM"),
	
	/// TODO.
	KYN = Self::letters_to_token(b"KYN"),
	
	/// TODO.
	KYO = Self::letters_to_token(b"KYO"),
	
	/// TODO.
	KYP = Self::letters_to_token(b"KYP"),
	
	/// TODO.
	KYQ = Self::letters_to_token(b"KYQ"),
	
	/// TODO.
	KYR = Self::letters_to_token(b"KYR"),
	
	/// TODO.
	KYS = Self::letters_to_token(b"KYS"),
	
	/// TODO.
	KYT = Self::letters_to_token(b"KYT"),
	
	/// TODO.
	KYU = Self::letters_to_token(b"KYU"),
	
	/// TODO.
	KYV = Self::letters_to_token(b"KYV"),
	
	/// TODO.
	KYW = Self::letters_to_token(b"KYW"),
	
	/// TODO.
	KYX = Self::letters_to_token(b"KYX"),
	
	/// TODO.
	KYY = Self::letters_to_token(b"KYY"),
	
	/// TODO.
	KYZ = Self::letters_to_token(b"KYZ"),
	
	/// TODO.
	KZA = Self::letters_to_token(b"KZA"),
	
	/// TODO.
	KZB = Self::letters_to_token(b"KZB"),
	
	/// TODO.
	KZC = Self::letters_to_token(b"KZC"),
	
	/// TODO.
	KZD = Self::letters_to_token(b"KZD"),
	
	/// TODO.
	KZE = Self::letters_to_token(b"KZE"),
	
	/// TODO.
	KZF = Self::letters_to_token(b"KZF"),
	
	/// TODO.
	KZG = Self::letters_to_token(b"KZG"),
	
	/// TODO.
	KZH = Self::letters_to_token(b"KZH"),
	
	/// TODO.
	KZI = Self::letters_to_token(b"KZI"),
	
	/// TODO.
	KZJ = Self::letters_to_token(b"KZJ"),
	
	/// TODO.
	KZK = Self::letters_to_token(b"KZK"),
	
	/// TODO.
	KZL = Self::letters_to_token(b"KZL"),
	
	/// TODO.
	KZM = Self::letters_to_token(b"KZM"),
	
	/// TODO.
	KZN = Self::letters_to_token(b"KZN"),
	
	/// TODO.
	KZO = Self::letters_to_token(b"KZO"),
	
	/// TODO.
	KZP = Self::letters_to_token(b"KZP"),
	
	/// TODO.
	KZQ = Self::letters_to_token(b"KZQ"),
	
	/// TODO.
	KZR = Self::letters_to_token(b"KZR"),
	
	/// TODO.
	KZS = Self::letters_to_token(b"KZS"),
	
	/// TODO.
	KZT = Self::letters_to_token(b"KZT"),
	
	/// TODO.
	KZU = Self::letters_to_token(b"KZU"),
	
	/// TODO.
	KZV = Self::letters_to_token(b"KZV"),
	
	/// TODO.
	KZW = Self::letters_to_token(b"KZW"),
	
	/// TODO.
	KZX = Self::letters_to_token(b"KZX"),
	
	/// TODO.
	KZY = Self::letters_to_token(b"KZY"),
	
	/// TODO.
	KZZ = Self::letters_to_token(b"KZZ"),
	
	/// TODO.
	LAA = Self::letters_to_token(b"LAA"),
	
	/// TODO.
	LAB = Self::letters_to_token(b"LAB"),
	
	/// TODO.
	LAC = Self::letters_to_token(b"LAC"),
	
	/// TODO.
	LAD = Self::letters_to_token(b"LAD"),
	
	/// TODO.
	LAE = Self::letters_to_token(b"LAE"),
	
	/// TODO.
	LAF = Self::letters_to_token(b"LAF"),
	
	/// TODO.
	LAG = Self::letters_to_token(b"LAG"),
	
	/// TODO.
	LAH = Self::letters_to_token(b"LAH"),
	
	/// TODO.
	LAI = Self::letters_to_token(b"LAI"),
	
	/// TODO.
	LAJ = Self::letters_to_token(b"LAJ"),
	
	/// TODO.
	LAK = Self::letters_to_token(b"LAK"),
	
	/// TODO.
	LAL = Self::letters_to_token(b"LAL"),
	
	/// TODO.
	LAM = Self::letters_to_token(b"LAM"),
	
	/// TODO.
	LAN = Self::letters_to_token(b"LAN"),
	
	/// TODO.
	LAO = Self::letters_to_token(b"LAO"),
	
	/// TODO.
	LAP = Self::letters_to_token(b"LAP"),
	
	/// TODO.
	LAQ = Self::letters_to_token(b"LAQ"),
	
	/// TODO.
	LAR = Self::letters_to_token(b"LAR"),
	
	/// TODO.
	LAS = Self::letters_to_token(b"LAS"),
	
	/// TODO.
	LAT = Self::letters_to_token(b"LAT"),
	
	/// TODO.
	LAU = Self::letters_to_token(b"LAU"),
	
	/// TODO.
	LAV = Self::letters_to_token(b"LAV"),
	
	/// TODO.
	LAW = Self::letters_to_token(b"LAW"),
	
	/// TODO.
	LAX = Self::letters_to_token(b"LAX"),
	
	/// TODO.
	LAY = Self::letters_to_token(b"LAY"),
	
	/// TODO.
	LAZ = Self::letters_to_token(b"LAZ"),
	
	/// TODO.
	LBA = Self::letters_to_token(b"LBA"),
	
	/// TODO.
	LBB = Self::letters_to_token(b"LBB"),
	
	/// TODO.
	LBC = Self::letters_to_token(b"LBC"),
	
	/// TODO.
	LBD = Self::letters_to_token(b"LBD"),
	
	/// TODO.
	LBE = Self::letters_to_token(b"LBE"),
	
	/// TODO.
	LBF = Self::letters_to_token(b"LBF"),
	
	/// TODO.
	LBG = Self::letters_to_token(b"LBG"),
	
	/// TODO.
	LBH = Self::letters_to_token(b"LBH"),
	
	/// TODO.
	LBI = Self::letters_to_token(b"LBI"),
	
	/// TODO.
	LBJ = Self::letters_to_token(b"LBJ"),
	
	/// TODO.
	LBK = Self::letters_to_token(b"LBK"),
	
	/// TODO.
	LBL = Self::letters_to_token(b"LBL"),
	
	/// TODO.
	LBM = Self::letters_to_token(b"LBM"),
	
	/// TODO.
	LBN = Self::letters_to_token(b"LBN"),
	
	/// TODO.
	LBO = Self::letters_to_token(b"LBO"),
	
	/// TODO.
	LBP = Self::letters_to_token(b"LBP"),
	
	/// TODO.
	LBQ = Self::letters_to_token(b"LBQ"),
	
	/// TODO.
	LBR = Self::letters_to_token(b"LBR"),
	
	/// TODO.
	LBS = Self::letters_to_token(b"LBS"),
	
	/// TODO.
	LBT = Self::letters_to_token(b"LBT"),
	
	/// TODO.
	LBU = Self::letters_to_token(b"LBU"),
	
	/// TODO.
	LBV = Self::letters_to_token(b"LBV"),
	
	/// TODO.
	LBW = Self::letters_to_token(b"LBW"),
	
	/// TODO.
	LBX = Self::letters_to_token(b"LBX"),
	
	/// TODO.
	LBY = Self::letters_to_token(b"LBY"),
	
	/// TODO.
	LBZ = Self::letters_to_token(b"LBZ"),
	
	/// TODO.
	LCA = Self::letters_to_token(b"LCA"),
	
	/// TODO.
	LCB = Self::letters_to_token(b"LCB"),
	
	/// TODO.
	LCC = Self::letters_to_token(b"LCC"),
	
	/// TODO.
	LCD = Self::letters_to_token(b"LCD"),
	
	/// TODO.
	LCE = Self::letters_to_token(b"LCE"),
	
	/// TODO.
	LCF = Self::letters_to_token(b"LCF"),
	
	/// TODO.
	LCG = Self::letters_to_token(b"LCG"),
	
	/// TODO.
	LCH = Self::letters_to_token(b"LCH"),
	
	/// TODO.
	LCI = Self::letters_to_token(b"LCI"),
	
	/// TODO.
	LCJ = Self::letters_to_token(b"LCJ"),
	
	/// TODO.
	LCK = Self::letters_to_token(b"LCK"),
	
	/// TODO.
	LCL = Self::letters_to_token(b"LCL"),
	
	/// TODO.
	LCM = Self::letters_to_token(b"LCM"),
	
	/// TODO.
	LCN = Self::letters_to_token(b"LCN"),
	
	/// TODO.
	LCO = Self::letters_to_token(b"LCO"),
	
	/// TODO.
	LCP = Self::letters_to_token(b"LCP"),
	
	/// TODO.
	LCQ = Self::letters_to_token(b"LCQ"),
	
	/// TODO.
	LCR = Self::letters_to_token(b"LCR"),
	
	/// TODO.
	LCS = Self::letters_to_token(b"LCS"),
	
	/// TODO.
	LCT = Self::letters_to_token(b"LCT"),
	
	/// TODO.
	LCU = Self::letters_to_token(b"LCU"),
	
	/// TODO.
	LCV = Self::letters_to_token(b"LCV"),
	
	/// TODO.
	LCW = Self::letters_to_token(b"LCW"),
	
	/// TODO.
	LCX = Self::letters_to_token(b"LCX"),
	
	/// TODO.
	LCY = Self::letters_to_token(b"LCY"),
	
	/// TODO.
	LCZ = Self::letters_to_token(b"LCZ"),
	
	/// TODO.
	LDA = Self::letters_to_token(b"LDA"),
	
	/// TODO.
	LDB = Self::letters_to_token(b"LDB"),
	
	/// TODO.
	LDC = Self::letters_to_token(b"LDC"),
	
	/// TODO.
	LDD = Self::letters_to_token(b"LDD"),
	
	/// TODO.
	LDE = Self::letters_to_token(b"LDE"),
	
	/// TODO.
	LDF = Self::letters_to_token(b"LDF"),
	
	/// TODO.
	LDG = Self::letters_to_token(b"LDG"),
	
	/// TODO.
	LDH = Self::letters_to_token(b"LDH"),
	
	/// TODO.
	LDI = Self::letters_to_token(b"LDI"),
	
	/// TODO.
	LDJ = Self::letters_to_token(b"LDJ"),
	
	/// TODO.
	LDK = Self::letters_to_token(b"LDK"),
	
	/// TODO.
	LDL = Self::letters_to_token(b"LDL"),
	
	/// TODO.
	LDM = Self::letters_to_token(b"LDM"),
	
	/// TODO.
	LDN = Self::letters_to_token(b"LDN"),
	
	/// TODO.
	LDO = Self::letters_to_token(b"LDO"),
	
	/// TODO.
	LDP = Self::letters_to_token(b"LDP"),
	
	/// TODO.
	LDQ = Self::letters_to_token(b"LDQ"),
	
	/// TODO.
	LDR = Self::letters_to_token(b"LDR"),
	
	/// TODO.
	LDS = Self::letters_to_token(b"LDS"),
	
	/// TODO.
	LDT = Self::letters_to_token(b"LDT"),
	
	/// TODO.
	LDU = Self::letters_to_token(b"LDU"),
	
	/// TODO.
	LDV = Self::letters_to_token(b"LDV"),
	
	/// TODO.
	LDW = Self::letters_to_token(b"LDW"),
	
	/// TODO.
	LDX = Self::letters_to_token(b"LDX"),
	
	/// TODO.
	LDY = Self::letters_to_token(b"LDY"),
	
	/// TODO.
	LDZ = Self::letters_to_token(b"LDZ"),
	
	/// TODO.
	LEA = Self::letters_to_token(b"LEA"),
	
	/// TODO.
	LEB = Self::letters_to_token(b"LEB"),
	
	/// TODO.
	LEC = Self::letters_to_token(b"LEC"),
	
	/// TODO.
	LED = Self::letters_to_token(b"LED"),
	
	/// TODO.
	LEE = Self::letters_to_token(b"LEE"),
	
	/// TODO.
	LEF = Self::letters_to_token(b"LEF"),
	
	/// TODO.
	LEG = Self::letters_to_token(b"LEG"),
	
	/// TODO.
	LEH = Self::letters_to_token(b"LEH"),
	
	/// TODO.
	LEI = Self::letters_to_token(b"LEI"),
	
	/// TODO.
	LEJ = Self::letters_to_token(b"LEJ"),
	
	/// TODO.
	LEK = Self::letters_to_token(b"LEK"),
	
	/// TODO.
	LEL = Self::letters_to_token(b"LEL"),
	
	/// TODO.
	LEM = Self::letters_to_token(b"LEM"),
	
	/// TODO.
	LEN = Self::letters_to_token(b"LEN"),
	
	/// TODO.
	LEO = Self::letters_to_token(b"LEO"),
	
	/// TODO.
	LEP = Self::letters_to_token(b"LEP"),
	
	/// TODO.
	LEQ = Self::letters_to_token(b"LEQ"),
	
	/// TODO.
	LER = Self::letters_to_token(b"LER"),
	
	/// TODO.
	LES = Self::letters_to_token(b"LES"),
	
	/// TODO.
	LET = Self::letters_to_token(b"LET"),
	
	/// TODO.
	LEU = Self::letters_to_token(b"LEU"),
	
	/// TODO.
	LEV = Self::letters_to_token(b"LEV"),
	
	/// TODO.
	LEW = Self::letters_to_token(b"LEW"),
	
	/// TODO.
	LEX = Self::letters_to_token(b"LEX"),
	
	/// TODO.
	LEY = Self::letters_to_token(b"LEY"),
	
	/// TODO.
	LEZ = Self::letters_to_token(b"LEZ"),
	
	/// TODO.
	LFA = Self::letters_to_token(b"LFA"),
	
	/// TODO.
	LFB = Self::letters_to_token(b"LFB"),
	
	/// TODO.
	LFC = Self::letters_to_token(b"LFC"),
	
	/// TODO.
	LFD = Self::letters_to_token(b"LFD"),
	
	/// TODO.
	LFE = Self::letters_to_token(b"LFE"),
	
	/// TODO.
	LFF = Self::letters_to_token(b"LFF"),
	
	/// TODO.
	LFG = Self::letters_to_token(b"LFG"),
	
	/// TODO.
	LFH = Self::letters_to_token(b"LFH"),
	
	/// TODO.
	LFI = Self::letters_to_token(b"LFI"),
	
	/// TODO.
	LFJ = Self::letters_to_token(b"LFJ"),
	
	/// TODO.
	LFK = Self::letters_to_token(b"LFK"),
	
	/// TODO.
	LFL = Self::letters_to_token(b"LFL"),
	
	/// TODO.
	LFM = Self::letters_to_token(b"LFM"),
	
	/// TODO.
	LFN = Self::letters_to_token(b"LFN"),
	
	/// TODO.
	LFO = Self::letters_to_token(b"LFO"),
	
	/// TODO.
	LFP = Self::letters_to_token(b"LFP"),
	
	/// TODO.
	LFQ = Self::letters_to_token(b"LFQ"),
	
	/// TODO.
	LFR = Self::letters_to_token(b"LFR"),
	
	/// TODO.
	LFS = Self::letters_to_token(b"LFS"),
	
	/// TODO.
	LFT = Self::letters_to_token(b"LFT"),
	
	/// TODO.
	LFU = Self::letters_to_token(b"LFU"),
	
	/// TODO.
	LFV = Self::letters_to_token(b"LFV"),
	
	/// TODO.
	LFW = Self::letters_to_token(b"LFW"),
	
	/// TODO.
	LFX = Self::letters_to_token(b"LFX"),
	
	/// TODO.
	LFY = Self::letters_to_token(b"LFY"),
	
	/// TODO.
	LFZ = Self::letters_to_token(b"LFZ"),
	
	/// TODO.
	LGA = Self::letters_to_token(b"LGA"),
	
	/// TODO.
	LGB = Self::letters_to_token(b"LGB"),
	
	/// TODO.
	LGC = Self::letters_to_token(b"LGC"),
	
	/// TODO.
	LGD = Self::letters_to_token(b"LGD"),
	
	/// TODO.
	LGE = Self::letters_to_token(b"LGE"),
	
	/// TODO.
	LGF = Self::letters_to_token(b"LGF"),
	
	/// TODO.
	LGG = Self::letters_to_token(b"LGG"),
	
	/// TODO.
	LGH = Self::letters_to_token(b"LGH"),
	
	/// TODO.
	LGI = Self::letters_to_token(b"LGI"),
	
	/// TODO.
	LGJ = Self::letters_to_token(b"LGJ"),
	
	/// TODO.
	LGK = Self::letters_to_token(b"LGK"),
	
	/// TODO.
	LGL = Self::letters_to_token(b"LGL"),
	
	/// TODO.
	LGM = Self::letters_to_token(b"LGM"),
	
	/// TODO.
	LGN = Self::letters_to_token(b"LGN"),
	
	/// TODO.
	LGO = Self::letters_to_token(b"LGO"),
	
	/// TODO.
	LGP = Self::letters_to_token(b"LGP"),
	
	/// TODO.
	LGQ = Self::letters_to_token(b"LGQ"),
	
	/// TODO.
	LGR = Self::letters_to_token(b"LGR"),
	
	/// TODO.
	LGS = Self::letters_to_token(b"LGS"),
	
	/// TODO.
	LGT = Self::letters_to_token(b"LGT"),
	
	/// TODO.
	LGU = Self::letters_to_token(b"LGU"),
	
	/// TODO.
	LGV = Self::letters_to_token(b"LGV"),
	
	/// TODO.
	LGW = Self::letters_to_token(b"LGW"),
	
	/// TODO.
	LGX = Self::letters_to_token(b"LGX"),
	
	/// TODO.
	LGY = Self::letters_to_token(b"LGY"),
	
	/// TODO.
	LGZ = Self::letters_to_token(b"LGZ"),
	
	/// TODO.
	LHA = Self::letters_to_token(b"LHA"),
	
	/// TODO.
	LHB = Self::letters_to_token(b"LHB"),
	
	/// TODO.
	LHC = Self::letters_to_token(b"LHC"),
	
	/// TODO.
	LHD = Self::letters_to_token(b"LHD"),
	
	/// TODO.
	LHE = Self::letters_to_token(b"LHE"),
	
	/// TODO.
	LHF = Self::letters_to_token(b"LHF"),
	
	/// TODO.
	LHG = Self::letters_to_token(b"LHG"),
	
	/// TODO.
	LHH = Self::letters_to_token(b"LHH"),
	
	/// TODO.
	LHI = Self::letters_to_token(b"LHI"),
	
	/// TODO.
	LHJ = Self::letters_to_token(b"LHJ"),
	
	/// TODO.
	LHK = Self::letters_to_token(b"LHK"),
	
	/// TODO.
	LHL = Self::letters_to_token(b"LHL"),
	
	/// TODO.
	LHM = Self::letters_to_token(b"LHM"),
	
	/// TODO.
	LHN = Self::letters_to_token(b"LHN"),
	
	/// TODO.
	LHO = Self::letters_to_token(b"LHO"),
	
	/// TODO.
	LHP = Self::letters_to_token(b"LHP"),
	
	/// TODO.
	LHQ = Self::letters_to_token(b"LHQ"),
	
	/// TODO.
	LHR = Self::letters_to_token(b"LHR"),
	
	/// TODO.
	LHS = Self::letters_to_token(b"LHS"),
	
	/// TODO.
	LHT = Self::letters_to_token(b"LHT"),
	
	/// TODO.
	LHU = Self::letters_to_token(b"LHU"),
	
	/// TODO.
	LHV = Self::letters_to_token(b"LHV"),
	
	/// TODO.
	LHW = Self::letters_to_token(b"LHW"),
	
	/// TODO.
	LHX = Self::letters_to_token(b"LHX"),
	
	/// TODO.
	LHY = Self::letters_to_token(b"LHY"),
	
	/// TODO.
	LHZ = Self::letters_to_token(b"LHZ"),
	
	/// TODO.
	LIA = Self::letters_to_token(b"LIA"),
	
	/// TODO.
	LIB = Self::letters_to_token(b"LIB"),
	
	/// TODO.
	LIC = Self::letters_to_token(b"LIC"),
	
	/// TODO.
	LID = Self::letters_to_token(b"LID"),
	
	/// TODO.
	LIE = Self::letters_to_token(b"LIE"),
	
	/// TODO.
	LIF = Self::letters_to_token(b"LIF"),
	
	/// TODO.
	LIG = Self::letters_to_token(b"LIG"),
	
	/// TODO.
	LIH = Self::letters_to_token(b"LIH"),
	
	/// TODO.
	LII = Self::letters_to_token(b"LII"),
	
	/// TODO.
	LIJ = Self::letters_to_token(b"LIJ"),
	
	/// TODO.
	LIK = Self::letters_to_token(b"LIK"),
	
	/// TODO.
	LIL = Self::letters_to_token(b"LIL"),
	
	/// TODO.
	LIM = Self::letters_to_token(b"LIM"),
	
	/// TODO.
	LIN = Self::letters_to_token(b"LIN"),
	
	/// TODO.
	LIO = Self::letters_to_token(b"LIO"),
	
	/// TODO.
	LIP = Self::letters_to_token(b"LIP"),
	
	/// TODO.
	LIQ = Self::letters_to_token(b"LIQ"),
	
	/// TODO.
	LIR = Self::letters_to_token(b"LIR"),
	
	/// TODO.
	LIS = Self::letters_to_token(b"LIS"),
	
	/// TODO.
	LIT = Self::letters_to_token(b"LIT"),
	
	/// TODO.
	LIU = Self::letters_to_token(b"LIU"),
	
	/// TODO.
	LIV = Self::letters_to_token(b"LIV"),
	
	/// TODO.
	LIW = Self::letters_to_token(b"LIW"),
	
	/// TODO.
	LIX = Self::letters_to_token(b"LIX"),
	
	/// TODO.
	LIY = Self::letters_to_token(b"LIY"),
	
	/// TODO.
	LIZ = Self::letters_to_token(b"LIZ"),
	
	/// TODO.
	LJA = Self::letters_to_token(b"LJA"),
	
	/// TODO.
	LJB = Self::letters_to_token(b"LJB"),
	
	/// TODO.
	LJC = Self::letters_to_token(b"LJC"),
	
	/// TODO.
	LJD = Self::letters_to_token(b"LJD"),
	
	/// TODO.
	LJE = Self::letters_to_token(b"LJE"),
	
	/// TODO.
	LJF = Self::letters_to_token(b"LJF"),
	
	/// TODO.
	LJG = Self::letters_to_token(b"LJG"),
	
	/// TODO.
	LJH = Self::letters_to_token(b"LJH"),
	
	/// TODO.
	LJI = Self::letters_to_token(b"LJI"),
	
	/// TODO.
	LJJ = Self::letters_to_token(b"LJJ"),
	
	/// TODO.
	LJK = Self::letters_to_token(b"LJK"),
	
	/// TODO.
	LJL = Self::letters_to_token(b"LJL"),
	
	/// TODO.
	LJM = Self::letters_to_token(b"LJM"),
	
	/// TODO.
	LJN = Self::letters_to_token(b"LJN"),
	
	/// TODO.
	LJO = Self::letters_to_token(b"LJO"),
	
	/// TODO.
	LJP = Self::letters_to_token(b"LJP"),
	
	/// TODO.
	LJQ = Self::letters_to_token(b"LJQ"),
	
	/// TODO.
	LJR = Self::letters_to_token(b"LJR"),
	
	/// TODO.
	LJS = Self::letters_to_token(b"LJS"),
	
	/// TODO.
	LJT = Self::letters_to_token(b"LJT"),
	
	/// TODO.
	LJU = Self::letters_to_token(b"LJU"),
	
	/// TODO.
	LJV = Self::letters_to_token(b"LJV"),
	
	/// TODO.
	LJW = Self::letters_to_token(b"LJW"),
	
	/// TODO.
	LJX = Self::letters_to_token(b"LJX"),
	
	/// TODO.
	LJY = Self::letters_to_token(b"LJY"),
	
	/// TODO.
	LJZ = Self::letters_to_token(b"LJZ"),
	
	/// TODO.
	LKA = Self::letters_to_token(b"LKA"),
	
	/// TODO.
	LKB = Self::letters_to_token(b"LKB"),
	
	/// TODO.
	LKC = Self::letters_to_token(b"LKC"),
	
	/// TODO.
	LKD = Self::letters_to_token(b"LKD"),
	
	/// TODO.
	LKE = Self::letters_to_token(b"LKE"),
	
	/// TODO.
	LKF = Self::letters_to_token(b"LKF"),
	
	/// TODO.
	LKG = Self::letters_to_token(b"LKG"),
	
	/// TODO.
	LKH = Self::letters_to_token(b"LKH"),
	
	/// TODO.
	LKI = Self::letters_to_token(b"LKI"),
	
	/// TODO.
	LKJ = Self::letters_to_token(b"LKJ"),
	
	/// TODO.
	LKK = Self::letters_to_token(b"LKK"),
	
	/// TODO.
	LKL = Self::letters_to_token(b"LKL"),
	
	/// TODO.
	LKM = Self::letters_to_token(b"LKM"),
	
	/// TODO.
	LKN = Self::letters_to_token(b"LKN"),
	
	/// TODO.
	LKO = Self::letters_to_token(b"LKO"),
	
	/// TODO.
	LKP = Self::letters_to_token(b"LKP"),
	
	/// TODO.
	LKQ = Self::letters_to_token(b"LKQ"),
	
	/// TODO.
	LKR = Self::letters_to_token(b"LKR"),
	
	/// TODO.
	LKS = Self::letters_to_token(b"LKS"),
	
	/// TODO.
	LKT = Self::letters_to_token(b"LKT"),
	
	/// TODO.
	LKU = Self::letters_to_token(b"LKU"),
	
	/// TODO.
	LKV = Self::letters_to_token(b"LKV"),
	
	/// TODO.
	LKW = Self::letters_to_token(b"LKW"),
	
	/// TODO.
	LKX = Self::letters_to_token(b"LKX"),
	
	/// TODO.
	LKY = Self::letters_to_token(b"LKY"),
	
	/// TODO.
	LKZ = Self::letters_to_token(b"LKZ"),
	
	/// TODO.
	LLA = Self::letters_to_token(b"LLA"),
	
	/// TODO.
	LLB = Self::letters_to_token(b"LLB"),
	
	/// TODO.
	LLC = Self::letters_to_token(b"LLC"),
	
	/// TODO.
	LLD = Self::letters_to_token(b"LLD"),
	
	/// TODO.
	LLE = Self::letters_to_token(b"LLE"),
	
	/// TODO.
	LLF = Self::letters_to_token(b"LLF"),
	
	/// TODO.
	LLG = Self::letters_to_token(b"LLG"),
	
	/// TODO.
	LLH = Self::letters_to_token(b"LLH"),
	
	/// TODO.
	LLI = Self::letters_to_token(b"LLI"),
	
	/// TODO.
	LLJ = Self::letters_to_token(b"LLJ"),
	
	/// TODO.
	LLK = Self::letters_to_token(b"LLK"),
	
	/// TODO.
	LLL = Self::letters_to_token(b"LLL"),
	
	/// TODO.
	LLM = Self::letters_to_token(b"LLM"),
	
	/// TODO.
	LLN = Self::letters_to_token(b"LLN"),
	
	/// TODO.
	LLO = Self::letters_to_token(b"LLO"),
	
	/// TODO.
	LLP = Self::letters_to_token(b"LLP"),
	
	/// TODO.
	LLQ = Self::letters_to_token(b"LLQ"),
	
	/// TODO.
	LLR = Self::letters_to_token(b"LLR"),
	
	/// TODO.
	LLS = Self::letters_to_token(b"LLS"),
	
	/// TODO.
	LLT = Self::letters_to_token(b"LLT"),
	
	/// TODO.
	LLU = Self::letters_to_token(b"LLU"),
	
	/// TODO.
	LLV = Self::letters_to_token(b"LLV"),
	
	/// TODO.
	LLW = Self::letters_to_token(b"LLW"),
	
	/// TODO.
	LLX = Self::letters_to_token(b"LLX"),
	
	/// TODO.
	LLY = Self::letters_to_token(b"LLY"),
	
	/// TODO.
	LLZ = Self::letters_to_token(b"LLZ"),
	
	/// TODO.
	LMA = Self::letters_to_token(b"LMA"),
	
	/// TODO.
	LMB = Self::letters_to_token(b"LMB"),
	
	/// TODO.
	LMC = Self::letters_to_token(b"LMC"),
	
	/// TODO.
	LMD = Self::letters_to_token(b"LMD"),
	
	/// TODO.
	LME = Self::letters_to_token(b"LME"),
	
	/// TODO.
	LMF = Self::letters_to_token(b"LMF"),
	
	/// TODO.
	LMG = Self::letters_to_token(b"LMG"),
	
	/// TODO.
	LMH = Self::letters_to_token(b"LMH"),
	
	/// TODO.
	LMI = Self::letters_to_token(b"LMI"),
	
	/// TODO.
	LMJ = Self::letters_to_token(b"LMJ"),
	
	/// TODO.
	LMK = Self::letters_to_token(b"LMK"),
	
	/// TODO.
	LML = Self::letters_to_token(b"LML"),
	
	/// TODO.
	LMM = Self::letters_to_token(b"LMM"),
	
	/// TODO.
	LMN = Self::letters_to_token(b"LMN"),
	
	/// TODO.
	LMO = Self::letters_to_token(b"LMO"),
	
	/// TODO.
	LMP = Self::letters_to_token(b"LMP"),
	
	/// TODO.
	LMQ = Self::letters_to_token(b"LMQ"),
	
	/// TODO.
	LMR = Self::letters_to_token(b"LMR"),
	
	/// TODO.
	LMS = Self::letters_to_token(b"LMS"),
	
	/// TODO.
	LMT = Self::letters_to_token(b"LMT"),
	
	/// TODO.
	LMU = Self::letters_to_token(b"LMU"),
	
	/// TODO.
	LMV = Self::letters_to_token(b"LMV"),
	
	/// TODO.
	LMW = Self::letters_to_token(b"LMW"),
	
	/// TODO.
	LMX = Self::letters_to_token(b"LMX"),
	
	/// TODO.
	LMY = Self::letters_to_token(b"LMY"),
	
	/// TODO.
	LMZ = Self::letters_to_token(b"LMZ"),
	
	/// TODO.
	LNA = Self::letters_to_token(b"LNA"),
	
	/// TODO.
	LNB = Self::letters_to_token(b"LNB"),
	
	/// TODO.
	LNC = Self::letters_to_token(b"LNC"),
	
	/// TODO.
	LND = Self::letters_to_token(b"LND"),
	
	/// TODO.
	LNE = Self::letters_to_token(b"LNE"),
	
	/// TODO.
	LNF = Self::letters_to_token(b"LNF"),
	
	/// TODO.
	LNG = Self::letters_to_token(b"LNG"),
	
	/// TODO.
	LNH = Self::letters_to_token(b"LNH"),
	
	/// TODO.
	LNI = Self::letters_to_token(b"LNI"),
	
	/// TODO.
	LNJ = Self::letters_to_token(b"LNJ"),
	
	/// TODO.
	LNK = Self::letters_to_token(b"LNK"),
	
	/// TODO.
	LNL = Self::letters_to_token(b"LNL"),
	
	/// TODO.
	LNM = Self::letters_to_token(b"LNM"),
	
	/// TODO.
	LNN = Self::letters_to_token(b"LNN"),
	
	/// TODO.
	LNO = Self::letters_to_token(b"LNO"),
	
	/// TODO.
	LNP = Self::letters_to_token(b"LNP"),
	
	/// TODO.
	LNQ = Self::letters_to_token(b"LNQ"),
	
	/// TODO.
	LNR = Self::letters_to_token(b"LNR"),
	
	/// TODO.
	LNS = Self::letters_to_token(b"LNS"),
	
	/// TODO.
	LNT = Self::letters_to_token(b"LNT"),
	
	/// TODO.
	LNU = Self::letters_to_token(b"LNU"),
	
	/// TODO.
	LNV = Self::letters_to_token(b"LNV"),
	
	/// TODO.
	LNW = Self::letters_to_token(b"LNW"),
	
	/// TODO.
	LNX = Self::letters_to_token(b"LNX"),
	
	/// TODO.
	LNY = Self::letters_to_token(b"LNY"),
	
	/// TODO.
	LNZ = Self::letters_to_token(b"LNZ"),
	
	/// TODO.
	LOA = Self::letters_to_token(b"LOA"),
	
	/// TODO.
	LOB = Self::letters_to_token(b"LOB"),
	
	/// TODO.
	LOC = Self::letters_to_token(b"LOC"),
	
	/// TODO.
	LOD = Self::letters_to_token(b"LOD"),
	
	/// TODO.
	LOE = Self::letters_to_token(b"LOE"),
	
	/// TODO.
	LOF = Self::letters_to_token(b"LOF"),
	
	/// TODO.
	LOG = Self::letters_to_token(b"LOG"),
	
	/// TODO.
	LOH = Self::letters_to_token(b"LOH"),
	
	/// TODO.
	LOI = Self::letters_to_token(b"LOI"),
	
	/// TODO.
	LOJ = Self::letters_to_token(b"LOJ"),
	
	/// TODO.
	LOK = Self::letters_to_token(b"LOK"),
	
	/// TODO.
	LOL = Self::letters_to_token(b"LOL"),
	
	/// TODO.
	LOM = Self::letters_to_token(b"LOM"),
	
	/// TODO.
	LON = Self::letters_to_token(b"LON"),
	
	/// TODO.
	LOO = Self::letters_to_token(b"LOO"),
	
	/// TODO.
	LOP = Self::letters_to_token(b"LOP"),
	
	/// TODO.
	LOQ = Self::letters_to_token(b"LOQ"),
	
	/// TODO.
	LOR = Self::letters_to_token(b"LOR"),
	
	/// TODO.
	LOS = Self::letters_to_token(b"LOS"),
	
	/// TODO.
	LOT = Self::letters_to_token(b"LOT"),
	
	/// TODO.
	LOU = Self::letters_to_token(b"LOU"),
	
	/// TODO.
	LOV = Self::letters_to_token(b"LOV"),
	
	/// TODO.
	LOW = Self::letters_to_token(b"LOW"),
	
	/// TODO.
	LOX = Self::letters_to_token(b"LOX"),
	
	/// TODO.
	LOY = Self::letters_to_token(b"LOY"),
	
	/// TODO.
	LOZ = Self::letters_to_token(b"LOZ"),
	
	/// TODO.
	LPA = Self::letters_to_token(b"LPA"),
	
	/// TODO.
	LPB = Self::letters_to_token(b"LPB"),
	
	/// TODO.
	LPC = Self::letters_to_token(b"LPC"),
	
	/// TODO.
	LPD = Self::letters_to_token(b"LPD"),
	
	/// TODO.
	LPE = Self::letters_to_token(b"LPE"),
	
	/// TODO.
	LPF = Self::letters_to_token(b"LPF"),
	
	/// TODO.
	LPG = Self::letters_to_token(b"LPG"),
	
	/// TODO.
	LPH = Self::letters_to_token(b"LPH"),
	
	/// TODO.
	LPI = Self::letters_to_token(b"LPI"),
	
	/// TODO.
	LPJ = Self::letters_to_token(b"LPJ"),
	
	/// TODO.
	LPK = Self::letters_to_token(b"LPK"),
	
	/// TODO.
	LPL = Self::letters_to_token(b"LPL"),
	
	/// TODO.
	LPM = Self::letters_to_token(b"LPM"),
	
	/// TODO.
	LPN = Self::letters_to_token(b"LPN"),
	
	/// TODO.
	LPO = Self::letters_to_token(b"LPO"),
	
	/// TODO.
	LPP = Self::letters_to_token(b"LPP"),
	
	/// TODO.
	LPQ = Self::letters_to_token(b"LPQ"),
	
	/// TODO.
	LPR = Self::letters_to_token(b"LPR"),
	
	/// TODO.
	LPS = Self::letters_to_token(b"LPS"),
	
	/// TODO.
	LPT = Self::letters_to_token(b"LPT"),
	
	/// TODO.
	LPU = Self::letters_to_token(b"LPU"),
	
	/// TODO.
	LPV = Self::letters_to_token(b"LPV"),
	
	/// TODO.
	LPW = Self::letters_to_token(b"LPW"),
	
	/// TODO.
	LPX = Self::letters_to_token(b"LPX"),
	
	/// TODO.
	LPY = Self::letters_to_token(b"LPY"),
	
	/// TODO.
	LPZ = Self::letters_to_token(b"LPZ"),
	
	/// TODO.
	LQA = Self::letters_to_token(b"LQA"),
	
	/// TODO.
	LQB = Self::letters_to_token(b"LQB"),
	
	/// TODO.
	LQC = Self::letters_to_token(b"LQC"),
	
	/// TODO.
	LQD = Self::letters_to_token(b"LQD"),
	
	/// TODO.
	LQE = Self::letters_to_token(b"LQE"),
	
	/// TODO.
	LQF = Self::letters_to_token(b"LQF"),
	
	/// TODO.
	LQG = Self::letters_to_token(b"LQG"),
	
	/// TODO.
	LQH = Self::letters_to_token(b"LQH"),
	
	/// TODO.
	LQI = Self::letters_to_token(b"LQI"),
	
	/// TODO.
	LQJ = Self::letters_to_token(b"LQJ"),
	
	/// TODO.
	LQK = Self::letters_to_token(b"LQK"),
	
	/// TODO.
	LQL = Self::letters_to_token(b"LQL"),
	
	/// TODO.
	LQM = Self::letters_to_token(b"LQM"),
	
	/// TODO.
	LQN = Self::letters_to_token(b"LQN"),
	
	/// TODO.
	LQO = Self::letters_to_token(b"LQO"),
	
	/// TODO.
	LQP = Self::letters_to_token(b"LQP"),
	
	/// TODO.
	LQQ = Self::letters_to_token(b"LQQ"),
	
	/// TODO.
	LQR = Self::letters_to_token(b"LQR"),
	
	/// TODO.
	LQS = Self::letters_to_token(b"LQS"),
	
	/// TODO.
	LQT = Self::letters_to_token(b"LQT"),
	
	/// TODO.
	LQU = Self::letters_to_token(b"LQU"),
	
	/// TODO.
	LQV = Self::letters_to_token(b"LQV"),
	
	/// TODO.
	LQW = Self::letters_to_token(b"LQW"),
	
	/// TODO.
	LQX = Self::letters_to_token(b"LQX"),
	
	/// TODO.
	LQY = Self::letters_to_token(b"LQY"),
	
	/// TODO.
	LQZ = Self::letters_to_token(b"LQZ"),
	
	/// TODO.
	LRA = Self::letters_to_token(b"LRA"),
	
	/// TODO.
	LRB = Self::letters_to_token(b"LRB"),
	
	/// TODO.
	LRC = Self::letters_to_token(b"LRC"),
	
	/// TODO.
	LRD = Self::letters_to_token(b"LRD"),
	
	/// TODO.
	LRE = Self::letters_to_token(b"LRE"),
	
	/// TODO.
	LRF = Self::letters_to_token(b"LRF"),
	
	/// TODO.
	LRG = Self::letters_to_token(b"LRG"),
	
	/// TODO.
	LRH = Self::letters_to_token(b"LRH"),
	
	/// TODO.
	LRI = Self::letters_to_token(b"LRI"),
	
	/// TODO.
	LRJ = Self::letters_to_token(b"LRJ"),
	
	/// TODO.
	LRK = Self::letters_to_token(b"LRK"),
	
	/// TODO.
	LRL = Self::letters_to_token(b"LRL"),
	
	/// TODO.
	LRM = Self::letters_to_token(b"LRM"),
	
	/// TODO.
	LRN = Self::letters_to_token(b"LRN"),
	
	/// TODO.
	LRO = Self::letters_to_token(b"LRO"),
	
	/// TODO.
	LRP = Self::letters_to_token(b"LRP"),
	
	/// TODO.
	LRQ = Self::letters_to_token(b"LRQ"),
	
	/// TODO.
	LRR = Self::letters_to_token(b"LRR"),
	
	/// TODO.
	LRS = Self::letters_to_token(b"LRS"),
	
	/// TODO.
	LRT = Self::letters_to_token(b"LRT"),
	
	/// TODO.
	LRU = Self::letters_to_token(b"LRU"),
	
	/// TODO.
	LRV = Self::letters_to_token(b"LRV"),
	
	/// TODO.
	LRW = Self::letters_to_token(b"LRW"),
	
	/// TODO.
	LRX = Self::letters_to_token(b"LRX"),
	
	/// TODO.
	LRY = Self::letters_to_token(b"LRY"),
	
	/// TODO.
	LRZ = Self::letters_to_token(b"LRZ"),
	
	/// TODO.
	LSA = Self::letters_to_token(b"LSA"),
	
	/// TODO.
	LSB = Self::letters_to_token(b"LSB"),
	
	/// TODO.
	LSC = Self::letters_to_token(b"LSC"),
	
	/// TODO.
	LSD = Self::letters_to_token(b"LSD"),
	
	/// TODO.
	LSE = Self::letters_to_token(b"LSE"),
	
	/// TODO.
	LSF = Self::letters_to_token(b"LSF"),
	
	/// TODO.
	LSG = Self::letters_to_token(b"LSG"),
	
	/// TODO.
	LSH = Self::letters_to_token(b"LSH"),
	
	/// TODO.
	LSI = Self::letters_to_token(b"LSI"),
	
	/// TODO.
	LSJ = Self::letters_to_token(b"LSJ"),
	
	/// TODO.
	LSK = Self::letters_to_token(b"LSK"),
	
	/// TODO.
	LSL = Self::letters_to_token(b"LSL"),
	
	/// TODO.
	LSM = Self::letters_to_token(b"LSM"),
	
	/// TODO.
	LSN = Self::letters_to_token(b"LSN"),
	
	/// TODO.
	LSO = Self::letters_to_token(b"LSO"),
	
	/// TODO.
	LSP = Self::letters_to_token(b"LSP"),
	
	/// TODO.
	LSQ = Self::letters_to_token(b"LSQ"),
	
	/// TODO.
	LSR = Self::letters_to_token(b"LSR"),
	
	/// TODO.
	LSS = Self::letters_to_token(b"LSS"),
	
	/// TODO.
	LST = Self::letters_to_token(b"LST"),
	
	/// TODO.
	LSU = Self::letters_to_token(b"LSU"),
	
	/// TODO.
	LSV = Self::letters_to_token(b"LSV"),
	
	/// TODO.
	LSW = Self::letters_to_token(b"LSW"),
	
	/// TODO.
	LSX = Self::letters_to_token(b"LSX"),
	
	/// TODO.
	LSY = Self::letters_to_token(b"LSY"),
	
	/// TODO.
	LSZ = Self::letters_to_token(b"LSZ"),
	
	/// TODO.
	LTA = Self::letters_to_token(b"LTA"),
	
	/// TODO.
	LTB = Self::letters_to_token(b"LTB"),
	
	/// TODO.
	LTC = Self::letters_to_token(b"LTC"),
	
	/// TODO.
	LTD = Self::letters_to_token(b"LTD"),
	
	/// TODO.
	LTE = Self::letters_to_token(b"LTE"),
	
	/// TODO.
	LTF = Self::letters_to_token(b"LTF"),
	
	/// TODO.
	LTG = Self::letters_to_token(b"LTG"),
	
	/// TODO.
	LTH = Self::letters_to_token(b"LTH"),
	
	/// TODO.
	LTI = Self::letters_to_token(b"LTI"),
	
	/// TODO.
	LTJ = Self::letters_to_token(b"LTJ"),
	
	/// TODO.
	LTK = Self::letters_to_token(b"LTK"),
	
	/// TODO.
	LTL = Self::letters_to_token(b"LTL"),
	
	/// TODO.
	LTM = Self::letters_to_token(b"LTM"),
	
	/// TODO.
	LTN = Self::letters_to_token(b"LTN"),
	
	/// TODO.
	LTO = Self::letters_to_token(b"LTO"),
	
	/// TODO.
	LTP = Self::letters_to_token(b"LTP"),
	
	/// TODO.
	LTQ = Self::letters_to_token(b"LTQ"),
	
	/// TODO.
	LTR = Self::letters_to_token(b"LTR"),
	
	/// TODO.
	LTS = Self::letters_to_token(b"LTS"),
	
	/// TODO.
	LTT = Self::letters_to_token(b"LTT"),
	
	/// TODO.
	LTU = Self::letters_to_token(b"LTU"),
	
	/// TODO.
	LTV = Self::letters_to_token(b"LTV"),
	
	/// TODO.
	LTW = Self::letters_to_token(b"LTW"),
	
	/// TODO.
	LTX = Self::letters_to_token(b"LTX"),
	
	/// TODO.
	LTY = Self::letters_to_token(b"LTY"),
	
	/// TODO.
	LTZ = Self::letters_to_token(b"LTZ"),
	
	/// TODO.
	LUA = Self::letters_to_token(b"LUA"),
	
	/// TODO.
	LUB = Self::letters_to_token(b"LUB"),
	
	/// TODO.
	LUC = Self::letters_to_token(b"LUC"),
	
	/// TODO.
	LUD = Self::letters_to_token(b"LUD"),
	
	/// TODO.
	LUE = Self::letters_to_token(b"LUE"),
	
	/// TODO.
	LUF = Self::letters_to_token(b"LUF"),
	
	/// TODO.
	LUG = Self::letters_to_token(b"LUG"),
	
	/// TODO.
	LUH = Self::letters_to_token(b"LUH"),
	
	/// TODO.
	LUI = Self::letters_to_token(b"LUI"),
	
	/// TODO.
	LUJ = Self::letters_to_token(b"LUJ"),
	
	/// TODO.
	LUK = Self::letters_to_token(b"LUK"),
	
	/// TODO.
	LUL = Self::letters_to_token(b"LUL"),
	
	/// TODO.
	LUM = Self::letters_to_token(b"LUM"),
	
	/// TODO.
	LUN = Self::letters_to_token(b"LUN"),
	
	/// TODO.
	LUO = Self::letters_to_token(b"LUO"),
	
	/// TODO.
	LUP = Self::letters_to_token(b"LUP"),
	
	/// TODO.
	LUQ = Self::letters_to_token(b"LUQ"),
	
	/// TODO.
	LUR = Self::letters_to_token(b"LUR"),
	
	/// TODO.
	LUS = Self::letters_to_token(b"LUS"),
	
	/// TODO.
	LUT = Self::letters_to_token(b"LUT"),
	
	/// TODO.
	LUU = Self::letters_to_token(b"LUU"),
	
	/// TODO.
	LUV = Self::letters_to_token(b"LUV"),
	
	/// TODO.
	LUW = Self::letters_to_token(b"LUW"),
	
	/// TODO.
	LUX = Self::letters_to_token(b"LUX"),
	
	/// TODO.
	LUY = Self::letters_to_token(b"LUY"),
	
	/// TODO.
	LUZ = Self::letters_to_token(b"LUZ"),
	
	/// TODO.
	LVA = Self::letters_to_token(b"LVA"),
	
	/// TODO.
	LVB = Self::letters_to_token(b"LVB"),
	
	/// TODO.
	LVC = Self::letters_to_token(b"LVC"),
	
	/// TODO.
	LVD = Self::letters_to_token(b"LVD"),
	
	/// TODO.
	LVE = Self::letters_to_token(b"LVE"),
	
	/// TODO.
	LVF = Self::letters_to_token(b"LVF"),
	
	/// TODO.
	LVG = Self::letters_to_token(b"LVG"),
	
	/// TODO.
	LVH = Self::letters_to_token(b"LVH"),
	
	/// TODO.
	LVI = Self::letters_to_token(b"LVI"),
	
	/// TODO.
	LVJ = Self::letters_to_token(b"LVJ"),
	
	/// TODO.
	LVK = Self::letters_to_token(b"LVK"),
	
	/// TODO.
	LVL = Self::letters_to_token(b"LVL"),
	
	/// TODO.
	LVM = Self::letters_to_token(b"LVM"),
	
	/// TODO.
	LVN = Self::letters_to_token(b"LVN"),
	
	/// TODO.
	LVO = Self::letters_to_token(b"LVO"),
	
	/// TODO.
	LVP = Self::letters_to_token(b"LVP"),
	
	/// TODO.
	LVQ = Self::letters_to_token(b"LVQ"),
	
	/// TODO.
	LVR = Self::letters_to_token(b"LVR"),
	
	/// TODO.
	LVS = Self::letters_to_token(b"LVS"),
	
	/// TODO.
	LVT = Self::letters_to_token(b"LVT"),
	
	/// TODO.
	LVU = Self::letters_to_token(b"LVU"),
	
	/// TODO.
	LVV = Self::letters_to_token(b"LVV"),
	
	/// TODO.
	LVW = Self::letters_to_token(b"LVW"),
	
	/// TODO.
	LVX = Self::letters_to_token(b"LVX"),
	
	/// TODO.
	LVY = Self::letters_to_token(b"LVY"),
	
	/// TODO.
	LVZ = Self::letters_to_token(b"LVZ"),
	
	/// TODO.
	LWA = Self::letters_to_token(b"LWA"),
	
	/// TODO.
	LWB = Self::letters_to_token(b"LWB"),
	
	/// TODO.
	LWC = Self::letters_to_token(b"LWC"),
	
	/// TODO.
	LWD = Self::letters_to_token(b"LWD"),
	
	/// TODO.
	LWE = Self::letters_to_token(b"LWE"),
	
	/// TODO.
	LWF = Self::letters_to_token(b"LWF"),
	
	/// TODO.
	LWG = Self::letters_to_token(b"LWG"),
	
	/// TODO.
	LWH = Self::letters_to_token(b"LWH"),
	
	/// TODO.
	LWI = Self::letters_to_token(b"LWI"),
	
	/// TODO.
	LWJ = Self::letters_to_token(b"LWJ"),
	
	/// TODO.
	LWK = Self::letters_to_token(b"LWK"),
	
	/// TODO.
	LWL = Self::letters_to_token(b"LWL"),
	
	/// TODO.
	LWM = Self::letters_to_token(b"LWM"),
	
	/// TODO.
	LWN = Self::letters_to_token(b"LWN"),
	
	/// TODO.
	LWO = Self::letters_to_token(b"LWO"),
	
	/// TODO.
	LWP = Self::letters_to_token(b"LWP"),
	
	/// TODO.
	LWQ = Self::letters_to_token(b"LWQ"),
	
	/// TODO.
	LWR = Self::letters_to_token(b"LWR"),
	
	/// TODO.
	LWS = Self::letters_to_token(b"LWS"),
	
	/// TODO.
	LWT = Self::letters_to_token(b"LWT"),
	
	/// TODO.
	LWU = Self::letters_to_token(b"LWU"),
	
	/// TODO.
	LWV = Self::letters_to_token(b"LWV"),
	
	/// TODO.
	LWW = Self::letters_to_token(b"LWW"),
	
	/// TODO.
	LWX = Self::letters_to_token(b"LWX"),
	
	/// TODO.
	LWY = Self::letters_to_token(b"LWY"),
	
	/// TODO.
	LWZ = Self::letters_to_token(b"LWZ"),
	
	/// TODO.
	LXA = Self::letters_to_token(b"LXA"),
	
	/// TODO.
	LXB = Self::letters_to_token(b"LXB"),
	
	/// TODO.
	LXC = Self::letters_to_token(b"LXC"),
	
	/// TODO.
	LXD = Self::letters_to_token(b"LXD"),
	
	/// TODO.
	LXE = Self::letters_to_token(b"LXE"),
	
	/// TODO.
	LXF = Self::letters_to_token(b"LXF"),
	
	/// TODO.
	LXG = Self::letters_to_token(b"LXG"),
	
	/// TODO.
	LXH = Self::letters_to_token(b"LXH"),
	
	/// TODO.
	LXI = Self::letters_to_token(b"LXI"),
	
	/// TODO.
	LXJ = Self::letters_to_token(b"LXJ"),
	
	/// TODO.
	LXK = Self::letters_to_token(b"LXK"),
	
	/// TODO.
	LXL = Self::letters_to_token(b"LXL"),
	
	/// TODO.
	LXM = Self::letters_to_token(b"LXM"),
	
	/// TODO.
	LXN = Self::letters_to_token(b"LXN"),
	
	/// TODO.
	LXO = Self::letters_to_token(b"LXO"),
	
	/// TODO.
	LXP = Self::letters_to_token(b"LXP"),
	
	/// TODO.
	LXQ = Self::letters_to_token(b"LXQ"),
	
	/// TODO.
	LXR = Self::letters_to_token(b"LXR"),
	
	/// TODO.
	LXS = Self::letters_to_token(b"LXS"),
	
	/// TODO.
	LXT = Self::letters_to_token(b"LXT"),
	
	/// TODO.
	LXU = Self::letters_to_token(b"LXU"),
	
	/// TODO.
	LXV = Self::letters_to_token(b"LXV"),
	
	/// TODO.
	LXW = Self::letters_to_token(b"LXW"),
	
	/// TODO.
	LXX = Self::letters_to_token(b"LXX"),
	
	/// TODO.
	LXY = Self::letters_to_token(b"LXY"),
	
	/// TODO.
	LXZ = Self::letters_to_token(b"LXZ"),
	
	/// TODO.
	LYA = Self::letters_to_token(b"LYA"),
	
	/// TODO.
	LYB = Self::letters_to_token(b"LYB"),
	
	/// TODO.
	LYC = Self::letters_to_token(b"LYC"),
	
	/// TODO.
	LYD = Self::letters_to_token(b"LYD"),
	
	/// TODO.
	LYE = Self::letters_to_token(b"LYE"),
	
	/// TODO.
	LYF = Self::letters_to_token(b"LYF"),
	
	/// TODO.
	LYG = Self::letters_to_token(b"LYG"),
	
	/// TODO.
	LYH = Self::letters_to_token(b"LYH"),
	
	/// TODO.
	LYI = Self::letters_to_token(b"LYI"),
	
	/// TODO.
	LYJ = Self::letters_to_token(b"LYJ"),
	
	/// TODO.
	LYK = Self::letters_to_token(b"LYK"),
	
	/// TODO.
	LYL = Self::letters_to_token(b"LYL"),
	
	/// TODO.
	LYM = Self::letters_to_token(b"LYM"),
	
	/// TODO.
	LYN = Self::letters_to_token(b"LYN"),
	
	/// TODO.
	LYO = Self::letters_to_token(b"LYO"),
	
	/// TODO.
	LYP = Self::letters_to_token(b"LYP"),
	
	/// TODO.
	LYQ = Self::letters_to_token(b"LYQ"),
	
	/// TODO.
	LYR = Self::letters_to_token(b"LYR"),
	
	/// TODO.
	LYS = Self::letters_to_token(b"LYS"),
	
	/// TODO.
	LYT = Self::letters_to_token(b"LYT"),
	
	/// TODO.
	LYU = Self::letters_to_token(b"LYU"),
	
	/// TODO.
	LYV = Self::letters_to_token(b"LYV"),
	
	/// TODO.
	LYW = Self::letters_to_token(b"LYW"),
	
	/// TODO.
	LYX = Self::letters_to_token(b"LYX"),
	
	/// TODO.
	LYY = Self::letters_to_token(b"LYY"),
	
	/// TODO.
	LYZ = Self::letters_to_token(b"LYZ"),
	
	/// TODO.
	LZA = Self::letters_to_token(b"LZA"),
	
	/// TODO.
	LZB = Self::letters_to_token(b"LZB"),
	
	/// TODO.
	LZC = Self::letters_to_token(b"LZC"),
	
	/// TODO.
	LZD = Self::letters_to_token(b"LZD"),
	
	/// TODO.
	LZE = Self::letters_to_token(b"LZE"),
	
	/// TODO.
	LZF = Self::letters_to_token(b"LZF"),
	
	/// TODO.
	LZG = Self::letters_to_token(b"LZG"),
	
	/// TODO.
	LZH = Self::letters_to_token(b"LZH"),
	
	/// TODO.
	LZI = Self::letters_to_token(b"LZI"),
	
	/// TODO.
	LZJ = Self::letters_to_token(b"LZJ"),
	
	/// TODO.
	LZK = Self::letters_to_token(b"LZK"),
	
	/// TODO.
	LZL = Self::letters_to_token(b"LZL"),
	
	/// TODO.
	LZM = Self::letters_to_token(b"LZM"),
	
	/// TODO.
	LZN = Self::letters_to_token(b"LZN"),
	
	/// TODO.
	LZO = Self::letters_to_token(b"LZO"),
	
	/// TODO.
	LZP = Self::letters_to_token(b"LZP"),
	
	/// TODO.
	LZQ = Self::letters_to_token(b"LZQ"),
	
	/// TODO.
	LZR = Self::letters_to_token(b"LZR"),
	
	/// TODO.
	LZS = Self::letters_to_token(b"LZS"),
	
	/// TODO.
	LZT = Self::letters_to_token(b"LZT"),
	
	/// TODO.
	LZU = Self::letters_to_token(b"LZU"),
	
	/// TODO.
	LZV = Self::letters_to_token(b"LZV"),
	
	/// TODO.
	LZW = Self::letters_to_token(b"LZW"),
	
	/// TODO.
	LZX = Self::letters_to_token(b"LZX"),
	
	/// TODO.
	LZY = Self::letters_to_token(b"LZY"),
	
	/// TODO.
	LZZ = Self::letters_to_token(b"LZZ"),
	
	/// TODO.
	MAA = Self::letters_to_token(b"MAA"),
	
	/// TODO.
	MAB = Self::letters_to_token(b"MAB"),
	
	/// TODO.
	MAC = Self::letters_to_token(b"MAC"),
	
	/// TODO.
	MAD = Self::letters_to_token(b"MAD"),
	
	/// TODO.
	MAE = Self::letters_to_token(b"MAE"),
	
	/// TODO.
	MAF = Self::letters_to_token(b"MAF"),
	
	/// TODO.
	MAG = Self::letters_to_token(b"MAG"),
	
	/// TODO.
	MAH = Self::letters_to_token(b"MAH"),
	
	/// TODO.
	MAI = Self::letters_to_token(b"MAI"),
	
	/// TODO.
	MAJ = Self::letters_to_token(b"MAJ"),
	
	/// TODO.
	MAK = Self::letters_to_token(b"MAK"),
	
	/// TODO.
	MAL = Self::letters_to_token(b"MAL"),
	
	/// TODO.
	MAM = Self::letters_to_token(b"MAM"),
	
	/// TODO.
	MAN = Self::letters_to_token(b"MAN"),
	
	/// TODO.
	MAO = Self::letters_to_token(b"MAO"),
	
	/// TODO.
	MAP = Self::letters_to_token(b"MAP"),
	
	/// TODO.
	MAQ = Self::letters_to_token(b"MAQ"),
	
	/// TODO.
	MAR = Self::letters_to_token(b"MAR"),
	
	/// TODO.
	MAS = Self::letters_to_token(b"MAS"),
	
	/// TODO.
	MAT = Self::letters_to_token(b"MAT"),
	
	/// TODO.
	MAU = Self::letters_to_token(b"MAU"),
	
	/// TODO.
	MAV = Self::letters_to_token(b"MAV"),
	
	/// TODO.
	MAW = Self::letters_to_token(b"MAW"),
	
	/// TODO.
	MAX = Self::letters_to_token(b"MAX"),
	
	/// TODO.
	MAY = Self::letters_to_token(b"MAY"),
	
	/// TODO.
	MAZ = Self::letters_to_token(b"MAZ"),
	
	/// TODO.
	MBA = Self::letters_to_token(b"MBA"),
	
	/// TODO.
	MBB = Self::letters_to_token(b"MBB"),
	
	/// TODO.
	MBC = Self::letters_to_token(b"MBC"),
	
	/// TODO.
	MBD = Self::letters_to_token(b"MBD"),
	
	/// TODO.
	MBE = Self::letters_to_token(b"MBE"),
	
	/// TODO.
	MBF = Self::letters_to_token(b"MBF"),
	
	/// TODO.
	MBG = Self::letters_to_token(b"MBG"),
	
	/// TODO.
	MBH = Self::letters_to_token(b"MBH"),
	
	/// TODO.
	MBI = Self::letters_to_token(b"MBI"),
	
	/// TODO.
	MBJ = Self::letters_to_token(b"MBJ"),
	
	/// TODO.
	MBK = Self::letters_to_token(b"MBK"),
	
	/// TODO.
	MBL = Self::letters_to_token(b"MBL"),
	
	/// TODO.
	MBM = Self::letters_to_token(b"MBM"),
	
	/// TODO.
	MBN = Self::letters_to_token(b"MBN"),
	
	/// TODO.
	MBO = Self::letters_to_token(b"MBO"),
	
	/// TODO.
	MBP = Self::letters_to_token(b"MBP"),
	
	/// TODO.
	MBQ = Self::letters_to_token(b"MBQ"),
	
	/// TODO.
	MBR = Self::letters_to_token(b"MBR"),
	
	/// TODO.
	MBS = Self::letters_to_token(b"MBS"),
	
	/// TODO.
	MBT = Self::letters_to_token(b"MBT"),
	
	/// TODO.
	MBU = Self::letters_to_token(b"MBU"),
	
	/// TODO.
	MBV = Self::letters_to_token(b"MBV"),
	
	/// TODO.
	MBW = Self::letters_to_token(b"MBW"),
	
	/// TODO.
	MBX = Self::letters_to_token(b"MBX"),
	
	/// TODO.
	MBY = Self::letters_to_token(b"MBY"),
	
	/// TODO.
	MBZ = Self::letters_to_token(b"MBZ"),
	
	/// TODO.
	MCA = Self::letters_to_token(b"MCA"),
	
	/// TODO.
	MCB = Self::letters_to_token(b"MCB"),
	
	/// TODO.
	MCC = Self::letters_to_token(b"MCC"),
	
	/// TODO.
	MCD = Self::letters_to_token(b"MCD"),
	
	/// TODO.
	MCE = Self::letters_to_token(b"MCE"),
	
	/// TODO.
	MCF = Self::letters_to_token(b"MCF"),
	
	/// TODO.
	MCG = Self::letters_to_token(b"MCG"),
	
	/// TODO.
	MCH = Self::letters_to_token(b"MCH"),
	
	/// TODO.
	MCI = Self::letters_to_token(b"MCI"),
	
	/// TODO.
	MCJ = Self::letters_to_token(b"MCJ"),
	
	/// TODO.
	MCK = Self::letters_to_token(b"MCK"),
	
	/// TODO.
	MCL = Self::letters_to_token(b"MCL"),
	
	/// TODO.
	MCM = Self::letters_to_token(b"MCM"),
	
	/// TODO.
	MCN = Self::letters_to_token(b"MCN"),
	
	/// TODO.
	MCO = Self::letters_to_token(b"MCO"),
	
	/// TODO.
	MCP = Self::letters_to_token(b"MCP"),
	
	/// TODO.
	MCQ = Self::letters_to_token(b"MCQ"),
	
	/// TODO.
	MCR = Self::letters_to_token(b"MCR"),
	
	/// TODO.
	MCS = Self::letters_to_token(b"MCS"),
	
	/// TODO.
	MCT = Self::letters_to_token(b"MCT"),
	
	/// TODO.
	MCU = Self::letters_to_token(b"MCU"),
	
	/// TODO.
	MCV = Self::letters_to_token(b"MCV"),
	
	/// TODO.
	MCW = Self::letters_to_token(b"MCW"),
	
	/// TODO.
	MCX = Self::letters_to_token(b"MCX"),
	
	/// TODO.
	MCY = Self::letters_to_token(b"MCY"),
	
	/// TODO.
	MCZ = Self::letters_to_token(b"MCZ"),
	
	/// TODO.
	MDA = Self::letters_to_token(b"MDA"),
	
	/// TODO.
	MDB = Self::letters_to_token(b"MDB"),
	
	/// TODO.
	MDC = Self::letters_to_token(b"MDC"),
	
	/// TODO.
	MDD = Self::letters_to_token(b"MDD"),
	
	/// TODO.
	MDE = Self::letters_to_token(b"MDE"),
	
	/// TODO.
	MDF = Self::letters_to_token(b"MDF"),
	
	/// TODO.
	MDG = Self::letters_to_token(b"MDG"),
	
	/// TODO.
	MDH = Self::letters_to_token(b"MDH"),
	
	/// TODO.
	MDI = Self::letters_to_token(b"MDI"),
	
	/// TODO.
	MDJ = Self::letters_to_token(b"MDJ"),
	
	/// TODO.
	MDK = Self::letters_to_token(b"MDK"),
	
	/// TODO.
	MDL = Self::letters_to_token(b"MDL"),
	
	/// TODO.
	MDM = Self::letters_to_token(b"MDM"),
	
	/// TODO.
	MDN = Self::letters_to_token(b"MDN"),
	
	/// TODO.
	MDO = Self::letters_to_token(b"MDO"),
	
	/// TODO.
	MDP = Self::letters_to_token(b"MDP"),
	
	/// TODO.
	MDQ = Self::letters_to_token(b"MDQ"),
	
	/// TODO.
	MDR = Self::letters_to_token(b"MDR"),
	
	/// TODO.
	MDS = Self::letters_to_token(b"MDS"),
	
	/// TODO.
	MDT = Self::letters_to_token(b"MDT"),
	
	/// TODO.
	MDU = Self::letters_to_token(b"MDU"),
	
	/// TODO.
	MDV = Self::letters_to_token(b"MDV"),
	
	/// TODO.
	MDW = Self::letters_to_token(b"MDW"),
	
	/// TODO.
	MDX = Self::letters_to_token(b"MDX"),
	
	/// TODO.
	MDY = Self::letters_to_token(b"MDY"),
	
	/// TODO.
	MDZ = Self::letters_to_token(b"MDZ"),
	
	/// TODO.
	MEA = Self::letters_to_token(b"MEA"),
	
	/// TODO.
	MEB = Self::letters_to_token(b"MEB"),
	
	/// TODO.
	MEC = Self::letters_to_token(b"MEC"),
	
	/// TODO.
	MED = Self::letters_to_token(b"MED"),
	
	/// TODO.
	MEE = Self::letters_to_token(b"MEE"),
	
	/// TODO.
	MEF = Self::letters_to_token(b"MEF"),
	
	/// TODO.
	MEG = Self::letters_to_token(b"MEG"),
	
	/// TODO.
	MEH = Self::letters_to_token(b"MEH"),
	
	/// TODO.
	MEI = Self::letters_to_token(b"MEI"),
	
	/// TODO.
	MEJ = Self::letters_to_token(b"MEJ"),
	
	/// TODO.
	MEK = Self::letters_to_token(b"MEK"),
	
	/// TODO.
	MEL = Self::letters_to_token(b"MEL"),
	
	/// TODO.
	MEM = Self::letters_to_token(b"MEM"),
	
	/// TODO.
	MEN = Self::letters_to_token(b"MEN"),
	
	/// TODO.
	MEO = Self::letters_to_token(b"MEO"),
	
	/// TODO.
	MEP = Self::letters_to_token(b"MEP"),
	
	/// TODO.
	MEQ = Self::letters_to_token(b"MEQ"),
	
	/// TODO.
	MER = Self::letters_to_token(b"MER"),
	
	/// TODO.
	MES = Self::letters_to_token(b"MES"),
	
	/// TODO.
	MET = Self::letters_to_token(b"MET"),
	
	/// TODO.
	MEU = Self::letters_to_token(b"MEU"),
	
	/// TODO.
	MEV = Self::letters_to_token(b"MEV"),
	
	/// TODO.
	MEW = Self::letters_to_token(b"MEW"),
	
	/// TODO.
	MEX = Self::letters_to_token(b"MEX"),
	
	/// TODO.
	MEY = Self::letters_to_token(b"MEY"),
	
	/// TODO.
	MEZ = Self::letters_to_token(b"MEZ"),
	
	/// TODO.
	MFA = Self::letters_to_token(b"MFA"),
	
	/// TODO.
	MFB = Self::letters_to_token(b"MFB"),
	
	/// TODO.
	MFC = Self::letters_to_token(b"MFC"),
	
	/// TODO.
	MFD = Self::letters_to_token(b"MFD"),
	
	/// TODO.
	MFE = Self::letters_to_token(b"MFE"),
	
	/// TODO.
	MFF = Self::letters_to_token(b"MFF"),
	
	/// TODO.
	MFG = Self::letters_to_token(b"MFG"),
	
	/// TODO.
	MFH = Self::letters_to_token(b"MFH"),
	
	/// TODO.
	MFI = Self::letters_to_token(b"MFI"),
	
	/// TODO.
	MFJ = Self::letters_to_token(b"MFJ"),
	
	/// TODO.
	MFK = Self::letters_to_token(b"MFK"),
	
	/// TODO.
	MFL = Self::letters_to_token(b"MFL"),
	
	/// TODO.
	MFM = Self::letters_to_token(b"MFM"),
	
	/// TODO.
	MFN = Self::letters_to_token(b"MFN"),
	
	/// TODO.
	MFO = Self::letters_to_token(b"MFO"),
	
	/// TODO.
	MFP = Self::letters_to_token(b"MFP"),
	
	/// TODO.
	MFQ = Self::letters_to_token(b"MFQ"),
	
	/// TODO.
	MFR = Self::letters_to_token(b"MFR"),
	
	/// TODO.
	MFS = Self::letters_to_token(b"MFS"),
	
	/// TODO.
	MFT = Self::letters_to_token(b"MFT"),
	
	/// TODO.
	MFU = Self::letters_to_token(b"MFU"),
	
	/// TODO.
	MFV = Self::letters_to_token(b"MFV"),
	
	/// TODO.
	MFW = Self::letters_to_token(b"MFW"),
	
	/// TODO.
	MFX = Self::letters_to_token(b"MFX"),
	
	/// TODO.
	MFY = Self::letters_to_token(b"MFY"),
	
	/// TODO.
	MFZ = Self::letters_to_token(b"MFZ"),
	
	/// TODO.
	MGA = Self::letters_to_token(b"MGA"),
	
	/// TODO.
	MGB = Self::letters_to_token(b"MGB"),
	
	/// TODO.
	MGC = Self::letters_to_token(b"MGC"),
	
	/// TODO.
	MGD = Self::letters_to_token(b"MGD"),
	
	/// TODO.
	MGE = Self::letters_to_token(b"MGE"),
	
	/// TODO.
	MGF = Self::letters_to_token(b"MGF"),
	
	/// TODO.
	MGG = Self::letters_to_token(b"MGG"),
	
	/// TODO.
	MGH = Self::letters_to_token(b"MGH"),
	
	/// TODO.
	MGI = Self::letters_to_token(b"MGI"),
	
	/// TODO.
	MGJ = Self::letters_to_token(b"MGJ"),
	
	/// TODO.
	MGK = Self::letters_to_token(b"MGK"),
	
	/// TODO.
	MGL = Self::letters_to_token(b"MGL"),
	
	/// TODO.
	MGM = Self::letters_to_token(b"MGM"),
	
	/// TODO.
	MGN = Self::letters_to_token(b"MGN"),
	
	/// TODO.
	MGO = Self::letters_to_token(b"MGO"),
	
	/// TODO.
	MGP = Self::letters_to_token(b"MGP"),
	
	/// TODO.
	MGQ = Self::letters_to_token(b"MGQ"),
	
	/// TODO.
	MGR = Self::letters_to_token(b"MGR"),
	
	/// TODO.
	MGS = Self::letters_to_token(b"MGS"),
	
	/// TODO.
	MGT = Self::letters_to_token(b"MGT"),
	
	/// TODO.
	MGU = Self::letters_to_token(b"MGU"),
	
	/// TODO.
	MGV = Self::letters_to_token(b"MGV"),
	
	/// TODO.
	MGW = Self::letters_to_token(b"MGW"),
	
	/// TODO.
	MGX = Self::letters_to_token(b"MGX"),
	
	/// TODO.
	MGY = Self::letters_to_token(b"MGY"),
	
	/// TODO.
	MGZ = Self::letters_to_token(b"MGZ"),
	
	/// TODO.
	MHA = Self::letters_to_token(b"MHA"),
	
	/// TODO.
	MHB = Self::letters_to_token(b"MHB"),
	
	/// TODO.
	MHC = Self::letters_to_token(b"MHC"),
	
	/// TODO.
	MHD = Self::letters_to_token(b"MHD"),
	
	/// TODO.
	MHE = Self::letters_to_token(b"MHE"),
	
	/// TODO.
	MHF = Self::letters_to_token(b"MHF"),
	
	/// TODO.
	MHG = Self::letters_to_token(b"MHG"),
	
	/// TODO.
	MHH = Self::letters_to_token(b"MHH"),
	
	/// TODO.
	MHI = Self::letters_to_token(b"MHI"),
	
	/// TODO.
	MHJ = Self::letters_to_token(b"MHJ"),
	
	/// TODO.
	MHK = Self::letters_to_token(b"MHK"),
	
	/// TODO.
	MHL = Self::letters_to_token(b"MHL"),
	
	/// TODO.
	MHM = Self::letters_to_token(b"MHM"),
	
	/// TODO.
	MHN = Self::letters_to_token(b"MHN"),
	
	/// TODO.
	MHO = Self::letters_to_token(b"MHO"),
	
	/// TODO.
	MHP = Self::letters_to_token(b"MHP"),
	
	/// TODO.
	MHQ = Self::letters_to_token(b"MHQ"),
	
	/// TODO.
	MHR = Self::letters_to_token(b"MHR"),
	
	/// TODO.
	MHS = Self::letters_to_token(b"MHS"),
	
	/// TODO.
	MHT = Self::letters_to_token(b"MHT"),
	
	/// TODO.
	MHU = Self::letters_to_token(b"MHU"),
	
	/// TODO.
	MHV = Self::letters_to_token(b"MHV"),
	
	/// TODO.
	MHW = Self::letters_to_token(b"MHW"),
	
	/// TODO.
	MHX = Self::letters_to_token(b"MHX"),
	
	/// TODO.
	MHY = Self::letters_to_token(b"MHY"),
	
	/// TODO.
	MHZ = Self::letters_to_token(b"MHZ"),
	
	/// TODO.
	MIA = Self::letters_to_token(b"MIA"),
	
	/// TODO.
	MIB = Self::letters_to_token(b"MIB"),
	
	/// TODO.
	MIC = Self::letters_to_token(b"MIC"),
	
	/// TODO.
	MID = Self::letters_to_token(b"MID"),
	
	/// TODO.
	MIE = Self::letters_to_token(b"MIE"),
	
	/// TODO.
	MIF = Self::letters_to_token(b"MIF"),
	
	/// TODO.
	MIG = Self::letters_to_token(b"MIG"),
	
	/// TODO.
	MIH = Self::letters_to_token(b"MIH"),
	
	/// TODO.
	MII = Self::letters_to_token(b"MII"),
	
	/// TODO.
	MIJ = Self::letters_to_token(b"MIJ"),
	
	/// TODO.
	MIK = Self::letters_to_token(b"MIK"),
	
	/// TODO.
	MIL = Self::letters_to_token(b"MIL"),
	
	/// TODO.
	MIM = Self::letters_to_token(b"MIM"),
	
	/// TODO.
	MIN = Self::letters_to_token(b"MIN"),
	
	/// TODO.
	MIO = Self::letters_to_token(b"MIO"),
	
	/// TODO.
	MIP = Self::letters_to_token(b"MIP"),
	
	/// TODO.
	MIQ = Self::letters_to_token(b"MIQ"),
	
	/// TODO.
	MIR = Self::letters_to_token(b"MIR"),
	
	/// TODO.
	MIS = Self::letters_to_token(b"MIS"),
	
	/// TODO.
	MIT = Self::letters_to_token(b"MIT"),
	
	/// TODO.
	MIU = Self::letters_to_token(b"MIU"),
	
	/// TODO.
	MIV = Self::letters_to_token(b"MIV"),
	
	/// TODO.
	MIW = Self::letters_to_token(b"MIW"),
	
	/// TODO.
	MIX = Self::letters_to_token(b"MIX"),
	
	/// TODO.
	MIY = Self::letters_to_token(b"MIY"),
	
	/// TODO.
	MIZ = Self::letters_to_token(b"MIZ"),
	
	/// TODO.
	MJA = Self::letters_to_token(b"MJA"),
	
	/// TODO.
	MJB = Self::letters_to_token(b"MJB"),
	
	/// TODO.
	MJC = Self::letters_to_token(b"MJC"),
	
	/// TODO.
	MJD = Self::letters_to_token(b"MJD"),
	
	/// TODO.
	MJE = Self::letters_to_token(b"MJE"),
	
	/// TODO.
	MJF = Self::letters_to_token(b"MJF"),
	
	/// TODO.
	MJG = Self::letters_to_token(b"MJG"),
	
	/// TODO.
	MJH = Self::letters_to_token(b"MJH"),
	
	/// TODO.
	MJI = Self::letters_to_token(b"MJI"),
	
	/// TODO.
	MJJ = Self::letters_to_token(b"MJJ"),
	
	/// TODO.
	MJK = Self::letters_to_token(b"MJK"),
	
	/// TODO.
	MJL = Self::letters_to_token(b"MJL"),
	
	/// TODO.
	MJM = Self::letters_to_token(b"MJM"),
	
	/// TODO.
	MJN = Self::letters_to_token(b"MJN"),
	
	/// TODO.
	MJO = Self::letters_to_token(b"MJO"),
	
	/// TODO.
	MJP = Self::letters_to_token(b"MJP"),
	
	/// TODO.
	MJQ = Self::letters_to_token(b"MJQ"),
	
	/// TODO.
	MJR = Self::letters_to_token(b"MJR"),
	
	/// TODO.
	MJS = Self::letters_to_token(b"MJS"),
	
	/// TODO.
	MJT = Self::letters_to_token(b"MJT"),
	
	/// TODO.
	MJU = Self::letters_to_token(b"MJU"),
	
	/// TODO.
	MJV = Self::letters_to_token(b"MJV"),
	
	/// TODO.
	MJW = Self::letters_to_token(b"MJW"),
	
	/// TODO.
	MJX = Self::letters_to_token(b"MJX"),
	
	/// TODO.
	MJY = Self::letters_to_token(b"MJY"),
	
	/// TODO.
	MJZ = Self::letters_to_token(b"MJZ"),
	
	/// TODO.
	MKA = Self::letters_to_token(b"MKA"),
	
	/// TODO.
	MKB = Self::letters_to_token(b"MKB"),
	
	/// TODO.
	MKC = Self::letters_to_token(b"MKC"),
	
	/// TODO.
	MKD = Self::letters_to_token(b"MKD"),
	
	/// TODO.
	MKE = Self::letters_to_token(b"MKE"),
	
	/// TODO.
	MKF = Self::letters_to_token(b"MKF"),
	
	/// TODO.
	MKG = Self::letters_to_token(b"MKG"),
	
	/// TODO.
	MKH = Self::letters_to_token(b"MKH"),
	
	/// TODO.
	MKI = Self::letters_to_token(b"MKI"),
	
	/// TODO.
	MKJ = Self::letters_to_token(b"MKJ"),
	
	/// TODO.
	MKK = Self::letters_to_token(b"MKK"),
	
	/// TODO.
	MKL = Self::letters_to_token(b"MKL"),
	
	/// TODO.
	MKM = Self::letters_to_token(b"MKM"),
	
	/// TODO.
	MKN = Self::letters_to_token(b"MKN"),
	
	/// TODO.
	MKO = Self::letters_to_token(b"MKO"),
	
	/// TODO.
	MKP = Self::letters_to_token(b"MKP"),
	
	/// TODO.
	MKQ = Self::letters_to_token(b"MKQ"),
	
	/// TODO.
	MKR = Self::letters_to_token(b"MKR"),
	
	/// TODO.
	MKS = Self::letters_to_token(b"MKS"),
	
	/// TODO.
	MKT = Self::letters_to_token(b"MKT"),
	
	/// TODO.
	MKU = Self::letters_to_token(b"MKU"),
	
	/// TODO.
	MKV = Self::letters_to_token(b"MKV"),
	
	/// TODO.
	MKW = Self::letters_to_token(b"MKW"),
	
	/// TODO.
	MKX = Self::letters_to_token(b"MKX"),
	
	/// TODO.
	MKY = Self::letters_to_token(b"MKY"),
	
	/// TODO.
	MKZ = Self::letters_to_token(b"MKZ"),
	
	/// TODO.
	MLA = Self::letters_to_token(b"MLA"),
	
	/// TODO.
	MLB = Self::letters_to_token(b"MLB"),
	
	/// TODO.
	MLC = Self::letters_to_token(b"MLC"),
	
	/// TODO.
	MLD = Self::letters_to_token(b"MLD"),
	
	/// TODO.
	MLE = Self::letters_to_token(b"MLE"),
	
	/// TODO.
	MLF = Self::letters_to_token(b"MLF"),
	
	/// TODO.
	MLG = Self::letters_to_token(b"MLG"),
	
	/// TODO.
	MLH = Self::letters_to_token(b"MLH"),
	
	/// TODO.
	MLI = Self::letters_to_token(b"MLI"),
	
	/// TODO.
	MLJ = Self::letters_to_token(b"MLJ"),
	
	/// TODO.
	MLK = Self::letters_to_token(b"MLK"),
	
	/// TODO.
	MLL = Self::letters_to_token(b"MLL"),
	
	/// TODO.
	MLM = Self::letters_to_token(b"MLM"),
	
	/// TODO.
	MLN = Self::letters_to_token(b"MLN"),
	
	/// TODO.
	MLO = Self::letters_to_token(b"MLO"),
	
	/// TODO.
	MLP = Self::letters_to_token(b"MLP"),
	
	/// TODO.
	MLQ = Self::letters_to_token(b"MLQ"),
	
	/// TODO.
	MLR = Self::letters_to_token(b"MLR"),
	
	/// TODO.
	MLS = Self::letters_to_token(b"MLS"),
	
	/// TODO.
	MLT = Self::letters_to_token(b"MLT"),
	
	/// TODO.
	MLU = Self::letters_to_token(b"MLU"),
	
	/// TODO.
	MLV = Self::letters_to_token(b"MLV"),
	
	/// TODO.
	MLW = Self::letters_to_token(b"MLW"),
	
	/// TODO.
	MLX = Self::letters_to_token(b"MLX"),
	
	/// TODO.
	MLY = Self::letters_to_token(b"MLY"),
	
	/// TODO.
	MLZ = Self::letters_to_token(b"MLZ"),
	
	/// TODO.
	MMA = Self::letters_to_token(b"MMA"),
	
	/// TODO.
	MMB = Self::letters_to_token(b"MMB"),
	
	/// TODO.
	MMC = Self::letters_to_token(b"MMC"),
	
	/// TODO.
	MMD = Self::letters_to_token(b"MMD"),
	
	/// TODO.
	MME = Self::letters_to_token(b"MME"),
	
	/// TODO.
	MMF = Self::letters_to_token(b"MMF"),
	
	/// TODO.
	MMG = Self::letters_to_token(b"MMG"),
	
	/// TODO.
	MMH = Self::letters_to_token(b"MMH"),
	
	/// TODO.
	MMI = Self::letters_to_token(b"MMI"),
	
	/// TODO.
	MMJ = Self::letters_to_token(b"MMJ"),
	
	/// TODO.
	MMK = Self::letters_to_token(b"MMK"),
	
	/// TODO.
	MML = Self::letters_to_token(b"MML"),
	
	/// TODO.
	MMM = Self::letters_to_token(b"MMM"),
	
	/// TODO.
	MMN = Self::letters_to_token(b"MMN"),
	
	/// TODO.
	MMO = Self::letters_to_token(b"MMO"),
	
	/// TODO.
	MMP = Self::letters_to_token(b"MMP"),
	
	/// TODO.
	MMQ = Self::letters_to_token(b"MMQ"),
	
	/// TODO.
	MMR = Self::letters_to_token(b"MMR"),
	
	/// TODO.
	MMS = Self::letters_to_token(b"MMS"),
	
	/// TODO.
	MMT = Self::letters_to_token(b"MMT"),
	
	/// TODO.
	MMU = Self::letters_to_token(b"MMU"),
	
	/// TODO.
	MMV = Self::letters_to_token(b"MMV"),
	
	/// TODO.
	MMW = Self::letters_to_token(b"MMW"),
	
	/// TODO.
	MMX = Self::letters_to_token(b"MMX"),
	
	/// TODO.
	MMY = Self::letters_to_token(b"MMY"),
	
	/// TODO.
	MMZ = Self::letters_to_token(b"MMZ"),
	
	/// TODO.
	MNA = Self::letters_to_token(b"MNA"),
	
	/// TODO.
	MNB = Self::letters_to_token(b"MNB"),
	
	/// TODO.
	MNC = Self::letters_to_token(b"MNC"),
	
	/// TODO.
	MND = Self::letters_to_token(b"MND"),
	
	/// TODO.
	MNE = Self::letters_to_token(b"MNE"),
	
	/// TODO.
	MNF = Self::letters_to_token(b"MNF"),
	
	/// TODO.
	MNG = Self::letters_to_token(b"MNG"),
	
	/// TODO.
	MNH = Self::letters_to_token(b"MNH"),
	
	/// TODO.
	MNI = Self::letters_to_token(b"MNI"),
	
	/// TODO.
	MNJ = Self::letters_to_token(b"MNJ"),
	
	/// TODO.
	MNK = Self::letters_to_token(b"MNK"),
	
	/// TODO.
	MNL = Self::letters_to_token(b"MNL"),
	
	/// TODO.
	MNM = Self::letters_to_token(b"MNM"),
	
	/// TODO.
	MNN = Self::letters_to_token(b"MNN"),
	
	/// TODO.
	MNO = Self::letters_to_token(b"MNO"),
	
	/// TODO.
	MNP = Self::letters_to_token(b"MNP"),
	
	/// TODO.
	MNQ = Self::letters_to_token(b"MNQ"),
	
	/// TODO.
	MNR = Self::letters_to_token(b"MNR"),
	
	/// TODO.
	MNS = Self::letters_to_token(b"MNS"),
	
	/// TODO.
	MNT = Self::letters_to_token(b"MNT"),
	
	/// TODO.
	MNU = Self::letters_to_token(b"MNU"),
	
	/// TODO.
	MNV = Self::letters_to_token(b"MNV"),
	
	/// TODO.
	MNW = Self::letters_to_token(b"MNW"),
	
	/// TODO.
	MNX = Self::letters_to_token(b"MNX"),
	
	/// TODO.
	MNY = Self::letters_to_token(b"MNY"),
	
	/// TODO.
	MNZ = Self::letters_to_token(b"MNZ"),
	
	/// TODO.
	MOA = Self::letters_to_token(b"MOA"),
	
	/// TODO.
	MOB = Self::letters_to_token(b"MOB"),
	
	/// TODO.
	MOC = Self::letters_to_token(b"MOC"),
	
	/// TODO.
	MOD = Self::letters_to_token(b"MOD"),
	
	/// TODO.
	MOE = Self::letters_to_token(b"MOE"),
	
	/// TODO.
	MOF = Self::letters_to_token(b"MOF"),
	
	/// TODO.
	MOG = Self::letters_to_token(b"MOG"),
	
	/// TODO.
	MOH = Self::letters_to_token(b"MOH"),
	
	/// TODO.
	MOI = Self::letters_to_token(b"MOI"),
	
	/// TODO.
	MOJ = Self::letters_to_token(b"MOJ"),
	
	/// TODO.
	MOK = Self::letters_to_token(b"MOK"),
	
	/// TODO.
	MOL = Self::letters_to_token(b"MOL"),
	
	/// TODO.
	MOM = Self::letters_to_token(b"MOM"),
	
	/// TODO.
	MON = Self::letters_to_token(b"MON"),
	
	/// TODO.
	MOO = Self::letters_to_token(b"MOO"),
	
	/// TODO.
	MOP = Self::letters_to_token(b"MOP"),
	
	/// TODO.
	MOQ = Self::letters_to_token(b"MOQ"),
	
	/// TODO.
	MOR = Self::letters_to_token(b"MOR"),
	
	/// TODO.
	MOS = Self::letters_to_token(b"MOS"),
	
	/// TODO.
	MOT = Self::letters_to_token(b"MOT"),
	
	/// TODO.
	MOU = Self::letters_to_token(b"MOU"),
	
	/// TODO.
	MOV = Self::letters_to_token(b"MOV"),
	
	/// TODO.
	MOW = Self::letters_to_token(b"MOW"),
	
	/// TODO.
	MOX = Self::letters_to_token(b"MOX"),
	
	/// TODO.
	MOY = Self::letters_to_token(b"MOY"),
	
	/// TODO.
	MOZ = Self::letters_to_token(b"MOZ"),
	
	/// TODO.
	MPA = Self::letters_to_token(b"MPA"),
	
	/// TODO.
	MPB = Self::letters_to_token(b"MPB"),
	
	/// TODO.
	MPC = Self::letters_to_token(b"MPC"),
	
	/// TODO.
	MPD = Self::letters_to_token(b"MPD"),
	
	/// TODO.
	MPE = Self::letters_to_token(b"MPE"),
	
	/// TODO.
	MPF = Self::letters_to_token(b"MPF"),
	
	/// TODO.
	MPG = Self::letters_to_token(b"MPG"),
	
	/// TODO.
	MPH = Self::letters_to_token(b"MPH"),
	
	/// TODO.
	MPI = Self::letters_to_token(b"MPI"),
	
	/// TODO.
	MPJ = Self::letters_to_token(b"MPJ"),
	
	/// TODO.
	MPK = Self::letters_to_token(b"MPK"),
	
	/// TODO.
	MPL = Self::letters_to_token(b"MPL"),
	
	/// TODO.
	MPM = Self::letters_to_token(b"MPM"),
	
	/// TODO.
	MPN = Self::letters_to_token(b"MPN"),
	
	/// TODO.
	MPO = Self::letters_to_token(b"MPO"),
	
	/// TODO.
	MPP = Self::letters_to_token(b"MPP"),
	
	/// TODO.
	MPQ = Self::letters_to_token(b"MPQ"),
	
	/// TODO.
	MPR = Self::letters_to_token(b"MPR"),
	
	/// TODO.
	MPS = Self::letters_to_token(b"MPS"),
	
	/// TODO.
	MPT = Self::letters_to_token(b"MPT"),
	
	/// TODO.
	MPU = Self::letters_to_token(b"MPU"),
	
	/// TODO.
	MPV = Self::letters_to_token(b"MPV"),
	
	/// TODO.
	MPW = Self::letters_to_token(b"MPW"),
	
	/// TODO.
	MPX = Self::letters_to_token(b"MPX"),
	
	/// TODO.
	MPY = Self::letters_to_token(b"MPY"),
	
	/// TODO.
	MPZ = Self::letters_to_token(b"MPZ"),
	
	/// TODO.
	MQA = Self::letters_to_token(b"MQA"),
	
	/// TODO.
	MQB = Self::letters_to_token(b"MQB"),
	
	/// TODO.
	MQC = Self::letters_to_token(b"MQC"),
	
	/// TODO.
	MQD = Self::letters_to_token(b"MQD"),
	
	/// TODO.
	MQE = Self::letters_to_token(b"MQE"),
	
	/// TODO.
	MQF = Self::letters_to_token(b"MQF"),
	
	/// TODO.
	MQG = Self::letters_to_token(b"MQG"),
	
	/// TODO.
	MQH = Self::letters_to_token(b"MQH"),
	
	/// TODO.
	MQI = Self::letters_to_token(b"MQI"),
	
	/// TODO.
	MQJ = Self::letters_to_token(b"MQJ"),
	
	/// TODO.
	MQK = Self::letters_to_token(b"MQK"),
	
	/// TODO.
	MQL = Self::letters_to_token(b"MQL"),
	
	/// TODO.
	MQM = Self::letters_to_token(b"MQM"),
	
	/// TODO.
	MQN = Self::letters_to_token(b"MQN"),
	
	/// TODO.
	MQO = Self::letters_to_token(b"MQO"),
	
	/// TODO.
	MQP = Self::letters_to_token(b"MQP"),
	
	/// TODO.
	MQQ = Self::letters_to_token(b"MQQ"),
	
	/// TODO.
	MQR = Self::letters_to_token(b"MQR"),
	
	/// TODO.
	MQS = Self::letters_to_token(b"MQS"),
	
	/// TODO.
	MQT = Self::letters_to_token(b"MQT"),
	
	/// TODO.
	MQU = Self::letters_to_token(b"MQU"),
	
	/// TODO.
	MQV = Self::letters_to_token(b"MQV"),
	
	/// TODO.
	MQW = Self::letters_to_token(b"MQW"),
	
	/// TODO.
	MQX = Self::letters_to_token(b"MQX"),
	
	/// TODO.
	MQY = Self::letters_to_token(b"MQY"),
	
	/// TODO.
	MQZ = Self::letters_to_token(b"MQZ"),
	
	/// TODO.
	MRA = Self::letters_to_token(b"MRA"),
	
	/// TODO.
	MRB = Self::letters_to_token(b"MRB"),
	
	/// TODO.
	MRC = Self::letters_to_token(b"MRC"),
	
	/// TODO.
	MRD = Self::letters_to_token(b"MRD"),
	
	/// TODO.
	MRE = Self::letters_to_token(b"MRE"),
	
	/// TODO.
	MRF = Self::letters_to_token(b"MRF"),
	
	/// TODO.
	MRG = Self::letters_to_token(b"MRG"),
	
	/// TODO.
	MRH = Self::letters_to_token(b"MRH"),
	
	/// TODO.
	MRI = Self::letters_to_token(b"MRI"),
	
	/// TODO.
	MRJ = Self::letters_to_token(b"MRJ"),
	
	/// TODO.
	MRK = Self::letters_to_token(b"MRK"),
	
	/// TODO.
	MRL = Self::letters_to_token(b"MRL"),
	
	/// TODO.
	MRM = Self::letters_to_token(b"MRM"),
	
	/// TODO.
	MRN = Self::letters_to_token(b"MRN"),
	
	/// TODO.
	MRO = Self::letters_to_token(b"MRO"),
	
	/// TODO.
	MRP = Self::letters_to_token(b"MRP"),
	
	/// TODO.
	MRQ = Self::letters_to_token(b"MRQ"),
	
	/// TODO.
	MRR = Self::letters_to_token(b"MRR"),
	
	/// TODO.
	MRS = Self::letters_to_token(b"MRS"),
	
	/// TODO.
	MRT = Self::letters_to_token(b"MRT"),
	
	/// TODO.
	MRU = Self::letters_to_token(b"MRU"),
	
	/// TODO.
	MRV = Self::letters_to_token(b"MRV"),
	
	/// TODO.
	MRW = Self::letters_to_token(b"MRW"),
	
	/// TODO.
	MRX = Self::letters_to_token(b"MRX"),
	
	/// TODO.
	MRY = Self::letters_to_token(b"MRY"),
	
	/// TODO.
	MRZ = Self::letters_to_token(b"MRZ"),
	
	/// TODO.
	MSA = Self::letters_to_token(b"MSA"),
	
	/// TODO.
	MSB = Self::letters_to_token(b"MSB"),
	
	/// TODO.
	MSC = Self::letters_to_token(b"MSC"),
	
	/// TODO.
	MSD = Self::letters_to_token(b"MSD"),
	
	/// TODO.
	MSE = Self::letters_to_token(b"MSE"),
	
	/// TODO.
	MSF = Self::letters_to_token(b"MSF"),
	
	/// TODO.
	MSG = Self::letters_to_token(b"MSG"),
	
	/// TODO.
	MSH = Self::letters_to_token(b"MSH"),
	
	/// TODO.
	MSI = Self::letters_to_token(b"MSI"),
	
	/// TODO.
	MSJ = Self::letters_to_token(b"MSJ"),
	
	/// TODO.
	MSK = Self::letters_to_token(b"MSK"),
	
	/// TODO.
	MSL = Self::letters_to_token(b"MSL"),
	
	/// TODO.
	MSM = Self::letters_to_token(b"MSM"),
	
	/// TODO.
	MSN = Self::letters_to_token(b"MSN"),
	
	/// TODO.
	MSO = Self::letters_to_token(b"MSO"),
	
	/// TODO.
	MSP = Self::letters_to_token(b"MSP"),
	
	/// TODO.
	MSQ = Self::letters_to_token(b"MSQ"),
	
	/// TODO.
	MSR = Self::letters_to_token(b"MSR"),
	
	/// TODO.
	MSS = Self::letters_to_token(b"MSS"),
	
	/// TODO.
	MST = Self::letters_to_token(b"MST"),
	
	/// TODO.
	MSU = Self::letters_to_token(b"MSU"),
	
	/// TODO.
	MSV = Self::letters_to_token(b"MSV"),
	
	/// TODO.
	MSW = Self::letters_to_token(b"MSW"),
	
	/// TODO.
	MSX = Self::letters_to_token(b"MSX"),
	
	/// TODO.
	MSY = Self::letters_to_token(b"MSY"),
	
	/// TODO.
	MSZ = Self::letters_to_token(b"MSZ"),
	
	/// TODO.
	MTA = Self::letters_to_token(b"MTA"),
	
	/// TODO.
	MTB = Self::letters_to_token(b"MTB"),
	
	/// TODO.
	MTC = Self::letters_to_token(b"MTC"),
	
	/// TODO.
	MTD = Self::letters_to_token(b"MTD"),
	
	/// TODO.
	MTE = Self::letters_to_token(b"MTE"),
	
	/// TODO.
	MTF = Self::letters_to_token(b"MTF"),
	
	/// TODO.
	MTG = Self::letters_to_token(b"MTG"),
	
	/// TODO.
	MTH = Self::letters_to_token(b"MTH"),
	
	/// TODO.
	MTI = Self::letters_to_token(b"MTI"),
	
	/// TODO.
	MTJ = Self::letters_to_token(b"MTJ"),
	
	/// TODO.
	MTK = Self::letters_to_token(b"MTK"),
	
	/// TODO.
	MTL = Self::letters_to_token(b"MTL"),
	
	/// TODO.
	MTM = Self::letters_to_token(b"MTM"),
	
	/// TODO.
	MTN = Self::letters_to_token(b"MTN"),
	
	/// TODO.
	MTO = Self::letters_to_token(b"MTO"),
	
	/// TODO.
	MTP = Self::letters_to_token(b"MTP"),
	
	/// TODO.
	MTQ = Self::letters_to_token(b"MTQ"),
	
	/// TODO.
	MTR = Self::letters_to_token(b"MTR"),
	
	/// TODO.
	MTS = Self::letters_to_token(b"MTS"),
	
	/// TODO.
	MTT = Self::letters_to_token(b"MTT"),
	
	/// TODO.
	MTU = Self::letters_to_token(b"MTU"),
	
	/// TODO.
	MTV = Self::letters_to_token(b"MTV"),
	
	/// TODO.
	MTW = Self::letters_to_token(b"MTW"),
	
	/// TODO.
	MTX = Self::letters_to_token(b"MTX"),
	
	/// TODO.
	MTY = Self::letters_to_token(b"MTY"),
	
	/// TODO.
	MTZ = Self::letters_to_token(b"MTZ"),
	
	/// TODO.
	MUA = Self::letters_to_token(b"MUA"),
	
	/// TODO.
	MUB = Self::letters_to_token(b"MUB"),
	
	/// TODO.
	MUC = Self::letters_to_token(b"MUC"),
	
	/// TODO.
	MUD = Self::letters_to_token(b"MUD"),
	
	/// TODO.
	MUE = Self::letters_to_token(b"MUE"),
	
	/// TODO.
	MUF = Self::letters_to_token(b"MUF"),
	
	/// TODO.
	MUG = Self::letters_to_token(b"MUG"),
	
	/// TODO.
	MUH = Self::letters_to_token(b"MUH"),
	
	/// TODO.
	MUI = Self::letters_to_token(b"MUI"),
	
	/// TODO.
	MUJ = Self::letters_to_token(b"MUJ"),
	
	/// TODO.
	MUK = Self::letters_to_token(b"MUK"),
	
	/// TODO.
	MUL = Self::letters_to_token(b"MUL"),
	
	/// TODO.
	MUM = Self::letters_to_token(b"MUM"),
	
	/// TODO.
	MUN = Self::letters_to_token(b"MUN"),
	
	/// TODO.
	MUO = Self::letters_to_token(b"MUO"),
	
	/// TODO.
	MUP = Self::letters_to_token(b"MUP"),
	
	/// TODO.
	MUQ = Self::letters_to_token(b"MUQ"),
	
	/// TODO.
	MUR = Self::letters_to_token(b"MUR"),
	
	/// TODO.
	MUS = Self::letters_to_token(b"MUS"),
	
	/// TODO.
	MUT = Self::letters_to_token(b"MUT"),
	
	/// TODO.
	MUU = Self::letters_to_token(b"MUU"),
	
	/// TODO.
	MUV = Self::letters_to_token(b"MUV"),
	
	/// TODO.
	MUW = Self::letters_to_token(b"MUW"),
	
	/// TODO.
	MUX = Self::letters_to_token(b"MUX"),
	
	/// TODO.
	MUY = Self::letters_to_token(b"MUY"),
	
	/// TODO.
	MUZ = Self::letters_to_token(b"MUZ"),
	
	/// TODO.
	MVA = Self::letters_to_token(b"MVA"),
	
	/// TODO.
	MVB = Self::letters_to_token(b"MVB"),
	
	/// TODO.
	MVC = Self::letters_to_token(b"MVC"),
	
	/// TODO.
	MVD = Self::letters_to_token(b"MVD"),
	
	/// TODO.
	MVE = Self::letters_to_token(b"MVE"),
	
	/// TODO.
	MVF = Self::letters_to_token(b"MVF"),
	
	/// TODO.
	MVG = Self::letters_to_token(b"MVG"),
	
	/// TODO.
	MVH = Self::letters_to_token(b"MVH"),
	
	/// TODO.
	MVI = Self::letters_to_token(b"MVI"),
	
	/// TODO.
	MVJ = Self::letters_to_token(b"MVJ"),
	
	/// TODO.
	MVK = Self::letters_to_token(b"MVK"),
	
	/// TODO.
	MVL = Self::letters_to_token(b"MVL"),
	
	/// TODO.
	MVM = Self::letters_to_token(b"MVM"),
	
	/// TODO.
	MVN = Self::letters_to_token(b"MVN"),
	
	/// TODO.
	MVO = Self::letters_to_token(b"MVO"),
	
	/// TODO.
	MVP = Self::letters_to_token(b"MVP"),
	
	/// TODO.
	MVQ = Self::letters_to_token(b"MVQ"),
	
	/// TODO.
	MVR = Self::letters_to_token(b"MVR"),
	
	/// TODO.
	MVS = Self::letters_to_token(b"MVS"),
	
	/// TODO.
	MVT = Self::letters_to_token(b"MVT"),
	
	/// TODO.
	MVU = Self::letters_to_token(b"MVU"),
	
	/// TODO.
	MVV = Self::letters_to_token(b"MVV"),
	
	/// TODO.
	MVW = Self::letters_to_token(b"MVW"),
	
	/// TODO.
	MVX = Self::letters_to_token(b"MVX"),
	
	/// TODO.
	MVY = Self::letters_to_token(b"MVY"),
	
	/// TODO.
	MVZ = Self::letters_to_token(b"MVZ"),
	
	/// TODO.
	MWA = Self::letters_to_token(b"MWA"),
	
	/// TODO.
	MWB = Self::letters_to_token(b"MWB"),
	
	/// TODO.
	MWC = Self::letters_to_token(b"MWC"),
	
	/// TODO.
	MWD = Self::letters_to_token(b"MWD"),
	
	/// TODO.
	MWE = Self::letters_to_token(b"MWE"),
	
	/// TODO.
	MWF = Self::letters_to_token(b"MWF"),
	
	/// TODO.
	MWG = Self::letters_to_token(b"MWG"),
	
	/// TODO.
	MWH = Self::letters_to_token(b"MWH"),
	
	/// TODO.
	MWI = Self::letters_to_token(b"MWI"),
	
	/// TODO.
	MWJ = Self::letters_to_token(b"MWJ"),
	
	/// TODO.
	MWK = Self::letters_to_token(b"MWK"),
	
	/// TODO.
	MWL = Self::letters_to_token(b"MWL"),
	
	/// TODO.
	MWM = Self::letters_to_token(b"MWM"),
	
	/// TODO.
	MWN = Self::letters_to_token(b"MWN"),
	
	/// TODO.
	MWO = Self::letters_to_token(b"MWO"),
	
	/// TODO.
	MWP = Self::letters_to_token(b"MWP"),
	
	/// TODO.
	MWQ = Self::letters_to_token(b"MWQ"),
	
	/// TODO.
	MWR = Self::letters_to_token(b"MWR"),
	
	/// TODO.
	MWS = Self::letters_to_token(b"MWS"),
	
	/// TODO.
	MWT = Self::letters_to_token(b"MWT"),
	
	/// TODO.
	MWU = Self::letters_to_token(b"MWU"),
	
	/// TODO.
	MWV = Self::letters_to_token(b"MWV"),
	
	/// TODO.
	MWW = Self::letters_to_token(b"MWW"),
	
	/// TODO.
	MWX = Self::letters_to_token(b"MWX"),
	
	/// TODO.
	MWY = Self::letters_to_token(b"MWY"),
	
	/// TODO.
	MWZ = Self::letters_to_token(b"MWZ"),
	
	/// TODO.
	MXA = Self::letters_to_token(b"MXA"),
	
	/// TODO.
	MXB = Self::letters_to_token(b"MXB"),
	
	/// TODO.
	MXC = Self::letters_to_token(b"MXC"),
	
	/// TODO.
	MXD = Self::letters_to_token(b"MXD"),
	
	/// TODO.
	MXE = Self::letters_to_token(b"MXE"),
	
	/// TODO.
	MXF = Self::letters_to_token(b"MXF"),
	
	/// TODO.
	MXG = Self::letters_to_token(b"MXG"),
	
	/// TODO.
	MXH = Self::letters_to_token(b"MXH"),
	
	/// TODO.
	MXI = Self::letters_to_token(b"MXI"),
	
	/// TODO.
	MXJ = Self::letters_to_token(b"MXJ"),
	
	/// TODO.
	MXK = Self::letters_to_token(b"MXK"),
	
	/// TODO.
	MXL = Self::letters_to_token(b"MXL"),
	
	/// TODO.
	MXM = Self::letters_to_token(b"MXM"),
	
	/// TODO.
	MXN = Self::letters_to_token(b"MXN"),
	
	/// TODO.
	MXO = Self::letters_to_token(b"MXO"),
	
	/// TODO.
	MXP = Self::letters_to_token(b"MXP"),
	
	/// TODO.
	MXQ = Self::letters_to_token(b"MXQ"),
	
	/// TODO.
	MXR = Self::letters_to_token(b"MXR"),
	
	/// TODO.
	MXS = Self::letters_to_token(b"MXS"),
	
	/// TODO.
	MXT = Self::letters_to_token(b"MXT"),
	
	/// TODO.
	MXU = Self::letters_to_token(b"MXU"),
	
	/// TODO.
	MXV = Self::letters_to_token(b"MXV"),
	
	/// TODO.
	MXW = Self::letters_to_token(b"MXW"),
	
	/// TODO.
	MXX = Self::letters_to_token(b"MXX"),
	
	/// TODO.
	MXY = Self::letters_to_token(b"MXY"),
	
	/// TODO.
	MXZ = Self::letters_to_token(b"MXZ"),
	
	/// TODO.
	MYA = Self::letters_to_token(b"MYA"),
	
	/// TODO.
	MYB = Self::letters_to_token(b"MYB"),
	
	/// TODO.
	MYC = Self::letters_to_token(b"MYC"),
	
	/// TODO.
	MYD = Self::letters_to_token(b"MYD"),
	
	/// TODO.
	MYE = Self::letters_to_token(b"MYE"),
	
	/// TODO.
	MYF = Self::letters_to_token(b"MYF"),
	
	/// TODO.
	MYG = Self::letters_to_token(b"MYG"),
	
	/// TODO.
	MYH = Self::letters_to_token(b"MYH"),
	
	/// TODO.
	MYI = Self::letters_to_token(b"MYI"),
	
	/// TODO.
	MYJ = Self::letters_to_token(b"MYJ"),
	
	/// TODO.
	MYK = Self::letters_to_token(b"MYK"),
	
	/// TODO.
	MYL = Self::letters_to_token(b"MYL"),
	
	/// TODO.
	MYM = Self::letters_to_token(b"MYM"),
	
	/// TODO.
	MYN = Self::letters_to_token(b"MYN"),
	
	/// TODO.
	MYO = Self::letters_to_token(b"MYO"),
	
	/// TODO.
	MYP = Self::letters_to_token(b"MYP"),
	
	/// TODO.
	MYQ = Self::letters_to_token(b"MYQ"),
	
	/// TODO.
	MYR = Self::letters_to_token(b"MYR"),
	
	/// TODO.
	MYS = Self::letters_to_token(b"MYS"),
	
	/// TODO.
	MYT = Self::letters_to_token(b"MYT"),
	
	/// TODO.
	MYU = Self::letters_to_token(b"MYU"),
	
	/// TODO.
	MYV = Self::letters_to_token(b"MYV"),
	
	/// TODO.
	MYW = Self::letters_to_token(b"MYW"),
	
	/// TODO.
	MYX = Self::letters_to_token(b"MYX"),
	
	/// TODO.
	MYY = Self::letters_to_token(b"MYY"),
	
	/// TODO.
	MYZ = Self::letters_to_token(b"MYZ"),
	
	/// TODO.
	MZA = Self::letters_to_token(b"MZA"),
	
	/// TODO.
	MZB = Self::letters_to_token(b"MZB"),
	
	/// TODO.
	MZC = Self::letters_to_token(b"MZC"),
	
	/// TODO.
	MZD = Self::letters_to_token(b"MZD"),
	
	/// TODO.
	MZE = Self::letters_to_token(b"MZE"),
	
	/// TODO.
	MZF = Self::letters_to_token(b"MZF"),
	
	/// TODO.
	MZG = Self::letters_to_token(b"MZG"),
	
	/// TODO.
	MZH = Self::letters_to_token(b"MZH"),
	
	/// TODO.
	MZI = Self::letters_to_token(b"MZI"),
	
	/// TODO.
	MZJ = Self::letters_to_token(b"MZJ"),
	
	/// TODO.
	MZK = Self::letters_to_token(b"MZK"),
	
	/// TODO.
	MZL = Self::letters_to_token(b"MZL"),
	
	/// TODO.
	MZM = Self::letters_to_token(b"MZM"),
	
	/// TODO.
	MZN = Self::letters_to_token(b"MZN"),
	
	/// TODO.
	MZO = Self::letters_to_token(b"MZO"),
	
	/// TODO.
	MZP = Self::letters_to_token(b"MZP"),
	
	/// TODO.
	MZQ = Self::letters_to_token(b"MZQ"),
	
	/// TODO.
	MZR = Self::letters_to_token(b"MZR"),
	
	/// TODO.
	MZS = Self::letters_to_token(b"MZS"),
	
	/// TODO.
	MZT = Self::letters_to_token(b"MZT"),
	
	/// TODO.
	MZU = Self::letters_to_token(b"MZU"),
	
	/// TODO.
	MZV = Self::letters_to_token(b"MZV"),
	
	/// TODO.
	MZW = Self::letters_to_token(b"MZW"),
	
	/// TODO.
	MZX = Self::letters_to_token(b"MZX"),
	
	/// TODO.
	MZY = Self::letters_to_token(b"MZY"),
	
	/// TODO.
	MZZ = Self::letters_to_token(b"MZZ"),
	
	/// TODO.
	NAA = Self::letters_to_token(b"NAA"),
	
	/// TODO.
	NAB = Self::letters_to_token(b"NAB"),
	
	/// TODO.
	NAC = Self::letters_to_token(b"NAC"),
	
	/// TODO.
	NAD = Self::letters_to_token(b"NAD"),
	
	/// TODO.
	NAE = Self::letters_to_token(b"NAE"),
	
	/// TODO.
	NAF = Self::letters_to_token(b"NAF"),
	
	/// TODO.
	NAG = Self::letters_to_token(b"NAG"),
	
	/// TODO.
	NAH = Self::letters_to_token(b"NAH"),
	
	/// TODO.
	NAI = Self::letters_to_token(b"NAI"),
	
	/// TODO.
	NAJ = Self::letters_to_token(b"NAJ"),
	
	/// TODO.
	NAK = Self::letters_to_token(b"NAK"),
	
	/// TODO.
	NAL = Self::letters_to_token(b"NAL"),
	
	/// TODO.
	NAM = Self::letters_to_token(b"NAM"),
	
	/// TODO.
	NAN = Self::letters_to_token(b"NAN"),
	
	/// TODO.
	NAO = Self::letters_to_token(b"NAO"),
	
	/// TODO.
	NAP = Self::letters_to_token(b"NAP"),
	
	/// TODO.
	NAQ = Self::letters_to_token(b"NAQ"),
	
	/// TODO.
	NAR = Self::letters_to_token(b"NAR"),
	
	/// TODO.
	NAS = Self::letters_to_token(b"NAS"),
	
	/// TODO.
	NAT = Self::letters_to_token(b"NAT"),
	
	/// TODO.
	NAU = Self::letters_to_token(b"NAU"),
	
	/// TODO.
	NAV = Self::letters_to_token(b"NAV"),
	
	/// TODO.
	NAW = Self::letters_to_token(b"NAW"),
	
	/// TODO.
	NAX = Self::letters_to_token(b"NAX"),
	
	/// TODO.
	NAY = Self::letters_to_token(b"NAY"),
	
	/// TODO.
	NAZ = Self::letters_to_token(b"NAZ"),
	
	/// TODO.
	NBA = Self::letters_to_token(b"NBA"),
	
	/// TODO.
	NBB = Self::letters_to_token(b"NBB"),
	
	/// TODO.
	NBC = Self::letters_to_token(b"NBC"),
	
	/// TODO.
	NBD = Self::letters_to_token(b"NBD"),
	
	/// TODO.
	NBE = Self::letters_to_token(b"NBE"),
	
	/// TODO.
	NBF = Self::letters_to_token(b"NBF"),
	
	/// TODO.
	NBG = Self::letters_to_token(b"NBG"),
	
	/// TODO.
	NBH = Self::letters_to_token(b"NBH"),
	
	/// TODO.
	NBI = Self::letters_to_token(b"NBI"),
	
	/// TODO.
	NBJ = Self::letters_to_token(b"NBJ"),
	
	/// TODO.
	NBK = Self::letters_to_token(b"NBK"),
	
	/// TODO.
	NBL = Self::letters_to_token(b"NBL"),
	
	/// TODO.
	NBM = Self::letters_to_token(b"NBM"),
	
	/// TODO.
	NBN = Self::letters_to_token(b"NBN"),
	
	/// TODO.
	NBO = Self::letters_to_token(b"NBO"),
	
	/// TODO.
	NBP = Self::letters_to_token(b"NBP"),
	
	/// TODO.
	NBQ = Self::letters_to_token(b"NBQ"),
	
	/// TODO.
	NBR = Self::letters_to_token(b"NBR"),
	
	/// TODO.
	NBS = Self::letters_to_token(b"NBS"),
	
	/// TODO.
	NBT = Self::letters_to_token(b"NBT"),
	
	/// TODO.
	NBU = Self::letters_to_token(b"NBU"),
	
	/// TODO.
	NBV = Self::letters_to_token(b"NBV"),
	
	/// TODO.
	NBW = Self::letters_to_token(b"NBW"),
	
	/// TODO.
	NBX = Self::letters_to_token(b"NBX"),
	
	/// TODO.
	NBY = Self::letters_to_token(b"NBY"),
	
	/// TODO.
	NBZ = Self::letters_to_token(b"NBZ"),
	
	/// TODO.
	NCA = Self::letters_to_token(b"NCA"),
	
	/// TODO.
	NCB = Self::letters_to_token(b"NCB"),
	
	/// TODO.
	NCC = Self::letters_to_token(b"NCC"),
	
	/// TODO.
	NCD = Self::letters_to_token(b"NCD"),
	
	/// TODO.
	NCE = Self::letters_to_token(b"NCE"),
	
	/// TODO.
	NCF = Self::letters_to_token(b"NCF"),
	
	/// TODO.
	NCG = Self::letters_to_token(b"NCG"),
	
	/// TODO.
	NCH = Self::letters_to_token(b"NCH"),
	
	/// TODO.
	NCI = Self::letters_to_token(b"NCI"),
	
	/// TODO.
	NCJ = Self::letters_to_token(b"NCJ"),
	
	/// TODO.
	NCK = Self::letters_to_token(b"NCK"),
	
	/// TODO.
	NCL = Self::letters_to_token(b"NCL"),
	
	/// TODO.
	NCM = Self::letters_to_token(b"NCM"),
	
	/// TODO.
	NCN = Self::letters_to_token(b"NCN"),
	
	/// TODO.
	NCO = Self::letters_to_token(b"NCO"),
	
	/// TODO.
	NCP = Self::letters_to_token(b"NCP"),
	
	/// TODO.
	NCQ = Self::letters_to_token(b"NCQ"),
	
	/// TODO.
	NCR = Self::letters_to_token(b"NCR"),
	
	/// TODO.
	NCS = Self::letters_to_token(b"NCS"),
	
	/// TODO.
	NCT = Self::letters_to_token(b"NCT"),
	
	/// TODO.
	NCU = Self::letters_to_token(b"NCU"),
	
	/// TODO.
	NCV = Self::letters_to_token(b"NCV"),
	
	/// TODO.
	NCW = Self::letters_to_token(b"NCW"),
	
	/// TODO.
	NCX = Self::letters_to_token(b"NCX"),
	
	/// TODO.
	NCY = Self::letters_to_token(b"NCY"),
	
	/// TODO.
	NCZ = Self::letters_to_token(b"NCZ"),
	
	/// TODO.
	NDA = Self::letters_to_token(b"NDA"),
	
	/// TODO.
	NDB = Self::letters_to_token(b"NDB"),
	
	/// TODO.
	NDC = Self::letters_to_token(b"NDC"),
	
	/// TODO.
	NDD = Self::letters_to_token(b"NDD"),
	
	/// TODO.
	NDE = Self::letters_to_token(b"NDE"),
	
	/// TODO.
	NDF = Self::letters_to_token(b"NDF"),
	
	/// TODO.
	NDG = Self::letters_to_token(b"NDG"),
	
	/// TODO.
	NDH = Self::letters_to_token(b"NDH"),
	
	/// TODO.
	NDI = Self::letters_to_token(b"NDI"),
	
	/// TODO.
	NDJ = Self::letters_to_token(b"NDJ"),
	
	/// TODO.
	NDK = Self::letters_to_token(b"NDK"),
	
	/// TODO.
	NDL = Self::letters_to_token(b"NDL"),
	
	/// TODO.
	NDM = Self::letters_to_token(b"NDM"),
	
	/// TODO.
	NDN = Self::letters_to_token(b"NDN"),
	
	/// TODO.
	NDO = Self::letters_to_token(b"NDO"),
	
	/// TODO.
	NDP = Self::letters_to_token(b"NDP"),
	
	/// TODO.
	NDQ = Self::letters_to_token(b"NDQ"),
	
	/// TODO.
	NDR = Self::letters_to_token(b"NDR"),
	
	/// TODO.
	NDS = Self::letters_to_token(b"NDS"),
	
	/// TODO.
	NDT = Self::letters_to_token(b"NDT"),
	
	/// TODO.
	NDU = Self::letters_to_token(b"NDU"),
	
	/// TODO.
	NDV = Self::letters_to_token(b"NDV"),
	
	/// TODO.
	NDW = Self::letters_to_token(b"NDW"),
	
	/// TODO.
	NDX = Self::letters_to_token(b"NDX"),
	
	/// TODO.
	NDY = Self::letters_to_token(b"NDY"),
	
	/// TODO.
	NDZ = Self::letters_to_token(b"NDZ"),
	
	/// TODO.
	NEA = Self::letters_to_token(b"NEA"),
	
	/// TODO.
	NEB = Self::letters_to_token(b"NEB"),
	
	/// TODO.
	NEC = Self::letters_to_token(b"NEC"),
	
	/// TODO.
	NED = Self::letters_to_token(b"NED"),
	
	/// TODO.
	NEE = Self::letters_to_token(b"NEE"),
	
	/// TODO.
	NEF = Self::letters_to_token(b"NEF"),
	
	/// TODO.
	NEG = Self::letters_to_token(b"NEG"),
	
	/// TODO.
	NEH = Self::letters_to_token(b"NEH"),
	
	/// TODO.
	NEI = Self::letters_to_token(b"NEI"),
	
	/// TODO.
	NEJ = Self::letters_to_token(b"NEJ"),
	
	/// TODO.
	NEK = Self::letters_to_token(b"NEK"),
	
	/// TODO.
	NEL = Self::letters_to_token(b"NEL"),
	
	/// TODO.
	NEM = Self::letters_to_token(b"NEM"),
	
	/// TODO.
	NEN = Self::letters_to_token(b"NEN"),
	
	/// TODO.
	NEO = Self::letters_to_token(b"NEO"),
	
	/// TODO.
	NEP = Self::letters_to_token(b"NEP"),
	
	/// TODO.
	NEQ = Self::letters_to_token(b"NEQ"),
	
	/// TODO.
	NER = Self::letters_to_token(b"NER"),
	
	/// TODO.
	NES = Self::letters_to_token(b"NES"),
	
	/// TODO.
	NET = Self::letters_to_token(b"NET"),
	
	/// TODO.
	NEU = Self::letters_to_token(b"NEU"),
	
	/// TODO.
	NEV = Self::letters_to_token(b"NEV"),
	
	/// TODO.
	NEW = Self::letters_to_token(b"NEW"),
	
	/// TODO.
	NEX = Self::letters_to_token(b"NEX"),
	
	/// TODO.
	NEY = Self::letters_to_token(b"NEY"),
	
	/// TODO.
	NEZ = Self::letters_to_token(b"NEZ"),
	
	/// TODO.
	NFA = Self::letters_to_token(b"NFA"),
	
	/// TODO.
	NFB = Self::letters_to_token(b"NFB"),
	
	/// TODO.
	NFC = Self::letters_to_token(b"NFC"),
	
	/// TODO.
	NFD = Self::letters_to_token(b"NFD"),
	
	/// TODO.
	NFE = Self::letters_to_token(b"NFE"),
	
	/// TODO.
	NFF = Self::letters_to_token(b"NFF"),
	
	/// TODO.
	NFG = Self::letters_to_token(b"NFG"),
	
	/// TODO.
	NFH = Self::letters_to_token(b"NFH"),
	
	/// TODO.
	NFI = Self::letters_to_token(b"NFI"),
	
	/// TODO.
	NFJ = Self::letters_to_token(b"NFJ"),
	
	/// Norfolk Island.
	NFK = Self::letters_to_token(b"NFK"),
	
	/// TODO.
	NFL = Self::letters_to_token(b"NFL"),
	
	/// TODO.
	NFM = Self::letters_to_token(b"NFM"),
	
	/// TODO.
	NFN = Self::letters_to_token(b"NFN"),
	
	/// TODO.
	NFO = Self::letters_to_token(b"NFO"),
	
	/// TODO.
	NFP = Self::letters_to_token(b"NFP"),
	
	/// TODO.
	NFQ = Self::letters_to_token(b"NFQ"),
	
	/// TODO.
	NFR = Self::letters_to_token(b"NFR"),
	
	/// TODO.
	NFS = Self::letters_to_token(b"NFS"),
	
	/// TODO.
	NFT = Self::letters_to_token(b"NFT"),
	
	/// TODO.
	NFU = Self::letters_to_token(b"NFU"),
	
	/// TODO.
	NFV = Self::letters_to_token(b"NFV"),
	
	/// TODO.
	NFW = Self::letters_to_token(b"NFW"),
	
	/// TODO.
	NFX = Self::letters_to_token(b"NFX"),
	
	/// TODO.
	NFY = Self::letters_to_token(b"NFY"),
	
	/// TODO.
	NFZ = Self::letters_to_token(b"NFZ"),
	
	/// TODO.
	NGA = Self::letters_to_token(b"NGA"),
	
	/// TODO.
	NGB = Self::letters_to_token(b"NGB"),
	
	/// TODO.
	NGC = Self::letters_to_token(b"NGC"),
	
	/// TODO.
	NGD = Self::letters_to_token(b"NGD"),
	
	/// TODO.
	NGE = Self::letters_to_token(b"NGE"),
	
	/// TODO.
	NGF = Self::letters_to_token(b"NGF"),
	
	/// TODO.
	NGG = Self::letters_to_token(b"NGG"),
	
	/// TODO.
	NGH = Self::letters_to_token(b"NGH"),
	
	/// TODO.
	NGI = Self::letters_to_token(b"NGI"),
	
	/// TODO.
	NGJ = Self::letters_to_token(b"NGJ"),
	
	/// TODO.
	NGK = Self::letters_to_token(b"NGK"),
	
	/// TODO.
	NGL = Self::letters_to_token(b"NGL"),
	
	/// TODO.
	NGM = Self::letters_to_token(b"NGM"),
	
	/// TODO.
	NGN = Self::letters_to_token(b"NGN"),
	
	/// TODO.
	NGO = Self::letters_to_token(b"NGO"),
	
	/// TODO.
	NGP = Self::letters_to_token(b"NGP"),
	
	/// TODO.
	NGQ = Self::letters_to_token(b"NGQ"),
	
	/// TODO.
	NGR = Self::letters_to_token(b"NGR"),
	
	/// TODO.
	NGS = Self::letters_to_token(b"NGS"),
	
	/// TODO.
	NGT = Self::letters_to_token(b"NGT"),
	
	/// TODO.
	NGU = Self::letters_to_token(b"NGU"),
	
	/// TODO.
	NGV = Self::letters_to_token(b"NGV"),
	
	/// TODO.
	NGW = Self::letters_to_token(b"NGW"),
	
	/// TODO.
	NGX = Self::letters_to_token(b"NGX"),
	
	/// TODO.
	NGY = Self::letters_to_token(b"NGY"),
	
	/// TODO.
	NGZ = Self::letters_to_token(b"NGZ"),
	
	/// TODO.
	NHA = Self::letters_to_token(b"NHA"),
	
	/// TODO.
	NHB = Self::letters_to_token(b"NHB"),
	
	/// TODO.
	NHC = Self::letters_to_token(b"NHC"),
	
	/// TODO.
	NHD = Self::letters_to_token(b"NHD"),
	
	/// TODO.
	NHE = Self::letters_to_token(b"NHE"),
	
	/// TODO.
	NHF = Self::letters_to_token(b"NHF"),
	
	/// TODO.
	NHG = Self::letters_to_token(b"NHG"),
	
	/// TODO.
	NHH = Self::letters_to_token(b"NHH"),
	
	/// TODO.
	NHI = Self::letters_to_token(b"NHI"),
	
	/// TODO.
	NHJ = Self::letters_to_token(b"NHJ"),
	
	/// TODO.
	NHK = Self::letters_to_token(b"NHK"),
	
	/// TODO.
	NHL = Self::letters_to_token(b"NHL"),
	
	/// TODO.
	NHM = Self::letters_to_token(b"NHM"),
	
	/// TODO.
	NHN = Self::letters_to_token(b"NHN"),
	
	/// TODO.
	NHO = Self::letters_to_token(b"NHO"),
	
	/// TODO.
	NHP = Self::letters_to_token(b"NHP"),
	
	/// TODO.
	NHQ = Self::letters_to_token(b"NHQ"),
	
	/// TODO.
	NHR = Self::letters_to_token(b"NHR"),
	
	/// TODO.
	NHS = Self::letters_to_token(b"NHS"),
	
	/// TODO.
	NHT = Self::letters_to_token(b"NHT"),
	
	/// TODO.
	NHU = Self::letters_to_token(b"NHU"),
	
	/// TODO.
	NHV = Self::letters_to_token(b"NHV"),
	
	/// TODO.
	NHW = Self::letters_to_token(b"NHW"),
	
	/// TODO.
	NHX = Self::letters_to_token(b"NHX"),
	
	/// TODO.
	NHY = Self::letters_to_token(b"NHY"),
	
	/// TODO.
	NHZ = Self::letters_to_token(b"NHZ"),
	
	/// TODO.
	NIA = Self::letters_to_token(b"NIA"),
	
	/// TODO.
	NIB = Self::letters_to_token(b"NIB"),
	
	/// TODO.
	NIC = Self::letters_to_token(b"NIC"),
	
	/// TODO.
	NID = Self::letters_to_token(b"NID"),
	
	/// TODO.
	NIE = Self::letters_to_token(b"NIE"),
	
	/// TODO.
	NIF = Self::letters_to_token(b"NIF"),
	
	/// TODO.
	NIG = Self::letters_to_token(b"NIG"),
	
	/// TODO.
	NIH = Self::letters_to_token(b"NIH"),
	
	/// TODO.
	NII = Self::letters_to_token(b"NII"),
	
	/// TODO.
	NIJ = Self::letters_to_token(b"NIJ"),
	
	/// TODO.
	NIK = Self::letters_to_token(b"NIK"),
	
	/// TODO.
	NIL = Self::letters_to_token(b"NIL"),
	
	/// TODO.
	NIM = Self::letters_to_token(b"NIM"),
	
	/// TODO.
	NIN = Self::letters_to_token(b"NIN"),
	
	/// TODO.
	NIO = Self::letters_to_token(b"NIO"),
	
	/// TODO.
	NIP = Self::letters_to_token(b"NIP"),
	
	/// TODO.
	NIQ = Self::letters_to_token(b"NIQ"),
	
	/// TODO.
	NIR = Self::letters_to_token(b"NIR"),
	
	/// TODO.
	NIS = Self::letters_to_token(b"NIS"),
	
	/// TODO.
	NIT = Self::letters_to_token(b"NIT"),
	
	/// TODO.
	NIU = Self::letters_to_token(b"NIU"),
	
	/// TODO.
	NIV = Self::letters_to_token(b"NIV"),
	
	/// TODO.
	NIW = Self::letters_to_token(b"NIW"),
	
	/// TODO.
	NIX = Self::letters_to_token(b"NIX"),
	
	/// TODO.
	NIY = Self::letters_to_token(b"NIY"),
	
	/// TODO.
	NIZ = Self::letters_to_token(b"NIZ"),
	
	/// TODO.
	NJA = Self::letters_to_token(b"NJA"),
	
	/// TODO.
	NJB = Self::letters_to_token(b"NJB"),
	
	/// TODO.
	NJC = Self::letters_to_token(b"NJC"),
	
	/// TODO.
	NJD = Self::letters_to_token(b"NJD"),
	
	/// TODO.
	NJE = Self::letters_to_token(b"NJE"),
	
	/// TODO.
	NJF = Self::letters_to_token(b"NJF"),
	
	/// TODO.
	NJG = Self::letters_to_token(b"NJG"),
	
	/// TODO.
	NJH = Self::letters_to_token(b"NJH"),
	
	/// TODO.
	NJI = Self::letters_to_token(b"NJI"),
	
	/// TODO.
	NJJ = Self::letters_to_token(b"NJJ"),
	
	/// TODO.
	NJK = Self::letters_to_token(b"NJK"),
	
	/// TODO.
	NJL = Self::letters_to_token(b"NJL"),
	
	/// TODO.
	NJM = Self::letters_to_token(b"NJM"),
	
	/// TODO.
	NJN = Self::letters_to_token(b"NJN"),
	
	/// TODO.
	NJO = Self::letters_to_token(b"NJO"),
	
	/// TODO.
	NJP = Self::letters_to_token(b"NJP"),
	
	/// TODO.
	NJQ = Self::letters_to_token(b"NJQ"),
	
	/// TODO.
	NJR = Self::letters_to_token(b"NJR"),
	
	/// TODO.
	NJS = Self::letters_to_token(b"NJS"),
	
	/// TODO.
	NJT = Self::letters_to_token(b"NJT"),
	
	/// TODO.
	NJU = Self::letters_to_token(b"NJU"),
	
	/// TODO.
	NJV = Self::letters_to_token(b"NJV"),
	
	/// TODO.
	NJW = Self::letters_to_token(b"NJW"),
	
	/// TODO.
	NJX = Self::letters_to_token(b"NJX"),
	
	/// TODO.
	NJY = Self::letters_to_token(b"NJY"),
	
	/// TODO.
	NJZ = Self::letters_to_token(b"NJZ"),
	
	/// TODO.
	NKA = Self::letters_to_token(b"NKA"),
	
	/// TODO.
	NKB = Self::letters_to_token(b"NKB"),
	
	/// TODO.
	NKC = Self::letters_to_token(b"NKC"),
	
	/// TODO.
	NKD = Self::letters_to_token(b"NKD"),
	
	/// TODO.
	NKE = Self::letters_to_token(b"NKE"),
	
	/// TODO.
	NKF = Self::letters_to_token(b"NKF"),
	
	/// TODO.
	NKG = Self::letters_to_token(b"NKG"),
	
	/// TODO.
	NKH = Self::letters_to_token(b"NKH"),
	
	/// TODO.
	NKI = Self::letters_to_token(b"NKI"),
	
	/// TODO.
	NKJ = Self::letters_to_token(b"NKJ"),
	
	/// TODO.
	NKK = Self::letters_to_token(b"NKK"),
	
	/// TODO.
	NKL = Self::letters_to_token(b"NKL"),
	
	/// TODO.
	NKM = Self::letters_to_token(b"NKM"),
	
	/// TODO.
	NKN = Self::letters_to_token(b"NKN"),
	
	/// TODO.
	NKO = Self::letters_to_token(b"NKO"),
	
	/// TODO.
	NKP = Self::letters_to_token(b"NKP"),
	
	/// TODO.
	NKQ = Self::letters_to_token(b"NKQ"),
	
	/// TODO.
	NKR = Self::letters_to_token(b"NKR"),
	
	/// TODO.
	NKS = Self::letters_to_token(b"NKS"),
	
	/// TODO.
	NKT = Self::letters_to_token(b"NKT"),
	
	/// TODO.
	NKU = Self::letters_to_token(b"NKU"),
	
	/// TODO.
	NKV = Self::letters_to_token(b"NKV"),
	
	/// TODO.
	NKW = Self::letters_to_token(b"NKW"),
	
	/// TODO.
	NKX = Self::letters_to_token(b"NKX"),
	
	/// TODO.
	NKY = Self::letters_to_token(b"NKY"),
	
	/// TODO.
	NKZ = Self::letters_to_token(b"NKZ"),
	
	/// TODO.
	NLA = Self::letters_to_token(b"NLA"),
	
	/// TODO.
	NLB = Self::letters_to_token(b"NLB"),
	
	/// TODO.
	NLC = Self::letters_to_token(b"NLC"),
	
	/// TODO.
	NLD = Self::letters_to_token(b"NLD"),
	
	/// TODO.
	NLE = Self::letters_to_token(b"NLE"),
	
	/// TODO.
	NLF = Self::letters_to_token(b"NLF"),
	
	/// TODO.
	NLG = Self::letters_to_token(b"NLG"),
	
	/// TODO.
	NLH = Self::letters_to_token(b"NLH"),
	
	/// TODO.
	NLI = Self::letters_to_token(b"NLI"),
	
	/// TODO.
	NLJ = Self::letters_to_token(b"NLJ"),
	
	/// TODO.
	NLK = Self::letters_to_token(b"NLK"),
	
	/// TODO.
	NLL = Self::letters_to_token(b"NLL"),
	
	/// TODO.
	NLM = Self::letters_to_token(b"NLM"),
	
	/// TODO.
	NLN = Self::letters_to_token(b"NLN"),
	
	/// TODO.
	NLO = Self::letters_to_token(b"NLO"),
	
	/// TODO.
	NLP = Self::letters_to_token(b"NLP"),
	
	/// TODO.
	NLQ = Self::letters_to_token(b"NLQ"),
	
	/// TODO.
	NLR = Self::letters_to_token(b"NLR"),
	
	/// TODO.
	NLS = Self::letters_to_token(b"NLS"),
	
	/// TODO.
	NLT = Self::letters_to_token(b"NLT"),
	
	/// TODO.
	NLU = Self::letters_to_token(b"NLU"),
	
	/// TODO.
	NLV = Self::letters_to_token(b"NLV"),
	
	/// TODO.
	NLW = Self::letters_to_token(b"NLW"),
	
	/// TODO.
	NLX = Self::letters_to_token(b"NLX"),
	
	/// TODO.
	NLY = Self::letters_to_token(b"NLY"),
	
	/// TODO.
	NLZ = Self::letters_to_token(b"NLZ"),
	
	/// TODO.
	NMA = Self::letters_to_token(b"NMA"),
	
	/// TODO.
	NMB = Self::letters_to_token(b"NMB"),
	
	/// TODO.
	NMC = Self::letters_to_token(b"NMC"),
	
	/// TODO.
	NMD = Self::letters_to_token(b"NMD"),
	
	/// TODO.
	NME = Self::letters_to_token(b"NME"),
	
	/// TODO.
	NMF = Self::letters_to_token(b"NMF"),
	
	/// TODO.
	NMG = Self::letters_to_token(b"NMG"),
	
	/// TODO.
	NMH = Self::letters_to_token(b"NMH"),
	
	/// TODO.
	NMI = Self::letters_to_token(b"NMI"),
	
	/// TODO.
	NMJ = Self::letters_to_token(b"NMJ"),
	
	/// TODO.
	NMK = Self::letters_to_token(b"NMK"),
	
	/// TODO.
	NML = Self::letters_to_token(b"NML"),
	
	/// TODO.
	NMM = Self::letters_to_token(b"NMM"),
	
	/// TODO.
	NMN = Self::letters_to_token(b"NMN"),
	
	/// TODO.
	NMO = Self::letters_to_token(b"NMO"),
	
	/// TODO.
	NMP = Self::letters_to_token(b"NMP"),
	
	/// TODO.
	NMQ = Self::letters_to_token(b"NMQ"),
	
	/// TODO.
	NMR = Self::letters_to_token(b"NMR"),
	
	/// TODO.
	NMS = Self::letters_to_token(b"NMS"),
	
	/// TODO.
	NMT = Self::letters_to_token(b"NMT"),
	
	/// TODO.
	NMU = Self::letters_to_token(b"NMU"),
	
	/// TODO.
	NMV = Self::letters_to_token(b"NMV"),
	
	/// TODO.
	NMW = Self::letters_to_token(b"NMW"),
	
	/// TODO.
	NMX = Self::letters_to_token(b"NMX"),
	
	/// TODO.
	NMY = Self::letters_to_token(b"NMY"),
	
	/// TODO.
	NMZ = Self::letters_to_token(b"NMZ"),
	
	/// TODO.
	NNA = Self::letters_to_token(b"NNA"),
	
	/// TODO.
	NNB = Self::letters_to_token(b"NNB"),
	
	/// TODO.
	NNC = Self::letters_to_token(b"NNC"),
	
	/// TODO.
	NND = Self::letters_to_token(b"NND"),
	
	/// TODO.
	NNE = Self::letters_to_token(b"NNE"),
	
	/// TODO.
	NNF = Self::letters_to_token(b"NNF"),
	
	/// TODO.
	NNG = Self::letters_to_token(b"NNG"),
	
	/// TODO.
	NNH = Self::letters_to_token(b"NNH"),
	
	/// TODO.
	NNI = Self::letters_to_token(b"NNI"),
	
	/// TODO.
	NNJ = Self::letters_to_token(b"NNJ"),
	
	/// TODO.
	NNK = Self::letters_to_token(b"NNK"),
	
	/// TODO.
	NNL = Self::letters_to_token(b"NNL"),
	
	/// TODO.
	NNM = Self::letters_to_token(b"NNM"),
	
	/// Unoffical.
	///
	/// North America (NATO STANAG 1059 INT).
	NNN = Self::letters_to_token(b"NNN"),
	
	/// TODO.
	NNO = Self::letters_to_token(b"NNO"),
	
	/// TODO.
	NNP = Self::letters_to_token(b"NNP"),
	
	/// TODO.
	NNQ = Self::letters_to_token(b"NNQ"),
	
	/// TODO.
	NNR = Self::letters_to_token(b"NNR"),
	
	/// TODO.
	NNS = Self::letters_to_token(b"NNS"),
	
	/// TODO.
	NNT = Self::letters_to_token(b"NNT"),
	
	/// TODO.
	NNU = Self::letters_to_token(b"NNU"),
	
	/// TODO.
	NNV = Self::letters_to_token(b"NNV"),
	
	/// TODO.
	NNW = Self::letters_to_token(b"NNW"),
	
	/// TODO.
	NNX = Self::letters_to_token(b"NNX"),
	
	/// TODO.
	NNY = Self::letters_to_token(b"NNY"),
	
	/// TODO.
	NNZ = Self::letters_to_token(b"NNZ"),
	
	/// TODO.
	NOA = Self::letters_to_token(b"NOA"),
	
	/// TODO.
	NOB = Self::letters_to_token(b"NOB"),
	
	/// TODO.
	NOC = Self::letters_to_token(b"NOC"),
	
	/// TODO.
	NOD = Self::letters_to_token(b"NOD"),
	
	/// TODO.
	NOE = Self::letters_to_token(b"NOE"),
	
	/// TODO.
	NOF = Self::letters_to_token(b"NOF"),
	
	/// TODO.
	NOG = Self::letters_to_token(b"NOG"),
	
	/// TODO.
	NOH = Self::letters_to_token(b"NOH"),
	
	/// TODO.
	NOI = Self::letters_to_token(b"NOI"),
	
	/// TODO.
	NOJ = Self::letters_to_token(b"NOJ"),
	
	/// TODO.
	NOK = Self::letters_to_token(b"NOK"),
	
	/// TODO.
	NOL = Self::letters_to_token(b"NOL"),
	
	/// TODO.
	NOM = Self::letters_to_token(b"NOM"),
	
	/// TODO.
	NON = Self::letters_to_token(b"NON"),
	
	/// TODO.
	NOO = Self::letters_to_token(b"NOO"),
	
	/// TODO.
	NOP = Self::letters_to_token(b"NOP"),
	
	/// TODO.
	NOQ = Self::letters_to_token(b"NOQ"),
	
	/// TODO.
	NOR = Self::letters_to_token(b"NOR"),
	
	/// TODO.
	NOS = Self::letters_to_token(b"NOS"),
	
	/// TODO.
	NOT = Self::letters_to_token(b"NOT"),
	
	/// TODO.
	NOU = Self::letters_to_token(b"NOU"),
	
	/// TODO.
	NOV = Self::letters_to_token(b"NOV"),
	
	/// TODO.
	NOW = Self::letters_to_token(b"NOW"),
	
	/// TODO.
	NOX = Self::letters_to_token(b"NOX"),
	
	/// TODO.
	NOY = Self::letters_to_token(b"NOY"),
	
	/// TODO.
	NOZ = Self::letters_to_token(b"NOZ"),
	
	/// TODO.
	NPA = Self::letters_to_token(b"NPA"),
	
	/// TODO.
	NPB = Self::letters_to_token(b"NPB"),
	
	/// TODO.
	NPC = Self::letters_to_token(b"NPC"),
	
	/// TODO.
	NPD = Self::letters_to_token(b"NPD"),
	
	/// TODO.
	NPE = Self::letters_to_token(b"NPE"),
	
	/// TODO.
	NPF = Self::letters_to_token(b"NPF"),
	
	/// TODO.
	NPG = Self::letters_to_token(b"NPG"),
	
	/// TODO.
	NPH = Self::letters_to_token(b"NPH"),
	
	/// TODO.
	NPI = Self::letters_to_token(b"NPI"),
	
	/// TODO.
	NPJ = Self::letters_to_token(b"NPJ"),
	
	/// TODO.
	NPK = Self::letters_to_token(b"NPK"),
	
	/// TODO.
	NPL = Self::letters_to_token(b"NPL"),
	
	/// TODO.
	NPM = Self::letters_to_token(b"NPM"),
	
	/// TODO.
	NPN = Self::letters_to_token(b"NPN"),
	
	/// TODO.
	NPO = Self::letters_to_token(b"NPO"),
	
	/// TODO.
	NPP = Self::letters_to_token(b"NPP"),
	
	/// TODO.
	NPQ = Self::letters_to_token(b"NPQ"),
	
	/// TODO.
	NPR = Self::letters_to_token(b"NPR"),
	
	/// TODO.
	NPS = Self::letters_to_token(b"NPS"),
	
	/// TODO.
	NPT = Self::letters_to_token(b"NPT"),
	
	/// TODO.
	NPU = Self::letters_to_token(b"NPU"),
	
	/// TODO.
	NPV = Self::letters_to_token(b"NPV"),
	
	/// TODO.
	NPW = Self::letters_to_token(b"NPW"),
	
	/// TODO.
	NPX = Self::letters_to_token(b"NPX"),
	
	/// TODO.
	NPY = Self::letters_to_token(b"NPY"),
	
	/// TODO.
	NPZ = Self::letters_to_token(b"NPZ"),
	
	/// TODO.
	NQA = Self::letters_to_token(b"NQA"),
	
	/// TODO.
	NQB = Self::letters_to_token(b"NQB"),
	
	/// TODO.
	NQC = Self::letters_to_token(b"NQC"),
	
	/// TODO.
	NQD = Self::letters_to_token(b"NQD"),
	
	/// TODO.
	NQE = Self::letters_to_token(b"NQE"),
	
	/// TODO.
	NQF = Self::letters_to_token(b"NQF"),
	
	/// TODO.
	NQG = Self::letters_to_token(b"NQG"),
	
	/// TODO.
	NQH = Self::letters_to_token(b"NQH"),
	
	/// TODO.
	NQI = Self::letters_to_token(b"NQI"),
	
	/// TODO.
	NQJ = Self::letters_to_token(b"NQJ"),
	
	/// TODO.
	NQK = Self::letters_to_token(b"NQK"),
	
	/// TODO.
	NQL = Self::letters_to_token(b"NQL"),
	
	/// TODO.
	NQM = Self::letters_to_token(b"NQM"),
	
	/// TODO.
	NQN = Self::letters_to_token(b"NQN"),
	
	/// TODO.
	NQO = Self::letters_to_token(b"NQO"),
	
	/// TODO.
	NQP = Self::letters_to_token(b"NQP"),
	
	/// TODO.
	NQQ = Self::letters_to_token(b"NQQ"),
	
	/// TODO.
	NQR = Self::letters_to_token(b"NQR"),
	
	/// TODO.
	NQS = Self::letters_to_token(b"NQS"),
	
	/// TODO.
	NQT = Self::letters_to_token(b"NQT"),
	
	/// TODO.
	NQU = Self::letters_to_token(b"NQU"),
	
	/// TODO.
	NQV = Self::letters_to_token(b"NQV"),
	
	/// TODO.
	NQW = Self::letters_to_token(b"NQW"),
	
	/// TODO.
	NQX = Self::letters_to_token(b"NQX"),
	
	/// TODO.
	NQY = Self::letters_to_token(b"NQY"),
	
	/// TODO.
	NQZ = Self::letters_to_token(b"NQZ"),
	
	/// TODO.
	NRA = Self::letters_to_token(b"NRA"),
	
	/// TODO.
	NRB = Self::letters_to_token(b"NRB"),
	
	/// TODO.
	NRC = Self::letters_to_token(b"NRC"),
	
	/// TODO.
	NRD = Self::letters_to_token(b"NRD"),
	
	/// TODO.
	NRE = Self::letters_to_token(b"NRE"),
	
	/// TODO.
	NRF = Self::letters_to_token(b"NRF"),
	
	/// TODO.
	NRG = Self::letters_to_token(b"NRG"),
	
	/// TODO.
	NRH = Self::letters_to_token(b"NRH"),
	
	/// TODO.
	NRI = Self::letters_to_token(b"NRI"),
	
	/// TODO.
	NRJ = Self::letters_to_token(b"NRJ"),
	
	/// TODO.
	NRK = Self::letters_to_token(b"NRK"),
	
	/// TODO.
	NRL = Self::letters_to_token(b"NRL"),
	
	/// TODO.
	NRM = Self::letters_to_token(b"NRM"),
	
	/// TODO.
	NRN = Self::letters_to_token(b"NRN"),
	
	/// TODO.
	NRO = Self::letters_to_token(b"NRO"),
	
	/// TODO.
	NRP = Self::letters_to_token(b"NRP"),
	
	/// TODO.
	NRQ = Self::letters_to_token(b"NRQ"),
	
	/// TODO.
	NRR = Self::letters_to_token(b"NRR"),
	
	/// TODO.
	NRS = Self::letters_to_token(b"NRS"),
	
	/// TODO.
	NRT = Self::letters_to_token(b"NRT"),
	
	/// TODO.
	NRU = Self::letters_to_token(b"NRU"),
	
	/// TODO.
	NRV = Self::letters_to_token(b"NRV"),
	
	/// TODO.
	NRW = Self::letters_to_token(b"NRW"),
	
	/// TODO.
	NRX = Self::letters_to_token(b"NRX"),
	
	/// TODO.
	NRY = Self::letters_to_token(b"NRY"),
	
	/// TODO.
	NRZ = Self::letters_to_token(b"NRZ"),
	
	/// TODO.
	NSA = Self::letters_to_token(b"NSA"),
	
	/// TODO.
	NSB = Self::letters_to_token(b"NSB"),
	
	/// TODO.
	NSC = Self::letters_to_token(b"NSC"),
	
	/// TODO.
	NSD = Self::letters_to_token(b"NSD"),
	
	/// TODO.
	NSE = Self::letters_to_token(b"NSE"),
	
	/// TODO.
	NSF = Self::letters_to_token(b"NSF"),
	
	/// TODO.
	NSG = Self::letters_to_token(b"NSG"),
	
	/// TODO.
	NSH = Self::letters_to_token(b"NSH"),
	
	/// TODO.
	NSI = Self::letters_to_token(b"NSI"),
	
	/// TODO.
	NSJ = Self::letters_to_token(b"NSJ"),
	
	/// TODO.
	NSK = Self::letters_to_token(b"NSK"),
	
	/// TODO.
	NSL = Self::letters_to_token(b"NSL"),
	
	/// TODO.
	NSM = Self::letters_to_token(b"NSM"),
	
	/// TODO.
	NSN = Self::letters_to_token(b"NSN"),
	
	/// TODO.
	NSO = Self::letters_to_token(b"NSO"),
	
	/// TODO.
	NSP = Self::letters_to_token(b"NSP"),
	
	/// TODO.
	NSQ = Self::letters_to_token(b"NSQ"),
	
	/// TODO.
	NSR = Self::letters_to_token(b"NSR"),
	
	/// TODO.
	NSS = Self::letters_to_token(b"NSS"),
	
	/// TODO.
	NST = Self::letters_to_token(b"NST"),
	
	/// TODO.
	NSU = Self::letters_to_token(b"NSU"),
	
	/// TODO.
	NSV = Self::letters_to_token(b"NSV"),
	
	/// TODO.
	NSW = Self::letters_to_token(b"NSW"),
	
	/// TODO.
	NSX = Self::letters_to_token(b"NSX"),
	
	/// TODO.
	NSY = Self::letters_to_token(b"NSY"),
	
	/// TODO.
	NSZ = Self::letters_to_token(b"NSZ"),
	
	/// TODO.
	NTA = Self::letters_to_token(b"NTA"),
	
	/// TODO.
	NTB = Self::letters_to_token(b"NTB"),
	
	/// TODO.
	NTC = Self::letters_to_token(b"NTC"),
	
	/// TODO.
	NTD = Self::letters_to_token(b"NTD"),
	
	/// TODO.
	NTE = Self::letters_to_token(b"NTE"),
	
	/// TODO.
	NTF = Self::letters_to_token(b"NTF"),
	
	/// TODO.
	NTG = Self::letters_to_token(b"NTG"),
	
	/// TODO.
	NTH = Self::letters_to_token(b"NTH"),
	
	/// TODO.
	NTI = Self::letters_to_token(b"NTI"),
	
	/// TODO.
	NTJ = Self::letters_to_token(b"NTJ"),
	
	/// TODO.
	NTK = Self::letters_to_token(b"NTK"),
	
	/// TODO.
	NTL = Self::letters_to_token(b"NTL"),
	
	/// TODO.
	NTM = Self::letters_to_token(b"NTM"),
	
	/// TODO.
	NTN = Self::letters_to_token(b"NTN"),
	
	/// TODO.
	NTO = Self::letters_to_token(b"NTO"),
	
	/// TODO.
	NTP = Self::letters_to_token(b"NTP"),
	
	/// TODO.
	NTQ = Self::letters_to_token(b"NTQ"),
	
	/// TODO.
	NTR = Self::letters_to_token(b"NTR"),
	
	/// TODO.
	NTS = Self::letters_to_token(b"NTS"),
	
	/// Unoffical.
	///
	/// NATO countries (NATO STANAG 1059 INT).
	NTT = Self::letters_to_token(b"NTT"),
	
	/// TODO.
	NTU = Self::letters_to_token(b"NTU"),
	
	/// TODO.
	NTV = Self::letters_to_token(b"NTV"),
	
	/// TODO.
	NTW = Self::letters_to_token(b"NTW"),
	
	/// TODO.
	NTX = Self::letters_to_token(b"NTX"),
	
	/// TODO.
	NTY = Self::letters_to_token(b"NTY"),
	
	/// TODO.
	NTZ = Self::letters_to_token(b"NTZ"),
	
	/// TODO.
	NUA = Self::letters_to_token(b"NUA"),
	
	/// TODO.
	NUB = Self::letters_to_token(b"NUB"),
	
	/// TODO.
	NUC = Self::letters_to_token(b"NUC"),
	
	/// TODO.
	NUD = Self::letters_to_token(b"NUD"),
	
	/// TODO.
	NUE = Self::letters_to_token(b"NUE"),
	
	/// TODO.
	NUF = Self::letters_to_token(b"NUF"),
	
	/// TODO.
	NUG = Self::letters_to_token(b"NUG"),
	
	/// TODO.
	NUH = Self::letters_to_token(b"NUH"),
	
	/// TODO.
	NUI = Self::letters_to_token(b"NUI"),
	
	/// TODO.
	NUJ = Self::letters_to_token(b"NUJ"),
	
	/// TODO.
	NUK = Self::letters_to_token(b"NUK"),
	
	/// TODO.
	NUL = Self::letters_to_token(b"NUL"),
	
	/// TODO.
	NUM = Self::letters_to_token(b"NUM"),
	
	/// TODO.
	NUN = Self::letters_to_token(b"NUN"),
	
	/// TODO.
	NUO = Self::letters_to_token(b"NUO"),
	
	/// TODO.
	NUP = Self::letters_to_token(b"NUP"),
	
	/// TODO.
	NUQ = Self::letters_to_token(b"NUQ"),
	
	/// TODO.
	NUR = Self::letters_to_token(b"NUR"),
	
	/// TODO.
	NUS = Self::letters_to_token(b"NUS"),
	
	/// TODO.
	NUT = Self::letters_to_token(b"NUT"),
	
	/// TODO.
	NUU = Self::letters_to_token(b"NUU"),
	
	/// TODO.
	NUV = Self::letters_to_token(b"NUV"),
	
	/// TODO.
	NUW = Self::letters_to_token(b"NUW"),
	
	/// TODO.
	NUX = Self::letters_to_token(b"NUX"),
	
	/// TODO.
	NUY = Self::letters_to_token(b"NUY"),
	
	/// TODO.
	NUZ = Self::letters_to_token(b"NUZ"),
	
	/// TODO.
	NVA = Self::letters_to_token(b"NVA"),
	
	/// TODO.
	NVB = Self::letters_to_token(b"NVB"),
	
	/// TODO.
	NVC = Self::letters_to_token(b"NVC"),
	
	/// TODO.
	NVD = Self::letters_to_token(b"NVD"),
	
	/// TODO.
	NVE = Self::letters_to_token(b"NVE"),
	
	/// TODO.
	NVF = Self::letters_to_token(b"NVF"),
	
	/// TODO.
	NVG = Self::letters_to_token(b"NVG"),
	
	/// TODO.
	NVH = Self::letters_to_token(b"NVH"),
	
	/// TODO.
	NVI = Self::letters_to_token(b"NVI"),
	
	/// TODO.
	NVJ = Self::letters_to_token(b"NVJ"),
	
	/// TODO.
	NVK = Self::letters_to_token(b"NVK"),
	
	/// TODO.
	NVL = Self::letters_to_token(b"NVL"),
	
	/// TODO.
	NVM = Self::letters_to_token(b"NVM"),
	
	/// TODO.
	NVN = Self::letters_to_token(b"NVN"),
	
	/// TODO.
	NVO = Self::letters_to_token(b"NVO"),
	
	/// TODO.
	NVP = Self::letters_to_token(b"NVP"),
	
	/// TODO.
	NVQ = Self::letters_to_token(b"NVQ"),
	
	/// TODO.
	NVR = Self::letters_to_token(b"NVR"),
	
	/// TODO.
	NVS = Self::letters_to_token(b"NVS"),
	
	/// TODO.
	NVT = Self::letters_to_token(b"NVT"),
	
	/// TODO.
	NVU = Self::letters_to_token(b"NVU"),
	
	/// TODO.
	NVV = Self::letters_to_token(b"NVV"),
	
	/// TODO.
	NVW = Self::letters_to_token(b"NVW"),
	
	/// TODO.
	NVX = Self::letters_to_token(b"NVX"),
	
	/// TODO.
	NVY = Self::letters_to_token(b"NVY"),
	
	/// TODO.
	NVZ = Self::letters_to_token(b"NVZ"),
	
	/// TODO.
	NWA = Self::letters_to_token(b"NWA"),
	
	/// TODO.
	NWB = Self::letters_to_token(b"NWB"),
	
	/// TODO.
	NWC = Self::letters_to_token(b"NWC"),
	
	/// TODO.
	NWD = Self::letters_to_token(b"NWD"),
	
	/// TODO.
	NWE = Self::letters_to_token(b"NWE"),
	
	/// TODO.
	NWF = Self::letters_to_token(b"NWF"),
	
	/// TODO.
	NWG = Self::letters_to_token(b"NWG"),
	
	/// TODO.
	NWH = Self::letters_to_token(b"NWH"),
	
	/// TODO.
	NWI = Self::letters_to_token(b"NWI"),
	
	/// TODO.
	NWJ = Self::letters_to_token(b"NWJ"),
	
	/// TODO.
	NWK = Self::letters_to_token(b"NWK"),
	
	/// TODO.
	NWL = Self::letters_to_token(b"NWL"),
	
	/// TODO.
	NWM = Self::letters_to_token(b"NWM"),
	
	/// TODO.
	NWN = Self::letters_to_token(b"NWN"),
	
	/// TODO.
	NWO = Self::letters_to_token(b"NWO"),
	
	/// TODO.
	NWP = Self::letters_to_token(b"NWP"),
	
	/// TODO.
	NWQ = Self::letters_to_token(b"NWQ"),
	
	/// TODO.
	NWR = Self::letters_to_token(b"NWR"),
	
	/// TODO.
	NWS = Self::letters_to_token(b"NWS"),
	
	/// TODO.
	NWT = Self::letters_to_token(b"NWT"),
	
	/// TODO.
	NWU = Self::letters_to_token(b"NWU"),
	
	/// TODO.
	NWV = Self::letters_to_token(b"NWV"),
	
	/// TODO.
	NWW = Self::letters_to_token(b"NWW"),
	
	/// TODO.
	NWX = Self::letters_to_token(b"NWX"),
	
	/// TODO.
	NWY = Self::letters_to_token(b"NWY"),
	
	/// TODO.
	NWZ = Self::letters_to_token(b"NWZ"),
	
	/// TODO.
	NXA = Self::letters_to_token(b"NXA"),
	
	/// TODO.
	NXB = Self::letters_to_token(b"NXB"),
	
	/// TODO.
	NXC = Self::letters_to_token(b"NXC"),
	
	/// TODO.
	NXD = Self::letters_to_token(b"NXD"),
	
	/// TODO.
	NXE = Self::letters_to_token(b"NXE"),
	
	/// TODO.
	NXF = Self::letters_to_token(b"NXF"),
	
	/// TODO.
	NXG = Self::letters_to_token(b"NXG"),
	
	/// TODO.
	NXH = Self::letters_to_token(b"NXH"),
	
	/// TODO.
	NXI = Self::letters_to_token(b"NXI"),
	
	/// TODO.
	NXJ = Self::letters_to_token(b"NXJ"),
	
	/// TODO.
	NXK = Self::letters_to_token(b"NXK"),
	
	/// TODO.
	NXL = Self::letters_to_token(b"NXL"),
	
	/// TODO.
	NXM = Self::letters_to_token(b"NXM"),
	
	/// TODO.
	NXN = Self::letters_to_token(b"NXN"),
	
	/// TODO.
	NXO = Self::letters_to_token(b"NXO"),
	
	/// TODO.
	NXP = Self::letters_to_token(b"NXP"),
	
	/// TODO.
	NXQ = Self::letters_to_token(b"NXQ"),
	
	/// TODO.
	NXR = Self::letters_to_token(b"NXR"),
	
	/// TODO.
	NXS = Self::letters_to_token(b"NXS"),
	
	/// TODO.
	NXT = Self::letters_to_token(b"NXT"),
	
	/// TODO.
	NXU = Self::letters_to_token(b"NXU"),
	
	/// TODO.
	NXV = Self::letters_to_token(b"NXV"),
	
	/// TODO.
	NXW = Self::letters_to_token(b"NXW"),
	
	/// TODO.
	NXX = Self::letters_to_token(b"NXX"),
	
	/// TODO.
	NXY = Self::letters_to_token(b"NXY"),
	
	/// TODO.
	NXZ = Self::letters_to_token(b"NXZ"),
	
	/// TODO.
	NYA = Self::letters_to_token(b"NYA"),
	
	/// TODO.
	NYB = Self::letters_to_token(b"NYB"),
	
	/// TODO.
	NYC = Self::letters_to_token(b"NYC"),
	
	/// TODO.
	NYD = Self::letters_to_token(b"NYD"),
	
	/// TODO.
	NYE = Self::letters_to_token(b"NYE"),
	
	/// TODO.
	NYF = Self::letters_to_token(b"NYF"),
	
	/// TODO.
	NYG = Self::letters_to_token(b"NYG"),
	
	/// TODO.
	NYH = Self::letters_to_token(b"NYH"),
	
	/// TODO.
	NYI = Self::letters_to_token(b"NYI"),
	
	/// TODO.
	NYJ = Self::letters_to_token(b"NYJ"),
	
	/// TODO.
	NYK = Self::letters_to_token(b"NYK"),
	
	/// TODO.
	NYL = Self::letters_to_token(b"NYL"),
	
	/// TODO.
	NYM = Self::letters_to_token(b"NYM"),
	
	/// TODO.
	NYN = Self::letters_to_token(b"NYN"),
	
	/// TODO.
	NYO = Self::letters_to_token(b"NYO"),
	
	/// TODO.
	NYP = Self::letters_to_token(b"NYP"),
	
	/// TODO.
	NYQ = Self::letters_to_token(b"NYQ"),
	
	/// TODO.
	NYR = Self::letters_to_token(b"NYR"),
	
	/// TODO.
	NYS = Self::letters_to_token(b"NYS"),
	
	/// TODO.
	NYT = Self::letters_to_token(b"NYT"),
	
	/// TODO.
	NYU = Self::letters_to_token(b"NYU"),
	
	/// TODO.
	NYV = Self::letters_to_token(b"NYV"),
	
	/// TODO.
	NYW = Self::letters_to_token(b"NYW"),
	
	/// TODO.
	NYX = Self::letters_to_token(b"NYX"),
	
	/// TODO.
	NYY = Self::letters_to_token(b"NYY"),
	
	/// TODO.
	NYZ = Self::letters_to_token(b"NYZ"),
	
	/// TODO.
	NZA = Self::letters_to_token(b"NZA"),
	
	/// TODO.
	NZB = Self::letters_to_token(b"NZB"),
	
	/// TODO.
	NZC = Self::letters_to_token(b"NZC"),
	
	/// TODO.
	NZD = Self::letters_to_token(b"NZD"),
	
	/// TODO.
	NZE = Self::letters_to_token(b"NZE"),
	
	/// TODO.
	NZF = Self::letters_to_token(b"NZF"),
	
	/// TODO.
	NZG = Self::letters_to_token(b"NZG"),
	
	/// TODO.
	NZH = Self::letters_to_token(b"NZH"),
	
	/// TODO.
	NZI = Self::letters_to_token(b"NZI"),
	
	/// TODO.
	NZJ = Self::letters_to_token(b"NZJ"),
	
	/// TODO.
	NZK = Self::letters_to_token(b"NZK"),
	
	/// TODO.
	NZL = Self::letters_to_token(b"NZL"),
	
	/// TODO.
	NZM = Self::letters_to_token(b"NZM"),
	
	/// TODO.
	NZN = Self::letters_to_token(b"NZN"),
	
	/// TODO.
	NZO = Self::letters_to_token(b"NZO"),
	
	/// TODO.
	NZP = Self::letters_to_token(b"NZP"),
	
	/// TODO.
	NZQ = Self::letters_to_token(b"NZQ"),
	
	/// TODO.
	NZR = Self::letters_to_token(b"NZR"),
	
	/// TODO.
	NZS = Self::letters_to_token(b"NZS"),
	
	/// TODO.
	NZT = Self::letters_to_token(b"NZT"),
	
	/// TODO.
	NZU = Self::letters_to_token(b"NZU"),
	
	/// TODO.
	NZV = Self::letters_to_token(b"NZV"),
	
	/// TODO.
	NZW = Self::letters_to_token(b"NZW"),
	
	/// TODO.
	NZX = Self::letters_to_token(b"NZX"),
	
	/// TODO.
	NZY = Self::letters_to_token(b"NZY"),
	
	/// TODO.
	NZZ = Self::letters_to_token(b"NZZ"),
	
	/// TODO.
	OAA = Self::letters_to_token(b"OAA"),
	
	/// TODO.
	OAB = Self::letters_to_token(b"OAB"),
	
	/// TODO.
	OAC = Self::letters_to_token(b"OAC"),
	
	/// TODO.
	OAD = Self::letters_to_token(b"OAD"),
	
	/// TODO.
	OAE = Self::letters_to_token(b"OAE"),
	
	/// TODO.
	OAF = Self::letters_to_token(b"OAF"),
	
	/// TODO.
	OAG = Self::letters_to_token(b"OAG"),
	
	/// TODO.
	OAH = Self::letters_to_token(b"OAH"),
	
	/// TODO.
	OAI = Self::letters_to_token(b"OAI"),
	
	/// TODO.
	OAJ = Self::letters_to_token(b"OAJ"),
	
	/// TODO.
	OAK = Self::letters_to_token(b"OAK"),
	
	/// TODO.
	OAL = Self::letters_to_token(b"OAL"),
	
	/// TODO.
	OAM = Self::letters_to_token(b"OAM"),
	
	/// TODO.
	OAN = Self::letters_to_token(b"OAN"),
	
	/// TODO.
	OAO = Self::letters_to_token(b"OAO"),
	
	/// TODO.
	OAP = Self::letters_to_token(b"OAP"),
	
	/// TODO.
	OAQ = Self::letters_to_token(b"OAQ"),
	
	/// TODO.
	OAR = Self::letters_to_token(b"OAR"),
	
	/// TODO.
	OAS = Self::letters_to_token(b"OAS"),
	
	/// TODO.
	OAT = Self::letters_to_token(b"OAT"),
	
	/// TODO.
	OAU = Self::letters_to_token(b"OAU"),
	
	/// TODO.
	OAV = Self::letters_to_token(b"OAV"),
	
	/// TODO.
	OAW = Self::letters_to_token(b"OAW"),
	
	/// TODO.
	OAX = Self::letters_to_token(b"OAX"),
	
	/// TODO.
	OAY = Self::letters_to_token(b"OAY"),
	
	/// TODO.
	OAZ = Self::letters_to_token(b"OAZ"),
	
	/// TODO.
	OBA = Self::letters_to_token(b"OBA"),
	
	/// TODO.
	OBB = Self::letters_to_token(b"OBB"),
	
	/// TODO.
	OBC = Self::letters_to_token(b"OBC"),
	
	/// TODO.
	OBD = Self::letters_to_token(b"OBD"),
	
	/// TODO.
	OBE = Self::letters_to_token(b"OBE"),
	
	/// TODO.
	OBF = Self::letters_to_token(b"OBF"),
	
	/// TODO.
	OBG = Self::letters_to_token(b"OBG"),
	
	/// TODO.
	OBH = Self::letters_to_token(b"OBH"),
	
	/// TODO.
	OBI = Self::letters_to_token(b"OBI"),
	
	/// TODO.
	OBJ = Self::letters_to_token(b"OBJ"),
	
	/// TODO.
	OBK = Self::letters_to_token(b"OBK"),
	
	/// TODO.
	OBL = Self::letters_to_token(b"OBL"),
	
	/// TODO.
	OBM = Self::letters_to_token(b"OBM"),
	
	/// TODO.
	OBN = Self::letters_to_token(b"OBN"),
	
	/// TODO.
	OBO = Self::letters_to_token(b"OBO"),
	
	/// TODO.
	OBP = Self::letters_to_token(b"OBP"),
	
	/// TODO.
	OBQ = Self::letters_to_token(b"OBQ"),
	
	/// TODO.
	OBR = Self::letters_to_token(b"OBR"),
	
	/// TODO.
	OBS = Self::letters_to_token(b"OBS"),
	
	/// TODO.
	OBT = Self::letters_to_token(b"OBT"),
	
	/// TODO.
	OBU = Self::letters_to_token(b"OBU"),
	
	/// TODO.
	OBV = Self::letters_to_token(b"OBV"),
	
	/// TODO.
	OBW = Self::letters_to_token(b"OBW"),
	
	/// TODO.
	OBX = Self::letters_to_token(b"OBX"),
	
	/// TODO.
	OBY = Self::letters_to_token(b"OBY"),
	
	/// TODO.
	OBZ = Self::letters_to_token(b"OBZ"),
	
	/// TODO.
	OCA = Self::letters_to_token(b"OCA"),
	
	/// TODO.
	OCB = Self::letters_to_token(b"OCB"),
	
	/// TODO.
	OCC = Self::letters_to_token(b"OCC"),
	
	/// TODO.
	OCD = Self::letters_to_token(b"OCD"),
	
	/// TODO.
	OCE = Self::letters_to_token(b"OCE"),
	
	/// TODO.
	OCF = Self::letters_to_token(b"OCF"),
	
	/// TODO.
	OCG = Self::letters_to_token(b"OCG"),
	
	/// TODO.
	OCH = Self::letters_to_token(b"OCH"),
	
	/// TODO.
	OCI = Self::letters_to_token(b"OCI"),
	
	/// TODO.
	OCJ = Self::letters_to_token(b"OCJ"),
	
	/// TODO.
	OCK = Self::letters_to_token(b"OCK"),
	
	/// TODO.
	OCL = Self::letters_to_token(b"OCL"),
	
	/// TODO.
	OCM = Self::letters_to_token(b"OCM"),
	
	/// TODO.
	OCN = Self::letters_to_token(b"OCN"),
	
	/// TODO.
	OCO = Self::letters_to_token(b"OCO"),
	
	/// TODO.
	OCP = Self::letters_to_token(b"OCP"),
	
	/// TODO.
	OCQ = Self::letters_to_token(b"OCQ"),
	
	/// TODO.
	OCR = Self::letters_to_token(b"OCR"),
	
	/// TODO.
	OCS = Self::letters_to_token(b"OCS"),
	
	/// TODO.
	OCT = Self::letters_to_token(b"OCT"),
	
	/// TODO.
	OCU = Self::letters_to_token(b"OCU"),
	
	/// TODO.
	OCV = Self::letters_to_token(b"OCV"),
	
	/// TODO.
	OCW = Self::letters_to_token(b"OCW"),
	
	/// TODO.
	OCX = Self::letters_to_token(b"OCX"),
	
	/// TODO.
	OCY = Self::letters_to_token(b"OCY"),
	
	/// TODO.
	OCZ = Self::letters_to_token(b"OCZ"),
	
	/// TODO.
	ODA = Self::letters_to_token(b"ODA"),
	
	/// TODO.
	ODB = Self::letters_to_token(b"ODB"),
	
	/// TODO.
	ODC = Self::letters_to_token(b"ODC"),
	
	/// TODO.
	ODD = Self::letters_to_token(b"ODD"),
	
	/// TODO.
	ODE = Self::letters_to_token(b"ODE"),
	
	/// TODO.
	ODF = Self::letters_to_token(b"ODF"),
	
	/// TODO.
	ODG = Self::letters_to_token(b"ODG"),
	
	/// TODO.
	ODH = Self::letters_to_token(b"ODH"),
	
	/// TODO.
	ODI = Self::letters_to_token(b"ODI"),
	
	/// TODO.
	ODJ = Self::letters_to_token(b"ODJ"),
	
	/// TODO.
	ODK = Self::letters_to_token(b"ODK"),
	
	/// TODO.
	ODL = Self::letters_to_token(b"ODL"),
	
	/// TODO.
	ODM = Self::letters_to_token(b"ODM"),
	
	/// TODO.
	ODN = Self::letters_to_token(b"ODN"),
	
	/// TODO.
	ODO = Self::letters_to_token(b"ODO"),
	
	/// TODO.
	ODP = Self::letters_to_token(b"ODP"),
	
	/// TODO.
	ODQ = Self::letters_to_token(b"ODQ"),
	
	/// TODO.
	ODR = Self::letters_to_token(b"ODR"),
	
	/// TODO.
	ODS = Self::letters_to_token(b"ODS"),
	
	/// TODO.
	ODT = Self::letters_to_token(b"ODT"),
	
	/// TODO.
	ODU = Self::letters_to_token(b"ODU"),
	
	/// TODO.
	ODV = Self::letters_to_token(b"ODV"),
	
	/// TODO.
	ODW = Self::letters_to_token(b"ODW"),
	
	/// TODO.
	ODX = Self::letters_to_token(b"ODX"),
	
	/// TODO.
	ODY = Self::letters_to_token(b"ODY"),
	
	/// TODO.
	ODZ = Self::letters_to_token(b"ODZ"),
	
	/// TODO.
	OEA = Self::letters_to_token(b"OEA"),
	
	/// TODO.
	OEB = Self::letters_to_token(b"OEB"),
	
	/// TODO.
	OEC = Self::letters_to_token(b"OEC"),
	
	/// TODO.
	OED = Self::letters_to_token(b"OED"),
	
	/// TODO.
	OEE = Self::letters_to_token(b"OEE"),
	
	/// TODO.
	OEF = Self::letters_to_token(b"OEF"),
	
	/// TODO.
	OEG = Self::letters_to_token(b"OEG"),
	
	/// TODO.
	OEH = Self::letters_to_token(b"OEH"),
	
	/// TODO.
	OEI = Self::letters_to_token(b"OEI"),
	
	/// TODO.
	OEJ = Self::letters_to_token(b"OEJ"),
	
	/// TODO.
	OEK = Self::letters_to_token(b"OEK"),
	
	/// TODO.
	OEL = Self::letters_to_token(b"OEL"),
	
	/// TODO.
	OEM = Self::letters_to_token(b"OEM"),
	
	/// TODO.
	OEN = Self::letters_to_token(b"OEN"),
	
	/// TODO.
	OEO = Self::letters_to_token(b"OEO"),
	
	/// TODO.
	OEP = Self::letters_to_token(b"OEP"),
	
	/// TODO.
	OEQ = Self::letters_to_token(b"OEQ"),
	
	/// TODO.
	OER = Self::letters_to_token(b"OER"),
	
	/// TODO.
	OES = Self::letters_to_token(b"OES"),
	
	/// TODO.
	OET = Self::letters_to_token(b"OET"),
	
	/// TODO.
	OEU = Self::letters_to_token(b"OEU"),
	
	/// TODO.
	OEV = Self::letters_to_token(b"OEV"),
	
	/// TODO.
	OEW = Self::letters_to_token(b"OEW"),
	
	/// TODO.
	OEX = Self::letters_to_token(b"OEX"),
	
	/// TODO.
	OEY = Self::letters_to_token(b"OEY"),
	
	/// TODO.
	OEZ = Self::letters_to_token(b"OEZ"),
	
	/// TODO.
	OFA = Self::letters_to_token(b"OFA"),
	
	/// TODO.
	OFB = Self::letters_to_token(b"OFB"),
	
	/// TODO.
	OFC = Self::letters_to_token(b"OFC"),
	
	/// TODO.
	OFD = Self::letters_to_token(b"OFD"),
	
	/// TODO.
	OFE = Self::letters_to_token(b"OFE"),
	
	/// TODO.
	OFF = Self::letters_to_token(b"OFF"),
	
	/// TODO.
	OFG = Self::letters_to_token(b"OFG"),
	
	/// TODO.
	OFH = Self::letters_to_token(b"OFH"),
	
	/// TODO.
	OFI = Self::letters_to_token(b"OFI"),
	
	/// TODO.
	OFJ = Self::letters_to_token(b"OFJ"),
	
	/// TODO.
	OFK = Self::letters_to_token(b"OFK"),
	
	/// TODO.
	OFL = Self::letters_to_token(b"OFL"),
	
	/// TODO.
	OFM = Self::letters_to_token(b"OFM"),
	
	/// TODO.
	OFN = Self::letters_to_token(b"OFN"),
	
	/// TODO.
	OFO = Self::letters_to_token(b"OFO"),
	
	/// TODO.
	OFP = Self::letters_to_token(b"OFP"),
	
	/// TODO.
	OFQ = Self::letters_to_token(b"OFQ"),
	
	/// TODO.
	OFR = Self::letters_to_token(b"OFR"),
	
	/// TODO.
	OFS = Self::letters_to_token(b"OFS"),
	
	/// TODO.
	OFT = Self::letters_to_token(b"OFT"),
	
	/// TODO.
	OFU = Self::letters_to_token(b"OFU"),
	
	/// TODO.
	OFV = Self::letters_to_token(b"OFV"),
	
	/// TODO.
	OFW = Self::letters_to_token(b"OFW"),
	
	/// TODO.
	OFX = Self::letters_to_token(b"OFX"),
	
	/// TODO.
	OFY = Self::letters_to_token(b"OFY"),
	
	/// TODO.
	OFZ = Self::letters_to_token(b"OFZ"),
	
	/// TODO.
	OGA = Self::letters_to_token(b"OGA"),
	
	/// TODO.
	OGB = Self::letters_to_token(b"OGB"),
	
	/// TODO.
	OGC = Self::letters_to_token(b"OGC"),
	
	/// TODO.
	OGD = Self::letters_to_token(b"OGD"),
	
	/// TODO.
	OGE = Self::letters_to_token(b"OGE"),
	
	/// TODO.
	OGF = Self::letters_to_token(b"OGF"),
	
	/// TODO.
	OGG = Self::letters_to_token(b"OGG"),
	
	/// TODO.
	OGH = Self::letters_to_token(b"OGH"),
	
	/// TODO.
	OGI = Self::letters_to_token(b"OGI"),
	
	/// TODO.
	OGJ = Self::letters_to_token(b"OGJ"),
	
	/// TODO.
	OGK = Self::letters_to_token(b"OGK"),
	
	/// TODO.
	OGL = Self::letters_to_token(b"OGL"),
	
	/// TODO.
	OGM = Self::letters_to_token(b"OGM"),
	
	/// TODO.
	OGN = Self::letters_to_token(b"OGN"),
	
	/// TODO.
	OGO = Self::letters_to_token(b"OGO"),
	
	/// TODO.
	OGP = Self::letters_to_token(b"OGP"),
	
	/// TODO.
	OGQ = Self::letters_to_token(b"OGQ"),
	
	/// TODO.
	OGR = Self::letters_to_token(b"OGR"),
	
	/// TODO.
	OGS = Self::letters_to_token(b"OGS"),
	
	/// TODO.
	OGT = Self::letters_to_token(b"OGT"),
	
	/// TODO.
	OGU = Self::letters_to_token(b"OGU"),
	
	/// TODO.
	OGV = Self::letters_to_token(b"OGV"),
	
	/// TODO.
	OGW = Self::letters_to_token(b"OGW"),
	
	/// TODO.
	OGX = Self::letters_to_token(b"OGX"),
	
	/// TODO.
	OGY = Self::letters_to_token(b"OGY"),
	
	/// TODO.
	OGZ = Self::letters_to_token(b"OGZ"),
	
	/// TODO.
	OHA = Self::letters_to_token(b"OHA"),
	
	/// TODO.
	OHB = Self::letters_to_token(b"OHB"),
	
	/// TODO.
	OHC = Self::letters_to_token(b"OHC"),
	
	/// TODO.
	OHD = Self::letters_to_token(b"OHD"),
	
	/// TODO.
	OHE = Self::letters_to_token(b"OHE"),
	
	/// TODO.
	OHF = Self::letters_to_token(b"OHF"),
	
	/// TODO.
	OHG = Self::letters_to_token(b"OHG"),
	
	/// TODO.
	OHH = Self::letters_to_token(b"OHH"),
	
	/// TODO.
	OHI = Self::letters_to_token(b"OHI"),
	
	/// TODO.
	OHJ = Self::letters_to_token(b"OHJ"),
	
	/// TODO.
	OHK = Self::letters_to_token(b"OHK"),
	
	/// TODO.
	OHL = Self::letters_to_token(b"OHL"),
	
	/// TODO.
	OHM = Self::letters_to_token(b"OHM"),
	
	/// TODO.
	OHN = Self::letters_to_token(b"OHN"),
	
	/// TODO.
	OHO = Self::letters_to_token(b"OHO"),
	
	/// TODO.
	OHP = Self::letters_to_token(b"OHP"),
	
	/// TODO.
	OHQ = Self::letters_to_token(b"OHQ"),
	
	/// TODO.
	OHR = Self::letters_to_token(b"OHR"),
	
	/// TODO.
	OHS = Self::letters_to_token(b"OHS"),
	
	/// TODO.
	OHT = Self::letters_to_token(b"OHT"),
	
	/// TODO.
	OHU = Self::letters_to_token(b"OHU"),
	
	/// TODO.
	OHV = Self::letters_to_token(b"OHV"),
	
	/// TODO.
	OHW = Self::letters_to_token(b"OHW"),
	
	/// TODO.
	OHX = Self::letters_to_token(b"OHX"),
	
	/// TODO.
	OHY = Self::letters_to_token(b"OHY"),
	
	/// TODO.
	OHZ = Self::letters_to_token(b"OHZ"),
	
	/// TODO.
	OIA = Self::letters_to_token(b"OIA"),
	
	/// TODO.
	OIB = Self::letters_to_token(b"OIB"),
	
	/// TODO.
	OIC = Self::letters_to_token(b"OIC"),
	
	/// TODO.
	OID = Self::letters_to_token(b"OID"),
	
	/// TODO.
	OIE = Self::letters_to_token(b"OIE"),
	
	/// TODO.
	OIF = Self::letters_to_token(b"OIF"),
	
	/// TODO.
	OIG = Self::letters_to_token(b"OIG"),
	
	/// TODO.
	OIH = Self::letters_to_token(b"OIH"),
	
	/// TODO.
	OII = Self::letters_to_token(b"OII"),
	
	/// TODO.
	OIJ = Self::letters_to_token(b"OIJ"),
	
	/// TODO.
	OIK = Self::letters_to_token(b"OIK"),
	
	/// TODO.
	OIL = Self::letters_to_token(b"OIL"),
	
	/// TODO.
	OIM = Self::letters_to_token(b"OIM"),
	
	/// TODO.
	OIN = Self::letters_to_token(b"OIN"),
	
	/// TODO.
	OIO = Self::letters_to_token(b"OIO"),
	
	/// TODO.
	OIP = Self::letters_to_token(b"OIP"),
	
	/// TODO.
	OIQ = Self::letters_to_token(b"OIQ"),
	
	/// TODO.
	OIR = Self::letters_to_token(b"OIR"),
	
	/// TODO.
	OIS = Self::letters_to_token(b"OIS"),
	
	/// TODO.
	OIT = Self::letters_to_token(b"OIT"),
	
	/// TODO.
	OIU = Self::letters_to_token(b"OIU"),
	
	/// TODO.
	OIV = Self::letters_to_token(b"OIV"),
	
	/// TODO.
	OIW = Self::letters_to_token(b"OIW"),
	
	/// TODO.
	OIX = Self::letters_to_token(b"OIX"),
	
	/// TODO.
	OIY = Self::letters_to_token(b"OIY"),
	
	/// TODO.
	OIZ = Self::letters_to_token(b"OIZ"),
	
	/// TODO.
	OJA = Self::letters_to_token(b"OJA"),
	
	/// TODO.
	OJB = Self::letters_to_token(b"OJB"),
	
	/// TODO.
	OJC = Self::letters_to_token(b"OJC"),
	
	/// TODO.
	OJD = Self::letters_to_token(b"OJD"),
	
	/// TODO.
	OJE = Self::letters_to_token(b"OJE"),
	
	/// TODO.
	OJF = Self::letters_to_token(b"OJF"),
	
	/// TODO.
	OJG = Self::letters_to_token(b"OJG"),
	
	/// TODO.
	OJH = Self::letters_to_token(b"OJH"),
	
	/// TODO.
	OJI = Self::letters_to_token(b"OJI"),
	
	/// TODO.
	OJJ = Self::letters_to_token(b"OJJ"),
	
	/// TODO.
	OJK = Self::letters_to_token(b"OJK"),
	
	/// TODO.
	OJL = Self::letters_to_token(b"OJL"),
	
	/// TODO.
	OJM = Self::letters_to_token(b"OJM"),
	
	/// TODO.
	OJN = Self::letters_to_token(b"OJN"),
	
	/// TODO.
	OJO = Self::letters_to_token(b"OJO"),
	
	/// TODO.
	OJP = Self::letters_to_token(b"OJP"),
	
	/// TODO.
	OJQ = Self::letters_to_token(b"OJQ"),
	
	/// TODO.
	OJR = Self::letters_to_token(b"OJR"),
	
	/// TODO.
	OJS = Self::letters_to_token(b"OJS"),
	
	/// TODO.
	OJT = Self::letters_to_token(b"OJT"),
	
	/// TODO.
	OJU = Self::letters_to_token(b"OJU"),
	
	/// TODO.
	OJV = Self::letters_to_token(b"OJV"),
	
	/// TODO.
	OJW = Self::letters_to_token(b"OJW"),
	
	/// TODO.
	OJX = Self::letters_to_token(b"OJX"),
	
	/// TODO.
	OJY = Self::letters_to_token(b"OJY"),
	
	/// TODO.
	OJZ = Self::letters_to_token(b"OJZ"),
	
	/// TODO.
	OKA = Self::letters_to_token(b"OKA"),
	
	/// TODO.
	OKB = Self::letters_to_token(b"OKB"),
	
	/// TODO.
	OKC = Self::letters_to_token(b"OKC"),
	
	/// TODO.
	OKD = Self::letters_to_token(b"OKD"),
	
	/// TODO.
	OKE = Self::letters_to_token(b"OKE"),
	
	/// TODO.
	OKF = Self::letters_to_token(b"OKF"),
	
	/// TODO.
	OKG = Self::letters_to_token(b"OKG"),
	
	/// TODO.
	OKH = Self::letters_to_token(b"OKH"),
	
	/// TODO.
	OKI = Self::letters_to_token(b"OKI"),
	
	/// TODO.
	OKJ = Self::letters_to_token(b"OKJ"),
	
	/// TODO.
	OKK = Self::letters_to_token(b"OKK"),
	
	/// TODO.
	OKL = Self::letters_to_token(b"OKL"),
	
	/// TODO.
	OKM = Self::letters_to_token(b"OKM"),
	
	/// TODO.
	OKN = Self::letters_to_token(b"OKN"),
	
	/// TODO.
	OKO = Self::letters_to_token(b"OKO"),
	
	/// TODO.
	OKP = Self::letters_to_token(b"OKP"),
	
	/// TODO.
	OKQ = Self::letters_to_token(b"OKQ"),
	
	/// TODO.
	OKR = Self::letters_to_token(b"OKR"),
	
	/// TODO.
	OKS = Self::letters_to_token(b"OKS"),
	
	/// TODO.
	OKT = Self::letters_to_token(b"OKT"),
	
	/// TODO.
	OKU = Self::letters_to_token(b"OKU"),
	
	/// TODO.
	OKV = Self::letters_to_token(b"OKV"),
	
	/// TODO.
	OKW = Self::letters_to_token(b"OKW"),
	
	/// TODO.
	OKX = Self::letters_to_token(b"OKX"),
	
	/// TODO.
	OKY = Self::letters_to_token(b"OKY"),
	
	/// TODO.
	OKZ = Self::letters_to_token(b"OKZ"),
	
	/// TODO.
	OLA = Self::letters_to_token(b"OLA"),
	
	/// TODO.
	OLB = Self::letters_to_token(b"OLB"),
	
	/// TODO.
	OLC = Self::letters_to_token(b"OLC"),
	
	/// TODO.
	OLD = Self::letters_to_token(b"OLD"),
	
	/// TODO.
	OLE = Self::letters_to_token(b"OLE"),
	
	/// TODO.
	OLF = Self::letters_to_token(b"OLF"),
	
	/// TODO.
	OLG = Self::letters_to_token(b"OLG"),
	
	/// TODO.
	OLH = Self::letters_to_token(b"OLH"),
	
	/// TODO.
	OLI = Self::letters_to_token(b"OLI"),
	
	/// TODO.
	OLJ = Self::letters_to_token(b"OLJ"),
	
	/// TODO.
	OLK = Self::letters_to_token(b"OLK"),
	
	/// TODO.
	OLL = Self::letters_to_token(b"OLL"),
	
	/// TODO.
	OLM = Self::letters_to_token(b"OLM"),
	
	/// TODO.
	OLN = Self::letters_to_token(b"OLN"),
	
	/// TODO.
	OLO = Self::letters_to_token(b"OLO"),
	
	/// TODO.
	OLP = Self::letters_to_token(b"OLP"),
	
	/// TODO.
	OLQ = Self::letters_to_token(b"OLQ"),
	
	/// TODO.
	OLR = Self::letters_to_token(b"OLR"),
	
	/// TODO.
	OLS = Self::letters_to_token(b"OLS"),
	
	/// TODO.
	OLT = Self::letters_to_token(b"OLT"),
	
	/// TODO.
	OLU = Self::letters_to_token(b"OLU"),
	
	/// TODO.
	OLV = Self::letters_to_token(b"OLV"),
	
	/// TODO.
	OLW = Self::letters_to_token(b"OLW"),
	
	/// TODO.
	OLX = Self::letters_to_token(b"OLX"),
	
	/// TODO.
	OLY = Self::letters_to_token(b"OLY"),
	
	/// TODO.
	OLZ = Self::letters_to_token(b"OLZ"),
	
	/// TODO.
	OMA = Self::letters_to_token(b"OMA"),
	
	/// TODO.
	OMB = Self::letters_to_token(b"OMB"),
	
	/// TODO.
	OMC = Self::letters_to_token(b"OMC"),
	
	/// TODO.
	OMD = Self::letters_to_token(b"OMD"),
	
	/// TODO.
	OME = Self::letters_to_token(b"OME"),
	
	/// TODO.
	OMF = Self::letters_to_token(b"OMF"),
	
	/// TODO.
	OMG = Self::letters_to_token(b"OMG"),
	
	/// TODO.
	OMH = Self::letters_to_token(b"OMH"),
	
	/// TODO.
	OMI = Self::letters_to_token(b"OMI"),
	
	/// TODO.
	OMJ = Self::letters_to_token(b"OMJ"),
	
	/// TODO.
	OMK = Self::letters_to_token(b"OMK"),
	
	/// TODO.
	OML = Self::letters_to_token(b"OML"),
	
	/// TODO.
	OMM = Self::letters_to_token(b"OMM"),
	
	/// TODO.
	OMN = Self::letters_to_token(b"OMN"),
	
	/// TODO.
	OMO = Self::letters_to_token(b"OMO"),
	
	/// TODO.
	OMP = Self::letters_to_token(b"OMP"),
	
	/// TODO.
	OMQ = Self::letters_to_token(b"OMQ"),
	
	/// TODO.
	OMR = Self::letters_to_token(b"OMR"),
	
	/// TODO.
	OMS = Self::letters_to_token(b"OMS"),
	
	/// TODO.
	OMT = Self::letters_to_token(b"OMT"),
	
	/// TODO.
	OMU = Self::letters_to_token(b"OMU"),
	
	/// TODO.
	OMV = Self::letters_to_token(b"OMV"),
	
	/// TODO.
	OMW = Self::letters_to_token(b"OMW"),
	
	/// TODO.
	OMX = Self::letters_to_token(b"OMX"),
	
	/// TODO.
	OMY = Self::letters_to_token(b"OMY"),
	
	/// TODO.
	OMZ = Self::letters_to_token(b"OMZ"),
	
	/// TODO.
	ONA = Self::letters_to_token(b"ONA"),
	
	/// TODO.
	ONB = Self::letters_to_token(b"ONB"),
	
	/// TODO.
	ONC = Self::letters_to_token(b"ONC"),
	
	/// TODO.
	OND = Self::letters_to_token(b"OND"),
	
	/// TODO.
	ONE = Self::letters_to_token(b"ONE"),
	
	/// TODO.
	ONF = Self::letters_to_token(b"ONF"),
	
	/// TODO.
	ONG = Self::letters_to_token(b"ONG"),
	
	/// TODO.
	ONH = Self::letters_to_token(b"ONH"),
	
	/// TODO.
	ONI = Self::letters_to_token(b"ONI"),
	
	/// TODO.
	ONJ = Self::letters_to_token(b"ONJ"),
	
	/// TODO.
	ONK = Self::letters_to_token(b"ONK"),
	
	/// TODO.
	ONL = Self::letters_to_token(b"ONL"),
	
	/// TODO.
	ONM = Self::letters_to_token(b"ONM"),
	
	/// TODO.
	ONN = Self::letters_to_token(b"ONN"),
	
	/// TODO.
	ONO = Self::letters_to_token(b"ONO"),
	
	/// TODO.
	ONP = Self::letters_to_token(b"ONP"),
	
	/// TODO.
	ONQ = Self::letters_to_token(b"ONQ"),
	
	/// TODO.
	ONR = Self::letters_to_token(b"ONR"),
	
	/// TODO.
	ONS = Self::letters_to_token(b"ONS"),
	
	/// TODO.
	ONT = Self::letters_to_token(b"ONT"),
	
	/// TODO.
	ONU = Self::letters_to_token(b"ONU"),
	
	/// TODO.
	ONV = Self::letters_to_token(b"ONV"),
	
	/// TODO.
	ONW = Self::letters_to_token(b"ONW"),
	
	/// TODO.
	ONX = Self::letters_to_token(b"ONX"),
	
	/// TODO.
	ONY = Self::letters_to_token(b"ONY"),
	
	/// TODO.
	ONZ = Self::letters_to_token(b"ONZ"),
	
	/// TODO.
	OOA = Self::letters_to_token(b"OOA"),
	
	/// TODO.
	OOB = Self::letters_to_token(b"OOB"),
	
	/// TODO.
	OOC = Self::letters_to_token(b"OOC"),
	
	/// TODO.
	OOD = Self::letters_to_token(b"OOD"),
	
	/// TODO.
	OOE = Self::letters_to_token(b"OOE"),
	
	/// TODO.
	OOF = Self::letters_to_token(b"OOF"),
	
	/// TODO.
	OOG = Self::letters_to_token(b"OOG"),
	
	/// TODO.
	OOH = Self::letters_to_token(b"OOH"),
	
	/// TODO.
	OOI = Self::letters_to_token(b"OOI"),
	
	/// TODO.
	OOJ = Self::letters_to_token(b"OOJ"),
	
	/// TODO.
	OOK = Self::letters_to_token(b"OOK"),
	
	/// TODO.
	OOL = Self::letters_to_token(b"OOL"),
	
	/// TODO.
	OOM = Self::letters_to_token(b"OOM"),
	
	/// TODO.
	OON = Self::letters_to_token(b"OON"),
	
	/// TODO.
	OOO = Self::letters_to_token(b"OOO"),
	
	/// TODO.
	OOP = Self::letters_to_token(b"OOP"),
	
	/// TODO.
	OOQ = Self::letters_to_token(b"OOQ"),
	
	/// TODO.
	OOR = Self::letters_to_token(b"OOR"),
	
	/// TODO.
	OOS = Self::letters_to_token(b"OOS"),
	
	/// TODO.
	OOT = Self::letters_to_token(b"OOT"),
	
	/// TODO.
	OOU = Self::letters_to_token(b"OOU"),
	
	/// TODO.
	OOV = Self::letters_to_token(b"OOV"),
	
	/// TODO.
	OOW = Self::letters_to_token(b"OOW"),
	
	/// TODO.
	OOX = Self::letters_to_token(b"OOX"),
	
	/// TODO.
	OOY = Self::letters_to_token(b"OOY"),
	
	/// TODO.
	OOZ = Self::letters_to_token(b"OOZ"),
	
	/// TODO.
	OPA = Self::letters_to_token(b"OPA"),
	
	/// TODO.
	OPB = Self::letters_to_token(b"OPB"),
	
	/// TODO.
	OPC = Self::letters_to_token(b"OPC"),
	
	/// TODO.
	OPD = Self::letters_to_token(b"OPD"),
	
	/// TODO.
	OPE = Self::letters_to_token(b"OPE"),
	
	/// TODO.
	OPF = Self::letters_to_token(b"OPF"),
	
	/// TODO.
	OPG = Self::letters_to_token(b"OPG"),
	
	/// TODO.
	OPH = Self::letters_to_token(b"OPH"),
	
	/// TODO.
	OPI = Self::letters_to_token(b"OPI"),
	
	/// TODO.
	OPJ = Self::letters_to_token(b"OPJ"),
	
	/// TODO.
	OPK = Self::letters_to_token(b"OPK"),
	
	/// TODO.
	OPL = Self::letters_to_token(b"OPL"),
	
	/// TODO.
	OPM = Self::letters_to_token(b"OPM"),
	
	/// TODO.
	OPN = Self::letters_to_token(b"OPN"),
	
	/// TODO.
	OPO = Self::letters_to_token(b"OPO"),
	
	/// TODO.
	OPP = Self::letters_to_token(b"OPP"),
	
	/// TODO.
	OPQ = Self::letters_to_token(b"OPQ"),
	
	/// TODO.
	OPR = Self::letters_to_token(b"OPR"),
	
	/// TODO.
	OPS = Self::letters_to_token(b"OPS"),
	
	/// TODO.
	OPT = Self::letters_to_token(b"OPT"),
	
	/// TODO.
	OPU = Self::letters_to_token(b"OPU"),
	
	/// TODO.
	OPV = Self::letters_to_token(b"OPV"),
	
	/// TODO.
	OPW = Self::letters_to_token(b"OPW"),
	
	/// TODO.
	OPX = Self::letters_to_token(b"OPX"),
	
	/// TODO.
	OPY = Self::letters_to_token(b"OPY"),
	
	/// TODO.
	OPZ = Self::letters_to_token(b"OPZ"),
	
	/// TODO.
	OQA = Self::letters_to_token(b"OQA"),
	
	/// TODO.
	OQB = Self::letters_to_token(b"OQB"),
	
	/// TODO.
	OQC = Self::letters_to_token(b"OQC"),
	
	/// TODO.
	OQD = Self::letters_to_token(b"OQD"),
	
	/// TODO.
	OQE = Self::letters_to_token(b"OQE"),
	
	/// TODO.
	OQF = Self::letters_to_token(b"OQF"),
	
	/// TODO.
	OQG = Self::letters_to_token(b"OQG"),
	
	/// TODO.
	OQH = Self::letters_to_token(b"OQH"),
	
	/// TODO.
	OQI = Self::letters_to_token(b"OQI"),
	
	/// TODO.
	OQJ = Self::letters_to_token(b"OQJ"),
	
	/// TODO.
	OQK = Self::letters_to_token(b"OQK"),
	
	/// TODO.
	OQL = Self::letters_to_token(b"OQL"),
	
	/// TODO.
	OQM = Self::letters_to_token(b"OQM"),
	
	/// TODO.
	OQN = Self::letters_to_token(b"OQN"),
	
	/// TODO.
	OQO = Self::letters_to_token(b"OQO"),
	
	/// TODO.
	OQP = Self::letters_to_token(b"OQP"),
	
	/// TODO.
	OQQ = Self::letters_to_token(b"OQQ"),
	
	/// TODO.
	OQR = Self::letters_to_token(b"OQR"),
	
	/// TODO.
	OQS = Self::letters_to_token(b"OQS"),
	
	/// TODO.
	OQT = Self::letters_to_token(b"OQT"),
	
	/// TODO.
	OQU = Self::letters_to_token(b"OQU"),
	
	/// TODO.
	OQV = Self::letters_to_token(b"OQV"),
	
	/// TODO.
	OQW = Self::letters_to_token(b"OQW"),
	
	/// TODO.
	OQX = Self::letters_to_token(b"OQX"),
	
	/// TODO.
	OQY = Self::letters_to_token(b"OQY"),
	
	/// TODO.
	OQZ = Self::letters_to_token(b"OQZ"),
	
	/// TODO.
	ORA = Self::letters_to_token(b"ORA"),
	
	/// TODO.
	ORB = Self::letters_to_token(b"ORB"),
	
	/// TODO.
	ORC = Self::letters_to_token(b"ORC"),
	
	/// TODO.
	ORD = Self::letters_to_token(b"ORD"),
	
	/// TODO.
	ORE = Self::letters_to_token(b"ORE"),
	
	/// TODO.
	ORF = Self::letters_to_token(b"ORF"),
	
	/// TODO.
	ORG = Self::letters_to_token(b"ORG"),
	
	/// TODO.
	ORH = Self::letters_to_token(b"ORH"),
	
	/// TODO.
	ORI = Self::letters_to_token(b"ORI"),
	
	/// TODO.
	ORJ = Self::letters_to_token(b"ORJ"),
	
	/// TODO.
	ORK = Self::letters_to_token(b"ORK"),
	
	/// TODO.
	ORL = Self::letters_to_token(b"ORL"),
	
	/// TODO.
	ORM = Self::letters_to_token(b"ORM"),
	
	/// TODO.
	ORN = Self::letters_to_token(b"ORN"),
	
	/// TODO.
	ORO = Self::letters_to_token(b"ORO"),
	
	/// TODO.
	ORP = Self::letters_to_token(b"ORP"),
	
	/// TODO.
	ORQ = Self::letters_to_token(b"ORQ"),
	
	/// TODO.
	ORR = Self::letters_to_token(b"ORR"),
	
	/// TODO.
	ORS = Self::letters_to_token(b"ORS"),
	
	/// TODO.
	ORT = Self::letters_to_token(b"ORT"),
	
	/// TODO.
	ORU = Self::letters_to_token(b"ORU"),
	
	/// TODO.
	ORV = Self::letters_to_token(b"ORV"),
	
	/// TODO.
	ORW = Self::letters_to_token(b"ORW"),
	
	/// TODO.
	ORX = Self::letters_to_token(b"ORX"),
	
	/// TODO.
	ORY = Self::letters_to_token(b"ORY"),
	
	/// TODO.
	ORZ = Self::letters_to_token(b"ORZ"),
	
	/// TODO.
	OSA = Self::letters_to_token(b"OSA"),
	
	/// TODO.
	OSB = Self::letters_to_token(b"OSB"),
	
	/// TODO.
	OSC = Self::letters_to_token(b"OSC"),
	
	/// TODO.
	OSD = Self::letters_to_token(b"OSD"),
	
	/// TODO.
	OSE = Self::letters_to_token(b"OSE"),
	
	/// TODO.
	OSF = Self::letters_to_token(b"OSF"),
	
	/// TODO.
	OSG = Self::letters_to_token(b"OSG"),
	
	/// TODO.
	OSH = Self::letters_to_token(b"OSH"),
	
	/// TODO.
	OSI = Self::letters_to_token(b"OSI"),
	
	/// TODO.
	OSJ = Self::letters_to_token(b"OSJ"),
	
	/// TODO.
	OSK = Self::letters_to_token(b"OSK"),
	
	/// TODO.
	OSL = Self::letters_to_token(b"OSL"),
	
	/// TODO.
	OSM = Self::letters_to_token(b"OSM"),
	
	/// TODO.
	OSN = Self::letters_to_token(b"OSN"),
	
	/// TODO.
	OSO = Self::letters_to_token(b"OSO"),
	
	/// TODO.
	OSP = Self::letters_to_token(b"OSP"),
	
	/// TODO.
	OSQ = Self::letters_to_token(b"OSQ"),
	
	/// TODO.
	OSR = Self::letters_to_token(b"OSR"),
	
	/// TODO.
	OSS = Self::letters_to_token(b"OSS"),
	
	/// TODO.
	OST = Self::letters_to_token(b"OST"),
	
	/// TODO.
	OSU = Self::letters_to_token(b"OSU"),
	
	/// TODO.
	OSV = Self::letters_to_token(b"OSV"),
	
	/// TODO.
	OSW = Self::letters_to_token(b"OSW"),
	
	/// TODO.
	OSX = Self::letters_to_token(b"OSX"),
	
	/// TODO.
	OSY = Self::letters_to_token(b"OSY"),
	
	/// TODO.
	OSZ = Self::letters_to_token(b"OSZ"),
	
	/// TODO.
	OTA = Self::letters_to_token(b"OTA"),
	
	/// TODO.
	OTB = Self::letters_to_token(b"OTB"),
	
	/// TODO.
	OTC = Self::letters_to_token(b"OTC"),
	
	/// TODO.
	OTD = Self::letters_to_token(b"OTD"),
	
	/// TODO.
	OTE = Self::letters_to_token(b"OTE"),
	
	/// TODO.
	OTF = Self::letters_to_token(b"OTF"),
	
	/// TODO.
	OTG = Self::letters_to_token(b"OTG"),
	
	/// TODO.
	OTH = Self::letters_to_token(b"OTH"),
	
	/// TODO.
	OTI = Self::letters_to_token(b"OTI"),
	
	/// TODO.
	OTJ = Self::letters_to_token(b"OTJ"),
	
	/// TODO.
	OTK = Self::letters_to_token(b"OTK"),
	
	/// TODO.
	OTL = Self::letters_to_token(b"OTL"),
	
	/// TODO.
	OTM = Self::letters_to_token(b"OTM"),
	
	/// TODO.
	OTN = Self::letters_to_token(b"OTN"),
	
	/// TODO.
	OTO = Self::letters_to_token(b"OTO"),
	
	/// TODO.
	OTP = Self::letters_to_token(b"OTP"),
	
	/// TODO.
	OTQ = Self::letters_to_token(b"OTQ"),
	
	/// TODO.
	OTR = Self::letters_to_token(b"OTR"),
	
	/// TODO.
	OTS = Self::letters_to_token(b"OTS"),
	
	/// TODO.
	OTT = Self::letters_to_token(b"OTT"),
	
	/// TODO.
	OTU = Self::letters_to_token(b"OTU"),
	
	/// TODO.
	OTV = Self::letters_to_token(b"OTV"),
	
	/// TODO.
	OTW = Self::letters_to_token(b"OTW"),
	
	/// TODO.
	OTX = Self::letters_to_token(b"OTX"),
	
	/// TODO.
	OTY = Self::letters_to_token(b"OTY"),
	
	/// TODO.
	OTZ = Self::letters_to_token(b"OTZ"),
	
	/// TODO.
	OUA = Self::letters_to_token(b"OUA"),
	
	/// TODO.
	OUB = Self::letters_to_token(b"OUB"),
	
	/// TODO.
	OUC = Self::letters_to_token(b"OUC"),
	
	/// TODO.
	OUD = Self::letters_to_token(b"OUD"),
	
	/// TODO.
	OUE = Self::letters_to_token(b"OUE"),
	
	/// TODO.
	OUF = Self::letters_to_token(b"OUF"),
	
	/// TODO.
	OUG = Self::letters_to_token(b"OUG"),
	
	/// TODO.
	OUH = Self::letters_to_token(b"OUH"),
	
	/// TODO.
	OUI = Self::letters_to_token(b"OUI"),
	
	/// TODO.
	OUJ = Self::letters_to_token(b"OUJ"),
	
	/// TODO.
	OUK = Self::letters_to_token(b"OUK"),
	
	/// TODO.
	OUL = Self::letters_to_token(b"OUL"),
	
	/// TODO.
	OUM = Self::letters_to_token(b"OUM"),
	
	/// TODO.
	OUN = Self::letters_to_token(b"OUN"),
	
	/// TODO.
	OUO = Self::letters_to_token(b"OUO"),
	
	/// TODO.
	OUP = Self::letters_to_token(b"OUP"),
	
	/// TODO.
	OUQ = Self::letters_to_token(b"OUQ"),
	
	/// TODO.
	OUR = Self::letters_to_token(b"OUR"),
	
	/// TODO.
	OUS = Self::letters_to_token(b"OUS"),
	
	/// TODO.
	OUT = Self::letters_to_token(b"OUT"),
	
	/// TODO.
	OUU = Self::letters_to_token(b"OUU"),
	
	/// TODO.
	OUV = Self::letters_to_token(b"OUV"),
	
	/// TODO.
	OUW = Self::letters_to_token(b"OUW"),
	
	/// TODO.
	OUX = Self::letters_to_token(b"OUX"),
	
	/// TODO.
	OUY = Self::letters_to_token(b"OUY"),
	
	/// TODO.
	OUZ = Self::letters_to_token(b"OUZ"),
	
	/// TODO.
	OVA = Self::letters_to_token(b"OVA"),
	
	/// TODO.
	OVB = Self::letters_to_token(b"OVB"),
	
	/// TODO.
	OVC = Self::letters_to_token(b"OVC"),
	
	/// TODO.
	OVD = Self::letters_to_token(b"OVD"),
	
	/// TODO.
	OVE = Self::letters_to_token(b"OVE"),
	
	/// TODO.
	OVF = Self::letters_to_token(b"OVF"),
	
	/// TODO.
	OVG = Self::letters_to_token(b"OVG"),
	
	/// TODO.
	OVH = Self::letters_to_token(b"OVH"),
	
	/// TODO.
	OVI = Self::letters_to_token(b"OVI"),
	
	/// TODO.
	OVJ = Self::letters_to_token(b"OVJ"),
	
	/// TODO.
	OVK = Self::letters_to_token(b"OVK"),
	
	/// TODO.
	OVL = Self::letters_to_token(b"OVL"),
	
	/// TODO.
	OVM = Self::letters_to_token(b"OVM"),
	
	/// TODO.
	OVN = Self::letters_to_token(b"OVN"),
	
	/// TODO.
	OVO = Self::letters_to_token(b"OVO"),
	
	/// TODO.
	OVP = Self::letters_to_token(b"OVP"),
	
	/// TODO.
	OVQ = Self::letters_to_token(b"OVQ"),
	
	/// TODO.
	OVR = Self::letters_to_token(b"OVR"),
	
	/// TODO.
	OVS = Self::letters_to_token(b"OVS"),
	
	/// TODO.
	OVT = Self::letters_to_token(b"OVT"),
	
	/// TODO.
	OVU = Self::letters_to_token(b"OVU"),
	
	/// TODO.
	OVV = Self::letters_to_token(b"OVV"),
	
	/// TODO.
	OVW = Self::letters_to_token(b"OVW"),
	
	/// TODO.
	OVX = Self::letters_to_token(b"OVX"),
	
	/// TODO.
	OVY = Self::letters_to_token(b"OVY"),
	
	/// TODO.
	OVZ = Self::letters_to_token(b"OVZ"),
	
	/// TODO.
	OWA = Self::letters_to_token(b"OWA"),
	
	/// TODO.
	OWB = Self::letters_to_token(b"OWB"),
	
	/// TODO.
	OWC = Self::letters_to_token(b"OWC"),
	
	/// TODO.
	OWD = Self::letters_to_token(b"OWD"),
	
	/// TODO.
	OWE = Self::letters_to_token(b"OWE"),
	
	/// TODO.
	OWF = Self::letters_to_token(b"OWF"),
	
	/// TODO.
	OWG = Self::letters_to_token(b"OWG"),
	
	/// TODO.
	OWH = Self::letters_to_token(b"OWH"),
	
	/// TODO.
	OWI = Self::letters_to_token(b"OWI"),
	
	/// TODO.
	OWJ = Self::letters_to_token(b"OWJ"),
	
	/// TODO.
	OWK = Self::letters_to_token(b"OWK"),
	
	/// TODO.
	OWL = Self::letters_to_token(b"OWL"),
	
	/// TODO.
	OWM = Self::letters_to_token(b"OWM"),
	
	/// TODO.
	OWN = Self::letters_to_token(b"OWN"),
	
	/// TODO.
	OWO = Self::letters_to_token(b"OWO"),
	
	/// TODO.
	OWP = Self::letters_to_token(b"OWP"),
	
	/// TODO.
	OWQ = Self::letters_to_token(b"OWQ"),
	
	/// TODO.
	OWR = Self::letters_to_token(b"OWR"),
	
	/// TODO.
	OWS = Self::letters_to_token(b"OWS"),
	
	/// TODO.
	OWT = Self::letters_to_token(b"OWT"),
	
	/// TODO.
	OWU = Self::letters_to_token(b"OWU"),
	
	/// TODO.
	OWV = Self::letters_to_token(b"OWV"),
	
	/// TODO.
	OWW = Self::letters_to_token(b"OWW"),
	
	/// TODO.
	OWX = Self::letters_to_token(b"OWX"),
	
	/// TODO.
	OWY = Self::letters_to_token(b"OWY"),
	
	/// TODO.
	OWZ = Self::letters_to_token(b"OWZ"),
	
	/// TODO.
	OXA = Self::letters_to_token(b"OXA"),
	
	/// TODO.
	OXB = Self::letters_to_token(b"OXB"),
	
	/// TODO.
	OXC = Self::letters_to_token(b"OXC"),
	
	/// TODO.
	OXD = Self::letters_to_token(b"OXD"),
	
	/// TODO.
	OXE = Self::letters_to_token(b"OXE"),
	
	/// TODO.
	OXF = Self::letters_to_token(b"OXF"),
	
	/// TODO.
	OXG = Self::letters_to_token(b"OXG"),
	
	/// TODO.
	OXH = Self::letters_to_token(b"OXH"),
	
	/// TODO.
	OXI = Self::letters_to_token(b"OXI"),
	
	/// TODO.
	OXJ = Self::letters_to_token(b"OXJ"),
	
	/// TODO.
	OXK = Self::letters_to_token(b"OXK"),
	
	/// TODO.
	OXL = Self::letters_to_token(b"OXL"),
	
	/// TODO.
	OXM = Self::letters_to_token(b"OXM"),
	
	/// TODO.
	OXN = Self::letters_to_token(b"OXN"),
	
	/// TODO.
	OXO = Self::letters_to_token(b"OXO"),
	
	/// TODO.
	OXP = Self::letters_to_token(b"OXP"),
	
	/// TODO.
	OXQ = Self::letters_to_token(b"OXQ"),
	
	/// TODO.
	OXR = Self::letters_to_token(b"OXR"),
	
	/// TODO.
	OXS = Self::letters_to_token(b"OXS"),
	
	/// TODO.
	OXT = Self::letters_to_token(b"OXT"),
	
	/// TODO.
	OXU = Self::letters_to_token(b"OXU"),
	
	/// TODO.
	OXV = Self::letters_to_token(b"OXV"),
	
	/// TODO.
	OXW = Self::letters_to_token(b"OXW"),
	
	/// TODO.
	OXX = Self::letters_to_token(b"OXX"),
	
	/// TODO.
	OXY = Self::letters_to_token(b"OXY"),
	
	/// TODO.
	OXZ = Self::letters_to_token(b"OXZ"),
	
	/// TODO.
	OYA = Self::letters_to_token(b"OYA"),
	
	/// TODO.
	OYB = Self::letters_to_token(b"OYB"),
	
	/// TODO.
	OYC = Self::letters_to_token(b"OYC"),
	
	/// TODO.
	OYD = Self::letters_to_token(b"OYD"),
	
	/// TODO.
	OYE = Self::letters_to_token(b"OYE"),
	
	/// TODO.
	OYF = Self::letters_to_token(b"OYF"),
	
	/// TODO.
	OYG = Self::letters_to_token(b"OYG"),
	
	/// TODO.
	OYH = Self::letters_to_token(b"OYH"),
	
	/// TODO.
	OYI = Self::letters_to_token(b"OYI"),
	
	/// TODO.
	OYJ = Self::letters_to_token(b"OYJ"),
	
	/// TODO.
	OYK = Self::letters_to_token(b"OYK"),
	
	/// TODO.
	OYL = Self::letters_to_token(b"OYL"),
	
	/// TODO.
	OYM = Self::letters_to_token(b"OYM"),
	
	/// TODO.
	OYN = Self::letters_to_token(b"OYN"),
	
	/// TODO.
	OYO = Self::letters_to_token(b"OYO"),
	
	/// TODO.
	OYP = Self::letters_to_token(b"OYP"),
	
	/// TODO.
	OYQ = Self::letters_to_token(b"OYQ"),
	
	/// TODO.
	OYR = Self::letters_to_token(b"OYR"),
	
	/// TODO.
	OYS = Self::letters_to_token(b"OYS"),
	
	/// TODO.
	OYT = Self::letters_to_token(b"OYT"),
	
	/// TODO.
	OYU = Self::letters_to_token(b"OYU"),
	
	/// TODO.
	OYV = Self::letters_to_token(b"OYV"),
	
	/// TODO.
	OYW = Self::letters_to_token(b"OYW"),
	
	/// TODO.
	OYX = Self::letters_to_token(b"OYX"),
	
	/// TODO.
	OYY = Self::letters_to_token(b"OYY"),
	
	/// TODO.
	OYZ = Self::letters_to_token(b"OYZ"),
	
	/// TODO.
	OZA = Self::letters_to_token(b"OZA"),
	
	/// TODO.
	OZB = Self::letters_to_token(b"OZB"),
	
	/// TODO.
	OZC = Self::letters_to_token(b"OZC"),
	
	/// TODO.
	OZD = Self::letters_to_token(b"OZD"),
	
	/// TODO.
	OZE = Self::letters_to_token(b"OZE"),
	
	/// TODO.
	OZF = Self::letters_to_token(b"OZF"),
	
	/// TODO.
	OZG = Self::letters_to_token(b"OZG"),
	
	/// TODO.
	OZH = Self::letters_to_token(b"OZH"),
	
	/// TODO.
	OZI = Self::letters_to_token(b"OZI"),
	
	/// TODO.
	OZJ = Self::letters_to_token(b"OZJ"),
	
	/// TODO.
	OZK = Self::letters_to_token(b"OZK"),
	
	/// TODO.
	OZL = Self::letters_to_token(b"OZL"),
	
	/// TODO.
	OZM = Self::letters_to_token(b"OZM"),
	
	/// TODO.
	OZN = Self::letters_to_token(b"OZN"),
	
	/// TODO.
	OZO = Self::letters_to_token(b"OZO"),
	
	/// TODO.
	OZP = Self::letters_to_token(b"OZP"),
	
	/// TODO.
	OZQ = Self::letters_to_token(b"OZQ"),
	
	/// TODO.
	OZR = Self::letters_to_token(b"OZR"),
	
	/// TODO.
	OZS = Self::letters_to_token(b"OZS"),
	
	/// TODO.
	OZT = Self::letters_to_token(b"OZT"),
	
	/// TODO.
	OZU = Self::letters_to_token(b"OZU"),
	
	/// TODO.
	OZV = Self::letters_to_token(b"OZV"),
	
	/// TODO.
	OZW = Self::letters_to_token(b"OZW"),
	
	/// TODO.
	OZX = Self::letters_to_token(b"OZX"),
	
	/// TODO.
	OZY = Self::letters_to_token(b"OZY"),
	
	/// TODO.
	OZZ = Self::letters_to_token(b"OZZ"),
	
	/// TODO.
	PAA = Self::letters_to_token(b"PAA"),
	
	/// TODO.
	PAB = Self::letters_to_token(b"PAB"),
	
	/// TODO.
	PAC = Self::letters_to_token(b"PAC"),
	
	/// TODO.
	PAD = Self::letters_to_token(b"PAD"),
	
	/// TODO.
	PAE = Self::letters_to_token(b"PAE"),
	
	/// TODO.
	PAF = Self::letters_to_token(b"PAF"),
	
	/// TODO.
	PAG = Self::letters_to_token(b"PAG"),
	
	/// TODO.
	PAH = Self::letters_to_token(b"PAH"),
	
	/// TODO.
	PAI = Self::letters_to_token(b"PAI"),
	
	/// TODO.
	PAJ = Self::letters_to_token(b"PAJ"),
	
	/// TODO.
	PAK = Self::letters_to_token(b"PAK"),
	
	/// TODO.
	PAL = Self::letters_to_token(b"PAL"),
	
	/// TODO.
	PAM = Self::letters_to_token(b"PAM"),
	
	/// TODO.
	PAN = Self::letters_to_token(b"PAN"),
	
	/// TODO.
	PAO = Self::letters_to_token(b"PAO"),
	
	/// TODO.
	PAP = Self::letters_to_token(b"PAP"),
	
	/// TODO.
	PAQ = Self::letters_to_token(b"PAQ"),
	
	/// TODO.
	PAR = Self::letters_to_token(b"PAR"),
	
	/// TODO.
	PAS = Self::letters_to_token(b"PAS"),
	
	/// TODO.
	PAT = Self::letters_to_token(b"PAT"),
	
	/// TODO.
	PAU = Self::letters_to_token(b"PAU"),
	
	/// TODO.
	PAV = Self::letters_to_token(b"PAV"),
	
	/// TODO.
	PAW = Self::letters_to_token(b"PAW"),
	
	/// TODO.
	PAX = Self::letters_to_token(b"PAX"),
	
	/// TODO.
	PAY = Self::letters_to_token(b"PAY"),
	
	/// TODO.
	PAZ = Self::letters_to_token(b"PAZ"),
	
	/// TODO.
	PBA = Self::letters_to_token(b"PBA"),
	
	/// TODO.
	PBB = Self::letters_to_token(b"PBB"),
	
	/// TODO.
	PBC = Self::letters_to_token(b"PBC"),
	
	/// TODO.
	PBD = Self::letters_to_token(b"PBD"),
	
	/// TODO.
	PBE = Self::letters_to_token(b"PBE"),
	
	/// TODO.
	PBF = Self::letters_to_token(b"PBF"),
	
	/// TODO.
	PBG = Self::letters_to_token(b"PBG"),
	
	/// TODO.
	PBH = Self::letters_to_token(b"PBH"),
	
	/// TODO.
	PBI = Self::letters_to_token(b"PBI"),
	
	/// TODO.
	PBJ = Self::letters_to_token(b"PBJ"),
	
	/// TODO.
	PBK = Self::letters_to_token(b"PBK"),
	
	/// TODO.
	PBL = Self::letters_to_token(b"PBL"),
	
	/// TODO.
	PBM = Self::letters_to_token(b"PBM"),
	
	/// TODO.
	PBN = Self::letters_to_token(b"PBN"),
	
	/// TODO.
	PBO = Self::letters_to_token(b"PBO"),
	
	/// TODO.
	PBP = Self::letters_to_token(b"PBP"),
	
	/// TODO.
	PBQ = Self::letters_to_token(b"PBQ"),
	
	/// TODO.
	PBR = Self::letters_to_token(b"PBR"),
	
	/// TODO.
	PBS = Self::letters_to_token(b"PBS"),
	
	/// TODO.
	PBT = Self::letters_to_token(b"PBT"),
	
	/// TODO.
	PBU = Self::letters_to_token(b"PBU"),
	
	/// TODO.
	PBV = Self::letters_to_token(b"PBV"),
	
	/// TODO.
	PBW = Self::letters_to_token(b"PBW"),
	
	/// TODO.
	PBX = Self::letters_to_token(b"PBX"),
	
	/// TODO.
	PBY = Self::letters_to_token(b"PBY"),
	
	/// TODO.
	PBZ = Self::letters_to_token(b"PBZ"),
	
	/// TODO.
	PCA = Self::letters_to_token(b"PCA"),
	
	/// TODO.
	PCB = Self::letters_to_token(b"PCB"),
	
	/// TODO.
	PCC = Self::letters_to_token(b"PCC"),
	
	/// TODO.
	PCD = Self::letters_to_token(b"PCD"),
	
	/// TODO.
	PCE = Self::letters_to_token(b"PCE"),
	
	/// TODO.
	PCF = Self::letters_to_token(b"PCF"),
	
	/// TODO.
	PCG = Self::letters_to_token(b"PCG"),
	
	/// TODO.
	PCH = Self::letters_to_token(b"PCH"),
	
	/// TODO.
	PCI = Self::letters_to_token(b"PCI"),
	
	/// TODO.
	PCJ = Self::letters_to_token(b"PCJ"),
	
	/// TODO.
	PCK = Self::letters_to_token(b"PCK"),
	
	/// TODO.
	PCL = Self::letters_to_token(b"PCL"),
	
	/// TODO.
	PCM = Self::letters_to_token(b"PCM"),
	
	/// TODO.
	PCN = Self::letters_to_token(b"PCN"),
	
	/// TODO.
	PCO = Self::letters_to_token(b"PCO"),
	
	/// TODO.
	PCP = Self::letters_to_token(b"PCP"),
	
	/// TODO.
	PCQ = Self::letters_to_token(b"PCQ"),
	
	/// TODO.
	PCR = Self::letters_to_token(b"PCR"),
	
	/// TODO.
	PCS = Self::letters_to_token(b"PCS"),
	
	/// TODO.
	PCT = Self::letters_to_token(b"PCT"),
	
	/// TODO.
	PCU = Self::letters_to_token(b"PCU"),
	
	/// TODO.
	PCV = Self::letters_to_token(b"PCV"),
	
	/// TODO.
	PCW = Self::letters_to_token(b"PCW"),
	
	/// TODO.
	PCX = Self::letters_to_token(b"PCX"),
	
	/// TODO.
	PCY = Self::letters_to_token(b"PCY"),
	
	/// TODO.
	PCZ = Self::letters_to_token(b"PCZ"),
	
	/// TODO.
	PDA = Self::letters_to_token(b"PDA"),
	
	/// TODO.
	PDB = Self::letters_to_token(b"PDB"),
	
	/// TODO.
	PDC = Self::letters_to_token(b"PDC"),
	
	/// TODO.
	PDD = Self::letters_to_token(b"PDD"),
	
	/// TODO.
	PDE = Self::letters_to_token(b"PDE"),
	
	/// TODO.
	PDF = Self::letters_to_token(b"PDF"),
	
	/// TODO.
	PDG = Self::letters_to_token(b"PDG"),
	
	/// TODO.
	PDH = Self::letters_to_token(b"PDH"),
	
	/// TODO.
	PDI = Self::letters_to_token(b"PDI"),
	
	/// TODO.
	PDJ = Self::letters_to_token(b"PDJ"),
	
	/// TODO.
	PDK = Self::letters_to_token(b"PDK"),
	
	/// TODO.
	PDL = Self::letters_to_token(b"PDL"),
	
	/// TODO.
	PDM = Self::letters_to_token(b"PDM"),
	
	/// TODO.
	PDN = Self::letters_to_token(b"PDN"),
	
	/// TODO.
	PDO = Self::letters_to_token(b"PDO"),
	
	/// TODO.
	PDP = Self::letters_to_token(b"PDP"),
	
	/// TODO.
	PDQ = Self::letters_to_token(b"PDQ"),
	
	/// TODO.
	PDR = Self::letters_to_token(b"PDR"),
	
	/// TODO.
	PDS = Self::letters_to_token(b"PDS"),
	
	/// TODO.
	PDT = Self::letters_to_token(b"PDT"),
	
	/// TODO.
	PDU = Self::letters_to_token(b"PDU"),
	
	/// TODO.
	PDV = Self::letters_to_token(b"PDV"),
	
	/// TODO.
	PDW = Self::letters_to_token(b"PDW"),
	
	/// TODO.
	PDX = Self::letters_to_token(b"PDX"),
	
	/// TODO.
	PDY = Self::letters_to_token(b"PDY"),
	
	/// TODO.
	PDZ = Self::letters_to_token(b"PDZ"),
	
	/// TODO.
	PEA = Self::letters_to_token(b"PEA"),
	
	/// TODO.
	PEB = Self::letters_to_token(b"PEB"),
	
	/// TODO.
	PEC = Self::letters_to_token(b"PEC"),
	
	/// TODO.
	PED = Self::letters_to_token(b"PED"),
	
	/// TODO.
	PEE = Self::letters_to_token(b"PEE"),
	
	/// TODO.
	PEF = Self::letters_to_token(b"PEF"),
	
	/// TODO.
	PEG = Self::letters_to_token(b"PEG"),
	
	/// TODO.
	PEH = Self::letters_to_token(b"PEH"),
	
	/// TODO.
	PEI = Self::letters_to_token(b"PEI"),
	
	/// TODO.
	PEJ = Self::letters_to_token(b"PEJ"),
	
	/// TODO.
	PEK = Self::letters_to_token(b"PEK"),
	
	/// TODO.
	PEL = Self::letters_to_token(b"PEL"),
	
	/// TODO.
	PEM = Self::letters_to_token(b"PEM"),
	
	/// TODO.
	PEN = Self::letters_to_token(b"PEN"),
	
	/// TODO.
	PEO = Self::letters_to_token(b"PEO"),
	
	/// TODO.
	PEP = Self::letters_to_token(b"PEP"),
	
	/// TODO.
	PEQ = Self::letters_to_token(b"PEQ"),
	
	/// TODO.
	PER = Self::letters_to_token(b"PER"),
	
	/// TODO.
	PES = Self::letters_to_token(b"PES"),
	
	/// TODO.
	PET = Self::letters_to_token(b"PET"),
	
	/// TODO.
	PEU = Self::letters_to_token(b"PEU"),
	
	/// TODO.
	PEV = Self::letters_to_token(b"PEV"),
	
	/// TODO.
	PEW = Self::letters_to_token(b"PEW"),
	
	/// TODO.
	PEX = Self::letters_to_token(b"PEX"),
	
	/// TODO.
	PEY = Self::letters_to_token(b"PEY"),
	
	/// TODO.
	PEZ = Self::letters_to_token(b"PEZ"),
	
	/// TODO.
	PFA = Self::letters_to_token(b"PFA"),
	
	/// TODO.
	PFB = Self::letters_to_token(b"PFB"),
	
	/// TODO.
	PFC = Self::letters_to_token(b"PFC"),
	
	/// TODO.
	PFD = Self::letters_to_token(b"PFD"),
	
	/// TODO.
	PFE = Self::letters_to_token(b"PFE"),
	
	/// TODO.
	PFF = Self::letters_to_token(b"PFF"),
	
	/// TODO.
	PFG = Self::letters_to_token(b"PFG"),
	
	/// TODO.
	PFH = Self::letters_to_token(b"PFH"),
	
	/// TODO.
	PFI = Self::letters_to_token(b"PFI"),
	
	/// TODO.
	PFJ = Self::letters_to_token(b"PFJ"),
	
	/// TODO.
	PFK = Self::letters_to_token(b"PFK"),
	
	/// TODO.
	PFL = Self::letters_to_token(b"PFL"),
	
	/// TODO.
	PFM = Self::letters_to_token(b"PFM"),
	
	/// TODO.
	PFN = Self::letters_to_token(b"PFN"),
	
	/// TODO.
	PFO = Self::letters_to_token(b"PFO"),
	
	/// TODO.
	PFP = Self::letters_to_token(b"PFP"),
	
	/// TODO.
	PFQ = Self::letters_to_token(b"PFQ"),
	
	/// TODO.
	PFR = Self::letters_to_token(b"PFR"),
	
	/// TODO.
	PFS = Self::letters_to_token(b"PFS"),
	
	/// TODO.
	PFT = Self::letters_to_token(b"PFT"),
	
	/// TODO.
	PFU = Self::letters_to_token(b"PFU"),
	
	/// TODO.
	PFV = Self::letters_to_token(b"PFV"),
	
	/// TODO.
	PFW = Self::letters_to_token(b"PFW"),
	
	/// TODO.
	PFX = Self::letters_to_token(b"PFX"),
	
	/// TODO.
	PFY = Self::letters_to_token(b"PFY"),
	
	/// TODO.
	PFZ = Self::letters_to_token(b"PFZ"),
	
	/// TODO.
	PGA = Self::letters_to_token(b"PGA"),
	
	/// TODO.
	PGB = Self::letters_to_token(b"PGB"),
	
	/// TODO.
	PGC = Self::letters_to_token(b"PGC"),
	
	/// TODO.
	PGD = Self::letters_to_token(b"PGD"),
	
	/// TODO.
	PGE = Self::letters_to_token(b"PGE"),
	
	/// TODO.
	PGF = Self::letters_to_token(b"PGF"),
	
	/// TODO.
	PGG = Self::letters_to_token(b"PGG"),
	
	/// TODO.
	PGH = Self::letters_to_token(b"PGH"),
	
	/// TODO.
	PGI = Self::letters_to_token(b"PGI"),
	
	/// TODO.
	PGJ = Self::letters_to_token(b"PGJ"),
	
	/// TODO.
	PGK = Self::letters_to_token(b"PGK"),
	
	/// TODO.
	PGL = Self::letters_to_token(b"PGL"),
	
	/// TODO.
	PGM = Self::letters_to_token(b"PGM"),
	
	/// TODO.
	PGN = Self::letters_to_token(b"PGN"),
	
	/// TODO.
	PGO = Self::letters_to_token(b"PGO"),
	
	/// TODO.
	PGP = Self::letters_to_token(b"PGP"),
	
	/// TODO.
	PGQ = Self::letters_to_token(b"PGQ"),
	
	/// TODO.
	PGR = Self::letters_to_token(b"PGR"),
	
	/// TODO.
	PGS = Self::letters_to_token(b"PGS"),
	
	/// TODO.
	PGT = Self::letters_to_token(b"PGT"),
	
	/// TODO.
	PGU = Self::letters_to_token(b"PGU"),
	
	/// TODO.
	PGV = Self::letters_to_token(b"PGV"),
	
	/// TODO.
	PGW = Self::letters_to_token(b"PGW"),
	
	/// TODO.
	PGX = Self::letters_to_token(b"PGX"),
	
	/// TODO.
	PGY = Self::letters_to_token(b"PGY"),
	
	/// TODO.
	PGZ = Self::letters_to_token(b"PGZ"),
	
	/// TODO.
	PHA = Self::letters_to_token(b"PHA"),
	
	/// TODO.
	PHB = Self::letters_to_token(b"PHB"),
	
	/// TODO.
	PHC = Self::letters_to_token(b"PHC"),
	
	/// TODO.
	PHD = Self::letters_to_token(b"PHD"),
	
	/// TODO.
	PHE = Self::letters_to_token(b"PHE"),
	
	/// TODO.
	PHF = Self::letters_to_token(b"PHF"),
	
	/// TODO.
	PHG = Self::letters_to_token(b"PHG"),
	
	/// TODO.
	PHH = Self::letters_to_token(b"PHH"),
	
	/// TODO.
	PHI = Self::letters_to_token(b"PHI"),
	
	/// TODO.
	PHJ = Self::letters_to_token(b"PHJ"),
	
	/// TODO.
	PHK = Self::letters_to_token(b"PHK"),
	
	/// TODO.
	PHL = Self::letters_to_token(b"PHL"),
	
	/// TODO.
	PHM = Self::letters_to_token(b"PHM"),
	
	/// TODO.
	PHN = Self::letters_to_token(b"PHN"),
	
	/// TODO.
	PHO = Self::letters_to_token(b"PHO"),
	
	/// TODO.
	PHP = Self::letters_to_token(b"PHP"),
	
	/// TODO.
	PHQ = Self::letters_to_token(b"PHQ"),
	
	/// TODO.
	PHR = Self::letters_to_token(b"PHR"),
	
	/// TODO.
	PHS = Self::letters_to_token(b"PHS"),
	
	/// TODO.
	PHT = Self::letters_to_token(b"PHT"),
	
	/// TODO.
	PHU = Self::letters_to_token(b"PHU"),
	
	/// TODO.
	PHV = Self::letters_to_token(b"PHV"),
	
	/// TODO.
	PHW = Self::letters_to_token(b"PHW"),
	
	/// TODO.
	PHX = Self::letters_to_token(b"PHX"),
	
	/// TODO.
	PHY = Self::letters_to_token(b"PHY"),
	
	/// TODO.
	PHZ = Self::letters_to_token(b"PHZ"),
	
	/// TODO.
	PIA = Self::letters_to_token(b"PIA"),
	
	/// TODO.
	PIB = Self::letters_to_token(b"PIB"),
	
	/// TODO.
	PIC = Self::letters_to_token(b"PIC"),
	
	/// TODO.
	PID = Self::letters_to_token(b"PID"),
	
	/// TODO.
	PIE = Self::letters_to_token(b"PIE"),
	
	/// TODO.
	PIF = Self::letters_to_token(b"PIF"),
	
	/// TODO.
	PIG = Self::letters_to_token(b"PIG"),
	
	/// TODO.
	PIH = Self::letters_to_token(b"PIH"),
	
	/// TODO.
	PII = Self::letters_to_token(b"PII"),
	
	/// TODO.
	PIJ = Self::letters_to_token(b"PIJ"),
	
	/// TODO.
	PIK = Self::letters_to_token(b"PIK"),
	
	/// TODO.
	PIL = Self::letters_to_token(b"PIL"),
	
	/// TODO.
	PIM = Self::letters_to_token(b"PIM"),
	
	/// TODO.
	PIN = Self::letters_to_token(b"PIN"),
	
	/// TODO.
	PIO = Self::letters_to_token(b"PIO"),
	
	/// TODO.
	PIP = Self::letters_to_token(b"PIP"),
	
	/// TODO.
	PIQ = Self::letters_to_token(b"PIQ"),
	
	/// TODO.
	PIR = Self::letters_to_token(b"PIR"),
	
	/// TODO.
	PIS = Self::letters_to_token(b"PIS"),
	
	/// TODO.
	PIT = Self::letters_to_token(b"PIT"),
	
	/// TODO.
	PIU = Self::letters_to_token(b"PIU"),
	
	/// TODO.
	PIV = Self::letters_to_token(b"PIV"),
	
	/// TODO.
	PIW = Self::letters_to_token(b"PIW"),
	
	/// TODO.
	PIX = Self::letters_to_token(b"PIX"),
	
	/// TODO.
	PIY = Self::letters_to_token(b"PIY"),
	
	/// TODO.
	PIZ = Self::letters_to_token(b"PIZ"),
	
	/// TODO.
	PJA = Self::letters_to_token(b"PJA"),
	
	/// TODO.
	PJB = Self::letters_to_token(b"PJB"),
	
	/// TODO.
	PJC = Self::letters_to_token(b"PJC"),
	
	/// TODO.
	PJD = Self::letters_to_token(b"PJD"),
	
	/// TODO.
	PJE = Self::letters_to_token(b"PJE"),
	
	/// TODO.
	PJF = Self::letters_to_token(b"PJF"),
	
	/// TODO.
	PJG = Self::letters_to_token(b"PJG"),
	
	/// TODO.
	PJH = Self::letters_to_token(b"PJH"),
	
	/// TODO.
	PJI = Self::letters_to_token(b"PJI"),
	
	/// TODO.
	PJJ = Self::letters_to_token(b"PJJ"),
	
	/// TODO.
	PJK = Self::letters_to_token(b"PJK"),
	
	/// TODO.
	PJL = Self::letters_to_token(b"PJL"),
	
	/// TODO.
	PJM = Self::letters_to_token(b"PJM"),
	
	/// TODO.
	PJN = Self::letters_to_token(b"PJN"),
	
	/// TODO.
	PJO = Self::letters_to_token(b"PJO"),
	
	/// TODO.
	PJP = Self::letters_to_token(b"PJP"),
	
	/// TODO.
	PJQ = Self::letters_to_token(b"PJQ"),
	
	/// TODO.
	PJR = Self::letters_to_token(b"PJR"),
	
	/// TODO.
	PJS = Self::letters_to_token(b"PJS"),
	
	/// TODO.
	PJT = Self::letters_to_token(b"PJT"),
	
	/// TODO.
	PJU = Self::letters_to_token(b"PJU"),
	
	/// TODO.
	PJV = Self::letters_to_token(b"PJV"),
	
	/// TODO.
	PJW = Self::letters_to_token(b"PJW"),
	
	/// TODO.
	PJX = Self::letters_to_token(b"PJX"),
	
	/// TODO.
	PJY = Self::letters_to_token(b"PJY"),
	
	/// TODO.
	PJZ = Self::letters_to_token(b"PJZ"),
	
	/// TODO.
	PKA = Self::letters_to_token(b"PKA"),
	
	/// TODO.
	PKB = Self::letters_to_token(b"PKB"),
	
	/// TODO.
	PKC = Self::letters_to_token(b"PKC"),
	
	/// TODO.
	PKD = Self::letters_to_token(b"PKD"),
	
	/// TODO.
	PKE = Self::letters_to_token(b"PKE"),
	
	/// TODO.
	PKF = Self::letters_to_token(b"PKF"),
	
	/// TODO.
	PKG = Self::letters_to_token(b"PKG"),
	
	/// TODO.
	PKH = Self::letters_to_token(b"PKH"),
	
	/// TODO.
	PKI = Self::letters_to_token(b"PKI"),
	
	/// TODO.
	PKJ = Self::letters_to_token(b"PKJ"),
	
	/// TODO.
	PKK = Self::letters_to_token(b"PKK"),
	
	/// TODO.
	PKL = Self::letters_to_token(b"PKL"),
	
	/// TODO.
	PKM = Self::letters_to_token(b"PKM"),
	
	/// TODO.
	PKN = Self::letters_to_token(b"PKN"),
	
	/// TODO.
	PKO = Self::letters_to_token(b"PKO"),
	
	/// TODO.
	PKP = Self::letters_to_token(b"PKP"),
	
	/// TODO.
	PKQ = Self::letters_to_token(b"PKQ"),
	
	/// TODO.
	PKR = Self::letters_to_token(b"PKR"),
	
	/// TODO.
	PKS = Self::letters_to_token(b"PKS"),
	
	/// TODO.
	PKT = Self::letters_to_token(b"PKT"),
	
	/// TODO.
	PKU = Self::letters_to_token(b"PKU"),
	
	/// TODO.
	PKV = Self::letters_to_token(b"PKV"),
	
	/// TODO.
	PKW = Self::letters_to_token(b"PKW"),
	
	/// TODO.
	PKX = Self::letters_to_token(b"PKX"),
	
	/// TODO.
	PKY = Self::letters_to_token(b"PKY"),
	
	/// TODO.
	PKZ = Self::letters_to_token(b"PKZ"),
	
	/// TODO.
	PLA = Self::letters_to_token(b"PLA"),
	
	/// TODO.
	PLB = Self::letters_to_token(b"PLB"),
	
	/// TODO.
	PLC = Self::letters_to_token(b"PLC"),
	
	/// TODO.
	PLD = Self::letters_to_token(b"PLD"),
	
	/// TODO.
	PLE = Self::letters_to_token(b"PLE"),
	
	/// TODO.
	PLF = Self::letters_to_token(b"PLF"),
	
	/// TODO.
	PLG = Self::letters_to_token(b"PLG"),
	
	/// TODO.
	PLH = Self::letters_to_token(b"PLH"),
	
	/// TODO.
	PLI = Self::letters_to_token(b"PLI"),
	
	/// TODO.
	PLJ = Self::letters_to_token(b"PLJ"),
	
	/// TODO.
	PLK = Self::letters_to_token(b"PLK"),
	
	/// TODO.
	PLL = Self::letters_to_token(b"PLL"),
	
	/// TODO.
	PLM = Self::letters_to_token(b"PLM"),
	
	/// TODO.
	PLN = Self::letters_to_token(b"PLN"),
	
	/// TODO.
	PLO = Self::letters_to_token(b"PLO"),
	
	/// TODO.
	PLP = Self::letters_to_token(b"PLP"),
	
	/// TODO.
	PLQ = Self::letters_to_token(b"PLQ"),
	
	/// TODO.
	PLR = Self::letters_to_token(b"PLR"),
	
	/// TODO.
	PLS = Self::letters_to_token(b"PLS"),
	
	/// TODO.
	PLT = Self::letters_to_token(b"PLT"),
	
	/// TODO.
	PLU = Self::letters_to_token(b"PLU"),
	
	/// TODO.
	PLV = Self::letters_to_token(b"PLV"),
	
	/// TODO.
	PLW = Self::letters_to_token(b"PLW"),
	
	/// TODO.
	PLX = Self::letters_to_token(b"PLX"),
	
	/// TODO.
	PLY = Self::letters_to_token(b"PLY"),
	
	/// TODO.
	PLZ = Self::letters_to_token(b"PLZ"),
	
	/// TODO.
	PMA = Self::letters_to_token(b"PMA"),
	
	/// TODO.
	PMB = Self::letters_to_token(b"PMB"),
	
	/// TODO.
	PMC = Self::letters_to_token(b"PMC"),
	
	/// TODO.
	PMD = Self::letters_to_token(b"PMD"),
	
	/// TODO.
	PME = Self::letters_to_token(b"PME"),
	
	/// TODO.
	PMF = Self::letters_to_token(b"PMF"),
	
	/// TODO.
	PMG = Self::letters_to_token(b"PMG"),
	
	/// TODO.
	PMH = Self::letters_to_token(b"PMH"),
	
	/// TODO.
	PMI = Self::letters_to_token(b"PMI"),
	
	/// TODO.
	PMJ = Self::letters_to_token(b"PMJ"),
	
	/// TODO.
	PMK = Self::letters_to_token(b"PMK"),
	
	/// TODO.
	PML = Self::letters_to_token(b"PML"),
	
	/// TODO.
	PMM = Self::letters_to_token(b"PMM"),
	
	/// TODO.
	PMN = Self::letters_to_token(b"PMN"),
	
	/// TODO.
	PMO = Self::letters_to_token(b"PMO"),
	
	/// TODO.
	PMP = Self::letters_to_token(b"PMP"),
	
	/// TODO.
	PMQ = Self::letters_to_token(b"PMQ"),
	
	/// TODO.
	PMR = Self::letters_to_token(b"PMR"),
	
	/// TODO.
	PMS = Self::letters_to_token(b"PMS"),
	
	/// TODO.
	PMT = Self::letters_to_token(b"PMT"),
	
	/// TODO.
	PMU = Self::letters_to_token(b"PMU"),
	
	/// TODO.
	PMV = Self::letters_to_token(b"PMV"),
	
	/// TODO.
	PMW = Self::letters_to_token(b"PMW"),
	
	/// TODO.
	PMX = Self::letters_to_token(b"PMX"),
	
	/// TODO.
	PMY = Self::letters_to_token(b"PMY"),
	
	/// TODO.
	PMZ = Self::letters_to_token(b"PMZ"),
	
	/// TODO.
	PNA = Self::letters_to_token(b"PNA"),
	
	/// TODO.
	PNB = Self::letters_to_token(b"PNB"),
	
	/// TODO.
	PNC = Self::letters_to_token(b"PNC"),
	
	/// TODO.
	PND = Self::letters_to_token(b"PND"),
	
	/// TODO.
	PNE = Self::letters_to_token(b"PNE"),
	
	/// TODO.
	PNF = Self::letters_to_token(b"PNF"),
	
	/// TODO.
	PNG = Self::letters_to_token(b"PNG"),
	
	/// TODO.
	PNH = Self::letters_to_token(b"PNH"),
	
	/// TODO.
	PNI = Self::letters_to_token(b"PNI"),
	
	/// TODO.
	PNJ = Self::letters_to_token(b"PNJ"),
	
	/// TODO.
	PNK = Self::letters_to_token(b"PNK"),
	
	/// TODO.
	PNL = Self::letters_to_token(b"PNL"),
	
	/// TODO.
	PNM = Self::letters_to_token(b"PNM"),
	
	/// TODO.
	PNN = Self::letters_to_token(b"PNN"),
	
	/// TODO.
	PNO = Self::letters_to_token(b"PNO"),
	
	/// TODO.
	PNP = Self::letters_to_token(b"PNP"),
	
	/// TODO.
	PNQ = Self::letters_to_token(b"PNQ"),
	
	/// TODO.
	PNR = Self::letters_to_token(b"PNR"),
	
	/// TODO.
	PNS = Self::letters_to_token(b"PNS"),
	
	/// TODO.
	PNT = Self::letters_to_token(b"PNT"),
	
	/// TODO.
	PNU = Self::letters_to_token(b"PNU"),
	
	/// TODO.
	PNV = Self::letters_to_token(b"PNV"),
	
	/// TODO.
	PNW = Self::letters_to_token(b"PNW"),
	
	/// TODO.
	PNX = Self::letters_to_token(b"PNX"),
	
	/// TODO.
	PNY = Self::letters_to_token(b"PNY"),
	
	/// TODO.
	PNZ = Self::letters_to_token(b"PNZ"),
	
	/// TODO.
	POA = Self::letters_to_token(b"POA"),
	
	/// TODO.
	POB = Self::letters_to_token(b"POB"),
	
	/// TODO.
	POC = Self::letters_to_token(b"POC"),
	
	/// TODO.
	POD = Self::letters_to_token(b"POD"),
	
	/// TODO.
	POE = Self::letters_to_token(b"POE"),
	
	/// TODO.
	POF = Self::letters_to_token(b"POF"),
	
	/// TODO.
	POG = Self::letters_to_token(b"POG"),
	
	/// TODO.
	POH = Self::letters_to_token(b"POH"),
	
	/// TODO.
	POI = Self::letters_to_token(b"POI"),
	
	/// TODO.
	POJ = Self::letters_to_token(b"POJ"),
	
	/// TODO.
	POK = Self::letters_to_token(b"POK"),
	
	/// TODO.
	POL = Self::letters_to_token(b"POL"),
	
	/// TODO.
	POM = Self::letters_to_token(b"POM"),
	
	/// TODO.
	PON = Self::letters_to_token(b"PON"),
	
	/// TODO.
	POO = Self::letters_to_token(b"POO"),
	
	/// TODO.
	POP = Self::letters_to_token(b"POP"),
	
	/// TODO.
	POQ = Self::letters_to_token(b"POQ"),
	
	/// TODO.
	POR = Self::letters_to_token(b"POR"),
	
	/// TODO.
	POS = Self::letters_to_token(b"POS"),
	
	/// TODO.
	POT = Self::letters_to_token(b"POT"),
	
	/// TODO.
	POU = Self::letters_to_token(b"POU"),
	
	/// TODO.
	POV = Self::letters_to_token(b"POV"),
	
	/// TODO.
	POW = Self::letters_to_token(b"POW"),
	
	/// TODO.
	POX = Self::letters_to_token(b"POX"),
	
	/// TODO.
	POY = Self::letters_to_token(b"POY"),
	
	/// TODO.
	POZ = Self::letters_to_token(b"POZ"),
	
	/// TODO.
	PPA = Self::letters_to_token(b"PPA"),
	
	/// TODO.
	PPB = Self::letters_to_token(b"PPB"),
	
	/// TODO.
	PPC = Self::letters_to_token(b"PPC"),
	
	/// TODO.
	PPD = Self::letters_to_token(b"PPD"),
	
	/// TODO.
	PPE = Self::letters_to_token(b"PPE"),
	
	/// TODO.
	PPF = Self::letters_to_token(b"PPF"),
	
	/// TODO.
	PPG = Self::letters_to_token(b"PPG"),
	
	/// TODO.
	PPH = Self::letters_to_token(b"PPH"),
	
	/// TODO.
	PPI = Self::letters_to_token(b"PPI"),
	
	/// TODO.
	PPJ = Self::letters_to_token(b"PPJ"),
	
	/// TODO.
	PPK = Self::letters_to_token(b"PPK"),
	
	/// TODO.
	PPL = Self::letters_to_token(b"PPL"),
	
	/// TODO.
	PPM = Self::letters_to_token(b"PPM"),
	
	/// TODO.
	PPN = Self::letters_to_token(b"PPN"),
	
	/// TODO.
	PPO = Self::letters_to_token(b"PPO"),
	
	/// TODO.
	PPP = Self::letters_to_token(b"PPP"),
	
	/// TODO.
	PPQ = Self::letters_to_token(b"PPQ"),
	
	/// TODO.
	PPR = Self::letters_to_token(b"PPR"),
	
	/// TODO.
	PPS = Self::letters_to_token(b"PPS"),
	
	/// TODO.
	PPT = Self::letters_to_token(b"PPT"),
	
	/// TODO.
	PPU = Self::letters_to_token(b"PPU"),
	
	/// TODO.
	PPV = Self::letters_to_token(b"PPV"),
	
	/// TODO.
	PPW = Self::letters_to_token(b"PPW"),
	
	/// TODO.
	PPX = Self::letters_to_token(b"PPX"),
	
	/// TODO.
	PPY = Self::letters_to_token(b"PPY"),
	
	/// TODO.
	PPZ = Self::letters_to_token(b"PPZ"),
	
	/// TODO.
	PQA = Self::letters_to_token(b"PQA"),
	
	/// TODO.
	PQB = Self::letters_to_token(b"PQB"),
	
	/// TODO.
	PQC = Self::letters_to_token(b"PQC"),
	
	/// TODO.
	PQD = Self::letters_to_token(b"PQD"),
	
	/// TODO.
	PQE = Self::letters_to_token(b"PQE"),
	
	/// TODO.
	PQF = Self::letters_to_token(b"PQF"),
	
	/// TODO.
	PQG = Self::letters_to_token(b"PQG"),
	
	/// TODO.
	PQH = Self::letters_to_token(b"PQH"),
	
	/// TODO.
	PQI = Self::letters_to_token(b"PQI"),
	
	/// TODO.
	PQJ = Self::letters_to_token(b"PQJ"),
	
	/// TODO.
	PQK = Self::letters_to_token(b"PQK"),
	
	/// TODO.
	PQL = Self::letters_to_token(b"PQL"),
	
	/// TODO.
	PQM = Self::letters_to_token(b"PQM"),
	
	/// TODO.
	PQN = Self::letters_to_token(b"PQN"),
	
	/// TODO.
	PQO = Self::letters_to_token(b"PQO"),
	
	/// TODO.
	PQP = Self::letters_to_token(b"PQP"),
	
	/// TODO.
	PQQ = Self::letters_to_token(b"PQQ"),
	
	/// TODO.
	PQR = Self::letters_to_token(b"PQR"),
	
	/// TODO.
	PQS = Self::letters_to_token(b"PQS"),
	
	/// TODO.
	PQT = Self::letters_to_token(b"PQT"),
	
	/// TODO.
	PQU = Self::letters_to_token(b"PQU"),
	
	/// TODO.
	PQV = Self::letters_to_token(b"PQV"),
	
	/// TODO.
	PQW = Self::letters_to_token(b"PQW"),
	
	/// TODO.
	PQX = Self::letters_to_token(b"PQX"),
	
	/// TODO.
	PQY = Self::letters_to_token(b"PQY"),
	
	/// TODO.
	PQZ = Self::letters_to_token(b"PQZ"),
	
	/// TODO.
	PRA = Self::letters_to_token(b"PRA"),
	
	/// TODO.
	PRB = Self::letters_to_token(b"PRB"),
	
	/// TODO.
	PRC = Self::letters_to_token(b"PRC"),
	
	/// TODO.
	PRD = Self::letters_to_token(b"PRD"),
	
	/// TODO.
	PRE = Self::letters_to_token(b"PRE"),
	
	/// TODO.
	PRF = Self::letters_to_token(b"PRF"),
	
	/// TODO.
	PRG = Self::letters_to_token(b"PRG"),
	
	/// TODO.
	PRH = Self::letters_to_token(b"PRH"),
	
	/// TODO.
	PRI = Self::letters_to_token(b"PRI"),
	
	/// TODO.
	PRJ = Self::letters_to_token(b"PRJ"),
	
	/// TODO.
	PRK = Self::letters_to_token(b"PRK"),
	
	/// TODO.
	PRL = Self::letters_to_token(b"PRL"),
	
	/// TODO.
	PRM = Self::letters_to_token(b"PRM"),
	
	/// TODO.
	PRN = Self::letters_to_token(b"PRN"),
	
	/// TODO.
	PRO = Self::letters_to_token(b"PRO"),
	
	/// TODO.
	PRP = Self::letters_to_token(b"PRP"),
	
	/// TODO.
	PRQ = Self::letters_to_token(b"PRQ"),
	
	/// TODO.
	PRR = Self::letters_to_token(b"PRR"),
	
	/// TODO.
	PRS = Self::letters_to_token(b"PRS"),
	
	/// TODO.
	PRT = Self::letters_to_token(b"PRT"),
	
	/// TODO.
	PRU = Self::letters_to_token(b"PRU"),
	
	/// TODO.
	PRV = Self::letters_to_token(b"PRV"),
	
	/// TODO.
	PRW = Self::letters_to_token(b"PRW"),
	
	/// TODO.
	PRX = Self::letters_to_token(b"PRX"),
	
	/// TODO.
	PRY = Self::letters_to_token(b"PRY"),
	
	/// TODO.
	PRZ = Self::letters_to_token(b"PRZ"),
	
	/// TODO.
	PSA = Self::letters_to_token(b"PSA"),
	
	/// TODO.
	PSB = Self::letters_to_token(b"PSB"),
	
	/// TODO.
	PSC = Self::letters_to_token(b"PSC"),
	
	/// TODO.
	PSD = Self::letters_to_token(b"PSD"),
	
	/// TODO.
	PSE = Self::letters_to_token(b"PSE"),
	
	/// TODO.
	PSF = Self::letters_to_token(b"PSF"),
	
	/// TODO.
	PSG = Self::letters_to_token(b"PSG"),
	
	/// TODO.
	PSH = Self::letters_to_token(b"PSH"),
	
	/// TODO.
	PSI = Self::letters_to_token(b"PSI"),
	
	/// TODO.
	PSJ = Self::letters_to_token(b"PSJ"),
	
	/// TODO.
	PSK = Self::letters_to_token(b"PSK"),
	
	/// TODO.
	PSL = Self::letters_to_token(b"PSL"),
	
	/// TODO.
	PSM = Self::letters_to_token(b"PSM"),
	
	/// TODO.
	PSN = Self::letters_to_token(b"PSN"),
	
	/// TODO.
	PSO = Self::letters_to_token(b"PSO"),
	
	/// TODO.
	PSP = Self::letters_to_token(b"PSP"),
	
	/// TODO.
	PSQ = Self::letters_to_token(b"PSQ"),
	
	/// TODO.
	PSR = Self::letters_to_token(b"PSR"),
	
	/// TODO.
	PSS = Self::letters_to_token(b"PSS"),
	
	/// TODO.
	PST = Self::letters_to_token(b"PST"),
	
	/// TODO.
	PSU = Self::letters_to_token(b"PSU"),
	
	/// TODO.
	PSV = Self::letters_to_token(b"PSV"),
	
	/// TODO.
	PSW = Self::letters_to_token(b"PSW"),
	
	/// TODO.
	PSX = Self::letters_to_token(b"PSX"),
	
	/// TODO.
	PSY = Self::letters_to_token(b"PSY"),
	
	/// TODO.
	PSZ = Self::letters_to_token(b"PSZ"),
	
	/// TODO.
	PTA = Self::letters_to_token(b"PTA"),
	
	/// TODO.
	PTB = Self::letters_to_token(b"PTB"),
	
	/// TODO.
	PTC = Self::letters_to_token(b"PTC"),
	
	/// TODO.
	PTD = Self::letters_to_token(b"PTD"),
	
	/// TODO.
	PTE = Self::letters_to_token(b"PTE"),
	
	/// TODO.
	PTF = Self::letters_to_token(b"PTF"),
	
	/// TODO.
	PTG = Self::letters_to_token(b"PTG"),
	
	/// TODO.
	PTH = Self::letters_to_token(b"PTH"),
	
	/// TODO.
	PTI = Self::letters_to_token(b"PTI"),
	
	/// TODO.
	PTJ = Self::letters_to_token(b"PTJ"),
	
	/// TODO.
	PTK = Self::letters_to_token(b"PTK"),
	
	/// TODO.
	PTL = Self::letters_to_token(b"PTL"),
	
	/// TODO.
	PTM = Self::letters_to_token(b"PTM"),
	
	/// TODO.
	PTN = Self::letters_to_token(b"PTN"),
	
	/// TODO.
	PTO = Self::letters_to_token(b"PTO"),
	
	/// TODO.
	PTP = Self::letters_to_token(b"PTP"),
	
	/// TODO.
	PTQ = Self::letters_to_token(b"PTQ"),
	
	/// TODO.
	PTR = Self::letters_to_token(b"PTR"),
	
	/// TODO.
	PTS = Self::letters_to_token(b"PTS"),
	
	/// TODO.
	PTT = Self::letters_to_token(b"PTT"),
	
	/// TODO.
	PTU = Self::letters_to_token(b"PTU"),
	
	/// TODO.
	PTV = Self::letters_to_token(b"PTV"),
	
	/// TODO.
	PTW = Self::letters_to_token(b"PTW"),
	
	/// TODO.
	PTX = Self::letters_to_token(b"PTX"),
	
	/// TODO.
	PTY = Self::letters_to_token(b"PTY"),
	
	/// TODO.
	PTZ = Self::letters_to_token(b"PTZ"),
	
	/// TODO.
	PUA = Self::letters_to_token(b"PUA"),
	
	/// TODO.
	PUB = Self::letters_to_token(b"PUB"),
	
	/// TODO.
	PUC = Self::letters_to_token(b"PUC"),
	
	/// TODO.
	PUD = Self::letters_to_token(b"PUD"),
	
	/// TODO.
	PUE = Self::letters_to_token(b"PUE"),
	
	/// TODO.
	PUF = Self::letters_to_token(b"PUF"),
	
	/// TODO.
	PUG = Self::letters_to_token(b"PUG"),
	
	/// TODO.
	PUH = Self::letters_to_token(b"PUH"),
	
	/// TODO.
	PUI = Self::letters_to_token(b"PUI"),
	
	/// TODO.
	PUJ = Self::letters_to_token(b"PUJ"),
	
	/// TODO.
	PUK = Self::letters_to_token(b"PUK"),
	
	/// TODO.
	PUL = Self::letters_to_token(b"PUL"),
	
	/// TODO.
	PUM = Self::letters_to_token(b"PUM"),
	
	/// TODO.
	PUN = Self::letters_to_token(b"PUN"),
	
	/// TODO.
	PUO = Self::letters_to_token(b"PUO"),
	
	/// TODO.
	PUP = Self::letters_to_token(b"PUP"),
	
	/// TODO.
	PUQ = Self::letters_to_token(b"PUQ"),
	
	/// TODO.
	PUR = Self::letters_to_token(b"PUR"),
	
	/// TODO.
	PUS = Self::letters_to_token(b"PUS"),
	
	/// TODO.
	PUT = Self::letters_to_token(b"PUT"),
	
	/// TODO.
	PUU = Self::letters_to_token(b"PUU"),
	
	/// TODO.
	PUV = Self::letters_to_token(b"PUV"),
	
	/// TODO.
	PUW = Self::letters_to_token(b"PUW"),
	
	/// TODO.
	PUX = Self::letters_to_token(b"PUX"),
	
	/// TODO.
	PUY = Self::letters_to_token(b"PUY"),
	
	/// TODO.
	PUZ = Self::letters_to_token(b"PUZ"),
	
	/// TODO.
	PVA = Self::letters_to_token(b"PVA"),
	
	/// TODO.
	PVB = Self::letters_to_token(b"PVB"),
	
	/// TODO.
	PVC = Self::letters_to_token(b"PVC"),
	
	/// TODO.
	PVD = Self::letters_to_token(b"PVD"),
	
	/// TODO.
	PVE = Self::letters_to_token(b"PVE"),
	
	/// TODO.
	PVF = Self::letters_to_token(b"PVF"),
	
	/// TODO.
	PVG = Self::letters_to_token(b"PVG"),
	
	/// TODO.
	PVH = Self::letters_to_token(b"PVH"),
	
	/// TODO.
	PVI = Self::letters_to_token(b"PVI"),
	
	/// TODO.
	PVJ = Self::letters_to_token(b"PVJ"),
	
	/// TODO.
	PVK = Self::letters_to_token(b"PVK"),
	
	/// TODO.
	PVL = Self::letters_to_token(b"PVL"),
	
	/// TODO.
	PVM = Self::letters_to_token(b"PVM"),
	
	/// TODO.
	PVN = Self::letters_to_token(b"PVN"),
	
	/// TODO.
	PVO = Self::letters_to_token(b"PVO"),
	
	/// TODO.
	PVP = Self::letters_to_token(b"PVP"),
	
	/// TODO.
	PVQ = Self::letters_to_token(b"PVQ"),
	
	/// TODO.
	PVR = Self::letters_to_token(b"PVR"),
	
	/// TODO.
	PVS = Self::letters_to_token(b"PVS"),
	
	/// TODO.
	PVT = Self::letters_to_token(b"PVT"),
	
	/// TODO.
	PVU = Self::letters_to_token(b"PVU"),
	
	/// TODO.
	PVV = Self::letters_to_token(b"PVV"),
	
	/// TODO.
	PVW = Self::letters_to_token(b"PVW"),
	
	/// TODO.
	PVX = Self::letters_to_token(b"PVX"),
	
	/// TODO.
	PVY = Self::letters_to_token(b"PVY"),
	
	/// TODO.
	PVZ = Self::letters_to_token(b"PVZ"),
	
	/// TODO.
	PWA = Self::letters_to_token(b"PWA"),
	
	/// TODO.
	PWB = Self::letters_to_token(b"PWB"),
	
	/// TODO.
	PWC = Self::letters_to_token(b"PWC"),
	
	/// TODO.
	PWD = Self::letters_to_token(b"PWD"),
	
	/// TODO.
	PWE = Self::letters_to_token(b"PWE"),
	
	/// TODO.
	PWF = Self::letters_to_token(b"PWF"),
	
	/// TODO.
	PWG = Self::letters_to_token(b"PWG"),
	
	/// TODO.
	PWH = Self::letters_to_token(b"PWH"),
	
	/// TODO.
	PWI = Self::letters_to_token(b"PWI"),
	
	/// TODO.
	PWJ = Self::letters_to_token(b"PWJ"),
	
	/// TODO.
	PWK = Self::letters_to_token(b"PWK"),
	
	/// TODO.
	PWL = Self::letters_to_token(b"PWL"),
	
	/// TODO.
	PWM = Self::letters_to_token(b"PWM"),
	
	/// TODO.
	PWN = Self::letters_to_token(b"PWN"),
	
	/// TODO.
	PWO = Self::letters_to_token(b"PWO"),
	
	/// TODO.
	PWP = Self::letters_to_token(b"PWP"),
	
	/// TODO.
	PWQ = Self::letters_to_token(b"PWQ"),
	
	/// TODO.
	PWR = Self::letters_to_token(b"PWR"),
	
	/// TODO.
	PWS = Self::letters_to_token(b"PWS"),
	
	/// TODO.
	PWT = Self::letters_to_token(b"PWT"),
	
	/// TODO.
	PWU = Self::letters_to_token(b"PWU"),
	
	/// TODO.
	PWV = Self::letters_to_token(b"PWV"),
	
	/// TODO.
	PWW = Self::letters_to_token(b"PWW"),
	
	/// TODO.
	PWX = Self::letters_to_token(b"PWX"),
	
	/// TODO.
	PWY = Self::letters_to_token(b"PWY"),
	
	/// TODO.
	PWZ = Self::letters_to_token(b"PWZ"),
	
	/// TODO.
	PXA = Self::letters_to_token(b"PXA"),
	
	/// TODO.
	PXB = Self::letters_to_token(b"PXB"),
	
	/// TODO.
	PXC = Self::letters_to_token(b"PXC"),
	
	/// TODO.
	PXD = Self::letters_to_token(b"PXD"),
	
	/// TODO.
	PXE = Self::letters_to_token(b"PXE"),
	
	/// TODO.
	PXF = Self::letters_to_token(b"PXF"),
	
	/// TODO.
	PXG = Self::letters_to_token(b"PXG"),
	
	/// TODO.
	PXH = Self::letters_to_token(b"PXH"),
	
	/// TODO.
	PXI = Self::letters_to_token(b"PXI"),
	
	/// TODO.
	PXJ = Self::letters_to_token(b"PXJ"),
	
	/// TODO.
	PXK = Self::letters_to_token(b"PXK"),
	
	/// TODO.
	PXL = Self::letters_to_token(b"PXL"),
	
	/// TODO.
	PXM = Self::letters_to_token(b"PXM"),
	
	/// TODO.
	PXN = Self::letters_to_token(b"PXN"),
	
	/// TODO.
	PXO = Self::letters_to_token(b"PXO"),
	
	/// TODO.
	PXP = Self::letters_to_token(b"PXP"),
	
	/// TODO.
	PXQ = Self::letters_to_token(b"PXQ"),
	
	/// TODO.
	PXR = Self::letters_to_token(b"PXR"),
	
	/// TODO.
	PXS = Self::letters_to_token(b"PXS"),
	
	/// TODO.
	PXT = Self::letters_to_token(b"PXT"),
	
	/// TODO.
	PXU = Self::letters_to_token(b"PXU"),
	
	/// TODO.
	PXV = Self::letters_to_token(b"PXV"),
	
	/// TODO.
	PXW = Self::letters_to_token(b"PXW"),
	
	/// TODO.
	PXX = Self::letters_to_token(b"PXX"),
	
	/// TODO.
	PXY = Self::letters_to_token(b"PXY"),
	
	/// TODO.
	PXZ = Self::letters_to_token(b"PXZ"),
	
	/// TODO.
	PYA = Self::letters_to_token(b"PYA"),
	
	/// TODO.
	PYB = Self::letters_to_token(b"PYB"),
	
	/// TODO.
	PYC = Self::letters_to_token(b"PYC"),
	
	/// TODO.
	PYD = Self::letters_to_token(b"PYD"),
	
	/// TODO.
	PYE = Self::letters_to_token(b"PYE"),
	
	/// TODO.
	PYF = Self::letters_to_token(b"PYF"),
	
	/// TODO.
	PYG = Self::letters_to_token(b"PYG"),
	
	/// TODO.
	PYH = Self::letters_to_token(b"PYH"),
	
	/// TODO.
	PYI = Self::letters_to_token(b"PYI"),
	
	/// TODO.
	PYJ = Self::letters_to_token(b"PYJ"),
	
	/// TODO.
	PYK = Self::letters_to_token(b"PYK"),
	
	/// TODO.
	PYL = Self::letters_to_token(b"PYL"),
	
	/// TODO.
	PYM = Self::letters_to_token(b"PYM"),
	
	/// TODO.
	PYN = Self::letters_to_token(b"PYN"),
	
	/// TODO.
	PYO = Self::letters_to_token(b"PYO"),
	
	/// TODO.
	PYP = Self::letters_to_token(b"PYP"),
	
	/// TODO.
	PYQ = Self::letters_to_token(b"PYQ"),
	
	/// TODO.
	PYR = Self::letters_to_token(b"PYR"),
	
	/// TODO.
	PYS = Self::letters_to_token(b"PYS"),
	
	/// TODO.
	PYT = Self::letters_to_token(b"PYT"),
	
	/// TODO.
	PYU = Self::letters_to_token(b"PYU"),
	
	/// TODO.
	PYV = Self::letters_to_token(b"PYV"),
	
	/// TODO.
	PYW = Self::letters_to_token(b"PYW"),
	
	/// TODO.
	PYX = Self::letters_to_token(b"PYX"),
	
	/// TODO.
	PYY = Self::letters_to_token(b"PYY"),
	
	/// TODO.
	PYZ = Self::letters_to_token(b"PYZ"),
	
	/// TODO.
	PZA = Self::letters_to_token(b"PZA"),
	
	/// TODO.
	PZB = Self::letters_to_token(b"PZB"),
	
	/// TODO.
	PZC = Self::letters_to_token(b"PZC"),
	
	/// TODO.
	PZD = Self::letters_to_token(b"PZD"),
	
	/// TODO.
	PZE = Self::letters_to_token(b"PZE"),
	
	/// TODO.
	PZF = Self::letters_to_token(b"PZF"),
	
	/// TODO.
	PZG = Self::letters_to_token(b"PZG"),
	
	/// TODO.
	PZH = Self::letters_to_token(b"PZH"),
	
	/// TODO.
	PZI = Self::letters_to_token(b"PZI"),
	
	/// TODO.
	PZJ = Self::letters_to_token(b"PZJ"),
	
	/// TODO.
	PZK = Self::letters_to_token(b"PZK"),
	
	/// TODO.
	PZL = Self::letters_to_token(b"PZL"),
	
	/// TODO.
	PZM = Self::letters_to_token(b"PZM"),
	
	/// TODO.
	PZN = Self::letters_to_token(b"PZN"),
	
	/// TODO.
	PZO = Self::letters_to_token(b"PZO"),
	
	/// TODO.
	PZP = Self::letters_to_token(b"PZP"),
	
	/// TODO.
	PZQ = Self::letters_to_token(b"PZQ"),
	
	/// TODO.
	PZR = Self::letters_to_token(b"PZR"),
	
	/// TODO.
	PZS = Self::letters_to_token(b"PZS"),
	
	/// TODO.
	PZT = Self::letters_to_token(b"PZT"),
	
	/// TODO.
	PZU = Self::letters_to_token(b"PZU"),
	
	/// TODO.
	PZV = Self::letters_to_token(b"PZV"),
	
	/// TODO.
	PZW = Self::letters_to_token(b"PZW"),
	
	/// TODO.
	PZX = Self::letters_to_token(b"PZX"),
	
	/// TODO.
	PZY = Self::letters_to_token(b"PZY"),
	
	/// TODO.
	PZZ = Self::letters_to_token(b"PZZ"),
	
	/// TODO.
	QAA = Self::letters_to_token(b"QAA"),
	
	/// TODO.
	QAB = Self::letters_to_token(b"QAB"),
	
	/// TODO.
	QAC = Self::letters_to_token(b"QAC"),
	
	/// TODO.
	QAD = Self::letters_to_token(b"QAD"),
	
	/// TODO.
	QAE = Self::letters_to_token(b"QAE"),
	
	/// TODO.
	QAF = Self::letters_to_token(b"QAF"),
	
	/// TODO.
	QAG = Self::letters_to_token(b"QAG"),
	
	/// TODO.
	QAH = Self::letters_to_token(b"QAH"),
	
	/// TODO.
	QAI = Self::letters_to_token(b"QAI"),
	
	/// TODO.
	QAJ = Self::letters_to_token(b"QAJ"),
	
	/// TODO.
	QAK = Self::letters_to_token(b"QAK"),
	
	/// TODO.
	QAL = Self::letters_to_token(b"QAL"),
	
	/// TODO.
	QAM = Self::letters_to_token(b"QAM"),
	
	/// TODO.
	QAN = Self::letters_to_token(b"QAN"),
	
	/// TODO.
	QAO = Self::letters_to_token(b"QAO"),
	
	/// TODO.
	QAP = Self::letters_to_token(b"QAP"),
	
	/// TODO.
	QAQ = Self::letters_to_token(b"QAQ"),
	
	/// TODO.
	QAR = Self::letters_to_token(b"QAR"),
	
	/// TODO.
	QAS = Self::letters_to_token(b"QAS"),
	
	/// TODO.
	QAT = Self::letters_to_token(b"QAT"),
	
	/// TODO.
	QAU = Self::letters_to_token(b"QAU"),
	
	/// TODO.
	QAV = Self::letters_to_token(b"QAV"),
	
	/// TODO.
	QAW = Self::letters_to_token(b"QAW"),
	
	/// TODO.
	QAX = Self::letters_to_token(b"QAX"),
	
	/// TODO.
	QAY = Self::letters_to_token(b"QAY"),
	
	/// TODO.
	QAZ = Self::letters_to_token(b"QAZ"),
	
	/// TODO.
	QBA = Self::letters_to_token(b"QBA"),
	
	/// TODO.
	QBB = Self::letters_to_token(b"QBB"),
	
	/// TODO.
	QBC = Self::letters_to_token(b"QBC"),
	
	/// TODO.
	QBD = Self::letters_to_token(b"QBD"),
	
	/// TODO.
	QBE = Self::letters_to_token(b"QBE"),
	
	/// TODO.
	QBF = Self::letters_to_token(b"QBF"),
	
	/// TODO.
	QBG = Self::letters_to_token(b"QBG"),
	
	/// TODO.
	QBH = Self::letters_to_token(b"QBH"),
	
	/// TODO.
	QBI = Self::letters_to_token(b"QBI"),
	
	/// TODO.
	QBJ = Self::letters_to_token(b"QBJ"),
	
	/// TODO.
	QBK = Self::letters_to_token(b"QBK"),
	
	/// TODO.
	QBL = Self::letters_to_token(b"QBL"),
	
	/// TODO.
	QBM = Self::letters_to_token(b"QBM"),
	
	/// TODO.
	QBN = Self::letters_to_token(b"QBN"),
	
	/// TODO.
	QBO = Self::letters_to_token(b"QBO"),
	
	/// TODO.
	QBP = Self::letters_to_token(b"QBP"),
	
	/// TODO.
	QBQ = Self::letters_to_token(b"QBQ"),
	
	/// TODO.
	QBR = Self::letters_to_token(b"QBR"),
	
	/// TODO.
	QBS = Self::letters_to_token(b"QBS"),
	
	/// TODO.
	QBT = Self::letters_to_token(b"QBT"),
	
	/// TODO.
	QBU = Self::letters_to_token(b"QBU"),
	
	/// TODO.
	QBV = Self::letters_to_token(b"QBV"),
	
	/// TODO.
	QBW = Self::letters_to_token(b"QBW"),
	
	/// TODO.
	QBX = Self::letters_to_token(b"QBX"),
	
	/// TODO.
	QBY = Self::letters_to_token(b"QBY"),
	
	/// TODO.
	QBZ = Self::letters_to_token(b"QBZ"),
	
	/// TODO.
	QCA = Self::letters_to_token(b"QCA"),
	
	/// TODO.
	QCB = Self::letters_to_token(b"QCB"),
	
	/// TODO.
	QCC = Self::letters_to_token(b"QCC"),
	
	/// TODO.
	QCD = Self::letters_to_token(b"QCD"),
	
	/// TODO.
	QCE = Self::letters_to_token(b"QCE"),
	
	/// TODO.
	QCF = Self::letters_to_token(b"QCF"),
	
	/// TODO.
	QCG = Self::letters_to_token(b"QCG"),
	
	/// TODO.
	QCH = Self::letters_to_token(b"QCH"),
	
	/// TODO.
	QCI = Self::letters_to_token(b"QCI"),
	
	/// TODO.
	QCJ = Self::letters_to_token(b"QCJ"),
	
	/// TODO.
	QCK = Self::letters_to_token(b"QCK"),
	
	/// TODO.
	QCL = Self::letters_to_token(b"QCL"),
	
	/// TODO.
	QCM = Self::letters_to_token(b"QCM"),
	
	/// TODO.
	QCN = Self::letters_to_token(b"QCN"),
	
	/// TODO.
	QCO = Self::letters_to_token(b"QCO"),
	
	/// TODO.
	QCP = Self::letters_to_token(b"QCP"),
	
	/// TODO.
	QCQ = Self::letters_to_token(b"QCQ"),
	
	/// TODO.
	QCR = Self::letters_to_token(b"QCR"),
	
	/// TODO.
	QCS = Self::letters_to_token(b"QCS"),
	
	/// TODO.
	QCT = Self::letters_to_token(b"QCT"),
	
	/// TODO.
	QCU = Self::letters_to_token(b"QCU"),
	
	/// TODO.
	QCV = Self::letters_to_token(b"QCV"),
	
	/// TODO.
	QCW = Self::letters_to_token(b"QCW"),
	
	/// TODO.
	QCX = Self::letters_to_token(b"QCX"),
	
	/// TODO.
	QCY = Self::letters_to_token(b"QCY"),
	
	/// TODO.
	QCZ = Self::letters_to_token(b"QCZ"),
	
	/// TODO.
	QDA = Self::letters_to_token(b"QDA"),
	
	/// TODO.
	QDB = Self::letters_to_token(b"QDB"),
	
	/// TODO.
	QDC = Self::letters_to_token(b"QDC"),
	
	/// TODO.
	QDD = Self::letters_to_token(b"QDD"),
	
	/// TODO.
	QDE = Self::letters_to_token(b"QDE"),
	
	/// TODO.
	QDF = Self::letters_to_token(b"QDF"),
	
	/// TODO.
	QDG = Self::letters_to_token(b"QDG"),
	
	/// TODO.
	QDH = Self::letters_to_token(b"QDH"),
	
	/// TODO.
	QDI = Self::letters_to_token(b"QDI"),
	
	/// TODO.
	QDJ = Self::letters_to_token(b"QDJ"),
	
	/// TODO.
	QDK = Self::letters_to_token(b"QDK"),
	
	/// TODO.
	QDL = Self::letters_to_token(b"QDL"),
	
	/// TODO.
	QDM = Self::letters_to_token(b"QDM"),
	
	/// TODO.
	QDN = Self::letters_to_token(b"QDN"),
	
	/// TODO.
	QDO = Self::letters_to_token(b"QDO"),
	
	/// TODO.
	QDP = Self::letters_to_token(b"QDP"),
	
	/// TODO.
	QDQ = Self::letters_to_token(b"QDQ"),
	
	/// TODO.
	QDR = Self::letters_to_token(b"QDR"),
	
	/// TODO.
	QDS = Self::letters_to_token(b"QDS"),
	
	/// TODO.
	QDT = Self::letters_to_token(b"QDT"),
	
	/// TODO.
	QDU = Self::letters_to_token(b"QDU"),
	
	/// TODO.
	QDV = Self::letters_to_token(b"QDV"),
	
	/// TODO.
	QDW = Self::letters_to_token(b"QDW"),
	
	/// TODO.
	QDX = Self::letters_to_token(b"QDX"),
	
	/// TODO.
	QDY = Self::letters_to_token(b"QDY"),
	
	/// TODO.
	QDZ = Self::letters_to_token(b"QDZ"),
	
	/// TODO.
	QEA = Self::letters_to_token(b"QEA"),
	
	/// TODO.
	QEB = Self::letters_to_token(b"QEB"),
	
	/// TODO.
	QEC = Self::letters_to_token(b"QEC"),
	
	/// TODO.
	QED = Self::letters_to_token(b"QED"),
	
	/// TODO.
	QEE = Self::letters_to_token(b"QEE"),
	
	/// TODO.
	QEF = Self::letters_to_token(b"QEF"),
	
	/// TODO.
	QEG = Self::letters_to_token(b"QEG"),
	
	/// TODO.
	QEH = Self::letters_to_token(b"QEH"),
	
	/// TODO.
	QEI = Self::letters_to_token(b"QEI"),
	
	/// TODO.
	QEJ = Self::letters_to_token(b"QEJ"),
	
	/// TODO.
	QEK = Self::letters_to_token(b"QEK"),
	
	/// TODO.
	QEL = Self::letters_to_token(b"QEL"),
	
	/// TODO.
	QEM = Self::letters_to_token(b"QEM"),
	
	/// TODO.
	QEN = Self::letters_to_token(b"QEN"),
	
	/// TODO.
	QEO = Self::letters_to_token(b"QEO"),
	
	/// TODO.
	QEP = Self::letters_to_token(b"QEP"),
	
	/// TODO.
	QEQ = Self::letters_to_token(b"QEQ"),
	
	/// TODO.
	QER = Self::letters_to_token(b"QER"),
	
	/// TODO.
	QES = Self::letters_to_token(b"QES"),
	
	/// TODO.
	QET = Self::letters_to_token(b"QET"),
	
	/// TODO.
	QEU = Self::letters_to_token(b"QEU"),
	
	/// TODO.
	QEV = Self::letters_to_token(b"QEV"),
	
	/// TODO.
	QEW = Self::letters_to_token(b"QEW"),
	
	/// TODO.
	QEX = Self::letters_to_token(b"QEX"),
	
	/// TODO.
	QEY = Self::letters_to_token(b"QEY"),
	
	/// TODO.
	QEZ = Self::letters_to_token(b"QEZ"),
	
	/// TODO.
	QFA = Self::letters_to_token(b"QFA"),
	
	/// TODO.
	QFB = Self::letters_to_token(b"QFB"),
	
	/// TODO.
	QFC = Self::letters_to_token(b"QFC"),
	
	/// TODO.
	QFD = Self::letters_to_token(b"QFD"),
	
	/// TODO.
	QFE = Self::letters_to_token(b"QFE"),
	
	/// TODO.
	QFF = Self::letters_to_token(b"QFF"),
	
	/// TODO.
	QFG = Self::letters_to_token(b"QFG"),
	
	/// TODO.
	QFH = Self::letters_to_token(b"QFH"),
	
	/// TODO.
	QFI = Self::letters_to_token(b"QFI"),
	
	/// TODO.
	QFJ = Self::letters_to_token(b"QFJ"),
	
	/// TODO.
	QFK = Self::letters_to_token(b"QFK"),
	
	/// TODO.
	QFL = Self::letters_to_token(b"QFL"),
	
	/// TODO.
	QFM = Self::letters_to_token(b"QFM"),
	
	/// TODO.
	QFN = Self::letters_to_token(b"QFN"),
	
	/// TODO.
	QFO = Self::letters_to_token(b"QFO"),
	
	/// TODO.
	QFP = Self::letters_to_token(b"QFP"),
	
	/// TODO.
	QFQ = Self::letters_to_token(b"QFQ"),
	
	/// TODO.
	QFR = Self::letters_to_token(b"QFR"),
	
	/// TODO.
	QFS = Self::letters_to_token(b"QFS"),
	
	/// TODO.
	QFT = Self::letters_to_token(b"QFT"),
	
	/// TODO.
	QFU = Self::letters_to_token(b"QFU"),
	
	/// TODO.
	QFV = Self::letters_to_token(b"QFV"),
	
	/// TODO.
	QFW = Self::letters_to_token(b"QFW"),
	
	/// TODO.
	QFX = Self::letters_to_token(b"QFX"),
	
	/// TODO.
	QFY = Self::letters_to_token(b"QFY"),
	
	/// TODO.
	QFZ = Self::letters_to_token(b"QFZ"),
	
	/// TODO.
	QGA = Self::letters_to_token(b"QGA"),
	
	/// TODO.
	QGB = Self::letters_to_token(b"QGB"),
	
	/// TODO.
	QGC = Self::letters_to_token(b"QGC"),
	
	/// TODO.
	QGD = Self::letters_to_token(b"QGD"),
	
	/// TODO.
	QGE = Self::letters_to_token(b"QGE"),
	
	/// TODO.
	QGF = Self::letters_to_token(b"QGF"),
	
	/// TODO.
	QGG = Self::letters_to_token(b"QGG"),
	
	/// TODO.
	QGH = Self::letters_to_token(b"QGH"),
	
	/// TODO.
	QGI = Self::letters_to_token(b"QGI"),
	
	/// TODO.
	QGJ = Self::letters_to_token(b"QGJ"),
	
	/// TODO.
	QGK = Self::letters_to_token(b"QGK"),
	
	/// TODO.
	QGL = Self::letters_to_token(b"QGL"),
	
	/// TODO.
	QGM = Self::letters_to_token(b"QGM"),
	
	/// TODO.
	QGN = Self::letters_to_token(b"QGN"),
	
	/// TODO.
	QGO = Self::letters_to_token(b"QGO"),
	
	/// TODO.
	QGP = Self::letters_to_token(b"QGP"),
	
	/// TODO.
	QGQ = Self::letters_to_token(b"QGQ"),
	
	/// TODO.
	QGR = Self::letters_to_token(b"QGR"),
	
	/// TODO.
	QGS = Self::letters_to_token(b"QGS"),
	
	/// TODO.
	QGT = Self::letters_to_token(b"QGT"),
	
	/// TODO.
	QGU = Self::letters_to_token(b"QGU"),
	
	/// TODO.
	QGV = Self::letters_to_token(b"QGV"),
	
	/// TODO.
	QGW = Self::letters_to_token(b"QGW"),
	
	/// TODO.
	QGX = Self::letters_to_token(b"QGX"),
	
	/// TODO.
	QGY = Self::letters_to_token(b"QGY"),
	
	/// TODO.
	QGZ = Self::letters_to_token(b"QGZ"),
	
	/// TODO.
	QHA = Self::letters_to_token(b"QHA"),
	
	/// TODO.
	QHB = Self::letters_to_token(b"QHB"),
	
	/// TODO.
	QHC = Self::letters_to_token(b"QHC"),
	
	/// TODO.
	QHD = Self::letters_to_token(b"QHD"),
	
	/// TODO.
	QHE = Self::letters_to_token(b"QHE"),
	
	/// TODO.
	QHF = Self::letters_to_token(b"QHF"),
	
	/// TODO.
	QHG = Self::letters_to_token(b"QHG"),
	
	/// TODO.
	QHH = Self::letters_to_token(b"QHH"),
	
	/// TODO.
	QHI = Self::letters_to_token(b"QHI"),
	
	/// TODO.
	QHJ = Self::letters_to_token(b"QHJ"),
	
	/// TODO.
	QHK = Self::letters_to_token(b"QHK"),
	
	/// TODO.
	QHL = Self::letters_to_token(b"QHL"),
	
	/// TODO.
	QHM = Self::letters_to_token(b"QHM"),
	
	/// TODO.
	QHN = Self::letters_to_token(b"QHN"),
	
	/// TODO.
	QHO = Self::letters_to_token(b"QHO"),
	
	/// TODO.
	QHP = Self::letters_to_token(b"QHP"),
	
	/// TODO.
	QHQ = Self::letters_to_token(b"QHQ"),
	
	/// TODO.
	QHR = Self::letters_to_token(b"QHR"),
	
	/// TODO.
	QHS = Self::letters_to_token(b"QHS"),
	
	/// TODO.
	QHT = Self::letters_to_token(b"QHT"),
	
	/// TODO.
	QHU = Self::letters_to_token(b"QHU"),
	
	/// TODO.
	QHV = Self::letters_to_token(b"QHV"),
	
	/// TODO.
	QHW = Self::letters_to_token(b"QHW"),
	
	/// TODO.
	QHX = Self::letters_to_token(b"QHX"),
	
	/// TODO.
	QHY = Self::letters_to_token(b"QHY"),
	
	/// TODO.
	QHZ = Self::letters_to_token(b"QHZ"),
	
	/// TODO.
	QIA = Self::letters_to_token(b"QIA"),
	
	/// TODO.
	QIB = Self::letters_to_token(b"QIB"),
	
	/// TODO.
	QIC = Self::letters_to_token(b"QIC"),
	
	/// TODO.
	QID = Self::letters_to_token(b"QID"),
	
	/// TODO.
	QIE = Self::letters_to_token(b"QIE"),
	
	/// TODO.
	QIF = Self::letters_to_token(b"QIF"),
	
	/// TODO.
	QIG = Self::letters_to_token(b"QIG"),
	
	/// TODO.
	QIH = Self::letters_to_token(b"QIH"),
	
	/// TODO.
	QII = Self::letters_to_token(b"QII"),
	
	/// TODO.
	QIJ = Self::letters_to_token(b"QIJ"),
	
	/// TODO.
	QIK = Self::letters_to_token(b"QIK"),
	
	/// TODO.
	QIL = Self::letters_to_token(b"QIL"),
	
	/// TODO.
	QIM = Self::letters_to_token(b"QIM"),
	
	/// TODO.
	QIN = Self::letters_to_token(b"QIN"),
	
	/// TODO.
	QIO = Self::letters_to_token(b"QIO"),
	
	/// TODO.
	QIP = Self::letters_to_token(b"QIP"),
	
	/// TODO.
	QIQ = Self::letters_to_token(b"QIQ"),
	
	/// TODO.
	QIR = Self::letters_to_token(b"QIR"),
	
	/// TODO.
	QIS = Self::letters_to_token(b"QIS"),
	
	/// TODO.
	QIT = Self::letters_to_token(b"QIT"),
	
	/// TODO.
	QIU = Self::letters_to_token(b"QIU"),
	
	/// TODO.
	QIV = Self::letters_to_token(b"QIV"),
	
	/// TODO.
	QIW = Self::letters_to_token(b"QIW"),
	
	/// TODO.
	QIX = Self::letters_to_token(b"QIX"),
	
	/// TODO.
	QIY = Self::letters_to_token(b"QIY"),
	
	/// TODO.
	QIZ = Self::letters_to_token(b"QIZ"),
	
	/// TODO.
	QJA = Self::letters_to_token(b"QJA"),
	
	/// TODO.
	QJB = Self::letters_to_token(b"QJB"),
	
	/// TODO.
	QJC = Self::letters_to_token(b"QJC"),
	
	/// TODO.
	QJD = Self::letters_to_token(b"QJD"),
	
	/// TODO.
	QJE = Self::letters_to_token(b"QJE"),
	
	/// TODO.
	QJF = Self::letters_to_token(b"QJF"),
	
	/// TODO.
	QJG = Self::letters_to_token(b"QJG"),
	
	/// TODO.
	QJH = Self::letters_to_token(b"QJH"),
	
	/// TODO.
	QJI = Self::letters_to_token(b"QJI"),
	
	/// TODO.
	QJJ = Self::letters_to_token(b"QJJ"),
	
	/// TODO.
	QJK = Self::letters_to_token(b"QJK"),
	
	/// TODO.
	QJL = Self::letters_to_token(b"QJL"),
	
	/// TODO.
	QJM = Self::letters_to_token(b"QJM"),
	
	/// TODO.
	QJN = Self::letters_to_token(b"QJN"),
	
	/// TODO.
	QJO = Self::letters_to_token(b"QJO"),
	
	/// TODO.
	QJP = Self::letters_to_token(b"QJP"),
	
	/// TODO.
	QJQ = Self::letters_to_token(b"QJQ"),
	
	/// TODO.
	QJR = Self::letters_to_token(b"QJR"),
	
	/// TODO.
	QJS = Self::letters_to_token(b"QJS"),
	
	/// TODO.
	QJT = Self::letters_to_token(b"QJT"),
	
	/// TODO.
	QJU = Self::letters_to_token(b"QJU"),
	
	/// TODO.
	QJV = Self::letters_to_token(b"QJV"),
	
	/// TODO.
	QJW = Self::letters_to_token(b"QJW"),
	
	/// TODO.
	QJX = Self::letters_to_token(b"QJX"),
	
	/// TODO.
	QJY = Self::letters_to_token(b"QJY"),
	
	/// TODO.
	QJZ = Self::letters_to_token(b"QJZ"),
	
	/// TODO.
	QKA = Self::letters_to_token(b"QKA"),
	
	/// TODO.
	QKB = Self::letters_to_token(b"QKB"),
	
	/// TODO.
	QKC = Self::letters_to_token(b"QKC"),
	
	/// TODO.
	QKD = Self::letters_to_token(b"QKD"),
	
	/// TODO.
	QKE = Self::letters_to_token(b"QKE"),
	
	/// TODO.
	QKF = Self::letters_to_token(b"QKF"),
	
	/// TODO.
	QKG = Self::letters_to_token(b"QKG"),
	
	/// TODO.
	QKH = Self::letters_to_token(b"QKH"),
	
	/// TODO.
	QKI = Self::letters_to_token(b"QKI"),
	
	/// TODO.
	QKJ = Self::letters_to_token(b"QKJ"),
	
	/// TODO.
	QKK = Self::letters_to_token(b"QKK"),
	
	/// TODO.
	QKL = Self::letters_to_token(b"QKL"),
	
	/// TODO.
	QKM = Self::letters_to_token(b"QKM"),
	
	/// TODO.
	QKN = Self::letters_to_token(b"QKN"),
	
	/// TODO.
	QKO = Self::letters_to_token(b"QKO"),
	
	/// TODO.
	QKP = Self::letters_to_token(b"QKP"),
	
	/// TODO.
	QKQ = Self::letters_to_token(b"QKQ"),
	
	/// TODO.
	QKR = Self::letters_to_token(b"QKR"),
	
	/// TODO.
	QKS = Self::letters_to_token(b"QKS"),
	
	/// TODO.
	QKT = Self::letters_to_token(b"QKT"),
	
	/// TODO.
	QKU = Self::letters_to_token(b"QKU"),
	
	/// TODO.
	QKV = Self::letters_to_token(b"QKV"),
	
	/// TODO.
	QKW = Self::letters_to_token(b"QKW"),
	
	/// TODO.
	QKX = Self::letters_to_token(b"QKX"),
	
	/// TODO.
	QKY = Self::letters_to_token(b"QKY"),
	
	/// TODO.
	QKZ = Self::letters_to_token(b"QKZ"),
	
	/// TODO.
	QLA = Self::letters_to_token(b"QLA"),
	
	/// TODO.
	QLB = Self::letters_to_token(b"QLB"),
	
	/// TODO.
	QLC = Self::letters_to_token(b"QLC"),
	
	/// TODO.
	QLD = Self::letters_to_token(b"QLD"),
	
	/// TODO.
	QLE = Self::letters_to_token(b"QLE"),
	
	/// TODO.
	QLF = Self::letters_to_token(b"QLF"),
	
	/// TODO.
	QLG = Self::letters_to_token(b"QLG"),
	
	/// TODO.
	QLH = Self::letters_to_token(b"QLH"),
	
	/// TODO.
	QLI = Self::letters_to_token(b"QLI"),
	
	/// TODO.
	QLJ = Self::letters_to_token(b"QLJ"),
	
	/// TODO.
	QLK = Self::letters_to_token(b"QLK"),
	
	/// TODO.
	QLL = Self::letters_to_token(b"QLL"),
	
	/// TODO.
	QLM = Self::letters_to_token(b"QLM"),
	
	/// TODO.
	QLN = Self::letters_to_token(b"QLN"),
	
	/// TODO.
	QLO = Self::letters_to_token(b"QLO"),
	
	/// TODO.
	QLP = Self::letters_to_token(b"QLP"),
	
	/// TODO.
	QLQ = Self::letters_to_token(b"QLQ"),
	
	/// TODO.
	QLR = Self::letters_to_token(b"QLR"),
	
	/// TODO.
	QLS = Self::letters_to_token(b"QLS"),
	
	/// TODO.
	QLT = Self::letters_to_token(b"QLT"),
	
	/// TODO.
	QLU = Self::letters_to_token(b"QLU"),
	
	/// TODO.
	QLV = Self::letters_to_token(b"QLV"),
	
	/// TODO.
	QLW = Self::letters_to_token(b"QLW"),
	
	/// TODO.
	QLX = Self::letters_to_token(b"QLX"),
	
	/// TODO.
	QLY = Self::letters_to_token(b"QLY"),
	
	/// TODO.
	QLZ = Self::letters_to_token(b"QLZ"),
	
	/// User assigned.
	QMA = Self::letters_to_token(b"QMA"),
	
	/// User assigned.
	QMB = Self::letters_to_token(b"QMB"),
	
	/// User assigned.
	QMC = Self::letters_to_token(b"QMC"),
	
	/// User assigned.
	QMD = Self::letters_to_token(b"QMD"),
	
	/// User assigned.
	QME = Self::letters_to_token(b"QME"),
	
	/// User assigned.
	QMF = Self::letters_to_token(b"QMF"),
	
	/// User assigned.
	QMG = Self::letters_to_token(b"QMG"),
	
	/// User assigned.
	QMH = Self::letters_to_token(b"QMH"),
	
	/// User assigned.
	QMI = Self::letters_to_token(b"QMI"),
	
	/// User assigned.
	QMJ = Self::letters_to_token(b"QMJ"),
	
	/// User assigned.
	QMK = Self::letters_to_token(b"QMK"),
	
	/// User assigned.
	QML = Self::letters_to_token(b"QML"),
	
	/// User assigned.
	QMM = Self::letters_to_token(b"QMM"),
	
	/// User assigned.
	QMN = Self::letters_to_token(b"QMN"),
	
	/// User assigned.
	QMO = Self::letters_to_token(b"QMO"),
	
	/// User assigned.
	QMP = Self::letters_to_token(b"QMP"),
	
	/// User assigned.
	QMQ = Self::letters_to_token(b"QMQ"),
	
	/// User assigned.
	QMR = Self::letters_to_token(b"QMR"),
	
	/// User assigned.
	QMS = Self::letters_to_token(b"QMS"),
	
	/// User assigned.
	QMT = Self::letters_to_token(b"QMT"),
	
	/// User assigned.
	QMU = Self::letters_to_token(b"QMU"),
	
	/// User assigned.
	QMV = Self::letters_to_token(b"QMV"),
	
	/// User assigned.
	QMW = Self::letters_to_token(b"QMW"),
	
	/// User assigned.
	QMX = Self::letters_to_token(b"QMX"),
	
	/// User assigned.
	QMY = Self::letters_to_token(b"QMY"),
	
	/// User assigned.
	QMZ = Self::letters_to_token(b"QMZ"),
	
	/// User assigned.
	QNA = Self::letters_to_token(b"QNA"),
	
	/// User assigned.
	QNB = Self::letters_to_token(b"QNB"),
	
	/// User assigned.
	QNC = Self::letters_to_token(b"QNC"),
	
	/// User assigned.
	QND = Self::letters_to_token(b"QND"),
	
	/// User assigned.
	QNE = Self::letters_to_token(b"QNE"),
	
	/// User assigned.
	QNF = Self::letters_to_token(b"QNF"),
	
	/// User assigned.
	QNG = Self::letters_to_token(b"QNG"),
	
	/// User assigned.
	QNH = Self::letters_to_token(b"QNH"),
	
	/// User assigned.
	QNI = Self::letters_to_token(b"QNI"),
	
	/// User assigned.
	QNJ = Self::letters_to_token(b"QNJ"),
	
	/// User assigned.
	QNK = Self::letters_to_token(b"QNK"),
	
	/// User assigned.
	QNL = Self::letters_to_token(b"QNL"),
	
	/// User assigned.
	QNM = Self::letters_to_token(b"QNM"),
	
	/// User assigned.
	QNN = Self::letters_to_token(b"QNN"),
	
	/// User assigned.
	QNO = Self::letters_to_token(b"QNO"),
	
	/// User assigned.
	QNP = Self::letters_to_token(b"QNP"),
	
	/// User assigned.
	QNQ = Self::letters_to_token(b"QNQ"),
	
	/// User assigned.
	QNR = Self::letters_to_token(b"QNR"),
	
	/// User assigned.
	QNS = Self::letters_to_token(b"QNS"),
	
	/// User assigned.
	QNT = Self::letters_to_token(b"QNT"),
	
	/// User assigned.
	QNU = Self::letters_to_token(b"QNU"),
	
	/// User assigned.
	QNV = Self::letters_to_token(b"QNV"),
	
	/// User assigned.
	QNW = Self::letters_to_token(b"QNW"),
	
	/// User assigned.
	QNX = Self::letters_to_token(b"QNX"),
	
	/// User assigned.
	QNY = Self::letters_to_token(b"QNY"),
	
	/// User assigned.
	QNZ = Self::letters_to_token(b"QNZ"),
	
	/// User assigned.
	QOA = Self::letters_to_token(b"QOA"),
	
	/// User assigned.
	QOB = Self::letters_to_token(b"QOB"),
	
	/// User assigned.
	QOC = Self::letters_to_token(b"QOC"),
	
	/// User assigned.
	QOD = Self::letters_to_token(b"QOD"),
	
	/// User assigned.
	QOE = Self::letters_to_token(b"QOE"),
	
	/// User assigned.
	QOF = Self::letters_to_token(b"QOF"),
	
	/// User assigned.
	QOG = Self::letters_to_token(b"QOG"),
	
	/// User assigned.
	QOH = Self::letters_to_token(b"QOH"),
	
	/// User assigned.
	QOI = Self::letters_to_token(b"QOI"),
	
	/// User assigned.
	QOJ = Self::letters_to_token(b"QOJ"),
	
	/// User assigned.
	QOK = Self::letters_to_token(b"QOK"),
	
	/// User assigned.
	QOL = Self::letters_to_token(b"QOL"),
	
	/// User assigned.
	QOM = Self::letters_to_token(b"QOM"),
	
	/// User assigned.
	QON = Self::letters_to_token(b"QON"),
	
	/// User assigned.
	QOO = Self::letters_to_token(b"QOO"),
	
	/// User assigned.
	QOP = Self::letters_to_token(b"QOP"),
	
	/// User assigned.
	QOQ = Self::letters_to_token(b"QOQ"),
	
	/// User assigned.
	QOR = Self::letters_to_token(b"QOR"),
	
	/// User assigned.
	QOS = Self::letters_to_token(b"QOS"),
	
	/// User assigned.
	QOT = Self::letters_to_token(b"QOT"),
	
	/// User assigned.
	QOU = Self::letters_to_token(b"QOU"),
	
	/// User assigned.
	QOV = Self::letters_to_token(b"QOV"),
	
	/// User assigned.
	QOW = Self::letters_to_token(b"QOW"),
	
	/// User assigned.
	QOX = Self::letters_to_token(b"QOX"),
	
	/// User assigned.
	QOY = Self::letters_to_token(b"QOY"),
	
	/// User assigned.
	QOZ = Self::letters_to_token(b"QOZ"),
	
	/// User assigned.
	QPA = Self::letters_to_token(b"QPA"),
	
	/// User assigned.
	QPB = Self::letters_to_token(b"QPB"),
	
	/// User assigned.
	QPC = Self::letters_to_token(b"QPC"),
	
	/// User assigned.
	QPD = Self::letters_to_token(b"QPD"),
	
	/// User assigned.
	QPE = Self::letters_to_token(b"QPE"),
	
	/// User assigned.
	QPF = Self::letters_to_token(b"QPF"),
	
	/// User assigned.
	QPG = Self::letters_to_token(b"QPG"),
	
	/// User assigned.
	QPH = Self::letters_to_token(b"QPH"),
	
	/// User assigned.
	QPI = Self::letters_to_token(b"QPI"),
	
	/// User assigned.
	QPJ = Self::letters_to_token(b"QPJ"),
	
	/// User assigned.
	QPK = Self::letters_to_token(b"QPK"),
	
	/// User assigned.
	QPL = Self::letters_to_token(b"QPL"),
	
	/// User assigned.
	QPM = Self::letters_to_token(b"QPM"),
	
	/// User assigned.
	QPN = Self::letters_to_token(b"QPN"),
	
	/// User assigned.
	QPO = Self::letters_to_token(b"QPO"),
	
	/// User assigned.
	QPP = Self::letters_to_token(b"QPP"),
	
	/// User assigned.
	QPQ = Self::letters_to_token(b"QPQ"),
	
	/// User assigned.
	QPR = Self::letters_to_token(b"QPR"),
	
	/// User assigned.
	QPS = Self::letters_to_token(b"QPS"),
	
	/// User assigned.
	QPT = Self::letters_to_token(b"QPT"),
	
	/// User assigned.
	QPU = Self::letters_to_token(b"QPU"),
	
	/// User assigned.
	QPV = Self::letters_to_token(b"QPV"),
	
	/// User assigned.
	QPW = Self::letters_to_token(b"QPW"),
	
	/// User assigned.
	QPX = Self::letters_to_token(b"QPX"),
	
	/// User assigned.
	QPY = Self::letters_to_token(b"QPY"),
	
	/// User assigned.
	QPZ = Self::letters_to_token(b"QPZ"),
	
	/// User assigned.
	QQA = Self::letters_to_token(b"QQA"),
	
	/// User assigned.
	QQB = Self::letters_to_token(b"QQB"),
	
	/// User assigned.
	QQC = Self::letters_to_token(b"QQC"),
	
	/// User assigned.
	QQD = Self::letters_to_token(b"QQD"),
	
	/// User assigned.
	QQE = Self::letters_to_token(b"QQE"),
	
	/// User assigned.
	QQF = Self::letters_to_token(b"QQF"),
	
	/// User assigned.
	QQG = Self::letters_to_token(b"QQG"),
	
	/// User assigned.
	QQH = Self::letters_to_token(b"QQH"),
	
	/// User assigned.
	QQI = Self::letters_to_token(b"QQI"),
	
	/// User assigned.
	QQJ = Self::letters_to_token(b"QQJ"),
	
	/// User assigned.
	QQK = Self::letters_to_token(b"QQK"),
	
	/// User assigned.
	QQL = Self::letters_to_token(b"QQL"),
	
	/// User assigned.
	QQM = Self::letters_to_token(b"QQM"),
	
	/// User assigned.
	QQN = Self::letters_to_token(b"QQN"),
	
	/// User assigned.
	QQO = Self::letters_to_token(b"QQO"),
	
	/// User assigned.
	QQP = Self::letters_to_token(b"QQP"),
	
	/// User assigned.
	QQQ = Self::letters_to_token(b"QQQ"),
	
	/// User assigned.
	QQR = Self::letters_to_token(b"QQR"),
	
	/// User assigned.
	QQS = Self::letters_to_token(b"QQS"),
	
	/// User assigned.
	QQT = Self::letters_to_token(b"QQT"),
	
	/// User assigned.
	QQU = Self::letters_to_token(b"QQU"),
	
	/// User assigned.
	QQV = Self::letters_to_token(b"QQV"),
	
	/// User assigned.
	QQW = Self::letters_to_token(b"QQW"),
	
	/// User assigned.
	QQX = Self::letters_to_token(b"QQX"),
	
	/// User assigned.
	QQY = Self::letters_to_token(b"QQY"),
	
	/// User assigned.
	QQZ = Self::letters_to_token(b"QQZ"),
	
	/// User assigned.
	QRA = Self::letters_to_token(b"QRA"),
	
	/// User assigned.
	QRB = Self::letters_to_token(b"QRB"),
	
	/// User assigned.
	QRC = Self::letters_to_token(b"QRC"),
	
	/// User assigned.
	QRD = Self::letters_to_token(b"QRD"),
	
	/// User assigned.
	QRE = Self::letters_to_token(b"QRE"),
	
	/// User assigned.
	QRF = Self::letters_to_token(b"QRF"),
	
	/// User assigned.
	QRG = Self::letters_to_token(b"QRG"),
	
	/// User assigned.
	QRH = Self::letters_to_token(b"QRH"),
	
	/// User assigned.
	QRI = Self::letters_to_token(b"QRI"),
	
	/// User assigned.
	QRJ = Self::letters_to_token(b"QRJ"),
	
	/// User assigned.
	QRK = Self::letters_to_token(b"QRK"),
	
	/// User assigned.
	QRL = Self::letters_to_token(b"QRL"),
	
	/// User assigned.
	QRM = Self::letters_to_token(b"QRM"),
	
	/// User assigned.
	QRN = Self::letters_to_token(b"QRN"),
	
	/// User assigned.
	QRO = Self::letters_to_token(b"QRO"),
	
	/// User assigned.
	QRP = Self::letters_to_token(b"QRP"),
	
	/// User assigned.
	QRQ = Self::letters_to_token(b"QRQ"),
	
	/// User assigned.
	QRR = Self::letters_to_token(b"QRR"),
	
	/// User assigned.
	QRS = Self::letters_to_token(b"QRS"),
	
	/// User assigned.
	QRT = Self::letters_to_token(b"QRT"),
	
	/// User assigned.
	QRU = Self::letters_to_token(b"QRU"),
	
	/// User assigned.
	QRV = Self::letters_to_token(b"QRV"),
	
	/// User assigned.
	QRW = Self::letters_to_token(b"QRW"),
	
	/// User assigned.
	QRX = Self::letters_to_token(b"QRX"),
	
	/// User assigned.
	QRY = Self::letters_to_token(b"QRY"),
	
	/// User assigned.
	QRZ = Self::letters_to_token(b"QRZ"),
	
	/// User assigned.
	QSA = Self::letters_to_token(b"QSA"),
	
	/// User assigned.
	QSB = Self::letters_to_token(b"QSB"),
	
	/// User assigned.
	QSC = Self::letters_to_token(b"QSC"),
	
	/// User assigned.
	QSD = Self::letters_to_token(b"QSD"),
	
	/// User assigned.
	QSE = Self::letters_to_token(b"QSE"),
	
	/// User assigned.
	QSF = Self::letters_to_token(b"QSF"),
	
	/// User assigned.
	QSG = Self::letters_to_token(b"QSG"),
	
	/// User assigned.
	QSH = Self::letters_to_token(b"QSH"),
	
	/// User assigned.
	QSI = Self::letters_to_token(b"QSI"),
	
	/// User assigned.
	QSJ = Self::letters_to_token(b"QSJ"),
	
	/// User assigned.
	QSK = Self::letters_to_token(b"QSK"),
	
	/// User assigned.
	QSL = Self::letters_to_token(b"QSL"),
	
	/// User assigned.
	QSM = Self::letters_to_token(b"QSM"),
	
	/// User assigned.
	QSN = Self::letters_to_token(b"QSN"),
	
	/// User assigned.
	QSO = Self::letters_to_token(b"QSO"),
	
	/// User assigned.
	QSP = Self::letters_to_token(b"QSP"),
	
	/// User assigned.
	QSQ = Self::letters_to_token(b"QSQ"),
	
	/// User assigned.
	QSR = Self::letters_to_token(b"QSR"),
	
	/// User assigned.
	QSS = Self::letters_to_token(b"QSS"),
	
	/// User assigned.
	QST = Self::letters_to_token(b"QST"),
	
	/// User assigned.
	QSU = Self::letters_to_token(b"QSU"),
	
	/// User assigned.
	QSV = Self::letters_to_token(b"QSV"),
	
	/// User assigned.
	QSW = Self::letters_to_token(b"QSW"),
	
	/// User assigned.
	QSX = Self::letters_to_token(b"QSX"),
	
	/// User assigned.
	QSY = Self::letters_to_token(b"QSY"),
	
	/// User assigned.
	QSZ = Self::letters_to_token(b"QSZ"),
	
	/// User assigned.
	QTA = Self::letters_to_token(b"QTA"),
	
	/// User assigned.
	QTB = Self::letters_to_token(b"QTB"),
	
	/// User assigned.
	QTC = Self::letters_to_token(b"QTC"),
	
	/// User assigned.
	QTD = Self::letters_to_token(b"QTD"),
	
	/// User assigned.
	QTE = Self::letters_to_token(b"QTE"),
	
	/// User assigned.
	QTF = Self::letters_to_token(b"QTF"),
	
	/// User assigned.
	QTG = Self::letters_to_token(b"QTG"),
	
	/// User assigned.
	QTH = Self::letters_to_token(b"QTH"),
	
	/// User assigned.
	QTI = Self::letters_to_token(b"QTI"),
	
	/// User assigned.
	QTJ = Self::letters_to_token(b"QTJ"),
	
	/// User assigned.
	QTK = Self::letters_to_token(b"QTK"),
	
	/// User assigned.
	QTL = Self::letters_to_token(b"QTL"),
	
	/// User assigned.
	QTM = Self::letters_to_token(b"QTM"),
	
	/// User assigned.
	QTN = Self::letters_to_token(b"QTN"),
	
	/// User assigned.
	QTO = Self::letters_to_token(b"QTO"),
	
	/// User assigned.
	QTP = Self::letters_to_token(b"QTP"),
	
	/// User assigned.
	QTQ = Self::letters_to_token(b"QTQ"),
	
	/// User assigned.
	QTR = Self::letters_to_token(b"QTR"),
	
	/// User assigned.
	QTS = Self::letters_to_token(b"QTS"),
	
	/// User assigned.
	QTT = Self::letters_to_token(b"QTT"),
	
	/// User assigned.
	QTU = Self::letters_to_token(b"QTU"),
	
	/// User assigned.
	QTV = Self::letters_to_token(b"QTV"),
	
	/// User assigned.
	QTW = Self::letters_to_token(b"QTW"),
	
	/// User assigned.
	QTX = Self::letters_to_token(b"QTX"),
	
	/// User assigned.
	QTY = Self::letters_to_token(b"QTY"),
	
	/// User assigned.
	QTZ = Self::letters_to_token(b"QTZ"),
	
	/// User assigned.
	QUA = Self::letters_to_token(b"QUA"),
	
	/// User assigned.
	QUB = Self::letters_to_token(b"QUB"),
	
	/// User assigned.
	QUC = Self::letters_to_token(b"QUC"),
	
	/// User assigned.
	QUD = Self::letters_to_token(b"QUD"),
	
	/// User assigned.
	QUE = Self::letters_to_token(b"QUE"),
	
	/// User assigned.
	QUF = Self::letters_to_token(b"QUF"),
	
	/// User assigned.
	QUG = Self::letters_to_token(b"QUG"),
	
	/// User assigned.
	QUH = Self::letters_to_token(b"QUH"),
	
	/// User assigned.
	QUI = Self::letters_to_token(b"QUI"),
	
	/// User assigned.
	QUJ = Self::letters_to_token(b"QUJ"),
	
	/// User assigned.
	QUK = Self::letters_to_token(b"QUK"),
	
	/// User assigned.
	QUL = Self::letters_to_token(b"QUL"),
	
	/// User assigned.
	QUM = Self::letters_to_token(b"QUM"),
	
	/// User assigned.
	QUN = Self::letters_to_token(b"QUN"),
	
	/// User assigned.
	QUO = Self::letters_to_token(b"QUO"),
	
	/// User assigned.
	QUP = Self::letters_to_token(b"QUP"),
	
	/// User assigned.
	QUQ = Self::letters_to_token(b"QUQ"),
	
	/// User assigned.
	QUR = Self::letters_to_token(b"QUR"),
	
	/// User assigned.
	QUS = Self::letters_to_token(b"QUS"),
	
	/// User assigned.
	QUT = Self::letters_to_token(b"QUT"),
	
	/// User assigned.
	QUU = Self::letters_to_token(b"QUU"),
	
	/// User assigned.
	QUV = Self::letters_to_token(b"QUV"),
	
	/// User assigned.
	QUW = Self::letters_to_token(b"QUW"),
	
	/// User assigned.
	QUX = Self::letters_to_token(b"QUX"),
	
	/// User assigned.
	QUY = Self::letters_to_token(b"QUY"),
	
	/// User assigned.
	QUZ = Self::letters_to_token(b"QUZ"),
	
	/// User assigned.
	QVA = Self::letters_to_token(b"QVA"),
	
	/// User assigned.
	QVB = Self::letters_to_token(b"QVB"),
	
	/// User assigned.
	QVC = Self::letters_to_token(b"QVC"),
	
	/// User assigned.
	QVD = Self::letters_to_token(b"QVD"),
	
	/// User assigned.
	QVE = Self::letters_to_token(b"QVE"),
	
	/// User assigned.
	QVF = Self::letters_to_token(b"QVF"),
	
	/// User assigned.
	QVG = Self::letters_to_token(b"QVG"),
	
	/// User assigned.
	QVH = Self::letters_to_token(b"QVH"),
	
	/// User assigned.
	QVI = Self::letters_to_token(b"QVI"),
	
	/// User assigned.
	QVJ = Self::letters_to_token(b"QVJ"),
	
	/// User assigned.
	QVK = Self::letters_to_token(b"QVK"),
	
	/// User assigned.
	QVL = Self::letters_to_token(b"QVL"),
	
	/// User assigned.
	QVM = Self::letters_to_token(b"QVM"),
	
	/// User assigned.
	QVN = Self::letters_to_token(b"QVN"),
	
	/// User assigned.
	QVO = Self::letters_to_token(b"QVO"),
	
	/// User assigned.
	QVP = Self::letters_to_token(b"QVP"),
	
	/// User assigned.
	QVQ = Self::letters_to_token(b"QVQ"),
	
	/// User assigned.
	QVR = Self::letters_to_token(b"QVR"),
	
	/// User assigned.
	QVS = Self::letters_to_token(b"QVS"),
	
	/// User assigned.
	QVT = Self::letters_to_token(b"QVT"),
	
	/// User assigned.
	QVU = Self::letters_to_token(b"QVU"),
	
	/// User assigned.
	QVV = Self::letters_to_token(b"QVV"),
	
	/// User assigned.
	QVW = Self::letters_to_token(b"QVW"),
	
	/// User assigned.
	QVX = Self::letters_to_token(b"QVX"),
	
	/// User assigned.
	QVY = Self::letters_to_token(b"QVY"),
	
	/// User assigned.
	QVZ = Self::letters_to_token(b"QVZ"),
	
	/// User assigned.
	QWA = Self::letters_to_token(b"QWA"),
	
	/// User assigned.
	QWB = Self::letters_to_token(b"QWB"),
	
	/// User assigned.
	QWC = Self::letters_to_token(b"QWC"),
	
	/// User assigned.
	QWD = Self::letters_to_token(b"QWD"),
	
	/// User assigned.
	QWE = Self::letters_to_token(b"QWE"),
	
	/// User assigned.
	QWF = Self::letters_to_token(b"QWF"),
	
	/// User assigned.
	QWG = Self::letters_to_token(b"QWG"),
	
	/// User assigned.
	QWH = Self::letters_to_token(b"QWH"),
	
	/// User assigned.
	QWI = Self::letters_to_token(b"QWI"),
	
	/// User assigned.
	QWJ = Self::letters_to_token(b"QWJ"),
	
	/// User assigned.
	QWK = Self::letters_to_token(b"QWK"),
	
	/// User assigned.
	QWL = Self::letters_to_token(b"QWL"),
	
	/// User assigned.
	QWM = Self::letters_to_token(b"QWM"),
	
	/// User assigned.
	QWN = Self::letters_to_token(b"QWN"),
	
	/// User assigned.
	QWO = Self::letters_to_token(b"QWO"),
	
	/// User assigned.
	QWP = Self::letters_to_token(b"QWP"),
	
	/// User assigned.
	QWQ = Self::letters_to_token(b"QWQ"),
	
	/// User assigned.
	QWR = Self::letters_to_token(b"QWR"),
	
	/// User assigned.
	QWS = Self::letters_to_token(b"QWS"),
	
	/// User assigned.
	QWT = Self::letters_to_token(b"QWT"),
	
	/// User assigned.
	QWU = Self::letters_to_token(b"QWU"),
	
	/// User assigned.
	QWV = Self::letters_to_token(b"QWV"),
	
	/// User assigned.
	QWW = Self::letters_to_token(b"QWW"),
	
	/// User assigned.
	QWX = Self::letters_to_token(b"QWX"),
	
	/// User assigned.
	QWY = Self::letters_to_token(b"QWY"),
	
	/// User assigned.
	QWZ = Self::letters_to_token(b"QWZ"),
	
	/// User assigned.
	QXA = Self::letters_to_token(b"QXA"),
	
	/// User assigned.
	QXB = Self::letters_to_token(b"QXB"),
	
	/// User assigned.
	QXC = Self::letters_to_token(b"QXC"),
	
	/// User assigned.
	QXD = Self::letters_to_token(b"QXD"),
	
	/// User assigned.
	QXE = Self::letters_to_token(b"QXE"),
	
	/// User assigned.
	QXF = Self::letters_to_token(b"QXF"),
	
	/// User assigned.
	QXG = Self::letters_to_token(b"QXG"),
	
	/// User assigned.
	QXH = Self::letters_to_token(b"QXH"),
	
	/// User assigned.
	QXI = Self::letters_to_token(b"QXI"),
	
	/// User assigned.
	QXJ = Self::letters_to_token(b"QXJ"),
	
	/// User assigned.
	QXK = Self::letters_to_token(b"QXK"),
	
	/// User assigned.
	QXL = Self::letters_to_token(b"QXL"),
	
	/// User assigned.
	QXM = Self::letters_to_token(b"QXM"),
	
	/// User assigned.
	QXN = Self::letters_to_token(b"QXN"),
	
	/// User assigned.
	QXO = Self::letters_to_token(b"QXO"),
	
	/// User assigned.
	QXP = Self::letters_to_token(b"QXP"),
	
	/// User assigned.
	QXQ = Self::letters_to_token(b"QXQ"),
	
	/// User assigned.
	QXR = Self::letters_to_token(b"QXR"),
	
	/// User assigned.
	QXS = Self::letters_to_token(b"QXS"),
	
	/// User assigned.
	QXT = Self::letters_to_token(b"QXT"),
	
	/// User assigned.
	QXU = Self::letters_to_token(b"QXU"),
	
	/// User assigned.
	QXV = Self::letters_to_token(b"QXV"),
	
	/// User assigned.
	QXW = Self::letters_to_token(b"QXW"),
	
	/// User assigned.
	QXX = Self::letters_to_token(b"QXX"),
	
	/// User assigned.
	QXY = Self::letters_to_token(b"QXY"),
	
	/// User assigned.
	QXZ = Self::letters_to_token(b"QXZ"),
	
	/// User assigned.
	QYA = Self::letters_to_token(b"QYA"),
	
	/// User assigned.
	QYB = Self::letters_to_token(b"QYB"),
	
	/// User assigned.
	QYC = Self::letters_to_token(b"QYC"),
	
	/// User assigned.
	QYD = Self::letters_to_token(b"QYD"),
	
	/// User assigned.
	QYE = Self::letters_to_token(b"QYE"),
	
	/// User assigned.
	QYF = Self::letters_to_token(b"QYF"),
	
	/// User assigned.
	QYG = Self::letters_to_token(b"QYG"),
	
	/// User assigned.
	QYH = Self::letters_to_token(b"QYH"),
	
	/// User assigned.
	QYI = Self::letters_to_token(b"QYI"),
	
	/// User assigned.
	QYJ = Self::letters_to_token(b"QYJ"),
	
	/// User assigned.
	QYK = Self::letters_to_token(b"QYK"),
	
	/// User assigned.
	QYL = Self::letters_to_token(b"QYL"),
	
	/// User assigned.
	QYM = Self::letters_to_token(b"QYM"),
	
	/// User assigned.
	QYN = Self::letters_to_token(b"QYN"),
	
	/// User assigned.
	QYO = Self::letters_to_token(b"QYO"),
	
	/// User assigned.
	QYP = Self::letters_to_token(b"QYP"),
	
	/// User assigned.
	QYQ = Self::letters_to_token(b"QYQ"),
	
	/// User assigned.
	QYR = Self::letters_to_token(b"QYR"),
	
	/// User assigned.
	QYS = Self::letters_to_token(b"QYS"),
	
	/// User assigned.
	QYT = Self::letters_to_token(b"QYT"),
	
	/// User assigned.
	QYU = Self::letters_to_token(b"QYU"),
	
	/// User assigned.
	QYV = Self::letters_to_token(b"QYV"),
	
	/// User assigned.
	QYW = Self::letters_to_token(b"QYW"),
	
	/// User assigned.
	QYX = Self::letters_to_token(b"QYX"),
	
	/// User assigned.
	QYY = Self::letters_to_token(b"QYY"),
	
	/// User assigned.
	QYZ = Self::letters_to_token(b"QYZ"),
	
	/// User assigned.
	QZA = Self::letters_to_token(b"QZA"),
	
	/// User assigned.
	QZB = Self::letters_to_token(b"QZB"),
	
	/// User assigned.
	QZC = Self::letters_to_token(b"QZC"),
	
	/// User assigned.
	QZD = Self::letters_to_token(b"QZD"),
	
	/// User assigned.
	QZE = Self::letters_to_token(b"QZE"),
	
	/// User assigned.
	QZF = Self::letters_to_token(b"QZF"),
	
	/// User assigned.
	QZG = Self::letters_to_token(b"QZG"),
	
	/// User assigned.
	QZH = Self::letters_to_token(b"QZH"),
	
	/// User assigned.
	QZI = Self::letters_to_token(b"QZI"),
	
	/// User assigned.
	QZJ = Self::letters_to_token(b"QZJ"),
	
	/// User assigned.
	QZK = Self::letters_to_token(b"QZK"),
	
	/// User assigned.
	QZL = Self::letters_to_token(b"QZL"),
	
	/// User assigned.
	QZM = Self::letters_to_token(b"QZM"),
	
	/// User assigned.
	QZN = Self::letters_to_token(b"QZN"),
	
	/// User assigned.
	QZO = Self::letters_to_token(b"QZO"),
	
	/// User assigned.
	QZP = Self::letters_to_token(b"QZP"),
	
	/// User assigned.
	QZQ = Self::letters_to_token(b"QZQ"),
	
	/// User assigned.
	QZR = Self::letters_to_token(b"QZR"),
	
	/// User assigned.
	QZS = Self::letters_to_token(b"QZS"),
	
	/// User assigned.
	QZT = Self::letters_to_token(b"QZT"),
	
	/// User assigned.
	QZU = Self::letters_to_token(b"QZU"),
	
	/// User assigned.
	QZV = Self::letters_to_token(b"QZV"),
	
	/// User assigned.
	QZW = Self::letters_to_token(b"QZW"),
	
	/// User assigned.
	QZX = Self::letters_to_token(b"QZX"),
	
	/// User assigned.
	QZY = Self::letters_to_token(b"QZY"),
	
	/// User assigned.
	QZZ = Self::letters_to_token(b"QZZ"),
	
	/// TODO.
	RAA = Self::letters_to_token(b"RAA"),
	
	/// TODO.
	RAB = Self::letters_to_token(b"RAB"),
	
	/// TODO.
	RAC = Self::letters_to_token(b"RAC"),
	
	/// TODO.
	RAD = Self::letters_to_token(b"RAD"),
	
	/// TODO.
	RAE = Self::letters_to_token(b"RAE"),
	
	/// TODO.
	RAF = Self::letters_to_token(b"RAF"),
	
	/// TODO.
	RAG = Self::letters_to_token(b"RAG"),
	
	/// TODO.
	RAH = Self::letters_to_token(b"RAH"),
	
	/// TODO.
	RAI = Self::letters_to_token(b"RAI"),
	
	/// TODO.
	RAJ = Self::letters_to_token(b"RAJ"),
	
	/// TODO.
	RAK = Self::letters_to_token(b"RAK"),
	
	/// TODO.
	RAL = Self::letters_to_token(b"RAL"),
	
	/// TODO.
	RAM = Self::letters_to_token(b"RAM"),
	
	/// TODO.
	RAN = Self::letters_to_token(b"RAN"),
	
	/// TODO.
	RAO = Self::letters_to_token(b"RAO"),
	
	/// TODO.
	RAP = Self::letters_to_token(b"RAP"),
	
	/// TODO.
	RAQ = Self::letters_to_token(b"RAQ"),
	
	/// TODO.
	RAR = Self::letters_to_token(b"RAR"),
	
	/// TODO.
	RAS = Self::letters_to_token(b"RAS"),
	
	/// TODO.
	RAT = Self::letters_to_token(b"RAT"),
	
	/// TODO.
	RAU = Self::letters_to_token(b"RAU"),
	
	/// TODO.
	RAV = Self::letters_to_token(b"RAV"),
	
	/// TODO.
	RAW = Self::letters_to_token(b"RAW"),
	
	/// TODO.
	RAX = Self::letters_to_token(b"RAX"),
	
	/// TODO.
	RAY = Self::letters_to_token(b"RAY"),
	
	/// TODO.
	RAZ = Self::letters_to_token(b"RAZ"),
	
	/// TODO.
	RBA = Self::letters_to_token(b"RBA"),
	
	/// TODO.
	RBB = Self::letters_to_token(b"RBB"),
	
	/// TODO.
	RBC = Self::letters_to_token(b"RBC"),
	
	/// TODO.
	RBD = Self::letters_to_token(b"RBD"),
	
	/// TODO.
	RBE = Self::letters_to_token(b"RBE"),
	
	/// TODO.
	RBF = Self::letters_to_token(b"RBF"),
	
	/// TODO.
	RBG = Self::letters_to_token(b"RBG"),
	
	/// TODO.
	RBH = Self::letters_to_token(b"RBH"),
	
	/// TODO.
	RBI = Self::letters_to_token(b"RBI"),
	
	/// TODO.
	RBJ = Self::letters_to_token(b"RBJ"),
	
	/// TODO.
	RBK = Self::letters_to_token(b"RBK"),
	
	/// TODO.
	RBL = Self::letters_to_token(b"RBL"),
	
	/// TODO.
	RBM = Self::letters_to_token(b"RBM"),
	
	/// TODO.
	RBN = Self::letters_to_token(b"RBN"),
	
	/// TODO.
	RBO = Self::letters_to_token(b"RBO"),
	
	/// TODO.
	RBP = Self::letters_to_token(b"RBP"),
	
	/// TODO.
	RBQ = Self::letters_to_token(b"RBQ"),
	
	/// TODO.
	RBR = Self::letters_to_token(b"RBR"),
	
	/// TODO.
	RBS = Self::letters_to_token(b"RBS"),
	
	/// TODO.
	RBT = Self::letters_to_token(b"RBT"),
	
	/// TODO.
	RBU = Self::letters_to_token(b"RBU"),
	
	/// TODO.
	RBV = Self::letters_to_token(b"RBV"),
	
	/// TODO.
	RBW = Self::letters_to_token(b"RBW"),
	
	/// TODO.
	RBX = Self::letters_to_token(b"RBX"),
	
	/// TODO.
	RBY = Self::letters_to_token(b"RBY"),
	
	/// TODO.
	RBZ = Self::letters_to_token(b"RBZ"),
	
	/// TODO.
	RCA = Self::letters_to_token(b"RCA"),
	
	/// TODO.
	RCB = Self::letters_to_token(b"RCB"),
	
	/// TODO.
	RCC = Self::letters_to_token(b"RCC"),
	
	/// TODO.
	RCD = Self::letters_to_token(b"RCD"),
	
	/// TODO.
	RCE = Self::letters_to_token(b"RCE"),
	
	/// TODO.
	RCF = Self::letters_to_token(b"RCF"),
	
	/// TODO.
	RCG = Self::letters_to_token(b"RCG"),
	
	/// TODO.
	RCH = Self::letters_to_token(b"RCH"),
	
	/// TODO.
	RCI = Self::letters_to_token(b"RCI"),
	
	/// TODO.
	RCJ = Self::letters_to_token(b"RCJ"),
	
	/// TODO.
	RCK = Self::letters_to_token(b"RCK"),
	
	/// TODO.
	RCL = Self::letters_to_token(b"RCL"),
	
	/// TODO.
	RCM = Self::letters_to_token(b"RCM"),
	
	/// TODO.
	RCN = Self::letters_to_token(b"RCN"),
	
	/// TODO.
	RCO = Self::letters_to_token(b"RCO"),
	
	/// TODO.
	RCP = Self::letters_to_token(b"RCP"),
	
	/// TODO.
	RCQ = Self::letters_to_token(b"RCQ"),
	
	/// TODO.
	RCR = Self::letters_to_token(b"RCR"),
	
	/// TODO.
	RCS = Self::letters_to_token(b"RCS"),
	
	/// TODO.
	RCT = Self::letters_to_token(b"RCT"),
	
	/// TODO.
	RCU = Self::letters_to_token(b"RCU"),
	
	/// TODO.
	RCV = Self::letters_to_token(b"RCV"),
	
	/// TODO.
	RCW = Self::letters_to_token(b"RCW"),
	
	/// TODO.
	RCX = Self::letters_to_token(b"RCX"),
	
	/// TODO.
	RCY = Self::letters_to_token(b"RCY"),
	
	/// TODO.
	RCZ = Self::letters_to_token(b"RCZ"),
	
	/// TODO.
	RDA = Self::letters_to_token(b"RDA"),
	
	/// TODO.
	RDB = Self::letters_to_token(b"RDB"),
	
	/// TODO.
	RDC = Self::letters_to_token(b"RDC"),
	
	/// TODO.
	RDD = Self::letters_to_token(b"RDD"),
	
	/// TODO.
	RDE = Self::letters_to_token(b"RDE"),
	
	/// TODO.
	RDF = Self::letters_to_token(b"RDF"),
	
	/// TODO.
	RDG = Self::letters_to_token(b"RDG"),
	
	/// TODO.
	RDH = Self::letters_to_token(b"RDH"),
	
	/// TODO.
	RDI = Self::letters_to_token(b"RDI"),
	
	/// TODO.
	RDJ = Self::letters_to_token(b"RDJ"),
	
	/// TODO.
	RDK = Self::letters_to_token(b"RDK"),
	
	/// TODO.
	RDL = Self::letters_to_token(b"RDL"),
	
	/// TODO.
	RDM = Self::letters_to_token(b"RDM"),
	
	/// TODO.
	RDN = Self::letters_to_token(b"RDN"),
	
	/// TODO.
	RDO = Self::letters_to_token(b"RDO"),
	
	/// TODO.
	RDP = Self::letters_to_token(b"RDP"),
	
	/// TODO.
	RDQ = Self::letters_to_token(b"RDQ"),
	
	/// TODO.
	RDR = Self::letters_to_token(b"RDR"),
	
	/// TODO.
	RDS = Self::letters_to_token(b"RDS"),
	
	/// TODO.
	RDT = Self::letters_to_token(b"RDT"),
	
	/// TODO.
	RDU = Self::letters_to_token(b"RDU"),
	
	/// TODO.
	RDV = Self::letters_to_token(b"RDV"),
	
	/// TODO.
	RDW = Self::letters_to_token(b"RDW"),
	
	/// TODO.
	RDX = Self::letters_to_token(b"RDX"),
	
	/// TODO.
	RDY = Self::letters_to_token(b"RDY"),
	
	/// TODO.
	RDZ = Self::letters_to_token(b"RDZ"),
	
	/// TODO.
	REA = Self::letters_to_token(b"REA"),
	
	/// TODO.
	REB = Self::letters_to_token(b"REB"),
	
	/// TODO.
	REC = Self::letters_to_token(b"REC"),
	
	/// TODO.
	RED = Self::letters_to_token(b"RED"),
	
	/// TODO.
	REE = Self::letters_to_token(b"REE"),
	
	/// TODO.
	REF = Self::letters_to_token(b"REF"),
	
	/// TODO.
	REG = Self::letters_to_token(b"REG"),
	
	/// TODO.
	REH = Self::letters_to_token(b"REH"),
	
	/// TODO.
	REI = Self::letters_to_token(b"REI"),
	
	/// TODO.
	REJ = Self::letters_to_token(b"REJ"),
	
	/// TODO.
	REK = Self::letters_to_token(b"REK"),
	
	/// TODO.
	REL = Self::letters_to_token(b"REL"),
	
	/// TODO.
	REM = Self::letters_to_token(b"REM"),
	
	/// TODO.
	REN = Self::letters_to_token(b"REN"),
	
	/// TODO.
	REO = Self::letters_to_token(b"REO"),
	
	/// TODO.
	REP = Self::letters_to_token(b"REP"),
	
	/// TODO.
	REQ = Self::letters_to_token(b"REQ"),
	
	/// TODO.
	RER = Self::letters_to_token(b"RER"),
	
	/// TODO.
	RES = Self::letters_to_token(b"RES"),
	
	/// TODO.
	RET = Self::letters_to_token(b"RET"),
	
	/// TODO.
	REU = Self::letters_to_token(b"REU"),
	
	/// TODO.
	REV = Self::letters_to_token(b"REV"),
	
	/// TODO.
	REW = Self::letters_to_token(b"REW"),
	
	/// TODO.
	REX = Self::letters_to_token(b"REX"),
	
	/// TODO.
	REY = Self::letters_to_token(b"REY"),
	
	/// TODO.
	REZ = Self::letters_to_token(b"REZ"),
	
	/// TODO.
	RFA = Self::letters_to_token(b"RFA"),
	
	/// TODO.
	RFB = Self::letters_to_token(b"RFB"),
	
	/// TODO.
	RFC = Self::letters_to_token(b"RFC"),
	
	/// TODO.
	RFD = Self::letters_to_token(b"RFD"),
	
	/// TODO.
	RFE = Self::letters_to_token(b"RFE"),
	
	/// TODO.
	RFF = Self::letters_to_token(b"RFF"),
	
	/// TODO.
	RFG = Self::letters_to_token(b"RFG"),
	
	/// TODO.
	RFH = Self::letters_to_token(b"RFH"),
	
	/// TODO.
	RFI = Self::letters_to_token(b"RFI"),
	
	/// TODO.
	RFJ = Self::letters_to_token(b"RFJ"),
	
	/// TODO.
	RFK = Self::letters_to_token(b"RFK"),
	
	/// TODO.
	RFL = Self::letters_to_token(b"RFL"),
	
	/// TODO.
	RFM = Self::letters_to_token(b"RFM"),
	
	/// TODO.
	RFN = Self::letters_to_token(b"RFN"),
	
	/// TODO.
	RFO = Self::letters_to_token(b"RFO"),
	
	/// TODO.
	RFP = Self::letters_to_token(b"RFP"),
	
	/// TODO.
	RFQ = Self::letters_to_token(b"RFQ"),
	
	/// TODO.
	RFR = Self::letters_to_token(b"RFR"),
	
	/// TODO.
	RFS = Self::letters_to_token(b"RFS"),
	
	/// TODO.
	RFT = Self::letters_to_token(b"RFT"),
	
	/// TODO.
	RFU = Self::letters_to_token(b"RFU"),
	
	/// TODO.
	RFV = Self::letters_to_token(b"RFV"),
	
	/// TODO.
	RFW = Self::letters_to_token(b"RFW"),
	
	/// TODO.
	RFX = Self::letters_to_token(b"RFX"),
	
	/// TODO.
	RFY = Self::letters_to_token(b"RFY"),
	
	/// TODO.
	RFZ = Self::letters_to_token(b"RFZ"),
	
	/// TODO.
	RGA = Self::letters_to_token(b"RGA"),
	
	/// TODO.
	RGB = Self::letters_to_token(b"RGB"),
	
	/// TODO.
	RGC = Self::letters_to_token(b"RGC"),
	
	/// TODO.
	RGD = Self::letters_to_token(b"RGD"),
	
	/// TODO.
	RGE = Self::letters_to_token(b"RGE"),
	
	/// TODO.
	RGF = Self::letters_to_token(b"RGF"),
	
	/// TODO.
	RGG = Self::letters_to_token(b"RGG"),
	
	/// TODO.
	RGH = Self::letters_to_token(b"RGH"),
	
	/// TODO.
	RGI = Self::letters_to_token(b"RGI"),
	
	/// TODO.
	RGJ = Self::letters_to_token(b"RGJ"),
	
	/// TODO.
	RGK = Self::letters_to_token(b"RGK"),
	
	/// TODO.
	RGL = Self::letters_to_token(b"RGL"),
	
	/// TODO.
	RGM = Self::letters_to_token(b"RGM"),
	
	/// TODO.
	RGN = Self::letters_to_token(b"RGN"),
	
	/// TODO.
	RGO = Self::letters_to_token(b"RGO"),
	
	/// TODO.
	RGP = Self::letters_to_token(b"RGP"),
	
	/// TODO.
	RGQ = Self::letters_to_token(b"RGQ"),
	
	/// TODO.
	RGR = Self::letters_to_token(b"RGR"),
	
	/// TODO.
	RGS = Self::letters_to_token(b"RGS"),
	
	/// TODO.
	RGT = Self::letters_to_token(b"RGT"),
	
	/// TODO.
	RGU = Self::letters_to_token(b"RGU"),
	
	/// TODO.
	RGV = Self::letters_to_token(b"RGV"),
	
	/// TODO.
	RGW = Self::letters_to_token(b"RGW"),
	
	/// TODO.
	RGX = Self::letters_to_token(b"RGX"),
	
	/// TODO.
	RGY = Self::letters_to_token(b"RGY"),
	
	/// TODO.
	RGZ = Self::letters_to_token(b"RGZ"),
	
	/// TODO.
	RHA = Self::letters_to_token(b"RHA"),
	
	/// TODO.
	RHB = Self::letters_to_token(b"RHB"),
	
	/// TODO.
	RHC = Self::letters_to_token(b"RHC"),
	
	/// TODO.
	RHD = Self::letters_to_token(b"RHD"),
	
	/// TODO.
	RHE = Self::letters_to_token(b"RHE"),
	
	/// TODO.
	RHF = Self::letters_to_token(b"RHF"),
	
	/// TODO.
	RHG = Self::letters_to_token(b"RHG"),
	
	/// TODO.
	RHH = Self::letters_to_token(b"RHH"),
	
	/// TODO.
	RHI = Self::letters_to_token(b"RHI"),
	
	/// TODO.
	RHJ = Self::letters_to_token(b"RHJ"),
	
	/// TODO.
	RHK = Self::letters_to_token(b"RHK"),
	
	/// TODO.
	RHL = Self::letters_to_token(b"RHL"),
	
	/// TODO.
	RHM = Self::letters_to_token(b"RHM"),
	
	/// TODO.
	RHN = Self::letters_to_token(b"RHN"),
	
	/// TODO.
	RHO = Self::letters_to_token(b"RHO"),
	
	/// TODO.
	RHP = Self::letters_to_token(b"RHP"),
	
	/// TODO.
	RHQ = Self::letters_to_token(b"RHQ"),
	
	/// TODO.
	RHR = Self::letters_to_token(b"RHR"),
	
	/// TODO.
	RHS = Self::letters_to_token(b"RHS"),
	
	/// TODO.
	RHT = Self::letters_to_token(b"RHT"),
	
	/// TODO.
	RHU = Self::letters_to_token(b"RHU"),
	
	/// TODO.
	RHV = Self::letters_to_token(b"RHV"),
	
	/// TODO.
	RHW = Self::letters_to_token(b"RHW"),
	
	/// TODO.
	RHX = Self::letters_to_token(b"RHX"),
	
	/// TODO.
	RHY = Self::letters_to_token(b"RHY"),
	
	/// TODO.
	RHZ = Self::letters_to_token(b"RHZ"),
	
	/// TODO.
	RIA = Self::letters_to_token(b"RIA"),
	
	/// TODO.
	RIB = Self::letters_to_token(b"RIB"),
	
	/// TODO.
	RIC = Self::letters_to_token(b"RIC"),
	
	/// TODO.
	RID = Self::letters_to_token(b"RID"),
	
	/// TODO.
	RIE = Self::letters_to_token(b"RIE"),
	
	/// TODO.
	RIF = Self::letters_to_token(b"RIF"),
	
	/// TODO.
	RIG = Self::letters_to_token(b"RIG"),
	
	/// TODO.
	RIH = Self::letters_to_token(b"RIH"),
	
	/// TODO.
	RII = Self::letters_to_token(b"RII"),
	
	/// TODO.
	RIJ = Self::letters_to_token(b"RIJ"),
	
	/// TODO.
	RIK = Self::letters_to_token(b"RIK"),
	
	/// TODO.
	RIL = Self::letters_to_token(b"RIL"),
	
	/// TODO.
	RIM = Self::letters_to_token(b"RIM"),
	
	/// TODO.
	RIN = Self::letters_to_token(b"RIN"),
	
	/// TODO.
	RIO = Self::letters_to_token(b"RIO"),
	
	/// TODO.
	RIP = Self::letters_to_token(b"RIP"),
	
	/// TODO.
	RIQ = Self::letters_to_token(b"RIQ"),
	
	/// TODO.
	RIR = Self::letters_to_token(b"RIR"),
	
	/// TODO.
	RIS = Self::letters_to_token(b"RIS"),
	
	/// TODO.
	RIT = Self::letters_to_token(b"RIT"),
	
	/// TODO.
	RIU = Self::letters_to_token(b"RIU"),
	
	/// TODO.
	RIV = Self::letters_to_token(b"RIV"),
	
	/// TODO.
	RIW = Self::letters_to_token(b"RIW"),
	
	/// TODO.
	RIX = Self::letters_to_token(b"RIX"),
	
	/// TODO.
	RIY = Self::letters_to_token(b"RIY"),
	
	/// TODO.
	RIZ = Self::letters_to_token(b"RIZ"),
	
	/// TODO.
	RJA = Self::letters_to_token(b"RJA"),
	
	/// TODO.
	RJB = Self::letters_to_token(b"RJB"),
	
	/// TODO.
	RJC = Self::letters_to_token(b"RJC"),
	
	/// TODO.
	RJD = Self::letters_to_token(b"RJD"),
	
	/// TODO.
	RJE = Self::letters_to_token(b"RJE"),
	
	/// TODO.
	RJF = Self::letters_to_token(b"RJF"),
	
	/// TODO.
	RJG = Self::letters_to_token(b"RJG"),
	
	/// TODO.
	RJH = Self::letters_to_token(b"RJH"),
	
	/// TODO.
	RJI = Self::letters_to_token(b"RJI"),
	
	/// TODO.
	RJJ = Self::letters_to_token(b"RJJ"),
	
	/// TODO.
	RJK = Self::letters_to_token(b"RJK"),
	
	/// TODO.
	RJL = Self::letters_to_token(b"RJL"),
	
	/// TODO.
	RJM = Self::letters_to_token(b"RJM"),
	
	/// TODO.
	RJN = Self::letters_to_token(b"RJN"),
	
	/// TODO.
	RJO = Self::letters_to_token(b"RJO"),
	
	/// TODO.
	RJP = Self::letters_to_token(b"RJP"),
	
	/// TODO.
	RJQ = Self::letters_to_token(b"RJQ"),
	
	/// TODO.
	RJR = Self::letters_to_token(b"RJR"),
	
	/// TODO.
	RJS = Self::letters_to_token(b"RJS"),
	
	/// TODO.
	RJT = Self::letters_to_token(b"RJT"),
	
	/// TODO.
	RJU = Self::letters_to_token(b"RJU"),
	
	/// TODO.
	RJV = Self::letters_to_token(b"RJV"),
	
	/// TODO.
	RJW = Self::letters_to_token(b"RJW"),
	
	/// TODO.
	RJX = Self::letters_to_token(b"RJX"),
	
	/// TODO.
	RJY = Self::letters_to_token(b"RJY"),
	
	/// TODO.
	RJZ = Self::letters_to_token(b"RJZ"),
	
	/// TODO.
	RKA = Self::letters_to_token(b"RKA"),
	
	/// TODO.
	RKB = Self::letters_to_token(b"RKB"),
	
	/// TODO.
	RKC = Self::letters_to_token(b"RKC"),
	
	/// TODO.
	RKD = Self::letters_to_token(b"RKD"),
	
	/// TODO.
	RKE = Self::letters_to_token(b"RKE"),
	
	/// TODO.
	RKF = Self::letters_to_token(b"RKF"),
	
	/// TODO.
	RKG = Self::letters_to_token(b"RKG"),
	
	/// TODO.
	RKH = Self::letters_to_token(b"RKH"),
	
	/// TODO.
	RKI = Self::letters_to_token(b"RKI"),
	
	/// TODO.
	RKJ = Self::letters_to_token(b"RKJ"),
	
	/// TODO.
	RKK = Self::letters_to_token(b"RKK"),
	
	/// TODO.
	RKL = Self::letters_to_token(b"RKL"),
	
	/// TODO.
	RKM = Self::letters_to_token(b"RKM"),
	
	/// TODO.
	RKN = Self::letters_to_token(b"RKN"),
	
	/// TODO.
	RKO = Self::letters_to_token(b"RKO"),
	
	/// TODO.
	RKP = Self::letters_to_token(b"RKP"),
	
	/// TODO.
	RKQ = Self::letters_to_token(b"RKQ"),
	
	/// TODO.
	RKR = Self::letters_to_token(b"RKR"),
	
	/// TODO.
	RKS = Self::letters_to_token(b"RKS"),
	
	/// TODO.
	RKT = Self::letters_to_token(b"RKT"),
	
	/// TODO.
	RKU = Self::letters_to_token(b"RKU"),
	
	/// TODO.
	RKV = Self::letters_to_token(b"RKV"),
	
	/// TODO.
	RKW = Self::letters_to_token(b"RKW"),
	
	/// TODO.
	RKX = Self::letters_to_token(b"RKX"),
	
	/// TODO.
	RKY = Self::letters_to_token(b"RKY"),
	
	/// TODO.
	RKZ = Self::letters_to_token(b"RKZ"),
	
	/// TODO.
	RLA = Self::letters_to_token(b"RLA"),
	
	/// TODO.
	RLB = Self::letters_to_token(b"RLB"),
	
	/// TODO.
	RLC = Self::letters_to_token(b"RLC"),
	
	/// TODO.
	RLD = Self::letters_to_token(b"RLD"),
	
	/// TODO.
	RLE = Self::letters_to_token(b"RLE"),
	
	/// TODO.
	RLF = Self::letters_to_token(b"RLF"),
	
	/// TODO.
	RLG = Self::letters_to_token(b"RLG"),
	
	/// TODO.
	RLH = Self::letters_to_token(b"RLH"),
	
	/// TODO.
	RLI = Self::letters_to_token(b"RLI"),
	
	/// TODO.
	RLJ = Self::letters_to_token(b"RLJ"),
	
	/// TODO.
	RLK = Self::letters_to_token(b"RLK"),
	
	/// TODO.
	RLL = Self::letters_to_token(b"RLL"),
	
	/// TODO.
	RLM = Self::letters_to_token(b"RLM"),
	
	/// TODO.
	RLN = Self::letters_to_token(b"RLN"),
	
	/// TODO.
	RLO = Self::letters_to_token(b"RLO"),
	
	/// TODO.
	RLP = Self::letters_to_token(b"RLP"),
	
	/// TODO.
	RLQ = Self::letters_to_token(b"RLQ"),
	
	/// TODO.
	RLR = Self::letters_to_token(b"RLR"),
	
	/// TODO.
	RLS = Self::letters_to_token(b"RLS"),
	
	/// TODO.
	RLT = Self::letters_to_token(b"RLT"),
	
	/// TODO.
	RLU = Self::letters_to_token(b"RLU"),
	
	/// TODO.
	RLV = Self::letters_to_token(b"RLV"),
	
	/// TODO.
	RLW = Self::letters_to_token(b"RLW"),
	
	/// TODO.
	RLX = Self::letters_to_token(b"RLX"),
	
	/// TODO.
	RLY = Self::letters_to_token(b"RLY"),
	
	/// TODO.
	RLZ = Self::letters_to_token(b"RLZ"),
	
	/// TODO.
	RMA = Self::letters_to_token(b"RMA"),
	
	/// TODO.
	RMB = Self::letters_to_token(b"RMB"),
	
	/// TODO.
	RMC = Self::letters_to_token(b"RMC"),
	
	/// TODO.
	RMD = Self::letters_to_token(b"RMD"),
	
	/// TODO.
	RME = Self::letters_to_token(b"RME"),
	
	/// TODO.
	RMF = Self::letters_to_token(b"RMF"),
	
	/// TODO.
	RMG = Self::letters_to_token(b"RMG"),
	
	/// TODO.
	RMH = Self::letters_to_token(b"RMH"),
	
	/// TODO.
	RMI = Self::letters_to_token(b"RMI"),
	
	/// TODO.
	RMJ = Self::letters_to_token(b"RMJ"),
	
	/// TODO.
	RMK = Self::letters_to_token(b"RMK"),
	
	/// TODO.
	RML = Self::letters_to_token(b"RML"),
	
	/// TODO.
	RMM = Self::letters_to_token(b"RMM"),
	
	/// TODO.
	RMN = Self::letters_to_token(b"RMN"),
	
	/// TODO.
	RMO = Self::letters_to_token(b"RMO"),
	
	/// TODO.
	RMP = Self::letters_to_token(b"RMP"),
	
	/// TODO.
	RMQ = Self::letters_to_token(b"RMQ"),
	
	/// TODO.
	RMR = Self::letters_to_token(b"RMR"),
	
	/// TODO.
	RMS = Self::letters_to_token(b"RMS"),
	
	/// TODO.
	RMT = Self::letters_to_token(b"RMT"),
	
	/// TODO.
	RMU = Self::letters_to_token(b"RMU"),
	
	/// TODO.
	RMV = Self::letters_to_token(b"RMV"),
	
	/// TODO.
	RMW = Self::letters_to_token(b"RMW"),
	
	/// TODO.
	RMX = Self::letters_to_token(b"RMX"),
	
	/// TODO.
	RMY = Self::letters_to_token(b"RMY"),
	
	/// TODO.
	RMZ = Self::letters_to_token(b"RMZ"),
	
	/// TODO.
	RNA = Self::letters_to_token(b"RNA"),
	
	/// TODO.
	RNB = Self::letters_to_token(b"RNB"),
	
	/// TODO.
	RNC = Self::letters_to_token(b"RNC"),
	
	/// TODO.
	RND = Self::letters_to_token(b"RND"),
	
	/// TODO.
	RNE = Self::letters_to_token(b"RNE"),
	
	/// TODO.
	RNF = Self::letters_to_token(b"RNF"),
	
	/// TODO.
	RNG = Self::letters_to_token(b"RNG"),
	
	/// TODO.
	RNH = Self::letters_to_token(b"RNH"),
	
	/// TODO.
	RNI = Self::letters_to_token(b"RNI"),
	
	/// TODO.
	RNJ = Self::letters_to_token(b"RNJ"),
	
	/// TODO.
	RNK = Self::letters_to_token(b"RNK"),
	
	/// TODO.
	RNL = Self::letters_to_token(b"RNL"),
	
	/// TODO.
	RNM = Self::letters_to_token(b"RNM"),
	
	/// TODO.
	RNN = Self::letters_to_token(b"RNN"),
	
	/// TODO.
	RNO = Self::letters_to_token(b"RNO"),
	
	/// TODO.
	RNP = Self::letters_to_token(b"RNP"),
	
	/// TODO.
	RNQ = Self::letters_to_token(b"RNQ"),
	
	/// TODO.
	RNR = Self::letters_to_token(b"RNR"),
	
	/// TODO.
	RNS = Self::letters_to_token(b"RNS"),
	
	/// TODO.
	RNT = Self::letters_to_token(b"RNT"),
	
	/// TODO.
	RNU = Self::letters_to_token(b"RNU"),
	
	/// TODO.
	RNV = Self::letters_to_token(b"RNV"),
	
	/// TODO.
	RNW = Self::letters_to_token(b"RNW"),
	
	/// TODO.
	RNX = Self::letters_to_token(b"RNX"),
	
	/// TODO.
	RNY = Self::letters_to_token(b"RNY"),
	
	/// TODO.
	RNZ = Self::letters_to_token(b"RNZ"),
	
	/// TODO.
	ROA = Self::letters_to_token(b"ROA"),
	
	/// TODO.
	ROB = Self::letters_to_token(b"ROB"),
	
	/// TODO.
	ROC = Self::letters_to_token(b"ROC"),
	
	/// TODO.
	ROD = Self::letters_to_token(b"ROD"),
	
	/// TODO.
	ROE = Self::letters_to_token(b"ROE"),
	
	/// TODO.
	ROF = Self::letters_to_token(b"ROF"),
	
	/// TODO.
	ROG = Self::letters_to_token(b"ROG"),
	
	/// TODO.
	ROH = Self::letters_to_token(b"ROH"),
	
	/// TODO.
	ROI = Self::letters_to_token(b"ROI"),
	
	/// TODO.
	ROJ = Self::letters_to_token(b"ROJ"),
	
	/// TODO.
	ROK = Self::letters_to_token(b"ROK"),
	
	/// TODO.
	ROL = Self::letters_to_token(b"ROL"),
	
	/// TODO.
	ROM = Self::letters_to_token(b"ROM"),
	
	/// TODO.
	RON = Self::letters_to_token(b"RON"),
	
	/// TODO.
	ROO = Self::letters_to_token(b"ROO"),
	
	/// TODO.
	ROP = Self::letters_to_token(b"ROP"),
	
	/// TODO.
	ROQ = Self::letters_to_token(b"ROQ"),
	
	/// TODO.
	ROR = Self::letters_to_token(b"ROR"),
	
	/// TODO.
	ROS = Self::letters_to_token(b"ROS"),
	
	/// TODO.
	ROT = Self::letters_to_token(b"ROT"),
	
	/// TODO.
	ROU = Self::letters_to_token(b"ROU"),
	
	/// TODO.
	ROV = Self::letters_to_token(b"ROV"),
	
	/// TODO.
	ROW = Self::letters_to_token(b"ROW"),
	
	/// TODO.
	ROX = Self::letters_to_token(b"ROX"),
	
	/// TODO.
	ROY = Self::letters_to_token(b"ROY"),
	
	/// TODO.
	ROZ = Self::letters_to_token(b"ROZ"),
	
	/// TODO.
	RPA = Self::letters_to_token(b"RPA"),
	
	/// TODO.
	RPB = Self::letters_to_token(b"RPB"),
	
	/// TODO.
	RPC = Self::letters_to_token(b"RPC"),
	
	/// TODO.
	RPD = Self::letters_to_token(b"RPD"),
	
	/// TODO.
	RPE = Self::letters_to_token(b"RPE"),
	
	/// TODO.
	RPF = Self::letters_to_token(b"RPF"),
	
	/// TODO.
	RPG = Self::letters_to_token(b"RPG"),
	
	/// TODO.
	RPH = Self::letters_to_token(b"RPH"),
	
	/// TODO.
	RPI = Self::letters_to_token(b"RPI"),
	
	/// TODO.
	RPJ = Self::letters_to_token(b"RPJ"),
	
	/// TODO.
	RPK = Self::letters_to_token(b"RPK"),
	
	/// TODO.
	RPL = Self::letters_to_token(b"RPL"),
	
	/// TODO.
	RPM = Self::letters_to_token(b"RPM"),
	
	/// TODO.
	RPN = Self::letters_to_token(b"RPN"),
	
	/// TODO.
	RPO = Self::letters_to_token(b"RPO"),
	
	/// TODO.
	RPP = Self::letters_to_token(b"RPP"),
	
	/// TODO.
	RPQ = Self::letters_to_token(b"RPQ"),
	
	/// TODO.
	RPR = Self::letters_to_token(b"RPR"),
	
	/// TODO.
	RPS = Self::letters_to_token(b"RPS"),
	
	/// TODO.
	RPT = Self::letters_to_token(b"RPT"),
	
	/// TODO.
	RPU = Self::letters_to_token(b"RPU"),
	
	/// TODO.
	RPV = Self::letters_to_token(b"RPV"),
	
	/// TODO.
	RPW = Self::letters_to_token(b"RPW"),
	
	/// TODO.
	RPX = Self::letters_to_token(b"RPX"),
	
	/// TODO.
	RPY = Self::letters_to_token(b"RPY"),
	
	/// TODO.
	RPZ = Self::letters_to_token(b"RPZ"),
	
	/// TODO.
	RQA = Self::letters_to_token(b"RQA"),
	
	/// TODO.
	RQB = Self::letters_to_token(b"RQB"),
	
	/// TODO.
	RQC = Self::letters_to_token(b"RQC"),
	
	/// TODO.
	RQD = Self::letters_to_token(b"RQD"),
	
	/// TODO.
	RQE = Self::letters_to_token(b"RQE"),
	
	/// TODO.
	RQF = Self::letters_to_token(b"RQF"),
	
	/// TODO.
	RQG = Self::letters_to_token(b"RQG"),
	
	/// TODO.
	RQH = Self::letters_to_token(b"RQH"),
	
	/// TODO.
	RQI = Self::letters_to_token(b"RQI"),
	
	/// TODO.
	RQJ = Self::letters_to_token(b"RQJ"),
	
	/// TODO.
	RQK = Self::letters_to_token(b"RQK"),
	
	/// TODO.
	RQL = Self::letters_to_token(b"RQL"),
	
	/// TODO.
	RQM = Self::letters_to_token(b"RQM"),
	
	/// TODO.
	RQN = Self::letters_to_token(b"RQN"),
	
	/// TODO.
	RQO = Self::letters_to_token(b"RQO"),
	
	/// TODO.
	RQP = Self::letters_to_token(b"RQP"),
	
	/// TODO.
	RQQ = Self::letters_to_token(b"RQQ"),
	
	/// TODO.
	RQR = Self::letters_to_token(b"RQR"),
	
	/// TODO.
	RQS = Self::letters_to_token(b"RQS"),
	
	/// TODO.
	RQT = Self::letters_to_token(b"RQT"),
	
	/// TODO.
	RQU = Self::letters_to_token(b"RQU"),
	
	/// TODO.
	RQV = Self::letters_to_token(b"RQV"),
	
	/// TODO.
	RQW = Self::letters_to_token(b"RQW"),
	
	/// TODO.
	RQX = Self::letters_to_token(b"RQX"),
	
	/// TODO.
	RQY = Self::letters_to_token(b"RQY"),
	
	/// TODO.
	RQZ = Self::letters_to_token(b"RQZ"),
	
	/// TODO.
	RRA = Self::letters_to_token(b"RRA"),
	
	/// TODO.
	RRB = Self::letters_to_token(b"RRB"),
	
	/// TODO.
	RRC = Self::letters_to_token(b"RRC"),
	
	/// TODO.
	RRD = Self::letters_to_token(b"RRD"),
	
	/// TODO.
	RRE = Self::letters_to_token(b"RRE"),
	
	/// TODO.
	RRF = Self::letters_to_token(b"RRF"),
	
	/// TODO.
	RRG = Self::letters_to_token(b"RRG"),
	
	/// TODO.
	RRH = Self::letters_to_token(b"RRH"),
	
	/// TODO.
	RRI = Self::letters_to_token(b"RRI"),
	
	/// TODO.
	RRJ = Self::letters_to_token(b"RRJ"),
	
	/// TODO.
	RRK = Self::letters_to_token(b"RRK"),
	
	/// TODO.
	RRL = Self::letters_to_token(b"RRL"),
	
	/// TODO.
	RRM = Self::letters_to_token(b"RRM"),
	
	/// TODO.
	RRN = Self::letters_to_token(b"RRN"),
	
	/// TODO.
	RRO = Self::letters_to_token(b"RRO"),
	
	/// TODO.
	RRP = Self::letters_to_token(b"RRP"),
	
	/// TODO.
	RRQ = Self::letters_to_token(b"RRQ"),
	
	/// TODO.
	RRR = Self::letters_to_token(b"RRR"),
	
	/// TODO.
	RRS = Self::letters_to_token(b"RRS"),
	
	/// TODO.
	RRT = Self::letters_to_token(b"RRT"),
	
	/// TODO.
	RRU = Self::letters_to_token(b"RRU"),
	
	/// TODO.
	RRV = Self::letters_to_token(b"RRV"),
	
	/// TODO.
	RRW = Self::letters_to_token(b"RRW"),
	
	/// TODO.
	RRX = Self::letters_to_token(b"RRX"),
	
	/// TODO.
	RRY = Self::letters_to_token(b"RRY"),
	
	/// TODO.
	RRZ = Self::letters_to_token(b"RRZ"),
	
	/// TODO.
	RSA = Self::letters_to_token(b"RSA"),
	
	/// TODO.
	RSB = Self::letters_to_token(b"RSB"),
	
	/// TODO.
	RSC = Self::letters_to_token(b"RSC"),
	
	/// TODO.
	RSD = Self::letters_to_token(b"RSD"),
	
	/// TODO.
	RSE = Self::letters_to_token(b"RSE"),
	
	/// TODO.
	RSF = Self::letters_to_token(b"RSF"),
	
	/// TODO.
	RSG = Self::letters_to_token(b"RSG"),
	
	/// TODO.
	RSH = Self::letters_to_token(b"RSH"),
	
	/// TODO.
	RSI = Self::letters_to_token(b"RSI"),
	
	/// TODO.
	RSJ = Self::letters_to_token(b"RSJ"),
	
	/// TODO.
	RSK = Self::letters_to_token(b"RSK"),
	
	/// TODO.
	RSL = Self::letters_to_token(b"RSL"),
	
	/// TODO.
	RSM = Self::letters_to_token(b"RSM"),
	
	/// TODO.
	RSN = Self::letters_to_token(b"RSN"),
	
	/// TODO.
	RSO = Self::letters_to_token(b"RSO"),
	
	/// TODO.
	RSP = Self::letters_to_token(b"RSP"),
	
	/// TODO.
	RSQ = Self::letters_to_token(b"RSQ"),
	
	/// TODO.
	RSR = Self::letters_to_token(b"RSR"),
	
	/// TODO.
	RSS = Self::letters_to_token(b"RSS"),
	
	/// TODO.
	RST = Self::letters_to_token(b"RST"),
	
	/// TODO.
	RSU = Self::letters_to_token(b"RSU"),
	
	/// TODO.
	RSV = Self::letters_to_token(b"RSV"),
	
	/// TODO.
	RSW = Self::letters_to_token(b"RSW"),
	
	/// TODO.
	RSX = Self::letters_to_token(b"RSX"),
	
	/// TODO.
	RSY = Self::letters_to_token(b"RSY"),
	
	/// TODO.
	RSZ = Self::letters_to_token(b"RSZ"),
	
	/// TODO.
	RTA = Self::letters_to_token(b"RTA"),
	
	/// TODO.
	RTB = Self::letters_to_token(b"RTB"),
	
	/// TODO.
	RTC = Self::letters_to_token(b"RTC"),
	
	/// TODO.
	RTD = Self::letters_to_token(b"RTD"),
	
	/// TODO.
	RTE = Self::letters_to_token(b"RTE"),
	
	/// TODO.
	RTF = Self::letters_to_token(b"RTF"),
	
	/// TODO.
	RTG = Self::letters_to_token(b"RTG"),
	
	/// TODO.
	RTH = Self::letters_to_token(b"RTH"),
	
	/// TODO.
	RTI = Self::letters_to_token(b"RTI"),
	
	/// TODO.
	RTJ = Self::letters_to_token(b"RTJ"),
	
	/// TODO.
	RTK = Self::letters_to_token(b"RTK"),
	
	/// TODO.
	RTL = Self::letters_to_token(b"RTL"),
	
	/// TODO.
	RTM = Self::letters_to_token(b"RTM"),
	
	/// TODO.
	RTN = Self::letters_to_token(b"RTN"),
	
	/// TODO.
	RTO = Self::letters_to_token(b"RTO"),
	
	/// TODO.
	RTP = Self::letters_to_token(b"RTP"),
	
	/// TODO.
	RTQ = Self::letters_to_token(b"RTQ"),
	
	/// TODO.
	RTR = Self::letters_to_token(b"RTR"),
	
	/// TODO.
	RTS = Self::letters_to_token(b"RTS"),
	
	/// TODO.
	RTT = Self::letters_to_token(b"RTT"),
	
	/// TODO.
	RTU = Self::letters_to_token(b"RTU"),
	
	/// TODO.
	RTV = Self::letters_to_token(b"RTV"),
	
	/// TODO.
	RTW = Self::letters_to_token(b"RTW"),
	
	/// TODO.
	RTX = Self::letters_to_token(b"RTX"),
	
	/// TODO.
	RTY = Self::letters_to_token(b"RTY"),
	
	/// TODO.
	RTZ = Self::letters_to_token(b"RTZ"),
	
	/// TODO.
	RUA = Self::letters_to_token(b"RUA"),
	
	/// TODO.
	RUB = Self::letters_to_token(b"RUB"),
	
	/// TODO.
	RUC = Self::letters_to_token(b"RUC"),
	
	/// TODO.
	RUD = Self::letters_to_token(b"RUD"),
	
	/// TODO.
	RUE = Self::letters_to_token(b"RUE"),
	
	/// TODO.
	RUF = Self::letters_to_token(b"RUF"),
	
	/// TODO.
	RUG = Self::letters_to_token(b"RUG"),
	
	/// TODO.
	RUH = Self::letters_to_token(b"RUH"),
	
	/// TODO.
	RUI = Self::letters_to_token(b"RUI"),
	
	/// TODO.
	RUJ = Self::letters_to_token(b"RUJ"),
	
	/// TODO.
	RUK = Self::letters_to_token(b"RUK"),
	
	/// TODO.
	RUL = Self::letters_to_token(b"RUL"),
	
	/// TODO.
	RUM = Self::letters_to_token(b"RUM"),
	
	/// TODO.
	RUN = Self::letters_to_token(b"RUN"),
	
	/// TODO.
	RUO = Self::letters_to_token(b"RUO"),
	
	/// TODO.
	RUP = Self::letters_to_token(b"RUP"),
	
	/// TODO.
	RUQ = Self::letters_to_token(b"RUQ"),
	
	/// TODO.
	RUR = Self::letters_to_token(b"RUR"),
	
	/// TODO.
	RUS = Self::letters_to_token(b"RUS"),
	
	/// TODO.
	RUT = Self::letters_to_token(b"RUT"),
	
	/// TODO.
	RUU = Self::letters_to_token(b"RUU"),
	
	/// TODO.
	RUV = Self::letters_to_token(b"RUV"),
	
	/// TODO.
	RUW = Self::letters_to_token(b"RUW"),
	
	/// TODO.
	RUX = Self::letters_to_token(b"RUX"),
	
	/// TODO.
	RUY = Self::letters_to_token(b"RUY"),
	
	/// TODO.
	RUZ = Self::letters_to_token(b"RUZ"),
	
	/// TODO.
	RVA = Self::letters_to_token(b"RVA"),
	
	/// TODO.
	RVB = Self::letters_to_token(b"RVB"),
	
	/// TODO.
	RVC = Self::letters_to_token(b"RVC"),
	
	/// TODO.
	RVD = Self::letters_to_token(b"RVD"),
	
	/// TODO.
	RVE = Self::letters_to_token(b"RVE"),
	
	/// TODO.
	RVF = Self::letters_to_token(b"RVF"),
	
	/// TODO.
	RVG = Self::letters_to_token(b"RVG"),
	
	/// TODO.
	RVH = Self::letters_to_token(b"RVH"),
	
	/// TODO.
	RVI = Self::letters_to_token(b"RVI"),
	
	/// TODO.
	RVJ = Self::letters_to_token(b"RVJ"),
	
	/// TODO.
	RVK = Self::letters_to_token(b"RVK"),
	
	/// TODO.
	RVL = Self::letters_to_token(b"RVL"),
	
	/// TODO.
	RVM = Self::letters_to_token(b"RVM"),
	
	/// TODO.
	RVN = Self::letters_to_token(b"RVN"),
	
	/// TODO.
	RVO = Self::letters_to_token(b"RVO"),
	
	/// TODO.
	RVP = Self::letters_to_token(b"RVP"),
	
	/// TODO.
	RVQ = Self::letters_to_token(b"RVQ"),
	
	/// TODO.
	RVR = Self::letters_to_token(b"RVR"),
	
	/// TODO.
	RVS = Self::letters_to_token(b"RVS"),
	
	/// TODO.
	RVT = Self::letters_to_token(b"RVT"),
	
	/// TODO.
	RVU = Self::letters_to_token(b"RVU"),
	
	/// TODO.
	RVV = Self::letters_to_token(b"RVV"),
	
	/// TODO.
	RVW = Self::letters_to_token(b"RVW"),
	
	/// TODO.
	RVX = Self::letters_to_token(b"RVX"),
	
	/// TODO.
	RVY = Self::letters_to_token(b"RVY"),
	
	/// TODO.
	RVZ = Self::letters_to_token(b"RVZ"),
	
	/// TODO.
	RWA = Self::letters_to_token(b"RWA"),
	
	/// TODO.
	RWB = Self::letters_to_token(b"RWB"),
	
	/// TODO.
	RWC = Self::letters_to_token(b"RWC"),
	
	/// TODO.
	RWD = Self::letters_to_token(b"RWD"),
	
	/// TODO.
	RWE = Self::letters_to_token(b"RWE"),
	
	/// TODO.
	RWF = Self::letters_to_token(b"RWF"),
	
	/// TODO.
	RWG = Self::letters_to_token(b"RWG"),
	
	/// TODO.
	RWH = Self::letters_to_token(b"RWH"),
	
	/// TODO.
	RWI = Self::letters_to_token(b"RWI"),
	
	/// TODO.
	RWJ = Self::letters_to_token(b"RWJ"),
	
	/// TODO.
	RWK = Self::letters_to_token(b"RWK"),
	
	/// TODO.
	RWL = Self::letters_to_token(b"RWL"),
	
	/// TODO.
	RWM = Self::letters_to_token(b"RWM"),
	
	/// TODO.
	RWN = Self::letters_to_token(b"RWN"),
	
	/// TODO.
	RWO = Self::letters_to_token(b"RWO"),
	
	/// TODO.
	RWP = Self::letters_to_token(b"RWP"),
	
	/// TODO.
	RWQ = Self::letters_to_token(b"RWQ"),
	
	/// TODO.
	RWR = Self::letters_to_token(b"RWR"),
	
	/// TODO.
	RWS = Self::letters_to_token(b"RWS"),
	
	/// TODO.
	RWT = Self::letters_to_token(b"RWT"),
	
	/// TODO.
	RWU = Self::letters_to_token(b"RWU"),
	
	/// TODO.
	RWV = Self::letters_to_token(b"RWV"),
	
	/// TODO.
	RWW = Self::letters_to_token(b"RWW"),
	
	/// TODO.
	RWX = Self::letters_to_token(b"RWX"),
	
	/// TODO.
	RWY = Self::letters_to_token(b"RWY"),
	
	/// TODO.
	RWZ = Self::letters_to_token(b"RWZ"),
	
	/// TODO.
	RXA = Self::letters_to_token(b"RXA"),
	
	/// TODO.
	RXB = Self::letters_to_token(b"RXB"),
	
	/// TODO.
	RXC = Self::letters_to_token(b"RXC"),
	
	/// TODO.
	RXD = Self::letters_to_token(b"RXD"),
	
	/// TODO.
	RXE = Self::letters_to_token(b"RXE"),
	
	/// TODO.
	RXF = Self::letters_to_token(b"RXF"),
	
	/// TODO.
	RXG = Self::letters_to_token(b"RXG"),
	
	/// TODO.
	RXH = Self::letters_to_token(b"RXH"),
	
	/// TODO.
	RXI = Self::letters_to_token(b"RXI"),
	
	/// TODO.
	RXJ = Self::letters_to_token(b"RXJ"),
	
	/// TODO.
	RXK = Self::letters_to_token(b"RXK"),
	
	/// TODO.
	RXL = Self::letters_to_token(b"RXL"),
	
	/// TODO.
	RXM = Self::letters_to_token(b"RXM"),
	
	/// TODO.
	RXN = Self::letters_to_token(b"RXN"),
	
	/// TODO.
	RXO = Self::letters_to_token(b"RXO"),
	
	/// TODO.
	RXP = Self::letters_to_token(b"RXP"),
	
	/// TODO.
	RXQ = Self::letters_to_token(b"RXQ"),
	
	/// TODO.
	RXR = Self::letters_to_token(b"RXR"),
	
	/// TODO.
	RXS = Self::letters_to_token(b"RXS"),
	
	/// TODO.
	RXT = Self::letters_to_token(b"RXT"),
	
	/// TODO.
	RXU = Self::letters_to_token(b"RXU"),
	
	/// TODO.
	RXV = Self::letters_to_token(b"RXV"),
	
	/// TODO.
	RXW = Self::letters_to_token(b"RXW"),
	
	/// TODO.
	RXX = Self::letters_to_token(b"RXX"),
	
	/// TODO.
	RXY = Self::letters_to_token(b"RXY"),
	
	/// TODO.
	RXZ = Self::letters_to_token(b"RXZ"),
	
	/// TODO.
	RYA = Self::letters_to_token(b"RYA"),
	
	/// TODO.
	RYB = Self::letters_to_token(b"RYB"),
	
	/// TODO.
	RYC = Self::letters_to_token(b"RYC"),
	
	/// TODO.
	RYD = Self::letters_to_token(b"RYD"),
	
	/// TODO.
	RYE = Self::letters_to_token(b"RYE"),
	
	/// TODO.
	RYF = Self::letters_to_token(b"RYF"),
	
	/// TODO.
	RYG = Self::letters_to_token(b"RYG"),
	
	/// TODO.
	RYH = Self::letters_to_token(b"RYH"),
	
	/// TODO.
	RYI = Self::letters_to_token(b"RYI"),
	
	/// TODO.
	RYJ = Self::letters_to_token(b"RYJ"),
	
	/// TODO.
	RYK = Self::letters_to_token(b"RYK"),
	
	/// TODO.
	RYL = Self::letters_to_token(b"RYL"),
	
	/// TODO.
	RYM = Self::letters_to_token(b"RYM"),
	
	/// TODO.
	RYN = Self::letters_to_token(b"RYN"),
	
	/// TODO.
	RYO = Self::letters_to_token(b"RYO"),
	
	/// TODO.
	RYP = Self::letters_to_token(b"RYP"),
	
	/// TODO.
	RYQ = Self::letters_to_token(b"RYQ"),
	
	/// TODO.
	RYR = Self::letters_to_token(b"RYR"),
	
	/// TODO.
	RYS = Self::letters_to_token(b"RYS"),
	
	/// TODO.
	RYT = Self::letters_to_token(b"RYT"),
	
	/// TODO.
	RYU = Self::letters_to_token(b"RYU"),
	
	/// TODO.
	RYV = Self::letters_to_token(b"RYV"),
	
	/// TODO.
	RYW = Self::letters_to_token(b"RYW"),
	
	/// TODO.
	RYX = Self::letters_to_token(b"RYX"),
	
	/// TODO.
	RYY = Self::letters_to_token(b"RYY"),
	
	/// TODO.
	RYZ = Self::letters_to_token(b"RYZ"),
	
	/// TODO.
	RZA = Self::letters_to_token(b"RZA"),
	
	/// TODO.
	RZB = Self::letters_to_token(b"RZB"),
	
	/// TODO.
	RZC = Self::letters_to_token(b"RZC"),
	
	/// TODO.
	RZD = Self::letters_to_token(b"RZD"),
	
	/// TODO.
	RZE = Self::letters_to_token(b"RZE"),
	
	/// TODO.
	RZF = Self::letters_to_token(b"RZF"),
	
	/// TODO.
	RZG = Self::letters_to_token(b"RZG"),
	
	/// TODO.
	RZH = Self::letters_to_token(b"RZH"),
	
	/// TODO.
	RZI = Self::letters_to_token(b"RZI"),
	
	/// TODO.
	RZJ = Self::letters_to_token(b"RZJ"),
	
	/// TODO.
	RZK = Self::letters_to_token(b"RZK"),
	
	/// TODO.
	RZL = Self::letters_to_token(b"RZL"),
	
	/// TODO.
	RZM = Self::letters_to_token(b"RZM"),
	
	/// TODO.
	RZN = Self::letters_to_token(b"RZN"),
	
	/// TODO.
	RZO = Self::letters_to_token(b"RZO"),
	
	/// TODO.
	RZP = Self::letters_to_token(b"RZP"),
	
	/// TODO.
	RZQ = Self::letters_to_token(b"RZQ"),
	
	/// TODO.
	RZR = Self::letters_to_token(b"RZR"),
	
	/// TODO.
	RZS = Self::letters_to_token(b"RZS"),
	
	/// TODO.
	RZT = Self::letters_to_token(b"RZT"),
	
	/// TODO.
	RZU = Self::letters_to_token(b"RZU"),
	
	/// TODO.
	RZV = Self::letters_to_token(b"RZV"),
	
	/// TODO.
	RZW = Self::letters_to_token(b"RZW"),
	
	/// TODO.
	RZX = Self::letters_to_token(b"RZX"),
	
	/// TODO.
	RZY = Self::letters_to_token(b"RZY"),
	
	/// TODO.
	RZZ = Self::letters_to_token(b"RZZ"),
	
	/// TODO.
	SAA = Self::letters_to_token(b"SAA"),
	
	/// TODO.
	SAB = Self::letters_to_token(b"SAB"),
	
	/// TODO.
	SAC = Self::letters_to_token(b"SAC"),
	
	/// TODO.
	SAD = Self::letters_to_token(b"SAD"),
	
	/// TODO.
	SAE = Self::letters_to_token(b"SAE"),
	
	/// TODO.
	SAF = Self::letters_to_token(b"SAF"),
	
	/// TODO.
	SAG = Self::letters_to_token(b"SAG"),
	
	/// TODO.
	SAH = Self::letters_to_token(b"SAH"),
	
	/// TODO.
	SAI = Self::letters_to_token(b"SAI"),
	
	/// TODO.
	SAJ = Self::letters_to_token(b"SAJ"),
	
	/// TODO.
	SAK = Self::letters_to_token(b"SAK"),
	
	/// TODO.
	SAL = Self::letters_to_token(b"SAL"),
	
	/// TODO.
	SAM = Self::letters_to_token(b"SAM"),
	
	/// TODO.
	SAN = Self::letters_to_token(b"SAN"),
	
	/// TODO.
	SAO = Self::letters_to_token(b"SAO"),
	
	/// TODO.
	SAP = Self::letters_to_token(b"SAP"),
	
	/// TODO.
	SAQ = Self::letters_to_token(b"SAQ"),
	
	/// TODO.
	SAR = Self::letters_to_token(b"SAR"),
	
	/// TODO.
	SAS = Self::letters_to_token(b"SAS"),
	
	/// TODO.
	SAT = Self::letters_to_token(b"SAT"),
	
	/// TODO.
	SAU = Self::letters_to_token(b"SAU"),
	
	/// TODO.
	SAV = Self::letters_to_token(b"SAV"),
	
	/// TODO.
	SAW = Self::letters_to_token(b"SAW"),
	
	/// TODO.
	SAX = Self::letters_to_token(b"SAX"),
	
	/// TODO.
	SAY = Self::letters_to_token(b"SAY"),
	
	/// TODO.
	SAZ = Self::letters_to_token(b"SAZ"),
	
	/// TODO.
	SBA = Self::letters_to_token(b"SBA"),
	
	/// TODO.
	SBB = Self::letters_to_token(b"SBB"),
	
	/// TODO.
	SBC = Self::letters_to_token(b"SBC"),
	
	/// TODO.
	SBD = Self::letters_to_token(b"SBD"),
	
	/// TODO.
	SBE = Self::letters_to_token(b"SBE"),
	
	/// TODO.
	SBF = Self::letters_to_token(b"SBF"),
	
	/// TODO.
	SBG = Self::letters_to_token(b"SBG"),
	
	/// TODO.
	SBH = Self::letters_to_token(b"SBH"),
	
	/// TODO.
	SBI = Self::letters_to_token(b"SBI"),
	
	/// TODO.
	SBJ = Self::letters_to_token(b"SBJ"),
	
	/// TODO.
	SBK = Self::letters_to_token(b"SBK"),
	
	/// TODO.
	SBL = Self::letters_to_token(b"SBL"),
	
	/// TODO.
	SBM = Self::letters_to_token(b"SBM"),
	
	/// TODO.
	SBN = Self::letters_to_token(b"SBN"),
	
	/// TODO.
	SBO = Self::letters_to_token(b"SBO"),
	
	/// TODO.
	SBP = Self::letters_to_token(b"SBP"),
	
	/// TODO.
	SBQ = Self::letters_to_token(b"SBQ"),
	
	/// TODO.
	SBR = Self::letters_to_token(b"SBR"),
	
	/// TODO.
	SBS = Self::letters_to_token(b"SBS"),
	
	/// TODO.
	SBT = Self::letters_to_token(b"SBT"),
	
	/// TODO.
	SBU = Self::letters_to_token(b"SBU"),
	
	/// TODO.
	SBV = Self::letters_to_token(b"SBV"),
	
	/// TODO.
	SBW = Self::letters_to_token(b"SBW"),
	
	/// TODO.
	SBX = Self::letters_to_token(b"SBX"),
	
	/// TODO.
	SBY = Self::letters_to_token(b"SBY"),
	
	/// TODO.
	SBZ = Self::letters_to_token(b"SBZ"),
	
	/// TODO.
	SCA = Self::letters_to_token(b"SCA"),
	
	/// TODO.
	SCB = Self::letters_to_token(b"SCB"),
	
	/// TODO.
	SCC = Self::letters_to_token(b"SCC"),
	
	/// TODO.
	SCD = Self::letters_to_token(b"SCD"),
	
	/// TODO.
	SCE = Self::letters_to_token(b"SCE"),
	
	/// TODO.
	SCF = Self::letters_to_token(b"SCF"),
	
	/// TODO.
	SCG = Self::letters_to_token(b"SCG"),
	
	/// TODO.
	SCH = Self::letters_to_token(b"SCH"),
	
	/// TODO.
	SCI = Self::letters_to_token(b"SCI"),
	
	/// TODO.
	SCJ = Self::letters_to_token(b"SCJ"),
	
	/// TODO.
	SCK = Self::letters_to_token(b"SCK"),
	
	/// TODO.
	SCL = Self::letters_to_token(b"SCL"),
	
	/// TODO.
	SCM = Self::letters_to_token(b"SCM"),
	
	/// TODO.
	SCN = Self::letters_to_token(b"SCN"),
	
	/// TODO.
	SCO = Self::letters_to_token(b"SCO"),
	
	/// TODO.
	SCP = Self::letters_to_token(b"SCP"),
	
	/// TODO.
	SCQ = Self::letters_to_token(b"SCQ"),
	
	/// TODO.
	SCR = Self::letters_to_token(b"SCR"),
	
	/// TODO.
	SCS = Self::letters_to_token(b"SCS"),
	
	/// TODO.
	SCT = Self::letters_to_token(b"SCT"),
	
	/// TODO.
	SCU = Self::letters_to_token(b"SCU"),
	
	/// TODO.
	SCV = Self::letters_to_token(b"SCV"),
	
	/// TODO.
	SCW = Self::letters_to_token(b"SCW"),
	
	/// TODO.
	SCX = Self::letters_to_token(b"SCX"),
	
	/// TODO.
	SCY = Self::letters_to_token(b"SCY"),
	
	/// TODO.
	SCZ = Self::letters_to_token(b"SCZ"),
	
	/// TODO.
	SDA = Self::letters_to_token(b"SDA"),
	
	/// TODO.
	SDB = Self::letters_to_token(b"SDB"),
	
	/// TODO.
	SDC = Self::letters_to_token(b"SDC"),
	
	/// TODO.
	SDD = Self::letters_to_token(b"SDD"),
	
	/// TODO.
	SDE = Self::letters_to_token(b"SDE"),
	
	/// TODO.
	SDF = Self::letters_to_token(b"SDF"),
	
	/// TODO.
	SDG = Self::letters_to_token(b"SDG"),
	
	/// TODO.
	SDH = Self::letters_to_token(b"SDH"),
	
	/// TODO.
	SDI = Self::letters_to_token(b"SDI"),
	
	/// TODO.
	SDJ = Self::letters_to_token(b"SDJ"),
	
	/// TODO.
	SDK = Self::letters_to_token(b"SDK"),
	
	/// TODO.
	SDL = Self::letters_to_token(b"SDL"),
	
	/// TODO.
	SDM = Self::letters_to_token(b"SDM"),
	
	/// TODO.
	SDN = Self::letters_to_token(b"SDN"),
	
	/// TODO.
	SDO = Self::letters_to_token(b"SDO"),
	
	/// TODO.
	SDP = Self::letters_to_token(b"SDP"),
	
	/// TODO.
	SDQ = Self::letters_to_token(b"SDQ"),
	
	/// TODO.
	SDR = Self::letters_to_token(b"SDR"),
	
	/// TODO.
	SDS = Self::letters_to_token(b"SDS"),
	
	/// TODO.
	SDT = Self::letters_to_token(b"SDT"),
	
	/// TODO.
	SDU = Self::letters_to_token(b"SDU"),
	
	/// TODO.
	SDV = Self::letters_to_token(b"SDV"),
	
	/// TODO.
	SDW = Self::letters_to_token(b"SDW"),
	
	/// TODO.
	SDX = Self::letters_to_token(b"SDX"),
	
	/// TODO.
	SDY = Self::letters_to_token(b"SDY"),
	
	/// TODO.
	SDZ = Self::letters_to_token(b"SDZ"),
	
	/// TODO.
	SEA = Self::letters_to_token(b"SEA"),
	
	/// TODO.
	SEB = Self::letters_to_token(b"SEB"),
	
	/// TODO.
	SEC = Self::letters_to_token(b"SEC"),
	
	/// TODO.
	SED = Self::letters_to_token(b"SED"),
	
	/// TODO.
	SEE = Self::letters_to_token(b"SEE"),
	
	/// TODO.
	SEF = Self::letters_to_token(b"SEF"),
	
	/// TODO.
	SEG = Self::letters_to_token(b"SEG"),
	
	/// TODO.
	SEH = Self::letters_to_token(b"SEH"),
	
	/// TODO.
	SEI = Self::letters_to_token(b"SEI"),
	
	/// TODO.
	SEJ = Self::letters_to_token(b"SEJ"),
	
	/// TODO.
	SEK = Self::letters_to_token(b"SEK"),
	
	/// TODO.
	SEL = Self::letters_to_token(b"SEL"),
	
	/// TODO.
	SEM = Self::letters_to_token(b"SEM"),
	
	/// TODO.
	SEN = Self::letters_to_token(b"SEN"),
	
	/// TODO.
	SEO = Self::letters_to_token(b"SEO"),
	
	/// TODO.
	SEP = Self::letters_to_token(b"SEP"),
	
	/// TODO.
	SEQ = Self::letters_to_token(b"SEQ"),
	
	/// TODO.
	SER = Self::letters_to_token(b"SER"),
	
	/// TODO.
	SES = Self::letters_to_token(b"SES"),
	
	/// TODO.
	SET = Self::letters_to_token(b"SET"),
	
	/// TODO.
	SEU = Self::letters_to_token(b"SEU"),
	
	/// TODO.
	SEV = Self::letters_to_token(b"SEV"),
	
	/// TODO.
	SEW = Self::letters_to_token(b"SEW"),
	
	/// TODO.
	SEX = Self::letters_to_token(b"SEX"),
	
	/// TODO.
	SEY = Self::letters_to_token(b"SEY"),
	
	/// TODO.
	SEZ = Self::letters_to_token(b"SEZ"),
	
	/// TODO.
	SFA = Self::letters_to_token(b"SFA"),
	
	/// TODO.
	SFB = Self::letters_to_token(b"SFB"),
	
	/// TODO.
	SFC = Self::letters_to_token(b"SFC"),
	
	/// TODO.
	SFD = Self::letters_to_token(b"SFD"),
	
	/// TODO.
	SFE = Self::letters_to_token(b"SFE"),
	
	/// TODO.
	SFF = Self::letters_to_token(b"SFF"),
	
	/// TODO.
	SFG = Self::letters_to_token(b"SFG"),
	
	/// TODO.
	SFH = Self::letters_to_token(b"SFH"),
	
	/// TODO.
	SFI = Self::letters_to_token(b"SFI"),
	
	/// TODO.
	SFJ = Self::letters_to_token(b"SFJ"),
	
	/// TODO.
	SFK = Self::letters_to_token(b"SFK"),
	
	/// TODO.
	SFL = Self::letters_to_token(b"SFL"),
	
	/// TODO.
	SFM = Self::letters_to_token(b"SFM"),
	
	/// TODO.
	SFN = Self::letters_to_token(b"SFN"),
	
	/// TODO.
	SFO = Self::letters_to_token(b"SFO"),
	
	/// TODO.
	SFP = Self::letters_to_token(b"SFP"),
	
	/// TODO.
	SFQ = Self::letters_to_token(b"SFQ"),
	
	/// TODO.
	SFR = Self::letters_to_token(b"SFR"),
	
	/// TODO.
	SFS = Self::letters_to_token(b"SFS"),
	
	/// TODO.
	SFT = Self::letters_to_token(b"SFT"),
	
	/// TODO.
	SFU = Self::letters_to_token(b"SFU"),
	
	/// TODO.
	SFV = Self::letters_to_token(b"SFV"),
	
	/// TODO.
	SFW = Self::letters_to_token(b"SFW"),
	
	/// TODO.
	SFX = Self::letters_to_token(b"SFX"),
	
	/// TODO.
	SFY = Self::letters_to_token(b"SFY"),
	
	/// TODO.
	SFZ = Self::letters_to_token(b"SFZ"),
	
	/// TODO.
	SGA = Self::letters_to_token(b"SGA"),
	
	/// TODO.
	SGB = Self::letters_to_token(b"SGB"),
	
	/// TODO.
	SGC = Self::letters_to_token(b"SGC"),
	
	/// TODO.
	SGD = Self::letters_to_token(b"SGD"),
	
	/// TODO.
	SGE = Self::letters_to_token(b"SGE"),
	
	/// TODO.
	SGF = Self::letters_to_token(b"SGF"),
	
	/// TODO.
	SGG = Self::letters_to_token(b"SGG"),
	
	/// TODO.
	SGH = Self::letters_to_token(b"SGH"),
	
	/// TODO.
	SGI = Self::letters_to_token(b"SGI"),
	
	/// TODO.
	SGJ = Self::letters_to_token(b"SGJ"),
	
	/// TODO.
	SGK = Self::letters_to_token(b"SGK"),
	
	/// TODO.
	SGL = Self::letters_to_token(b"SGL"),
	
	/// TODO.
	SGM = Self::letters_to_token(b"SGM"),
	
	/// TODO.
	SGN = Self::letters_to_token(b"SGN"),
	
	/// TODO.
	SGO = Self::letters_to_token(b"SGO"),
	
	/// TODO.
	SGP = Self::letters_to_token(b"SGP"),
	
	/// TODO.
	SGQ = Self::letters_to_token(b"SGQ"),
	
	/// TODO.
	SGR = Self::letters_to_token(b"SGR"),
	
	/// TODO.
	SGS = Self::letters_to_token(b"SGS"),
	
	/// TODO.
	SGT = Self::letters_to_token(b"SGT"),
	
	/// TODO.
	SGU = Self::letters_to_token(b"SGU"),
	
	/// TODO.
	SGV = Self::letters_to_token(b"SGV"),
	
	/// TODO.
	SGW = Self::letters_to_token(b"SGW"),
	
	/// TODO.
	SGX = Self::letters_to_token(b"SGX"),
	
	/// TODO.
	SGY = Self::letters_to_token(b"SGY"),
	
	/// TODO.
	SGZ = Self::letters_to_token(b"SGZ"),
	
	/// TODO.
	SHA = Self::letters_to_token(b"SHA"),
	
	/// TODO.
	SHB = Self::letters_to_token(b"SHB"),
	
	/// TODO.
	SHC = Self::letters_to_token(b"SHC"),
	
	/// TODO.
	SHD = Self::letters_to_token(b"SHD"),
	
	/// TODO.
	SHE = Self::letters_to_token(b"SHE"),
	
	/// TODO.
	SHF = Self::letters_to_token(b"SHF"),
	
	/// TODO.
	SHG = Self::letters_to_token(b"SHG"),
	
	/// TODO.
	SHH = Self::letters_to_token(b"SHH"),
	
	/// TODO.
	SHI = Self::letters_to_token(b"SHI"),
	
	/// TODO.
	SHJ = Self::letters_to_token(b"SHJ"),
	
	/// TODO.
	SHK = Self::letters_to_token(b"SHK"),
	
	/// TODO.
	SHL = Self::letters_to_token(b"SHL"),
	
	/// TODO.
	SHM = Self::letters_to_token(b"SHM"),
	
	/// Saint Helena, Ascension and Tristan da Cunha.
	SHN = Self::letters_to_token(b"SHN"),
	
	/// TODO.
	SHO = Self::letters_to_token(b"SHO"),
	
	/// TODO.
	SHP = Self::letters_to_token(b"SHP"),
	
	/// TODO.
	SHQ = Self::letters_to_token(b"SHQ"),
	
	/// TODO.
	SHR = Self::letters_to_token(b"SHR"),
	
	/// TODO.
	SHS = Self::letters_to_token(b"SHS"),
	
	/// TODO.
	SHT = Self::letters_to_token(b"SHT"),
	
	/// TODO.
	SHU = Self::letters_to_token(b"SHU"),
	
	/// TODO.
	SHV = Self::letters_to_token(b"SHV"),
	
	/// TODO.
	SHW = Self::letters_to_token(b"SHW"),
	
	/// TODO.
	SHX = Self::letters_to_token(b"SHX"),
	
	/// TODO.
	SHY = Self::letters_to_token(b"SHY"),
	
	/// TODO.
	SHZ = Self::letters_to_token(b"SHZ"),
	
	/// TODO.
	SIA = Self::letters_to_token(b"SIA"),
	
	/// TODO.
	SIB = Self::letters_to_token(b"SIB"),
	
	/// TODO.
	SIC = Self::letters_to_token(b"SIC"),
	
	/// TODO.
	SID = Self::letters_to_token(b"SID"),
	
	/// TODO.
	SIE = Self::letters_to_token(b"SIE"),
	
	/// TODO.
	SIF = Self::letters_to_token(b"SIF"),
	
	/// TODO.
	SIG = Self::letters_to_token(b"SIG"),
	
	/// TODO.
	SIH = Self::letters_to_token(b"SIH"),
	
	/// TODO.
	SII = Self::letters_to_token(b"SII"),
	
	/// TODO.
	SIJ = Self::letters_to_token(b"SIJ"),
	
	/// TODO.
	SIK = Self::letters_to_token(b"SIK"),
	
	/// TODO.
	SIL = Self::letters_to_token(b"SIL"),
	
	/// TODO.
	SIM = Self::letters_to_token(b"SIM"),
	
	/// TODO.
	SIN = Self::letters_to_token(b"SIN"),
	
	/// TODO.
	SIO = Self::letters_to_token(b"SIO"),
	
	/// TODO.
	SIP = Self::letters_to_token(b"SIP"),
	
	/// TODO.
	SIQ = Self::letters_to_token(b"SIQ"),
	
	/// TODO.
	SIR = Self::letters_to_token(b"SIR"),
	
	/// TODO.
	SIS = Self::letters_to_token(b"SIS"),
	
	/// TODO.
	SIT = Self::letters_to_token(b"SIT"),
	
	/// TODO.
	SIU = Self::letters_to_token(b"SIU"),
	
	/// TODO.
	SIV = Self::letters_to_token(b"SIV"),
	
	/// TODO.
	SIW = Self::letters_to_token(b"SIW"),
	
	/// TODO.
	SIX = Self::letters_to_token(b"SIX"),
	
	/// TODO.
	SIY = Self::letters_to_token(b"SIY"),
	
	/// TODO.
	SIZ = Self::letters_to_token(b"SIZ"),
	
	/// TODO.
	SJA = Self::letters_to_token(b"SJA"),
	
	/// TODO.
	SJB = Self::letters_to_token(b"SJB"),
	
	/// TODO.
	SJC = Self::letters_to_token(b"SJC"),
	
	/// TODO.
	SJD = Self::letters_to_token(b"SJD"),
	
	/// TODO.
	SJE = Self::letters_to_token(b"SJE"),
	
	/// TODO.
	SJF = Self::letters_to_token(b"SJF"),
	
	/// TODO.
	SJG = Self::letters_to_token(b"SJG"),
	
	/// TODO.
	SJH = Self::letters_to_token(b"SJH"),
	
	/// TODO.
	SJI = Self::letters_to_token(b"SJI"),
	
	/// TODO.
	SJJ = Self::letters_to_token(b"SJJ"),
	
	/// TODO.
	SJK = Self::letters_to_token(b"SJK"),
	
	/// TODO.
	SJL = Self::letters_to_token(b"SJL"),
	
	/// TODO.
	SJM = Self::letters_to_token(b"SJM"),
	
	/// TODO.
	SJN = Self::letters_to_token(b"SJN"),
	
	/// TODO.
	SJO = Self::letters_to_token(b"SJO"),
	
	/// TODO.
	SJP = Self::letters_to_token(b"SJP"),
	
	/// TODO.
	SJQ = Self::letters_to_token(b"SJQ"),
	
	/// TODO.
	SJR = Self::letters_to_token(b"SJR"),
	
	/// TODO.
	SJS = Self::letters_to_token(b"SJS"),
	
	/// TODO.
	SJT = Self::letters_to_token(b"SJT"),
	
	/// TODO.
	SJU = Self::letters_to_token(b"SJU"),
	
	/// TODO.
	SJV = Self::letters_to_token(b"SJV"),
	
	/// TODO.
	SJW = Self::letters_to_token(b"SJW"),
	
	/// TODO.
	SJX = Self::letters_to_token(b"SJX"),
	
	/// TODO.
	SJY = Self::letters_to_token(b"SJY"),
	
	/// TODO.
	SJZ = Self::letters_to_token(b"SJZ"),
	
	/// TODO.
	SKA = Self::letters_to_token(b"SKA"),
	
	/// TODO.
	SKB = Self::letters_to_token(b"SKB"),
	
	/// TODO.
	SKC = Self::letters_to_token(b"SKC"),
	
	/// TODO.
	SKD = Self::letters_to_token(b"SKD"),
	
	/// TODO.
	SKE = Self::letters_to_token(b"SKE"),
	
	/// TODO.
	SKF = Self::letters_to_token(b"SKF"),
	
	/// TODO.
	SKG = Self::letters_to_token(b"SKG"),
	
	/// TODO.
	SKH = Self::letters_to_token(b"SKH"),
	
	/// TODO.
	SKI = Self::letters_to_token(b"SKI"),
	
	/// TODO.
	SKJ = Self::letters_to_token(b"SKJ"),
	
	/// TODO.
	SKK = Self::letters_to_token(b"SKK"),
	
	/// TODO.
	SKL = Self::letters_to_token(b"SKL"),
	
	/// TODO.
	SKM = Self::letters_to_token(b"SKM"),
	
	/// TODO.
	SKN = Self::letters_to_token(b"SKN"),
	
	/// TODO.
	SKO = Self::letters_to_token(b"SKO"),
	
	/// TODO.
	SKP = Self::letters_to_token(b"SKP"),
	
	/// TODO.
	SKQ = Self::letters_to_token(b"SKQ"),
	
	/// TODO.
	SKR = Self::letters_to_token(b"SKR"),
	
	/// TODO.
	SKS = Self::letters_to_token(b"SKS"),
	
	/// TODO.
	SKT = Self::letters_to_token(b"SKT"),
	
	/// TODO.
	SKU = Self::letters_to_token(b"SKU"),
	
	/// TODO.
	SKV = Self::letters_to_token(b"SKV"),
	
	/// TODO.
	SKW = Self::letters_to_token(b"SKW"),
	
	/// TODO.
	SKX = Self::letters_to_token(b"SKX"),
	
	/// TODO.
	SKY = Self::letters_to_token(b"SKY"),
	
	/// TODO.
	SKZ = Self::letters_to_token(b"SKZ"),
	
	/// TODO.
	SLA = Self::letters_to_token(b"SLA"),
	
	/// TODO.
	SLB = Self::letters_to_token(b"SLB"),
	
	/// TODO.
	SLC = Self::letters_to_token(b"SLC"),
	
	/// TODO.
	SLD = Self::letters_to_token(b"SLD"),
	
	/// TODO.
	SLE = Self::letters_to_token(b"SLE"),
	
	/// TODO.
	SLF = Self::letters_to_token(b"SLF"),
	
	/// TODO.
	SLG = Self::letters_to_token(b"SLG"),
	
	/// TODO.
	SLH = Self::letters_to_token(b"SLH"),
	
	/// TODO.
	SLI = Self::letters_to_token(b"SLI"),
	
	/// TODO.
	SLJ = Self::letters_to_token(b"SLJ"),
	
	/// TODO.
	SLK = Self::letters_to_token(b"SLK"),
	
	/// TODO.
	SLL = Self::letters_to_token(b"SLL"),
	
	/// TODO.
	SLM = Self::letters_to_token(b"SLM"),
	
	/// TODO.
	SLN = Self::letters_to_token(b"SLN"),
	
	/// TODO.
	SLO = Self::letters_to_token(b"SLO"),
	
	/// TODO.
	SLP = Self::letters_to_token(b"SLP"),
	
	/// TODO.
	SLQ = Self::letters_to_token(b"SLQ"),
	
	/// TODO.
	SLR = Self::letters_to_token(b"SLR"),
	
	/// TODO.
	SLS = Self::letters_to_token(b"SLS"),
	
	/// TODO.
	SLT = Self::letters_to_token(b"SLT"),
	
	/// TODO.
	SLU = Self::letters_to_token(b"SLU"),
	
	/// TODO.
	SLV = Self::letters_to_token(b"SLV"),
	
	/// TODO.
	SLW = Self::letters_to_token(b"SLW"),
	
	/// TODO.
	SLX = Self::letters_to_token(b"SLX"),
	
	/// TODO.
	SLY = Self::letters_to_token(b"SLY"),
	
	/// TODO.
	SLZ = Self::letters_to_token(b"SLZ"),
	
	/// TODO.
	SMA = Self::letters_to_token(b"SMA"),
	
	/// TODO.
	SMB = Self::letters_to_token(b"SMB"),
	
	/// TODO.
	SMC = Self::letters_to_token(b"SMC"),
	
	/// TODO.
	SMD = Self::letters_to_token(b"SMD"),
	
	/// TODO.
	SME = Self::letters_to_token(b"SME"),
	
	/// TODO.
	SMF = Self::letters_to_token(b"SMF"),
	
	/// TODO.
	SMG = Self::letters_to_token(b"SMG"),
	
	/// TODO.
	SMH = Self::letters_to_token(b"SMH"),
	
	/// TODO.
	SMI = Self::letters_to_token(b"SMI"),
	
	/// TODO.
	SMJ = Self::letters_to_token(b"SMJ"),
	
	/// TODO.
	SMK = Self::letters_to_token(b"SMK"),
	
	/// TODO.
	SML = Self::letters_to_token(b"SML"),
	
	/// TODO.
	SMM = Self::letters_to_token(b"SMM"),
	
	/// TODO.
	SMN = Self::letters_to_token(b"SMN"),
	
	/// TODO.
	SMO = Self::letters_to_token(b"SMO"),
	
	/// TODO.
	SMP = Self::letters_to_token(b"SMP"),
	
	/// TODO.
	SMQ = Self::letters_to_token(b"SMQ"),
	
	/// TODO.
	SMR = Self::letters_to_token(b"SMR"),
	
	/// TODO.
	SMS = Self::letters_to_token(b"SMS"),
	
	/// TODO.
	SMT = Self::letters_to_token(b"SMT"),
	
	/// TODO.
	SMU = Self::letters_to_token(b"SMU"),
	
	/// TODO.
	SMV = Self::letters_to_token(b"SMV"),
	
	/// TODO.
	SMW = Self::letters_to_token(b"SMW"),
	
	/// TODO.
	SMX = Self::letters_to_token(b"SMX"),
	
	/// TODO.
	SMY = Self::letters_to_token(b"SMY"),
	
	/// TODO.
	SMZ = Self::letters_to_token(b"SMZ"),
	
	/// TODO.
	SNA = Self::letters_to_token(b"SNA"),
	
	/// TODO.
	SNB = Self::letters_to_token(b"SNB"),
	
	/// TODO.
	SNC = Self::letters_to_token(b"SNC"),
	
	/// TODO.
	SND = Self::letters_to_token(b"SND"),
	
	/// TODO.
	SNE = Self::letters_to_token(b"SNE"),
	
	/// TODO.
	SNF = Self::letters_to_token(b"SNF"),
	
	/// TODO.
	SNG = Self::letters_to_token(b"SNG"),
	
	/// TODO.
	SNH = Self::letters_to_token(b"SNH"),
	
	/// TODO.
	SNI = Self::letters_to_token(b"SNI"),
	
	/// TODO.
	SNJ = Self::letters_to_token(b"SNJ"),
	
	/// TODO.
	SNK = Self::letters_to_token(b"SNK"),
	
	/// TODO.
	SNL = Self::letters_to_token(b"SNL"),
	
	/// TODO.
	SNM = Self::letters_to_token(b"SNM"),
	
	/// TODO.
	SNN = Self::letters_to_token(b"SNN"),
	
	/// TODO.
	SNO = Self::letters_to_token(b"SNO"),
	
	/// TODO.
	SNP = Self::letters_to_token(b"SNP"),
	
	/// TODO.
	SNQ = Self::letters_to_token(b"SNQ"),
	
	/// TODO.
	SNR = Self::letters_to_token(b"SNR"),
	
	/// TODO.
	SNS = Self::letters_to_token(b"SNS"),
	
	/// TODO.
	SNT = Self::letters_to_token(b"SNT"),
	
	/// TODO.
	SNU = Self::letters_to_token(b"SNU"),
	
	/// TODO.
	SNV = Self::letters_to_token(b"SNV"),
	
	/// TODO.
	SNW = Self::letters_to_token(b"SNW"),
	
	/// TODO.
	SNX = Self::letters_to_token(b"SNX"),
	
	/// TODO.
	SNY = Self::letters_to_token(b"SNY"),
	
	/// TODO.
	SNZ = Self::letters_to_token(b"SNZ"),
	
	/// TODO.
	SOA = Self::letters_to_token(b"SOA"),
	
	/// TODO.
	SOB = Self::letters_to_token(b"SOB"),
	
	/// TODO.
	SOC = Self::letters_to_token(b"SOC"),
	
	/// TODO.
	SOD = Self::letters_to_token(b"SOD"),
	
	/// TODO.
	SOE = Self::letters_to_token(b"SOE"),
	
	/// TODO.
	SOF = Self::letters_to_token(b"SOF"),
	
	/// TODO.
	SOG = Self::letters_to_token(b"SOG"),
	
	/// TODO.
	SOH = Self::letters_to_token(b"SOH"),
	
	/// TODO.
	SOI = Self::letters_to_token(b"SOI"),
	
	/// TODO.
	SOJ = Self::letters_to_token(b"SOJ"),
	
	/// TODO.
	SOK = Self::letters_to_token(b"SOK"),
	
	/// TODO.
	SOL = Self::letters_to_token(b"SOL"),
	
	/// TODO.
	SOM = Self::letters_to_token(b"SOM"),
	
	/// TODO.
	SON = Self::letters_to_token(b"SON"),
	
	/// TODO.
	SOO = Self::letters_to_token(b"SOO"),
	
	/// TODO.
	SOP = Self::letters_to_token(b"SOP"),
	
	/// TODO.
	SOQ = Self::letters_to_token(b"SOQ"),
	
	/// TODO.
	SOR = Self::letters_to_token(b"SOR"),
	
	/// TODO.
	SOS = Self::letters_to_token(b"SOS"),
	
	/// TODO.
	SOT = Self::letters_to_token(b"SOT"),
	
	/// TODO.
	SOU = Self::letters_to_token(b"SOU"),
	
	/// TODO.
	SOV = Self::letters_to_token(b"SOV"),
	
	/// TODO.
	SOW = Self::letters_to_token(b"SOW"),
	
	/// TODO.
	SOX = Self::letters_to_token(b"SOX"),
	
	/// TODO.
	SOY = Self::letters_to_token(b"SOY"),
	
	/// TODO.
	SOZ = Self::letters_to_token(b"SOZ"),
	
	/// TODO.
	SPA = Self::letters_to_token(b"SPA"),
	
	/// TODO.
	SPB = Self::letters_to_token(b"SPB"),
	
	/// TODO.
	SPC = Self::letters_to_token(b"SPC"),
	
	/// TODO.
	SPD = Self::letters_to_token(b"SPD"),
	
	/// TODO.
	SPE = Self::letters_to_token(b"SPE"),
	
	/// TODO.
	SPF = Self::letters_to_token(b"SPF"),
	
	/// TODO.
	SPG = Self::letters_to_token(b"SPG"),
	
	/// TODO.
	SPH = Self::letters_to_token(b"SPH"),
	
	/// TODO.
	SPI = Self::letters_to_token(b"SPI"),
	
	/// TODO.
	SPJ = Self::letters_to_token(b"SPJ"),
	
	/// TODO.
	SPK = Self::letters_to_token(b"SPK"),
	
	/// TODO.
	SPL = Self::letters_to_token(b"SPL"),
	
	/// TODO.
	SPM = Self::letters_to_token(b"SPM"),
	
	/// TODO.
	SPN = Self::letters_to_token(b"SPN"),
	
	/// TODO.
	SPO = Self::letters_to_token(b"SPO"),
	
	/// TODO.
	SPP = Self::letters_to_token(b"SPP"),
	
	/// TODO.
	SPQ = Self::letters_to_token(b"SPQ"),
	
	/// TODO.
	SPR = Self::letters_to_token(b"SPR"),
	
	/// TODO.
	SPS = Self::letters_to_token(b"SPS"),
	
	/// TODO.
	SPT = Self::letters_to_token(b"SPT"),
	
	/// TODO.
	SPU = Self::letters_to_token(b"SPU"),
	
	/// TODO.
	SPV = Self::letters_to_token(b"SPV"),
	
	/// TODO.
	SPW = Self::letters_to_token(b"SPW"),
	
	/// TODO.
	SPX = Self::letters_to_token(b"SPX"),
	
	/// TODO.
	SPY = Self::letters_to_token(b"SPY"),
	
	/// TODO.
	SPZ = Self::letters_to_token(b"SPZ"),
	
	/// TODO.
	SQA = Self::letters_to_token(b"SQA"),
	
	/// TODO.
	SQB = Self::letters_to_token(b"SQB"),
	
	/// TODO.
	SQC = Self::letters_to_token(b"SQC"),
	
	/// TODO.
	SQD = Self::letters_to_token(b"SQD"),
	
	/// TODO.
	SQE = Self::letters_to_token(b"SQE"),
	
	/// TODO.
	SQF = Self::letters_to_token(b"SQF"),
	
	/// TODO.
	SQG = Self::letters_to_token(b"SQG"),
	
	/// TODO.
	SQH = Self::letters_to_token(b"SQH"),
	
	/// TODO.
	SQI = Self::letters_to_token(b"SQI"),
	
	/// TODO.
	SQJ = Self::letters_to_token(b"SQJ"),
	
	/// TODO.
	SQK = Self::letters_to_token(b"SQK"),
	
	/// TODO.
	SQL = Self::letters_to_token(b"SQL"),
	
	/// TODO.
	SQM = Self::letters_to_token(b"SQM"),
	
	/// TODO.
	SQN = Self::letters_to_token(b"SQN"),
	
	/// TODO.
	SQO = Self::letters_to_token(b"SQO"),
	
	/// TODO.
	SQP = Self::letters_to_token(b"SQP"),
	
	/// TODO.
	SQQ = Self::letters_to_token(b"SQQ"),
	
	/// TODO.
	SQR = Self::letters_to_token(b"SQR"),
	
	/// TODO.
	SQS = Self::letters_to_token(b"SQS"),
	
	/// TODO.
	SQT = Self::letters_to_token(b"SQT"),
	
	/// TODO.
	SQU = Self::letters_to_token(b"SQU"),
	
	/// TODO.
	SQV = Self::letters_to_token(b"SQV"),
	
	/// TODO.
	SQW = Self::letters_to_token(b"SQW"),
	
	/// TODO.
	SQX = Self::letters_to_token(b"SQX"),
	
	/// TODO.
	SQY = Self::letters_to_token(b"SQY"),
	
	/// TODO.
	SQZ = Self::letters_to_token(b"SQZ"),
	
	/// TODO.
	SRA = Self::letters_to_token(b"SRA"),
	
	/// TODO.
	SRB = Self::letters_to_token(b"SRB"),
	
	/// TODO.
	SRC = Self::letters_to_token(b"SRC"),
	
	/// TODO.
	SRD = Self::letters_to_token(b"SRD"),
	
	/// TODO.
	SRE = Self::letters_to_token(b"SRE"),
	
	/// TODO.
	SRF = Self::letters_to_token(b"SRF"),
	
	/// TODO.
	SRG = Self::letters_to_token(b"SRG"),
	
	/// TODO.
	SRH = Self::letters_to_token(b"SRH"),
	
	/// TODO.
	SRI = Self::letters_to_token(b"SRI"),
	
	/// TODO.
	SRJ = Self::letters_to_token(b"SRJ"),
	
	/// TODO.
	SRK = Self::letters_to_token(b"SRK"),
	
	/// TODO.
	SRL = Self::letters_to_token(b"SRL"),
	
	/// TODO.
	SRM = Self::letters_to_token(b"SRM"),
	
	/// TODO.
	SRN = Self::letters_to_token(b"SRN"),
	
	/// TODO.
	SRO = Self::letters_to_token(b"SRO"),
	
	/// TODO.
	SRP = Self::letters_to_token(b"SRP"),
	
	/// TODO.
	SRQ = Self::letters_to_token(b"SRQ"),
	
	/// Unoffical.
	///
	/// South America (NATO STANAG 1059 INT).
	SRR = Self::letters_to_token(b"SRR"),
	
	/// TODO.
	SRS = Self::letters_to_token(b"SRS"),
	
	/// TODO.
	SRT = Self::letters_to_token(b"SRT"),
	
	/// TODO.
	SRU = Self::letters_to_token(b"SRU"),
	
	/// TODO.
	SRV = Self::letters_to_token(b"SRV"),
	
	/// TODO.
	SRW = Self::letters_to_token(b"SRW"),
	
	/// TODO.
	SRX = Self::letters_to_token(b"SRX"),
	
	/// TODO.
	SRY = Self::letters_to_token(b"SRY"),
	
	/// TODO.
	SRZ = Self::letters_to_token(b"SRZ"),
	
	/// TODO.
	SSA = Self::letters_to_token(b"SSA"),
	
	/// TODO.
	SSB = Self::letters_to_token(b"SSB"),
	
	/// TODO.
	SSC = Self::letters_to_token(b"SSC"),
	
	/// TODO.
	SSD = Self::letters_to_token(b"SSD"),
	
	/// TODO.
	SSE = Self::letters_to_token(b"SSE"),
	
	/// TODO.
	SSF = Self::letters_to_token(b"SSF"),
	
	/// TODO.
	SSG = Self::letters_to_token(b"SSG"),
	
	/// TODO.
	SSH = Self::letters_to_token(b"SSH"),
	
	/// TODO.
	SSI = Self::letters_to_token(b"SSI"),
	
	/// TODO.
	SSJ = Self::letters_to_token(b"SSJ"),
	
	/// TODO.
	SSK = Self::letters_to_token(b"SSK"),
	
	/// TODO.
	SSL = Self::letters_to_token(b"SSL"),
	
	/// TODO.
	SSM = Self::letters_to_token(b"SSM"),
	
	/// TODO.
	SSN = Self::letters_to_token(b"SSN"),
	
	/// TODO.
	SSO = Self::letters_to_token(b"SSO"),
	
	/// TODO.
	SSP = Self::letters_to_token(b"SSP"),
	
	/// TODO.
	SSQ = Self::letters_to_token(b"SSQ"),
	
	/// TODO.
	SSR = Self::letters_to_token(b"SSR"),
	
	/// TODO.
	SSS = Self::letters_to_token(b"SSS"),
	
	/// TODO.
	SST = Self::letters_to_token(b"SST"),
	
	/// TODO.
	SSU = Self::letters_to_token(b"SSU"),
	
	/// TODO.
	SSV = Self::letters_to_token(b"SSV"),
	
	/// TODO.
	SSW = Self::letters_to_token(b"SSW"),
	
	/// TODO.
	SSX = Self::letters_to_token(b"SSX"),
	
	/// TODO.
	SSY = Self::letters_to_token(b"SSY"),
	
	/// TODO.
	SSZ = Self::letters_to_token(b"SSZ"),
	
	/// TODO.
	STA = Self::letters_to_token(b"STA"),
	
	/// TODO.
	STB = Self::letters_to_token(b"STB"),
	
	/// TODO.
	STC = Self::letters_to_token(b"STC"),
	
	/// TODO.
	STD = Self::letters_to_token(b"STD"),
	
	/// TODO.
	STE = Self::letters_to_token(b"STE"),
	
	/// TODO.
	STF = Self::letters_to_token(b"STF"),
	
	/// TODO.
	STG = Self::letters_to_token(b"STG"),
	
	/// TODO.
	STH = Self::letters_to_token(b"STH"),
	
	/// TODO.
	STI = Self::letters_to_token(b"STI"),
	
	/// TODO.
	STJ = Self::letters_to_token(b"STJ"),
	
	/// TODO.
	STK = Self::letters_to_token(b"STK"),
	
	/// TODO.
	STL = Self::letters_to_token(b"STL"),
	
	/// TODO.
	STM = Self::letters_to_token(b"STM"),
	
	/// TODO.
	STN = Self::letters_to_token(b"STN"),
	
	/// TODO.
	STO = Self::letters_to_token(b"STO"),
	
	/// TODO.
	STP = Self::letters_to_token(b"STP"),
	
	/// TODO.
	STQ = Self::letters_to_token(b"STQ"),
	
	/// TODO.
	STR = Self::letters_to_token(b"STR"),
	
	/// TODO.
	STS = Self::letters_to_token(b"STS"),
	
	/// TODO.
	STT = Self::letters_to_token(b"STT"),
	
	/// TODO.
	STU = Self::letters_to_token(b"STU"),
	
	/// TODO.
	STV = Self::letters_to_token(b"STV"),
	
	/// TODO.
	STW = Self::letters_to_token(b"STW"),
	
	/// TODO.
	STX = Self::letters_to_token(b"STX"),
	
	/// TODO.
	STY = Self::letters_to_token(b"STY"),
	
	/// TODO.
	STZ = Self::letters_to_token(b"STZ"),
	
	/// TODO.
	SUA = Self::letters_to_token(b"SUA"),
	
	/// TODO.
	SUB = Self::letters_to_token(b"SUB"),
	
	/// TODO.
	SUC = Self::letters_to_token(b"SUC"),
	
	/// TODO.
	SUD = Self::letters_to_token(b"SUD"),
	
	/// TODO.
	SUE = Self::letters_to_token(b"SUE"),
	
	/// TODO.
	SUF = Self::letters_to_token(b"SUF"),
	
	/// TODO.
	SUG = Self::letters_to_token(b"SUG"),
	
	/// TODO.
	SUH = Self::letters_to_token(b"SUH"),
	
	/// TODO.
	SUI = Self::letters_to_token(b"SUI"),
	
	/// TODO.
	SUJ = Self::letters_to_token(b"SUJ"),
	
	/// TODO.
	SUK = Self::letters_to_token(b"SUK"),
	
	/// TODO.
	SUL = Self::letters_to_token(b"SUL"),
	
	/// TODO.
	SUM = Self::letters_to_token(b"SUM"),
	
	/// Soviet Union.
	SUN = Self::letters_to_token(b"SUN"),
	
	/// TODO.
	SUO = Self::letters_to_token(b"SUO"),
	
	/// TODO.
	SUP = Self::letters_to_token(b"SUP"),
	
	/// TODO.
	SUQ = Self::letters_to_token(b"SUQ"),
	
	/// TODO.
	SUR = Self::letters_to_token(b"SUR"),
	
	/// TODO.
	SUS = Self::letters_to_token(b"SUS"),
	
	/// TODO.
	SUT = Self::letters_to_token(b"SUT"),
	
	/// TODO.
	SUU = Self::letters_to_token(b"SUU"),
	
	/// TODO.
	SUV = Self::letters_to_token(b"SUV"),
	
	/// TODO.
	SUW = Self::letters_to_token(b"SUW"),
	
	/// TODO.
	SUX = Self::letters_to_token(b"SUX"),
	
	/// TODO.
	SUY = Self::letters_to_token(b"SUY"),
	
	/// TODO.
	SUZ = Self::letters_to_token(b"SUZ"),
	
	/// TODO.
	SVA = Self::letters_to_token(b"SVA"),
	
	/// TODO.
	SVB = Self::letters_to_token(b"SVB"),
	
	/// TODO.
	SVC = Self::letters_to_token(b"SVC"),
	
	/// TODO.
	SVD = Self::letters_to_token(b"SVD"),
	
	/// TODO.
	SVE = Self::letters_to_token(b"SVE"),
	
	/// TODO.
	SVF = Self::letters_to_token(b"SVF"),
	
	/// TODO.
	SVG = Self::letters_to_token(b"SVG"),
	
	/// TODO.
	SVH = Self::letters_to_token(b"SVH"),
	
	/// TODO.
	SVI = Self::letters_to_token(b"SVI"),
	
	/// TODO.
	SVJ = Self::letters_to_token(b"SVJ"),
	
	/// TODO.
	SVK = Self::letters_to_token(b"SVK"),
	
	/// TODO.
	SVL = Self::letters_to_token(b"SVL"),
	
	/// TODO.
	SVM = Self::letters_to_token(b"SVM"),
	
	/// TODO.
	SVN = Self::letters_to_token(b"SVN"),
	
	/// TODO.
	SVO = Self::letters_to_token(b"SVO"),
	
	/// TODO.
	SVP = Self::letters_to_token(b"SVP"),
	
	/// TODO.
	SVQ = Self::letters_to_token(b"SVQ"),
	
	/// TODO.
	SVR = Self::letters_to_token(b"SVR"),
	
	/// TODO.
	SVS = Self::letters_to_token(b"SVS"),
	
	/// TODO.
	SVT = Self::letters_to_token(b"SVT"),
	
	/// TODO.
	SVU = Self::letters_to_token(b"SVU"),
	
	/// TODO.
	SVV = Self::letters_to_token(b"SVV"),
	
	/// TODO.
	SVW = Self::letters_to_token(b"SVW"),
	
	/// TODO.
	SVX = Self::letters_to_token(b"SVX"),
	
	/// TODO.
	SVY = Self::letters_to_token(b"SVY"),
	
	/// TODO.
	SVZ = Self::letters_to_token(b"SVZ"),
	
	/// TODO.
	SWA = Self::letters_to_token(b"SWA"),
	
	/// TODO.
	SWB = Self::letters_to_token(b"SWB"),
	
	/// TODO.
	SWC = Self::letters_to_token(b"SWC"),
	
	/// TODO.
	SWD = Self::letters_to_token(b"SWD"),
	
	/// TODO.
	SWE = Self::letters_to_token(b"SWE"),
	
	/// TODO.
	SWF = Self::letters_to_token(b"SWF"),
	
	/// TODO.
	SWG = Self::letters_to_token(b"SWG"),
	
	/// TODO.
	SWH = Self::letters_to_token(b"SWH"),
	
	/// TODO.
	SWI = Self::letters_to_token(b"SWI"),
	
	/// TODO.
	SWJ = Self::letters_to_token(b"SWJ"),
	
	/// TODO.
	SWK = Self::letters_to_token(b"SWK"),
	
	/// TODO.
	SWL = Self::letters_to_token(b"SWL"),
	
	/// TODO.
	SWM = Self::letters_to_token(b"SWM"),
	
	/// TODO.
	SWN = Self::letters_to_token(b"SWN"),
	
	/// TODO.
	SWO = Self::letters_to_token(b"SWO"),
	
	/// TODO.
	SWP = Self::letters_to_token(b"SWP"),
	
	/// TODO.
	SWQ = Self::letters_to_token(b"SWQ"),
	
	/// TODO.
	SWR = Self::letters_to_token(b"SWR"),
	
	/// TODO.
	SWS = Self::letters_to_token(b"SWS"),
	
	/// TODO.
	SWT = Self::letters_to_token(b"SWT"),
	
	/// TODO.
	SWU = Self::letters_to_token(b"SWU"),
	
	/// TODO.
	SWV = Self::letters_to_token(b"SWV"),
	
	/// TODO.
	SWW = Self::letters_to_token(b"SWW"),
	
	/// TODO.
	SWX = Self::letters_to_token(b"SWX"),
	
	/// TODO.
	SWY = Self::letters_to_token(b"SWY"),
	
	/// TODO.
	SWZ = Self::letters_to_token(b"SWZ"),
	
	/// TODO.
	SXA = Self::letters_to_token(b"SXA"),
	
	/// TODO.
	SXB = Self::letters_to_token(b"SXB"),
	
	/// TODO.
	SXC = Self::letters_to_token(b"SXC"),
	
	/// TODO.
	SXD = Self::letters_to_token(b"SXD"),
	
	/// TODO.
	SXE = Self::letters_to_token(b"SXE"),
	
	/// TODO.
	SXF = Self::letters_to_token(b"SXF"),
	
	/// TODO.
	SXG = Self::letters_to_token(b"SXG"),
	
	/// TODO.
	SXH = Self::letters_to_token(b"SXH"),
	
	/// TODO.
	SXI = Self::letters_to_token(b"SXI"),
	
	/// TODO.
	SXJ = Self::letters_to_token(b"SXJ"),
	
	/// TODO.
	SXK = Self::letters_to_token(b"SXK"),
	
	/// TODO.
	SXL = Self::letters_to_token(b"SXL"),
	
	/// TODO.
	SXM = Self::letters_to_token(b"SXM"),
	
	/// TODO.
	SXN = Self::letters_to_token(b"SXN"),
	
	/// TODO.
	SXO = Self::letters_to_token(b"SXO"),
	
	/// TODO.
	SXP = Self::letters_to_token(b"SXP"),
	
	/// TODO.
	SXQ = Self::letters_to_token(b"SXQ"),
	
	/// TODO.
	SXR = Self::letters_to_token(b"SXR"),
	
	/// TODO.
	SXS = Self::letters_to_token(b"SXS"),
	
	/// TODO.
	SXT = Self::letters_to_token(b"SXT"),
	
	/// TODO.
	SXU = Self::letters_to_token(b"SXU"),
	
	/// TODO.
	SXV = Self::letters_to_token(b"SXV"),
	
	/// TODO.
	SXW = Self::letters_to_token(b"SXW"),
	
	/// TODO.
	SXX = Self::letters_to_token(b"SXX"),
	
	/// TODO.
	SXY = Self::letters_to_token(b"SXY"),
	
	/// TODO.
	SXZ = Self::letters_to_token(b"SXZ"),
	
	/// TODO.
	SYA = Self::letters_to_token(b"SYA"),
	
	/// TODO.
	SYB = Self::letters_to_token(b"SYB"),
	
	/// TODO.
	SYC = Self::letters_to_token(b"SYC"),
	
	/// TODO.
	SYD = Self::letters_to_token(b"SYD"),
	
	/// TODO.
	SYE = Self::letters_to_token(b"SYE"),
	
	/// TODO.
	SYF = Self::letters_to_token(b"SYF"),
	
	/// TODO.
	SYG = Self::letters_to_token(b"SYG"),
	
	/// TODO.
	SYH = Self::letters_to_token(b"SYH"),
	
	/// TODO.
	SYI = Self::letters_to_token(b"SYI"),
	
	/// TODO.
	SYJ = Self::letters_to_token(b"SYJ"),
	
	/// TODO.
	SYK = Self::letters_to_token(b"SYK"),
	
	/// TODO.
	SYL = Self::letters_to_token(b"SYL"),
	
	/// TODO.
	SYM = Self::letters_to_token(b"SYM"),
	
	/// TODO.
	SYN = Self::letters_to_token(b"SYN"),
	
	/// TODO.
	SYO = Self::letters_to_token(b"SYO"),
	
	/// TODO.
	SYP = Self::letters_to_token(b"SYP"),
	
	/// TODO.
	SYQ = Self::letters_to_token(b"SYQ"),
	
	/// TODO.
	SYR = Self::letters_to_token(b"SYR"),
	
	/// TODO.
	SYS = Self::letters_to_token(b"SYS"),
	
	/// TODO.
	SYT = Self::letters_to_token(b"SYT"),
	
	/// TODO.
	SYU = Self::letters_to_token(b"SYU"),
	
	/// TODO.
	SYV = Self::letters_to_token(b"SYV"),
	
	/// TODO.
	SYW = Self::letters_to_token(b"SYW"),
	
	/// TODO.
	SYX = Self::letters_to_token(b"SYX"),
	
	/// TODO.
	SYY = Self::letters_to_token(b"SYY"),
	
	/// TODO.
	SYZ = Self::letters_to_token(b"SYZ"),
	
	/// TODO.
	SZA = Self::letters_to_token(b"SZA"),
	
	/// TODO.
	SZB = Self::letters_to_token(b"SZB"),
	
	/// TODO.
	SZC = Self::letters_to_token(b"SZC"),
	
	/// TODO.
	SZD = Self::letters_to_token(b"SZD"),
	
	/// TODO.
	SZE = Self::letters_to_token(b"SZE"),
	
	/// TODO.
	SZF = Self::letters_to_token(b"SZF"),
	
	/// TODO.
	SZG = Self::letters_to_token(b"SZG"),
	
	/// TODO.
	SZH = Self::letters_to_token(b"SZH"),
	
	/// TODO.
	SZI = Self::letters_to_token(b"SZI"),
	
	/// TODO.
	SZJ = Self::letters_to_token(b"SZJ"),
	
	/// TODO.
	SZK = Self::letters_to_token(b"SZK"),
	
	/// TODO.
	SZL = Self::letters_to_token(b"SZL"),
	
	/// TODO.
	SZM = Self::letters_to_token(b"SZM"),
	
	/// TODO.
	SZN = Self::letters_to_token(b"SZN"),
	
	/// TODO.
	SZO = Self::letters_to_token(b"SZO"),
	
	/// TODO.
	SZP = Self::letters_to_token(b"SZP"),
	
	/// TODO.
	SZQ = Self::letters_to_token(b"SZQ"),
	
	/// TODO.
	SZR = Self::letters_to_token(b"SZR"),
	
	/// TODO.
	SZS = Self::letters_to_token(b"SZS"),
	
	/// TODO.
	SZT = Self::letters_to_token(b"SZT"),
	
	/// TODO.
	SZU = Self::letters_to_token(b"SZU"),
	
	/// TODO.
	SZV = Self::letters_to_token(b"SZV"),
	
	/// TODO.
	SZW = Self::letters_to_token(b"SZW"),
	
	/// TODO.
	SZX = Self::letters_to_token(b"SZX"),
	
	/// TODO.
	SZY = Self::letters_to_token(b"SZY"),
	
	/// TODO.
	SZZ = Self::letters_to_token(b"SZZ"),
	
	/// Exceptional reservation.
	///
	/// Tristan da Cunha.
	TAA = Self::letters_to_token(b"TAA"),
	
	/// TODO.
	TAB = Self::letters_to_token(b"TAB"),
	
	/// TODO.
	TAC = Self::letters_to_token(b"TAC"),
	
	/// TODO.
	TAD = Self::letters_to_token(b"TAD"),
	
	/// TODO.
	TAE = Self::letters_to_token(b"TAE"),
	
	/// TODO.
	TAF = Self::letters_to_token(b"TAF"),
	
	/// TODO.
	TAG = Self::letters_to_token(b"TAG"),
	
	/// TODO.
	TAH = Self::letters_to_token(b"TAH"),
	
	/// TODO.
	TAI = Self::letters_to_token(b"TAI"),
	
	/// TODO.
	TAJ = Self::letters_to_token(b"TAJ"),
	
	/// TODO.
	TAK = Self::letters_to_token(b"TAK"),
	
	/// TODO.
	TAL = Self::letters_to_token(b"TAL"),
	
	/// TODO.
	TAM = Self::letters_to_token(b"TAM"),
	
	/// TODO.
	TAN = Self::letters_to_token(b"TAN"),
	
	/// TODO.
	TAO = Self::letters_to_token(b"TAO"),
	
	/// TODO.
	TAP = Self::letters_to_token(b"TAP"),
	
	/// TODO.
	TAQ = Self::letters_to_token(b"TAQ"),
	
	/// TODO.
	TAR = Self::letters_to_token(b"TAR"),
	
	/// TODO.
	TAS = Self::letters_to_token(b"TAS"),
	
	/// TODO.
	TAT = Self::letters_to_token(b"TAT"),
	
	/// TODO.
	TAU = Self::letters_to_token(b"TAU"),
	
	/// TODO.
	TAV = Self::letters_to_token(b"TAV"),
	
	/// TODO.
	TAW = Self::letters_to_token(b"TAW"),
	
	/// TODO.
	TAX = Self::letters_to_token(b"TAX"),
	
	/// TODO.
	TAY = Self::letters_to_token(b"TAY"),
	
	/// TODO.
	TAZ = Self::letters_to_token(b"TAZ"),
	
	/// TODO.
	TBA = Self::letters_to_token(b"TBA"),
	
	/// TODO.
	TBB = Self::letters_to_token(b"TBB"),
	
	/// TODO.
	TBC = Self::letters_to_token(b"TBC"),
	
	/// TODO.
	TBD = Self::letters_to_token(b"TBD"),
	
	/// TODO.
	TBE = Self::letters_to_token(b"TBE"),
	
	/// TODO.
	TBF = Self::letters_to_token(b"TBF"),
	
	/// TODO.
	TBG = Self::letters_to_token(b"TBG"),
	
	/// TODO.
	TBH = Self::letters_to_token(b"TBH"),
	
	/// TODO.
	TBI = Self::letters_to_token(b"TBI"),
	
	/// TODO.
	TBJ = Self::letters_to_token(b"TBJ"),
	
	/// TODO.
	TBK = Self::letters_to_token(b"TBK"),
	
	/// TODO.
	TBL = Self::letters_to_token(b"TBL"),
	
	/// TODO.
	TBM = Self::letters_to_token(b"TBM"),
	
	/// TODO.
	TBN = Self::letters_to_token(b"TBN"),
	
	/// TODO.
	TBO = Self::letters_to_token(b"TBO"),
	
	/// TODO.
	TBP = Self::letters_to_token(b"TBP"),
	
	/// TODO.
	TBQ = Self::letters_to_token(b"TBQ"),
	
	/// TODO.
	TBR = Self::letters_to_token(b"TBR"),
	
	/// TODO.
	TBS = Self::letters_to_token(b"TBS"),
	
	/// TODO.
	TBT = Self::letters_to_token(b"TBT"),
	
	/// TODO.
	TBU = Self::letters_to_token(b"TBU"),
	
	/// TODO.
	TBV = Self::letters_to_token(b"TBV"),
	
	/// TODO.
	TBW = Self::letters_to_token(b"TBW"),
	
	/// TODO.
	TBX = Self::letters_to_token(b"TBX"),
	
	/// TODO.
	TBY = Self::letters_to_token(b"TBY"),
	
	/// TODO.
	TBZ = Self::letters_to_token(b"TBZ"),
	
	/// TODO.
	TCA = Self::letters_to_token(b"TCA"),
	
	/// TODO.
	TCB = Self::letters_to_token(b"TCB"),
	
	/// TODO.
	TCC = Self::letters_to_token(b"TCC"),
	
	/// TODO.
	TCD = Self::letters_to_token(b"TCD"),
	
	/// TODO.
	TCE = Self::letters_to_token(b"TCE"),
	
	/// TODO.
	TCF = Self::letters_to_token(b"TCF"),
	
	/// TODO.
	TCG = Self::letters_to_token(b"TCG"),
	
	/// TODO.
	TCH = Self::letters_to_token(b"TCH"),
	
	/// TODO.
	TCI = Self::letters_to_token(b"TCI"),
	
	/// TODO.
	TCJ = Self::letters_to_token(b"TCJ"),
	
	/// TODO.
	TCK = Self::letters_to_token(b"TCK"),
	
	/// TODO.
	TCL = Self::letters_to_token(b"TCL"),
	
	/// TODO.
	TCM = Self::letters_to_token(b"TCM"),
	
	/// TODO.
	TCN = Self::letters_to_token(b"TCN"),
	
	/// TODO.
	TCO = Self::letters_to_token(b"TCO"),
	
	/// TODO.
	TCP = Self::letters_to_token(b"TCP"),
	
	/// TODO.
	TCQ = Self::letters_to_token(b"TCQ"),
	
	/// TODO.
	TCR = Self::letters_to_token(b"TCR"),
	
	/// TODO.
	TCS = Self::letters_to_token(b"TCS"),
	
	/// TODO.
	TCT = Self::letters_to_token(b"TCT"),
	
	/// TODO.
	TCU = Self::letters_to_token(b"TCU"),
	
	/// TODO.
	TCV = Self::letters_to_token(b"TCV"),
	
	/// TODO.
	TCW = Self::letters_to_token(b"TCW"),
	
	/// TODO.
	TCX = Self::letters_to_token(b"TCX"),
	
	/// TODO.
	TCY = Self::letters_to_token(b"TCY"),
	
	/// TODO.
	TCZ = Self::letters_to_token(b"TCZ"),
	
	/// TODO.
	TDA = Self::letters_to_token(b"TDA"),
	
	/// TODO.
	TDB = Self::letters_to_token(b"TDB"),
	
	/// TODO.
	TDC = Self::letters_to_token(b"TDC"),
	
	/// TODO.
	TDD = Self::letters_to_token(b"TDD"),
	
	/// TODO.
	TDE = Self::letters_to_token(b"TDE"),
	
	/// TODO.
	TDF = Self::letters_to_token(b"TDF"),
	
	/// TODO.
	TDG = Self::letters_to_token(b"TDG"),
	
	/// TODO.
	TDH = Self::letters_to_token(b"TDH"),
	
	/// TODO.
	TDI = Self::letters_to_token(b"TDI"),
	
	/// TODO.
	TDJ = Self::letters_to_token(b"TDJ"),
	
	/// TODO.
	TDK = Self::letters_to_token(b"TDK"),
	
	/// TODO.
	TDL = Self::letters_to_token(b"TDL"),
	
	/// TODO.
	TDM = Self::letters_to_token(b"TDM"),
	
	/// TODO.
	TDN = Self::letters_to_token(b"TDN"),
	
	/// TODO.
	TDO = Self::letters_to_token(b"TDO"),
	
	/// TODO.
	TDP = Self::letters_to_token(b"TDP"),
	
	/// TODO.
	TDQ = Self::letters_to_token(b"TDQ"),
	
	/// TODO.
	TDR = Self::letters_to_token(b"TDR"),
	
	/// TODO.
	TDS = Self::letters_to_token(b"TDS"),
	
	/// TODO.
	TDT = Self::letters_to_token(b"TDT"),
	
	/// TODO.
	TDU = Self::letters_to_token(b"TDU"),
	
	/// TODO.
	TDV = Self::letters_to_token(b"TDV"),
	
	/// TODO.
	TDW = Self::letters_to_token(b"TDW"),
	
	/// TODO.
	TDX = Self::letters_to_token(b"TDX"),
	
	/// TODO.
	TDY = Self::letters_to_token(b"TDY"),
	
	/// TODO.
	TDZ = Self::letters_to_token(b"TDZ"),
	
	/// TODO.
	TEA = Self::letters_to_token(b"TEA"),
	
	/// TODO.
	TEB = Self::letters_to_token(b"TEB"),
	
	/// TODO.
	TEC = Self::letters_to_token(b"TEC"),
	
	/// TODO.
	TED = Self::letters_to_token(b"TED"),
	
	/// TODO.
	TEE = Self::letters_to_token(b"TEE"),
	
	/// TODO.
	TEF = Self::letters_to_token(b"TEF"),
	
	/// TODO.
	TEG = Self::letters_to_token(b"TEG"),
	
	/// TODO.
	TEH = Self::letters_to_token(b"TEH"),
	
	/// TODO.
	TEI = Self::letters_to_token(b"TEI"),
	
	/// TODO.
	TEJ = Self::letters_to_token(b"TEJ"),
	
	/// TODO.
	TEK = Self::letters_to_token(b"TEK"),
	
	/// TODO.
	TEL = Self::letters_to_token(b"TEL"),
	
	/// TODO.
	TEM = Self::letters_to_token(b"TEM"),
	
	/// TODO.
	TEN = Self::letters_to_token(b"TEN"),
	
	/// TODO.
	TEO = Self::letters_to_token(b"TEO"),
	
	/// TODO.
	TEP = Self::letters_to_token(b"TEP"),
	
	/// TODO.
	TEQ = Self::letters_to_token(b"TEQ"),
	
	/// TODO.
	TER = Self::letters_to_token(b"TER"),
	
	/// TODO.
	TES = Self::letters_to_token(b"TES"),
	
	/// TODO.
	TET = Self::letters_to_token(b"TET"),
	
	/// TODO.
	TEU = Self::letters_to_token(b"TEU"),
	
	/// TODO.
	TEV = Self::letters_to_token(b"TEV"),
	
	/// TODO.
	TEW = Self::letters_to_token(b"TEW"),
	
	/// TODO.
	TEX = Self::letters_to_token(b"TEX"),
	
	/// TODO.
	TEY = Self::letters_to_token(b"TEY"),
	
	/// TODO.
	TEZ = Self::letters_to_token(b"TEZ"),
	
	/// TODO.
	TFA = Self::letters_to_token(b"TFA"),
	
	/// TODO.
	TFB = Self::letters_to_token(b"TFB"),
	
	/// TODO.
	TFC = Self::letters_to_token(b"TFC"),
	
	/// TODO.
	TFD = Self::letters_to_token(b"TFD"),
	
	/// TODO.
	TFE = Self::letters_to_token(b"TFE"),
	
	/// TODO.
	TFF = Self::letters_to_token(b"TFF"),
	
	/// TODO.
	TFG = Self::letters_to_token(b"TFG"),
	
	/// TODO.
	TFH = Self::letters_to_token(b"TFH"),
	
	/// TODO.
	TFI = Self::letters_to_token(b"TFI"),
	
	/// TODO.
	TFJ = Self::letters_to_token(b"TFJ"),
	
	/// TODO.
	TFK = Self::letters_to_token(b"TFK"),
	
	/// TODO.
	TFL = Self::letters_to_token(b"TFL"),
	
	/// TODO.
	TFM = Self::letters_to_token(b"TFM"),
	
	/// TODO.
	TFN = Self::letters_to_token(b"TFN"),
	
	/// TODO.
	TFO = Self::letters_to_token(b"TFO"),
	
	/// TODO.
	TFP = Self::letters_to_token(b"TFP"),
	
	/// TODO.
	TFQ = Self::letters_to_token(b"TFQ"),
	
	/// TODO.
	TFR = Self::letters_to_token(b"TFR"),
	
	/// TODO.
	TFS = Self::letters_to_token(b"TFS"),
	
	/// TODO.
	TFT = Self::letters_to_token(b"TFT"),
	
	/// TODO.
	TFU = Self::letters_to_token(b"TFU"),
	
	/// TODO.
	TFV = Self::letters_to_token(b"TFV"),
	
	/// TODO.
	TFW = Self::letters_to_token(b"TFW"),
	
	/// TODO.
	TFX = Self::letters_to_token(b"TFX"),
	
	/// TODO.
	TFY = Self::letters_to_token(b"TFY"),
	
	/// TODO.
	TFZ = Self::letters_to_token(b"TFZ"),
	
	/// TODO.
	TGA = Self::letters_to_token(b"TGA"),
	
	/// TODO.
	TGB = Self::letters_to_token(b"TGB"),
	
	/// TODO.
	TGC = Self::letters_to_token(b"TGC"),
	
	/// TODO.
	TGD = Self::letters_to_token(b"TGD"),
	
	/// TODO.
	TGE = Self::letters_to_token(b"TGE"),
	
	/// TODO.
	TGF = Self::letters_to_token(b"TGF"),
	
	/// TODO.
	TGG = Self::letters_to_token(b"TGG"),
	
	/// TODO.
	TGH = Self::letters_to_token(b"TGH"),
	
	/// TODO.
	TGI = Self::letters_to_token(b"TGI"),
	
	/// TODO.
	TGJ = Self::letters_to_token(b"TGJ"),
	
	/// TODO.
	TGK = Self::letters_to_token(b"TGK"),
	
	/// TODO.
	TGL = Self::letters_to_token(b"TGL"),
	
	/// TODO.
	TGM = Self::letters_to_token(b"TGM"),
	
	/// TODO.
	TGN = Self::letters_to_token(b"TGN"),
	
	/// TODO.
	TGO = Self::letters_to_token(b"TGO"),
	
	/// TODO.
	TGP = Self::letters_to_token(b"TGP"),
	
	/// TODO.
	TGQ = Self::letters_to_token(b"TGQ"),
	
	/// TODO.
	TGR = Self::letters_to_token(b"TGR"),
	
	/// TODO.
	TGS = Self::letters_to_token(b"TGS"),
	
	/// TODO.
	TGT = Self::letters_to_token(b"TGT"),
	
	/// TODO.
	TGU = Self::letters_to_token(b"TGU"),
	
	/// TODO.
	TGV = Self::letters_to_token(b"TGV"),
	
	/// TODO.
	TGW = Self::letters_to_token(b"TGW"),
	
	/// TODO.
	TGX = Self::letters_to_token(b"TGX"),
	
	/// TODO.
	TGY = Self::letters_to_token(b"TGY"),
	
	/// TODO.
	TGZ = Self::letters_to_token(b"TGZ"),
	
	/// TODO.
	THA = Self::letters_to_token(b"THA"),
	
	/// TODO.
	THB = Self::letters_to_token(b"THB"),
	
	/// TODO.
	THC = Self::letters_to_token(b"THC"),
	
	/// TODO.
	THD = Self::letters_to_token(b"THD"),
	
	/// TODO.
	THE = Self::letters_to_token(b"THE"),
	
	/// TODO.
	THF = Self::letters_to_token(b"THF"),
	
	/// TODO.
	THG = Self::letters_to_token(b"THG"),
	
	/// TODO.
	THH = Self::letters_to_token(b"THH"),
	
	/// TODO.
	THI = Self::letters_to_token(b"THI"),
	
	/// TODO.
	THJ = Self::letters_to_token(b"THJ"),
	
	/// TODO.
	THK = Self::letters_to_token(b"THK"),
	
	/// TODO.
	THL = Self::letters_to_token(b"THL"),
	
	/// TODO.
	THM = Self::letters_to_token(b"THM"),
	
	/// TODO.
	THN = Self::letters_to_token(b"THN"),
	
	/// TODO.
	THO = Self::letters_to_token(b"THO"),
	
	/// TODO.
	THP = Self::letters_to_token(b"THP"),
	
	/// TODO.
	THQ = Self::letters_to_token(b"THQ"),
	
	/// TODO.
	THR = Self::letters_to_token(b"THR"),
	
	/// TODO.
	THS = Self::letters_to_token(b"THS"),
	
	/// TODO.
	THT = Self::letters_to_token(b"THT"),
	
	/// TODO.
	THU = Self::letters_to_token(b"THU"),
	
	/// TODO.
	THV = Self::letters_to_token(b"THV"),
	
	/// TODO.
	THW = Self::letters_to_token(b"THW"),
	
	/// TODO.
	THX = Self::letters_to_token(b"THX"),
	
	/// TODO.
	THY = Self::letters_to_token(b"THY"),
	
	/// TODO.
	THZ = Self::letters_to_token(b"THZ"),
	
	/// TODO.
	TIA = Self::letters_to_token(b"TIA"),
	
	/// TODO.
	TIB = Self::letters_to_token(b"TIB"),
	
	/// TODO.
	TIC = Self::letters_to_token(b"TIC"),
	
	/// TODO.
	TID = Self::letters_to_token(b"TID"),
	
	/// TODO.
	TIE = Self::letters_to_token(b"TIE"),
	
	/// TODO.
	TIF = Self::letters_to_token(b"TIF"),
	
	/// TODO.
	TIG = Self::letters_to_token(b"TIG"),
	
	/// TODO.
	TIH = Self::letters_to_token(b"TIH"),
	
	/// TODO.
	TII = Self::letters_to_token(b"TII"),
	
	/// TODO.
	TIJ = Self::letters_to_token(b"TIJ"),
	
	/// TODO.
	TIK = Self::letters_to_token(b"TIK"),
	
	/// TODO.
	TIL = Self::letters_to_token(b"TIL"),
	
	/// TODO.
	TIM = Self::letters_to_token(b"TIM"),
	
	/// TODO.
	TIN = Self::letters_to_token(b"TIN"),
	
	/// TODO.
	TIO = Self::letters_to_token(b"TIO"),
	
	/// TODO.
	TIP = Self::letters_to_token(b"TIP"),
	
	/// TODO.
	TIQ = Self::letters_to_token(b"TIQ"),
	
	/// TODO.
	TIR = Self::letters_to_token(b"TIR"),
	
	/// TODO.
	TIS = Self::letters_to_token(b"TIS"),
	
	/// TODO.
	TIT = Self::letters_to_token(b"TIT"),
	
	/// TODO.
	TIU = Self::letters_to_token(b"TIU"),
	
	/// TODO.
	TIV = Self::letters_to_token(b"TIV"),
	
	/// TODO.
	TIW = Self::letters_to_token(b"TIW"),
	
	/// TODO.
	TIX = Self::letters_to_token(b"TIX"),
	
	/// TODO.
	TIY = Self::letters_to_token(b"TIY"),
	
	/// TODO.
	TIZ = Self::letters_to_token(b"TIZ"),
	
	/// TODO.
	TJA = Self::letters_to_token(b"TJA"),
	
	/// TODO.
	TJB = Self::letters_to_token(b"TJB"),
	
	/// TODO.
	TJC = Self::letters_to_token(b"TJC"),
	
	/// TODO.
	TJD = Self::letters_to_token(b"TJD"),
	
	/// TODO.
	TJE = Self::letters_to_token(b"TJE"),
	
	/// TODO.
	TJF = Self::letters_to_token(b"TJF"),
	
	/// TODO.
	TJG = Self::letters_to_token(b"TJG"),
	
	/// TODO.
	TJH = Self::letters_to_token(b"TJH"),
	
	/// TODO.
	TJI = Self::letters_to_token(b"TJI"),
	
	/// TODO.
	TJJ = Self::letters_to_token(b"TJJ"),
	
	/// TODO.
	TJK = Self::letters_to_token(b"TJK"),
	
	/// TODO.
	TJL = Self::letters_to_token(b"TJL"),
	
	/// TODO.
	TJM = Self::letters_to_token(b"TJM"),
	
	/// TODO.
	TJN = Self::letters_to_token(b"TJN"),
	
	/// TODO.
	TJO = Self::letters_to_token(b"TJO"),
	
	/// TODO.
	TJP = Self::letters_to_token(b"TJP"),
	
	/// TODO.
	TJQ = Self::letters_to_token(b"TJQ"),
	
	/// TODO.
	TJR = Self::letters_to_token(b"TJR"),
	
	/// TODO.
	TJS = Self::letters_to_token(b"TJS"),
	
	/// TODO.
	TJT = Self::letters_to_token(b"TJT"),
	
	/// TODO.
	TJU = Self::letters_to_token(b"TJU"),
	
	/// TODO.
	TJV = Self::letters_to_token(b"TJV"),
	
	/// TODO.
	TJW = Self::letters_to_token(b"TJW"),
	
	/// TODO.
	TJX = Self::letters_to_token(b"TJX"),
	
	/// TODO.
	TJY = Self::letters_to_token(b"TJY"),
	
	/// TODO.
	TJZ = Self::letters_to_token(b"TJZ"),
	
	/// TODO.
	TKA = Self::letters_to_token(b"TKA"),
	
	/// TODO.
	TKB = Self::letters_to_token(b"TKB"),
	
	/// TODO.
	TKC = Self::letters_to_token(b"TKC"),
	
	/// TODO.
	TKD = Self::letters_to_token(b"TKD"),
	
	/// TODO.
	TKE = Self::letters_to_token(b"TKE"),
	
	/// TODO.
	TKF = Self::letters_to_token(b"TKF"),
	
	/// TODO.
	TKG = Self::letters_to_token(b"TKG"),
	
	/// TODO.
	TKH = Self::letters_to_token(b"TKH"),
	
	/// TODO.
	TKI = Self::letters_to_token(b"TKI"),
	
	/// TODO.
	TKJ = Self::letters_to_token(b"TKJ"),
	
	/// TODO.
	TKK = Self::letters_to_token(b"TKK"),
	
	/// TODO.
	TKL = Self::letters_to_token(b"TKL"),
	
	/// TODO.
	TKM = Self::letters_to_token(b"TKM"),
	
	/// TODO.
	TKN = Self::letters_to_token(b"TKN"),
	
	/// TODO.
	TKO = Self::letters_to_token(b"TKO"),
	
	/// TODO.
	TKP = Self::letters_to_token(b"TKP"),
	
	/// TODO.
	TKQ = Self::letters_to_token(b"TKQ"),
	
	/// TODO.
	TKR = Self::letters_to_token(b"TKR"),
	
	/// TODO.
	TKS = Self::letters_to_token(b"TKS"),
	
	/// TODO.
	TKT = Self::letters_to_token(b"TKT"),
	
	/// TODO.
	TKU = Self::letters_to_token(b"TKU"),
	
	/// TODO.
	TKV = Self::letters_to_token(b"TKV"),
	
	/// TODO.
	TKW = Self::letters_to_token(b"TKW"),
	
	/// TODO.
	TKX = Self::letters_to_token(b"TKX"),
	
	/// TODO.
	TKY = Self::letters_to_token(b"TKY"),
	
	/// TODO.
	TKZ = Self::letters_to_token(b"TKZ"),
	
	/// TODO.
	TLA = Self::letters_to_token(b"TLA"),
	
	/// TODO.
	TLB = Self::letters_to_token(b"TLB"),
	
	/// TODO.
	TLC = Self::letters_to_token(b"TLC"),
	
	/// TODO.
	TLD = Self::letters_to_token(b"TLD"),
	
	/// TODO.
	TLE = Self::letters_to_token(b"TLE"),
	
	/// TODO.
	TLF = Self::letters_to_token(b"TLF"),
	
	/// TODO.
	TLG = Self::letters_to_token(b"TLG"),
	
	/// TODO.
	TLH = Self::letters_to_token(b"TLH"),
	
	/// TODO.
	TLI = Self::letters_to_token(b"TLI"),
	
	/// TODO.
	TLJ = Self::letters_to_token(b"TLJ"),
	
	/// TODO.
	TLK = Self::letters_to_token(b"TLK"),
	
	/// TODO.
	TLL = Self::letters_to_token(b"TLL"),
	
	/// TODO.
	TLM = Self::letters_to_token(b"TLM"),
	
	/// TODO.
	TLN = Self::letters_to_token(b"TLN"),
	
	/// TODO.
	TLO = Self::letters_to_token(b"TLO"),
	
	/// TODO.
	TLP = Self::letters_to_token(b"TLP"),
	
	/// TODO.
	TLQ = Self::letters_to_token(b"TLQ"),
	
	/// TODO.
	TLR = Self::letters_to_token(b"TLR"),
	
	/// TODO.
	TLS = Self::letters_to_token(b"TLS"),
	
	/// TODO.
	TLT = Self::letters_to_token(b"TLT"),
	
	/// TODO.
	TLU = Self::letters_to_token(b"TLU"),
	
	/// TODO.
	TLV = Self::letters_to_token(b"TLV"),
	
	/// TODO.
	TLW = Self::letters_to_token(b"TLW"),
	
	/// TODO.
	TLX = Self::letters_to_token(b"TLX"),
	
	/// TODO.
	TLY = Self::letters_to_token(b"TLY"),
	
	/// TODO.
	TLZ = Self::letters_to_token(b"TLZ"),
	
	/// TODO.
	TMA = Self::letters_to_token(b"TMA"),
	
	/// TODO.
	TMB = Self::letters_to_token(b"TMB"),
	
	/// TODO.
	TMC = Self::letters_to_token(b"TMC"),
	
	/// TODO.
	TMD = Self::letters_to_token(b"TMD"),
	
	/// TODO.
	TME = Self::letters_to_token(b"TME"),
	
	/// TODO.
	TMF = Self::letters_to_token(b"TMF"),
	
	/// TODO.
	TMG = Self::letters_to_token(b"TMG"),
	
	/// TODO.
	TMH = Self::letters_to_token(b"TMH"),
	
	/// TODO.
	TMI = Self::letters_to_token(b"TMI"),
	
	/// TODO.
	TMJ = Self::letters_to_token(b"TMJ"),
	
	/// TODO.
	TMK = Self::letters_to_token(b"TMK"),
	
	/// TODO.
	TML = Self::letters_to_token(b"TML"),
	
	/// TODO.
	TMM = Self::letters_to_token(b"TMM"),
	
	/// TODO.
	TMN = Self::letters_to_token(b"TMN"),
	
	/// TODO.
	TMO = Self::letters_to_token(b"TMO"),
	
	/// TODO.
	TMP = Self::letters_to_token(b"TMP"),
	
	/// TODO.
	TMQ = Self::letters_to_token(b"TMQ"),
	
	/// TODO.
	TMR = Self::letters_to_token(b"TMR"),
	
	/// TODO.
	TMS = Self::letters_to_token(b"TMS"),
	
	/// TODO.
	TMT = Self::letters_to_token(b"TMT"),
	
	/// TODO.
	TMU = Self::letters_to_token(b"TMU"),
	
	/// TODO.
	TMV = Self::letters_to_token(b"TMV"),
	
	/// TODO.
	TMW = Self::letters_to_token(b"TMW"),
	
	/// TODO.
	TMX = Self::letters_to_token(b"TMX"),
	
	/// TODO.
	TMY = Self::letters_to_token(b"TMY"),
	
	/// TODO.
	TMZ = Self::letters_to_token(b"TMZ"),
	
	/// TODO.
	TNA = Self::letters_to_token(b"TNA"),
	
	/// TODO.
	TNB = Self::letters_to_token(b"TNB"),
	
	/// TODO.
	TNC = Self::letters_to_token(b"TNC"),
	
	/// TODO.
	TND = Self::letters_to_token(b"TND"),
	
	/// TODO.
	TNE = Self::letters_to_token(b"TNE"),
	
	/// TODO.
	TNF = Self::letters_to_token(b"TNF"),
	
	/// TODO.
	TNG = Self::letters_to_token(b"TNG"),
	
	/// TODO.
	TNH = Self::letters_to_token(b"TNH"),
	
	/// TODO.
	TNI = Self::letters_to_token(b"TNI"),
	
	/// TODO.
	TNJ = Self::letters_to_token(b"TNJ"),
	
	/// TODO.
	TNK = Self::letters_to_token(b"TNK"),
	
	/// TODO.
	TNL = Self::letters_to_token(b"TNL"),
	
	/// TODO.
	TNM = Self::letters_to_token(b"TNM"),
	
	/// TODO.
	TNN = Self::letters_to_token(b"TNN"),
	
	/// TODO.
	TNO = Self::letters_to_token(b"TNO"),
	
	/// TODO.
	TNP = Self::letters_to_token(b"TNP"),
	
	/// TODO.
	TNQ = Self::letters_to_token(b"TNQ"),
	
	/// TODO.
	TNR = Self::letters_to_token(b"TNR"),
	
	/// TODO.
	TNS = Self::letters_to_token(b"TNS"),
	
	/// TODO.
	TNT = Self::letters_to_token(b"TNT"),
	
	/// TODO.
	TNU = Self::letters_to_token(b"TNU"),
	
	/// TODO.
	TNV = Self::letters_to_token(b"TNV"),
	
	/// TODO.
	TNW = Self::letters_to_token(b"TNW"),
	
	/// TODO.
	TNX = Self::letters_to_token(b"TNX"),
	
	/// TODO.
	TNY = Self::letters_to_token(b"TNY"),
	
	/// TODO.
	TNZ = Self::letters_to_token(b"TNZ"),
	
	/// TODO.
	TOA = Self::letters_to_token(b"TOA"),
	
	/// TODO.
	TOB = Self::letters_to_token(b"TOB"),
	
	/// TODO.
	TOC = Self::letters_to_token(b"TOC"),
	
	/// TODO.
	TOD = Self::letters_to_token(b"TOD"),
	
	/// TODO.
	TOE = Self::letters_to_token(b"TOE"),
	
	/// TODO.
	TOF = Self::letters_to_token(b"TOF"),
	
	/// TODO.
	TOG = Self::letters_to_token(b"TOG"),
	
	/// TODO.
	TOH = Self::letters_to_token(b"TOH"),
	
	/// TODO.
	TOI = Self::letters_to_token(b"TOI"),
	
	/// TODO.
	TOJ = Self::letters_to_token(b"TOJ"),
	
	/// TODO.
	TOK = Self::letters_to_token(b"TOK"),
	
	/// TODO.
	TOL = Self::letters_to_token(b"TOL"),
	
	/// TODO.
	TOM = Self::letters_to_token(b"TOM"),
	
	/// TODO.
	TON = Self::letters_to_token(b"TON"),
	
	/// TODO.
	TOO = Self::letters_to_token(b"TOO"),
	
	/// TODO.
	TOP = Self::letters_to_token(b"TOP"),
	
	/// TODO.
	TOQ = Self::letters_to_token(b"TOQ"),
	
	/// TODO.
	TOR = Self::letters_to_token(b"TOR"),
	
	/// TODO.
	TOS = Self::letters_to_token(b"TOS"),
	
	/// TODO.
	TOT = Self::letters_to_token(b"TOT"),
	
	/// TODO.
	TOU = Self::letters_to_token(b"TOU"),
	
	/// TODO.
	TOV = Self::letters_to_token(b"TOV"),
	
	/// TODO.
	TOW = Self::letters_to_token(b"TOW"),
	
	/// TODO.
	TOX = Self::letters_to_token(b"TOX"),
	
	/// TODO.
	TOY = Self::letters_to_token(b"TOY"),
	
	/// TODO.
	TOZ = Self::letters_to_token(b"TOZ"),
	
	/// TODO.
	TPA = Self::letters_to_token(b"TPA"),
	
	/// TODO.
	TPB = Self::letters_to_token(b"TPB"),
	
	/// TODO.
	TPC = Self::letters_to_token(b"TPC"),
	
	/// TODO.
	TPD = Self::letters_to_token(b"TPD"),
	
	/// TODO.
	TPE = Self::letters_to_token(b"TPE"),
	
	/// TODO.
	TPF = Self::letters_to_token(b"TPF"),
	
	/// TODO.
	TPG = Self::letters_to_token(b"TPG"),
	
	/// TODO.
	TPH = Self::letters_to_token(b"TPH"),
	
	/// TODO.
	TPI = Self::letters_to_token(b"TPI"),
	
	/// TODO.
	TPJ = Self::letters_to_token(b"TPJ"),
	
	/// TODO.
	TPK = Self::letters_to_token(b"TPK"),
	
	/// TODO.
	TPL = Self::letters_to_token(b"TPL"),
	
	/// TODO.
	TPM = Self::letters_to_token(b"TPM"),
	
	/// TODO.
	TPN = Self::letters_to_token(b"TPN"),
	
	/// TODO.
	TPO = Self::letters_to_token(b"TPO"),
	
	/// TODO.
	TPP = Self::letters_to_token(b"TPP"),
	
	/// TODO.
	TPQ = Self::letters_to_token(b"TPQ"),
	
	/// TODO.
	TPR = Self::letters_to_token(b"TPR"),
	
	/// TODO.
	TPS = Self::letters_to_token(b"TPS"),
	
	/// TODO.
	TPT = Self::letters_to_token(b"TPT"),
	
	/// TODO.
	TPU = Self::letters_to_token(b"TPU"),
	
	/// TODO.
	TPV = Self::letters_to_token(b"TPV"),
	
	/// TODO.
	TPW = Self::letters_to_token(b"TPW"),
	
	/// TODO.
	TPX = Self::letters_to_token(b"TPX"),
	
	/// TODO.
	TPY = Self::letters_to_token(b"TPY"),
	
	/// TODO.
	TPZ = Self::letters_to_token(b"TPZ"),
	
	/// TODO.
	TQA = Self::letters_to_token(b"TQA"),
	
	/// TODO.
	TQB = Self::letters_to_token(b"TQB"),
	
	/// TODO.
	TQC = Self::letters_to_token(b"TQC"),
	
	/// TODO.
	TQD = Self::letters_to_token(b"TQD"),
	
	/// TODO.
	TQE = Self::letters_to_token(b"TQE"),
	
	/// TODO.
	TQF = Self::letters_to_token(b"TQF"),
	
	/// TODO.
	TQG = Self::letters_to_token(b"TQG"),
	
	/// TODO.
	TQH = Self::letters_to_token(b"TQH"),
	
	/// TODO.
	TQI = Self::letters_to_token(b"TQI"),
	
	/// TODO.
	TQJ = Self::letters_to_token(b"TQJ"),
	
	/// TODO.
	TQK = Self::letters_to_token(b"TQK"),
	
	/// TODO.
	TQL = Self::letters_to_token(b"TQL"),
	
	/// TODO.
	TQM = Self::letters_to_token(b"TQM"),
	
	/// TODO.
	TQN = Self::letters_to_token(b"TQN"),
	
	/// TODO.
	TQO = Self::letters_to_token(b"TQO"),
	
	/// TODO.
	TQP = Self::letters_to_token(b"TQP"),
	
	/// TODO.
	TQQ = Self::letters_to_token(b"TQQ"),
	
	/// TODO.
	TQR = Self::letters_to_token(b"TQR"),
	
	/// TODO.
	TQS = Self::letters_to_token(b"TQS"),
	
	/// TODO.
	TQT = Self::letters_to_token(b"TQT"),
	
	/// TODO.
	TQU = Self::letters_to_token(b"TQU"),
	
	/// TODO.
	TQV = Self::letters_to_token(b"TQV"),
	
	/// TODO.
	TQW = Self::letters_to_token(b"TQW"),
	
	/// TODO.
	TQX = Self::letters_to_token(b"TQX"),
	
	/// TODO.
	TQY = Self::letters_to_token(b"TQY"),
	
	/// TODO.
	TQZ = Self::letters_to_token(b"TQZ"),
	
	/// TODO.
	TRA = Self::letters_to_token(b"TRA"),
	
	/// TODO.
	TRB = Self::letters_to_token(b"TRB"),
	
	/// TODO.
	TRC = Self::letters_to_token(b"TRC"),
	
	/// TODO.
	TRD = Self::letters_to_token(b"TRD"),
	
	/// TODO.
	TRE = Self::letters_to_token(b"TRE"),
	
	/// TODO.
	TRF = Self::letters_to_token(b"TRF"),
	
	/// TODO.
	TRG = Self::letters_to_token(b"TRG"),
	
	/// TODO.
	TRH = Self::letters_to_token(b"TRH"),
	
	/// TODO.
	TRI = Self::letters_to_token(b"TRI"),
	
	/// TODO.
	TRJ = Self::letters_to_token(b"TRJ"),
	
	/// TODO.
	TRK = Self::letters_to_token(b"TRK"),
	
	/// TODO.
	TRL = Self::letters_to_token(b"TRL"),
	
	/// TODO.
	TRM = Self::letters_to_token(b"TRM"),
	
	/// TODO.
	TRN = Self::letters_to_token(b"TRN"),
	
	/// TODO.
	TRO = Self::letters_to_token(b"TRO"),
	
	/// TODO.
	TRP = Self::letters_to_token(b"TRP"),
	
	/// TODO.
	TRQ = Self::letters_to_token(b"TRQ"),
	
	/// TODO.
	TRR = Self::letters_to_token(b"TRR"),
	
	/// TODO.
	TRS = Self::letters_to_token(b"TRS"),
	
	/// TODO.
	TRT = Self::letters_to_token(b"TRT"),
	
	/// TODO.
	TRU = Self::letters_to_token(b"TRU"),
	
	/// TODO.
	TRV = Self::letters_to_token(b"TRV"),
	
	/// TODO.
	TRW = Self::letters_to_token(b"TRW"),
	
	/// TODO.
	TRX = Self::letters_to_token(b"TRX"),
	
	/// TODO.
	TRY = Self::letters_to_token(b"TRY"),
	
	/// TODO.
	TRZ = Self::letters_to_token(b"TRZ"),
	
	/// TODO.
	TSA = Self::letters_to_token(b"TSA"),
	
	/// TODO.
	TSB = Self::letters_to_token(b"TSB"),
	
	/// TODO.
	TSC = Self::letters_to_token(b"TSC"),
	
	/// TODO.
	TSD = Self::letters_to_token(b"TSD"),
	
	/// TODO.
	TSE = Self::letters_to_token(b"TSE"),
	
	/// TODO.
	TSF = Self::letters_to_token(b"TSF"),
	
	/// TODO.
	TSG = Self::letters_to_token(b"TSG"),
	
	/// TODO.
	TSH = Self::letters_to_token(b"TSH"),
	
	/// TODO.
	TSI = Self::letters_to_token(b"TSI"),
	
	/// TODO.
	TSJ = Self::letters_to_token(b"TSJ"),
	
	/// TODO.
	TSK = Self::letters_to_token(b"TSK"),
	
	/// TODO.
	TSL = Self::letters_to_token(b"TSL"),
	
	/// TODO.
	TSM = Self::letters_to_token(b"TSM"),
	
	/// TODO.
	TSN = Self::letters_to_token(b"TSN"),
	
	/// TODO.
	TSO = Self::letters_to_token(b"TSO"),
	
	/// TODO.
	TSP = Self::letters_to_token(b"TSP"),
	
	/// TODO.
	TSQ = Self::letters_to_token(b"TSQ"),
	
	/// TODO.
	TSR = Self::letters_to_token(b"TSR"),
	
	/// TODO.
	TSS = Self::letters_to_token(b"TSS"),
	
	/// TODO.
	TST = Self::letters_to_token(b"TST"),
	
	/// TODO.
	TSU = Self::letters_to_token(b"TSU"),
	
	/// TODO.
	TSV = Self::letters_to_token(b"TSV"),
	
	/// TODO.
	TSW = Self::letters_to_token(b"TSW"),
	
	/// TODO.
	TSX = Self::letters_to_token(b"TSX"),
	
	/// TODO.
	TSY = Self::letters_to_token(b"TSY"),
	
	/// TODO.
	TSZ = Self::letters_to_token(b"TSZ"),
	
	/// TODO.
	TTA = Self::letters_to_token(b"TTA"),
	
	/// TODO.
	TTB = Self::letters_to_token(b"TTB"),
	
	/// TODO.
	TTC = Self::letters_to_token(b"TTC"),
	
	/// TODO.
	TTD = Self::letters_to_token(b"TTD"),
	
	/// TODO.
	TTE = Self::letters_to_token(b"TTE"),
	
	/// TODO.
	TTF = Self::letters_to_token(b"TTF"),
	
	/// TODO.
	TTG = Self::letters_to_token(b"TTG"),
	
	/// TODO.
	TTH = Self::letters_to_token(b"TTH"),
	
	/// TODO.
	TTI = Self::letters_to_token(b"TTI"),
	
	/// TODO.
	TTJ = Self::letters_to_token(b"TTJ"),
	
	/// TODO.
	TTK = Self::letters_to_token(b"TTK"),
	
	/// TODO.
	TTL = Self::letters_to_token(b"TTL"),
	
	/// TODO.
	TTM = Self::letters_to_token(b"TTM"),
	
	/// TODO.
	TTN = Self::letters_to_token(b"TTN"),
	
	/// TODO.
	TTO = Self::letters_to_token(b"TTO"),
	
	/// TODO.
	TTP = Self::letters_to_token(b"TTP"),
	
	/// TODO.
	TTQ = Self::letters_to_token(b"TTQ"),
	
	/// TODO.
	TTR = Self::letters_to_token(b"TTR"),
	
	/// TODO.
	TTS = Self::letters_to_token(b"TTS"),
	
	/// TODO.
	TTT = Self::letters_to_token(b"TTT"),
	
	/// TODO.
	TTU = Self::letters_to_token(b"TTU"),
	
	/// TODO.
	TTV = Self::letters_to_token(b"TTV"),
	
	/// TODO.
	TTW = Self::letters_to_token(b"TTW"),
	
	/// TODO.
	TTX = Self::letters_to_token(b"TTX"),
	
	/// TODO.
	TTY = Self::letters_to_token(b"TTY"),
	
	/// TODO.
	TTZ = Self::letters_to_token(b"TTZ"),
	
	/// TODO.
	TUA = Self::letters_to_token(b"TUA"),
	
	/// TODO.
	TUB = Self::letters_to_token(b"TUB"),
	
	/// TODO.
	TUC = Self::letters_to_token(b"TUC"),
	
	/// TODO.
	TUD = Self::letters_to_token(b"TUD"),
	
	/// TODO.
	TUE = Self::letters_to_token(b"TUE"),
	
	/// TODO.
	TUF = Self::letters_to_token(b"TUF"),
	
	/// TODO.
	TUG = Self::letters_to_token(b"TUG"),
	
	/// TODO.
	TUH = Self::letters_to_token(b"TUH"),
	
	/// TODO.
	TUI = Self::letters_to_token(b"TUI"),
	
	/// TODO.
	TUJ = Self::letters_to_token(b"TUJ"),
	
	/// TODO.
	TUK = Self::letters_to_token(b"TUK"),
	
	/// TODO.
	TUL = Self::letters_to_token(b"TUL"),
	
	/// TODO.
	TUM = Self::letters_to_token(b"TUM"),
	
	/// TODO.
	TUN = Self::letters_to_token(b"TUN"),
	
	/// TODO.
	TUO = Self::letters_to_token(b"TUO"),
	
	/// TODO.
	TUP = Self::letters_to_token(b"TUP"),
	
	/// TODO.
	TUQ = Self::letters_to_token(b"TUQ"),
	
	/// TODO.
	TUR = Self::letters_to_token(b"TUR"),
	
	/// TODO.
	TUS = Self::letters_to_token(b"TUS"),
	
	/// TODO.
	TUT = Self::letters_to_token(b"TUT"),
	
	/// TODO.
	TUU = Self::letters_to_token(b"TUU"),
	
	/// TODO.
	TUV = Self::letters_to_token(b"TUV"),
	
	/// TODO.
	TUW = Self::letters_to_token(b"TUW"),
	
	/// TODO.
	TUX = Self::letters_to_token(b"TUX"),
	
	/// TODO.
	TUY = Self::letters_to_token(b"TUY"),
	
	/// TODO.
	TUZ = Self::letters_to_token(b"TUZ"),
	
	/// TODO.
	TVA = Self::letters_to_token(b"TVA"),
	
	/// TODO.
	TVB = Self::letters_to_token(b"TVB"),
	
	/// TODO.
	TVC = Self::letters_to_token(b"TVC"),
	
	/// TODO.
	TVD = Self::letters_to_token(b"TVD"),
	
	/// TODO.
	TVE = Self::letters_to_token(b"TVE"),
	
	/// TODO.
	TVF = Self::letters_to_token(b"TVF"),
	
	/// TODO.
	TVG = Self::letters_to_token(b"TVG"),
	
	/// TODO.
	TVH = Self::letters_to_token(b"TVH"),
	
	/// TODO.
	TVI = Self::letters_to_token(b"TVI"),
	
	/// TODO.
	TVJ = Self::letters_to_token(b"TVJ"),
	
	/// TODO.
	TVK = Self::letters_to_token(b"TVK"),
	
	/// TODO.
	TVL = Self::letters_to_token(b"TVL"),
	
	/// TODO.
	TVM = Self::letters_to_token(b"TVM"),
	
	/// TODO.
	TVN = Self::letters_to_token(b"TVN"),
	
	/// TODO.
	TVO = Self::letters_to_token(b"TVO"),
	
	/// TODO.
	TVP = Self::letters_to_token(b"TVP"),
	
	/// TODO.
	TVQ = Self::letters_to_token(b"TVQ"),
	
	/// TODO.
	TVR = Self::letters_to_token(b"TVR"),
	
	/// TODO.
	TVS = Self::letters_to_token(b"TVS"),
	
	/// TODO.
	TVT = Self::letters_to_token(b"TVT"),
	
	/// TODO.
	TVU = Self::letters_to_token(b"TVU"),
	
	/// TODO.
	TVV = Self::letters_to_token(b"TVV"),
	
	/// TODO.
	TVW = Self::letters_to_token(b"TVW"),
	
	/// TODO.
	TVX = Self::letters_to_token(b"TVX"),
	
	/// TODO.
	TVY = Self::letters_to_token(b"TVY"),
	
	/// TODO.
	TVZ = Self::letters_to_token(b"TVZ"),
	
	/// TODO.
	TWA = Self::letters_to_token(b"TWA"),
	
	/// TODO.
	TWB = Self::letters_to_token(b"TWB"),
	
	/// TODO.
	TWC = Self::letters_to_token(b"TWC"),
	
	/// TODO.
	TWD = Self::letters_to_token(b"TWD"),
	
	/// TODO.
	TWE = Self::letters_to_token(b"TWE"),
	
	/// TODO.
	TWF = Self::letters_to_token(b"TWF"),
	
	/// TODO.
	TWG = Self::letters_to_token(b"TWG"),
	
	/// TODO.
	TWH = Self::letters_to_token(b"TWH"),
	
	/// TODO.
	TWI = Self::letters_to_token(b"TWI"),
	
	/// TODO.
	TWJ = Self::letters_to_token(b"TWJ"),
	
	/// TODO.
	TWK = Self::letters_to_token(b"TWK"),
	
	/// TODO.
	TWL = Self::letters_to_token(b"TWL"),
	
	/// TODO.
	TWM = Self::letters_to_token(b"TWM"),
	
	/// TODO.
	TWN = Self::letters_to_token(b"TWN"),
	
	/// TODO.
	TWO = Self::letters_to_token(b"TWO"),
	
	/// TODO.
	TWP = Self::letters_to_token(b"TWP"),
	
	/// TODO.
	TWQ = Self::letters_to_token(b"TWQ"),
	
	/// TODO.
	TWR = Self::letters_to_token(b"TWR"),
	
	/// TODO.
	TWS = Self::letters_to_token(b"TWS"),
	
	/// TODO.
	TWT = Self::letters_to_token(b"TWT"),
	
	/// TODO.
	TWU = Self::letters_to_token(b"TWU"),
	
	/// TODO.
	TWV = Self::letters_to_token(b"TWV"),
	
	/// TODO.
	TWW = Self::letters_to_token(b"TWW"),
	
	/// TODO.
	TWX = Self::letters_to_token(b"TWX"),
	
	/// TODO.
	TWY = Self::letters_to_token(b"TWY"),
	
	/// TODO.
	TWZ = Self::letters_to_token(b"TWZ"),
	
	/// TODO.
	TXA = Self::letters_to_token(b"TXA"),
	
	/// TODO.
	TXB = Self::letters_to_token(b"TXB"),
	
	/// TODO.
	TXC = Self::letters_to_token(b"TXC"),
	
	/// TODO.
	TXD = Self::letters_to_token(b"TXD"),
	
	/// TODO.
	TXE = Self::letters_to_token(b"TXE"),
	
	/// TODO.
	TXF = Self::letters_to_token(b"TXF"),
	
	/// TODO.
	TXG = Self::letters_to_token(b"TXG"),
	
	/// TODO.
	TXH = Self::letters_to_token(b"TXH"),
	
	/// TODO.
	TXI = Self::letters_to_token(b"TXI"),
	
	/// TODO.
	TXJ = Self::letters_to_token(b"TXJ"),
	
	/// TODO.
	TXK = Self::letters_to_token(b"TXK"),
	
	/// TODO.
	TXL = Self::letters_to_token(b"TXL"),
	
	/// TODO.
	TXM = Self::letters_to_token(b"TXM"),
	
	/// TODO.
	TXN = Self::letters_to_token(b"TXN"),
	
	/// TODO.
	TXO = Self::letters_to_token(b"TXO"),
	
	/// TODO.
	TXP = Self::letters_to_token(b"TXP"),
	
	/// TODO.
	TXQ = Self::letters_to_token(b"TXQ"),
	
	/// TODO.
	TXR = Self::letters_to_token(b"TXR"),
	
	/// TODO.
	TXS = Self::letters_to_token(b"TXS"),
	
	/// TODO.
	TXT = Self::letters_to_token(b"TXT"),
	
	/// TODO.
	TXU = Self::letters_to_token(b"TXU"),
	
	/// TODO.
	TXV = Self::letters_to_token(b"TXV"),
	
	/// TODO.
	TXW = Self::letters_to_token(b"TXW"),
	
	/// TODO.
	TXX = Self::letters_to_token(b"TXX"),
	
	/// TODO.
	TXY = Self::letters_to_token(b"TXY"),
	
	/// TODO.
	TXZ = Self::letters_to_token(b"TXZ"),
	
	/// TODO.
	TYA = Self::letters_to_token(b"TYA"),
	
	/// TODO.
	TYB = Self::letters_to_token(b"TYB"),
	
	/// TODO.
	TYC = Self::letters_to_token(b"TYC"),
	
	/// TODO.
	TYD = Self::letters_to_token(b"TYD"),
	
	/// TODO.
	TYE = Self::letters_to_token(b"TYE"),
	
	/// TODO.
	TYF = Self::letters_to_token(b"TYF"),
	
	/// TODO.
	TYG = Self::letters_to_token(b"TYG"),
	
	/// TODO.
	TYH = Self::letters_to_token(b"TYH"),
	
	/// TODO.
	TYI = Self::letters_to_token(b"TYI"),
	
	/// TODO.
	TYJ = Self::letters_to_token(b"TYJ"),
	
	/// TODO.
	TYK = Self::letters_to_token(b"TYK"),
	
	/// TODO.
	TYL = Self::letters_to_token(b"TYL"),
	
	/// TODO.
	TYM = Self::letters_to_token(b"TYM"),
	
	/// TODO.
	TYN = Self::letters_to_token(b"TYN"),
	
	/// TODO.
	TYO = Self::letters_to_token(b"TYO"),
	
	/// TODO.
	TYP = Self::letters_to_token(b"TYP"),
	
	/// TODO.
	TYQ = Self::letters_to_token(b"TYQ"),
	
	/// TODO.
	TYR = Self::letters_to_token(b"TYR"),
	
	/// TODO.
	TYS = Self::letters_to_token(b"TYS"),
	
	/// TODO.
	TYT = Self::letters_to_token(b"TYT"),
	
	/// TODO.
	TYU = Self::letters_to_token(b"TYU"),
	
	/// TODO.
	TYV = Self::letters_to_token(b"TYV"),
	
	/// TODO.
	TYW = Self::letters_to_token(b"TYW"),
	
	/// TODO.
	TYX = Self::letters_to_token(b"TYX"),
	
	/// TODO.
	TYY = Self::letters_to_token(b"TYY"),
	
	/// TODO.
	TYZ = Self::letters_to_token(b"TYZ"),
	
	/// TODO.
	TZA = Self::letters_to_token(b"TZA"),
	
	/// TODO.
	TZB = Self::letters_to_token(b"TZB"),
	
	/// TODO.
	TZC = Self::letters_to_token(b"TZC"),
	
	/// TODO.
	TZD = Self::letters_to_token(b"TZD"),
	
	/// TODO.
	TZE = Self::letters_to_token(b"TZE"),
	
	/// TODO.
	TZF = Self::letters_to_token(b"TZF"),
	
	/// TODO.
	TZG = Self::letters_to_token(b"TZG"),
	
	/// TODO.
	TZH = Self::letters_to_token(b"TZH"),
	
	/// TODO.
	TZI = Self::letters_to_token(b"TZI"),
	
	/// TODO.
	TZJ = Self::letters_to_token(b"TZJ"),
	
	/// TODO.
	TZK = Self::letters_to_token(b"TZK"),
	
	/// TODO.
	TZL = Self::letters_to_token(b"TZL"),
	
	/// TODO.
	TZM = Self::letters_to_token(b"TZM"),
	
	/// TODO.
	TZN = Self::letters_to_token(b"TZN"),
	
	/// TODO.
	TZO = Self::letters_to_token(b"TZO"),
	
	/// TODO.
	TZP = Self::letters_to_token(b"TZP"),
	
	/// TODO.
	TZQ = Self::letters_to_token(b"TZQ"),
	
	/// TODO.
	TZR = Self::letters_to_token(b"TZR"),
	
	/// TODO.
	TZS = Self::letters_to_token(b"TZS"),
	
	/// TODO.
	TZT = Self::letters_to_token(b"TZT"),
	
	/// TODO.
	TZU = Self::letters_to_token(b"TZU"),
	
	/// TODO.
	TZV = Self::letters_to_token(b"TZV"),
	
	/// TODO.
	TZW = Self::letters_to_token(b"TZW"),
	
	/// TODO.
	TZX = Self::letters_to_token(b"TZX"),
	
	/// TODO.
	TZY = Self::letters_to_token(b"TZY"),
	
	/// TODO.
	TZZ = Self::letters_to_token(b"TZZ"),
	
	/// TODO.
	UAA = Self::letters_to_token(b"UAA"),
	
	/// TODO.
	UAB = Self::letters_to_token(b"UAB"),
	
	/// TODO.
	UAC = Self::letters_to_token(b"UAC"),
	
	/// TODO.
	UAD = Self::letters_to_token(b"UAD"),
	
	/// TODO.
	UAE = Self::letters_to_token(b"UAE"),
	
	/// TODO.
	UAF = Self::letters_to_token(b"UAF"),
	
	/// TODO.
	UAG = Self::letters_to_token(b"UAG"),
	
	/// TODO.
	UAH = Self::letters_to_token(b"UAH"),
	
	/// TODO.
	UAI = Self::letters_to_token(b"UAI"),
	
	/// TODO.
	UAJ = Self::letters_to_token(b"UAJ"),
	
	/// TODO.
	UAK = Self::letters_to_token(b"UAK"),
	
	/// TODO.
	UAL = Self::letters_to_token(b"UAL"),
	
	/// TODO.
	UAM = Self::letters_to_token(b"UAM"),
	
	/// TODO.
	UAN = Self::letters_to_token(b"UAN"),
	
	/// TODO.
	UAO = Self::letters_to_token(b"UAO"),
	
	/// TODO.
	UAP = Self::letters_to_token(b"UAP"),
	
	/// TODO.
	UAQ = Self::letters_to_token(b"UAQ"),
	
	/// TODO.
	UAR = Self::letters_to_token(b"UAR"),
	
	/// TODO.
	UAS = Self::letters_to_token(b"UAS"),
	
	/// TODO.
	UAT = Self::letters_to_token(b"UAT"),
	
	/// TODO.
	UAU = Self::letters_to_token(b"UAU"),
	
	/// TODO.
	UAV = Self::letters_to_token(b"UAV"),
	
	/// TODO.
	UAW = Self::letters_to_token(b"UAW"),
	
	/// TODO.
	UAX = Self::letters_to_token(b"UAX"),
	
	/// TODO.
	UAY = Self::letters_to_token(b"UAY"),
	
	/// TODO.
	UAZ = Self::letters_to_token(b"UAZ"),
	
	/// TODO.
	UBA = Self::letters_to_token(b"UBA"),
	
	/// TODO.
	UBB = Self::letters_to_token(b"UBB"),
	
	/// TODO.
	UBC = Self::letters_to_token(b"UBC"),
	
	/// TODO.
	UBD = Self::letters_to_token(b"UBD"),
	
	/// TODO.
	UBE = Self::letters_to_token(b"UBE"),
	
	/// TODO.
	UBF = Self::letters_to_token(b"UBF"),
	
	/// TODO.
	UBG = Self::letters_to_token(b"UBG"),
	
	/// TODO.
	UBH = Self::letters_to_token(b"UBH"),
	
	/// TODO.
	UBI = Self::letters_to_token(b"UBI"),
	
	/// TODO.
	UBJ = Self::letters_to_token(b"UBJ"),
	
	/// TODO.
	UBK = Self::letters_to_token(b"UBK"),
	
	/// TODO.
	UBL = Self::letters_to_token(b"UBL"),
	
	/// TODO.
	UBM = Self::letters_to_token(b"UBM"),
	
	/// TODO.
	UBN = Self::letters_to_token(b"UBN"),
	
	/// TODO.
	UBO = Self::letters_to_token(b"UBO"),
	
	/// TODO.
	UBP = Self::letters_to_token(b"UBP"),
	
	/// TODO.
	UBQ = Self::letters_to_token(b"UBQ"),
	
	/// TODO.
	UBR = Self::letters_to_token(b"UBR"),
	
	/// TODO.
	UBS = Self::letters_to_token(b"UBS"),
	
	/// TODO.
	UBT = Self::letters_to_token(b"UBT"),
	
	/// TODO.
	UBU = Self::letters_to_token(b"UBU"),
	
	/// TODO.
	UBV = Self::letters_to_token(b"UBV"),
	
	/// TODO.
	UBW = Self::letters_to_token(b"UBW"),
	
	/// TODO.
	UBX = Self::letters_to_token(b"UBX"),
	
	/// TODO.
	UBY = Self::letters_to_token(b"UBY"),
	
	/// TODO.
	UBZ = Self::letters_to_token(b"UBZ"),
	
	/// TODO.
	UCA = Self::letters_to_token(b"UCA"),
	
	/// TODO.
	UCB = Self::letters_to_token(b"UCB"),
	
	/// TODO.
	UCC = Self::letters_to_token(b"UCC"),
	
	/// TODO.
	UCD = Self::letters_to_token(b"UCD"),
	
	/// TODO.
	UCE = Self::letters_to_token(b"UCE"),
	
	/// TODO.
	UCF = Self::letters_to_token(b"UCF"),
	
	/// TODO.
	UCG = Self::letters_to_token(b"UCG"),
	
	/// TODO.
	UCH = Self::letters_to_token(b"UCH"),
	
	/// TODO.
	UCI = Self::letters_to_token(b"UCI"),
	
	/// TODO.
	UCJ = Self::letters_to_token(b"UCJ"),
	
	/// TODO.
	UCK = Self::letters_to_token(b"UCK"),
	
	/// TODO.
	UCL = Self::letters_to_token(b"UCL"),
	
	/// TODO.
	UCM = Self::letters_to_token(b"UCM"),
	
	/// TODO.
	UCN = Self::letters_to_token(b"UCN"),
	
	/// TODO.
	UCO = Self::letters_to_token(b"UCO"),
	
	/// TODO.
	UCP = Self::letters_to_token(b"UCP"),
	
	/// TODO.
	UCQ = Self::letters_to_token(b"UCQ"),
	
	/// TODO.
	UCR = Self::letters_to_token(b"UCR"),
	
	/// TODO.
	UCS = Self::letters_to_token(b"UCS"),
	
	/// TODO.
	UCT = Self::letters_to_token(b"UCT"),
	
	/// TODO.
	UCU = Self::letters_to_token(b"UCU"),
	
	/// TODO.
	UCV = Self::letters_to_token(b"UCV"),
	
	/// TODO.
	UCW = Self::letters_to_token(b"UCW"),
	
	/// TODO.
	UCX = Self::letters_to_token(b"UCX"),
	
	/// TODO.
	UCY = Self::letters_to_token(b"UCY"),
	
	/// TODO.
	UCZ = Self::letters_to_token(b"UCZ"),
	
	/// TODO.
	UDA = Self::letters_to_token(b"UDA"),
	
	/// TODO.
	UDB = Self::letters_to_token(b"UDB"),
	
	/// TODO.
	UDC = Self::letters_to_token(b"UDC"),
	
	/// TODO.
	UDD = Self::letters_to_token(b"UDD"),
	
	/// TODO.
	UDE = Self::letters_to_token(b"UDE"),
	
	/// TODO.
	UDF = Self::letters_to_token(b"UDF"),
	
	/// TODO.
	UDG = Self::letters_to_token(b"UDG"),
	
	/// TODO.
	UDH = Self::letters_to_token(b"UDH"),
	
	/// TODO.
	UDI = Self::letters_to_token(b"UDI"),
	
	/// TODO.
	UDJ = Self::letters_to_token(b"UDJ"),
	
	/// TODO.
	UDK = Self::letters_to_token(b"UDK"),
	
	/// TODO.
	UDL = Self::letters_to_token(b"UDL"),
	
	/// TODO.
	UDM = Self::letters_to_token(b"UDM"),
	
	/// TODO.
	UDN = Self::letters_to_token(b"UDN"),
	
	/// TODO.
	UDO = Self::letters_to_token(b"UDO"),
	
	/// TODO.
	UDP = Self::letters_to_token(b"UDP"),
	
	/// TODO.
	UDQ = Self::letters_to_token(b"UDQ"),
	
	/// TODO.
	UDR = Self::letters_to_token(b"UDR"),
	
	/// TODO.
	UDS = Self::letters_to_token(b"UDS"),
	
	/// TODO.
	UDT = Self::letters_to_token(b"UDT"),
	
	/// TODO.
	UDU = Self::letters_to_token(b"UDU"),
	
	/// TODO.
	UDV = Self::letters_to_token(b"UDV"),
	
	/// TODO.
	UDW = Self::letters_to_token(b"UDW"),
	
	/// TODO.
	UDX = Self::letters_to_token(b"UDX"),
	
	/// TODO.
	UDY = Self::letters_to_token(b"UDY"),
	
	/// TODO.
	UDZ = Self::letters_to_token(b"UDZ"),
	
	/// TODO.
	UEA = Self::letters_to_token(b"UEA"),
	
	/// TODO.
	UEB = Self::letters_to_token(b"UEB"),
	
	/// TODO.
	UEC = Self::letters_to_token(b"UEC"),
	
	/// TODO.
	UED = Self::letters_to_token(b"UED"),
	
	/// TODO.
	UEE = Self::letters_to_token(b"UEE"),
	
	/// TODO.
	UEF = Self::letters_to_token(b"UEF"),
	
	/// TODO.
	UEG = Self::letters_to_token(b"UEG"),
	
	/// TODO.
	UEH = Self::letters_to_token(b"UEH"),
	
	/// TODO.
	UEI = Self::letters_to_token(b"UEI"),
	
	/// TODO.
	UEJ = Self::letters_to_token(b"UEJ"),
	
	/// TODO.
	UEK = Self::letters_to_token(b"UEK"),
	
	/// TODO.
	UEL = Self::letters_to_token(b"UEL"),
	
	/// TODO.
	UEM = Self::letters_to_token(b"UEM"),
	
	/// TODO.
	UEN = Self::letters_to_token(b"UEN"),
	
	/// TODO.
	UEO = Self::letters_to_token(b"UEO"),
	
	/// TODO.
	UEP = Self::letters_to_token(b"UEP"),
	
	/// TODO.
	UEQ = Self::letters_to_token(b"UEQ"),
	
	/// TODO.
	UER = Self::letters_to_token(b"UER"),
	
	/// TODO.
	UES = Self::letters_to_token(b"UES"),
	
	/// TODO.
	UET = Self::letters_to_token(b"UET"),
	
	/// TODO.
	UEU = Self::letters_to_token(b"UEU"),
	
	/// TODO.
	UEV = Self::letters_to_token(b"UEV"),
	
	/// TODO.
	UEW = Self::letters_to_token(b"UEW"),
	
	/// TODO.
	UEX = Self::letters_to_token(b"UEX"),
	
	/// TODO.
	UEY = Self::letters_to_token(b"UEY"),
	
	/// TODO.
	UEZ = Self::letters_to_token(b"UEZ"),
	
	/// TODO.
	UFA = Self::letters_to_token(b"UFA"),
	
	/// TODO.
	UFB = Self::letters_to_token(b"UFB"),
	
	/// TODO.
	UFC = Self::letters_to_token(b"UFC"),
	
	/// TODO.
	UFD = Self::letters_to_token(b"UFD"),
	
	/// TODO.
	UFE = Self::letters_to_token(b"UFE"),
	
	/// TODO.
	UFF = Self::letters_to_token(b"UFF"),
	
	/// TODO.
	UFG = Self::letters_to_token(b"UFG"),
	
	/// TODO.
	UFH = Self::letters_to_token(b"UFH"),
	
	/// TODO.
	UFI = Self::letters_to_token(b"UFI"),
	
	/// TODO.
	UFJ = Self::letters_to_token(b"UFJ"),
	
	/// TODO.
	UFK = Self::letters_to_token(b"UFK"),
	
	/// TODO.
	UFL = Self::letters_to_token(b"UFL"),
	
	/// TODO.
	UFM = Self::letters_to_token(b"UFM"),
	
	/// TODO.
	UFN = Self::letters_to_token(b"UFN"),
	
	/// TODO.
	UFO = Self::letters_to_token(b"UFO"),
	
	/// TODO.
	UFP = Self::letters_to_token(b"UFP"),
	
	/// TODO.
	UFQ = Self::letters_to_token(b"UFQ"),
	
	/// TODO.
	UFR = Self::letters_to_token(b"UFR"),
	
	/// TODO.
	UFS = Self::letters_to_token(b"UFS"),
	
	/// TODO.
	UFT = Self::letters_to_token(b"UFT"),
	
	/// TODO.
	UFU = Self::letters_to_token(b"UFU"),
	
	/// TODO.
	UFV = Self::letters_to_token(b"UFV"),
	
	/// TODO.
	UFW = Self::letters_to_token(b"UFW"),
	
	/// TODO.
	UFX = Self::letters_to_token(b"UFX"),
	
	/// TODO.
	UFY = Self::letters_to_token(b"UFY"),
	
	/// TODO.
	UFZ = Self::letters_to_token(b"UFZ"),
	
	/// TODO.
	UGA = Self::letters_to_token(b"UGA"),
	
	/// TODO.
	UGB = Self::letters_to_token(b"UGB"),
	
	/// TODO.
	UGC = Self::letters_to_token(b"UGC"),
	
	/// TODO.
	UGD = Self::letters_to_token(b"UGD"),
	
	/// TODO.
	UGE = Self::letters_to_token(b"UGE"),
	
	/// TODO.
	UGF = Self::letters_to_token(b"UGF"),
	
	/// TODO.
	UGG = Self::letters_to_token(b"UGG"),
	
	/// TODO.
	UGH = Self::letters_to_token(b"UGH"),
	
	/// TODO.
	UGI = Self::letters_to_token(b"UGI"),
	
	/// TODO.
	UGJ = Self::letters_to_token(b"UGJ"),
	
	/// TODO.
	UGK = Self::letters_to_token(b"UGK"),
	
	/// TODO.
	UGL = Self::letters_to_token(b"UGL"),
	
	/// TODO.
	UGM = Self::letters_to_token(b"UGM"),
	
	/// TODO.
	UGN = Self::letters_to_token(b"UGN"),
	
	/// TODO.
	UGO = Self::letters_to_token(b"UGO"),
	
	/// TODO.
	UGP = Self::letters_to_token(b"UGP"),
	
	/// TODO.
	UGQ = Self::letters_to_token(b"UGQ"),
	
	/// TODO.
	UGR = Self::letters_to_token(b"UGR"),
	
	/// TODO.
	UGS = Self::letters_to_token(b"UGS"),
	
	/// TODO.
	UGT = Self::letters_to_token(b"UGT"),
	
	/// TODO.
	UGU = Self::letters_to_token(b"UGU"),
	
	/// TODO.
	UGV = Self::letters_to_token(b"UGV"),
	
	/// TODO.
	UGW = Self::letters_to_token(b"UGW"),
	
	/// TODO.
	UGX = Self::letters_to_token(b"UGX"),
	
	/// TODO.
	UGY = Self::letters_to_token(b"UGY"),
	
	/// TODO.
	UGZ = Self::letters_to_token(b"UGZ"),
	
	/// TODO.
	UHA = Self::letters_to_token(b"UHA"),
	
	/// TODO.
	UHB = Self::letters_to_token(b"UHB"),
	
	/// TODO.
	UHC = Self::letters_to_token(b"UHC"),
	
	/// TODO.
	UHD = Self::letters_to_token(b"UHD"),
	
	/// TODO.
	UHE = Self::letters_to_token(b"UHE"),
	
	/// TODO.
	UHF = Self::letters_to_token(b"UHF"),
	
	/// TODO.
	UHG = Self::letters_to_token(b"UHG"),
	
	/// TODO.
	UHH = Self::letters_to_token(b"UHH"),
	
	/// TODO.
	UHI = Self::letters_to_token(b"UHI"),
	
	/// TODO.
	UHJ = Self::letters_to_token(b"UHJ"),
	
	/// TODO.
	UHK = Self::letters_to_token(b"UHK"),
	
	/// TODO.
	UHL = Self::letters_to_token(b"UHL"),
	
	/// TODO.
	UHM = Self::letters_to_token(b"UHM"),
	
	/// TODO.
	UHN = Self::letters_to_token(b"UHN"),
	
	/// TODO.
	UHO = Self::letters_to_token(b"UHO"),
	
	/// TODO.
	UHP = Self::letters_to_token(b"UHP"),
	
	/// TODO.
	UHQ = Self::letters_to_token(b"UHQ"),
	
	/// TODO.
	UHR = Self::letters_to_token(b"UHR"),
	
	/// TODO.
	UHS = Self::letters_to_token(b"UHS"),
	
	/// TODO.
	UHT = Self::letters_to_token(b"UHT"),
	
	/// TODO.
	UHU = Self::letters_to_token(b"UHU"),
	
	/// TODO.
	UHV = Self::letters_to_token(b"UHV"),
	
	/// TODO.
	UHW = Self::letters_to_token(b"UHW"),
	
	/// TODO.
	UHX = Self::letters_to_token(b"UHX"),
	
	/// TODO.
	UHY = Self::letters_to_token(b"UHY"),
	
	/// TODO.
	UHZ = Self::letters_to_token(b"UHZ"),
	
	/// TODO.
	UIA = Self::letters_to_token(b"UIA"),
	
	/// TODO.
	UIB = Self::letters_to_token(b"UIB"),
	
	/// TODO.
	UIC = Self::letters_to_token(b"UIC"),
	
	/// TODO.
	UID = Self::letters_to_token(b"UID"),
	
	/// TODO.
	UIE = Self::letters_to_token(b"UIE"),
	
	/// TODO.
	UIF = Self::letters_to_token(b"UIF"),
	
	/// TODO.
	UIG = Self::letters_to_token(b"UIG"),
	
	/// TODO.
	UIH = Self::letters_to_token(b"UIH"),
	
	/// TODO.
	UII = Self::letters_to_token(b"UII"),
	
	/// TODO.
	UIJ = Self::letters_to_token(b"UIJ"),
	
	/// TODO.
	UIK = Self::letters_to_token(b"UIK"),
	
	/// TODO.
	UIL = Self::letters_to_token(b"UIL"),
	
	/// TODO.
	UIM = Self::letters_to_token(b"UIM"),
	
	/// TODO.
	UIN = Self::letters_to_token(b"UIN"),
	
	/// TODO.
	UIO = Self::letters_to_token(b"UIO"),
	
	/// TODO.
	UIP = Self::letters_to_token(b"UIP"),
	
	/// TODO.
	UIQ = Self::letters_to_token(b"UIQ"),
	
	/// TODO.
	UIR = Self::letters_to_token(b"UIR"),
	
	/// TODO.
	UIS = Self::letters_to_token(b"UIS"),
	
	/// TODO.
	UIT = Self::letters_to_token(b"UIT"),
	
	/// TODO.
	UIU = Self::letters_to_token(b"UIU"),
	
	/// TODO.
	UIV = Self::letters_to_token(b"UIV"),
	
	/// TODO.
	UIW = Self::letters_to_token(b"UIW"),
	
	/// TODO.
	UIX = Self::letters_to_token(b"UIX"),
	
	/// TODO.
	UIY = Self::letters_to_token(b"UIY"),
	
	/// TODO.
	UIZ = Self::letters_to_token(b"UIZ"),
	
	/// TODO.
	UJA = Self::letters_to_token(b"UJA"),
	
	/// TODO.
	UJB = Self::letters_to_token(b"UJB"),
	
	/// TODO.
	UJC = Self::letters_to_token(b"UJC"),
	
	/// TODO.
	UJD = Self::letters_to_token(b"UJD"),
	
	/// TODO.
	UJE = Self::letters_to_token(b"UJE"),
	
	/// TODO.
	UJF = Self::letters_to_token(b"UJF"),
	
	/// TODO.
	UJG = Self::letters_to_token(b"UJG"),
	
	/// TODO.
	UJH = Self::letters_to_token(b"UJH"),
	
	/// TODO.
	UJI = Self::letters_to_token(b"UJI"),
	
	/// TODO.
	UJJ = Self::letters_to_token(b"UJJ"),
	
	/// TODO.
	UJK = Self::letters_to_token(b"UJK"),
	
	/// TODO.
	UJL = Self::letters_to_token(b"UJL"),
	
	/// TODO.
	UJM = Self::letters_to_token(b"UJM"),
	
	/// TODO.
	UJN = Self::letters_to_token(b"UJN"),
	
	/// TODO.
	UJO = Self::letters_to_token(b"UJO"),
	
	/// TODO.
	UJP = Self::letters_to_token(b"UJP"),
	
	/// TODO.
	UJQ = Self::letters_to_token(b"UJQ"),
	
	/// TODO.
	UJR = Self::letters_to_token(b"UJR"),
	
	/// TODO.
	UJS = Self::letters_to_token(b"UJS"),
	
	/// TODO.
	UJT = Self::letters_to_token(b"UJT"),
	
	/// TODO.
	UJU = Self::letters_to_token(b"UJU"),
	
	/// TODO.
	UJV = Self::letters_to_token(b"UJV"),
	
	/// TODO.
	UJW = Self::letters_to_token(b"UJW"),
	
	/// TODO.
	UJX = Self::letters_to_token(b"UJX"),
	
	/// TODO.
	UJY = Self::letters_to_token(b"UJY"),
	
	/// TODO.
	UJZ = Self::letters_to_token(b"UJZ"),
	
	/// TODO.
	UKA = Self::letters_to_token(b"UKA"),
	
	/// TODO.
	UKB = Self::letters_to_token(b"UKB"),
	
	/// TODO.
	UKC = Self::letters_to_token(b"UKC"),
	
	/// TODO.
	UKD = Self::letters_to_token(b"UKD"),
	
	/// TODO.
	UKE = Self::letters_to_token(b"UKE"),
	
	/// TODO.
	UKF = Self::letters_to_token(b"UKF"),
	
	/// TODO.
	UKG = Self::letters_to_token(b"UKG"),
	
	/// TODO.
	UKH = Self::letters_to_token(b"UKH"),
	
	/// TODO.
	UKI = Self::letters_to_token(b"UKI"),
	
	/// TODO.
	UKJ = Self::letters_to_token(b"UKJ"),
	
	/// TODO.
	UKK = Self::letters_to_token(b"UKK"),
	
	/// TODO.
	UKL = Self::letters_to_token(b"UKL"),
	
	/// TODO.
	UKM = Self::letters_to_token(b"UKM"),
	
	/// TODO.
	UKN = Self::letters_to_token(b"UKN"),
	
	/// TODO.
	UKO = Self::letters_to_token(b"UKO"),
	
	/// TODO.
	UKP = Self::letters_to_token(b"UKP"),
	
	/// TODO.
	UKQ = Self::letters_to_token(b"UKQ"),
	
	/// TODO.
	UKR = Self::letters_to_token(b"UKR"),
	
	/// TODO.
	UKS = Self::letters_to_token(b"UKS"),
	
	/// TODO.
	UKT = Self::letters_to_token(b"UKT"),
	
	/// TODO.
	UKU = Self::letters_to_token(b"UKU"),
	
	/// TODO.
	UKV = Self::letters_to_token(b"UKV"),
	
	/// TODO.
	UKW = Self::letters_to_token(b"UKW"),
	
	/// TODO.
	UKX = Self::letters_to_token(b"UKX"),
	
	/// TODO.
	UKY = Self::letters_to_token(b"UKY"),
	
	/// TODO.
	UKZ = Self::letters_to_token(b"UKZ"),
	
	/// TODO.
	ULA = Self::letters_to_token(b"ULA"),
	
	/// TODO.
	ULB = Self::letters_to_token(b"ULB"),
	
	/// TODO.
	ULC = Self::letters_to_token(b"ULC"),
	
	/// TODO.
	ULD = Self::letters_to_token(b"ULD"),
	
	/// TODO.
	ULE = Self::letters_to_token(b"ULE"),
	
	/// TODO.
	ULF = Self::letters_to_token(b"ULF"),
	
	/// TODO.
	ULG = Self::letters_to_token(b"ULG"),
	
	/// TODO.
	ULH = Self::letters_to_token(b"ULH"),
	
	/// TODO.
	ULI = Self::letters_to_token(b"ULI"),
	
	/// TODO.
	ULJ = Self::letters_to_token(b"ULJ"),
	
	/// TODO.
	ULK = Self::letters_to_token(b"ULK"),
	
	/// TODO.
	ULL = Self::letters_to_token(b"ULL"),
	
	/// TODO.
	ULM = Self::letters_to_token(b"ULM"),
	
	/// TODO.
	ULN = Self::letters_to_token(b"ULN"),
	
	/// TODO.
	ULO = Self::letters_to_token(b"ULO"),
	
	/// TODO.
	ULP = Self::letters_to_token(b"ULP"),
	
	/// TODO.
	ULQ = Self::letters_to_token(b"ULQ"),
	
	/// TODO.
	ULR = Self::letters_to_token(b"ULR"),
	
	/// TODO.
	ULS = Self::letters_to_token(b"ULS"),
	
	/// TODO.
	ULT = Self::letters_to_token(b"ULT"),
	
	/// TODO.
	ULU = Self::letters_to_token(b"ULU"),
	
	/// TODO.
	ULV = Self::letters_to_token(b"ULV"),
	
	/// TODO.
	ULW = Self::letters_to_token(b"ULW"),
	
	/// TODO.
	ULX = Self::letters_to_token(b"ULX"),
	
	/// TODO.
	ULY = Self::letters_to_token(b"ULY"),
	
	/// TODO.
	ULZ = Self::letters_to_token(b"ULZ"),
	
	/// TODO.
	UMA = Self::letters_to_token(b"UMA"),
	
	/// TODO.
	UMB = Self::letters_to_token(b"UMB"),
	
	/// TODO.
	UMC = Self::letters_to_token(b"UMC"),
	
	/// TODO.
	UMD = Self::letters_to_token(b"UMD"),
	
	/// TODO.
	UME = Self::letters_to_token(b"UME"),
	
	/// TODO.
	UMF = Self::letters_to_token(b"UMF"),
	
	/// TODO.
	UMG = Self::letters_to_token(b"UMG"),
	
	/// TODO.
	UMH = Self::letters_to_token(b"UMH"),
	
	/// TODO.
	UMI = Self::letters_to_token(b"UMI"),
	
	/// TODO.
	UMJ = Self::letters_to_token(b"UMJ"),
	
	/// TODO.
	UMK = Self::letters_to_token(b"UMK"),
	
	/// TODO.
	UML = Self::letters_to_token(b"UML"),
	
	/// TODO.
	UMM = Self::letters_to_token(b"UMM"),
	
	/// TODO.
	UMN = Self::letters_to_token(b"UMN"),
	
	/// TODO.
	UMO = Self::letters_to_token(b"UMO"),
	
	/// TODO.
	UMP = Self::letters_to_token(b"UMP"),
	
	/// TODO.
	UMQ = Self::letters_to_token(b"UMQ"),
	
	/// TODO.
	UMR = Self::letters_to_token(b"UMR"),
	
	/// TODO.
	UMS = Self::letters_to_token(b"UMS"),
	
	/// TODO.
	UMT = Self::letters_to_token(b"UMT"),
	
	/// TODO.
	UMU = Self::letters_to_token(b"UMU"),
	
	/// TODO.
	UMV = Self::letters_to_token(b"UMV"),
	
	/// TODO.
	UMW = Self::letters_to_token(b"UMW"),
	
	/// TODO.
	UMX = Self::letters_to_token(b"UMX"),
	
	/// TODO.
	UMY = Self::letters_to_token(b"UMY"),
	
	/// TODO.
	UMZ = Self::letters_to_token(b"UMZ"),
	
	/// TODO.
	UNA = Self::letters_to_token(b"UNA"),
	
	/// TODO.
	UNB = Self::letters_to_token(b"UNB"),
	
	/// TODO.
	UNC = Self::letters_to_token(b"UNC"),
	
	/// TODO.
	UND = Self::letters_to_token(b"UND"),
	
	/// TODO.
	UNE = Self::letters_to_token(b"UNE"),
	
	/// TODO.
	UNF = Self::letters_to_token(b"UNF"),
	
	/// TODO.
	UNG = Self::letters_to_token(b"UNG"),
	
	/// TODO.
	UNH = Self::letters_to_token(b"UNH"),
	
	/// TODO.
	UNI = Self::letters_to_token(b"UNI"),
	
	/// TODO.
	UNJ = Self::letters_to_token(b"UNJ"),
	
	/// TODO.
	UNK = Self::letters_to_token(b"UNK"),
	
	/// TODO.
	UNL = Self::letters_to_token(b"UNL"),
	
	/// TODO.
	UNM = Self::letters_to_token(b"UNM"),
	
	/// TODO.
	UNN = Self::letters_to_token(b"UNN"),
	
	/// TODO.
	UNO = Self::letters_to_token(b"UNO"),
	
	/// TODO.
	UNP = Self::letters_to_token(b"UNP"),
	
	/// TODO.
	UNQ = Self::letters_to_token(b"UNQ"),
	
	/// TODO.
	UNR = Self::letters_to_token(b"UNR"),
	
	/// TODO.
	UNS = Self::letters_to_token(b"UNS"),
	
	/// TODO.
	UNT = Self::letters_to_token(b"UNT"),
	
	/// TODO.
	UNU = Self::letters_to_token(b"UNU"),
	
	/// TODO.
	UNV = Self::letters_to_token(b"UNV"),
	
	/// TODO.
	UNW = Self::letters_to_token(b"UNW"),
	
	/// TODO.
	UNX = Self::letters_to_token(b"UNX"),
	
	/// TODO.
	UNY = Self::letters_to_token(b"UNY"),
	
	/// TODO.
	UNZ = Self::letters_to_token(b"UNZ"),
	
	/// TODO.
	UOA = Self::letters_to_token(b"UOA"),
	
	/// TODO.
	UOB = Self::letters_to_token(b"UOB"),
	
	/// TODO.
	UOC = Self::letters_to_token(b"UOC"),
	
	/// TODO.
	UOD = Self::letters_to_token(b"UOD"),
	
	/// TODO.
	UOE = Self::letters_to_token(b"UOE"),
	
	/// TODO.
	UOF = Self::letters_to_token(b"UOF"),
	
	/// TODO.
	UOG = Self::letters_to_token(b"UOG"),
	
	/// TODO.
	UOH = Self::letters_to_token(b"UOH"),
	
	/// TODO.
	UOI = Self::letters_to_token(b"UOI"),
	
	/// TODO.
	UOJ = Self::letters_to_token(b"UOJ"),
	
	/// TODO.
	UOK = Self::letters_to_token(b"UOK"),
	
	/// TODO.
	UOL = Self::letters_to_token(b"UOL"),
	
	/// TODO.
	UOM = Self::letters_to_token(b"UOM"),
	
	/// TODO.
	UON = Self::letters_to_token(b"UON"),
	
	/// TODO.
	UOO = Self::letters_to_token(b"UOO"),
	
	/// TODO.
	UOP = Self::letters_to_token(b"UOP"),
	
	/// TODO.
	UOQ = Self::letters_to_token(b"UOQ"),
	
	/// TODO.
	UOR = Self::letters_to_token(b"UOR"),
	
	/// TODO.
	UOS = Self::letters_to_token(b"UOS"),
	
	/// TODO.
	UOT = Self::letters_to_token(b"UOT"),
	
	/// TODO.
	UOU = Self::letters_to_token(b"UOU"),
	
	/// TODO.
	UOV = Self::letters_to_token(b"UOV"),
	
	/// TODO.
	UOW = Self::letters_to_token(b"UOW"),
	
	/// TODO.
	UOX = Self::letters_to_token(b"UOX"),
	
	/// TODO.
	UOY = Self::letters_to_token(b"UOY"),
	
	/// TODO.
	UOZ = Self::letters_to_token(b"UOZ"),
	
	/// TODO.
	UPA = Self::letters_to_token(b"UPA"),
	
	/// TODO.
	UPB = Self::letters_to_token(b"UPB"),
	
	/// TODO.
	UPC = Self::letters_to_token(b"UPC"),
	
	/// TODO.
	UPD = Self::letters_to_token(b"UPD"),
	
	/// TODO.
	UPE = Self::letters_to_token(b"UPE"),
	
	/// TODO.
	UPF = Self::letters_to_token(b"UPF"),
	
	/// TODO.
	UPG = Self::letters_to_token(b"UPG"),
	
	/// TODO.
	UPH = Self::letters_to_token(b"UPH"),
	
	/// TODO.
	UPI = Self::letters_to_token(b"UPI"),
	
	/// TODO.
	UPJ = Self::letters_to_token(b"UPJ"),
	
	/// TODO.
	UPK = Self::letters_to_token(b"UPK"),
	
	/// TODO.
	UPL = Self::letters_to_token(b"UPL"),
	
	/// TODO.
	UPM = Self::letters_to_token(b"UPM"),
	
	/// TODO.
	UPN = Self::letters_to_token(b"UPN"),
	
	/// TODO.
	UPO = Self::letters_to_token(b"UPO"),
	
	/// TODO.
	UPP = Self::letters_to_token(b"UPP"),
	
	/// TODO.
	UPQ = Self::letters_to_token(b"UPQ"),
	
	/// TODO.
	UPR = Self::letters_to_token(b"UPR"),
	
	/// TODO.
	UPS = Self::letters_to_token(b"UPS"),
	
	/// TODO.
	UPT = Self::letters_to_token(b"UPT"),
	
	/// TODO.
	UPU = Self::letters_to_token(b"UPU"),
	
	/// TODO.
	UPV = Self::letters_to_token(b"UPV"),
	
	/// TODO.
	UPW = Self::letters_to_token(b"UPW"),
	
	/// TODO.
	UPX = Self::letters_to_token(b"UPX"),
	
	/// TODO.
	UPY = Self::letters_to_token(b"UPY"),
	
	/// TODO.
	UPZ = Self::letters_to_token(b"UPZ"),
	
	/// TODO.
	UQA = Self::letters_to_token(b"UQA"),
	
	/// TODO.
	UQB = Self::letters_to_token(b"UQB"),
	
	/// TODO.
	UQC = Self::letters_to_token(b"UQC"),
	
	/// TODO.
	UQD = Self::letters_to_token(b"UQD"),
	
	/// TODO.
	UQE = Self::letters_to_token(b"UQE"),
	
	/// TODO.
	UQF = Self::letters_to_token(b"UQF"),
	
	/// TODO.
	UQG = Self::letters_to_token(b"UQG"),
	
	/// TODO.
	UQH = Self::letters_to_token(b"UQH"),
	
	/// TODO.
	UQI = Self::letters_to_token(b"UQI"),
	
	/// TODO.
	UQJ = Self::letters_to_token(b"UQJ"),
	
	/// TODO.
	UQK = Self::letters_to_token(b"UQK"),
	
	/// TODO.
	UQL = Self::letters_to_token(b"UQL"),
	
	/// TODO.
	UQM = Self::letters_to_token(b"UQM"),
	
	/// TODO.
	UQN = Self::letters_to_token(b"UQN"),
	
	/// TODO.
	UQO = Self::letters_to_token(b"UQO"),
	
	/// TODO.
	UQP = Self::letters_to_token(b"UQP"),
	
	/// TODO.
	UQQ = Self::letters_to_token(b"UQQ"),
	
	/// TODO.
	UQR = Self::letters_to_token(b"UQR"),
	
	/// TODO.
	UQS = Self::letters_to_token(b"UQS"),
	
	/// TODO.
	UQT = Self::letters_to_token(b"UQT"),
	
	/// TODO.
	UQU = Self::letters_to_token(b"UQU"),
	
	/// TODO.
	UQV = Self::letters_to_token(b"UQV"),
	
	/// TODO.
	UQW = Self::letters_to_token(b"UQW"),
	
	/// TODO.
	UQX = Self::letters_to_token(b"UQX"),
	
	/// TODO.
	UQY = Self::letters_to_token(b"UQY"),
	
	/// TODO.
	UQZ = Self::letters_to_token(b"UQZ"),
	
	/// TODO.
	URA = Self::letters_to_token(b"URA"),
	
	/// TODO.
	URB = Self::letters_to_token(b"URB"),
	
	/// TODO.
	URC = Self::letters_to_token(b"URC"),
	
	/// TODO.
	URD = Self::letters_to_token(b"URD"),
	
	/// TODO.
	URE = Self::letters_to_token(b"URE"),
	
	/// TODO.
	URF = Self::letters_to_token(b"URF"),
	
	/// TODO.
	URG = Self::letters_to_token(b"URG"),
	
	/// TODO.
	URH = Self::letters_to_token(b"URH"),
	
	/// TODO.
	URI = Self::letters_to_token(b"URI"),
	
	/// TODO.
	URJ = Self::letters_to_token(b"URJ"),
	
	/// TODO.
	URK = Self::letters_to_token(b"URK"),
	
	/// TODO.
	URL = Self::letters_to_token(b"URL"),
	
	/// TODO.
	URM = Self::letters_to_token(b"URM"),
	
	/// TODO.
	URN = Self::letters_to_token(b"URN"),
	
	/// TODO.
	URO = Self::letters_to_token(b"URO"),
	
	/// TODO.
	URP = Self::letters_to_token(b"URP"),
	
	/// TODO.
	URQ = Self::letters_to_token(b"URQ"),
	
	/// TODO.
	URR = Self::letters_to_token(b"URR"),
	
	/// TODO.
	URS = Self::letters_to_token(b"URS"),
	
	/// TODO.
	URT = Self::letters_to_token(b"URT"),
	
	/// TODO.
	URU = Self::letters_to_token(b"URU"),
	
	/// TODO.
	URV = Self::letters_to_token(b"URV"),
	
	/// TODO.
	URW = Self::letters_to_token(b"URW"),
	
	/// TODO.
	URX = Self::letters_to_token(b"URX"),
	
	/// TODO.
	URY = Self::letters_to_token(b"URY"),
	
	/// TODO.
	URZ = Self::letters_to_token(b"URZ"),
	
	/// TODO.
	USA = Self::letters_to_token(b"USA"),
	
	/// TODO.
	USB = Self::letters_to_token(b"USB"),
	
	/// TODO.
	USC = Self::letters_to_token(b"USC"),
	
	/// TODO.
	USD = Self::letters_to_token(b"USD"),
	
	/// TODO.
	USE = Self::letters_to_token(b"USE"),
	
	/// TODO.
	USF = Self::letters_to_token(b"USF"),
	
	/// TODO.
	USG = Self::letters_to_token(b"USG"),
	
	/// TODO.
	USH = Self::letters_to_token(b"USH"),
	
	/// TODO.
	USI = Self::letters_to_token(b"USI"),
	
	/// TODO.
	USJ = Self::letters_to_token(b"USJ"),
	
	/// TODO.
	USK = Self::letters_to_token(b"USK"),
	
	/// TODO.
	USL = Self::letters_to_token(b"USL"),
	
	/// TODO.
	USM = Self::letters_to_token(b"USM"),
	
	/// TODO.
	USN = Self::letters_to_token(b"USN"),
	
	/// TODO.
	USO = Self::letters_to_token(b"USO"),
	
	/// TODO.
	USP = Self::letters_to_token(b"USP"),
	
	/// TODO.
	USQ = Self::letters_to_token(b"USQ"),
	
	/// TODO.
	USR = Self::letters_to_token(b"USR"),
	
	/// TODO.
	USS = Self::letters_to_token(b"USS"),
	
	/// TODO.
	UST = Self::letters_to_token(b"UST"),
	
	/// TODO.
	USU = Self::letters_to_token(b"USU"),
	
	/// TODO.
	USV = Self::letters_to_token(b"USV"),
	
	/// TODO.
	USW = Self::letters_to_token(b"USW"),
	
	/// TODO.
	USX = Self::letters_to_token(b"USX"),
	
	/// TODO.
	USY = Self::letters_to_token(b"USY"),
	
	/// TODO.
	USZ = Self::letters_to_token(b"USZ"),
	
	/// TODO.
	UTA = Self::letters_to_token(b"UTA"),
	
	/// TODO.
	UTB = Self::letters_to_token(b"UTB"),
	
	/// TODO.
	UTC = Self::letters_to_token(b"UTC"),
	
	/// TODO.
	UTD = Self::letters_to_token(b"UTD"),
	
	/// TODO.
	UTE = Self::letters_to_token(b"UTE"),
	
	/// TODO.
	UTF = Self::letters_to_token(b"UTF"),
	
	/// TODO.
	UTG = Self::letters_to_token(b"UTG"),
	
	/// TODO.
	UTH = Self::letters_to_token(b"UTH"),
	
	/// TODO.
	UTI = Self::letters_to_token(b"UTI"),
	
	/// TODO.
	UTJ = Self::letters_to_token(b"UTJ"),
	
	/// TODO.
	UTK = Self::letters_to_token(b"UTK"),
	
	/// TODO.
	UTL = Self::letters_to_token(b"UTL"),
	
	/// TODO.
	UTM = Self::letters_to_token(b"UTM"),
	
	/// TODO.
	UTN = Self::letters_to_token(b"UTN"),
	
	/// TODO.
	UTO = Self::letters_to_token(b"UTO"),
	
	/// TODO.
	UTP = Self::letters_to_token(b"UTP"),
	
	/// TODO.
	UTQ = Self::letters_to_token(b"UTQ"),
	
	/// TODO.
	UTR = Self::letters_to_token(b"UTR"),
	
	/// TODO.
	UTS = Self::letters_to_token(b"UTS"),
	
	/// TODO.
	UTT = Self::letters_to_token(b"UTT"),
	
	/// TODO.
	UTU = Self::letters_to_token(b"UTU"),
	
	/// TODO.
	UTV = Self::letters_to_token(b"UTV"),
	
	/// TODO.
	UTW = Self::letters_to_token(b"UTW"),
	
	/// TODO.
	UTX = Self::letters_to_token(b"UTX"),
	
	/// TODO.
	UTY = Self::letters_to_token(b"UTY"),
	
	/// TODO.
	UTZ = Self::letters_to_token(b"UTZ"),
	
	/// TODO.
	UUA = Self::letters_to_token(b"UUA"),
	
	/// TODO.
	UUB = Self::letters_to_token(b"UUB"),
	
	/// TODO.
	UUC = Self::letters_to_token(b"UUC"),
	
	/// TODO.
	UUD = Self::letters_to_token(b"UUD"),
	
	/// TODO.
	UUE = Self::letters_to_token(b"UUE"),
	
	/// TODO.
	UUF = Self::letters_to_token(b"UUF"),
	
	/// TODO.
	UUG = Self::letters_to_token(b"UUG"),
	
	/// TODO.
	UUH = Self::letters_to_token(b"UUH"),
	
	/// TODO.
	UUI = Self::letters_to_token(b"UUI"),
	
	/// TODO.
	UUJ = Self::letters_to_token(b"UUJ"),
	
	/// TODO.
	UUK = Self::letters_to_token(b"UUK"),
	
	/// TODO.
	UUL = Self::letters_to_token(b"UUL"),
	
	/// TODO.
	UUM = Self::letters_to_token(b"UUM"),
	
	/// TODO.
	UUN = Self::letters_to_token(b"UUN"),
	
	/// TODO.
	UUO = Self::letters_to_token(b"UUO"),
	
	/// TODO.
	UUP = Self::letters_to_token(b"UUP"),
	
	/// TODO.
	UUQ = Self::letters_to_token(b"UUQ"),
	
	/// TODO.
	UUR = Self::letters_to_token(b"UUR"),
	
	/// TODO.
	UUS = Self::letters_to_token(b"UUS"),
	
	/// TODO.
	UUT = Self::letters_to_token(b"UUT"),
	
	/// Unoffical.
	///
	/// Oceania (NATO STANAG 1059 INT).
	UUU = Self::letters_to_token(b"UUU"),
	
	/// TODO.
	UUV = Self::letters_to_token(b"UUV"),
	
	/// TODO.
	UUW = Self::letters_to_token(b"UUW"),
	
	/// TODO.
	UUX = Self::letters_to_token(b"UUX"),
	
	/// TODO.
	UUY = Self::letters_to_token(b"UUY"),
	
	/// TODO.
	UUZ = Self::letters_to_token(b"UUZ"),
	
	/// TODO.
	UVA = Self::letters_to_token(b"UVA"),
	
	/// TODO.
	UVB = Self::letters_to_token(b"UVB"),
	
	/// TODO.
	UVC = Self::letters_to_token(b"UVC"),
	
	/// TODO.
	UVD = Self::letters_to_token(b"UVD"),
	
	/// TODO.
	UVE = Self::letters_to_token(b"UVE"),
	
	/// TODO.
	UVF = Self::letters_to_token(b"UVF"),
	
	/// TODO.
	UVG = Self::letters_to_token(b"UVG"),
	
	/// TODO.
	UVH = Self::letters_to_token(b"UVH"),
	
	/// TODO.
	UVI = Self::letters_to_token(b"UVI"),
	
	/// TODO.
	UVJ = Self::letters_to_token(b"UVJ"),
	
	/// TODO.
	UVK = Self::letters_to_token(b"UVK"),
	
	/// TODO.
	UVL = Self::letters_to_token(b"UVL"),
	
	/// TODO.
	UVM = Self::letters_to_token(b"UVM"),
	
	/// TODO.
	UVN = Self::letters_to_token(b"UVN"),
	
	/// TODO.
	UVO = Self::letters_to_token(b"UVO"),
	
	/// TODO.
	UVP = Self::letters_to_token(b"UVP"),
	
	/// TODO.
	UVQ = Self::letters_to_token(b"UVQ"),
	
	/// TODO.
	UVR = Self::letters_to_token(b"UVR"),
	
	/// TODO.
	UVS = Self::letters_to_token(b"UVS"),
	
	/// TODO.
	UVT = Self::letters_to_token(b"UVT"),
	
	/// TODO.
	UVU = Self::letters_to_token(b"UVU"),
	
	/// TODO.
	UVV = Self::letters_to_token(b"UVV"),
	
	/// TODO.
	UVW = Self::letters_to_token(b"UVW"),
	
	/// TODO.
	UVX = Self::letters_to_token(b"UVX"),
	
	/// TODO.
	UVY = Self::letters_to_token(b"UVY"),
	
	/// TODO.
	UVZ = Self::letters_to_token(b"UVZ"),
	
	/// TODO.
	UWA = Self::letters_to_token(b"UWA"),
	
	/// TODO.
	UWB = Self::letters_to_token(b"UWB"),
	
	/// TODO.
	UWC = Self::letters_to_token(b"UWC"),
	
	/// TODO.
	UWD = Self::letters_to_token(b"UWD"),
	
	/// TODO.
	UWE = Self::letters_to_token(b"UWE"),
	
	/// TODO.
	UWF = Self::letters_to_token(b"UWF"),
	
	/// TODO.
	UWG = Self::letters_to_token(b"UWG"),
	
	/// TODO.
	UWH = Self::letters_to_token(b"UWH"),
	
	/// TODO.
	UWI = Self::letters_to_token(b"UWI"),
	
	/// TODO.
	UWJ = Self::letters_to_token(b"UWJ"),
	
	/// TODO.
	UWK = Self::letters_to_token(b"UWK"),
	
	/// TODO.
	UWL = Self::letters_to_token(b"UWL"),
	
	/// TODO.
	UWM = Self::letters_to_token(b"UWM"),
	
	/// TODO.
	UWN = Self::letters_to_token(b"UWN"),
	
	/// TODO.
	UWO = Self::letters_to_token(b"UWO"),
	
	/// TODO.
	UWP = Self::letters_to_token(b"UWP"),
	
	/// TODO.
	UWQ = Self::letters_to_token(b"UWQ"),
	
	/// TODO.
	UWR = Self::letters_to_token(b"UWR"),
	
	/// TODO.
	UWS = Self::letters_to_token(b"UWS"),
	
	/// TODO.
	UWT = Self::letters_to_token(b"UWT"),
	
	/// TODO.
	UWU = Self::letters_to_token(b"UWU"),
	
	/// TODO.
	UWV = Self::letters_to_token(b"UWV"),
	
	/// TODO.
	UWW = Self::letters_to_token(b"UWW"),
	
	/// TODO.
	UWX = Self::letters_to_token(b"UWX"),
	
	/// TODO.
	UWY = Self::letters_to_token(b"UWY"),
	
	/// TODO.
	UWZ = Self::letters_to_token(b"UWZ"),
	
	/// TODO.
	UXA = Self::letters_to_token(b"UXA"),
	
	/// TODO.
	UXB = Self::letters_to_token(b"UXB"),
	
	/// TODO.
	UXC = Self::letters_to_token(b"UXC"),
	
	/// TODO.
	UXD = Self::letters_to_token(b"UXD"),
	
	/// TODO.
	UXE = Self::letters_to_token(b"UXE"),
	
	/// TODO.
	UXF = Self::letters_to_token(b"UXF"),
	
	/// TODO.
	UXG = Self::letters_to_token(b"UXG"),
	
	/// TODO.
	UXH = Self::letters_to_token(b"UXH"),
	
	/// TODO.
	UXI = Self::letters_to_token(b"UXI"),
	
	/// TODO.
	UXJ = Self::letters_to_token(b"UXJ"),
	
	/// TODO.
	UXK = Self::letters_to_token(b"UXK"),
	
	/// TODO.
	UXL = Self::letters_to_token(b"UXL"),
	
	/// TODO.
	UXM = Self::letters_to_token(b"UXM"),
	
	/// TODO.
	UXN = Self::letters_to_token(b"UXN"),
	
	/// TODO.
	UXO = Self::letters_to_token(b"UXO"),
	
	/// TODO.
	UXP = Self::letters_to_token(b"UXP"),
	
	/// TODO.
	UXQ = Self::letters_to_token(b"UXQ"),
	
	/// TODO.
	UXR = Self::letters_to_token(b"UXR"),
	
	/// TODO.
	UXS = Self::letters_to_token(b"UXS"),
	
	/// TODO.
	UXT = Self::letters_to_token(b"UXT"),
	
	/// TODO.
	UXU = Self::letters_to_token(b"UXU"),
	
	/// TODO.
	UXV = Self::letters_to_token(b"UXV"),
	
	/// TODO.
	UXW = Self::letters_to_token(b"UXW"),
	
	/// TODO.
	UXX = Self::letters_to_token(b"UXX"),
	
	/// TODO.
	UXY = Self::letters_to_token(b"UXY"),
	
	/// TODO.
	UXZ = Self::letters_to_token(b"UXZ"),
	
	/// TODO.
	UYA = Self::letters_to_token(b"UYA"),
	
	/// TODO.
	UYB = Self::letters_to_token(b"UYB"),
	
	/// TODO.
	UYC = Self::letters_to_token(b"UYC"),
	
	/// TODO.
	UYD = Self::letters_to_token(b"UYD"),
	
	/// TODO.
	UYE = Self::letters_to_token(b"UYE"),
	
	/// TODO.
	UYF = Self::letters_to_token(b"UYF"),
	
	/// TODO.
	UYG = Self::letters_to_token(b"UYG"),
	
	/// TODO.
	UYH = Self::letters_to_token(b"UYH"),
	
	/// TODO.
	UYI = Self::letters_to_token(b"UYI"),
	
	/// TODO.
	UYJ = Self::letters_to_token(b"UYJ"),
	
	/// TODO.
	UYK = Self::letters_to_token(b"UYK"),
	
	/// TODO.
	UYL = Self::letters_to_token(b"UYL"),
	
	/// TODO.
	UYM = Self::letters_to_token(b"UYM"),
	
	/// TODO.
	UYN = Self::letters_to_token(b"UYN"),
	
	/// TODO.
	UYO = Self::letters_to_token(b"UYO"),
	
	/// TODO.
	UYP = Self::letters_to_token(b"UYP"),
	
	/// TODO.
	UYQ = Self::letters_to_token(b"UYQ"),
	
	/// TODO.
	UYR = Self::letters_to_token(b"UYR"),
	
	/// TODO.
	UYS = Self::letters_to_token(b"UYS"),
	
	/// TODO.
	UYT = Self::letters_to_token(b"UYT"),
	
	/// TODO.
	UYU = Self::letters_to_token(b"UYU"),
	
	/// TODO.
	UYV = Self::letters_to_token(b"UYV"),
	
	/// TODO.
	UYW = Self::letters_to_token(b"UYW"),
	
	/// TODO.
	UYX = Self::letters_to_token(b"UYX"),
	
	/// TODO.
	UYY = Self::letters_to_token(b"UYY"),
	
	/// TODO.
	UYZ = Self::letters_to_token(b"UYZ"),
	
	/// TODO.
	UZA = Self::letters_to_token(b"UZA"),
	
	/// TODO.
	UZB = Self::letters_to_token(b"UZB"),
	
	/// TODO.
	UZC = Self::letters_to_token(b"UZC"),
	
	/// TODO.
	UZD = Self::letters_to_token(b"UZD"),
	
	/// TODO.
	UZE = Self::letters_to_token(b"UZE"),
	
	/// TODO.
	UZF = Self::letters_to_token(b"UZF"),
	
	/// TODO.
	UZG = Self::letters_to_token(b"UZG"),
	
	/// TODO.
	UZH = Self::letters_to_token(b"UZH"),
	
	/// TODO.
	UZI = Self::letters_to_token(b"UZI"),
	
	/// TODO.
	UZJ = Self::letters_to_token(b"UZJ"),
	
	/// TODO.
	UZK = Self::letters_to_token(b"UZK"),
	
	/// TODO.
	UZL = Self::letters_to_token(b"UZL"),
	
	/// TODO.
	UZM = Self::letters_to_token(b"UZM"),
	
	/// TODO.
	UZN = Self::letters_to_token(b"UZN"),
	
	/// TODO.
	UZO = Self::letters_to_token(b"UZO"),
	
	/// TODO.
	UZP = Self::letters_to_token(b"UZP"),
	
	/// TODO.
	UZQ = Self::letters_to_token(b"UZQ"),
	
	/// TODO.
	UZR = Self::letters_to_token(b"UZR"),
	
	/// TODO.
	UZS = Self::letters_to_token(b"UZS"),
	
	/// TODO.
	UZT = Self::letters_to_token(b"UZT"),
	
	/// TODO.
	UZU = Self::letters_to_token(b"UZU"),
	
	/// TODO.
	UZV = Self::letters_to_token(b"UZV"),
	
	/// TODO.
	UZW = Self::letters_to_token(b"UZW"),
	
	/// TODO.
	UZX = Self::letters_to_token(b"UZX"),
	
	/// TODO.
	UZY = Self::letters_to_token(b"UZY"),
	
	/// TODO.
	UZZ = Self::letters_to_token(b"UZZ"),
	
	/// TODO.
	VAA = Self::letters_to_token(b"VAA"),
	
	/// TODO.
	VAB = Self::letters_to_token(b"VAB"),
	
	/// TODO.
	VAC = Self::letters_to_token(b"VAC"),
	
	/// TODO.
	VAD = Self::letters_to_token(b"VAD"),
	
	/// TODO.
	VAE = Self::letters_to_token(b"VAE"),
	
	/// TODO.
	VAF = Self::letters_to_token(b"VAF"),
	
	/// TODO.
	VAG = Self::letters_to_token(b"VAG"),
	
	/// TODO.
	VAH = Self::letters_to_token(b"VAH"),
	
	/// TODO.
	VAI = Self::letters_to_token(b"VAI"),
	
	/// TODO.
	VAJ = Self::letters_to_token(b"VAJ"),
	
	/// TODO.
	VAK = Self::letters_to_token(b"VAK"),
	
	/// TODO.
	VAL = Self::letters_to_token(b"VAL"),
	
	/// TODO.
	VAM = Self::letters_to_token(b"VAM"),
	
	/// TODO.
	VAN = Self::letters_to_token(b"VAN"),
	
	/// TODO.
	VAO = Self::letters_to_token(b"VAO"),
	
	/// TODO.
	VAP = Self::letters_to_token(b"VAP"),
	
	/// TODO.
	VAQ = Self::letters_to_token(b"VAQ"),
	
	/// TODO.
	VAR = Self::letters_to_token(b"VAR"),
	
	/// TODO.
	VAS = Self::letters_to_token(b"VAS"),
	
	/// TODO.
	VAT = Self::letters_to_token(b"VAT"),
	
	/// TODO.
	VAU = Self::letters_to_token(b"VAU"),
	
	/// TODO.
	VAV = Self::letters_to_token(b"VAV"),
	
	/// TODO.
	VAW = Self::letters_to_token(b"VAW"),
	
	/// TODO.
	VAX = Self::letters_to_token(b"VAX"),
	
	/// TODO.
	VAY = Self::letters_to_token(b"VAY"),
	
	/// TODO.
	VAZ = Self::letters_to_token(b"VAZ"),
	
	/// TODO.
	VBA = Self::letters_to_token(b"VBA"),
	
	/// TODO.
	VBB = Self::letters_to_token(b"VBB"),
	
	/// TODO.
	VBC = Self::letters_to_token(b"VBC"),
	
	/// TODO.
	VBD = Self::letters_to_token(b"VBD"),
	
	/// TODO.
	VBE = Self::letters_to_token(b"VBE"),
	
	/// TODO.
	VBF = Self::letters_to_token(b"VBF"),
	
	/// TODO.
	VBG = Self::letters_to_token(b"VBG"),
	
	/// TODO.
	VBH = Self::letters_to_token(b"VBH"),
	
	/// TODO.
	VBI = Self::letters_to_token(b"VBI"),
	
	/// TODO.
	VBJ = Self::letters_to_token(b"VBJ"),
	
	/// TODO.
	VBK = Self::letters_to_token(b"VBK"),
	
	/// TODO.
	VBL = Self::letters_to_token(b"VBL"),
	
	/// TODO.
	VBM = Self::letters_to_token(b"VBM"),
	
	/// TODO.
	VBN = Self::letters_to_token(b"VBN"),
	
	/// TODO.
	VBO = Self::letters_to_token(b"VBO"),
	
	/// TODO.
	VBP = Self::letters_to_token(b"VBP"),
	
	/// TODO.
	VBQ = Self::letters_to_token(b"VBQ"),
	
	/// TODO.
	VBR = Self::letters_to_token(b"VBR"),
	
	/// TODO.
	VBS = Self::letters_to_token(b"VBS"),
	
	/// TODO.
	VBT = Self::letters_to_token(b"VBT"),
	
	/// TODO.
	VBU = Self::letters_to_token(b"VBU"),
	
	/// TODO.
	VBV = Self::letters_to_token(b"VBV"),
	
	/// TODO.
	VBW = Self::letters_to_token(b"VBW"),
	
	/// TODO.
	VBX = Self::letters_to_token(b"VBX"),
	
	/// TODO.
	VBY = Self::letters_to_token(b"VBY"),
	
	/// TODO.
	VBZ = Self::letters_to_token(b"VBZ"),
	
	/// TODO.
	VCA = Self::letters_to_token(b"VCA"),
	
	/// TODO.
	VCB = Self::letters_to_token(b"VCB"),
	
	/// TODO.
	VCC = Self::letters_to_token(b"VCC"),
	
	/// TODO.
	VCD = Self::letters_to_token(b"VCD"),
	
	/// TODO.
	VCE = Self::letters_to_token(b"VCE"),
	
	/// TODO.
	VCF = Self::letters_to_token(b"VCF"),
	
	/// TODO.
	VCG = Self::letters_to_token(b"VCG"),
	
	/// TODO.
	VCH = Self::letters_to_token(b"VCH"),
	
	/// TODO.
	VCI = Self::letters_to_token(b"VCI"),
	
	/// TODO.
	VCJ = Self::letters_to_token(b"VCJ"),
	
	/// TODO.
	VCK = Self::letters_to_token(b"VCK"),
	
	/// TODO.
	VCL = Self::letters_to_token(b"VCL"),
	
	/// TODO.
	VCM = Self::letters_to_token(b"VCM"),
	
	/// TODO.
	VCN = Self::letters_to_token(b"VCN"),
	
	/// TODO.
	VCO = Self::letters_to_token(b"VCO"),
	
	/// TODO.
	VCP = Self::letters_to_token(b"VCP"),
	
	/// TODO.
	VCQ = Self::letters_to_token(b"VCQ"),
	
	/// TODO.
	VCR = Self::letters_to_token(b"VCR"),
	
	/// TODO.
	VCS = Self::letters_to_token(b"VCS"),
	
	/// TODO.
	VCT = Self::letters_to_token(b"VCT"),
	
	/// TODO.
	VCU = Self::letters_to_token(b"VCU"),
	
	/// TODO.
	VCV = Self::letters_to_token(b"VCV"),
	
	/// TODO.
	VCW = Self::letters_to_token(b"VCW"),
	
	/// TODO.
	VCX = Self::letters_to_token(b"VCX"),
	
	/// TODO.
	VCY = Self::letters_to_token(b"VCY"),
	
	/// TODO.
	VCZ = Self::letters_to_token(b"VCZ"),
	
	/// TODO.
	VDA = Self::letters_to_token(b"VDA"),
	
	/// TODO.
	VDB = Self::letters_to_token(b"VDB"),
	
	/// TODO.
	VDC = Self::letters_to_token(b"VDC"),
	
	/// TODO.
	VDD = Self::letters_to_token(b"VDD"),
	
	/// TODO.
	VDE = Self::letters_to_token(b"VDE"),
	
	/// TODO.
	VDF = Self::letters_to_token(b"VDF"),
	
	/// TODO.
	VDG = Self::letters_to_token(b"VDG"),
	
	/// TODO.
	VDH = Self::letters_to_token(b"VDH"),
	
	/// TODO.
	VDI = Self::letters_to_token(b"VDI"),
	
	/// TODO.
	VDJ = Self::letters_to_token(b"VDJ"),
	
	/// TODO.
	VDK = Self::letters_to_token(b"VDK"),
	
	/// TODO.
	VDL = Self::letters_to_token(b"VDL"),
	
	/// TODO.
	VDM = Self::letters_to_token(b"VDM"),
	
	/// TODO.
	VDN = Self::letters_to_token(b"VDN"),
	
	/// TODO.
	VDO = Self::letters_to_token(b"VDO"),
	
	/// TODO.
	VDP = Self::letters_to_token(b"VDP"),
	
	/// TODO.
	VDQ = Self::letters_to_token(b"VDQ"),
	
	/// TODO.
	VDR = Self::letters_to_token(b"VDR"),
	
	/// TODO.
	VDS = Self::letters_to_token(b"VDS"),
	
	/// TODO.
	VDT = Self::letters_to_token(b"VDT"),
	
	/// TODO.
	VDU = Self::letters_to_token(b"VDU"),
	
	/// TODO.
	VDV = Self::letters_to_token(b"VDV"),
	
	/// TODO.
	VDW = Self::letters_to_token(b"VDW"),
	
	/// TODO.
	VDX = Self::letters_to_token(b"VDX"),
	
	/// TODO.
	VDY = Self::letters_to_token(b"VDY"),
	
	/// TODO.
	VDZ = Self::letters_to_token(b"VDZ"),
	
	/// TODO.
	VEA = Self::letters_to_token(b"VEA"),
	
	/// TODO.
	VEB = Self::letters_to_token(b"VEB"),
	
	/// TODO.
	VEC = Self::letters_to_token(b"VEC"),
	
	/// TODO.
	VED = Self::letters_to_token(b"VED"),
	
	/// TODO.
	VEE = Self::letters_to_token(b"VEE"),
	
	/// TODO.
	VEF = Self::letters_to_token(b"VEF"),
	
	/// TODO.
	VEG = Self::letters_to_token(b"VEG"),
	
	/// TODO.
	VEH = Self::letters_to_token(b"VEH"),
	
	/// TODO.
	VEI = Self::letters_to_token(b"VEI"),
	
	/// TODO.
	VEJ = Self::letters_to_token(b"VEJ"),
	
	/// TODO.
	VEK = Self::letters_to_token(b"VEK"),
	
	/// TODO.
	VEL = Self::letters_to_token(b"VEL"),
	
	/// TODO.
	VEM = Self::letters_to_token(b"VEM"),
	
	/// TODO.
	VEN = Self::letters_to_token(b"VEN"),
	
	/// TODO.
	VEO = Self::letters_to_token(b"VEO"),
	
	/// TODO.
	VEP = Self::letters_to_token(b"VEP"),
	
	/// TODO.
	VEQ = Self::letters_to_token(b"VEQ"),
	
	/// TODO.
	VER = Self::letters_to_token(b"VER"),
	
	/// TODO.
	VES = Self::letters_to_token(b"VES"),
	
	/// TODO.
	VET = Self::letters_to_token(b"VET"),
	
	/// TODO.
	VEU = Self::letters_to_token(b"VEU"),
	
	/// TODO.
	VEV = Self::letters_to_token(b"VEV"),
	
	/// TODO.
	VEW = Self::letters_to_token(b"VEW"),
	
	/// TODO.
	VEX = Self::letters_to_token(b"VEX"),
	
	/// TODO.
	VEY = Self::letters_to_token(b"VEY"),
	
	/// TODO.
	VEZ = Self::letters_to_token(b"VEZ"),
	
	/// TODO.
	VFA = Self::letters_to_token(b"VFA"),
	
	/// TODO.
	VFB = Self::letters_to_token(b"VFB"),
	
	/// TODO.
	VFC = Self::letters_to_token(b"VFC"),
	
	/// TODO.
	VFD = Self::letters_to_token(b"VFD"),
	
	/// TODO.
	VFE = Self::letters_to_token(b"VFE"),
	
	/// TODO.
	VFF = Self::letters_to_token(b"VFF"),
	
	/// TODO.
	VFG = Self::letters_to_token(b"VFG"),
	
	/// TODO.
	VFH = Self::letters_to_token(b"VFH"),
	
	/// TODO.
	VFI = Self::letters_to_token(b"VFI"),
	
	/// TODO.
	VFJ = Self::letters_to_token(b"VFJ"),
	
	/// TODO.
	VFK = Self::letters_to_token(b"VFK"),
	
	/// TODO.
	VFL = Self::letters_to_token(b"VFL"),
	
	/// TODO.
	VFM = Self::letters_to_token(b"VFM"),
	
	/// TODO.
	VFN = Self::letters_to_token(b"VFN"),
	
	/// TODO.
	VFO = Self::letters_to_token(b"VFO"),
	
	/// TODO.
	VFP = Self::letters_to_token(b"VFP"),
	
	/// TODO.
	VFQ = Self::letters_to_token(b"VFQ"),
	
	/// TODO.
	VFR = Self::letters_to_token(b"VFR"),
	
	/// TODO.
	VFS = Self::letters_to_token(b"VFS"),
	
	/// TODO.
	VFT = Self::letters_to_token(b"VFT"),
	
	/// TODO.
	VFU = Self::letters_to_token(b"VFU"),
	
	/// TODO.
	VFV = Self::letters_to_token(b"VFV"),
	
	/// TODO.
	VFW = Self::letters_to_token(b"VFW"),
	
	/// TODO.
	VFX = Self::letters_to_token(b"VFX"),
	
	/// TODO.
	VFY = Self::letters_to_token(b"VFY"),
	
	/// TODO.
	VFZ = Self::letters_to_token(b"VFZ"),
	
	/// TODO.
	VGA = Self::letters_to_token(b"VGA"),
	
	/// TODO.
	VGB = Self::letters_to_token(b"VGB"),
	
	/// TODO.
	VGC = Self::letters_to_token(b"VGC"),
	
	/// TODO.
	VGD = Self::letters_to_token(b"VGD"),
	
	/// TODO.
	VGE = Self::letters_to_token(b"VGE"),
	
	/// TODO.
	VGF = Self::letters_to_token(b"VGF"),
	
	/// TODO.
	VGG = Self::letters_to_token(b"VGG"),
	
	/// TODO.
	VGH = Self::letters_to_token(b"VGH"),
	
	/// TODO.
	VGI = Self::letters_to_token(b"VGI"),
	
	/// TODO.
	VGJ = Self::letters_to_token(b"VGJ"),
	
	/// TODO.
	VGK = Self::letters_to_token(b"VGK"),
	
	/// TODO.
	VGL = Self::letters_to_token(b"VGL"),
	
	/// TODO.
	VGM = Self::letters_to_token(b"VGM"),
	
	/// TODO.
	VGN = Self::letters_to_token(b"VGN"),
	
	/// TODO.
	VGO = Self::letters_to_token(b"VGO"),
	
	/// TODO.
	VGP = Self::letters_to_token(b"VGP"),
	
	/// TODO.
	VGQ = Self::letters_to_token(b"VGQ"),
	
	/// TODO.
	VGR = Self::letters_to_token(b"VGR"),
	
	/// TODO.
	VGS = Self::letters_to_token(b"VGS"),
	
	/// TODO.
	VGT = Self::letters_to_token(b"VGT"),
	
	/// TODO.
	VGU = Self::letters_to_token(b"VGU"),
	
	/// TODO.
	VGV = Self::letters_to_token(b"VGV"),
	
	/// TODO.
	VGW = Self::letters_to_token(b"VGW"),
	
	/// TODO.
	VGX = Self::letters_to_token(b"VGX"),
	
	/// TODO.
	VGY = Self::letters_to_token(b"VGY"),
	
	/// TODO.
	VGZ = Self::letters_to_token(b"VGZ"),
	
	/// TODO.
	VHA = Self::letters_to_token(b"VHA"),
	
	/// TODO.
	VHB = Self::letters_to_token(b"VHB"),
	
	/// TODO.
	VHC = Self::letters_to_token(b"VHC"),
	
	/// TODO.
	VHD = Self::letters_to_token(b"VHD"),
	
	/// TODO.
	VHE = Self::letters_to_token(b"VHE"),
	
	/// TODO.
	VHF = Self::letters_to_token(b"VHF"),
	
	/// TODO.
	VHG = Self::letters_to_token(b"VHG"),
	
	/// TODO.
	VHH = Self::letters_to_token(b"VHH"),
	
	/// TODO.
	VHI = Self::letters_to_token(b"VHI"),
	
	/// TODO.
	VHJ = Self::letters_to_token(b"VHJ"),
	
	/// TODO.
	VHK = Self::letters_to_token(b"VHK"),
	
	/// TODO.
	VHL = Self::letters_to_token(b"VHL"),
	
	/// TODO.
	VHM = Self::letters_to_token(b"VHM"),
	
	/// TODO.
	VHN = Self::letters_to_token(b"VHN"),
	
	/// TODO.
	VHO = Self::letters_to_token(b"VHO"),
	
	/// TODO.
	VHP = Self::letters_to_token(b"VHP"),
	
	/// TODO.
	VHQ = Self::letters_to_token(b"VHQ"),
	
	/// TODO.
	VHR = Self::letters_to_token(b"VHR"),
	
	/// TODO.
	VHS = Self::letters_to_token(b"VHS"),
	
	/// TODO.
	VHT = Self::letters_to_token(b"VHT"),
	
	/// TODO.
	VHU = Self::letters_to_token(b"VHU"),
	
	/// TODO.
	VHV = Self::letters_to_token(b"VHV"),
	
	/// TODO.
	VHW = Self::letters_to_token(b"VHW"),
	
	/// TODO.
	VHX = Self::letters_to_token(b"VHX"),
	
	/// TODO.
	VHY = Self::letters_to_token(b"VHY"),
	
	/// TODO.
	VHZ = Self::letters_to_token(b"VHZ"),
	
	/// TODO.
	VIA = Self::letters_to_token(b"VIA"),
	
	/// TODO.
	VIB = Self::letters_to_token(b"VIB"),
	
	/// TODO.
	VIC = Self::letters_to_token(b"VIC"),
	
	/// TODO.
	VID = Self::letters_to_token(b"VID"),
	
	/// TODO.
	VIE = Self::letters_to_token(b"VIE"),
	
	/// TODO.
	VIF = Self::letters_to_token(b"VIF"),
	
	/// TODO.
	VIG = Self::letters_to_token(b"VIG"),
	
	/// TODO.
	VIH = Self::letters_to_token(b"VIH"),
	
	/// TODO.
	VII = Self::letters_to_token(b"VII"),
	
	/// TODO.
	VIJ = Self::letters_to_token(b"VIJ"),
	
	/// TODO.
	VIK = Self::letters_to_token(b"VIK"),
	
	/// TODO.
	VIL = Self::letters_to_token(b"VIL"),
	
	/// TODO.
	VIM = Self::letters_to_token(b"VIM"),
	
	/// TODO.
	VIN = Self::letters_to_token(b"VIN"),
	
	/// TODO.
	VIO = Self::letters_to_token(b"VIO"),
	
	/// TODO.
	VIP = Self::letters_to_token(b"VIP"),
	
	/// TODO.
	VIQ = Self::letters_to_token(b"VIQ"),
	
	/// TODO.
	VIR = Self::letters_to_token(b"VIR"),
	
	/// TODO.
	VIS = Self::letters_to_token(b"VIS"),
	
	/// TODO.
	VIT = Self::letters_to_token(b"VIT"),
	
	/// TODO.
	VIU = Self::letters_to_token(b"VIU"),
	
	/// TODO.
	VIV = Self::letters_to_token(b"VIV"),
	
	/// TODO.
	VIW = Self::letters_to_token(b"VIW"),
	
	/// TODO.
	VIX = Self::letters_to_token(b"VIX"),
	
	/// TODO.
	VIY = Self::letters_to_token(b"VIY"),
	
	/// TODO.
	VIZ = Self::letters_to_token(b"VIZ"),
	
	/// TODO.
	VJA = Self::letters_to_token(b"VJA"),
	
	/// TODO.
	VJB = Self::letters_to_token(b"VJB"),
	
	/// TODO.
	VJC = Self::letters_to_token(b"VJC"),
	
	/// TODO.
	VJD = Self::letters_to_token(b"VJD"),
	
	/// TODO.
	VJE = Self::letters_to_token(b"VJE"),
	
	/// TODO.
	VJF = Self::letters_to_token(b"VJF"),
	
	/// TODO.
	VJG = Self::letters_to_token(b"VJG"),
	
	/// TODO.
	VJH = Self::letters_to_token(b"VJH"),
	
	/// TODO.
	VJI = Self::letters_to_token(b"VJI"),
	
	/// TODO.
	VJJ = Self::letters_to_token(b"VJJ"),
	
	/// TODO.
	VJK = Self::letters_to_token(b"VJK"),
	
	/// TODO.
	VJL = Self::letters_to_token(b"VJL"),
	
	/// TODO.
	VJM = Self::letters_to_token(b"VJM"),
	
	/// TODO.
	VJN = Self::letters_to_token(b"VJN"),
	
	/// TODO.
	VJO = Self::letters_to_token(b"VJO"),
	
	/// TODO.
	VJP = Self::letters_to_token(b"VJP"),
	
	/// TODO.
	VJQ = Self::letters_to_token(b"VJQ"),
	
	/// TODO.
	VJR = Self::letters_to_token(b"VJR"),
	
	/// TODO.
	VJS = Self::letters_to_token(b"VJS"),
	
	/// TODO.
	VJT = Self::letters_to_token(b"VJT"),
	
	/// TODO.
	VJU = Self::letters_to_token(b"VJU"),
	
	/// TODO.
	VJV = Self::letters_to_token(b"VJV"),
	
	/// TODO.
	VJW = Self::letters_to_token(b"VJW"),
	
	/// TODO.
	VJX = Self::letters_to_token(b"VJX"),
	
	/// TODO.
	VJY = Self::letters_to_token(b"VJY"),
	
	/// TODO.
	VJZ = Self::letters_to_token(b"VJZ"),
	
	/// TODO.
	VKA = Self::letters_to_token(b"VKA"),
	
	/// TODO.
	VKB = Self::letters_to_token(b"VKB"),
	
	/// TODO.
	VKC = Self::letters_to_token(b"VKC"),
	
	/// TODO.
	VKD = Self::letters_to_token(b"VKD"),
	
	/// TODO.
	VKE = Self::letters_to_token(b"VKE"),
	
	/// TODO.
	VKF = Self::letters_to_token(b"VKF"),
	
	/// TODO.
	VKG = Self::letters_to_token(b"VKG"),
	
	/// TODO.
	VKH = Self::letters_to_token(b"VKH"),
	
	/// TODO.
	VKI = Self::letters_to_token(b"VKI"),
	
	/// TODO.
	VKJ = Self::letters_to_token(b"VKJ"),
	
	/// TODO.
	VKK = Self::letters_to_token(b"VKK"),
	
	/// TODO.
	VKL = Self::letters_to_token(b"VKL"),
	
	/// TODO.
	VKM = Self::letters_to_token(b"VKM"),
	
	/// TODO.
	VKN = Self::letters_to_token(b"VKN"),
	
	/// TODO.
	VKO = Self::letters_to_token(b"VKO"),
	
	/// TODO.
	VKP = Self::letters_to_token(b"VKP"),
	
	/// TODO.
	VKQ = Self::letters_to_token(b"VKQ"),
	
	/// TODO.
	VKR = Self::letters_to_token(b"VKR"),
	
	/// TODO.
	VKS = Self::letters_to_token(b"VKS"),
	
	/// TODO.
	VKT = Self::letters_to_token(b"VKT"),
	
	/// TODO.
	VKU = Self::letters_to_token(b"VKU"),
	
	/// TODO.
	VKV = Self::letters_to_token(b"VKV"),
	
	/// TODO.
	VKW = Self::letters_to_token(b"VKW"),
	
	/// TODO.
	VKX = Self::letters_to_token(b"VKX"),
	
	/// TODO.
	VKY = Self::letters_to_token(b"VKY"),
	
	/// TODO.
	VKZ = Self::letters_to_token(b"VKZ"),
	
	/// TODO.
	VLA = Self::letters_to_token(b"VLA"),
	
	/// TODO.
	VLB = Self::letters_to_token(b"VLB"),
	
	/// TODO.
	VLC = Self::letters_to_token(b"VLC"),
	
	/// TODO.
	VLD = Self::letters_to_token(b"VLD"),
	
	/// TODO.
	VLE = Self::letters_to_token(b"VLE"),
	
	/// TODO.
	VLF = Self::letters_to_token(b"VLF"),
	
	/// TODO.
	VLG = Self::letters_to_token(b"VLG"),
	
	/// TODO.
	VLH = Self::letters_to_token(b"VLH"),
	
	/// TODO.
	VLI = Self::letters_to_token(b"VLI"),
	
	/// TODO.
	VLJ = Self::letters_to_token(b"VLJ"),
	
	/// TODO.
	VLK = Self::letters_to_token(b"VLK"),
	
	/// TODO.
	VLL = Self::letters_to_token(b"VLL"),
	
	/// TODO.
	VLM = Self::letters_to_token(b"VLM"),
	
	/// TODO.
	VLN = Self::letters_to_token(b"VLN"),
	
	/// TODO.
	VLO = Self::letters_to_token(b"VLO"),
	
	/// TODO.
	VLP = Self::letters_to_token(b"VLP"),
	
	/// TODO.
	VLQ = Self::letters_to_token(b"VLQ"),
	
	/// TODO.
	VLR = Self::letters_to_token(b"VLR"),
	
	/// TODO.
	VLS = Self::letters_to_token(b"VLS"),
	
	/// TODO.
	VLT = Self::letters_to_token(b"VLT"),
	
	/// TODO.
	VLU = Self::letters_to_token(b"VLU"),
	
	/// TODO.
	VLV = Self::letters_to_token(b"VLV"),
	
	/// TODO.
	VLW = Self::letters_to_token(b"VLW"),
	
	/// TODO.
	VLX = Self::letters_to_token(b"VLX"),
	
	/// TODO.
	VLY = Self::letters_to_token(b"VLY"),
	
	/// TODO.
	VLZ = Self::letters_to_token(b"VLZ"),
	
	/// TODO.
	VMA = Self::letters_to_token(b"VMA"),
	
	/// TODO.
	VMB = Self::letters_to_token(b"VMB"),
	
	/// TODO.
	VMC = Self::letters_to_token(b"VMC"),
	
	/// TODO.
	VMD = Self::letters_to_token(b"VMD"),
	
	/// TODO.
	VME = Self::letters_to_token(b"VME"),
	
	/// TODO.
	VMF = Self::letters_to_token(b"VMF"),
	
	/// TODO.
	VMG = Self::letters_to_token(b"VMG"),
	
	/// TODO.
	VMH = Self::letters_to_token(b"VMH"),
	
	/// TODO.
	VMI = Self::letters_to_token(b"VMI"),
	
	/// TODO.
	VMJ = Self::letters_to_token(b"VMJ"),
	
	/// TODO.
	VMK = Self::letters_to_token(b"VMK"),
	
	/// TODO.
	VML = Self::letters_to_token(b"VML"),
	
	/// TODO.
	VMM = Self::letters_to_token(b"VMM"),
	
	/// TODO.
	VMN = Self::letters_to_token(b"VMN"),
	
	/// TODO.
	VMO = Self::letters_to_token(b"VMO"),
	
	/// TODO.
	VMP = Self::letters_to_token(b"VMP"),
	
	/// TODO.
	VMQ = Self::letters_to_token(b"VMQ"),
	
	/// TODO.
	VMR = Self::letters_to_token(b"VMR"),
	
	/// TODO.
	VMS = Self::letters_to_token(b"VMS"),
	
	/// TODO.
	VMT = Self::letters_to_token(b"VMT"),
	
	/// TODO.
	VMU = Self::letters_to_token(b"VMU"),
	
	/// TODO.
	VMV = Self::letters_to_token(b"VMV"),
	
	/// TODO.
	VMW = Self::letters_to_token(b"VMW"),
	
	/// TODO.
	VMX = Self::letters_to_token(b"VMX"),
	
	/// TODO.
	VMY = Self::letters_to_token(b"VMY"),
	
	/// TODO.
	VMZ = Self::letters_to_token(b"VMZ"),
	
	/// TODO.
	VNA = Self::letters_to_token(b"VNA"),
	
	/// TODO.
	VNB = Self::letters_to_token(b"VNB"),
	
	/// TODO.
	VNC = Self::letters_to_token(b"VNC"),
	
	/// TODO.
	VND = Self::letters_to_token(b"VND"),
	
	/// TODO.
	VNE = Self::letters_to_token(b"VNE"),
	
	/// TODO.
	VNF = Self::letters_to_token(b"VNF"),
	
	/// TODO.
	VNG = Self::letters_to_token(b"VNG"),
	
	/// TODO.
	VNH = Self::letters_to_token(b"VNH"),
	
	/// TODO.
	VNI = Self::letters_to_token(b"VNI"),
	
	/// TODO.
	VNJ = Self::letters_to_token(b"VNJ"),
	
	/// TODO.
	VNK = Self::letters_to_token(b"VNK"),
	
	/// TODO.
	VNL = Self::letters_to_token(b"VNL"),
	
	/// TODO.
	VNM = Self::letters_to_token(b"VNM"),
	
	/// TODO.
	VNN = Self::letters_to_token(b"VNN"),
	
	/// TODO.
	VNO = Self::letters_to_token(b"VNO"),
	
	/// TODO.
	VNP = Self::letters_to_token(b"VNP"),
	
	/// TODO.
	VNQ = Self::letters_to_token(b"VNQ"),
	
	/// TODO.
	VNR = Self::letters_to_token(b"VNR"),
	
	/// TODO.
	VNS = Self::letters_to_token(b"VNS"),
	
	/// TODO.
	VNT = Self::letters_to_token(b"VNT"),
	
	/// TODO.
	VNU = Self::letters_to_token(b"VNU"),
	
	/// TODO.
	VNV = Self::letters_to_token(b"VNV"),
	
	/// TODO.
	VNW = Self::letters_to_token(b"VNW"),
	
	/// TODO.
	VNX = Self::letters_to_token(b"VNX"),
	
	/// TODO.
	VNY = Self::letters_to_token(b"VNY"),
	
	/// TODO.
	VNZ = Self::letters_to_token(b"VNZ"),
	
	/// TODO.
	VOA = Self::letters_to_token(b"VOA"),
	
	/// TODO.
	VOB = Self::letters_to_token(b"VOB"),
	
	/// TODO.
	VOC = Self::letters_to_token(b"VOC"),
	
	/// TODO.
	VOD = Self::letters_to_token(b"VOD"),
	
	/// TODO.
	VOE = Self::letters_to_token(b"VOE"),
	
	/// TODO.
	VOF = Self::letters_to_token(b"VOF"),
	
	/// TODO.
	VOG = Self::letters_to_token(b"VOG"),
	
	/// TODO.
	VOH = Self::letters_to_token(b"VOH"),
	
	/// TODO.
	VOI = Self::letters_to_token(b"VOI"),
	
	/// TODO.
	VOJ = Self::letters_to_token(b"VOJ"),
	
	/// TODO.
	VOK = Self::letters_to_token(b"VOK"),
	
	/// TODO.
	VOL = Self::letters_to_token(b"VOL"),
	
	/// TODO.
	VOM = Self::letters_to_token(b"VOM"),
	
	/// TODO.
	VON = Self::letters_to_token(b"VON"),
	
	/// TODO.
	VOO = Self::letters_to_token(b"VOO"),
	
	/// TODO.
	VOP = Self::letters_to_token(b"VOP"),
	
	/// TODO.
	VOQ = Self::letters_to_token(b"VOQ"),
	
	/// TODO.
	VOR = Self::letters_to_token(b"VOR"),
	
	/// TODO.
	VOS = Self::letters_to_token(b"VOS"),
	
	/// TODO.
	VOT = Self::letters_to_token(b"VOT"),
	
	/// TODO.
	VOU = Self::letters_to_token(b"VOU"),
	
	/// TODO.
	VOV = Self::letters_to_token(b"VOV"),
	
	/// TODO.
	VOW = Self::letters_to_token(b"VOW"),
	
	/// TODO.
	VOX = Self::letters_to_token(b"VOX"),
	
	/// TODO.
	VOY = Self::letters_to_token(b"VOY"),
	
	/// TODO.
	VOZ = Self::letters_to_token(b"VOZ"),
	
	/// TODO.
	VPA = Self::letters_to_token(b"VPA"),
	
	/// TODO.
	VPB = Self::letters_to_token(b"VPB"),
	
	/// TODO.
	VPC = Self::letters_to_token(b"VPC"),
	
	/// TODO.
	VPD = Self::letters_to_token(b"VPD"),
	
	/// TODO.
	VPE = Self::letters_to_token(b"VPE"),
	
	/// TODO.
	VPF = Self::letters_to_token(b"VPF"),
	
	/// TODO.
	VPG = Self::letters_to_token(b"VPG"),
	
	/// TODO.
	VPH = Self::letters_to_token(b"VPH"),
	
	/// TODO.
	VPI = Self::letters_to_token(b"VPI"),
	
	/// TODO.
	VPJ = Self::letters_to_token(b"VPJ"),
	
	/// TODO.
	VPK = Self::letters_to_token(b"VPK"),
	
	/// TODO.
	VPL = Self::letters_to_token(b"VPL"),
	
	/// TODO.
	VPM = Self::letters_to_token(b"VPM"),
	
	/// TODO.
	VPN = Self::letters_to_token(b"VPN"),
	
	/// TODO.
	VPO = Self::letters_to_token(b"VPO"),
	
	/// TODO.
	VPP = Self::letters_to_token(b"VPP"),
	
	/// TODO.
	VPQ = Self::letters_to_token(b"VPQ"),
	
	/// TODO.
	VPR = Self::letters_to_token(b"VPR"),
	
	/// TODO.
	VPS = Self::letters_to_token(b"VPS"),
	
	/// TODO.
	VPT = Self::letters_to_token(b"VPT"),
	
	/// TODO.
	VPU = Self::letters_to_token(b"VPU"),
	
	/// TODO.
	VPV = Self::letters_to_token(b"VPV"),
	
	/// TODO.
	VPW = Self::letters_to_token(b"VPW"),
	
	/// TODO.
	VPX = Self::letters_to_token(b"VPX"),
	
	/// TODO.
	VPY = Self::letters_to_token(b"VPY"),
	
	/// TODO.
	VPZ = Self::letters_to_token(b"VPZ"),
	
	/// TODO.
	VQA = Self::letters_to_token(b"VQA"),
	
	/// TODO.
	VQB = Self::letters_to_token(b"VQB"),
	
	/// TODO.
	VQC = Self::letters_to_token(b"VQC"),
	
	/// TODO.
	VQD = Self::letters_to_token(b"VQD"),
	
	/// TODO.
	VQE = Self::letters_to_token(b"VQE"),
	
	/// TODO.
	VQF = Self::letters_to_token(b"VQF"),
	
	/// TODO.
	VQG = Self::letters_to_token(b"VQG"),
	
	/// TODO.
	VQH = Self::letters_to_token(b"VQH"),
	
	/// TODO.
	VQI = Self::letters_to_token(b"VQI"),
	
	/// TODO.
	VQJ = Self::letters_to_token(b"VQJ"),
	
	/// TODO.
	VQK = Self::letters_to_token(b"VQK"),
	
	/// TODO.
	VQL = Self::letters_to_token(b"VQL"),
	
	/// TODO.
	VQM = Self::letters_to_token(b"VQM"),
	
	/// TODO.
	VQN = Self::letters_to_token(b"VQN"),
	
	/// TODO.
	VQO = Self::letters_to_token(b"VQO"),
	
	/// TODO.
	VQP = Self::letters_to_token(b"VQP"),
	
	/// TODO.
	VQQ = Self::letters_to_token(b"VQQ"),
	
	/// TODO.
	VQR = Self::letters_to_token(b"VQR"),
	
	/// TODO.
	VQS = Self::letters_to_token(b"VQS"),
	
	/// TODO.
	VQT = Self::letters_to_token(b"VQT"),
	
	/// TODO.
	VQU = Self::letters_to_token(b"VQU"),
	
	/// TODO.
	VQV = Self::letters_to_token(b"VQV"),
	
	/// TODO.
	VQW = Self::letters_to_token(b"VQW"),
	
	/// TODO.
	VQX = Self::letters_to_token(b"VQX"),
	
	/// TODO.
	VQY = Self::letters_to_token(b"VQY"),
	
	/// TODO.
	VQZ = Self::letters_to_token(b"VQZ"),
	
	/// TODO.
	VRA = Self::letters_to_token(b"VRA"),
	
	/// TODO.
	VRB = Self::letters_to_token(b"VRB"),
	
	/// TODO.
	VRC = Self::letters_to_token(b"VRC"),
	
	/// TODO.
	VRD = Self::letters_to_token(b"VRD"),
	
	/// TODO.
	VRE = Self::letters_to_token(b"VRE"),
	
	/// TODO.
	VRF = Self::letters_to_token(b"VRF"),
	
	/// TODO.
	VRG = Self::letters_to_token(b"VRG"),
	
	/// TODO.
	VRH = Self::letters_to_token(b"VRH"),
	
	/// TODO.
	VRI = Self::letters_to_token(b"VRI"),
	
	/// TODO.
	VRJ = Self::letters_to_token(b"VRJ"),
	
	/// TODO.
	VRK = Self::letters_to_token(b"VRK"),
	
	/// TODO.
	VRL = Self::letters_to_token(b"VRL"),
	
	/// TODO.
	VRM = Self::letters_to_token(b"VRM"),
	
	/// TODO.
	VRN = Self::letters_to_token(b"VRN"),
	
	/// TODO.
	VRO = Self::letters_to_token(b"VRO"),
	
	/// TODO.
	VRP = Self::letters_to_token(b"VRP"),
	
	/// TODO.
	VRQ = Self::letters_to_token(b"VRQ"),
	
	/// TODO.
	VRR = Self::letters_to_token(b"VRR"),
	
	/// TODO.
	VRS = Self::letters_to_token(b"VRS"),
	
	/// TODO.
	VRT = Self::letters_to_token(b"VRT"),
	
	/// TODO.
	VRU = Self::letters_to_token(b"VRU"),
	
	/// TODO.
	VRV = Self::letters_to_token(b"VRV"),
	
	/// TODO.
	VRW = Self::letters_to_token(b"VRW"),
	
	/// TODO.
	VRX = Self::letters_to_token(b"VRX"),
	
	/// TODO.
	VRY = Self::letters_to_token(b"VRY"),
	
	/// TODO.
	VRZ = Self::letters_to_token(b"VRZ"),
	
	/// TODO.
	VSA = Self::letters_to_token(b"VSA"),
	
	/// TODO.
	VSB = Self::letters_to_token(b"VSB"),
	
	/// TODO.
	VSC = Self::letters_to_token(b"VSC"),
	
	/// TODO.
	VSD = Self::letters_to_token(b"VSD"),
	
	/// TODO.
	VSE = Self::letters_to_token(b"VSE"),
	
	/// TODO.
	VSF = Self::letters_to_token(b"VSF"),
	
	/// TODO.
	VSG = Self::letters_to_token(b"VSG"),
	
	/// TODO.
	VSH = Self::letters_to_token(b"VSH"),
	
	/// TODO.
	VSI = Self::letters_to_token(b"VSI"),
	
	/// TODO.
	VSJ = Self::letters_to_token(b"VSJ"),
	
	/// TODO.
	VSK = Self::letters_to_token(b"VSK"),
	
	/// TODO.
	VSL = Self::letters_to_token(b"VSL"),
	
	/// TODO.
	VSM = Self::letters_to_token(b"VSM"),
	
	/// TODO.
	VSN = Self::letters_to_token(b"VSN"),
	
	/// TODO.
	VSO = Self::letters_to_token(b"VSO"),
	
	/// TODO.
	VSP = Self::letters_to_token(b"VSP"),
	
	/// TODO.
	VSQ = Self::letters_to_token(b"VSQ"),
	
	/// TODO.
	VSR = Self::letters_to_token(b"VSR"),
	
	/// TODO.
	VSS = Self::letters_to_token(b"VSS"),
	
	/// TODO.
	VST = Self::letters_to_token(b"VST"),
	
	/// TODO.
	VSU = Self::letters_to_token(b"VSU"),
	
	/// TODO.
	VSV = Self::letters_to_token(b"VSV"),
	
	/// TODO.
	VSW = Self::letters_to_token(b"VSW"),
	
	/// TODO.
	VSX = Self::letters_to_token(b"VSX"),
	
	/// TODO.
	VSY = Self::letters_to_token(b"VSY"),
	
	/// TODO.
	VSZ = Self::letters_to_token(b"VSZ"),
	
	/// TODO.
	VTA = Self::letters_to_token(b"VTA"),
	
	/// TODO.
	VTB = Self::letters_to_token(b"VTB"),
	
	/// TODO.
	VTC = Self::letters_to_token(b"VTC"),
	
	/// TODO.
	VTD = Self::letters_to_token(b"VTD"),
	
	/// TODO.
	VTE = Self::letters_to_token(b"VTE"),
	
	/// TODO.
	VTF = Self::letters_to_token(b"VTF"),
	
	/// TODO.
	VTG = Self::letters_to_token(b"VTG"),
	
	/// TODO.
	VTH = Self::letters_to_token(b"VTH"),
	
	/// TODO.
	VTI = Self::letters_to_token(b"VTI"),
	
	/// TODO.
	VTJ = Self::letters_to_token(b"VTJ"),
	
	/// TODO.
	VTK = Self::letters_to_token(b"VTK"),
	
	/// TODO.
	VTL = Self::letters_to_token(b"VTL"),
	
	/// TODO.
	VTM = Self::letters_to_token(b"VTM"),
	
	/// TODO.
	VTN = Self::letters_to_token(b"VTN"),
	
	/// TODO.
	VTO = Self::letters_to_token(b"VTO"),
	
	/// TODO.
	VTP = Self::letters_to_token(b"VTP"),
	
	/// TODO.
	VTQ = Self::letters_to_token(b"VTQ"),
	
	/// TODO.
	VTR = Self::letters_to_token(b"VTR"),
	
	/// TODO.
	VTS = Self::letters_to_token(b"VTS"),
	
	/// TODO.
	VTT = Self::letters_to_token(b"VTT"),
	
	/// TODO.
	VTU = Self::letters_to_token(b"VTU"),
	
	/// TODO.
	VTV = Self::letters_to_token(b"VTV"),
	
	/// TODO.
	VTW = Self::letters_to_token(b"VTW"),
	
	/// TODO.
	VTX = Self::letters_to_token(b"VTX"),
	
	/// TODO.
	VTY = Self::letters_to_token(b"VTY"),
	
	/// TODO.
	VTZ = Self::letters_to_token(b"VTZ"),
	
	/// TODO.
	VUA = Self::letters_to_token(b"VUA"),
	
	/// TODO.
	VUB = Self::letters_to_token(b"VUB"),
	
	/// TODO.
	VUC = Self::letters_to_token(b"VUC"),
	
	/// TODO.
	VUD = Self::letters_to_token(b"VUD"),
	
	/// TODO.
	VUE = Self::letters_to_token(b"VUE"),
	
	/// TODO.
	VUF = Self::letters_to_token(b"VUF"),
	
	/// TODO.
	VUG = Self::letters_to_token(b"VUG"),
	
	/// TODO.
	VUH = Self::letters_to_token(b"VUH"),
	
	/// TODO.
	VUI = Self::letters_to_token(b"VUI"),
	
	/// TODO.
	VUJ = Self::letters_to_token(b"VUJ"),
	
	/// TODO.
	VUK = Self::letters_to_token(b"VUK"),
	
	/// TODO.
	VUL = Self::letters_to_token(b"VUL"),
	
	/// TODO.
	VUM = Self::letters_to_token(b"VUM"),
	
	/// TODO.
	VUN = Self::letters_to_token(b"VUN"),
	
	/// TODO.
	VUO = Self::letters_to_token(b"VUO"),
	
	/// TODO.
	VUP = Self::letters_to_token(b"VUP"),
	
	/// TODO.
	VUQ = Self::letters_to_token(b"VUQ"),
	
	/// TODO.
	VUR = Self::letters_to_token(b"VUR"),
	
	/// TODO.
	VUS = Self::letters_to_token(b"VUS"),
	
	/// TODO.
	VUT = Self::letters_to_token(b"VUT"),
	
	/// TODO.
	VUU = Self::letters_to_token(b"VUU"),
	
	/// TODO.
	VUV = Self::letters_to_token(b"VUV"),
	
	/// TODO.
	VUW = Self::letters_to_token(b"VUW"),
	
	/// TODO.
	VUX = Self::letters_to_token(b"VUX"),
	
	/// TODO.
	VUY = Self::letters_to_token(b"VUY"),
	
	/// TODO.
	VUZ = Self::letters_to_token(b"VUZ"),
	
	/// TODO.
	VVA = Self::letters_to_token(b"VVA"),
	
	/// TODO.
	VVB = Self::letters_to_token(b"VVB"),
	
	/// TODO.
	VVC = Self::letters_to_token(b"VVC"),
	
	/// TODO.
	VVD = Self::letters_to_token(b"VVD"),
	
	/// TODO.
	VVE = Self::letters_to_token(b"VVE"),
	
	/// TODO.
	VVF = Self::letters_to_token(b"VVF"),
	
	/// TODO.
	VVG = Self::letters_to_token(b"VVG"),
	
	/// TODO.
	VVH = Self::letters_to_token(b"VVH"),
	
	/// TODO.
	VVI = Self::letters_to_token(b"VVI"),
	
	/// TODO.
	VVJ = Self::letters_to_token(b"VVJ"),
	
	/// TODO.
	VVK = Self::letters_to_token(b"VVK"),
	
	/// TODO.
	VVL = Self::letters_to_token(b"VVL"),
	
	/// TODO.
	VVM = Self::letters_to_token(b"VVM"),
	
	/// TODO.
	VVN = Self::letters_to_token(b"VVN"),
	
	/// TODO.
	VVO = Self::letters_to_token(b"VVO"),
	
	/// TODO.
	VVP = Self::letters_to_token(b"VVP"),
	
	/// TODO.
	VVQ = Self::letters_to_token(b"VVQ"),
	
	/// TODO.
	VVR = Self::letters_to_token(b"VVR"),
	
	/// TODO.
	VVS = Self::letters_to_token(b"VVS"),
	
	/// TODO.
	VVT = Self::letters_to_token(b"VVT"),
	
	/// TODO.
	VVU = Self::letters_to_token(b"VVU"),
	
	/// TODO.
	VVV = Self::letters_to_token(b"VVV"),
	
	/// TODO.
	VVW = Self::letters_to_token(b"VVW"),
	
	/// TODO.
	VVX = Self::letters_to_token(b"VVX"),
	
	/// TODO.
	VVY = Self::letters_to_token(b"VVY"),
	
	/// TODO.
	VVZ = Self::letters_to_token(b"VVZ"),
	
	/// TODO.
	VWA = Self::letters_to_token(b"VWA"),
	
	/// TODO.
	VWB = Self::letters_to_token(b"VWB"),
	
	/// TODO.
	VWC = Self::letters_to_token(b"VWC"),
	
	/// TODO.
	VWD = Self::letters_to_token(b"VWD"),
	
	/// TODO.
	VWE = Self::letters_to_token(b"VWE"),
	
	/// TODO.
	VWF = Self::letters_to_token(b"VWF"),
	
	/// TODO.
	VWG = Self::letters_to_token(b"VWG"),
	
	/// TODO.
	VWH = Self::letters_to_token(b"VWH"),
	
	/// TODO.
	VWI = Self::letters_to_token(b"VWI"),
	
	/// TODO.
	VWJ = Self::letters_to_token(b"VWJ"),
	
	/// TODO.
	VWK = Self::letters_to_token(b"VWK"),
	
	/// TODO.
	VWL = Self::letters_to_token(b"VWL"),
	
	/// TODO.
	VWM = Self::letters_to_token(b"VWM"),
	
	/// TODO.
	VWN = Self::letters_to_token(b"VWN"),
	
	/// TODO.
	VWO = Self::letters_to_token(b"VWO"),
	
	/// TODO.
	VWP = Self::letters_to_token(b"VWP"),
	
	/// TODO.
	VWQ = Self::letters_to_token(b"VWQ"),
	
	/// TODO.
	VWR = Self::letters_to_token(b"VWR"),
	
	/// TODO.
	VWS = Self::letters_to_token(b"VWS"),
	
	/// TODO.
	VWT = Self::letters_to_token(b"VWT"),
	
	/// TODO.
	VWU = Self::letters_to_token(b"VWU"),
	
	/// TODO.
	VWV = Self::letters_to_token(b"VWV"),
	
	/// TODO.
	VWW = Self::letters_to_token(b"VWW"),
	
	/// TODO.
	VWX = Self::letters_to_token(b"VWX"),
	
	/// TODO.
	VWY = Self::letters_to_token(b"VWY"),
	
	/// TODO.
	VWZ = Self::letters_to_token(b"VWZ"),
	
	/// TODO.
	VXA = Self::letters_to_token(b"VXA"),
	
	/// TODO.
	VXB = Self::letters_to_token(b"VXB"),
	
	/// TODO.
	VXC = Self::letters_to_token(b"VXC"),
	
	/// TODO.
	VXD = Self::letters_to_token(b"VXD"),
	
	/// TODO.
	VXE = Self::letters_to_token(b"VXE"),
	
	/// TODO.
	VXF = Self::letters_to_token(b"VXF"),
	
	/// TODO.
	VXG = Self::letters_to_token(b"VXG"),
	
	/// TODO.
	VXH = Self::letters_to_token(b"VXH"),
	
	/// TODO.
	VXI = Self::letters_to_token(b"VXI"),
	
	/// TODO.
	VXJ = Self::letters_to_token(b"VXJ"),
	
	/// TODO.
	VXK = Self::letters_to_token(b"VXK"),
	
	/// TODO.
	VXL = Self::letters_to_token(b"VXL"),
	
	/// TODO.
	VXM = Self::letters_to_token(b"VXM"),
	
	/// TODO.
	VXN = Self::letters_to_token(b"VXN"),
	
	/// TODO.
	VXO = Self::letters_to_token(b"VXO"),
	
	/// TODO.
	VXP = Self::letters_to_token(b"VXP"),
	
	/// TODO.
	VXQ = Self::letters_to_token(b"VXQ"),
	
	/// TODO.
	VXR = Self::letters_to_token(b"VXR"),
	
	/// TODO.
	VXS = Self::letters_to_token(b"VXS"),
	
	/// TODO.
	VXT = Self::letters_to_token(b"VXT"),
	
	/// TODO.
	VXU = Self::letters_to_token(b"VXU"),
	
	/// TODO.
	VXV = Self::letters_to_token(b"VXV"),
	
	/// TODO.
	VXW = Self::letters_to_token(b"VXW"),
	
	/// TODO.
	VXX = Self::letters_to_token(b"VXX"),
	
	/// TODO.
	VXY = Self::letters_to_token(b"VXY"),
	
	/// TODO.
	VXZ = Self::letters_to_token(b"VXZ"),
	
	/// TODO.
	VYA = Self::letters_to_token(b"VYA"),
	
	/// TODO.
	VYB = Self::letters_to_token(b"VYB"),
	
	/// TODO.
	VYC = Self::letters_to_token(b"VYC"),
	
	/// TODO.
	VYD = Self::letters_to_token(b"VYD"),
	
	/// TODO.
	VYE = Self::letters_to_token(b"VYE"),
	
	/// TODO.
	VYF = Self::letters_to_token(b"VYF"),
	
	/// TODO.
	VYG = Self::letters_to_token(b"VYG"),
	
	/// TODO.
	VYH = Self::letters_to_token(b"VYH"),
	
	/// TODO.
	VYI = Self::letters_to_token(b"VYI"),
	
	/// TODO.
	VYJ = Self::letters_to_token(b"VYJ"),
	
	/// TODO.
	VYK = Self::letters_to_token(b"VYK"),
	
	/// TODO.
	VYL = Self::letters_to_token(b"VYL"),
	
	/// TODO.
	VYM = Self::letters_to_token(b"VYM"),
	
	/// TODO.
	VYN = Self::letters_to_token(b"VYN"),
	
	/// TODO.
	VYO = Self::letters_to_token(b"VYO"),
	
	/// TODO.
	VYP = Self::letters_to_token(b"VYP"),
	
	/// TODO.
	VYQ = Self::letters_to_token(b"VYQ"),
	
	/// TODO.
	VYR = Self::letters_to_token(b"VYR"),
	
	/// TODO.
	VYS = Self::letters_to_token(b"VYS"),
	
	/// TODO.
	VYT = Self::letters_to_token(b"VYT"),
	
	/// TODO.
	VYU = Self::letters_to_token(b"VYU"),
	
	/// TODO.
	VYV = Self::letters_to_token(b"VYV"),
	
	/// TODO.
	VYW = Self::letters_to_token(b"VYW"),
	
	/// TODO.
	VYX = Self::letters_to_token(b"VYX"),
	
	/// TODO.
	VYY = Self::letters_to_token(b"VYY"),
	
	/// TODO.
	VYZ = Self::letters_to_token(b"VYZ"),
	
	/// TODO.
	VZA = Self::letters_to_token(b"VZA"),
	
	/// TODO.
	VZB = Self::letters_to_token(b"VZB"),
	
	/// TODO.
	VZC = Self::letters_to_token(b"VZC"),
	
	/// TODO.
	VZD = Self::letters_to_token(b"VZD"),
	
	/// TODO.
	VZE = Self::letters_to_token(b"VZE"),
	
	/// TODO.
	VZF = Self::letters_to_token(b"VZF"),
	
	/// TODO.
	VZG = Self::letters_to_token(b"VZG"),
	
	/// TODO.
	VZH = Self::letters_to_token(b"VZH"),
	
	/// TODO.
	VZI = Self::letters_to_token(b"VZI"),
	
	/// TODO.
	VZJ = Self::letters_to_token(b"VZJ"),
	
	/// TODO.
	VZK = Self::letters_to_token(b"VZK"),
	
	/// TODO.
	VZL = Self::letters_to_token(b"VZL"),
	
	/// TODO.
	VZM = Self::letters_to_token(b"VZM"),
	
	/// TODO.
	VZN = Self::letters_to_token(b"VZN"),
	
	/// TODO.
	VZO = Self::letters_to_token(b"VZO"),
	
	/// TODO.
	VZP = Self::letters_to_token(b"VZP"),
	
	/// TODO.
	VZQ = Self::letters_to_token(b"VZQ"),
	
	/// TODO.
	VZR = Self::letters_to_token(b"VZR"),
	
	/// TODO.
	VZS = Self::letters_to_token(b"VZS"),
	
	/// TODO.
	VZT = Self::letters_to_token(b"VZT"),
	
	/// TODO.
	VZU = Self::letters_to_token(b"VZU"),
	
	/// TODO.
	VZV = Self::letters_to_token(b"VZV"),
	
	/// TODO.
	VZW = Self::letters_to_token(b"VZW"),
	
	/// TODO.
	VZX = Self::letters_to_token(b"VZX"),
	
	/// TODO.
	VZY = Self::letters_to_token(b"VZY"),
	
	/// TODO.
	VZZ = Self::letters_to_token(b"VZZ"),
	
	/// TODO.
	WAA = Self::letters_to_token(b"WAA"),
	
	/// TODO.
	WAB = Self::letters_to_token(b"WAB"),
	
	/// TODO.
	WAC = Self::letters_to_token(b"WAC"),
	
	/// TODO.
	WAD = Self::letters_to_token(b"WAD"),
	
	/// TODO.
	WAE = Self::letters_to_token(b"WAE"),
	
	/// TODO.
	WAF = Self::letters_to_token(b"WAF"),
	
	/// TODO.
	WAG = Self::letters_to_token(b"WAG"),
	
	/// TODO.
	WAH = Self::letters_to_token(b"WAH"),
	
	/// TODO.
	WAI = Self::letters_to_token(b"WAI"),
	
	/// TODO.
	WAJ = Self::letters_to_token(b"WAJ"),
	
	/// TODO.
	WAK = Self::letters_to_token(b"WAK"),
	
	/// TODO.
	WAL = Self::letters_to_token(b"WAL"),
	
	/// TODO.
	WAM = Self::letters_to_token(b"WAM"),
	
	/// TODO.
	WAN = Self::letters_to_token(b"WAN"),
	
	/// TODO.
	WAO = Self::letters_to_token(b"WAO"),
	
	/// TODO.
	WAP = Self::letters_to_token(b"WAP"),
	
	/// TODO.
	WAQ = Self::letters_to_token(b"WAQ"),
	
	/// TODO.
	WAR = Self::letters_to_token(b"WAR"),
	
	/// TODO.
	WAS = Self::letters_to_token(b"WAS"),
	
	/// TODO.
	WAT = Self::letters_to_token(b"WAT"),
	
	/// TODO.
	WAU = Self::letters_to_token(b"WAU"),
	
	/// TODO.
	WAV = Self::letters_to_token(b"WAV"),
	
	/// TODO.
	WAW = Self::letters_to_token(b"WAW"),
	
	/// TODO.
	WAX = Self::letters_to_token(b"WAX"),
	
	/// TODO.
	WAY = Self::letters_to_token(b"WAY"),
	
	/// TODO.
	WAZ = Self::letters_to_token(b"WAZ"),
	
	/// TODO.
	WBA = Self::letters_to_token(b"WBA"),
	
	/// TODO.
	WBB = Self::letters_to_token(b"WBB"),
	
	/// TODO.
	WBC = Self::letters_to_token(b"WBC"),
	
	/// TODO.
	WBD = Self::letters_to_token(b"WBD"),
	
	/// TODO.
	WBE = Self::letters_to_token(b"WBE"),
	
	/// TODO.
	WBF = Self::letters_to_token(b"WBF"),
	
	/// TODO.
	WBG = Self::letters_to_token(b"WBG"),
	
	/// TODO.
	WBH = Self::letters_to_token(b"WBH"),
	
	/// TODO.
	WBI = Self::letters_to_token(b"WBI"),
	
	/// TODO.
	WBJ = Self::letters_to_token(b"WBJ"),
	
	/// TODO.
	WBK = Self::letters_to_token(b"WBK"),
	
	/// TODO.
	WBL = Self::letters_to_token(b"WBL"),
	
	/// TODO.
	WBM = Self::letters_to_token(b"WBM"),
	
	/// TODO.
	WBN = Self::letters_to_token(b"WBN"),
	
	/// TODO.
	WBO = Self::letters_to_token(b"WBO"),
	
	/// TODO.
	WBP = Self::letters_to_token(b"WBP"),
	
	/// TODO.
	WBQ = Self::letters_to_token(b"WBQ"),
	
	/// TODO.
	WBR = Self::letters_to_token(b"WBR"),
	
	/// TODO.
	WBS = Self::letters_to_token(b"WBS"),
	
	/// TODO.
	WBT = Self::letters_to_token(b"WBT"),
	
	/// TODO.
	WBU = Self::letters_to_token(b"WBU"),
	
	/// TODO.
	WBV = Self::letters_to_token(b"WBV"),
	
	/// TODO.
	WBW = Self::letters_to_token(b"WBW"),
	
	/// TODO.
	WBX = Self::letters_to_token(b"WBX"),
	
	/// TODO.
	WBY = Self::letters_to_token(b"WBY"),
	
	/// TODO.
	WBZ = Self::letters_to_token(b"WBZ"),
	
	/// TODO.
	WCA = Self::letters_to_token(b"WCA"),
	
	/// TODO.
	WCB = Self::letters_to_token(b"WCB"),
	
	/// TODO.
	WCC = Self::letters_to_token(b"WCC"),
	
	/// TODO.
	WCD = Self::letters_to_token(b"WCD"),
	
	/// TODO.
	WCE = Self::letters_to_token(b"WCE"),
	
	/// TODO.
	WCF = Self::letters_to_token(b"WCF"),
	
	/// TODO.
	WCG = Self::letters_to_token(b"WCG"),
	
	/// TODO.
	WCH = Self::letters_to_token(b"WCH"),
	
	/// TODO.
	WCI = Self::letters_to_token(b"WCI"),
	
	/// TODO.
	WCJ = Self::letters_to_token(b"WCJ"),
	
	/// TODO.
	WCK = Self::letters_to_token(b"WCK"),
	
	/// TODO.
	WCL = Self::letters_to_token(b"WCL"),
	
	/// TODO.
	WCM = Self::letters_to_token(b"WCM"),
	
	/// TODO.
	WCN = Self::letters_to_token(b"WCN"),
	
	/// TODO.
	WCO = Self::letters_to_token(b"WCO"),
	
	/// TODO.
	WCP = Self::letters_to_token(b"WCP"),
	
	/// TODO.
	WCQ = Self::letters_to_token(b"WCQ"),
	
	/// TODO.
	WCR = Self::letters_to_token(b"WCR"),
	
	/// TODO.
	WCS = Self::letters_to_token(b"WCS"),
	
	/// TODO.
	WCT = Self::letters_to_token(b"WCT"),
	
	/// TODO.
	WCU = Self::letters_to_token(b"WCU"),
	
	/// TODO.
	WCV = Self::letters_to_token(b"WCV"),
	
	/// TODO.
	WCW = Self::letters_to_token(b"WCW"),
	
	/// TODO.
	WCX = Self::letters_to_token(b"WCX"),
	
	/// TODO.
	WCY = Self::letters_to_token(b"WCY"),
	
	/// TODO.
	WCZ = Self::letters_to_token(b"WCZ"),
	
	/// TODO.
	WDA = Self::letters_to_token(b"WDA"),
	
	/// TODO.
	WDB = Self::letters_to_token(b"WDB"),
	
	/// TODO.
	WDC = Self::letters_to_token(b"WDC"),
	
	/// TODO.
	WDD = Self::letters_to_token(b"WDD"),
	
	/// TODO.
	WDE = Self::letters_to_token(b"WDE"),
	
	/// TODO.
	WDF = Self::letters_to_token(b"WDF"),
	
	/// TODO.
	WDG = Self::letters_to_token(b"WDG"),
	
	/// TODO.
	WDH = Self::letters_to_token(b"WDH"),
	
	/// TODO.
	WDI = Self::letters_to_token(b"WDI"),
	
	/// TODO.
	WDJ = Self::letters_to_token(b"WDJ"),
	
	/// TODO.
	WDK = Self::letters_to_token(b"WDK"),
	
	/// TODO.
	WDL = Self::letters_to_token(b"WDL"),
	
	/// TODO.
	WDM = Self::letters_to_token(b"WDM"),
	
	/// TODO.
	WDN = Self::letters_to_token(b"WDN"),
	
	/// TODO.
	WDO = Self::letters_to_token(b"WDO"),
	
	/// TODO.
	WDP = Self::letters_to_token(b"WDP"),
	
	/// TODO.
	WDQ = Self::letters_to_token(b"WDQ"),
	
	/// TODO.
	WDR = Self::letters_to_token(b"WDR"),
	
	/// TODO.
	WDS = Self::letters_to_token(b"WDS"),
	
	/// TODO.
	WDT = Self::letters_to_token(b"WDT"),
	
	/// TODO.
	WDU = Self::letters_to_token(b"WDU"),
	
	/// TODO.
	WDV = Self::letters_to_token(b"WDV"),
	
	/// TODO.
	WDW = Self::letters_to_token(b"WDW"),
	
	/// TODO.
	WDX = Self::letters_to_token(b"WDX"),
	
	/// TODO.
	WDY = Self::letters_to_token(b"WDY"),
	
	/// TODO.
	WDZ = Self::letters_to_token(b"WDZ"),
	
	/// TODO.
	WEA = Self::letters_to_token(b"WEA"),
	
	/// TODO.
	WEB = Self::letters_to_token(b"WEB"),
	
	/// TODO.
	WEC = Self::letters_to_token(b"WEC"),
	
	/// TODO.
	WED = Self::letters_to_token(b"WED"),
	
	/// TODO.
	WEE = Self::letters_to_token(b"WEE"),
	
	/// TODO.
	WEF = Self::letters_to_token(b"WEF"),
	
	/// TODO.
	WEG = Self::letters_to_token(b"WEG"),
	
	/// TODO.
	WEH = Self::letters_to_token(b"WEH"),
	
	/// TODO.
	WEI = Self::letters_to_token(b"WEI"),
	
	/// TODO.
	WEJ = Self::letters_to_token(b"WEJ"),
	
	/// TODO.
	WEK = Self::letters_to_token(b"WEK"),
	
	/// TODO.
	WEL = Self::letters_to_token(b"WEL"),
	
	/// TODO.
	WEM = Self::letters_to_token(b"WEM"),
	
	/// TODO.
	WEN = Self::letters_to_token(b"WEN"),
	
	/// TODO.
	WEO = Self::letters_to_token(b"WEO"),
	
	/// TODO.
	WEP = Self::letters_to_token(b"WEP"),
	
	/// TODO.
	WEQ = Self::letters_to_token(b"WEQ"),
	
	/// TODO.
	WER = Self::letters_to_token(b"WER"),
	
	/// TODO.
	WES = Self::letters_to_token(b"WES"),
	
	/// TODO.
	WET = Self::letters_to_token(b"WET"),
	
	/// TODO.
	WEU = Self::letters_to_token(b"WEU"),
	
	/// TODO.
	WEV = Self::letters_to_token(b"WEV"),
	
	/// TODO.
	WEW = Self::letters_to_token(b"WEW"),
	
	/// TODO.
	WEX = Self::letters_to_token(b"WEX"),
	
	/// TODO.
	WEY = Self::letters_to_token(b"WEY"),
	
	/// TODO.
	WEZ = Self::letters_to_token(b"WEZ"),
	
	/// TODO.
	WFA = Self::letters_to_token(b"WFA"),
	
	/// TODO.
	WFB = Self::letters_to_token(b"WFB"),
	
	/// TODO.
	WFC = Self::letters_to_token(b"WFC"),
	
	/// TODO.
	WFD = Self::letters_to_token(b"WFD"),
	
	/// TODO.
	WFE = Self::letters_to_token(b"WFE"),
	
	/// TODO.
	WFF = Self::letters_to_token(b"WFF"),
	
	/// TODO.
	WFG = Self::letters_to_token(b"WFG"),
	
	/// TODO.
	WFH = Self::letters_to_token(b"WFH"),
	
	/// TODO.
	WFI = Self::letters_to_token(b"WFI"),
	
	/// TODO.
	WFJ = Self::letters_to_token(b"WFJ"),
	
	/// TODO.
	WFK = Self::letters_to_token(b"WFK"),
	
	/// TODO.
	WFL = Self::letters_to_token(b"WFL"),
	
	/// TODO.
	WFM = Self::letters_to_token(b"WFM"),
	
	/// TODO.
	WFN = Self::letters_to_token(b"WFN"),
	
	/// TODO.
	WFO = Self::letters_to_token(b"WFO"),
	
	/// TODO.
	WFP = Self::letters_to_token(b"WFP"),
	
	/// TODO.
	WFQ = Self::letters_to_token(b"WFQ"),
	
	/// TODO.
	WFR = Self::letters_to_token(b"WFR"),
	
	/// TODO.
	WFS = Self::letters_to_token(b"WFS"),
	
	/// TODO.
	WFT = Self::letters_to_token(b"WFT"),
	
	/// TODO.
	WFU = Self::letters_to_token(b"WFU"),
	
	/// TODO.
	WFV = Self::letters_to_token(b"WFV"),
	
	/// TODO.
	WFW = Self::letters_to_token(b"WFW"),
	
	/// TODO.
	WFX = Self::letters_to_token(b"WFX"),
	
	/// TODO.
	WFY = Self::letters_to_token(b"WFY"),
	
	/// TODO.
	WFZ = Self::letters_to_token(b"WFZ"),
	
	/// TODO.
	WGA = Self::letters_to_token(b"WGA"),
	
	/// TODO.
	WGB = Self::letters_to_token(b"WGB"),
	
	/// TODO.
	WGC = Self::letters_to_token(b"WGC"),
	
	/// TODO.
	WGD = Self::letters_to_token(b"WGD"),
	
	/// TODO.
	WGE = Self::letters_to_token(b"WGE"),
	
	/// TODO.
	WGF = Self::letters_to_token(b"WGF"),
	
	/// TODO.
	WGG = Self::letters_to_token(b"WGG"),
	
	/// TODO.
	WGH = Self::letters_to_token(b"WGH"),
	
	/// TODO.
	WGI = Self::letters_to_token(b"WGI"),
	
	/// TODO.
	WGJ = Self::letters_to_token(b"WGJ"),
	
	/// TODO.
	WGK = Self::letters_to_token(b"WGK"),
	
	/// TODO.
	WGL = Self::letters_to_token(b"WGL"),
	
	/// TODO.
	WGM = Self::letters_to_token(b"WGM"),
	
	/// TODO.
	WGN = Self::letters_to_token(b"WGN"),
	
	/// TODO.
	WGO = Self::letters_to_token(b"WGO"),
	
	/// TODO.
	WGP = Self::letters_to_token(b"WGP"),
	
	/// TODO.
	WGQ = Self::letters_to_token(b"WGQ"),
	
	/// TODO.
	WGR = Self::letters_to_token(b"WGR"),
	
	/// TODO.
	WGS = Self::letters_to_token(b"WGS"),
	
	/// TODO.
	WGT = Self::letters_to_token(b"WGT"),
	
	/// TODO.
	WGU = Self::letters_to_token(b"WGU"),
	
	/// TODO.
	WGV = Self::letters_to_token(b"WGV"),
	
	/// TODO.
	WGW = Self::letters_to_token(b"WGW"),
	
	/// TODO.
	WGX = Self::letters_to_token(b"WGX"),
	
	/// TODO.
	WGY = Self::letters_to_token(b"WGY"),
	
	/// TODO.
	WGZ = Self::letters_to_token(b"WGZ"),
	
	/// TODO.
	WHA = Self::letters_to_token(b"WHA"),
	
	/// TODO.
	WHB = Self::letters_to_token(b"WHB"),
	
	/// TODO.
	WHC = Self::letters_to_token(b"WHC"),
	
	/// TODO.
	WHD = Self::letters_to_token(b"WHD"),
	
	/// TODO.
	WHE = Self::letters_to_token(b"WHE"),
	
	/// TODO.
	WHF = Self::letters_to_token(b"WHF"),
	
	/// TODO.
	WHG = Self::letters_to_token(b"WHG"),
	
	/// TODO.
	WHH = Self::letters_to_token(b"WHH"),
	
	/// TODO.
	WHI = Self::letters_to_token(b"WHI"),
	
	/// TODO.
	WHJ = Self::letters_to_token(b"WHJ"),
	
	/// TODO.
	WHK = Self::letters_to_token(b"WHK"),
	
	/// TODO.
	WHL = Self::letters_to_token(b"WHL"),
	
	/// TODO.
	WHM = Self::letters_to_token(b"WHM"),
	
	/// TODO.
	WHN = Self::letters_to_token(b"WHN"),
	
	/// TODO.
	WHO = Self::letters_to_token(b"WHO"),
	
	/// TODO.
	WHP = Self::letters_to_token(b"WHP"),
	
	/// TODO.
	WHQ = Self::letters_to_token(b"WHQ"),
	
	/// TODO.
	WHR = Self::letters_to_token(b"WHR"),
	
	/// TODO.
	WHS = Self::letters_to_token(b"WHS"),
	
	/// TODO.
	WHT = Self::letters_to_token(b"WHT"),
	
	/// TODO.
	WHU = Self::letters_to_token(b"WHU"),
	
	/// TODO.
	WHV = Self::letters_to_token(b"WHV"),
	
	/// TODO.
	WHW = Self::letters_to_token(b"WHW"),
	
	/// TODO.
	WHX = Self::letters_to_token(b"WHX"),
	
	/// TODO.
	WHY = Self::letters_to_token(b"WHY"),
	
	/// TODO.
	WHZ = Self::letters_to_token(b"WHZ"),
	
	/// TODO.
	WIA = Self::letters_to_token(b"WIA"),
	
	/// TODO.
	WIB = Self::letters_to_token(b"WIB"),
	
	/// TODO.
	WIC = Self::letters_to_token(b"WIC"),
	
	/// TODO.
	WID = Self::letters_to_token(b"WID"),
	
	/// TODO.
	WIE = Self::letters_to_token(b"WIE"),
	
	/// TODO.
	WIF = Self::letters_to_token(b"WIF"),
	
	/// TODO.
	WIG = Self::letters_to_token(b"WIG"),
	
	/// TODO.
	WIH = Self::letters_to_token(b"WIH"),
	
	/// TODO.
	WII = Self::letters_to_token(b"WII"),
	
	/// TODO.
	WIJ = Self::letters_to_token(b"WIJ"),
	
	/// TODO.
	WIK = Self::letters_to_token(b"WIK"),
	
	/// TODO.
	WIL = Self::letters_to_token(b"WIL"),
	
	/// TODO.
	WIM = Self::letters_to_token(b"WIM"),
	
	/// TODO.
	WIN = Self::letters_to_token(b"WIN"),
	
	/// TODO.
	WIO = Self::letters_to_token(b"WIO"),
	
	/// TODO.
	WIP = Self::letters_to_token(b"WIP"),
	
	/// TODO.
	WIQ = Self::letters_to_token(b"WIQ"),
	
	/// TODO.
	WIR = Self::letters_to_token(b"WIR"),
	
	/// TODO.
	WIS = Self::letters_to_token(b"WIS"),
	
	/// TODO.
	WIT = Self::letters_to_token(b"WIT"),
	
	/// TODO.
	WIU = Self::letters_to_token(b"WIU"),
	
	/// TODO.
	WIV = Self::letters_to_token(b"WIV"),
	
	/// TODO.
	WIW = Self::letters_to_token(b"WIW"),
	
	/// TODO.
	WIX = Self::letters_to_token(b"WIX"),
	
	/// TODO.
	WIY = Self::letters_to_token(b"WIY"),
	
	/// TODO.
	WIZ = Self::letters_to_token(b"WIZ"),
	
	/// TODO.
	WJA = Self::letters_to_token(b"WJA"),
	
	/// TODO.
	WJB = Self::letters_to_token(b"WJB"),
	
	/// TODO.
	WJC = Self::letters_to_token(b"WJC"),
	
	/// TODO.
	WJD = Self::letters_to_token(b"WJD"),
	
	/// TODO.
	WJE = Self::letters_to_token(b"WJE"),
	
	/// TODO.
	WJF = Self::letters_to_token(b"WJF"),
	
	/// TODO.
	WJG = Self::letters_to_token(b"WJG"),
	
	/// TODO.
	WJH = Self::letters_to_token(b"WJH"),
	
	/// TODO.
	WJI = Self::letters_to_token(b"WJI"),
	
	/// TODO.
	WJJ = Self::letters_to_token(b"WJJ"),
	
	/// TODO.
	WJK = Self::letters_to_token(b"WJK"),
	
	/// TODO.
	WJL = Self::letters_to_token(b"WJL"),
	
	/// TODO.
	WJM = Self::letters_to_token(b"WJM"),
	
	/// TODO.
	WJN = Self::letters_to_token(b"WJN"),
	
	/// TODO.
	WJO = Self::letters_to_token(b"WJO"),
	
	/// TODO.
	WJP = Self::letters_to_token(b"WJP"),
	
	/// TODO.
	WJQ = Self::letters_to_token(b"WJQ"),
	
	/// TODO.
	WJR = Self::letters_to_token(b"WJR"),
	
	/// TODO.
	WJS = Self::letters_to_token(b"WJS"),
	
	/// TODO.
	WJT = Self::letters_to_token(b"WJT"),
	
	/// TODO.
	WJU = Self::letters_to_token(b"WJU"),
	
	/// TODO.
	WJV = Self::letters_to_token(b"WJV"),
	
	/// TODO.
	WJW = Self::letters_to_token(b"WJW"),
	
	/// TODO.
	WJX = Self::letters_to_token(b"WJX"),
	
	/// TODO.
	WJY = Self::letters_to_token(b"WJY"),
	
	/// TODO.
	WJZ = Self::letters_to_token(b"WJZ"),
	
	/// TODO.
	WKA = Self::letters_to_token(b"WKA"),
	
	/// TODO.
	WKB = Self::letters_to_token(b"WKB"),
	
	/// TODO.
	WKC = Self::letters_to_token(b"WKC"),
	
	/// TODO.
	WKD = Self::letters_to_token(b"WKD"),
	
	/// TODO.
	WKE = Self::letters_to_token(b"WKE"),
	
	/// TODO.
	WKF = Self::letters_to_token(b"WKF"),
	
	/// TODO.
	WKG = Self::letters_to_token(b"WKG"),
	
	/// TODO.
	WKH = Self::letters_to_token(b"WKH"),
	
	/// TODO.
	WKI = Self::letters_to_token(b"WKI"),
	
	/// TODO.
	WKJ = Self::letters_to_token(b"WKJ"),
	
	/// TODO.
	WKK = Self::letters_to_token(b"WKK"),
	
	/// TODO.
	WKL = Self::letters_to_token(b"WKL"),
	
	/// TODO.
	WKM = Self::letters_to_token(b"WKM"),
	
	/// TODO.
	WKN = Self::letters_to_token(b"WKN"),
	
	/// TODO.
	WKO = Self::letters_to_token(b"WKO"),
	
	/// TODO.
	WKP = Self::letters_to_token(b"WKP"),
	
	/// TODO.
	WKQ = Self::letters_to_token(b"WKQ"),
	
	/// TODO.
	WKR = Self::letters_to_token(b"WKR"),
	
	/// TODO.
	WKS = Self::letters_to_token(b"WKS"),
	
	/// TODO.
	WKT = Self::letters_to_token(b"WKT"),
	
	/// TODO.
	WKU = Self::letters_to_token(b"WKU"),
	
	/// TODO.
	WKV = Self::letters_to_token(b"WKV"),
	
	/// TODO.
	WKW = Self::letters_to_token(b"WKW"),
	
	/// TODO.
	WKX = Self::letters_to_token(b"WKX"),
	
	/// TODO.
	WKY = Self::letters_to_token(b"WKY"),
	
	/// TODO.
	WKZ = Self::letters_to_token(b"WKZ"),
	
	/// TODO.
	WLA = Self::letters_to_token(b"WLA"),
	
	/// TODO.
	WLB = Self::letters_to_token(b"WLB"),
	
	/// TODO.
	WLC = Self::letters_to_token(b"WLC"),
	
	/// TODO.
	WLD = Self::letters_to_token(b"WLD"),
	
	/// TODO.
	WLE = Self::letters_to_token(b"WLE"),
	
	/// TODO.
	WLF = Self::letters_to_token(b"WLF"),
	
	/// TODO.
	WLG = Self::letters_to_token(b"WLG"),
	
	/// TODO.
	WLH = Self::letters_to_token(b"WLH"),
	
	/// TODO.
	WLI = Self::letters_to_token(b"WLI"),
	
	/// TODO.
	WLJ = Self::letters_to_token(b"WLJ"),
	
	/// TODO.
	WLK = Self::letters_to_token(b"WLK"),
	
	/// TODO.
	WLL = Self::letters_to_token(b"WLL"),
	
	/// TODO.
	WLM = Self::letters_to_token(b"WLM"),
	
	/// TODO.
	WLN = Self::letters_to_token(b"WLN"),
	
	/// TODO.
	WLO = Self::letters_to_token(b"WLO"),
	
	/// TODO.
	WLP = Self::letters_to_token(b"WLP"),
	
	/// TODO.
	WLQ = Self::letters_to_token(b"WLQ"),
	
	/// TODO.
	WLR = Self::letters_to_token(b"WLR"),
	
	/// TODO.
	WLS = Self::letters_to_token(b"WLS"),
	
	/// TODO.
	WLT = Self::letters_to_token(b"WLT"),
	
	/// TODO.
	WLU = Self::letters_to_token(b"WLU"),
	
	/// TODO.
	WLV = Self::letters_to_token(b"WLV"),
	
	/// TODO.
	WLW = Self::letters_to_token(b"WLW"),
	
	/// TODO.
	WLX = Self::letters_to_token(b"WLX"),
	
	/// TODO.
	WLY = Self::letters_to_token(b"WLY"),
	
	/// TODO.
	WLZ = Self::letters_to_token(b"WLZ"),
	
	/// TODO.
	WMA = Self::letters_to_token(b"WMA"),
	
	/// TODO.
	WMB = Self::letters_to_token(b"WMB"),
	
	/// TODO.
	WMC = Self::letters_to_token(b"WMC"),
	
	/// TODO.
	WMD = Self::letters_to_token(b"WMD"),
	
	/// TODO.
	WME = Self::letters_to_token(b"WME"),
	
	/// TODO.
	WMF = Self::letters_to_token(b"WMF"),
	
	/// TODO.
	WMG = Self::letters_to_token(b"WMG"),
	
	/// TODO.
	WMH = Self::letters_to_token(b"WMH"),
	
	/// TODO.
	WMI = Self::letters_to_token(b"WMI"),
	
	/// TODO.
	WMJ = Self::letters_to_token(b"WMJ"),
	
	/// TODO.
	WMK = Self::letters_to_token(b"WMK"),
	
	/// TODO.
	WML = Self::letters_to_token(b"WML"),
	
	/// TODO.
	WMM = Self::letters_to_token(b"WMM"),
	
	/// TODO.
	WMN = Self::letters_to_token(b"WMN"),
	
	/// TODO.
	WMO = Self::letters_to_token(b"WMO"),
	
	/// TODO.
	WMP = Self::letters_to_token(b"WMP"),
	
	/// TODO.
	WMQ = Self::letters_to_token(b"WMQ"),
	
	/// TODO.
	WMR = Self::letters_to_token(b"WMR"),
	
	/// TODO.
	WMS = Self::letters_to_token(b"WMS"),
	
	/// TODO.
	WMT = Self::letters_to_token(b"WMT"),
	
	/// TODO.
	WMU = Self::letters_to_token(b"WMU"),
	
	/// TODO.
	WMV = Self::letters_to_token(b"WMV"),
	
	/// TODO.
	WMW = Self::letters_to_token(b"WMW"),
	
	/// TODO.
	WMX = Self::letters_to_token(b"WMX"),
	
	/// TODO.
	WMY = Self::letters_to_token(b"WMY"),
	
	/// TODO.
	WMZ = Self::letters_to_token(b"WMZ"),
	
	/// TODO.
	WNA = Self::letters_to_token(b"WNA"),
	
	/// TODO.
	WNB = Self::letters_to_token(b"WNB"),
	
	/// TODO.
	WNC = Self::letters_to_token(b"WNC"),
	
	/// TODO.
	WND = Self::letters_to_token(b"WND"),
	
	/// TODO.
	WNE = Self::letters_to_token(b"WNE"),
	
	/// TODO.
	WNF = Self::letters_to_token(b"WNF"),
	
	/// TODO.
	WNG = Self::letters_to_token(b"WNG"),
	
	/// TODO.
	WNH = Self::letters_to_token(b"WNH"),
	
	/// TODO.
	WNI = Self::letters_to_token(b"WNI"),
	
	/// TODO.
	WNJ = Self::letters_to_token(b"WNJ"),
	
	/// TODO.
	WNK = Self::letters_to_token(b"WNK"),
	
	/// TODO.
	WNL = Self::letters_to_token(b"WNL"),
	
	/// TODO.
	WNM = Self::letters_to_token(b"WNM"),
	
	/// TODO.
	WNN = Self::letters_to_token(b"WNN"),
	
	/// TODO.
	WNO = Self::letters_to_token(b"WNO"),
	
	/// TODO.
	WNP = Self::letters_to_token(b"WNP"),
	
	/// TODO.
	WNQ = Self::letters_to_token(b"WNQ"),
	
	/// TODO.
	WNR = Self::letters_to_token(b"WNR"),
	
	/// TODO.
	WNS = Self::letters_to_token(b"WNS"),
	
	/// TODO.
	WNT = Self::letters_to_token(b"WNT"),
	
	/// TODO.
	WNU = Self::letters_to_token(b"WNU"),
	
	/// TODO.
	WNV = Self::letters_to_token(b"WNV"),
	
	/// TODO.
	WNW = Self::letters_to_token(b"WNW"),
	
	/// TODO.
	WNX = Self::letters_to_token(b"WNX"),
	
	/// TODO.
	WNY = Self::letters_to_token(b"WNY"),
	
	/// TODO.
	WNZ = Self::letters_to_token(b"WNZ"),
	
	/// TODO.
	WOA = Self::letters_to_token(b"WOA"),
	
	/// TODO.
	WOB = Self::letters_to_token(b"WOB"),
	
	/// TODO.
	WOC = Self::letters_to_token(b"WOC"),
	
	/// TODO.
	WOD = Self::letters_to_token(b"WOD"),
	
	/// TODO.
	WOE = Self::letters_to_token(b"WOE"),
	
	/// TODO.
	WOF = Self::letters_to_token(b"WOF"),
	
	/// TODO.
	WOG = Self::letters_to_token(b"WOG"),
	
	/// TODO.
	WOH = Self::letters_to_token(b"WOH"),
	
	/// TODO.
	WOI = Self::letters_to_token(b"WOI"),
	
	/// TODO.
	WOJ = Self::letters_to_token(b"WOJ"),
	
	/// TODO.
	WOK = Self::letters_to_token(b"WOK"),
	
	/// TODO.
	WOL = Self::letters_to_token(b"WOL"),
	
	/// TODO.
	WOM = Self::letters_to_token(b"WOM"),
	
	/// TODO.
	WON = Self::letters_to_token(b"WON"),
	
	/// TODO.
	WOO = Self::letters_to_token(b"WOO"),
	
	/// TODO.
	WOP = Self::letters_to_token(b"WOP"),
	
	/// TODO.
	WOQ = Self::letters_to_token(b"WOQ"),
	
	/// TODO.
	WOR = Self::letters_to_token(b"WOR"),
	
	/// TODO.
	WOS = Self::letters_to_token(b"WOS"),
	
	/// TODO.
	WOT = Self::letters_to_token(b"WOT"),
	
	/// TODO.
	WOU = Self::letters_to_token(b"WOU"),
	
	/// TODO.
	WOV = Self::letters_to_token(b"WOV"),
	
	/// TODO.
	WOW = Self::letters_to_token(b"WOW"),
	
	/// TODO.
	WOX = Self::letters_to_token(b"WOX"),
	
	/// TODO.
	WOY = Self::letters_to_token(b"WOY"),
	
	/// TODO.
	WOZ = Self::letters_to_token(b"WOZ"),
	
	/// TODO.
	WPA = Self::letters_to_token(b"WPA"),
	
	/// TODO.
	WPB = Self::letters_to_token(b"WPB"),
	
	/// TODO.
	WPC = Self::letters_to_token(b"WPC"),
	
	/// TODO.
	WPD = Self::letters_to_token(b"WPD"),
	
	/// TODO.
	WPE = Self::letters_to_token(b"WPE"),
	
	/// TODO.
	WPF = Self::letters_to_token(b"WPF"),
	
	/// TODO.
	WPG = Self::letters_to_token(b"WPG"),
	
	/// TODO.
	WPH = Self::letters_to_token(b"WPH"),
	
	/// TODO.
	WPI = Self::letters_to_token(b"WPI"),
	
	/// TODO.
	WPJ = Self::letters_to_token(b"WPJ"),
	
	/// TODO.
	WPK = Self::letters_to_token(b"WPK"),
	
	/// TODO.
	WPL = Self::letters_to_token(b"WPL"),
	
	/// TODO.
	WPM = Self::letters_to_token(b"WPM"),
	
	/// TODO.
	WPN = Self::letters_to_token(b"WPN"),
	
	/// TODO.
	WPO = Self::letters_to_token(b"WPO"),
	
	/// TODO.
	WPP = Self::letters_to_token(b"WPP"),
	
	/// TODO.
	WPQ = Self::letters_to_token(b"WPQ"),
	
	/// TODO.
	WPR = Self::letters_to_token(b"WPR"),
	
	/// TODO.
	WPS = Self::letters_to_token(b"WPS"),
	
	/// TODO.
	WPT = Self::letters_to_token(b"WPT"),
	
	/// TODO.
	WPU = Self::letters_to_token(b"WPU"),
	
	/// TODO.
	WPV = Self::letters_to_token(b"WPV"),
	
	/// TODO.
	WPW = Self::letters_to_token(b"WPW"),
	
	/// TODO.
	WPX = Self::letters_to_token(b"WPX"),
	
	/// TODO.
	WPY = Self::letters_to_token(b"WPY"),
	
	/// TODO.
	WPZ = Self::letters_to_token(b"WPZ"),
	
	/// TODO.
	WQA = Self::letters_to_token(b"WQA"),
	
	/// TODO.
	WQB = Self::letters_to_token(b"WQB"),
	
	/// TODO.
	WQC = Self::letters_to_token(b"WQC"),
	
	/// TODO.
	WQD = Self::letters_to_token(b"WQD"),
	
	/// TODO.
	WQE = Self::letters_to_token(b"WQE"),
	
	/// TODO.
	WQF = Self::letters_to_token(b"WQF"),
	
	/// TODO.
	WQG = Self::letters_to_token(b"WQG"),
	
	/// TODO.
	WQH = Self::letters_to_token(b"WQH"),
	
	/// TODO.
	WQI = Self::letters_to_token(b"WQI"),
	
	/// TODO.
	WQJ = Self::letters_to_token(b"WQJ"),
	
	/// TODO.
	WQK = Self::letters_to_token(b"WQK"),
	
	/// TODO.
	WQL = Self::letters_to_token(b"WQL"),
	
	/// TODO.
	WQM = Self::letters_to_token(b"WQM"),
	
	/// TODO.
	WQN = Self::letters_to_token(b"WQN"),
	
	/// TODO.
	WQO = Self::letters_to_token(b"WQO"),
	
	/// TODO.
	WQP = Self::letters_to_token(b"WQP"),
	
	/// TODO.
	WQQ = Self::letters_to_token(b"WQQ"),
	
	/// TODO.
	WQR = Self::letters_to_token(b"WQR"),
	
	/// TODO.
	WQS = Self::letters_to_token(b"WQS"),
	
	/// TODO.
	WQT = Self::letters_to_token(b"WQT"),
	
	/// TODO.
	WQU = Self::letters_to_token(b"WQU"),
	
	/// TODO.
	WQV = Self::letters_to_token(b"WQV"),
	
	/// TODO.
	WQW = Self::letters_to_token(b"WQW"),
	
	/// TODO.
	WQX = Self::letters_to_token(b"WQX"),
	
	/// TODO.
	WQY = Self::letters_to_token(b"WQY"),
	
	/// TODO.
	WQZ = Self::letters_to_token(b"WQZ"),
	
	/// TODO.
	WRA = Self::letters_to_token(b"WRA"),
	
	/// TODO.
	WRB = Self::letters_to_token(b"WRB"),
	
	/// TODO.
	WRC = Self::letters_to_token(b"WRC"),
	
	/// TODO.
	WRD = Self::letters_to_token(b"WRD"),
	
	/// TODO.
	WRE = Self::letters_to_token(b"WRE"),
	
	/// TODO.
	WRF = Self::letters_to_token(b"WRF"),
	
	/// TODO.
	WRG = Self::letters_to_token(b"WRG"),
	
	/// TODO.
	WRH = Self::letters_to_token(b"WRH"),
	
	/// TODO.
	WRI = Self::letters_to_token(b"WRI"),
	
	/// TODO.
	WRJ = Self::letters_to_token(b"WRJ"),
	
	/// TODO.
	WRK = Self::letters_to_token(b"WRK"),
	
	/// TODO.
	WRL = Self::letters_to_token(b"WRL"),
	
	/// TODO.
	WRM = Self::letters_to_token(b"WRM"),
	
	/// TODO.
	WRN = Self::letters_to_token(b"WRN"),
	
	/// TODO.
	WRO = Self::letters_to_token(b"WRO"),
	
	/// TODO.
	WRP = Self::letters_to_token(b"WRP"),
	
	/// TODO.
	WRQ = Self::letters_to_token(b"WRQ"),
	
	/// TODO.
	WRR = Self::letters_to_token(b"WRR"),
	
	/// TODO.
	WRS = Self::letters_to_token(b"WRS"),
	
	/// TODO.
	WRT = Self::letters_to_token(b"WRT"),
	
	/// TODO.
	WRU = Self::letters_to_token(b"WRU"),
	
	/// TODO.
	WRV = Self::letters_to_token(b"WRV"),
	
	/// TODO.
	WRW = Self::letters_to_token(b"WRW"),
	
	/// TODO.
	WRX = Self::letters_to_token(b"WRX"),
	
	/// TODO.
	WRY = Self::letters_to_token(b"WRY"),
	
	/// TODO.
	WRZ = Self::letters_to_token(b"WRZ"),
	
	/// Unoffical.
	///
	/// World Service Authority (WSA) code filed but unadopted by International Civil Aviation Organization ICAO for passports.
	WSA = Self::letters_to_token(b"WSA"),
	
	/// TODO.
	WSB = Self::letters_to_token(b"WSB"),
	
	/// TODO.
	WSC = Self::letters_to_token(b"WSC"),
	
	/// TODO.
	WSD = Self::letters_to_token(b"WSD"),
	
	/// TODO.
	WSE = Self::letters_to_token(b"WSE"),
	
	/// TODO.
	WSF = Self::letters_to_token(b"WSF"),
	
	/// TODO.
	WSG = Self::letters_to_token(b"WSG"),
	
	/// TODO.
	WSH = Self::letters_to_token(b"WSH"),
	
	/// TODO.
	WSI = Self::letters_to_token(b"WSI"),
	
	/// TODO.
	WSJ = Self::letters_to_token(b"WSJ"),
	
	/// TODO.
	WSK = Self::letters_to_token(b"WSK"),
	
	/// TODO.
	WSL = Self::letters_to_token(b"WSL"),
	
	/// TODO.
	WSM = Self::letters_to_token(b"WSM"),
	
	/// TODO.
	WSN = Self::letters_to_token(b"WSN"),
	
	/// TODO.
	WSO = Self::letters_to_token(b"WSO"),
	
	/// TODO.
	WSP = Self::letters_to_token(b"WSP"),
	
	/// TODO.
	WSQ = Self::letters_to_token(b"WSQ"),
	
	/// TODO.
	WSR = Self::letters_to_token(b"WSR"),
	
	/// TODO.
	WSS = Self::letters_to_token(b"WSS"),
	
	/// TODO.
	WST = Self::letters_to_token(b"WST"),
	
	/// TODO.
	WSU = Self::letters_to_token(b"WSU"),
	
	/// TODO.
	WSV = Self::letters_to_token(b"WSV"),
	
	/// TODO.
	WSW = Self::letters_to_token(b"WSW"),
	
	/// TODO.
	WSX = Self::letters_to_token(b"WSX"),
	
	/// TODO.
	WSY = Self::letters_to_token(b"WSY"),
	
	/// TODO.
	WSZ = Self::letters_to_token(b"WSZ"),
	
	/// TODO.
	WTA = Self::letters_to_token(b"WTA"),
	
	/// TODO.
	WTB = Self::letters_to_token(b"WTB"),
	
	/// TODO.
	WTC = Self::letters_to_token(b"WTC"),
	
	/// TODO.
	WTD = Self::letters_to_token(b"WTD"),
	
	/// TODO.
	WTE = Self::letters_to_token(b"WTE"),
	
	/// TODO.
	WTF = Self::letters_to_token(b"WTF"),
	
	/// TODO.
	WTG = Self::letters_to_token(b"WTG"),
	
	/// TODO.
	WTH = Self::letters_to_token(b"WTH"),
	
	/// TODO.
	WTI = Self::letters_to_token(b"WTI"),
	
	/// TODO.
	WTJ = Self::letters_to_token(b"WTJ"),
	
	/// TODO.
	WTK = Self::letters_to_token(b"WTK"),
	
	/// TODO.
	WTL = Self::letters_to_token(b"WTL"),
	
	/// TODO.
	WTM = Self::letters_to_token(b"WTM"),
	
	/// TODO.
	WTN = Self::letters_to_token(b"WTN"),
	
	/// TODO.
	WTO = Self::letters_to_token(b"WTO"),
	
	/// TODO.
	WTP = Self::letters_to_token(b"WTP"),
	
	/// TODO.
	WTQ = Self::letters_to_token(b"WTQ"),
	
	/// TODO.
	WTR = Self::letters_to_token(b"WTR"),
	
	/// TODO.
	WTS = Self::letters_to_token(b"WTS"),
	
	/// TODO.
	WTT = Self::letters_to_token(b"WTT"),
	
	/// TODO.
	WTU = Self::letters_to_token(b"WTU"),
	
	/// TODO.
	WTV = Self::letters_to_token(b"WTV"),
	
	/// TODO.
	WTW = Self::letters_to_token(b"WTW"),
	
	/// TODO.
	WTX = Self::letters_to_token(b"WTX"),
	
	/// TODO.
	WTY = Self::letters_to_token(b"WTY"),
	
	/// TODO.
	WTZ = Self::letters_to_token(b"WTZ"),
	
	/// TODO.
	WUA = Self::letters_to_token(b"WUA"),
	
	/// TODO.
	WUB = Self::letters_to_token(b"WUB"),
	
	/// TODO.
	WUC = Self::letters_to_token(b"WUC"),
	
	/// TODO.
	WUD = Self::letters_to_token(b"WUD"),
	
	/// TODO.
	WUE = Self::letters_to_token(b"WUE"),
	
	/// TODO.
	WUF = Self::letters_to_token(b"WUF"),
	
	/// TODO.
	WUG = Self::letters_to_token(b"WUG"),
	
	/// TODO.
	WUH = Self::letters_to_token(b"WUH"),
	
	/// TODO.
	WUI = Self::letters_to_token(b"WUI"),
	
	/// TODO.
	WUJ = Self::letters_to_token(b"WUJ"),
	
	/// TODO.
	WUK = Self::letters_to_token(b"WUK"),
	
	/// TODO.
	WUL = Self::letters_to_token(b"WUL"),
	
	/// TODO.
	WUM = Self::letters_to_token(b"WUM"),
	
	/// TODO.
	WUN = Self::letters_to_token(b"WUN"),
	
	/// TODO.
	WUO = Self::letters_to_token(b"WUO"),
	
	/// TODO.
	WUP = Self::letters_to_token(b"WUP"),
	
	/// TODO.
	WUQ = Self::letters_to_token(b"WUQ"),
	
	/// TODO.
	WUR = Self::letters_to_token(b"WUR"),
	
	/// TODO.
	WUS = Self::letters_to_token(b"WUS"),
	
	/// TODO.
	WUT = Self::letters_to_token(b"WUT"),
	
	/// TODO.
	WUU = Self::letters_to_token(b"WUU"),
	
	/// TODO.
	WUV = Self::letters_to_token(b"WUV"),
	
	/// TODO.
	WUW = Self::letters_to_token(b"WUW"),
	
	/// TODO.
	WUX = Self::letters_to_token(b"WUX"),
	
	/// TODO.
	WUY = Self::letters_to_token(b"WUY"),
	
	/// TODO.
	WUZ = Self::letters_to_token(b"WUZ"),
	
	/// TODO.
	WVA = Self::letters_to_token(b"WVA"),
	
	/// TODO.
	WVB = Self::letters_to_token(b"WVB"),
	
	/// TODO.
	WVC = Self::letters_to_token(b"WVC"),
	
	/// TODO.
	WVD = Self::letters_to_token(b"WVD"),
	
	/// TODO.
	WVE = Self::letters_to_token(b"WVE"),
	
	/// TODO.
	WVF = Self::letters_to_token(b"WVF"),
	
	/// TODO.
	WVG = Self::letters_to_token(b"WVG"),
	
	/// TODO.
	WVH = Self::letters_to_token(b"WVH"),
	
	/// TODO.
	WVI = Self::letters_to_token(b"WVI"),
	
	/// TODO.
	WVJ = Self::letters_to_token(b"WVJ"),
	
	/// TODO.
	WVK = Self::letters_to_token(b"WVK"),
	
	/// TODO.
	WVL = Self::letters_to_token(b"WVL"),
	
	/// TODO.
	WVM = Self::letters_to_token(b"WVM"),
	
	/// TODO.
	WVN = Self::letters_to_token(b"WVN"),
	
	/// TODO.
	WVO = Self::letters_to_token(b"WVO"),
	
	/// TODO.
	WVP = Self::letters_to_token(b"WVP"),
	
	/// TODO.
	WVQ = Self::letters_to_token(b"WVQ"),
	
	/// TODO.
	WVR = Self::letters_to_token(b"WVR"),
	
	/// TODO.
	WVS = Self::letters_to_token(b"WVS"),
	
	/// TODO.
	WVT = Self::letters_to_token(b"WVT"),
	
	/// TODO.
	WVU = Self::letters_to_token(b"WVU"),
	
	/// TODO.
	WVV = Self::letters_to_token(b"WVV"),
	
	/// TODO.
	WVW = Self::letters_to_token(b"WVW"),
	
	/// TODO.
	WVX = Self::letters_to_token(b"WVX"),
	
	/// TODO.
	WVY = Self::letters_to_token(b"WVY"),
	
	/// TODO.
	WVZ = Self::letters_to_token(b"WVZ"),
	
	/// TODO.
	WWA = Self::letters_to_token(b"WWA"),
	
	/// TODO.
	WWB = Self::letters_to_token(b"WWB"),
	
	/// TODO.
	WWC = Self::letters_to_token(b"WWC"),
	
	/// TODO.
	WWD = Self::letters_to_token(b"WWD"),
	
	/// TODO.
	WWE = Self::letters_to_token(b"WWE"),
	
	/// TODO.
	WWF = Self::letters_to_token(b"WWF"),
	
	/// TODO.
	WWG = Self::letters_to_token(b"WWG"),
	
	/// TODO.
	WWH = Self::letters_to_token(b"WWH"),
	
	/// TODO.
	WWI = Self::letters_to_token(b"WWI"),
	
	/// TODO.
	WWJ = Self::letters_to_token(b"WWJ"),
	
	/// TODO.
	WWK = Self::letters_to_token(b"WWK"),
	
	/// TODO.
	WWL = Self::letters_to_token(b"WWL"),
	
	/// TODO.
	WWM = Self::letters_to_token(b"WWM"),
	
	/// TODO.
	WWN = Self::letters_to_token(b"WWN"),
	
	/// TODO.
	WWO = Self::letters_to_token(b"WWO"),
	
	/// TODO.
	WWP = Self::letters_to_token(b"WWP"),
	
	/// TODO.
	WWQ = Self::letters_to_token(b"WWQ"),
	
	/// TODO.
	WWR = Self::letters_to_token(b"WWR"),
	
	/// TODO.
	WWS = Self::letters_to_token(b"WWS"),
	
	/// TODO.
	WWT = Self::letters_to_token(b"WWT"),
	
	/// TODO.
	WWU = Self::letters_to_token(b"WWU"),
	
	/// TODO.
	WWV = Self::letters_to_token(b"WWV"),
	
	/// TODO.
	WWW = Self::letters_to_token(b"WWW"),
	
	/// TODO.
	WWX = Self::letters_to_token(b"WWX"),
	
	/// TODO.
	WWY = Self::letters_to_token(b"WWY"),
	
	/// TODO.
	WWZ = Self::letters_to_token(b"WWZ"),
	
	/// TODO.
	WXA = Self::letters_to_token(b"WXA"),
	
	/// TODO.
	WXB = Self::letters_to_token(b"WXB"),
	
	/// TODO.
	WXC = Self::letters_to_token(b"WXC"),
	
	/// TODO.
	WXD = Self::letters_to_token(b"WXD"),
	
	/// TODO.
	WXE = Self::letters_to_token(b"WXE"),
	
	/// TODO.
	WXF = Self::letters_to_token(b"WXF"),
	
	/// TODO.
	WXG = Self::letters_to_token(b"WXG"),
	
	/// TODO.
	WXH = Self::letters_to_token(b"WXH"),
	
	/// TODO.
	WXI = Self::letters_to_token(b"WXI"),
	
	/// TODO.
	WXJ = Self::letters_to_token(b"WXJ"),
	
	/// TODO.
	WXK = Self::letters_to_token(b"WXK"),
	
	/// TODO.
	WXL = Self::letters_to_token(b"WXL"),
	
	/// TODO.
	WXM = Self::letters_to_token(b"WXM"),
	
	/// TODO.
	WXN = Self::letters_to_token(b"WXN"),
	
	/// TODO.
	WXO = Self::letters_to_token(b"WXO"),
	
	/// TODO.
	WXP = Self::letters_to_token(b"WXP"),
	
	/// TODO.
	WXQ = Self::letters_to_token(b"WXQ"),
	
	/// TODO.
	WXR = Self::letters_to_token(b"WXR"),
	
	/// TODO.
	WXS = Self::letters_to_token(b"WXS"),
	
	/// TODO.
	WXT = Self::letters_to_token(b"WXT"),
	
	/// TODO.
	WXU = Self::letters_to_token(b"WXU"),
	
	/// TODO.
	WXV = Self::letters_to_token(b"WXV"),
	
	/// TODO.
	WXW = Self::letters_to_token(b"WXW"),
	
	/// TODO.
	WXX = Self::letters_to_token(b"WXX"),
	
	/// TODO.
	WXY = Self::letters_to_token(b"WXY"),
	
	/// TODO.
	WXZ = Self::letters_to_token(b"WXZ"),
	
	/// TODO.
	WYA = Self::letters_to_token(b"WYA"),
	
	/// TODO.
	WYB = Self::letters_to_token(b"WYB"),
	
	/// TODO.
	WYC = Self::letters_to_token(b"WYC"),
	
	/// TODO.
	WYD = Self::letters_to_token(b"WYD"),
	
	/// TODO.
	WYE = Self::letters_to_token(b"WYE"),
	
	/// TODO.
	WYF = Self::letters_to_token(b"WYF"),
	
	/// TODO.
	WYG = Self::letters_to_token(b"WYG"),
	
	/// TODO.
	WYH = Self::letters_to_token(b"WYH"),
	
	/// TODO.
	WYI = Self::letters_to_token(b"WYI"),
	
	/// TODO.
	WYJ = Self::letters_to_token(b"WYJ"),
	
	/// TODO.
	WYK = Self::letters_to_token(b"WYK"),
	
	/// TODO.
	WYL = Self::letters_to_token(b"WYL"),
	
	/// TODO.
	WYM = Self::letters_to_token(b"WYM"),
	
	/// TODO.
	WYN = Self::letters_to_token(b"WYN"),
	
	/// TODO.
	WYO = Self::letters_to_token(b"WYO"),
	
	/// TODO.
	WYP = Self::letters_to_token(b"WYP"),
	
	/// TODO.
	WYQ = Self::letters_to_token(b"WYQ"),
	
	/// TODO.
	WYR = Self::letters_to_token(b"WYR"),
	
	/// TODO.
	WYS = Self::letters_to_token(b"WYS"),
	
	/// TODO.
	WYT = Self::letters_to_token(b"WYT"),
	
	/// TODO.
	WYU = Self::letters_to_token(b"WYU"),
	
	/// TODO.
	WYV = Self::letters_to_token(b"WYV"),
	
	/// TODO.
	WYW = Self::letters_to_token(b"WYW"),
	
	/// TODO.
	WYX = Self::letters_to_token(b"WYX"),
	
	/// TODO.
	WYY = Self::letters_to_token(b"WYY"),
	
	/// TODO.
	WYZ = Self::letters_to_token(b"WYZ"),
	
	/// TODO.
	WZA = Self::letters_to_token(b"WZA"),
	
	/// TODO.
	WZB = Self::letters_to_token(b"WZB"),
	
	/// TODO.
	WZC = Self::letters_to_token(b"WZC"),
	
	/// TODO.
	WZD = Self::letters_to_token(b"WZD"),
	
	/// TODO.
	WZE = Self::letters_to_token(b"WZE"),
	
	/// TODO.
	WZF = Self::letters_to_token(b"WZF"),
	
	/// TODO.
	WZG = Self::letters_to_token(b"WZG"),
	
	/// TODO.
	WZH = Self::letters_to_token(b"WZH"),
	
	/// TODO.
	WZI = Self::letters_to_token(b"WZI"),
	
	/// TODO.
	WZJ = Self::letters_to_token(b"WZJ"),
	
	/// TODO.
	WZK = Self::letters_to_token(b"WZK"),
	
	/// TODO.
	WZL = Self::letters_to_token(b"WZL"),
	
	/// TODO.
	WZM = Self::letters_to_token(b"WZM"),
	
	/// TODO.
	WZN = Self::letters_to_token(b"WZN"),
	
	/// TODO.
	WZO = Self::letters_to_token(b"WZO"),
	
	/// TODO.
	WZP = Self::letters_to_token(b"WZP"),
	
	/// TODO.
	WZQ = Self::letters_to_token(b"WZQ"),
	
	/// TODO.
	WZR = Self::letters_to_token(b"WZR"),
	
	/// TODO.
	WZS = Self::letters_to_token(b"WZS"),
	
	/// TODO.
	WZT = Self::letters_to_token(b"WZT"),
	
	/// TODO.
	WZU = Self::letters_to_token(b"WZU"),
	
	/// TODO.
	WZV = Self::letters_to_token(b"WZV"),
	
	/// TODO.
	WZW = Self::letters_to_token(b"WZW"),
	
	/// TODO.
	WZX = Self::letters_to_token(b"WZX"),
	
	/// TODO.
	WZY = Self::letters_to_token(b"WZY"),
	
	/// TODO.
	WZZ = Self::letters_to_token(b"WZZ"),
	
	/// User assigned.
	XAA = Self::letters_to_token(b"XAA"),
	
	/// User assigned.
	XAB = Self::letters_to_token(b"XAB"),
	
	/// User assigned.
	XAC = Self::letters_to_token(b"XAC"),
	
	/// User assigned.
	XAD = Self::letters_to_token(b"XAD"),
	
	/// User assigned.
	XAE = Self::letters_to_token(b"XAE"),
	
	/// User assigned.
	XAF = Self::letters_to_token(b"XAF"),
	
	/// User assigned.
	XAG = Self::letters_to_token(b"XAG"),
	
	/// User assigned.
	XAH = Self::letters_to_token(b"XAH"),
	
	/// User assigned.
	XAI = Self::letters_to_token(b"XAI"),
	
	/// User assigned.
	XAJ = Self::letters_to_token(b"XAJ"),
	
	/// User assigned.
	XAK = Self::letters_to_token(b"XAK"),
	
	/// User assigned.
	XAL = Self::letters_to_token(b"XAL"),
	
	/// User assigned.
	XAM = Self::letters_to_token(b"XAM"),
	
	/// User assigned.
	XAN = Self::letters_to_token(b"XAN"),
	
	/// User assigned.
	XAO = Self::letters_to_token(b"XAO"),
	
	/// User assigned.
	XAP = Self::letters_to_token(b"XAP"),
	
	/// User assigned.
	XAQ = Self::letters_to_token(b"XAQ"),
	
	/// User assigned.
	XAR = Self::letters_to_token(b"XAR"),
	
	/// User assigned.
	XAS = Self::letters_to_token(b"XAS"),
	
	/// User assigned.
	XAT = Self::letters_to_token(b"XAT"),
	
	/// User assigned.
	XAU = Self::letters_to_token(b"XAU"),
	
	/// User assigned.
	XAV = Self::letters_to_token(b"XAV"),
	
	/// User assigned.
	XAW = Self::letters_to_token(b"XAW"),
	
	/// User assigned.
	XAX = Self::letters_to_token(b"XAX"),
	
	/// User assigned.
	XAY = Self::letters_to_token(b"XAY"),
	
	/// User assigned.
	XAZ = Self::letters_to_token(b"XAZ"),
	
	/// User assigned.
	XBA = Self::letters_to_token(b"XBA"),
	
	/// User assigned.
	XBB = Self::letters_to_token(b"XBB"),
	
	/// User assigned.
	XBC = Self::letters_to_token(b"XBC"),
	
	/// User assigned.
	XBD = Self::letters_to_token(b"XBD"),
	
	/// User assigned.
	XBE = Self::letters_to_token(b"XBE"),
	
	/// User assigned.
	XBF = Self::letters_to_token(b"XBF"),
	
	/// User assigned.
	XBG = Self::letters_to_token(b"XBG"),
	
	/// User assigned.
	XBH = Self::letters_to_token(b"XBH"),
	
	/// User assigned.
	XBI = Self::letters_to_token(b"XBI"),
	
	/// User assigned.
	XBJ = Self::letters_to_token(b"XBJ"),
	
	/// User assigned.
	XBK = Self::letters_to_token(b"XBK"),
	
	/// User assigned.
	XBL = Self::letters_to_token(b"XBL"),
	
	/// User assigned.
	XBM = Self::letters_to_token(b"XBM"),
	
	/// User assigned.
	XBN = Self::letters_to_token(b"XBN"),
	
	/// User assigned.
	XBO = Self::letters_to_token(b"XBO"),
	
	/// User assigned.
	XBP = Self::letters_to_token(b"XBP"),
	
	/// User assigned.
	XBQ = Self::letters_to_token(b"XBQ"),
	
	/// User assigned.
	XBR = Self::letters_to_token(b"XBR"),
	
	/// User assigned.
	XBS = Self::letters_to_token(b"XBS"),
	
	/// User assigned.
	XBT = Self::letters_to_token(b"XBT"),
	
	/// User assigned.
	XBU = Self::letters_to_token(b"XBU"),
	
	/// User assigned.
	XBV = Self::letters_to_token(b"XBV"),
	
	/// User assigned.
	XBW = Self::letters_to_token(b"XBW"),
	
	/// User assigned.
	XBX = Self::letters_to_token(b"XBX"),
	
	/// User assigned.
	XBY = Self::letters_to_token(b"XBY"),
	
	/// User assigned.
	XBZ = Self::letters_to_token(b"XBZ"),
	
	/// User assigned.
	XCA = Self::letters_to_token(b"XCA"),
	
	/// User assigned.
	XCB = Self::letters_to_token(b"XCB"),
	
	/// User assigned.
	XCC = Self::letters_to_token(b"XCC"),
	
	/// User assigned.
	XCD = Self::letters_to_token(b"XCD"),
	
	/// User assigned.
	XCE = Self::letters_to_token(b"XCE"),
	
	/// User assigned.
	XCF = Self::letters_to_token(b"XCF"),
	
	/// User assigned.
	XCG = Self::letters_to_token(b"XCG"),
	
	/// User assigned.
	XCH = Self::letters_to_token(b"XCH"),
	
	/// User assigned.
	XCI = Self::letters_to_token(b"XCI"),
	
	/// User assigned.
	XCJ = Self::letters_to_token(b"XCJ"),
	
	/// User assigned.
	XCK = Self::letters_to_token(b"XCK"),
	
	/// User assigned.
	XCL = Self::letters_to_token(b"XCL"),
	
	/// User assigned.
	XCM = Self::letters_to_token(b"XCM"),
	
	/// User assigned.
	XCN = Self::letters_to_token(b"XCN"),
	
	/// User assigned.
	XCO = Self::letters_to_token(b"XCO"),
	
	/// User assigned.
	XCP = Self::letters_to_token(b"XCP"),
	
	/// User assigned.
	XCQ = Self::letters_to_token(b"XCQ"),
	
	/// User assigned.
	XCR = Self::letters_to_token(b"XCR"),
	
	/// User assigned.
	XCS = Self::letters_to_token(b"XCS"),
	
	/// User assigned.
	XCT = Self::letters_to_token(b"XCT"),
	
	/// User assigned.
	XCU = Self::letters_to_token(b"XCU"),
	
	/// User assigned.
	XCV = Self::letters_to_token(b"XCV"),
	
	/// User assigned.
	XCW = Self::letters_to_token(b"XCW"),
	
	/// User assigned.
	XCX = Self::letters_to_token(b"XCX"),
	
	/// User assigned.
	XCY = Self::letters_to_token(b"XCY"),
	
	/// User assigned.
	XCZ = Self::letters_to_token(b"XCZ"),
	
	/// User assigned.
	XDA = Self::letters_to_token(b"XDA"),
	
	/// User assigned.
	XDB = Self::letters_to_token(b"XDB"),
	
	/// User assigned.
	XDC = Self::letters_to_token(b"XDC"),
	
	/// User assigned.
	XDD = Self::letters_to_token(b"XDD"),
	
	/// User assigned.
	XDE = Self::letters_to_token(b"XDE"),
	
	/// User assigned.
	XDF = Self::letters_to_token(b"XDF"),
	
	/// User assigned.
	XDG = Self::letters_to_token(b"XDG"),
	
	/// User assigned.
	XDH = Self::letters_to_token(b"XDH"),
	
	/// User assigned.
	XDI = Self::letters_to_token(b"XDI"),
	
	/// User assigned.
	XDJ = Self::letters_to_token(b"XDJ"),
	
	/// User assigned.
	XDK = Self::letters_to_token(b"XDK"),
	
	/// User assigned.
	XDL = Self::letters_to_token(b"XDL"),
	
	/// User assigned.
	XDM = Self::letters_to_token(b"XDM"),
	
	/// User assigned.
	XDN = Self::letters_to_token(b"XDN"),
	
	/// User assigned.
	XDO = Self::letters_to_token(b"XDO"),
	
	/// User assigned.
	XDP = Self::letters_to_token(b"XDP"),
	
	/// User assigned.
	XDQ = Self::letters_to_token(b"XDQ"),
	
	/// User assigned.
	XDR = Self::letters_to_token(b"XDR"),
	
	/// User assigned.
	XDS = Self::letters_to_token(b"XDS"),
	
	/// User assigned.
	XDT = Self::letters_to_token(b"XDT"),
	
	/// User assigned.
	XDU = Self::letters_to_token(b"XDU"),
	
	/// User assigned.
	XDV = Self::letters_to_token(b"XDV"),
	
	/// User assigned.
	XDW = Self::letters_to_token(b"XDW"),
	
	/// User assigned.
	XDX = Self::letters_to_token(b"XDX"),
	
	/// User assigned.
	XDY = Self::letters_to_token(b"XDY"),
	
	/// User assigned.
	XDZ = Self::letters_to_token(b"XDZ"),
	
	/// User assigned.
	XEA = Self::letters_to_token(b"XEA"),
	
	/// User assigned.
	XEB = Self::letters_to_token(b"XEB"),
	
	/// User assigned.
	XEC = Self::letters_to_token(b"XEC"),
	
	/// User assigned.
	XED = Self::letters_to_token(b"XED"),
	
	/// User assigned.
	XEE = Self::letters_to_token(b"XEE"),
	
	/// User assigned.
	XEF = Self::letters_to_token(b"XEF"),
	
	/// User assigned.
	XEG = Self::letters_to_token(b"XEG"),
	
	/// User assigned.
	XEH = Self::letters_to_token(b"XEH"),
	
	/// User assigned.
	XEI = Self::letters_to_token(b"XEI"),
	
	/// User assigned.
	XEJ = Self::letters_to_token(b"XEJ"),
	
	/// User assigned.
	XEK = Self::letters_to_token(b"XEK"),
	
	/// User assigned.
	XEL = Self::letters_to_token(b"XEL"),
	
	/// User assigned.
	XEM = Self::letters_to_token(b"XEM"),
	
	/// User assigned.
	XEN = Self::letters_to_token(b"XEN"),
	
	/// User assigned.
	XEO = Self::letters_to_token(b"XEO"),
	
	/// User assigned.
	XEP = Self::letters_to_token(b"XEP"),
	
	/// User assigned.
	XEQ = Self::letters_to_token(b"XEQ"),
	
	/// User assigned.
	XER = Self::letters_to_token(b"XER"),
	
	/// User assigned.
	XES = Self::letters_to_token(b"XES"),
	
	/// User assigned.
	XET = Self::letters_to_token(b"XET"),
	
	/// User assigned.
	XEU = Self::letters_to_token(b"XEU"),
	
	/// User assigned.
	XEV = Self::letters_to_token(b"XEV"),
	
	/// User assigned.
	XEW = Self::letters_to_token(b"XEW"),
	
	/// User assigned.
	XEX = Self::letters_to_token(b"XEX"),
	
	/// User assigned.
	XEY = Self::letters_to_token(b"XEY"),
	
	/// User assigned.
	XEZ = Self::letters_to_token(b"XEZ"),
	
	/// User assigned.
	XFA = Self::letters_to_token(b"XFA"),
	
	/// User assigned.
	XFB = Self::letters_to_token(b"XFB"),
	
	/// User assigned.
	XFC = Self::letters_to_token(b"XFC"),
	
	/// User assigned.
	XFD = Self::letters_to_token(b"XFD"),
	
	/// User assigned.
	XFE = Self::letters_to_token(b"XFE"),
	
	/// User assigned.
	XFF = Self::letters_to_token(b"XFF"),
	
	/// User assigned.
	XFG = Self::letters_to_token(b"XFG"),
	
	/// User assigned.
	XFH = Self::letters_to_token(b"XFH"),
	
	/// User assigned.
	XFI = Self::letters_to_token(b"XFI"),
	
	/// User assigned.
	XFJ = Self::letters_to_token(b"XFJ"),
	
	/// User assigned.
	XFK = Self::letters_to_token(b"XFK"),
	
	/// User assigned.
	XFL = Self::letters_to_token(b"XFL"),
	
	/// User assigned.
	XFM = Self::letters_to_token(b"XFM"),
	
	/// User assigned.
	XFN = Self::letters_to_token(b"XFN"),
	
	/// User assigned.
	XFO = Self::letters_to_token(b"XFO"),
	
	/// User assigned.
	XFP = Self::letters_to_token(b"XFP"),
	
	/// User assigned.
	XFQ = Self::letters_to_token(b"XFQ"),
	
	/// User assigned.
	XFR = Self::letters_to_token(b"XFR"),
	
	/// User assigned.
	XFS = Self::letters_to_token(b"XFS"),
	
	/// User assigned.
	XFT = Self::letters_to_token(b"XFT"),
	
	/// User assigned.
	XFU = Self::letters_to_token(b"XFU"),
	
	/// User assigned.
	XFV = Self::letters_to_token(b"XFV"),
	
	/// User assigned.
	XFW = Self::letters_to_token(b"XFW"),
	
	/// User assigned.
	XFX = Self::letters_to_token(b"XFX"),
	
	/// User assigned.
	XFY = Self::letters_to_token(b"XFY"),
	
	/// User assigned.
	XFZ = Self::letters_to_token(b"XFZ"),
	
	/// User assigned.
	XGA = Self::letters_to_token(b"XGA"),
	
	/// User assigned.
	XGB = Self::letters_to_token(b"XGB"),
	
	/// User assigned.
	XGC = Self::letters_to_token(b"XGC"),
	
	/// User assigned.
	XGD = Self::letters_to_token(b"XGD"),
	
	/// User assigned.
	XGE = Self::letters_to_token(b"XGE"),
	
	/// User assigned.
	XGF = Self::letters_to_token(b"XGF"),
	
	/// User assigned.
	XGG = Self::letters_to_token(b"XGG"),
	
	/// User assigned.
	XGH = Self::letters_to_token(b"XGH"),
	
	/// User assigned.
	XGI = Self::letters_to_token(b"XGI"),
	
	/// User assigned.
	XGJ = Self::letters_to_token(b"XGJ"),
	
	/// User assigned.
	XGK = Self::letters_to_token(b"XGK"),
	
	/// User assigned.
	XGL = Self::letters_to_token(b"XGL"),
	
	/// User assigned.
	XGM = Self::letters_to_token(b"XGM"),
	
	/// User assigned.
	XGN = Self::letters_to_token(b"XGN"),
	
	/// User assigned.
	XGO = Self::letters_to_token(b"XGO"),
	
	/// User assigned.
	XGP = Self::letters_to_token(b"XGP"),
	
	/// User assigned.
	XGQ = Self::letters_to_token(b"XGQ"),
	
	/// User assigned.
	XGR = Self::letters_to_token(b"XGR"),
	
	/// User assigned.
	XGS = Self::letters_to_token(b"XGS"),
	
	/// User assigned.
	XGT = Self::letters_to_token(b"XGT"),
	
	/// User assigned.
	XGU = Self::letters_to_token(b"XGU"),
	
	/// User assigned.
	XGV = Self::letters_to_token(b"XGV"),
	
	/// User assigned.
	XGW = Self::letters_to_token(b"XGW"),
	
	/// User assigned.
	XGX = Self::letters_to_token(b"XGX"),
	
	/// User assigned.
	XGY = Self::letters_to_token(b"XGY"),
	
	/// User assigned.
	XGZ = Self::letters_to_token(b"XGZ"),
	
	/// User assigned.
	XHA = Self::letters_to_token(b"XHA"),
	
	/// User assigned.
	XHB = Self::letters_to_token(b"XHB"),
	
	/// User assigned.
	XHC = Self::letters_to_token(b"XHC"),
	
	/// User assigned.
	XHD = Self::letters_to_token(b"XHD"),
	
	/// User assigned.
	XHE = Self::letters_to_token(b"XHE"),
	
	/// User assigned.
	XHF = Self::letters_to_token(b"XHF"),
	
	/// User assigned.
	XHG = Self::letters_to_token(b"XHG"),
	
	/// User assigned.
	XHH = Self::letters_to_token(b"XHH"),
	
	/// User assigned.
	XHI = Self::letters_to_token(b"XHI"),
	
	/// User assigned.
	XHJ = Self::letters_to_token(b"XHJ"),
	
	/// User assigned.
	XHK = Self::letters_to_token(b"XHK"),
	
	/// User assigned.
	XHL = Self::letters_to_token(b"XHL"),
	
	/// User assigned.
	XHM = Self::letters_to_token(b"XHM"),
	
	/// User assigned.
	XHN = Self::letters_to_token(b"XHN"),
	
	/// User assigned.
	XHO = Self::letters_to_token(b"XHO"),
	
	/// User assigned.
	XHP = Self::letters_to_token(b"XHP"),
	
	/// User assigned.
	XHQ = Self::letters_to_token(b"XHQ"),
	
	/// User assigned.
	XHR = Self::letters_to_token(b"XHR"),
	
	/// User assigned.
	XHS = Self::letters_to_token(b"XHS"),
	
	/// User assigned.
	XHT = Self::letters_to_token(b"XHT"),
	
	/// User assigned.
	XHU = Self::letters_to_token(b"XHU"),
	
	/// User assigned.
	XHV = Self::letters_to_token(b"XHV"),
	
	/// User assigned.
	XHW = Self::letters_to_token(b"XHW"),
	
	/// User assigned.
	XHX = Self::letters_to_token(b"XHX"),
	
	/// User assigned.
	XHY = Self::letters_to_token(b"XHY"),
	
	/// User assigned.
	XHZ = Self::letters_to_token(b"XHZ"),
	
	/// User assigned.
	XIA = Self::letters_to_token(b"XIA"),
	
	/// User assigned.
	XIB = Self::letters_to_token(b"XIB"),
	
	/// User assigned.
	XIC = Self::letters_to_token(b"XIC"),
	
	/// User assigned.
	XID = Self::letters_to_token(b"XID"),
	
	/// User assigned.
	XIE = Self::letters_to_token(b"XIE"),
	
	/// User assigned.
	XIF = Self::letters_to_token(b"XIF"),
	
	/// User assigned.
	XIG = Self::letters_to_token(b"XIG"),
	
	/// User assigned.
	XIH = Self::letters_to_token(b"XIH"),
	
	/// User assigned.
	XII = Self::letters_to_token(b"XII"),
	
	/// User assigned.
	XIJ = Self::letters_to_token(b"XIJ"),
	
	/// User assigned.
	XIK = Self::letters_to_token(b"XIK"),
	
	/// User assigned.
	XIL = Self::letters_to_token(b"XIL"),
	
	/// User assigned.
	XIM = Self::letters_to_token(b"XIM"),
	
	/// User assigned.
	XIN = Self::letters_to_token(b"XIN"),
	
	/// User assigned.
	XIO = Self::letters_to_token(b"XIO"),
	
	/// User assigned.
	XIP = Self::letters_to_token(b"XIP"),
	
	/// User assigned.
	XIQ = Self::letters_to_token(b"XIQ"),
	
	/// User assigned.
	XIR = Self::letters_to_token(b"XIR"),
	
	/// User assigned.
	XIS = Self::letters_to_token(b"XIS"),
	
	/// User assigned.
	XIT = Self::letters_to_token(b"XIT"),
	
	/// User assigned.
	XIU = Self::letters_to_token(b"XIU"),
	
	/// User assigned.
	XIV = Self::letters_to_token(b"XIV"),
	
	/// User assigned.
	XIW = Self::letters_to_token(b"XIW"),
	
	/// User assigned.
	XIX = Self::letters_to_token(b"XIX"),
	
	/// User assigned.
	XIY = Self::letters_to_token(b"XIY"),
	
	/// User assigned.
	XIZ = Self::letters_to_token(b"XIZ"),
	
	/// User assigned.
	XJA = Self::letters_to_token(b"XJA"),
	
	/// User assigned.
	XJB = Self::letters_to_token(b"XJB"),
	
	/// User assigned.
	XJC = Self::letters_to_token(b"XJC"),
	
	/// User assigned.
	XJD = Self::letters_to_token(b"XJD"),
	
	/// User assigned.
	XJE = Self::letters_to_token(b"XJE"),
	
	/// User assigned.
	XJF = Self::letters_to_token(b"XJF"),
	
	/// User assigned.
	XJG = Self::letters_to_token(b"XJG"),
	
	/// User assigned.
	XJH = Self::letters_to_token(b"XJH"),
	
	/// User assigned.
	XJI = Self::letters_to_token(b"XJI"),
	
	/// User assigned.
	XJJ = Self::letters_to_token(b"XJJ"),
	
	/// User assigned.
	XJK = Self::letters_to_token(b"XJK"),
	
	/// User assigned.
	XJL = Self::letters_to_token(b"XJL"),
	
	/// User assigned.
	XJM = Self::letters_to_token(b"XJM"),
	
	/// User assigned.
	XJN = Self::letters_to_token(b"XJN"),
	
	/// User assigned.
	XJO = Self::letters_to_token(b"XJO"),
	
	/// User assigned.
	XJP = Self::letters_to_token(b"XJP"),
	
	/// User assigned.
	XJQ = Self::letters_to_token(b"XJQ"),
	
	/// User assigned.
	XJR = Self::letters_to_token(b"XJR"),
	
	/// User assigned.
	XJS = Self::letters_to_token(b"XJS"),
	
	/// User assigned.
	XJT = Self::letters_to_token(b"XJT"),
	
	/// User assigned.
	XJU = Self::letters_to_token(b"XJU"),
	
	/// User assigned.
	XJV = Self::letters_to_token(b"XJV"),
	
	/// User assigned.
	XJW = Self::letters_to_token(b"XJW"),
	
	/// User assigned.
	XJX = Self::letters_to_token(b"XJX"),
	
	/// User assigned.
	XJY = Self::letters_to_token(b"XJY"),
	
	/// User assigned.
	XJZ = Self::letters_to_token(b"XJZ"),
	
	/// User assigned.
	XKA = Self::letters_to_token(b"XKA"),
	
	/// User assigned.
	XKB = Self::letters_to_token(b"XKB"),
	
	/// User assigned.
	XKC = Self::letters_to_token(b"XKC"),
	
	/// User assigned.
	XKD = Self::letters_to_token(b"XKD"),
	
	/// User assigned.
	XKE = Self::letters_to_token(b"XKE"),
	
	/// User assigned.
	XKF = Self::letters_to_token(b"XKF"),
	
	/// User assigned.
	XKG = Self::letters_to_token(b"XKG"),
	
	/// User assigned.
	XKH = Self::letters_to_token(b"XKH"),
	
	/// User assigned.
	XKI = Self::letters_to_token(b"XKI"),
	
	/// User assigned.
	XKJ = Self::letters_to_token(b"XKJ"),
	
	/// User assigned.
	XKK = Self::letters_to_token(b"XKK"),
	
	/// User assigned.
	XKL = Self::letters_to_token(b"XKL"),
	
	/// User assigned.
	XKM = Self::letters_to_token(b"XKM"),
	
	/// User assigned.
	XKN = Self::letters_to_token(b"XKN"),
	
	/// User assigned.
	XKO = Self::letters_to_token(b"XKO"),
	
	/// User assigned.
	XKP = Self::letters_to_token(b"XKP"),
	
	/// User assigned.
	XKQ = Self::letters_to_token(b"XKQ"),
	
	/// User assigned.
	XKR = Self::letters_to_token(b"XKR"),
	
	/// User assigned.
	XKS = Self::letters_to_token(b"XKS"),
	
	/// User assigned.
	XKT = Self::letters_to_token(b"XKT"),
	
	/// User assigned.
	XKU = Self::letters_to_token(b"XKU"),
	
	/// User assigned.
	XKV = Self::letters_to_token(b"XKV"),
	
	/// User assigned.
	XKW = Self::letters_to_token(b"XKW"),
	
	/// User assigned.
	XKX = Self::letters_to_token(b"XKX"),
	
	/// User assigned.
	XKY = Self::letters_to_token(b"XKY"),
	
	/// User assigned.
	XKZ = Self::letters_to_token(b"XKZ"),
	
	/// User assigned.
	XLA = Self::letters_to_token(b"XLA"),
	
	/// User assigned.
	XLB = Self::letters_to_token(b"XLB"),
	
	/// User assigned.
	XLC = Self::letters_to_token(b"XLC"),
	
	/// User assigned.
	XLD = Self::letters_to_token(b"XLD"),
	
	/// User assigned.
	XLE = Self::letters_to_token(b"XLE"),
	
	/// User assigned.
	XLF = Self::letters_to_token(b"XLF"),
	
	/// User assigned.
	XLG = Self::letters_to_token(b"XLG"),
	
	/// User assigned.
	XLH = Self::letters_to_token(b"XLH"),
	
	/// User assigned.
	XLI = Self::letters_to_token(b"XLI"),
	
	/// User assigned.
	XLJ = Self::letters_to_token(b"XLJ"),
	
	/// User assigned.
	XLK = Self::letters_to_token(b"XLK"),
	
	/// User assigned.
	XLL = Self::letters_to_token(b"XLL"),
	
	/// User assigned.
	XLM = Self::letters_to_token(b"XLM"),
	
	/// User assigned.
	XLN = Self::letters_to_token(b"XLN"),
	
	/// User assigned.
	XLO = Self::letters_to_token(b"XLO"),
	
	/// User assigned.
	XLP = Self::letters_to_token(b"XLP"),
	
	/// User assigned.
	XLQ = Self::letters_to_token(b"XLQ"),
	
	/// User assigned.
	XLR = Self::letters_to_token(b"XLR"),
	
	/// User assigned.
	XLS = Self::letters_to_token(b"XLS"),
	
	/// User assigned.
	XLT = Self::letters_to_token(b"XLT"),
	
	/// User assigned.
	XLU = Self::letters_to_token(b"XLU"),
	
	/// User assigned.
	XLV = Self::letters_to_token(b"XLV"),
	
	/// User assigned.
	XLW = Self::letters_to_token(b"XLW"),
	
	/// User assigned.
	XLX = Self::letters_to_token(b"XLX"),
	
	/// User assigned.
	XLY = Self::letters_to_token(b"XLY"),
	
	/// User assigned.
	XLZ = Self::letters_to_token(b"XLZ"),
	
	/// User assigned.
	XMA = Self::letters_to_token(b"XMA"),
	
	/// User assigned.
	XMB = Self::letters_to_token(b"XMB"),
	
	/// User assigned.
	XMC = Self::letters_to_token(b"XMC"),
	
	/// User assigned.
	XMD = Self::letters_to_token(b"XMD"),
	
	/// User assigned.
	XME = Self::letters_to_token(b"XME"),
	
	/// User assigned.
	XMF = Self::letters_to_token(b"XMF"),
	
	/// User assigned.
	XMG = Self::letters_to_token(b"XMG"),
	
	/// User assigned.
	XMH = Self::letters_to_token(b"XMH"),
	
	/// User assigned.
	XMI = Self::letters_to_token(b"XMI"),
	
	/// User assigned.
	XMJ = Self::letters_to_token(b"XMJ"),
	
	/// User assigned.
	XMK = Self::letters_to_token(b"XMK"),
	
	/// User assigned.
	XML = Self::letters_to_token(b"XML"),
	
	/// User assigned.
	XMM = Self::letters_to_token(b"XMM"),
	
	/// User assigned.
	XMN = Self::letters_to_token(b"XMN"),
	
	/// User assigned.
	XMO = Self::letters_to_token(b"XMO"),
	
	/// User assigned.
	XMP = Self::letters_to_token(b"XMP"),
	
	/// User assigned.
	XMQ = Self::letters_to_token(b"XMQ"),
	
	/// User assigned.
	XMR = Self::letters_to_token(b"XMR"),
	
	/// User assigned.
	XMS = Self::letters_to_token(b"XMS"),
	
	/// User assigned.
	XMT = Self::letters_to_token(b"XMT"),
	
	/// User assigned.
	XMU = Self::letters_to_token(b"XMU"),
	
	/// User assigned.
	XMV = Self::letters_to_token(b"XMV"),
	
	/// User assigned.
	XMW = Self::letters_to_token(b"XMW"),
	
	/// User assigned.
	XMX = Self::letters_to_token(b"XMX"),
	
	/// User assigned.
	XMY = Self::letters_to_token(b"XMY"),
	
	/// User assigned.
	XMZ = Self::letters_to_token(b"XMZ"),
	
	/// User assigned.
	XNA = Self::letters_to_token(b"XNA"),
	
	/// User assigned.
	XNB = Self::letters_to_token(b"XNB"),
	
	/// User assigned.
	XNC = Self::letters_to_token(b"XNC"),
	
	/// User assigned.
	XND = Self::letters_to_token(b"XND"),
	
	/// User assigned.
	XNE = Self::letters_to_token(b"XNE"),
	
	/// User assigned.
	XNF = Self::letters_to_token(b"XNF"),
	
	/// User assigned.
	XNG = Self::letters_to_token(b"XNG"),
	
	/// User assigned.
	XNH = Self::letters_to_token(b"XNH"),
	
	/// User assigned.
	XNI = Self::letters_to_token(b"XNI"),
	
	/// User assigned.
	XNJ = Self::letters_to_token(b"XNJ"),
	
	/// User assigned.
	XNK = Self::letters_to_token(b"XNK"),
	
	/// User assigned.
	XNL = Self::letters_to_token(b"XNL"),
	
	/// User assigned.
	XNM = Self::letters_to_token(b"XNM"),
	
	/// User assigned.
	XNN = Self::letters_to_token(b"XNN"),
	
	/// User assigned.
	XNO = Self::letters_to_token(b"XNO"),
	
	/// User assigned.
	XNP = Self::letters_to_token(b"XNP"),
	
	/// User assigned.
	XNQ = Self::letters_to_token(b"XNQ"),
	
	/// User assigned.
	XNR = Self::letters_to_token(b"XNR"),
	
	/// User assigned.
	XNS = Self::letters_to_token(b"XNS"),
	
	/// User assigned.
	XNT = Self::letters_to_token(b"XNT"),
	
	/// User assigned.
	XNU = Self::letters_to_token(b"XNU"),
	
	/// User assigned.
	XNV = Self::letters_to_token(b"XNV"),
	
	/// User assigned.
	XNW = Self::letters_to_token(b"XNW"),
	
	/// User assigned.
	XNX = Self::letters_to_token(b"XNX"),
	
	/// User assigned.
	XNY = Self::letters_to_token(b"XNY"),
	
	/// User assigned.
	XNZ = Self::letters_to_token(b"XNZ"),
	
	/// User assigned.
	XOA = Self::letters_to_token(b"XOA"),
	
	/// User assigned.
	XOB = Self::letters_to_token(b"XOB"),
	
	/// User assigned.
	XOC = Self::letters_to_token(b"XOC"),
	
	/// User assigned.
	XOD = Self::letters_to_token(b"XOD"),
	
	/// User assigned.
	XOE = Self::letters_to_token(b"XOE"),
	
	/// User assigned.
	XOF = Self::letters_to_token(b"XOF"),
	
	/// User assigned.
	XOG = Self::letters_to_token(b"XOG"),
	
	/// User assigned.
	XOH = Self::letters_to_token(b"XOH"),
	
	/// User assigned.
	XOI = Self::letters_to_token(b"XOI"),
	
	/// User assigned.
	XOJ = Self::letters_to_token(b"XOJ"),
	
	/// User assigned.
	XOK = Self::letters_to_token(b"XOK"),
	
	/// User assigned.
	XOL = Self::letters_to_token(b"XOL"),
	
	/// User assigned.
	///
	/// Used for the Soverign Miitary Order of Malta.
	XOM = Self::letters_to_token(b"XOM"),
	
	/// User assigned.
	XON = Self::letters_to_token(b"XON"),
	
	/// User assigned.
	XOO = Self::letters_to_token(b"XOO"),
	
	/// User assigned.
	XOP = Self::letters_to_token(b"XOP"),
	
	/// User assigned.
	XOQ = Self::letters_to_token(b"XOQ"),
	
	/// User assigned.
	XOR = Self::letters_to_token(b"XOR"),
	
	/// User assigned.
	XOS = Self::letters_to_token(b"XOS"),
	
	/// User assigned.
	XOT = Self::letters_to_token(b"XOT"),
	
	/// User assigned.
	XOU = Self::letters_to_token(b"XOU"),
	
	/// User assigned.
	XOV = Self::letters_to_token(b"XOV"),
	
	/// User assigned.
	XOW = Self::letters_to_token(b"XOW"),
	
	/// User assigned.
	XOX = Self::letters_to_token(b"XOX"),
	
	/// User assigned.
	XOY = Self::letters_to_token(b"XOY"),
	
	/// User assigned.
	XOZ = Self::letters_to_token(b"XOZ"),
	
	/// User assigned.
	XPA = Self::letters_to_token(b"XPA"),
	
	/// User assigned.
	XPB = Self::letters_to_token(b"XPB"),
	
	/// User assigned.
	XPC = Self::letters_to_token(b"XPC"),
	
	/// User assigned.
	XPD = Self::letters_to_token(b"XPD"),
	
	/// User assigned.
	XPE = Self::letters_to_token(b"XPE"),
	
	/// User assigned.
	XPF = Self::letters_to_token(b"XPF"),
	
	/// User assigned.
	XPG = Self::letters_to_token(b"XPG"),
	
	/// User assigned.
	XPH = Self::letters_to_token(b"XPH"),
	
	/// User assigned.
	XPI = Self::letters_to_token(b"XPI"),
	
	/// User assigned.
	XPJ = Self::letters_to_token(b"XPJ"),
	
	/// User assigned.
	XPK = Self::letters_to_token(b"XPK"),
	
	/// User assigned.
	XPL = Self::letters_to_token(b"XPL"),
	
	/// User assigned.
	XPM = Self::letters_to_token(b"XPM"),
	
	/// User assigned.
	XPN = Self::letters_to_token(b"XPN"),
	
	/// User assigned.
	///
	/// Used for Interpol travel documents.
	XPO = Self::letters_to_token(b"XPO"),
	
	/// User assigned.
	XPP = Self::letters_to_token(b"XPP"),
	
	/// User assigned.
	XPQ = Self::letters_to_token(b"XPQ"),
	
	/// User assigned.
	XPR = Self::letters_to_token(b"XPR"),
	
	/// User assigned.
	XPS = Self::letters_to_token(b"XPS"),
	
	/// User assigned.
	XPT = Self::letters_to_token(b"XPT"),
	
	/// User assigned.
	XPU = Self::letters_to_token(b"XPU"),
	
	/// User assigned.
	XPV = Self::letters_to_token(b"XPV"),
	
	/// User assigned.
	XPW = Self::letters_to_token(b"XPW"),
	
	/// User assigned.
	XPX = Self::letters_to_token(b"XPX"),
	
	/// User assigned.
	XPY = Self::letters_to_token(b"XPY"),
	
	/// User assigned.
	XPZ = Self::letters_to_token(b"XPZ"),
	
	/// User assigned.
	XQA = Self::letters_to_token(b"XQA"),
	
	/// User assigned.
	XQB = Self::letters_to_token(b"XQB"),
	
	/// User assigned.
	XQC = Self::letters_to_token(b"XQC"),
	
	/// User assigned.
	XQD = Self::letters_to_token(b"XQD"),
	
	/// User assigned.
	XQE = Self::letters_to_token(b"XQE"),
	
	/// User assigned.
	XQF = Self::letters_to_token(b"XQF"),
	
	/// User assigned.
	XQG = Self::letters_to_token(b"XQG"),
	
	/// User assigned.
	XQH = Self::letters_to_token(b"XQH"),
	
	/// User assigned.
	XQI = Self::letters_to_token(b"XQI"),
	
	/// User assigned.
	XQJ = Self::letters_to_token(b"XQJ"),
	
	/// User assigned.
	XQK = Self::letters_to_token(b"XQK"),
	
	/// User assigned.
	XQL = Self::letters_to_token(b"XQL"),
	
	/// User assigned.
	XQM = Self::letters_to_token(b"XQM"),
	
	/// User assigned.
	XQN = Self::letters_to_token(b"XQN"),
	
	/// User assigned.
	XQO = Self::letters_to_token(b"XQO"),
	
	/// User assigned.
	XQP = Self::letters_to_token(b"XQP"),
	
	/// User assigned.
	XQQ = Self::letters_to_token(b"XQQ"),
	
	/// User assigned.
	XQR = Self::letters_to_token(b"XQR"),
	
	/// User assigned.
	XQS = Self::letters_to_token(b"XQS"),
	
	/// User assigned.
	XQT = Self::letters_to_token(b"XQT"),
	
	/// User assigned.
	XQU = Self::letters_to_token(b"XQU"),
	
	/// User assigned.
	XQV = Self::letters_to_token(b"XQV"),
	
	/// User assigned.
	XQW = Self::letters_to_token(b"XQW"),
	
	/// User assigned.
	XQX = Self::letters_to_token(b"XQX"),
	
	/// User assigned.
	XQY = Self::letters_to_token(b"XQY"),
	
	/// User assigned.
	XQZ = Self::letters_to_token(b"XQZ"),
	
	/// User assigned.
	XRA = Self::letters_to_token(b"XRA"),
	
	/// User assigned.
	XRB = Self::letters_to_token(b"XRB"),
	
	/// User assigned.
	XRC = Self::letters_to_token(b"XRC"),
	
	/// User assigned.
	XRD = Self::letters_to_token(b"XRD"),
	
	/// User assigned.
	XRE = Self::letters_to_token(b"XRE"),
	
	/// User assigned.
	XRF = Self::letters_to_token(b"XRF"),
	
	/// User assigned.
	XRG = Self::letters_to_token(b"XRG"),
	
	/// User assigned.
	XRH = Self::letters_to_token(b"XRH"),
	
	/// User assigned.
	XRI = Self::letters_to_token(b"XRI"),
	
	/// User assigned.
	XRJ = Self::letters_to_token(b"XRJ"),
	
	/// User assigned.
	XRK = Self::letters_to_token(b"XRK"),
	
	/// User assigned.
	XRL = Self::letters_to_token(b"XRL"),
	
	/// User assigned.
	XRM = Self::letters_to_token(b"XRM"),
	
	/// User assigned.
	XRN = Self::letters_to_token(b"XRN"),
	
	/// User assigned.
	XRO = Self::letters_to_token(b"XRO"),
	
	/// User assigned.
	XRP = Self::letters_to_token(b"XRP"),
	
	/// User assigned.
	XRQ = Self::letters_to_token(b"XRQ"),
	
	/// User assigned.
	XRR = Self::letters_to_token(b"XRR"),
	
	/// User assigned.
	XRS = Self::letters_to_token(b"XRS"),
	
	/// User assigned.
	XRT = Self::letters_to_token(b"XRT"),
	
	/// User assigned.
	XRU = Self::letters_to_token(b"XRU"),
	
	/// User assigned.
	XRV = Self::letters_to_token(b"XRV"),
	
	/// User assigned.
	XRW = Self::letters_to_token(b"XRW"),
	
	/// User assigned.
	XRX = Self::letters_to_token(b"XRX"),
	
	/// User assigned.
	XRY = Self::letters_to_token(b"XRY"),
	
	/// User assigned.
	XRZ = Self::letters_to_token(b"XRZ"),
	
	/// User assigned.
	XSA = Self::letters_to_token(b"XSA"),
	
	/// User assigned.
	XSB = Self::letters_to_token(b"XSB"),
	
	/// User assigned.
	XSC = Self::letters_to_token(b"XSC"),
	
	/// User assigned.
	XSD = Self::letters_to_token(b"XSD"),
	
	/// User assigned.
	XSE = Self::letters_to_token(b"XSE"),
	
	/// User assigned.
	XSF = Self::letters_to_token(b"XSF"),
	
	/// User assigned.
	XSG = Self::letters_to_token(b"XSG"),
	
	/// User assigned.
	XSH = Self::letters_to_token(b"XSH"),
	
	/// User assigned.
	XSI = Self::letters_to_token(b"XSI"),
	
	/// User assigned.
	XSJ = Self::letters_to_token(b"XSJ"),
	
	/// User assigned.
	XSK = Self::letters_to_token(b"XSK"),
	
	/// User assigned.
	XSL = Self::letters_to_token(b"XSL"),
	
	/// User assigned.
	XSM = Self::letters_to_token(b"XSM"),
	
	/// User assigned.
	XSN = Self::letters_to_token(b"XSN"),
	
	/// User assigned.
	XSO = Self::letters_to_token(b"XSO"),
	
	/// User assigned.
	XSP = Self::letters_to_token(b"XSP"),
	
	/// User assigned.
	XSQ = Self::letters_to_token(b"XSQ"),
	
	/// User assigned.
	XSR = Self::letters_to_token(b"XSR"),
	
	/// User assigned.
	XSS = Self::letters_to_token(b"XSS"),
	
	/// User assigned.
	XST = Self::letters_to_token(b"XST"),
	
	/// User assigned.
	XSU = Self::letters_to_token(b"XSU"),
	
	/// User assigned.
	XSV = Self::letters_to_token(b"XSV"),
	
	/// User assigned.
	XSW = Self::letters_to_token(b"XSW"),
	
	/// User assigned.
	XSX = Self::letters_to_token(b"XSX"),
	
	/// User assigned.
	XSY = Self::letters_to_token(b"XSY"),
	
	/// User assigned.
	XSZ = Self::letters_to_token(b"XSZ"),
	
	/// User assigned.
	XTA = Self::letters_to_token(b"XTA"),
	
	/// User assigned.
	XTB = Self::letters_to_token(b"XTB"),
	
	/// User assigned.
	XTC = Self::letters_to_token(b"XTC"),
	
	/// User assigned.
	XTD = Self::letters_to_token(b"XTD"),
	
	/// User assigned.
	XTE = Self::letters_to_token(b"XTE"),
	
	/// User assigned.
	XTF = Self::letters_to_token(b"XTF"),
	
	/// User assigned.
	XTG = Self::letters_to_token(b"XTG"),
	
	/// User assigned.
	XTH = Self::letters_to_token(b"XTH"),
	
	/// User assigned.
	XTI = Self::letters_to_token(b"XTI"),
	
	/// User assigned.
	XTJ = Self::letters_to_token(b"XTJ"),
	
	/// User assigned.
	XTK = Self::letters_to_token(b"XTK"),
	
	/// User assigned.
	XTL = Self::letters_to_token(b"XTL"),
	
	/// User assigned.
	XTM = Self::letters_to_token(b"XTM"),
	
	/// User assigned.
	XTN = Self::letters_to_token(b"XTN"),
	
	/// User assigned.
	XTO = Self::letters_to_token(b"XTO"),
	
	/// User assigned.
	XTP = Self::letters_to_token(b"XTP"),
	
	/// User assigned.
	XTQ = Self::letters_to_token(b"XTQ"),
	
	/// User assigned.
	XTR = Self::letters_to_token(b"XTR"),
	
	/// User assigned.
	XTS = Self::letters_to_token(b"XTS"),
	
	/// User assigned.
	XTT = Self::letters_to_token(b"XTT"),
	
	/// User assigned.
	XTU = Self::letters_to_token(b"XTU"),
	
	/// User assigned.
	XTV = Self::letters_to_token(b"XTV"),
	
	/// User assigned.
	XTW = Self::letters_to_token(b"XTW"),
	
	/// User assigned.
	XTX = Self::letters_to_token(b"XTX"),
	
	/// User assigned.
	XTY = Self::letters_to_token(b"XTY"),
	
	/// User assigned.
	XTZ = Self::letters_to_token(b"XTZ"),
	
	/// User assigned.
	XUA = Self::letters_to_token(b"XUA"),
	
	/// User assigned.
	XUB = Self::letters_to_token(b"XUB"),
	
	/// User assigned.
	XUC = Self::letters_to_token(b"XUC"),
	
	/// User assigned.
	XUD = Self::letters_to_token(b"XUD"),
	
	/// User assigned.
	XUE = Self::letters_to_token(b"XUE"),
	
	/// User assigned.
	XUF = Self::letters_to_token(b"XUF"),
	
	/// User assigned.
	XUG = Self::letters_to_token(b"XUG"),
	
	/// User assigned.
	XUH = Self::letters_to_token(b"XUH"),
	
	/// User assigned.
	XUI = Self::letters_to_token(b"XUI"),
	
	/// User assigned.
	XUJ = Self::letters_to_token(b"XUJ"),
	
	/// User assigned.
	XUK = Self::letters_to_token(b"XUK"),
	
	/// User assigned.
	XUL = Self::letters_to_token(b"XUL"),
	
	/// User assigned.
	XUM = Self::letters_to_token(b"XUM"),
	
	/// User assigned.
	XUN = Self::letters_to_token(b"XUN"),
	
	/// User assigned.
	XUO = Self::letters_to_token(b"XUO"),
	
	/// User assigned.
	XUP = Self::letters_to_token(b"XUP"),
	
	/// User assigned.
	XUQ = Self::letters_to_token(b"XUQ"),
	
	/// User assigned.
	XUR = Self::letters_to_token(b"XUR"),
	
	/// User assigned.
	XUS = Self::letters_to_token(b"XUS"),
	
	/// User assigned.
	XUT = Self::letters_to_token(b"XUT"),
	
	/// User assigned.
	XUU = Self::letters_to_token(b"XUU"),
	
	/// User assigned.
	XUV = Self::letters_to_token(b"XUV"),
	
	/// User assigned.
	XUW = Self::letters_to_token(b"XUW"),
	
	/// User assigned.
	XUX = Self::letters_to_token(b"XUX"),
	
	/// User assigned.
	XUY = Self::letters_to_token(b"XUY"),
	
	/// User assigned.
	XUZ = Self::letters_to_token(b"XUZ"),
	
	/// User assigned.
	XVA = Self::letters_to_token(b"XVA"),
	
	/// User assigned.
	XVB = Self::letters_to_token(b"XVB"),
	
	/// User assigned.
	XVC = Self::letters_to_token(b"XVC"),
	
	/// User assigned.
	XVD = Self::letters_to_token(b"XVD"),
	
	/// User assigned.
	XVE = Self::letters_to_token(b"XVE"),
	
	/// User assigned.
	XVF = Self::letters_to_token(b"XVF"),
	
	/// User assigned.
	XVG = Self::letters_to_token(b"XVG"),
	
	/// User assigned.
	XVH = Self::letters_to_token(b"XVH"),
	
	/// User assigned.
	XVI = Self::letters_to_token(b"XVI"),
	
	/// User assigned.
	XVJ = Self::letters_to_token(b"XVJ"),
	
	/// User assigned.
	XVK = Self::letters_to_token(b"XVK"),
	
	/// User assigned.
	XVL = Self::letters_to_token(b"XVL"),
	
	/// User assigned.
	XVM = Self::letters_to_token(b"XVM"),
	
	/// User assigned.
	XVN = Self::letters_to_token(b"XVN"),
	
	/// User assigned.
	XVO = Self::letters_to_token(b"XVO"),
	
	/// User assigned.
	XVP = Self::letters_to_token(b"XVP"),
	
	/// User assigned.
	XVQ = Self::letters_to_token(b"XVQ"),
	
	/// User assigned.
	XVR = Self::letters_to_token(b"XVR"),
	
	/// User assigned.
	XVS = Self::letters_to_token(b"XVS"),
	
	/// User assigned.
	XVT = Self::letters_to_token(b"XVT"),
	
	/// User assigned.
	XVU = Self::letters_to_token(b"XVU"),
	
	/// User assigned.
	XVV = Self::letters_to_token(b"XVV"),
	
	/// User assigned.
	XVW = Self::letters_to_token(b"XVW"),
	
	/// User assigned.
	XVX = Self::letters_to_token(b"XVX"),
	
	/// User assigned.
	XVY = Self::letters_to_token(b"XVY"),
	
	/// User assigned.
	XVZ = Self::letters_to_token(b"XVZ"),
	
	/// User assigned.
	XWA = Self::letters_to_token(b"XWA"),
	
	/// User assigned.
	XWB = Self::letters_to_token(b"XWB"),
	
	/// User assigned.
	XWC = Self::letters_to_token(b"XWC"),
	
	/// User assigned.
	XWD = Self::letters_to_token(b"XWD"),
	
	/// User assigned.
	XWE = Self::letters_to_token(b"XWE"),
	
	/// User assigned.
	XWF = Self::letters_to_token(b"XWF"),
	
	/// User assigned.
	XWG = Self::letters_to_token(b"XWG"),
	
	/// User assigned.
	XWH = Self::letters_to_token(b"XWH"),
	
	/// User assigned.
	XWI = Self::letters_to_token(b"XWI"),
	
	/// User assigned.
	XWJ = Self::letters_to_token(b"XWJ"),
	
	/// User assigned.
	XWK = Self::letters_to_token(b"XWK"),
	
	/// User assigned.
	XWL = Self::letters_to_token(b"XWL"),
	
	/// User assigned.
	XWM = Self::letters_to_token(b"XWM"),
	
	/// User assigned.
	XWN = Self::letters_to_token(b"XWN"),
	
	/// User assigned.
	XWO = Self::letters_to_token(b"XWO"),
	
	/// User assigned.
	XWP = Self::letters_to_token(b"XWP"),
	
	/// User assigned.
	XWQ = Self::letters_to_token(b"XWQ"),
	
	/// User assigned.
	XWR = Self::letters_to_token(b"XWR"),
	
	/// User assigned.
	XWS = Self::letters_to_token(b"XWS"),
	
	/// User assigned.
	XWT = Self::letters_to_token(b"XWT"),
	
	/// User assigned.
	XWU = Self::letters_to_token(b"XWU"),
	
	/// User assigned.
	XWV = Self::letters_to_token(b"XWV"),
	
	/// User assigned.
	XWW = Self::letters_to_token(b"XWW"),
	
	/// User assigned.
	XWX = Self::letters_to_token(b"XWX"),
	
	/// User assigned.
	XWY = Self::letters_to_token(b"XWY"),
	
	/// User assigned.
	XWZ = Self::letters_to_token(b"XWZ"),
	
	/// User assigned.
	///
	/// Stateless person as defined in Article 1 of the 1954 Convention Relating to the Status of Stateless Persons.
	XXA = Self::letters_to_token(b"XXA"),
	
	/// User assigned.
	///
	/// Refugee as defined in Article 1 of the 1954 Convention Relating to the Status of Stateless Persons as amended by the 1967 Protocol.
	/// Fictional Brownland country (NATO STANAG 1059 INT).
	XXB = Self::letters_to_token(b"XXB"),
	
	/// User assigned.
	///
	/// Refugee if not defined by in Article 1 of the 1954 Convention Relating to the Status of Stateless Persons as amended by the 1967 Protocol.
	XXC = Self::letters_to_token(b"XXC"),
	
	/// User assigned.
	XXD = Self::letters_to_token(b"XXD"),
	
	/// User assigned.
	///
	/// SHAPE (NATO STANAG 1059 INT).
	XXE = Self::letters_to_token(b"XXE"),
	
	/// User assigned.
	XXF = Self::letters_to_token(b"XXF"),
	
	/// User assigned.
	///
	/// Fictional Greyland country (NATO STANAG 1059 INT).
	XXG = Self::letters_to_token(b"XXG"),
	
	/// User assigned.
	XXH = Self::letters_to_token(b"XXH"),
	
	/// User assigned.
	///
	/// Fictional Indigoland country (NATO STANAG 1059 INT).
	XXI = Self::letters_to_token(b"XXI"),
	
	/// User assigned.
	XXJ = Self::letters_to_token(b"XXJ"),
	
	/// User assigned.
	XXK = Self::letters_to_token(b"XXK"),
	
	/// User assigned.
	///
	/// Fictional Limeland country (NATO STANAG 1059 INT).
	XXL = Self::letters_to_token(b"XXL"),
	
	/// User assigned.
	///
	/// NATO (NATO STANAG 1059 INT).
	XXM = Self::letters_to_token(b"XXM"),
	
	/// User assigned.
	///
	/// NATO Blue Command (NATO STANAG 1059 INT).
	XXN = Self::letters_to_token(b"XXN"),
	
	/// User assigned.
	XXO = Self::letters_to_token(b"XXO"),
	
	/// User assigned.
	///
	/// Fictional Purpleland country (NATO STANAG 1059 INT).
	XXP = Self::letters_to_token(b"XXP"),
	
	/// User assigned.
	XXQ = Self::letters_to_token(b"XXQ"),
	
	/// User assigned.
	///
	/// Fictional Redland country (NATO STANAG 1059 INT).
	XXR = Self::letters_to_token(b"XXR"),
	
	/// User assigned.
	///
	/// SACLANT (NATO STANAG 1059 INT).
	XXS = Self::letters_to_token(b"XXS"),
	
	/// User assigned.
	XXT = Self::letters_to_token(b"XXT"),
	
	/// User assigned.
	XXU = Self::letters_to_token(b"XXU"),
	
	/// User assigned.
	XXV = Self::letters_to_token(b"XXV"),
	
	/// User assigned.
	///
	/// Fictional Whiteland country (NATO STANAG 1059 INT).
	XXW = Self::letters_to_token(b"XXW"),
	
	/// User assigned.
	///
	/// Person of unspecified nationality.
	XXX = Self::letters_to_token(b"XXX"),
	
	/// User assigned.
	///
	/// Fictional Yellowland country (NATO STANAG 1059 INT).
	XXY = Self::letters_to_token(b"XXY"),
	
	/// User assigned.
	XXZ = Self::letters_to_token(b"XXZ"),
	
	/// User assigned.
	XYA = Self::letters_to_token(b"XYA"),
	
	/// User assigned.
	XYB = Self::letters_to_token(b"XYB"),
	
	/// User assigned.
	XYC = Self::letters_to_token(b"XYC"),
	
	/// User assigned.
	XYD = Self::letters_to_token(b"XYD"),
	
	/// User assigned.
	XYE = Self::letters_to_token(b"XYE"),
	
	/// User assigned.
	XYF = Self::letters_to_token(b"XYF"),
	
	/// User assigned.
	XYG = Self::letters_to_token(b"XYG"),
	
	/// User assigned.
	XYH = Self::letters_to_token(b"XYH"),
	
	/// User assigned.
	XYI = Self::letters_to_token(b"XYI"),
	
	/// User assigned.
	XYJ = Self::letters_to_token(b"XYJ"),
	
	/// User assigned.
	XYK = Self::letters_to_token(b"XYK"),
	
	/// User assigned.
	XYL = Self::letters_to_token(b"XYL"),
	
	/// User assigned.
	XYM = Self::letters_to_token(b"XYM"),
	
	/// User assigned.
	XYN = Self::letters_to_token(b"XYN"),
	
	/// User assigned.
	XYO = Self::letters_to_token(b"XYO"),
	
	/// User assigned.
	XYP = Self::letters_to_token(b"XYP"),
	
	/// User assigned.
	XYQ = Self::letters_to_token(b"XYQ"),
	
	/// User assigned.
	XYR = Self::letters_to_token(b"XYR"),
	
	/// User assigned.
	XYS = Self::letters_to_token(b"XYS"),
	
	/// User assigned.
	XYT = Self::letters_to_token(b"XYT"),
	
	/// User assigned.
	XYU = Self::letters_to_token(b"XYU"),
	
	/// User assigned.
	XYV = Self::letters_to_token(b"XYV"),
	
	/// User assigned.
	XYW = Self::letters_to_token(b"XYW"),
	
	/// User assigned.
	XYX = Self::letters_to_token(b"XYX"),
	
	/// User assigned.
	XYY = Self::letters_to_token(b"XYY"),
	
	/// User assigned.
	XYZ = Self::letters_to_token(b"XYZ"),
	
	/// User assigned.
	XZA = Self::letters_to_token(b"XZA"),
	
	/// User assigned.
	XZB = Self::letters_to_token(b"XZB"),
	
	/// User assigned.
	XZC = Self::letters_to_token(b"XZC"),
	
	/// User assigned.
	XZD = Self::letters_to_token(b"XZD"),
	
	/// User assigned.
	XZE = Self::letters_to_token(b"XZE"),
	
	/// User assigned.
	XZF = Self::letters_to_token(b"XZF"),
	
	/// User assigned.
	XZG = Self::letters_to_token(b"XZG"),
	
	/// User assigned.
	XZH = Self::letters_to_token(b"XZH"),
	
	/// User assigned.
	XZI = Self::letters_to_token(b"XZI"),
	
	/// User assigned.
	XZJ = Self::letters_to_token(b"XZJ"),
	
	/// User assigned.
	XZK = Self::letters_to_token(b"XZK"),
	
	/// User assigned.
	XZL = Self::letters_to_token(b"XZL"),
	
	/// User assigned.
	XZM = Self::letters_to_token(b"XZM"),
	
	/// User assigned.
	XZN = Self::letters_to_token(b"XZN"),
	
	/// User assigned.
	XZO = Self::letters_to_token(b"XZO"),
	
	/// User assigned.
	XZP = Self::letters_to_token(b"XZP"),
	
	/// User assigned.
	XZQ = Self::letters_to_token(b"XZQ"),
	
	/// User assigned.
	XZR = Self::letters_to_token(b"XZR"),
	
	/// User assigned.
	XZS = Self::letters_to_token(b"XZS"),
	
	/// User assigned.
	XZT = Self::letters_to_token(b"XZT"),
	
	/// User assigned.
	XZU = Self::letters_to_token(b"XZU"),
	
	/// User assigned.
	XZV = Self::letters_to_token(b"XZV"),
	
	/// User assigned.
	XZW = Self::letters_to_token(b"XZW"),
	
	/// User assigned.
	XZX = Self::letters_to_token(b"XZX"),
	
	/// User assigned.
	XZY = Self::letters_to_token(b"XZY"),
	
	/// User assigned.
	XZZ = Self::letters_to_token(b"XZZ"),
	/// TODO.
	YAA = Self::letters_to_token(b"YAA"),
	
	/// TODO.
	YAB = Self::letters_to_token(b"YAB"),
	
	/// TODO.
	YAC = Self::letters_to_token(b"YAC"),
	
	/// TODO.
	YAD = Self::letters_to_token(b"YAD"),
	
	/// TODO.
	YAE = Self::letters_to_token(b"YAE"),
	
	/// TODO.
	YAF = Self::letters_to_token(b"YAF"),
	
	/// TODO.
	YAG = Self::letters_to_token(b"YAG"),
	
	/// TODO.
	YAH = Self::letters_to_token(b"YAH"),
	
	/// TODO.
	YAI = Self::letters_to_token(b"YAI"),
	
	/// TODO.
	YAJ = Self::letters_to_token(b"YAJ"),
	
	/// TODO.
	YAK = Self::letters_to_token(b"YAK"),
	
	/// TODO.
	YAL = Self::letters_to_token(b"YAL"),
	
	/// TODO.
	YAM = Self::letters_to_token(b"YAM"),
	
	/// TODO.
	YAN = Self::letters_to_token(b"YAN"),
	
	/// TODO.
	YAO = Self::letters_to_token(b"YAO"),
	
	/// TODO.
	YAP = Self::letters_to_token(b"YAP"),
	
	/// TODO.
	YAQ = Self::letters_to_token(b"YAQ"),
	
	/// TODO.
	YAR = Self::letters_to_token(b"YAR"),
	
	/// TODO.
	YAS = Self::letters_to_token(b"YAS"),
	
	/// TODO.
	YAT = Self::letters_to_token(b"YAT"),
	
	/// TODO.
	YAU = Self::letters_to_token(b"YAU"),
	
	/// TODO.
	YAV = Self::letters_to_token(b"YAV"),
	
	/// TODO.
	YAW = Self::letters_to_token(b"YAW"),
	
	/// TODO.
	YAX = Self::letters_to_token(b"YAX"),
	
	/// TODO.
	YAY = Self::letters_to_token(b"YAY"),
	
	/// TODO.
	YAZ = Self::letters_to_token(b"YAZ"),
	
	/// TODO.
	YBA = Self::letters_to_token(b"YBA"),
	
	/// TODO.
	YBB = Self::letters_to_token(b"YBB"),
	
	/// TODO.
	YBC = Self::letters_to_token(b"YBC"),
	
	/// TODO.
	YBD = Self::letters_to_token(b"YBD"),
	
	/// TODO.
	YBE = Self::letters_to_token(b"YBE"),
	
	/// TODO.
	YBF = Self::letters_to_token(b"YBF"),
	
	/// TODO.
	YBG = Self::letters_to_token(b"YBG"),
	
	/// TODO.
	YBH = Self::letters_to_token(b"YBH"),
	
	/// TODO.
	YBI = Self::letters_to_token(b"YBI"),
	
	/// TODO.
	YBJ = Self::letters_to_token(b"YBJ"),
	
	/// TODO.
	YBK = Self::letters_to_token(b"YBK"),
	
	/// TODO.
	YBL = Self::letters_to_token(b"YBL"),
	
	/// TODO.
	YBM = Self::letters_to_token(b"YBM"),
	
	/// TODO.
	YBN = Self::letters_to_token(b"YBN"),
	
	/// TODO.
	YBO = Self::letters_to_token(b"YBO"),
	
	/// TODO.
	YBP = Self::letters_to_token(b"YBP"),
	
	/// TODO.
	YBQ = Self::letters_to_token(b"YBQ"),
	
	/// TODO.
	YBR = Self::letters_to_token(b"YBR"),
	
	/// TODO.
	YBS = Self::letters_to_token(b"YBS"),
	
	/// TODO.
	YBT = Self::letters_to_token(b"YBT"),
	
	/// TODO.
	YBU = Self::letters_to_token(b"YBU"),
	
	/// TODO.
	YBV = Self::letters_to_token(b"YBV"),
	
	/// TODO.
	YBW = Self::letters_to_token(b"YBW"),
	
	/// TODO.
	YBX = Self::letters_to_token(b"YBX"),
	
	/// TODO.
	YBY = Self::letters_to_token(b"YBY"),
	
	/// TODO.
	YBZ = Self::letters_to_token(b"YBZ"),
	
	/// TODO.
	YCA = Self::letters_to_token(b"YCA"),
	
	/// TODO.
	YCB = Self::letters_to_token(b"YCB"),
	
	/// TODO.
	YCC = Self::letters_to_token(b"YCC"),
	
	/// TODO.
	YCD = Self::letters_to_token(b"YCD"),
	
	/// TODO.
	YCE = Self::letters_to_token(b"YCE"),
	
	/// TODO.
	YCF = Self::letters_to_token(b"YCF"),
	
	/// TODO.
	YCG = Self::letters_to_token(b"YCG"),
	
	/// TODO.
	YCH = Self::letters_to_token(b"YCH"),
	
	/// TODO.
	YCI = Self::letters_to_token(b"YCI"),
	
	/// TODO.
	YCJ = Self::letters_to_token(b"YCJ"),
	
	/// TODO.
	YCK = Self::letters_to_token(b"YCK"),
	
	/// TODO.
	YCL = Self::letters_to_token(b"YCL"),
	
	/// TODO.
	YCM = Self::letters_to_token(b"YCM"),
	
	/// TODO.
	YCN = Self::letters_to_token(b"YCN"),
	
	/// TODO.
	YCO = Self::letters_to_token(b"YCO"),
	
	/// TODO.
	YCP = Self::letters_to_token(b"YCP"),
	
	/// TODO.
	YCQ = Self::letters_to_token(b"YCQ"),
	
	/// TODO.
	YCR = Self::letters_to_token(b"YCR"),
	
	/// TODO.
	YCS = Self::letters_to_token(b"YCS"),
	
	/// TODO.
	YCT = Self::letters_to_token(b"YCT"),
	
	/// TODO.
	YCU = Self::letters_to_token(b"YCU"),
	
	/// TODO.
	YCV = Self::letters_to_token(b"YCV"),
	
	/// TODO.
	YCW = Self::letters_to_token(b"YCW"),
	
	/// TODO.
	YCX = Self::letters_to_token(b"YCX"),
	
	/// TODO.
	YCY = Self::letters_to_token(b"YCY"),
	
	/// TODO.
	YCZ = Self::letters_to_token(b"YCZ"),
	
	/// TODO.
	YDA = Self::letters_to_token(b"YDA"),
	
	/// TODO.
	YDB = Self::letters_to_token(b"YDB"),
	
	/// TODO.
	YDC = Self::letters_to_token(b"YDC"),
	
	/// TODO.
	YDD = Self::letters_to_token(b"YDD"),
	
	/// TODO.
	YDE = Self::letters_to_token(b"YDE"),
	
	/// TODO.
	YDF = Self::letters_to_token(b"YDF"),
	
	/// TODO.
	YDG = Self::letters_to_token(b"YDG"),
	
	/// TODO.
	YDH = Self::letters_to_token(b"YDH"),
	
	/// TODO.
	YDI = Self::letters_to_token(b"YDI"),
	
	/// TODO.
	YDJ = Self::letters_to_token(b"YDJ"),
	
	/// TODO.
	YDK = Self::letters_to_token(b"YDK"),
	
	/// TODO.
	YDL = Self::letters_to_token(b"YDL"),
	
	/// TODO.
	YDM = Self::letters_to_token(b"YDM"),
	
	/// TODO.
	YDN = Self::letters_to_token(b"YDN"),
	
	/// TODO.
	YDO = Self::letters_to_token(b"YDO"),
	
	/// TODO.
	YDP = Self::letters_to_token(b"YDP"),
	
	/// TODO.
	YDQ = Self::letters_to_token(b"YDQ"),
	
	/// TODO.
	YDR = Self::letters_to_token(b"YDR"),
	
	/// TODO.
	YDS = Self::letters_to_token(b"YDS"),
	
	/// TODO.
	YDT = Self::letters_to_token(b"YDT"),
	
	/// TODO.
	YDU = Self::letters_to_token(b"YDU"),
	
	/// TODO.
	YDV = Self::letters_to_token(b"YDV"),
	
	/// TODO.
	YDW = Self::letters_to_token(b"YDW"),
	
	/// TODO.
	YDX = Self::letters_to_token(b"YDX"),
	
	/// TODO.
	YDY = Self::letters_to_token(b"YDY"),
	
	/// TODO.
	YDZ = Self::letters_to_token(b"YDZ"),
	
	/// TODO.
	YEA = Self::letters_to_token(b"YEA"),
	
	/// TODO.
	YEB = Self::letters_to_token(b"YEB"),
	
	/// TODO.
	YEC = Self::letters_to_token(b"YEC"),
	
	/// TODO.
	YED = Self::letters_to_token(b"YED"),
	
	/// TODO.
	YEE = Self::letters_to_token(b"YEE"),
	
	/// TODO.
	YEF = Self::letters_to_token(b"YEF"),
	
	/// TODO.
	YEG = Self::letters_to_token(b"YEG"),
	
	/// TODO.
	YEH = Self::letters_to_token(b"YEH"),
	
	/// TODO.
	YEI = Self::letters_to_token(b"YEI"),
	
	/// TODO.
	YEJ = Self::letters_to_token(b"YEJ"),
	
	/// TODO.
	YEK = Self::letters_to_token(b"YEK"),
	
	/// TODO.
	YEL = Self::letters_to_token(b"YEL"),
	
	/// TODO.
	YEM = Self::letters_to_token(b"YEM"),
	
	/// TODO.
	YEN = Self::letters_to_token(b"YEN"),
	
	/// TODO.
	YEO = Self::letters_to_token(b"YEO"),
	
	/// TODO.
	YEP = Self::letters_to_token(b"YEP"),
	
	/// TODO.
	YEQ = Self::letters_to_token(b"YEQ"),
	
	/// TODO.
	YER = Self::letters_to_token(b"YER"),
	
	/// TODO.
	YES = Self::letters_to_token(b"YES"),
	
	/// TODO.
	YET = Self::letters_to_token(b"YET"),
	
	/// TODO.
	YEU = Self::letters_to_token(b"YEU"),
	
	/// TODO.
	YEV = Self::letters_to_token(b"YEV"),
	
	/// TODO.
	YEW = Self::letters_to_token(b"YEW"),
	
	/// TODO.
	YEX = Self::letters_to_token(b"YEX"),
	
	/// TODO.
	YEY = Self::letters_to_token(b"YEY"),
	
	/// TODO.
	YEZ = Self::letters_to_token(b"YEZ"),
	
	/// TODO.
	YFA = Self::letters_to_token(b"YFA"),
	
	/// TODO.
	YFB = Self::letters_to_token(b"YFB"),
	
	/// TODO.
	YFC = Self::letters_to_token(b"YFC"),
	
	/// TODO.
	YFD = Self::letters_to_token(b"YFD"),
	
	/// TODO.
	YFE = Self::letters_to_token(b"YFE"),
	
	/// TODO.
	YFF = Self::letters_to_token(b"YFF"),
	
	/// TODO.
	YFG = Self::letters_to_token(b"YFG"),
	
	/// TODO.
	YFH = Self::letters_to_token(b"YFH"),
	
	/// TODO.
	YFI = Self::letters_to_token(b"YFI"),
	
	/// TODO.
	YFJ = Self::letters_to_token(b"YFJ"),
	
	/// TODO.
	YFK = Self::letters_to_token(b"YFK"),
	
	/// TODO.
	YFL = Self::letters_to_token(b"YFL"),
	
	/// TODO.
	YFM = Self::letters_to_token(b"YFM"),
	
	/// TODO.
	YFN = Self::letters_to_token(b"YFN"),
	
	/// TODO.
	YFO = Self::letters_to_token(b"YFO"),
	
	/// TODO.
	YFP = Self::letters_to_token(b"YFP"),
	
	/// TODO.
	YFQ = Self::letters_to_token(b"YFQ"),
	
	/// TODO.
	YFR = Self::letters_to_token(b"YFR"),
	
	/// TODO.
	YFS = Self::letters_to_token(b"YFS"),
	
	/// TODO.
	YFT = Self::letters_to_token(b"YFT"),
	
	/// TODO.
	YFU = Self::letters_to_token(b"YFU"),
	
	/// TODO.
	YFV = Self::letters_to_token(b"YFV"),
	
	/// TODO.
	YFW = Self::letters_to_token(b"YFW"),
	
	/// TODO.
	YFX = Self::letters_to_token(b"YFX"),
	
	/// TODO.
	YFY = Self::letters_to_token(b"YFY"),
	
	/// TODO.
	YFZ = Self::letters_to_token(b"YFZ"),
	
	/// TODO.
	YGA = Self::letters_to_token(b"YGA"),
	
	/// TODO.
	YGB = Self::letters_to_token(b"YGB"),
	
	/// TODO.
	YGC = Self::letters_to_token(b"YGC"),
	
	/// TODO.
	YGD = Self::letters_to_token(b"YGD"),
	
	/// TODO.
	YGE = Self::letters_to_token(b"YGE"),
	
	/// TODO.
	YGF = Self::letters_to_token(b"YGF"),
	
	/// TODO.
	YGG = Self::letters_to_token(b"YGG"),
	
	/// TODO.
	YGH = Self::letters_to_token(b"YGH"),
	
	/// TODO.
	YGI = Self::letters_to_token(b"YGI"),
	
	/// TODO.
	YGJ = Self::letters_to_token(b"YGJ"),
	
	/// TODO.
	YGK = Self::letters_to_token(b"YGK"),
	
	/// TODO.
	YGL = Self::letters_to_token(b"YGL"),
	
	/// TODO.
	YGM = Self::letters_to_token(b"YGM"),
	
	/// TODO.
	YGN = Self::letters_to_token(b"YGN"),
	
	/// TODO.
	YGO = Self::letters_to_token(b"YGO"),
	
	/// TODO.
	YGP = Self::letters_to_token(b"YGP"),
	
	/// TODO.
	YGQ = Self::letters_to_token(b"YGQ"),
	
	/// TODO.
	YGR = Self::letters_to_token(b"YGR"),
	
	/// TODO.
	YGS = Self::letters_to_token(b"YGS"),
	
	/// TODO.
	YGT = Self::letters_to_token(b"YGT"),
	
	/// TODO.
	YGU = Self::letters_to_token(b"YGU"),
	
	/// TODO.
	YGV = Self::letters_to_token(b"YGV"),
	
	/// TODO.
	YGW = Self::letters_to_token(b"YGW"),
	
	/// TODO.
	YGX = Self::letters_to_token(b"YGX"),
	
	/// TODO.
	YGY = Self::letters_to_token(b"YGY"),
	
	/// TODO.
	YGZ = Self::letters_to_token(b"YGZ"),
	
	/// TODO.
	YHA = Self::letters_to_token(b"YHA"),
	
	/// TODO.
	YHB = Self::letters_to_token(b"YHB"),
	
	/// TODO.
	YHC = Self::letters_to_token(b"YHC"),
	
	/// TODO.
	YHD = Self::letters_to_token(b"YHD"),
	
	/// TODO.
	YHE = Self::letters_to_token(b"YHE"),
	
	/// TODO.
	YHF = Self::letters_to_token(b"YHF"),
	
	/// TODO.
	YHG = Self::letters_to_token(b"YHG"),
	
	/// TODO.
	YHH = Self::letters_to_token(b"YHH"),
	
	/// TODO.
	YHI = Self::letters_to_token(b"YHI"),
	
	/// TODO.
	YHJ = Self::letters_to_token(b"YHJ"),
	
	/// TODO.
	YHK = Self::letters_to_token(b"YHK"),
	
	/// TODO.
	YHL = Self::letters_to_token(b"YHL"),
	
	/// TODO.
	YHM = Self::letters_to_token(b"YHM"),
	
	/// TODO.
	YHN = Self::letters_to_token(b"YHN"),
	
	/// TODO.
	YHO = Self::letters_to_token(b"YHO"),
	
	/// TODO.
	YHP = Self::letters_to_token(b"YHP"),
	
	/// TODO.
	YHQ = Self::letters_to_token(b"YHQ"),
	
	/// TODO.
	YHR = Self::letters_to_token(b"YHR"),
	
	/// TODO.
	YHS = Self::letters_to_token(b"YHS"),
	
	/// TODO.
	YHT = Self::letters_to_token(b"YHT"),
	
	/// TODO.
	YHU = Self::letters_to_token(b"YHU"),
	
	/// TODO.
	YHV = Self::letters_to_token(b"YHV"),
	
	/// TODO.
	YHW = Self::letters_to_token(b"YHW"),
	
	/// TODO.
	YHX = Self::letters_to_token(b"YHX"),
	
	/// TODO.
	YHY = Self::letters_to_token(b"YHY"),
	
	/// TODO.
	YHZ = Self::letters_to_token(b"YHZ"),
	
	/// TODO.
	YIA = Self::letters_to_token(b"YIA"),
	
	/// TODO.
	YIB = Self::letters_to_token(b"YIB"),
	
	/// TODO.
	YIC = Self::letters_to_token(b"YIC"),
	
	/// TODO.
	YID = Self::letters_to_token(b"YID"),
	
	/// TODO.
	YIE = Self::letters_to_token(b"YIE"),
	
	/// TODO.
	YIF = Self::letters_to_token(b"YIF"),
	
	/// TODO.
	YIG = Self::letters_to_token(b"YIG"),
	
	/// TODO.
	YIH = Self::letters_to_token(b"YIH"),
	
	/// TODO.
	YII = Self::letters_to_token(b"YII"),
	
	/// TODO.
	YIJ = Self::letters_to_token(b"YIJ"),
	
	/// TODO.
	YIK = Self::letters_to_token(b"YIK"),
	
	/// TODO.
	YIL = Self::letters_to_token(b"YIL"),
	
	/// TODO.
	YIM = Self::letters_to_token(b"YIM"),
	
	/// TODO.
	YIN = Self::letters_to_token(b"YIN"),
	
	/// TODO.
	YIO = Self::letters_to_token(b"YIO"),
	
	/// TODO.
	YIP = Self::letters_to_token(b"YIP"),
	
	/// TODO.
	YIQ = Self::letters_to_token(b"YIQ"),
	
	/// TODO.
	YIR = Self::letters_to_token(b"YIR"),
	
	/// TODO.
	YIS = Self::letters_to_token(b"YIS"),
	
	/// TODO.
	YIT = Self::letters_to_token(b"YIT"),
	
	/// TODO.
	YIU = Self::letters_to_token(b"YIU"),
	
	/// TODO.
	YIV = Self::letters_to_token(b"YIV"),
	
	/// TODO.
	YIW = Self::letters_to_token(b"YIW"),
	
	/// TODO.
	YIX = Self::letters_to_token(b"YIX"),
	
	/// TODO.
	YIY = Self::letters_to_token(b"YIY"),
	
	/// TODO.
	YIZ = Self::letters_to_token(b"YIZ"),
	
	/// TODO.
	YJA = Self::letters_to_token(b"YJA"),
	
	/// TODO.
	YJB = Self::letters_to_token(b"YJB"),
	
	/// TODO.
	YJC = Self::letters_to_token(b"YJC"),
	
	/// TODO.
	YJD = Self::letters_to_token(b"YJD"),
	
	/// TODO.
	YJE = Self::letters_to_token(b"YJE"),
	
	/// TODO.
	YJF = Self::letters_to_token(b"YJF"),
	
	/// TODO.
	YJG = Self::letters_to_token(b"YJG"),
	
	/// TODO.
	YJH = Self::letters_to_token(b"YJH"),
	
	/// TODO.
	YJI = Self::letters_to_token(b"YJI"),
	
	/// TODO.
	YJJ = Self::letters_to_token(b"YJJ"),
	
	/// TODO.
	YJK = Self::letters_to_token(b"YJK"),
	
	/// TODO.
	YJL = Self::letters_to_token(b"YJL"),
	
	/// TODO.
	YJM = Self::letters_to_token(b"YJM"),
	
	/// TODO.
	YJN = Self::letters_to_token(b"YJN"),
	
	/// TODO.
	YJO = Self::letters_to_token(b"YJO"),
	
	/// TODO.
	YJP = Self::letters_to_token(b"YJP"),
	
	/// TODO.
	YJQ = Self::letters_to_token(b"YJQ"),
	
	/// TODO.
	YJR = Self::letters_to_token(b"YJR"),
	
	/// TODO.
	YJS = Self::letters_to_token(b"YJS"),
	
	/// TODO.
	YJT = Self::letters_to_token(b"YJT"),
	
	/// TODO.
	YJU = Self::letters_to_token(b"YJU"),
	
	/// TODO.
	YJV = Self::letters_to_token(b"YJV"),
	
	/// TODO.
	YJW = Self::letters_to_token(b"YJW"),
	
	/// TODO.
	YJX = Self::letters_to_token(b"YJX"),
	
	/// TODO.
	YJY = Self::letters_to_token(b"YJY"),
	
	/// TODO.
	YJZ = Self::letters_to_token(b"YJZ"),
	
	/// TODO.
	YKA = Self::letters_to_token(b"YKA"),
	
	/// TODO.
	YKB = Self::letters_to_token(b"YKB"),
	
	/// TODO.
	YKC = Self::letters_to_token(b"YKC"),
	
	/// TODO.
	YKD = Self::letters_to_token(b"YKD"),
	
	/// TODO.
	YKE = Self::letters_to_token(b"YKE"),
	
	/// TODO.
	YKF = Self::letters_to_token(b"YKF"),
	
	/// TODO.
	YKG = Self::letters_to_token(b"YKG"),
	
	/// TODO.
	YKH = Self::letters_to_token(b"YKH"),
	
	/// TODO.
	YKI = Self::letters_to_token(b"YKI"),
	
	/// TODO.
	YKJ = Self::letters_to_token(b"YKJ"),
	
	/// TODO.
	YKK = Self::letters_to_token(b"YKK"),
	
	/// TODO.
	YKL = Self::letters_to_token(b"YKL"),
	
	/// TODO.
	YKM = Self::letters_to_token(b"YKM"),
	
	/// TODO.
	YKN = Self::letters_to_token(b"YKN"),
	
	/// TODO.
	YKO = Self::letters_to_token(b"YKO"),
	
	/// TODO.
	YKP = Self::letters_to_token(b"YKP"),
	
	/// TODO.
	YKQ = Self::letters_to_token(b"YKQ"),
	
	/// TODO.
	YKR = Self::letters_to_token(b"YKR"),
	
	/// TODO.
	YKS = Self::letters_to_token(b"YKS"),
	
	/// TODO.
	YKT = Self::letters_to_token(b"YKT"),
	
	/// TODO.
	YKU = Self::letters_to_token(b"YKU"),
	
	/// TODO.
	YKV = Self::letters_to_token(b"YKV"),
	
	/// TODO.
	YKW = Self::letters_to_token(b"YKW"),
	
	/// TODO.
	YKX = Self::letters_to_token(b"YKX"),
	
	/// TODO.
	YKY = Self::letters_to_token(b"YKY"),
	
	/// TODO.
	YKZ = Self::letters_to_token(b"YKZ"),
	
	/// TODO.
	YLA = Self::letters_to_token(b"YLA"),
	
	/// TODO.
	YLB = Self::letters_to_token(b"YLB"),
	
	/// TODO.
	YLC = Self::letters_to_token(b"YLC"),
	
	/// TODO.
	YLD = Self::letters_to_token(b"YLD"),
	
	/// TODO.
	YLE = Self::letters_to_token(b"YLE"),
	
	/// TODO.
	YLF = Self::letters_to_token(b"YLF"),
	
	/// TODO.
	YLG = Self::letters_to_token(b"YLG"),
	
	/// TODO.
	YLH = Self::letters_to_token(b"YLH"),
	
	/// TODO.
	YLI = Self::letters_to_token(b"YLI"),
	
	/// TODO.
	YLJ = Self::letters_to_token(b"YLJ"),
	
	/// TODO.
	YLK = Self::letters_to_token(b"YLK"),
	
	/// TODO.
	YLL = Self::letters_to_token(b"YLL"),
	
	/// TODO.
	YLM = Self::letters_to_token(b"YLM"),
	
	/// TODO.
	YLN = Self::letters_to_token(b"YLN"),
	
	/// TODO.
	YLO = Self::letters_to_token(b"YLO"),
	
	/// TODO.
	YLP = Self::letters_to_token(b"YLP"),
	
	/// TODO.
	YLQ = Self::letters_to_token(b"YLQ"),
	
	/// TODO.
	YLR = Self::letters_to_token(b"YLR"),
	
	/// TODO.
	YLS = Self::letters_to_token(b"YLS"),
	
	/// TODO.
	YLT = Self::letters_to_token(b"YLT"),
	
	/// TODO.
	YLU = Self::letters_to_token(b"YLU"),
	
	/// TODO.
	YLV = Self::letters_to_token(b"YLV"),
	
	/// TODO.
	YLW = Self::letters_to_token(b"YLW"),
	
	/// TODO.
	YLX = Self::letters_to_token(b"YLX"),
	
	/// TODO.
	YLY = Self::letters_to_token(b"YLY"),
	
	/// TODO.
	YLZ = Self::letters_to_token(b"YLZ"),
	
	/// TODO.
	YMA = Self::letters_to_token(b"YMA"),
	
	/// TODO.
	YMB = Self::letters_to_token(b"YMB"),
	
	/// TODO.
	YMC = Self::letters_to_token(b"YMC"),
	
	/// TODO.
	YMD = Self::letters_to_token(b"YMD"),
	
	/// TODO.
	YME = Self::letters_to_token(b"YME"),
	
	/// TODO.
	YMF = Self::letters_to_token(b"YMF"),
	
	/// TODO.
	YMG = Self::letters_to_token(b"YMG"),
	
	/// TODO.
	YMH = Self::letters_to_token(b"YMH"),
	
	/// TODO.
	YMI = Self::letters_to_token(b"YMI"),
	
	/// TODO.
	YMJ = Self::letters_to_token(b"YMJ"),
	
	/// TODO.
	YMK = Self::letters_to_token(b"YMK"),
	
	/// TODO.
	YML = Self::letters_to_token(b"YML"),
	
	/// TODO.
	YMM = Self::letters_to_token(b"YMM"),
	
	/// TODO.
	YMN = Self::letters_to_token(b"YMN"),
	
	/// TODO.
	YMO = Self::letters_to_token(b"YMO"),
	
	/// TODO.
	YMP = Self::letters_to_token(b"YMP"),
	
	/// TODO.
	YMQ = Self::letters_to_token(b"YMQ"),
	
	/// TODO.
	YMR = Self::letters_to_token(b"YMR"),
	
	/// TODO.
	YMS = Self::letters_to_token(b"YMS"),
	
	/// TODO.
	YMT = Self::letters_to_token(b"YMT"),
	
	/// TODO.
	YMU = Self::letters_to_token(b"YMU"),
	
	/// TODO.
	YMV = Self::letters_to_token(b"YMV"),
	
	/// TODO.
	YMW = Self::letters_to_token(b"YMW"),
	
	/// TODO.
	YMX = Self::letters_to_token(b"YMX"),
	
	/// TODO.
	YMY = Self::letters_to_token(b"YMY"),
	
	/// TODO.
	YMZ = Self::letters_to_token(b"YMZ"),
	
	/// TODO.
	YNA = Self::letters_to_token(b"YNA"),
	
	/// TODO.
	YNB = Self::letters_to_token(b"YNB"),
	
	/// TODO.
	YNC = Self::letters_to_token(b"YNC"),
	
	/// TODO.
	YND = Self::letters_to_token(b"YND"),
	
	/// TODO.
	YNE = Self::letters_to_token(b"YNE"),
	
	/// TODO.
	YNF = Self::letters_to_token(b"YNF"),
	
	/// TODO.
	YNG = Self::letters_to_token(b"YNG"),
	
	/// TODO.
	YNH = Self::letters_to_token(b"YNH"),
	
	/// TODO.
	YNI = Self::letters_to_token(b"YNI"),
	
	/// TODO.
	YNJ = Self::letters_to_token(b"YNJ"),
	
	/// TODO.
	YNK = Self::letters_to_token(b"YNK"),
	
	/// TODO.
	YNL = Self::letters_to_token(b"YNL"),
	
	/// TODO.
	YNM = Self::letters_to_token(b"YNM"),
	
	/// TODO.
	YNN = Self::letters_to_token(b"YNN"),
	
	/// TODO.
	YNO = Self::letters_to_token(b"YNO"),
	
	/// TODO.
	YNP = Self::letters_to_token(b"YNP"),
	
	/// TODO.
	YNQ = Self::letters_to_token(b"YNQ"),
	
	/// TODO.
	YNR = Self::letters_to_token(b"YNR"),
	
	/// TODO.
	YNS = Self::letters_to_token(b"YNS"),
	
	/// TODO.
	YNT = Self::letters_to_token(b"YNT"),
	
	/// TODO.
	YNU = Self::letters_to_token(b"YNU"),
	
	/// TODO.
	YNV = Self::letters_to_token(b"YNV"),
	
	/// TODO.
	YNW = Self::letters_to_token(b"YNW"),
	
	/// TODO.
	YNX = Self::letters_to_token(b"YNX"),
	
	/// TODO.
	YNY = Self::letters_to_token(b"YNY"),
	
	/// TODO.
	YNZ = Self::letters_to_token(b"YNZ"),
	
	/// TODO.
	YOA = Self::letters_to_token(b"YOA"),
	
	/// TODO.
	YOB = Self::letters_to_token(b"YOB"),
	
	/// TODO.
	YOC = Self::letters_to_token(b"YOC"),
	
	/// TODO.
	YOD = Self::letters_to_token(b"YOD"),
	
	/// TODO.
	YOE = Self::letters_to_token(b"YOE"),
	
	/// TODO.
	YOF = Self::letters_to_token(b"YOF"),
	
	/// TODO.
	YOG = Self::letters_to_token(b"YOG"),
	
	/// TODO.
	YOH = Self::letters_to_token(b"YOH"),
	
	/// TODO.
	YOI = Self::letters_to_token(b"YOI"),
	
	/// TODO.
	YOJ = Self::letters_to_token(b"YOJ"),
	
	/// TODO.
	YOK = Self::letters_to_token(b"YOK"),
	
	/// TODO.
	YOL = Self::letters_to_token(b"YOL"),
	
	/// TODO.
	YOM = Self::letters_to_token(b"YOM"),
	
	/// TODO.
	YON = Self::letters_to_token(b"YON"),
	
	/// TODO.
	YOO = Self::letters_to_token(b"YOO"),
	
	/// TODO.
	YOP = Self::letters_to_token(b"YOP"),
	
	/// TODO.
	YOQ = Self::letters_to_token(b"YOQ"),
	
	/// TODO.
	YOR = Self::letters_to_token(b"YOR"),
	
	/// TODO.
	YOS = Self::letters_to_token(b"YOS"),
	
	/// TODO.
	YOT = Self::letters_to_token(b"YOT"),
	
	/// TODO.
	YOU = Self::letters_to_token(b"YOU"),
	
	/// TODO.
	YOV = Self::letters_to_token(b"YOV"),
	
	/// TODO.
	YOW = Self::letters_to_token(b"YOW"),
	
	/// TODO.
	YOX = Self::letters_to_token(b"YOX"),
	
	/// TODO.
	YOY = Self::letters_to_token(b"YOY"),
	
	/// TODO.
	YOZ = Self::letters_to_token(b"YOZ"),
	
	/// TODO.
	YPA = Self::letters_to_token(b"YPA"),
	
	/// TODO.
	YPB = Self::letters_to_token(b"YPB"),
	
	/// TODO.
	YPC = Self::letters_to_token(b"YPC"),
	
	/// TODO.
	YPD = Self::letters_to_token(b"YPD"),
	
	/// TODO.
	YPE = Self::letters_to_token(b"YPE"),
	
	/// TODO.
	YPF = Self::letters_to_token(b"YPF"),
	
	/// TODO.
	YPG = Self::letters_to_token(b"YPG"),
	
	/// TODO.
	YPH = Self::letters_to_token(b"YPH"),
	
	/// TODO.
	YPI = Self::letters_to_token(b"YPI"),
	
	/// TODO.
	YPJ = Self::letters_to_token(b"YPJ"),
	
	/// TODO.
	YPK = Self::letters_to_token(b"YPK"),
	
	/// TODO.
	YPL = Self::letters_to_token(b"YPL"),
	
	/// TODO.
	YPM = Self::letters_to_token(b"YPM"),
	
	/// TODO.
	YPN = Self::letters_to_token(b"YPN"),
	
	/// TODO.
	YPO = Self::letters_to_token(b"YPO"),
	
	/// TODO.
	YPP = Self::letters_to_token(b"YPP"),
	
	/// TODO.
	YPQ = Self::letters_to_token(b"YPQ"),
	
	/// TODO.
	YPR = Self::letters_to_token(b"YPR"),
	
	/// TODO.
	YPS = Self::letters_to_token(b"YPS"),
	
	/// TODO.
	YPT = Self::letters_to_token(b"YPT"),
	
	/// TODO.
	YPU = Self::letters_to_token(b"YPU"),
	
	/// TODO.
	YPV = Self::letters_to_token(b"YPV"),
	
	/// TODO.
	YPW = Self::letters_to_token(b"YPW"),
	
	/// TODO.
	YPX = Self::letters_to_token(b"YPX"),
	
	/// TODO.
	YPY = Self::letters_to_token(b"YPY"),
	
	/// TODO.
	YPZ = Self::letters_to_token(b"YPZ"),
	
	/// TODO.
	YQA = Self::letters_to_token(b"YQA"),
	
	/// TODO.
	YQB = Self::letters_to_token(b"YQB"),
	
	/// TODO.
	YQC = Self::letters_to_token(b"YQC"),
	
	/// TODO.
	YQD = Self::letters_to_token(b"YQD"),
	
	/// TODO.
	YQE = Self::letters_to_token(b"YQE"),
	
	/// TODO.
	YQF = Self::letters_to_token(b"YQF"),
	
	/// TODO.
	YQG = Self::letters_to_token(b"YQG"),
	
	/// TODO.
	YQH = Self::letters_to_token(b"YQH"),
	
	/// TODO.
	YQI = Self::letters_to_token(b"YQI"),
	
	/// TODO.
	YQJ = Self::letters_to_token(b"YQJ"),
	
	/// TODO.
	YQK = Self::letters_to_token(b"YQK"),
	
	/// TODO.
	YQL = Self::letters_to_token(b"YQL"),
	
	/// TODO.
	YQM = Self::letters_to_token(b"YQM"),
	
	/// TODO.
	YQN = Self::letters_to_token(b"YQN"),
	
	/// TODO.
	YQO = Self::letters_to_token(b"YQO"),
	
	/// TODO.
	YQP = Self::letters_to_token(b"YQP"),
	
	/// TODO.
	YQQ = Self::letters_to_token(b"YQQ"),
	
	/// TODO.
	YQR = Self::letters_to_token(b"YQR"),
	
	/// TODO.
	YQS = Self::letters_to_token(b"YQS"),
	
	/// TODO.
	YQT = Self::letters_to_token(b"YQT"),
	
	/// TODO.
	YQU = Self::letters_to_token(b"YQU"),
	
	/// TODO.
	YQV = Self::letters_to_token(b"YQV"),
	
	/// TODO.
	YQW = Self::letters_to_token(b"YQW"),
	
	/// TODO.
	YQX = Self::letters_to_token(b"YQX"),
	
	/// TODO.
	YQY = Self::letters_to_token(b"YQY"),
	
	/// TODO.
	YQZ = Self::letters_to_token(b"YQZ"),
	
	/// TODO.
	YRA = Self::letters_to_token(b"YRA"),
	
	/// TODO.
	YRB = Self::letters_to_token(b"YRB"),
	
	/// TODO.
	YRC = Self::letters_to_token(b"YRC"),
	
	/// TODO.
	YRD = Self::letters_to_token(b"YRD"),
	
	/// TODO.
	YRE = Self::letters_to_token(b"YRE"),
	
	/// TODO.
	YRF = Self::letters_to_token(b"YRF"),
	
	/// TODO.
	YRG = Self::letters_to_token(b"YRG"),
	
	/// TODO.
	YRH = Self::letters_to_token(b"YRH"),
	
	/// TODO.
	YRI = Self::letters_to_token(b"YRI"),
	
	/// TODO.
	YRJ = Self::letters_to_token(b"YRJ"),
	
	/// TODO.
	YRK = Self::letters_to_token(b"YRK"),
	
	/// TODO.
	YRL = Self::letters_to_token(b"YRL"),
	
	/// TODO.
	YRM = Self::letters_to_token(b"YRM"),
	
	/// TODO.
	YRN = Self::letters_to_token(b"YRN"),
	
	/// TODO.
	YRO = Self::letters_to_token(b"YRO"),
	
	/// TODO.
	YRP = Self::letters_to_token(b"YRP"),
	
	/// TODO.
	YRQ = Self::letters_to_token(b"YRQ"),
	
	/// TODO.
	YRR = Self::letters_to_token(b"YRR"),
	
	/// TODO.
	YRS = Self::letters_to_token(b"YRS"),
	
	/// TODO.
	YRT = Self::letters_to_token(b"YRT"),
	
	/// TODO.
	YRU = Self::letters_to_token(b"YRU"),
	
	/// TODO.
	YRV = Self::letters_to_token(b"YRV"),
	
	/// TODO.
	YRW = Self::letters_to_token(b"YRW"),
	
	/// TODO.
	YRX = Self::letters_to_token(b"YRX"),
	
	/// TODO.
	YRY = Self::letters_to_token(b"YRY"),
	
	/// TODO.
	YRZ = Self::letters_to_token(b"YRZ"),
	
	/// TODO.
	YSA = Self::letters_to_token(b"YSA"),
	
	/// TODO.
	YSB = Self::letters_to_token(b"YSB"),
	
	/// TODO.
	YSC = Self::letters_to_token(b"YSC"),
	
	/// TODO.
	YSD = Self::letters_to_token(b"YSD"),
	
	/// TODO.
	YSE = Self::letters_to_token(b"YSE"),
	
	/// TODO.
	YSF = Self::letters_to_token(b"YSF"),
	
	/// TODO.
	YSG = Self::letters_to_token(b"YSG"),
	
	/// TODO.
	YSH = Self::letters_to_token(b"YSH"),
	
	/// TODO.
	YSI = Self::letters_to_token(b"YSI"),
	
	/// TODO.
	YSJ = Self::letters_to_token(b"YSJ"),
	
	/// TODO.
	YSK = Self::letters_to_token(b"YSK"),
	
	/// TODO.
	YSL = Self::letters_to_token(b"YSL"),
	
	/// TODO.
	YSM = Self::letters_to_token(b"YSM"),
	
	/// TODO.
	YSN = Self::letters_to_token(b"YSN"),
	
	/// TODO.
	YSO = Self::letters_to_token(b"YSO"),
	
	/// TODO.
	YSP = Self::letters_to_token(b"YSP"),
	
	/// TODO.
	YSQ = Self::letters_to_token(b"YSQ"),
	
	/// TODO.
	YSR = Self::letters_to_token(b"YSR"),
	
	/// TODO.
	YSS = Self::letters_to_token(b"YSS"),
	
	/// TODO.
	YST = Self::letters_to_token(b"YST"),
	
	/// TODO.
	YSU = Self::letters_to_token(b"YSU"),
	
	/// TODO.
	YSV = Self::letters_to_token(b"YSV"),
	
	/// TODO.
	YSW = Self::letters_to_token(b"YSW"),
	
	/// TODO.
	YSX = Self::letters_to_token(b"YSX"),
	
	/// TODO.
	YSY = Self::letters_to_token(b"YSY"),
	
	/// TODO.
	YSZ = Self::letters_to_token(b"YSZ"),
	
	/// TODO.
	YTA = Self::letters_to_token(b"YTA"),
	
	/// TODO.
	YTB = Self::letters_to_token(b"YTB"),
	
	/// TODO.
	YTC = Self::letters_to_token(b"YTC"),
	
	/// TODO.
	YTD = Self::letters_to_token(b"YTD"),
	
	/// TODO.
	YTE = Self::letters_to_token(b"YTE"),
	
	/// TODO.
	YTF = Self::letters_to_token(b"YTF"),
	
	/// TODO.
	YTG = Self::letters_to_token(b"YTG"),
	
	/// TODO.
	YTH = Self::letters_to_token(b"YTH"),
	
	/// TODO.
	YTI = Self::letters_to_token(b"YTI"),
	
	/// TODO.
	YTJ = Self::letters_to_token(b"YTJ"),
	
	/// TODO.
	YTK = Self::letters_to_token(b"YTK"),
	
	/// TODO.
	YTL = Self::letters_to_token(b"YTL"),
	
	/// TODO.
	YTM = Self::letters_to_token(b"YTM"),
	
	/// TODO.
	YTN = Self::letters_to_token(b"YTN"),
	
	/// TODO.
	YTO = Self::letters_to_token(b"YTO"),
	
	/// TODO.
	YTP = Self::letters_to_token(b"YTP"),
	
	/// TODO.
	YTQ = Self::letters_to_token(b"YTQ"),
	
	/// TODO.
	YTR = Self::letters_to_token(b"YTR"),
	
	/// TODO.
	YTS = Self::letters_to_token(b"YTS"),
	
	/// TODO.
	YTT = Self::letters_to_token(b"YTT"),
	
	/// TODO.
	YTU = Self::letters_to_token(b"YTU"),
	
	/// TODO.
	YTV = Self::letters_to_token(b"YTV"),
	
	/// TODO.
	YTW = Self::letters_to_token(b"YTW"),
	
	/// TODO.
	YTX = Self::letters_to_token(b"YTX"),
	
	/// TODO.
	YTY = Self::letters_to_token(b"YTY"),
	
	/// TODO.
	YTZ = Self::letters_to_token(b"YTZ"),
	
	/// TODO.
	YUA = Self::letters_to_token(b"YUA"),
	
	/// TODO.
	YUB = Self::letters_to_token(b"YUB"),
	
	/// TODO.
	YUC = Self::letters_to_token(b"YUC"),
	
	/// TODO.
	YUD = Self::letters_to_token(b"YUD"),
	
	/// TODO.
	YUE = Self::letters_to_token(b"YUE"),
	
	/// TODO.
	YUF = Self::letters_to_token(b"YUF"),
	
	/// TODO.
	YUG = Self::letters_to_token(b"YUG"),
	
	/// TODO.
	YUH = Self::letters_to_token(b"YUH"),
	
	/// TODO.
	YUI = Self::letters_to_token(b"YUI"),
	
	/// TODO.
	YUJ = Self::letters_to_token(b"YUJ"),
	
	/// TODO.
	YUK = Self::letters_to_token(b"YUK"),
	
	/// TODO.
	YUL = Self::letters_to_token(b"YUL"),
	
	/// TODO.
	YUM = Self::letters_to_token(b"YUM"),
	
	/// TODO.
	YUN = Self::letters_to_token(b"YUN"),
	
	/// TODO.
	YUO = Self::letters_to_token(b"YUO"),
	
	/// TODO.
	YUP = Self::letters_to_token(b"YUP"),
	
	/// TODO.
	YUQ = Self::letters_to_token(b"YUQ"),
	
	/// TODO.
	YUR = Self::letters_to_token(b"YUR"),
	
	/// TODO.
	YUS = Self::letters_to_token(b"YUS"),
	
	/// TODO.
	YUT = Self::letters_to_token(b"YUT"),
	
	/// TODO.
	YUU = Self::letters_to_token(b"YUU"),
	
	/// TODO.
	YUV = Self::letters_to_token(b"YUV"),
	
	/// TODO.
	YUW = Self::letters_to_token(b"YUW"),
	
	/// TODO.
	YUX = Self::letters_to_token(b"YUX"),
	
	/// TODO.
	YUY = Self::letters_to_token(b"YUY"),
	
	/// TODO.
	YUZ = Self::letters_to_token(b"YUZ"),
	
	/// TODO.
	YVA = Self::letters_to_token(b"YVA"),
	
	/// TODO.
	YVB = Self::letters_to_token(b"YVB"),
	
	/// TODO.
	YVC = Self::letters_to_token(b"YVC"),
	
	/// TODO.
	YVD = Self::letters_to_token(b"YVD"),
	
	/// TODO.
	YVE = Self::letters_to_token(b"YVE"),
	
	/// TODO.
	YVF = Self::letters_to_token(b"YVF"),
	
	/// TODO.
	YVG = Self::letters_to_token(b"YVG"),
	
	/// TODO.
	YVH = Self::letters_to_token(b"YVH"),
	
	/// TODO.
	YVI = Self::letters_to_token(b"YVI"),
	
	/// TODO.
	YVJ = Self::letters_to_token(b"YVJ"),
	
	/// TODO.
	YVK = Self::letters_to_token(b"YVK"),
	
	/// TODO.
	YVL = Self::letters_to_token(b"YVL"),
	
	/// TODO.
	YVM = Self::letters_to_token(b"YVM"),
	
	/// TODO.
	YVN = Self::letters_to_token(b"YVN"),
	
	/// TODO.
	YVO = Self::letters_to_token(b"YVO"),
	
	/// TODO.
	YVP = Self::letters_to_token(b"YVP"),
	
	/// TODO.
	YVQ = Self::letters_to_token(b"YVQ"),
	
	/// TODO.
	YVR = Self::letters_to_token(b"YVR"),
	
	/// TODO.
	YVS = Self::letters_to_token(b"YVS"),
	
	/// TODO.
	YVT = Self::letters_to_token(b"YVT"),
	
	/// TODO.
	YVU = Self::letters_to_token(b"YVU"),
	
	/// TODO.
	YVV = Self::letters_to_token(b"YVV"),
	
	/// TODO.
	YVW = Self::letters_to_token(b"YVW"),
	
	/// TODO.
	YVX = Self::letters_to_token(b"YVX"),
	
	/// TODO.
	YVY = Self::letters_to_token(b"YVY"),
	
	/// TODO.
	YVZ = Self::letters_to_token(b"YVZ"),
	
	/// TODO.
	YWA = Self::letters_to_token(b"YWA"),
	
	/// TODO.
	YWB = Self::letters_to_token(b"YWB"),
	
	/// TODO.
	YWC = Self::letters_to_token(b"YWC"),
	
	/// TODO.
	YWD = Self::letters_to_token(b"YWD"),
	
	/// TODO.
	YWE = Self::letters_to_token(b"YWE"),
	
	/// TODO.
	YWF = Self::letters_to_token(b"YWF"),
	
	/// TODO.
	YWG = Self::letters_to_token(b"YWG"),
	
	/// TODO.
	YWH = Self::letters_to_token(b"YWH"),
	
	/// TODO.
	YWI = Self::letters_to_token(b"YWI"),
	
	/// TODO.
	YWJ = Self::letters_to_token(b"YWJ"),
	
	/// TODO.
	YWK = Self::letters_to_token(b"YWK"),
	
	/// TODO.
	YWL = Self::letters_to_token(b"YWL"),
	
	/// TODO.
	YWM = Self::letters_to_token(b"YWM"),
	
	/// TODO.
	YWN = Self::letters_to_token(b"YWN"),
	
	/// TODO.
	YWO = Self::letters_to_token(b"YWO"),
	
	/// TODO.
	YWP = Self::letters_to_token(b"YWP"),
	
	/// TODO.
	YWQ = Self::letters_to_token(b"YWQ"),
	
	/// TODO.
	YWR = Self::letters_to_token(b"YWR"),
	
	/// TODO.
	YWS = Self::letters_to_token(b"YWS"),
	
	/// TODO.
	YWT = Self::letters_to_token(b"YWT"),
	
	/// TODO.
	YWU = Self::letters_to_token(b"YWU"),
	
	/// TODO.
	YWV = Self::letters_to_token(b"YWV"),
	
	/// TODO.
	YWW = Self::letters_to_token(b"YWW"),
	
	/// TODO.
	YWX = Self::letters_to_token(b"YWX"),
	
	/// TODO.
	YWY = Self::letters_to_token(b"YWY"),
	
	/// TODO.
	YWZ = Self::letters_to_token(b"YWZ"),
	
	/// TODO.
	YXA = Self::letters_to_token(b"YXA"),
	
	/// TODO.
	YXB = Self::letters_to_token(b"YXB"),
	
	/// TODO.
	YXC = Self::letters_to_token(b"YXC"),
	
	/// TODO.
	YXD = Self::letters_to_token(b"YXD"),
	
	/// TODO.
	YXE = Self::letters_to_token(b"YXE"),
	
	/// TODO.
	YXF = Self::letters_to_token(b"YXF"),
	
	/// TODO.
	YXG = Self::letters_to_token(b"YXG"),
	
	/// TODO.
	YXH = Self::letters_to_token(b"YXH"),
	
	/// TODO.
	YXI = Self::letters_to_token(b"YXI"),
	
	/// TODO.
	YXJ = Self::letters_to_token(b"YXJ"),
	
	/// TODO.
	YXK = Self::letters_to_token(b"YXK"),
	
	/// TODO.
	YXL = Self::letters_to_token(b"YXL"),
	
	/// TODO.
	YXM = Self::letters_to_token(b"YXM"),
	
	/// TODO.
	YXN = Self::letters_to_token(b"YXN"),
	
	/// TODO.
	YXO = Self::letters_to_token(b"YXO"),
	
	/// TODO.
	YXP = Self::letters_to_token(b"YXP"),
	
	/// TODO.
	YXQ = Self::letters_to_token(b"YXQ"),
	
	/// TODO.
	YXR = Self::letters_to_token(b"YXR"),
	
	/// TODO.
	YXS = Self::letters_to_token(b"YXS"),
	
	/// TODO.
	YXT = Self::letters_to_token(b"YXT"),
	
	/// TODO.
	YXU = Self::letters_to_token(b"YXU"),
	
	/// TODO.
	YXV = Self::letters_to_token(b"YXV"),
	
	/// TODO.
	YXW = Self::letters_to_token(b"YXW"),
	
	/// TODO.
	YXX = Self::letters_to_token(b"YXX"),
	
	/// TODO.
	YXY = Self::letters_to_token(b"YXY"),
	
	/// TODO.
	YXZ = Self::letters_to_token(b"YXZ"),
	
	/// TODO.
	YYA = Self::letters_to_token(b"YYA"),
	
	/// TODO.
	YYB = Self::letters_to_token(b"YYB"),
	
	/// TODO.
	YYC = Self::letters_to_token(b"YYC"),
	
	/// TODO.
	YYD = Self::letters_to_token(b"YYD"),
	
	/// TODO.
	YYE = Self::letters_to_token(b"YYE"),
	
	/// TODO.
	YYF = Self::letters_to_token(b"YYF"),
	
	/// TODO.
	YYG = Self::letters_to_token(b"YYG"),
	
	/// TODO.
	YYH = Self::letters_to_token(b"YYH"),
	
	/// TODO.
	YYI = Self::letters_to_token(b"YYI"),
	
	/// TODO.
	YYJ = Self::letters_to_token(b"YYJ"),
	
	/// TODO.
	YYK = Self::letters_to_token(b"YYK"),
	
	/// TODO.
	YYL = Self::letters_to_token(b"YYL"),
	
	/// TODO.
	YYM = Self::letters_to_token(b"YYM"),
	
	/// TODO.
	YYN = Self::letters_to_token(b"YYN"),
	
	/// TODO.
	YYO = Self::letters_to_token(b"YYO"),
	
	/// TODO.
	YYP = Self::letters_to_token(b"YYP"),
	
	/// TODO.
	YYQ = Self::letters_to_token(b"YYQ"),
	
	/// TODO.
	YYR = Self::letters_to_token(b"YYR"),
	
	/// TODO.
	YYS = Self::letters_to_token(b"YYS"),
	
	/// TODO.
	YYT = Self::letters_to_token(b"YYT"),
	
	/// TODO.
	YYU = Self::letters_to_token(b"YYU"),
	
	/// TODO.
	YYV = Self::letters_to_token(b"YYV"),
	
	/// TODO.
	YYW = Self::letters_to_token(b"YYW"),
	
	/// TODO.
	YYX = Self::letters_to_token(b"YYX"),
	
	/// TODO.
	YYY = Self::letters_to_token(b"YYY"),
	
	/// TODO.
	YYZ = Self::letters_to_token(b"YYZ"),
	
	/// TODO.
	YZA = Self::letters_to_token(b"YZA"),
	
	/// TODO.
	YZB = Self::letters_to_token(b"YZB"),
	
	/// TODO.
	YZC = Self::letters_to_token(b"YZC"),
	
	/// TODO.
	YZD = Self::letters_to_token(b"YZD"),
	
	/// TODO.
	YZE = Self::letters_to_token(b"YZE"),
	
	/// TODO.
	YZF = Self::letters_to_token(b"YZF"),
	
	/// TODO.
	YZG = Self::letters_to_token(b"YZG"),
	
	/// TODO.
	YZH = Self::letters_to_token(b"YZH"),
	
	/// TODO.
	YZI = Self::letters_to_token(b"YZI"),
	
	/// TODO.
	YZJ = Self::letters_to_token(b"YZJ"),
	
	/// TODO.
	YZK = Self::letters_to_token(b"YZK"),
	
	/// TODO.
	YZL = Self::letters_to_token(b"YZL"),
	
	/// TODO.
	YZM = Self::letters_to_token(b"YZM"),
	
	/// TODO.
	YZN = Self::letters_to_token(b"YZN"),
	
	/// TODO.
	YZO = Self::letters_to_token(b"YZO"),
	
	/// TODO.
	YZP = Self::letters_to_token(b"YZP"),
	
	/// TODO.
	YZQ = Self::letters_to_token(b"YZQ"),
	
	/// TODO.
	YZR = Self::letters_to_token(b"YZR"),
	
	/// TODO.
	YZS = Self::letters_to_token(b"YZS"),
	
	/// TODO.
	YZT = Self::letters_to_token(b"YZT"),
	
	/// TODO.
	YZU = Self::letters_to_token(b"YZU"),
	
	/// TODO.
	YZV = Self::letters_to_token(b"YZV"),
	
	/// TODO.
	YZW = Self::letters_to_token(b"YZW"),
	
	/// TODO.
	YZX = Self::letters_to_token(b"YZX"),
	
	/// TODO.
	YZY = Self::letters_to_token(b"YZY"),
	
	/// TODO.
	YZZ = Self::letters_to_token(b"YZZ"),
	
	/// TODO.
	ZAA = Self::letters_to_token(b"ZAA"),
	
	/// TODO.
	ZAB = Self::letters_to_token(b"ZAB"),
	
	/// TODO.
	ZAC = Self::letters_to_token(b"ZAC"),
	
	/// TODO.
	ZAD = Self::letters_to_token(b"ZAD"),
	
	/// TODO.
	ZAE = Self::letters_to_token(b"ZAE"),
	
	/// TODO.
	ZAF = Self::letters_to_token(b"ZAF"),
	
	/// TODO.
	ZAG = Self::letters_to_token(b"ZAG"),
	
	/// TODO.
	ZAH = Self::letters_to_token(b"ZAH"),
	
	/// TODO.
	ZAI = Self::letters_to_token(b"ZAI"),
	
	/// TODO.
	ZAJ = Self::letters_to_token(b"ZAJ"),
	
	/// TODO.
	ZAK = Self::letters_to_token(b"ZAK"),
	
	/// TODO.
	ZAL = Self::letters_to_token(b"ZAL"),
	
	/// TODO.
	ZAM = Self::letters_to_token(b"ZAM"),
	
	/// TODO.
	ZAN = Self::letters_to_token(b"ZAN"),
	
	/// TODO.
	ZAO = Self::letters_to_token(b"ZAO"),
	
	/// TODO.
	ZAP = Self::letters_to_token(b"ZAP"),
	
	/// TODO.
	ZAQ = Self::letters_to_token(b"ZAQ"),
	
	/// TODO.
	ZAR = Self::letters_to_token(b"ZAR"),
	
	/// TODO.
	ZAS = Self::letters_to_token(b"ZAS"),
	
	/// TODO.
	ZAT = Self::letters_to_token(b"ZAT"),
	
	/// TODO.
	ZAU = Self::letters_to_token(b"ZAU"),
	
	/// TODO.
	ZAV = Self::letters_to_token(b"ZAV"),
	
	/// TODO.
	ZAW = Self::letters_to_token(b"ZAW"),
	
	/// TODO.
	ZAX = Self::letters_to_token(b"ZAX"),
	
	/// TODO.
	ZAY = Self::letters_to_token(b"ZAY"),
	
	/// TODO.
	ZAZ = Self::letters_to_token(b"ZAZ"),
	
	/// TODO.
	ZBA = Self::letters_to_token(b"ZBA"),
	
	/// TODO.
	ZBB = Self::letters_to_token(b"ZBB"),
	
	/// TODO.
	ZBC = Self::letters_to_token(b"ZBC"),
	
	/// TODO.
	ZBD = Self::letters_to_token(b"ZBD"),
	
	/// TODO.
	ZBE = Self::letters_to_token(b"ZBE"),
	
	/// TODO.
	ZBF = Self::letters_to_token(b"ZBF"),
	
	/// TODO.
	ZBG = Self::letters_to_token(b"ZBG"),
	
	/// TODO.
	ZBH = Self::letters_to_token(b"ZBH"),
	
	/// TODO.
	ZBI = Self::letters_to_token(b"ZBI"),
	
	/// TODO.
	ZBJ = Self::letters_to_token(b"ZBJ"),
	
	/// TODO.
	ZBK = Self::letters_to_token(b"ZBK"),
	
	/// TODO.
	ZBL = Self::letters_to_token(b"ZBL"),
	
	/// TODO.
	ZBM = Self::letters_to_token(b"ZBM"),
	
	/// TODO.
	ZBN = Self::letters_to_token(b"ZBN"),
	
	/// TODO.
	ZBO = Self::letters_to_token(b"ZBO"),
	
	/// TODO.
	ZBP = Self::letters_to_token(b"ZBP"),
	
	/// TODO.
	ZBQ = Self::letters_to_token(b"ZBQ"),
	
	/// TODO.
	ZBR = Self::letters_to_token(b"ZBR"),
	
	/// TODO.
	ZBS = Self::letters_to_token(b"ZBS"),
	
	/// TODO.
	ZBT = Self::letters_to_token(b"ZBT"),
	
	/// TODO.
	ZBU = Self::letters_to_token(b"ZBU"),
	
	/// TODO.
	ZBV = Self::letters_to_token(b"ZBV"),
	
	/// TODO.
	ZBW = Self::letters_to_token(b"ZBW"),
	
	/// TODO.
	ZBX = Self::letters_to_token(b"ZBX"),
	
	/// TODO.
	ZBY = Self::letters_to_token(b"ZBY"),
	
	/// TODO.
	ZBZ = Self::letters_to_token(b"ZBZ"),
	
	/// TODO.
	ZCA = Self::letters_to_token(b"ZCA"),
	
	/// TODO.
	ZCB = Self::letters_to_token(b"ZCB"),
	
	/// TODO.
	ZCC = Self::letters_to_token(b"ZCC"),
	
	/// TODO.
	ZCD = Self::letters_to_token(b"ZCD"),
	
	/// TODO.
	ZCE = Self::letters_to_token(b"ZCE"),
	
	/// TODO.
	ZCF = Self::letters_to_token(b"ZCF"),
	
	/// TODO.
	ZCG = Self::letters_to_token(b"ZCG"),
	
	/// TODO.
	ZCH = Self::letters_to_token(b"ZCH"),
	
	/// TODO.
	ZCI = Self::letters_to_token(b"ZCI"),
	
	/// TODO.
	ZCJ = Self::letters_to_token(b"ZCJ"),
	
	/// TODO.
	ZCK = Self::letters_to_token(b"ZCK"),
	
	/// TODO.
	ZCL = Self::letters_to_token(b"ZCL"),
	
	/// TODO.
	ZCM = Self::letters_to_token(b"ZCM"),
	
	/// TODO.
	ZCN = Self::letters_to_token(b"ZCN"),
	
	/// TODO.
	ZCO = Self::letters_to_token(b"ZCO"),
	
	/// TODO.
	ZCP = Self::letters_to_token(b"ZCP"),
	
	/// TODO.
	ZCQ = Self::letters_to_token(b"ZCQ"),
	
	/// TODO.
	ZCR = Self::letters_to_token(b"ZCR"),
	
	/// TODO.
	ZCS = Self::letters_to_token(b"ZCS"),
	
	/// TODO.
	ZCT = Self::letters_to_token(b"ZCT"),
	
	/// TODO.
	ZCU = Self::letters_to_token(b"ZCU"),
	
	/// TODO.
	ZCV = Self::letters_to_token(b"ZCV"),
	
	/// TODO.
	ZCW = Self::letters_to_token(b"ZCW"),
	
	/// TODO.
	ZCX = Self::letters_to_token(b"ZCX"),
	
	/// TODO.
	ZCY = Self::letters_to_token(b"ZCY"),
	
	/// TODO.
	ZCZ = Self::letters_to_token(b"ZCZ"),
	
	/// TODO.
	ZDA = Self::letters_to_token(b"ZDA"),
	
	/// TODO.
	ZDB = Self::letters_to_token(b"ZDB"),
	
	/// TODO.
	ZDC = Self::letters_to_token(b"ZDC"),
	
	/// TODO.
	ZDD = Self::letters_to_token(b"ZDD"),
	
	/// TODO.
	ZDE = Self::letters_to_token(b"ZDE"),
	
	/// TODO.
	ZDF = Self::letters_to_token(b"ZDF"),
	
	/// TODO.
	ZDG = Self::letters_to_token(b"ZDG"),
	
	/// TODO.
	ZDH = Self::letters_to_token(b"ZDH"),
	
	/// TODO.
	ZDI = Self::letters_to_token(b"ZDI"),
	
	/// TODO.
	ZDJ = Self::letters_to_token(b"ZDJ"),
	
	/// TODO.
	ZDK = Self::letters_to_token(b"ZDK"),
	
	/// TODO.
	ZDL = Self::letters_to_token(b"ZDL"),
	
	/// TODO.
	ZDM = Self::letters_to_token(b"ZDM"),
	
	/// TODO.
	ZDN = Self::letters_to_token(b"ZDN"),
	
	/// TODO.
	ZDO = Self::letters_to_token(b"ZDO"),
	
	/// TODO.
	ZDP = Self::letters_to_token(b"ZDP"),
	
	/// TODO.
	ZDQ = Self::letters_to_token(b"ZDQ"),
	
	/// TODO.
	ZDR = Self::letters_to_token(b"ZDR"),
	
	/// TODO.
	ZDS = Self::letters_to_token(b"ZDS"),
	
	/// TODO.
	ZDT = Self::letters_to_token(b"ZDT"),
	
	/// TODO.
	ZDU = Self::letters_to_token(b"ZDU"),
	
	/// TODO.
	ZDV = Self::letters_to_token(b"ZDV"),
	
	/// TODO.
	ZDW = Self::letters_to_token(b"ZDW"),
	
	/// TODO.
	ZDX = Self::letters_to_token(b"ZDX"),
	
	/// TODO.
	ZDY = Self::letters_to_token(b"ZDY"),
	
	/// TODO.
	ZDZ = Self::letters_to_token(b"ZDZ"),
	
	/// TODO.
	ZEA = Self::letters_to_token(b"ZEA"),
	
	/// TODO.
	ZEB = Self::letters_to_token(b"ZEB"),
	
	/// TODO.
	ZEC = Self::letters_to_token(b"ZEC"),
	
	/// TODO.
	ZED = Self::letters_to_token(b"ZED"),
	
	/// TODO.
	ZEE = Self::letters_to_token(b"ZEE"),
	
	/// TODO.
	ZEF = Self::letters_to_token(b"ZEF"),
	
	/// TODO.
	ZEG = Self::letters_to_token(b"ZEG"),
	
	/// TODO.
	ZEH = Self::letters_to_token(b"ZEH"),
	
	/// TODO.
	ZEI = Self::letters_to_token(b"ZEI"),
	
	/// TODO.
	ZEJ = Self::letters_to_token(b"ZEJ"),
	
	/// TODO.
	ZEK = Self::letters_to_token(b"ZEK"),
	
	/// TODO.
	ZEL = Self::letters_to_token(b"ZEL"),
	
	/// TODO.
	ZEM = Self::letters_to_token(b"ZEM"),
	
	/// TODO.
	ZEN = Self::letters_to_token(b"ZEN"),
	
	/// TODO.
	ZEO = Self::letters_to_token(b"ZEO"),
	
	/// TODO.
	ZEP = Self::letters_to_token(b"ZEP"),
	
	/// TODO.
	ZEQ = Self::letters_to_token(b"ZEQ"),
	
	/// TODO.
	ZER = Self::letters_to_token(b"ZER"),
	
	/// TODO.
	ZES = Self::letters_to_token(b"ZES"),
	
	/// TODO.
	ZET = Self::letters_to_token(b"ZET"),
	
	/// TODO.
	ZEU = Self::letters_to_token(b"ZEU"),
	
	/// TODO.
	ZEV = Self::letters_to_token(b"ZEV"),
	
	/// TODO.
	ZEW = Self::letters_to_token(b"ZEW"),
	
	/// TODO.
	ZEX = Self::letters_to_token(b"ZEX"),
	
	/// TODO.
	ZEY = Self::letters_to_token(b"ZEY"),
	
	/// TODO.
	ZEZ = Self::letters_to_token(b"ZEZ"),
	
	/// TODO.
	ZFA = Self::letters_to_token(b"ZFA"),
	
	/// TODO.
	ZFB = Self::letters_to_token(b"ZFB"),
	
	/// TODO.
	ZFC = Self::letters_to_token(b"ZFC"),
	
	/// TODO.
	ZFD = Self::letters_to_token(b"ZFD"),
	
	/// TODO.
	ZFE = Self::letters_to_token(b"ZFE"),
	
	/// TODO.
	ZFF = Self::letters_to_token(b"ZFF"),
	
	/// TODO.
	ZFG = Self::letters_to_token(b"ZFG"),
	
	/// TODO.
	ZFH = Self::letters_to_token(b"ZFH"),
	
	/// TODO.
	ZFI = Self::letters_to_token(b"ZFI"),
	
	/// TODO.
	ZFJ = Self::letters_to_token(b"ZFJ"),
	
	/// TODO.
	ZFK = Self::letters_to_token(b"ZFK"),
	
	/// TODO.
	ZFL = Self::letters_to_token(b"ZFL"),
	
	/// TODO.
	ZFM = Self::letters_to_token(b"ZFM"),
	
	/// TODO.
	ZFN = Self::letters_to_token(b"ZFN"),
	
	/// TODO.
	ZFO = Self::letters_to_token(b"ZFO"),
	
	/// TODO.
	ZFP = Self::letters_to_token(b"ZFP"),
	
	/// TODO.
	ZFQ = Self::letters_to_token(b"ZFQ"),
	
	/// TODO.
	ZFR = Self::letters_to_token(b"ZFR"),
	
	/// TODO.
	ZFS = Self::letters_to_token(b"ZFS"),
	
	/// TODO.
	ZFT = Self::letters_to_token(b"ZFT"),
	
	/// TODO.
	ZFU = Self::letters_to_token(b"ZFU"),
	
	/// TODO.
	ZFV = Self::letters_to_token(b"ZFV"),
	
	/// TODO.
	ZFW = Self::letters_to_token(b"ZFW"),
	
	/// TODO.
	ZFX = Self::letters_to_token(b"ZFX"),
	
	/// TODO.
	ZFY = Self::letters_to_token(b"ZFY"),
	
	/// TODO.
	ZFZ = Self::letters_to_token(b"ZFZ"),
	
	/// TODO.
	ZGA = Self::letters_to_token(b"ZGA"),
	
	/// TODO.
	ZGB = Self::letters_to_token(b"ZGB"),
	
	/// TODO.
	ZGC = Self::letters_to_token(b"ZGC"),
	
	/// TODO.
	ZGD = Self::letters_to_token(b"ZGD"),
	
	/// TODO.
	ZGE = Self::letters_to_token(b"ZGE"),
	
	/// TODO.
	ZGF = Self::letters_to_token(b"ZGF"),
	
	/// TODO.
	ZGG = Self::letters_to_token(b"ZGG"),
	
	/// TODO.
	ZGH = Self::letters_to_token(b"ZGH"),
	
	/// TODO.
	ZGI = Self::letters_to_token(b"ZGI"),
	
	/// TODO.
	ZGJ = Self::letters_to_token(b"ZGJ"),
	
	/// TODO.
	ZGK = Self::letters_to_token(b"ZGK"),
	
	/// TODO.
	ZGL = Self::letters_to_token(b"ZGL"),
	
	/// TODO.
	ZGM = Self::letters_to_token(b"ZGM"),
	
	/// TODO.
	ZGN = Self::letters_to_token(b"ZGN"),
	
	/// TODO.
	ZGO = Self::letters_to_token(b"ZGO"),
	
	/// TODO.
	ZGP = Self::letters_to_token(b"ZGP"),
	
	/// TODO.
	ZGQ = Self::letters_to_token(b"ZGQ"),
	
	/// TODO.
	ZGR = Self::letters_to_token(b"ZGR"),
	
	/// TODO.
	ZGS = Self::letters_to_token(b"ZGS"),
	
	/// TODO.
	ZGT = Self::letters_to_token(b"ZGT"),
	
	/// TODO.
	ZGU = Self::letters_to_token(b"ZGU"),
	
	/// TODO.
	ZGV = Self::letters_to_token(b"ZGV"),
	
	/// TODO.
	ZGW = Self::letters_to_token(b"ZGW"),
	
	/// TODO.
	ZGX = Self::letters_to_token(b"ZGX"),
	
	/// TODO.
	ZGY = Self::letters_to_token(b"ZGY"),
	
	/// TODO.
	ZGZ = Self::letters_to_token(b"ZGZ"),
	
	/// TODO.
	ZHA = Self::letters_to_token(b"ZHA"),
	
	/// TODO.
	ZHB = Self::letters_to_token(b"ZHB"),
	
	/// TODO.
	ZHC = Self::letters_to_token(b"ZHC"),
	
	/// TODO.
	ZHD = Self::letters_to_token(b"ZHD"),
	
	/// TODO.
	ZHE = Self::letters_to_token(b"ZHE"),
	
	/// TODO.
	ZHF = Self::letters_to_token(b"ZHF"),
	
	/// TODO.
	ZHG = Self::letters_to_token(b"ZHG"),
	
	/// TODO.
	ZHH = Self::letters_to_token(b"ZHH"),
	
	/// TODO.
	ZHI = Self::letters_to_token(b"ZHI"),
	
	/// TODO.
	ZHJ = Self::letters_to_token(b"ZHJ"),
	
	/// TODO.
	ZHK = Self::letters_to_token(b"ZHK"),
	
	/// TODO.
	ZHL = Self::letters_to_token(b"ZHL"),
	
	/// TODO.
	ZHM = Self::letters_to_token(b"ZHM"),
	
	/// TODO.
	ZHN = Self::letters_to_token(b"ZHN"),
	
	/// TODO.
	ZHO = Self::letters_to_token(b"ZHO"),
	
	/// TODO.
	ZHP = Self::letters_to_token(b"ZHP"),
	
	/// TODO.
	ZHQ = Self::letters_to_token(b"ZHQ"),
	
	/// TODO.
	ZHR = Self::letters_to_token(b"ZHR"),
	
	/// TODO.
	ZHS = Self::letters_to_token(b"ZHS"),
	
	/// TODO.
	ZHT = Self::letters_to_token(b"ZHT"),
	
	/// TODO.
	ZHU = Self::letters_to_token(b"ZHU"),
	
	/// TODO.
	ZHV = Self::letters_to_token(b"ZHV"),
	
	/// TODO.
	ZHW = Self::letters_to_token(b"ZHW"),
	
	/// TODO.
	ZHX = Self::letters_to_token(b"ZHX"),
	
	/// TODO.
	ZHY = Self::letters_to_token(b"ZHY"),
	
	/// TODO.
	ZHZ = Self::letters_to_token(b"ZHZ"),
	
	/// TODO.
	ZIA = Self::letters_to_token(b"ZIA"),
	
	/// TODO.
	ZIB = Self::letters_to_token(b"ZIB"),
	
	/// TODO.
	ZIC = Self::letters_to_token(b"ZIC"),
	
	/// TODO.
	ZID = Self::letters_to_token(b"ZID"),
	
	/// TODO.
	ZIE = Self::letters_to_token(b"ZIE"),
	
	/// TODO.
	ZIF = Self::letters_to_token(b"ZIF"),
	
	/// TODO.
	ZIG = Self::letters_to_token(b"ZIG"),
	
	/// TODO.
	ZIH = Self::letters_to_token(b"ZIH"),
	
	/// TODO.
	ZII = Self::letters_to_token(b"ZII"),
	
	/// TODO.
	ZIJ = Self::letters_to_token(b"ZIJ"),
	
	/// TODO.
	ZIK = Self::letters_to_token(b"ZIK"),
	
	/// TODO.
	ZIL = Self::letters_to_token(b"ZIL"),
	
	/// TODO.
	ZIM = Self::letters_to_token(b"ZIM"),
	
	/// TODO.
	ZIN = Self::letters_to_token(b"ZIN"),
	
	/// TODO.
	ZIO = Self::letters_to_token(b"ZIO"),
	
	/// TODO.
	ZIP = Self::letters_to_token(b"ZIP"),
	
	/// TODO.
	ZIQ = Self::letters_to_token(b"ZIQ"),
	
	/// TODO.
	ZIR = Self::letters_to_token(b"ZIR"),
	
	/// TODO.
	ZIS = Self::letters_to_token(b"ZIS"),
	
	/// TODO.
	ZIT = Self::letters_to_token(b"ZIT"),
	
	/// TODO.
	ZIU = Self::letters_to_token(b"ZIU"),
	
	/// TODO.
	ZIV = Self::letters_to_token(b"ZIV"),
	
	/// TODO.
	ZIW = Self::letters_to_token(b"ZIW"),
	
	/// TODO.
	ZIX = Self::letters_to_token(b"ZIX"),
	
	/// TODO.
	ZIY = Self::letters_to_token(b"ZIY"),
	
	/// TODO.
	ZIZ = Self::letters_to_token(b"ZIZ"),
	
	/// TODO.
	ZJA = Self::letters_to_token(b"ZJA"),
	
	/// TODO.
	ZJB = Self::letters_to_token(b"ZJB"),
	
	/// TODO.
	ZJC = Self::letters_to_token(b"ZJC"),
	
	/// TODO.
	ZJD = Self::letters_to_token(b"ZJD"),
	
	/// TODO.
	ZJE = Self::letters_to_token(b"ZJE"),
	
	/// TODO.
	ZJF = Self::letters_to_token(b"ZJF"),
	
	/// TODO.
	ZJG = Self::letters_to_token(b"ZJG"),
	
	/// TODO.
	ZJH = Self::letters_to_token(b"ZJH"),
	
	/// TODO.
	ZJI = Self::letters_to_token(b"ZJI"),
	
	/// TODO.
	ZJJ = Self::letters_to_token(b"ZJJ"),
	
	/// TODO.
	ZJK = Self::letters_to_token(b"ZJK"),
	
	/// TODO.
	ZJL = Self::letters_to_token(b"ZJL"),
	
	/// TODO.
	ZJM = Self::letters_to_token(b"ZJM"),
	
	/// TODO.
	ZJN = Self::letters_to_token(b"ZJN"),
	
	/// TODO.
	ZJO = Self::letters_to_token(b"ZJO"),
	
	/// TODO.
	ZJP = Self::letters_to_token(b"ZJP"),
	
	/// TODO.
	ZJQ = Self::letters_to_token(b"ZJQ"),
	
	/// TODO.
	ZJR = Self::letters_to_token(b"ZJR"),
	
	/// TODO.
	ZJS = Self::letters_to_token(b"ZJS"),
	
	/// TODO.
	ZJT = Self::letters_to_token(b"ZJT"),
	
	/// TODO.
	ZJU = Self::letters_to_token(b"ZJU"),
	
	/// TODO.
	ZJV = Self::letters_to_token(b"ZJV"),
	
	/// TODO.
	ZJW = Self::letters_to_token(b"ZJW"),
	
	/// TODO.
	ZJX = Self::letters_to_token(b"ZJX"),
	
	/// TODO.
	ZJY = Self::letters_to_token(b"ZJY"),
	
	/// TODO.
	ZJZ = Self::letters_to_token(b"ZJZ"),
	
	/// TODO.
	ZKA = Self::letters_to_token(b"ZKA"),
	
	/// TODO.
	ZKB = Self::letters_to_token(b"ZKB"),
	
	/// TODO.
	ZKC = Self::letters_to_token(b"ZKC"),
	
	/// TODO.
	ZKD = Self::letters_to_token(b"ZKD"),
	
	/// TODO.
	ZKE = Self::letters_to_token(b"ZKE"),
	
	/// TODO.
	ZKF = Self::letters_to_token(b"ZKF"),
	
	/// TODO.
	ZKG = Self::letters_to_token(b"ZKG"),
	
	/// TODO.
	ZKH = Self::letters_to_token(b"ZKH"),
	
	/// TODO.
	ZKI = Self::letters_to_token(b"ZKI"),
	
	/// TODO.
	ZKJ = Self::letters_to_token(b"ZKJ"),
	
	/// TODO.
	ZKK = Self::letters_to_token(b"ZKK"),
	
	/// TODO.
	ZKL = Self::letters_to_token(b"ZKL"),
	
	/// TODO.
	ZKM = Self::letters_to_token(b"ZKM"),
	
	/// TODO.
	ZKN = Self::letters_to_token(b"ZKN"),
	
	/// TODO.
	ZKO = Self::letters_to_token(b"ZKO"),
	
	/// TODO.
	ZKP = Self::letters_to_token(b"ZKP"),
	
	/// TODO.
	ZKQ = Self::letters_to_token(b"ZKQ"),
	
	/// TODO.
	ZKR = Self::letters_to_token(b"ZKR"),
	
	/// TODO.
	ZKS = Self::letters_to_token(b"ZKS"),
	
	/// TODO.
	ZKT = Self::letters_to_token(b"ZKT"),
	
	/// TODO.
	ZKU = Self::letters_to_token(b"ZKU"),
	
	/// TODO.
	ZKV = Self::letters_to_token(b"ZKV"),
	
	/// TODO.
	ZKW = Self::letters_to_token(b"ZKW"),
	
	/// TODO.
	ZKX = Self::letters_to_token(b"ZKX"),
	
	/// TODO.
	ZKY = Self::letters_to_token(b"ZKY"),
	
	/// TODO.
	ZKZ = Self::letters_to_token(b"ZKZ"),
	
	/// TODO.
	ZLA = Self::letters_to_token(b"ZLA"),
	
	/// TODO.
	ZLB = Self::letters_to_token(b"ZLB"),
	
	/// TODO.
	ZLC = Self::letters_to_token(b"ZLC"),
	
	/// TODO.
	ZLD = Self::letters_to_token(b"ZLD"),
	
	/// TODO.
	ZLE = Self::letters_to_token(b"ZLE"),
	
	/// TODO.
	ZLF = Self::letters_to_token(b"ZLF"),
	
	/// TODO.
	ZLG = Self::letters_to_token(b"ZLG"),
	
	/// TODO.
	ZLH = Self::letters_to_token(b"ZLH"),
	
	/// TODO.
	ZLI = Self::letters_to_token(b"ZLI"),
	
	/// TODO.
	ZLJ = Self::letters_to_token(b"ZLJ"),
	
	/// TODO.
	ZLK = Self::letters_to_token(b"ZLK"),
	
	/// TODO.
	ZLL = Self::letters_to_token(b"ZLL"),
	
	/// TODO.
	ZLM = Self::letters_to_token(b"ZLM"),
	
	/// TODO.
	ZLN = Self::letters_to_token(b"ZLN"),
	
	/// TODO.
	ZLO = Self::letters_to_token(b"ZLO"),
	
	/// TODO.
	ZLP = Self::letters_to_token(b"ZLP"),
	
	/// TODO.
	ZLQ = Self::letters_to_token(b"ZLQ"),
	
	/// TODO.
	ZLR = Self::letters_to_token(b"ZLR"),
	
	/// TODO.
	ZLS = Self::letters_to_token(b"ZLS"),
	
	/// TODO.
	ZLT = Self::letters_to_token(b"ZLT"),
	
	/// TODO.
	ZLU = Self::letters_to_token(b"ZLU"),
	
	/// TODO.
	ZLV = Self::letters_to_token(b"ZLV"),
	
	/// TODO.
	ZLW = Self::letters_to_token(b"ZLW"),
	
	/// TODO.
	ZLX = Self::letters_to_token(b"ZLX"),
	
	/// TODO.
	ZLY = Self::letters_to_token(b"ZLY"),
	
	/// TODO.
	ZLZ = Self::letters_to_token(b"ZLZ"),
	
	/// TODO.
	ZMA = Self::letters_to_token(b"ZMA"),
	
	/// TODO.
	ZMB = Self::letters_to_token(b"ZMB"),
	
	/// TODO.
	ZMC = Self::letters_to_token(b"ZMC"),
	
	/// TODO.
	ZMD = Self::letters_to_token(b"ZMD"),
	
	/// TODO.
	ZME = Self::letters_to_token(b"ZME"),
	
	/// TODO.
	ZMF = Self::letters_to_token(b"ZMF"),
	
	/// TODO.
	ZMG = Self::letters_to_token(b"ZMG"),
	
	/// TODO.
	ZMH = Self::letters_to_token(b"ZMH"),
	
	/// TODO.
	ZMI = Self::letters_to_token(b"ZMI"),
	
	/// TODO.
	ZMJ = Self::letters_to_token(b"ZMJ"),
	
	/// TODO.
	ZMK = Self::letters_to_token(b"ZMK"),
	
	/// TODO.
	ZML = Self::letters_to_token(b"ZML"),
	
	/// TODO.
	ZMM = Self::letters_to_token(b"ZMM"),
	
	/// TODO.
	ZMN = Self::letters_to_token(b"ZMN"),
	
	/// TODO.
	ZMO = Self::letters_to_token(b"ZMO"),
	
	/// TODO.
	ZMP = Self::letters_to_token(b"ZMP"),
	
	/// TODO.
	ZMQ = Self::letters_to_token(b"ZMQ"),
	
	/// TODO.
	ZMR = Self::letters_to_token(b"ZMR"),
	
	/// TODO.
	ZMS = Self::letters_to_token(b"ZMS"),
	
	/// TODO.
	ZMT = Self::letters_to_token(b"ZMT"),
	
	/// TODO.
	ZMU = Self::letters_to_token(b"ZMU"),
	
	/// TODO.
	ZMV = Self::letters_to_token(b"ZMV"),
	
	/// TODO.
	ZMW = Self::letters_to_token(b"ZMW"),
	
	/// TODO.
	ZMX = Self::letters_to_token(b"ZMX"),
	
	/// TODO.
	ZMY = Self::letters_to_token(b"ZMY"),
	
	/// TODO.
	ZMZ = Self::letters_to_token(b"ZMZ"),
	
	/// TODO.
	ZNA = Self::letters_to_token(b"ZNA"),
	
	/// TODO.
	ZNB = Self::letters_to_token(b"ZNB"),
	
	/// TODO.
	ZNC = Self::letters_to_token(b"ZNC"),
	
	/// TODO.
	ZND = Self::letters_to_token(b"ZND"),
	
	/// TODO.
	ZNE = Self::letters_to_token(b"ZNE"),
	
	/// TODO.
	ZNF = Self::letters_to_token(b"ZNF"),
	
	/// TODO.
	ZNG = Self::letters_to_token(b"ZNG"),
	
	/// TODO.
	ZNH = Self::letters_to_token(b"ZNH"),
	
	/// TODO.
	ZNI = Self::letters_to_token(b"ZNI"),
	
	/// TODO.
	ZNJ = Self::letters_to_token(b"ZNJ"),
	
	/// TODO.
	ZNK = Self::letters_to_token(b"ZNK"),
	
	/// TODO.
	ZNL = Self::letters_to_token(b"ZNL"),
	
	/// TODO.
	ZNM = Self::letters_to_token(b"ZNM"),
	
	/// TODO.
	ZNN = Self::letters_to_token(b"ZNN"),
	
	/// TODO.
	ZNO = Self::letters_to_token(b"ZNO"),
	
	/// TODO.
	ZNP = Self::letters_to_token(b"ZNP"),
	
	/// TODO.
	ZNQ = Self::letters_to_token(b"ZNQ"),
	
	/// TODO.
	ZNR = Self::letters_to_token(b"ZNR"),
	
	/// TODO.
	ZNS = Self::letters_to_token(b"ZNS"),
	
	/// TODO.
	ZNT = Self::letters_to_token(b"ZNT"),
	
	/// TODO.
	ZNU = Self::letters_to_token(b"ZNU"),
	
	/// TODO.
	ZNV = Self::letters_to_token(b"ZNV"),
	
	/// TODO.
	ZNW = Self::letters_to_token(b"ZNW"),
	
	/// TODO.
	ZNX = Self::letters_to_token(b"ZNX"),
	
	/// TODO.
	ZNY = Self::letters_to_token(b"ZNY"),
	
	/// TODO.
	ZNZ = Self::letters_to_token(b"ZNZ"),
	
	/// TODO.
	ZOA = Self::letters_to_token(b"ZOA"),
	
	/// TODO.
	ZOB = Self::letters_to_token(b"ZOB"),
	
	/// TODO.
	ZOC = Self::letters_to_token(b"ZOC"),
	
	/// TODO.
	ZOD = Self::letters_to_token(b"ZOD"),
	
	/// TODO.
	ZOE = Self::letters_to_token(b"ZOE"),
	
	/// TODO.
	ZOF = Self::letters_to_token(b"ZOF"),
	
	/// TODO.
	ZOG = Self::letters_to_token(b"ZOG"),
	
	/// TODO.
	ZOH = Self::letters_to_token(b"ZOH"),
	
	/// TODO.
	ZOI = Self::letters_to_token(b"ZOI"),
	
	/// TODO.
	ZOJ = Self::letters_to_token(b"ZOJ"),
	
	/// TODO.
	ZOK = Self::letters_to_token(b"ZOK"),
	
	/// TODO.
	ZOL = Self::letters_to_token(b"ZOL"),
	
	/// TODO.
	ZOM = Self::letters_to_token(b"ZOM"),
	
	/// TODO.
	ZON = Self::letters_to_token(b"ZON"),
	
	/// TODO.
	ZOO = Self::letters_to_token(b"ZOO"),
	
	/// TODO.
	ZOP = Self::letters_to_token(b"ZOP"),
	
	/// TODO.
	ZOQ = Self::letters_to_token(b"ZOQ"),
	
	/// TODO.
	ZOR = Self::letters_to_token(b"ZOR"),
	
	/// TODO.
	ZOS = Self::letters_to_token(b"ZOS"),
	
	/// TODO.
	ZOT = Self::letters_to_token(b"ZOT"),
	
	/// TODO.
	ZOU = Self::letters_to_token(b"ZOU"),
	
	/// TODO.
	ZOV = Self::letters_to_token(b"ZOV"),
	
	/// TODO.
	ZOW = Self::letters_to_token(b"ZOW"),
	
	/// TODO.
	ZOX = Self::letters_to_token(b"ZOX"),
	
	/// TODO.
	ZOY = Self::letters_to_token(b"ZOY"),
	
	/// TODO.
	ZOZ = Self::letters_to_token(b"ZOZ"),
	
	/// TODO.
	ZPA = Self::letters_to_token(b"ZPA"),
	
	/// TODO.
	ZPB = Self::letters_to_token(b"ZPB"),
	
	/// TODO.
	ZPC = Self::letters_to_token(b"ZPC"),
	
	/// TODO.
	ZPD = Self::letters_to_token(b"ZPD"),
	
	/// TODO.
	ZPE = Self::letters_to_token(b"ZPE"),
	
	/// TODO.
	ZPF = Self::letters_to_token(b"ZPF"),
	
	/// TODO.
	ZPG = Self::letters_to_token(b"ZPG"),
	
	/// TODO.
	ZPH = Self::letters_to_token(b"ZPH"),
	
	/// TODO.
	ZPI = Self::letters_to_token(b"ZPI"),
	
	/// TODO.
	ZPJ = Self::letters_to_token(b"ZPJ"),
	
	/// TODO.
	ZPK = Self::letters_to_token(b"ZPK"),
	
	/// TODO.
	ZPL = Self::letters_to_token(b"ZPL"),
	
	/// TODO.
	ZPM = Self::letters_to_token(b"ZPM"),
	
	/// TODO.
	ZPN = Self::letters_to_token(b"ZPN"),
	
	/// TODO.
	ZPO = Self::letters_to_token(b"ZPO"),
	
	/// TODO.
	ZPP = Self::letters_to_token(b"ZPP"),
	
	/// TODO.
	ZPQ = Self::letters_to_token(b"ZPQ"),
	
	/// TODO.
	ZPR = Self::letters_to_token(b"ZPR"),
	
	/// TODO.
	ZPS = Self::letters_to_token(b"ZPS"),
	
	/// TODO.
	ZPT = Self::letters_to_token(b"ZPT"),
	
	/// TODO.
	ZPU = Self::letters_to_token(b"ZPU"),
	
	/// TODO.
	ZPV = Self::letters_to_token(b"ZPV"),
	
	/// TODO.
	ZPW = Self::letters_to_token(b"ZPW"),
	
	/// TODO.
	ZPX = Self::letters_to_token(b"ZPX"),
	
	/// TODO.
	ZPY = Self::letters_to_token(b"ZPY"),
	
	/// TODO.
	ZPZ = Self::letters_to_token(b"ZPZ"),
	
	/// TODO.
	ZQA = Self::letters_to_token(b"ZQA"),
	
	/// TODO.
	ZQB = Self::letters_to_token(b"ZQB"),
	
	/// TODO.
	ZQC = Self::letters_to_token(b"ZQC"),
	
	/// TODO.
	ZQD = Self::letters_to_token(b"ZQD"),
	
	/// TODO.
	ZQE = Self::letters_to_token(b"ZQE"),
	
	/// TODO.
	ZQF = Self::letters_to_token(b"ZQF"),
	
	/// TODO.
	ZQG = Self::letters_to_token(b"ZQG"),
	
	/// TODO.
	ZQH = Self::letters_to_token(b"ZQH"),
	
	/// TODO.
	ZQI = Self::letters_to_token(b"ZQI"),
	
	/// TODO.
	ZQJ = Self::letters_to_token(b"ZQJ"),
	
	/// TODO.
	ZQK = Self::letters_to_token(b"ZQK"),
	
	/// TODO.
	ZQL = Self::letters_to_token(b"ZQL"),
	
	/// TODO.
	ZQM = Self::letters_to_token(b"ZQM"),
	
	/// TODO.
	ZQN = Self::letters_to_token(b"ZQN"),
	
	/// TODO.
	ZQO = Self::letters_to_token(b"ZQO"),
	
	/// TODO.
	ZQP = Self::letters_to_token(b"ZQP"),
	
	/// TODO.
	ZQQ = Self::letters_to_token(b"ZQQ"),
	
	/// TODO.
	ZQR = Self::letters_to_token(b"ZQR"),
	
	/// TODO.
	ZQS = Self::letters_to_token(b"ZQS"),
	
	/// TODO.
	ZQT = Self::letters_to_token(b"ZQT"),
	
	/// TODO.
	ZQU = Self::letters_to_token(b"ZQU"),
	
	/// TODO.
	ZQV = Self::letters_to_token(b"ZQV"),
	
	/// TODO.
	ZQW = Self::letters_to_token(b"ZQW"),
	
	/// TODO.
	ZQX = Self::letters_to_token(b"ZQX"),
	
	/// TODO.
	ZQY = Self::letters_to_token(b"ZQY"),
	
	/// TODO.
	ZQZ = Self::letters_to_token(b"ZQZ"),
	
	/// TODO.
	ZRA = Self::letters_to_token(b"ZRA"),
	
	/// TODO.
	ZRB = Self::letters_to_token(b"ZRB"),
	
	/// TODO.
	ZRC = Self::letters_to_token(b"ZRC"),
	
	/// TODO.
	ZRD = Self::letters_to_token(b"ZRD"),
	
	/// TODO.
	ZRE = Self::letters_to_token(b"ZRE"),
	
	/// TODO.
	ZRF = Self::letters_to_token(b"ZRF"),
	
	/// TODO.
	ZRG = Self::letters_to_token(b"ZRG"),
	
	/// TODO.
	ZRH = Self::letters_to_token(b"ZRH"),
	
	/// TODO.
	ZRI = Self::letters_to_token(b"ZRI"),
	
	/// TODO.
	ZRJ = Self::letters_to_token(b"ZRJ"),
	
	/// TODO.
	ZRK = Self::letters_to_token(b"ZRK"),
	
	/// TODO.
	ZRL = Self::letters_to_token(b"ZRL"),
	
	/// TODO.
	ZRM = Self::letters_to_token(b"ZRM"),
	
	/// TODO.
	ZRN = Self::letters_to_token(b"ZRN"),
	
	/// TODO.
	ZRO = Self::letters_to_token(b"ZRO"),
	
	/// TODO.
	ZRP = Self::letters_to_token(b"ZRP"),
	
	/// TODO.
	ZRQ = Self::letters_to_token(b"ZRQ"),
	
	/// TODO.
	ZRR = Self::letters_to_token(b"ZRR"),
	
	/// TODO.
	ZRS = Self::letters_to_token(b"ZRS"),
	
	/// TODO.
	ZRT = Self::letters_to_token(b"ZRT"),
	
	/// TODO.
	ZRU = Self::letters_to_token(b"ZRU"),
	
	/// TODO.
	ZRV = Self::letters_to_token(b"ZRV"),
	
	/// TODO.
	ZRW = Self::letters_to_token(b"ZRW"),
	
	/// TODO.
	ZRX = Self::letters_to_token(b"ZRX"),
	
	/// TODO.
	ZRY = Self::letters_to_token(b"ZRY"),
	
	/// TODO.
	ZRZ = Self::letters_to_token(b"ZRZ"),
	
	/// TODO.
	ZSA = Self::letters_to_token(b"ZSA"),
	
	/// TODO.
	ZSB = Self::letters_to_token(b"ZSB"),
	
	/// TODO.
	ZSC = Self::letters_to_token(b"ZSC"),
	
	/// TODO.
	ZSD = Self::letters_to_token(b"ZSD"),
	
	/// TODO.
	ZSE = Self::letters_to_token(b"ZSE"),
	
	/// TODO.
	ZSF = Self::letters_to_token(b"ZSF"),
	
	/// TODO.
	ZSG = Self::letters_to_token(b"ZSG"),
	
	/// TODO.
	ZSH = Self::letters_to_token(b"ZSH"),
	
	/// TODO.
	ZSI = Self::letters_to_token(b"ZSI"),
	
	/// TODO.
	ZSJ = Self::letters_to_token(b"ZSJ"),
	
	/// TODO.
	ZSK = Self::letters_to_token(b"ZSK"),
	
	/// TODO.
	ZSL = Self::letters_to_token(b"ZSL"),
	
	/// TODO.
	ZSM = Self::letters_to_token(b"ZSM"),
	
	/// TODO.
	ZSN = Self::letters_to_token(b"ZSN"),
	
	/// TODO.
	ZSO = Self::letters_to_token(b"ZSO"),
	
	/// TODO.
	ZSP = Self::letters_to_token(b"ZSP"),
	
	/// TODO.
	ZSQ = Self::letters_to_token(b"ZSQ"),
	
	/// TODO.
	ZSR = Self::letters_to_token(b"ZSR"),
	
	/// TODO.
	ZSS = Self::letters_to_token(b"ZSS"),
	
	/// TODO.
	ZST = Self::letters_to_token(b"ZST"),
	
	/// TODO.
	ZSU = Self::letters_to_token(b"ZSU"),
	
	/// TODO.
	ZSV = Self::letters_to_token(b"ZSV"),
	
	/// TODO.
	ZSW = Self::letters_to_token(b"ZSW"),
	
	/// TODO.
	ZSX = Self::letters_to_token(b"ZSX"),
	
	/// TODO.
	ZSY = Self::letters_to_token(b"ZSY"),
	
	/// TODO.
	ZSZ = Self::letters_to_token(b"ZSZ"),
	
	/// TODO.
	ZTA = Self::letters_to_token(b"ZTA"),
	
	/// TODO.
	ZTB = Self::letters_to_token(b"ZTB"),
	
	/// TODO.
	ZTC = Self::letters_to_token(b"ZTC"),
	
	/// TODO.
	ZTD = Self::letters_to_token(b"ZTD"),
	
	/// TODO.
	ZTE = Self::letters_to_token(b"ZTE"),
	
	/// TODO.
	ZTF = Self::letters_to_token(b"ZTF"),
	
	/// TODO.
	ZTG = Self::letters_to_token(b"ZTG"),
	
	/// TODO.
	ZTH = Self::letters_to_token(b"ZTH"),
	
	/// TODO.
	ZTI = Self::letters_to_token(b"ZTI"),
	
	/// TODO.
	ZTJ = Self::letters_to_token(b"ZTJ"),
	
	/// TODO.
	ZTK = Self::letters_to_token(b"ZTK"),
	
	/// TODO.
	ZTL = Self::letters_to_token(b"ZTL"),
	
	/// TODO.
	ZTM = Self::letters_to_token(b"ZTM"),
	
	/// TODO.
	ZTN = Self::letters_to_token(b"ZTN"),
	
	/// TODO.
	ZTO = Self::letters_to_token(b"ZTO"),
	
	/// TODO.
	ZTP = Self::letters_to_token(b"ZTP"),
	
	/// TODO.
	ZTQ = Self::letters_to_token(b"ZTQ"),
	
	/// TODO.
	ZTR = Self::letters_to_token(b"ZTR"),
	
	/// TODO.
	ZTS = Self::letters_to_token(b"ZTS"),
	
	/// TODO.
	ZTT = Self::letters_to_token(b"ZTT"),
	
	/// TODO.
	ZTU = Self::letters_to_token(b"ZTU"),
	
	/// TODO.
	ZTV = Self::letters_to_token(b"ZTV"),
	
	/// TODO.
	ZTW = Self::letters_to_token(b"ZTW"),
	
	/// TODO.
	ZTX = Self::letters_to_token(b"ZTX"),
	
	/// TODO.
	ZTY = Self::letters_to_token(b"ZTY"),
	
	/// TODO.
	ZTZ = Self::letters_to_token(b"ZTZ"),
	
	/// TODO.
	ZUA = Self::letters_to_token(b"ZUA"),
	
	/// TODO.
	ZUB = Self::letters_to_token(b"ZUB"),
	
	/// TODO.
	ZUC = Self::letters_to_token(b"ZUC"),
	
	/// TODO.
	ZUD = Self::letters_to_token(b"ZUD"),
	
	/// TODO.
	ZUE = Self::letters_to_token(b"ZUE"),
	
	/// TODO.
	ZUF = Self::letters_to_token(b"ZUF"),
	
	/// TODO.
	ZUG = Self::letters_to_token(b"ZUG"),
	
	/// TODO.
	ZUH = Self::letters_to_token(b"ZUH"),
	
	/// TODO.
	ZUI = Self::letters_to_token(b"ZUI"),
	
	/// TODO.
	ZUJ = Self::letters_to_token(b"ZUJ"),
	
	/// TODO.
	ZUK = Self::letters_to_token(b"ZUK"),
	
	/// TODO.
	ZUL = Self::letters_to_token(b"ZUL"),
	
	/// TODO.
	ZUM = Self::letters_to_token(b"ZUM"),
	
	/// TODO.
	ZUN = Self::letters_to_token(b"ZUN"),
	
	/// TODO.
	ZUO = Self::letters_to_token(b"ZUO"),
	
	/// TODO.
	ZUP = Self::letters_to_token(b"ZUP"),
	
	/// TODO.
	ZUQ = Self::letters_to_token(b"ZUQ"),
	
	/// TODO.
	ZUR = Self::letters_to_token(b"ZUR"),
	
	/// TODO.
	ZUS = Self::letters_to_token(b"ZUS"),
	
	/// TODO.
	ZUT = Self::letters_to_token(b"ZUT"),
	
	/// TODO.
	ZUU = Self::letters_to_token(b"ZUU"),
	
	/// TODO.
	ZUV = Self::letters_to_token(b"ZUV"),
	
	/// TODO.
	ZUW = Self::letters_to_token(b"ZUW"),
	
	/// TODO.
	ZUX = Self::letters_to_token(b"ZUX"),
	
	/// TODO.
	ZUY = Self::letters_to_token(b"ZUY"),
	
	/// TODO.
	ZUZ = Self::letters_to_token(b"ZUZ"),
	
	/// TODO.
	ZVA = Self::letters_to_token(b"ZVA"),
	
	/// TODO.
	ZVB = Self::letters_to_token(b"ZVB"),
	
	/// TODO.
	ZVC = Self::letters_to_token(b"ZVC"),
	
	/// TODO.
	ZVD = Self::letters_to_token(b"ZVD"),
	
	/// TODO.
	ZVE = Self::letters_to_token(b"ZVE"),
	
	/// TODO.
	ZVF = Self::letters_to_token(b"ZVF"),
	
	/// TODO.
	ZVG = Self::letters_to_token(b"ZVG"),
	
	/// TODO.
	ZVH = Self::letters_to_token(b"ZVH"),
	
	/// TODO.
	ZVI = Self::letters_to_token(b"ZVI"),
	
	/// TODO.
	ZVJ = Self::letters_to_token(b"ZVJ"),
	
	/// TODO.
	ZVK = Self::letters_to_token(b"ZVK"),
	
	/// TODO.
	ZVL = Self::letters_to_token(b"ZVL"),
	
	/// TODO.
	ZVM = Self::letters_to_token(b"ZVM"),
	
	/// TODO.
	ZVN = Self::letters_to_token(b"ZVN"),
	
	/// TODO.
	ZVO = Self::letters_to_token(b"ZVO"),
	
	/// TODO.
	ZVP = Self::letters_to_token(b"ZVP"),
	
	/// TODO.
	ZVQ = Self::letters_to_token(b"ZVQ"),
	
	/// TODO.
	ZVR = Self::letters_to_token(b"ZVR"),
	
	/// TODO.
	ZVS = Self::letters_to_token(b"ZVS"),
	
	/// TODO.
	ZVT = Self::letters_to_token(b"ZVT"),
	
	/// TODO.
	ZVU = Self::letters_to_token(b"ZVU"),
	
	/// TODO.
	ZVV = Self::letters_to_token(b"ZVV"),
	
	/// TODO.
	ZVW = Self::letters_to_token(b"ZVW"),
	
	/// TODO.
	ZVX = Self::letters_to_token(b"ZVX"),
	
	/// TODO.
	ZVY = Self::letters_to_token(b"ZVY"),
	
	/// TODO.
	ZVZ = Self::letters_to_token(b"ZVZ"),
	
	/// TODO.
	ZWA = Self::letters_to_token(b"ZWA"),
	
	/// TODO.
	ZWB = Self::letters_to_token(b"ZWB"),
	
	/// TODO.
	ZWC = Self::letters_to_token(b"ZWC"),
	
	/// TODO.
	ZWD = Self::letters_to_token(b"ZWD"),
	
	/// TODO.
	ZWE = Self::letters_to_token(b"ZWE"),
	
	/// TODO.
	ZWF = Self::letters_to_token(b"ZWF"),
	
	/// TODO.
	ZWG = Self::letters_to_token(b"ZWG"),
	
	/// TODO.
	ZWH = Self::letters_to_token(b"ZWH"),
	
	/// TODO.
	ZWI = Self::letters_to_token(b"ZWI"),
	
	/// TODO.
	ZWJ = Self::letters_to_token(b"ZWJ"),
	
	/// TODO.
	ZWK = Self::letters_to_token(b"ZWK"),
	
	/// TODO.
	ZWL = Self::letters_to_token(b"ZWL"),
	
	/// TODO.
	ZWM = Self::letters_to_token(b"ZWM"),
	
	/// TODO.
	ZWN = Self::letters_to_token(b"ZWN"),
	
	/// TODO.
	ZWO = Self::letters_to_token(b"ZWO"),
	
	/// TODO.
	ZWP = Self::letters_to_token(b"ZWP"),
	
	/// TODO.
	ZWQ = Self::letters_to_token(b"ZWQ"),
	
	/// TODO.
	ZWR = Self::letters_to_token(b"ZWR"),
	
	/// TODO.
	ZWS = Self::letters_to_token(b"ZWS"),
	
	/// TODO.
	ZWT = Self::letters_to_token(b"ZWT"),
	
	/// TODO.
	ZWU = Self::letters_to_token(b"ZWU"),
	
	/// TODO.
	ZWV = Self::letters_to_token(b"ZWV"),
	
	/// TODO.
	ZWW = Self::letters_to_token(b"ZWW"),
	
	/// TODO.
	ZWX = Self::letters_to_token(b"ZWX"),
	
	/// TODO.
	ZWY = Self::letters_to_token(b"ZWY"),
	
	/// TODO.
	ZWZ = Self::letters_to_token(b"ZWZ"),
	
	/// TODO.
	ZXA = Self::letters_to_token(b"ZXA"),
	
	/// TODO.
	ZXB = Self::letters_to_token(b"ZXB"),
	
	/// TODO.
	ZXC = Self::letters_to_token(b"ZXC"),
	
	/// TODO.
	ZXD = Self::letters_to_token(b"ZXD"),
	
	/// TODO.
	ZXE = Self::letters_to_token(b"ZXE"),
	
	/// TODO.
	ZXF = Self::letters_to_token(b"ZXF"),
	
	/// TODO.
	ZXG = Self::letters_to_token(b"ZXG"),
	
	/// TODO.
	ZXH = Self::letters_to_token(b"ZXH"),
	
	/// TODO.
	ZXI = Self::letters_to_token(b"ZXI"),
	
	/// TODO.
	ZXJ = Self::letters_to_token(b"ZXJ"),
	
	/// TODO.
	ZXK = Self::letters_to_token(b"ZXK"),
	
	/// TODO.
	ZXL = Self::letters_to_token(b"ZXL"),
	
	/// TODO.
	ZXM = Self::letters_to_token(b"ZXM"),
	
	/// TODO.
	ZXN = Self::letters_to_token(b"ZXN"),
	
	/// TODO.
	ZXO = Self::letters_to_token(b"ZXO"),
	
	/// TODO.
	ZXP = Self::letters_to_token(b"ZXP"),
	
	/// TODO.
	ZXQ = Self::letters_to_token(b"ZXQ"),
	
	/// TODO.
	ZXR = Self::letters_to_token(b"ZXR"),
	
	/// TODO.
	ZXS = Self::letters_to_token(b"ZXS"),
	
	/// TODO.
	ZXT = Self::letters_to_token(b"ZXT"),
	
	/// TODO.
	ZXU = Self::letters_to_token(b"ZXU"),
	
	/// TODO.
	ZXV = Self::letters_to_token(b"ZXV"),
	
	/// TODO.
	ZXW = Self::letters_to_token(b"ZXW"),
	
	/// TODO.
	ZXX = Self::letters_to_token(b"ZXX"),
	
	/// TODO.
	ZXY = Self::letters_to_token(b"ZXY"),
	
	/// TODO.
	ZXZ = Self::letters_to_token(b"ZXZ"),
	
	/// TODO.
	ZYA = Self::letters_to_token(b"ZYA"),
	
	/// TODO.
	ZYB = Self::letters_to_token(b"ZYB"),
	
	/// TODO.
	ZYC = Self::letters_to_token(b"ZYC"),
	
	/// TODO.
	ZYD = Self::letters_to_token(b"ZYD"),
	
	/// TODO.
	ZYE = Self::letters_to_token(b"ZYE"),
	
	/// TODO.
	ZYF = Self::letters_to_token(b"ZYF"),
	
	/// TODO.
	ZYG = Self::letters_to_token(b"ZYG"),
	
	/// TODO.
	ZYH = Self::letters_to_token(b"ZYH"),
	
	/// TODO.
	ZYI = Self::letters_to_token(b"ZYI"),
	
	/// TODO.
	ZYJ = Self::letters_to_token(b"ZYJ"),
	
	/// TODO.
	ZYK = Self::letters_to_token(b"ZYK"),
	
	/// TODO.
	ZYL = Self::letters_to_token(b"ZYL"),
	
	/// TODO.
	ZYM = Self::letters_to_token(b"ZYM"),
	
	/// TODO.
	ZYN = Self::letters_to_token(b"ZYN"),
	
	/// TODO.
	ZYO = Self::letters_to_token(b"ZYO"),
	
	/// TODO.
	ZYP = Self::letters_to_token(b"ZYP"),
	
	/// TODO.
	ZYQ = Self::letters_to_token(b"ZYQ"),
	
	/// TODO.
	ZYR = Self::letters_to_token(b"ZYR"),
	
	/// TODO.
	ZYS = Self::letters_to_token(b"ZYS"),
	
	/// TODO.
	ZYT = Self::letters_to_token(b"ZYT"),
	
	/// TODO.
	ZYU = Self::letters_to_token(b"ZYU"),
	
	/// TODO.
	ZYV = Self::letters_to_token(b"ZYV"),
	
	/// TODO.
	ZYW = Self::letters_to_token(b"ZYW"),
	
	/// TODO.
	ZYX = Self::letters_to_token(b"ZYX"),
	
	/// TODO.
	ZYY = Self::letters_to_token(b"ZYY"),
	
	/// TODO.
	ZYZ = Self::letters_to_token(b"ZYZ"),
	
	/// User assigned.
	ZZA = Self::letters_to_token(b"ZZA"),
	
	/// User assigned.
	ZZB = Self::letters_to_token(b"ZZB"),
	
	/// User assigned.
	ZZC = Self::letters_to_token(b"ZZC"),
	
	/// User assigned.
	ZZD = Self::letters_to_token(b"ZZD"),
	
	/// User assigned.
	ZZE = Self::letters_to_token(b"ZZE"),
	
	/// User assigned.
	ZZF = Self::letters_to_token(b"ZZF"),
	
	/// User assigned.
	ZZG = Self::letters_to_token(b"ZZG"),
	
	/// User assigned.
	ZZH = Self::letters_to_token(b"ZZH"),
	
	/// User assigned.
	ZZI = Self::letters_to_token(b"ZZI"),
	
	/// User assigned.
	ZZJ = Self::letters_to_token(b"ZZJ"),
	
	/// User assigned.
	ZZK = Self::letters_to_token(b"ZZK"),
	
	/// User assigned.
	ZZL = Self::letters_to_token(b"ZZL"),
	
	/// User assigned.
	ZZM = Self::letters_to_token(b"ZZM"),
	
	/// User assigned.
	ZZN = Self::letters_to_token(b"ZZN"),
	
	/// User assigned.
	ZZO = Self::letters_to_token(b"ZZO"),
	
	/// User assigned.
	ZZP = Self::letters_to_token(b"ZZP"),
	
	/// User assigned.
	ZZQ = Self::letters_to_token(b"ZZQ"),
	
	/// User assigned.
	ZZR = Self::letters_to_token(b"ZZR"),
	
	/// User assigned.
	ZZS = Self::letters_to_token(b"ZZS"),
	
	/// User assigned.
	ZZT = Self::letters_to_token(b"ZZT"),
	
	/// User assigned.
	ZZU = Self::letters_to_token(b"ZZU"),
	
	/// User assigned.
	ZZV = Self::letters_to_token(b"ZZV"),
	
	/// User assigned.
	ZZW = Self::letters_to_token(b"ZZW"),
	
	/// User assigned.
	ZZX = Self::letters_to_token(b"ZZX"),
	
	/// User assigned.
	ZZY = Self::letters_to_token(b"ZZY"),
	
	/// User assigned.
	ZZZ = Self::letters_to_token(b"ZZZ"),
}

impl Iso3166Dash1Alpha3CountryCode
{
	/// Returns if exceptionally reserved and if has been deleted.
	#[inline(always)]
	pub const fn is_exceptionally_reserved(self) -> (bool, bool)
	{
		match self
		{
			ASC => (true, false),
			
			CPT => (true, false),
			
			DGA => (true, false),
			
			FXX => (true, true),
			
			SUN => (true, true),
			
			TAA => (true, false),
			
			_ => (false, false)
		}
	}
	
	#[inline(always)]
	const fn letters_to_token(letters: &[u8; 3]) -> u16
	{
		Iso3166Dash1AlphaCountryCode::letter_to_number_unchecked::<0, _>(letters) + Iso3166Dash1AlphaCountryCode::letter_to_number_unchecked::<1, _>(letters) + Iso3166Dash1AlphaCountryCode::letter_to_number_unchecked::<2, _>(letters)
	}
}
