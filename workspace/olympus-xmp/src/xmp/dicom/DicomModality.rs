// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Type of equipment that originally acquired the data used to create the images in a Series.
///
/// See Section C.7.3.1.1.1 for Defined Terms.
///
/// This is a Digital Image Communications in Medicine (DICOM) Code String (CS), so strictly speaking:-
///
/// * Leading and trailing 0x20 (space) should be stripped.
/// * Valid bytes are A-Z, 0-9, space and underscore.
/// * Maximum length is 16 bytes.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum DicomModality
{
	/// Autorefraction.
	AR,
	
	/// Content Assessment Results.
	ASMT,
	
	/// Audio.
	AU,
	
	/// Bone Densitometry (ultrasound).
	BDUS,
	
	/// Biomagnetic imaging.
	BI,
	
	/// Bone Densitometry (X-Ray).
	BMD,
	
	/// Computed Radiography.
	CR,
	
	/// Computed Tomography.
	CT,
	
	/// CT Protocol (Performed).
	CTPROTOCOL,
	
	/// Diaphanography.
	DG,
	
	/// Document.
	DOC,
	
	/// Digital Radiography.
	DX,
	
	/// Electrocardiography.
	ECG,
	
	/// Cardiac Electrophysiology.
	EPS,
	
	/// Endoscopy.
	ES,
	
	/// Fiducials.
	FID,
	
	/// General Microscopy.
	GM,
	
	/// Hard Copy.
	HC,
	
	/// Hemodynamic Waveform.
	HD,
	
	/// Intra-Oral Radiography.
	IO,
	
	/// Intraocular Lens Data.
	IOL,
	
	/// Intravascular Optical Coherence Tomography.
	IVOCT,
	
	/// Intravascular Ultrasound.
	IVUS,
	
	/// Keratometry.
	KER,
	
	/// Key Object Selection.
	KO,
	
	/// Lensometry.
	LEN,
	
	/// Laser surface scan.
	LS,
	
	/// Mammography.
	MG,
	
	/// Magnetic Resonance.
	///
	/// The `MR` modality incorporates the retired modalities `MA` and `MS`.
	MR,
	
	/// Model for 3D Manufacturing.
	M3D,
	
	/// Nuclear Medicine.
	///
	/// The `NM` modality incorporates the retired modality `ST`.
	NM,
	
	/// Ophthalmic Axial Measurements.
	OAM,
	
	/// Optical Coherence Tomography (non-Ophthalmic).
	OCT,
	
	/// Ophthalmic Photography.
	OP,
	
	/// Ophthalmic Mapping.
	OPM,
	
	/// Ophthalmic Tomography.
	OPT,
	
	/// Ophthalmic Tomography B-scan Volume Analysis.
	OPTBSV,
	
	/// Ophthalmic Tomography En Face.
	OPTENF,
	
	/// Ophthalmic Visual Field.
	OPV,
	
	/// Optical Surface Scan.
	OSS,
	
	/// Other.
	OT,
	
	/// Plan.
	PLAN,
	
	/// Presentation State.
	PR,
	
	/// Positron emission tomography (PET).
	PT,
	
	/// Panoramic X-Ray.
	PX,
	
	/// Registration.
	REG,
	
	/// Respiratory Waveform.
	RESP,
	
	/// Radio Fluoroscopy.
	///
	/// The `RF` modality incorporates the retired modalities `CF`, `DF` and `VF`.
	RF,
	
	/// Radiographic imaging (conventional film / screen).
	RG,
	
	/// Radiotherapy Dose.
	RTDOSE,
	
	/// Radiotherapy Image.
	RTIMAGE,
	
	/// Radiotherapy Intent.
	RTINTENT,
	
	/// Radiotherapy Plan.
	RTPLAN,
	
	/// RT Radiation.
	RTRAD,
	
	/// RT Treatment Record.
	RTRECORD,
	
	/// Radiotherapy Segment Annotation.
	RTSEGANN,
	
	/// Radiotherapy Structure Set.
	RTSTRUCT,
	
	/// Real World Value Map.
	RWV,
	
	/// Segmentation.
	SEG,
	
	/// Slide Microscopy.
	SM,
	
	/// Stereometric Relationship.
	SMR,
	
	/// SR Document.
	SR,
	
	/// Subjective Refraction.
	SRF,
	
	/// Automated Slide Stainer.
	STAIN,
	
	/// Texture Map.
	TEXTUREMAP,
	
	/// Thermography.
	TG,
	
	/// Ultrasound.
	///
	/// The `US` modality incorporates the retired modalities `EC`, `CD`, and `DD`.
	US,
	
	/// Visual Acuity.
	VA,
	
	/// X-Ray Angiography.
	///
	/// The `XA` modality incorporates the retired modality `DS`.
	XA,
	
	/// External-camera Photography.
	XC,
	
	/// Angioscopy.
	///
	/// (Retired).
	AS,
	
	/// Color flow Doppler.
	///
	/// (Retired).
	CD,
	
	/// Cinefluorography.
	///
	/// (Retired).
	CF,
	
	/// Culposcopy.
	///
	/// (Retired).
	CP,
	
	/// Cystoscopy.
	///
	/// (Retired).
	CS,
	
	/// Duplex Doppler.
	///
	/// (Retired).
	DD,
	
	/// Digital fluoroscopy.
	///
	/// (Retired).
	DF,
	
	/// Digital microscopy.
	///
	/// (Retired).
	DM,
	
	/// Digital Subtraction Angiography.
	///
	/// (Retired).
	DS,
	
	/// Echocardiography.
	///
	/// (Retired).
	EC,
	
	/// Fluorescein angiography.
	///
	/// (Retired).
	FA,
	
	/// Fundoscopy.
	///
	/// (Retired).
	FS,
	
	/// Laparoscopy.
	///
	/// (Retired).
	LP,
	
	/// Magnetic resonance angiography.
	///
	/// (Retired).
	MA,
	
	/// Magnetic resonance spectroscopy.
	///
	/// (Retired).
	MS,
	
	/// Ophthalmic Refraction.
	///
	/// (Retired).
	OPR,
	
	/// Single-photon emission computed tomography (SPECT).
	///
	/// (Retired).
	ST,
	
	/// Videofluorography.
	///
	/// (Retired).
	VF,
}

impl DicomModality
{
	/// Is retired?
	#[inline(always)]
	pub fn is_retired(self) -> bool
	{
		use DicomModality::*;
		match self
		{
			AS => true,
			
			CD => true,
			
			CF => true,
			
			CP => true,
			
			CS => true,
			
			DD => true,
			
			DF => true,
			
			DM => true,
			
			DS => true,
			
			EC => true,
			
			FA => true,
			
			FS => true,
			
			LP => true,
			
			MA => true,
			
			MS => true,
			
			OPR => true,
			
			ST => true,
			
			VF => true,
			
			_ => false,
		}
	}
}

impl_xmp_attribute_value_parse_str!
(
	DicomModality, DicomModality,
	
	"AR" => AR,

	"ASMT" => ASMT,

	"AU" => AU,

	"BDUS" => BDUS,

	"BI" => BI,

	"BMD" => BMD,

	"CR" => CR,

	"CT" => CT,

	"CTPROTOCOL" => CTPROTOCOL,

	"DG" => DG,

	"DOC" => DOC,

	"DX" => DX,

	"ECG" => ECG,

	"EPS" => EPS,

	"ES" => ES,

	"FID" => FID,

	"GM" => GM,

	"HC" => HC,

	"HD" => HD,

	"IO" => IO,

	"IOL" => IOL,

	"IVOCT" => IVOCT,

	"IVUS" => IVUS,

	"KER" => KER,

	"KO" => KO,

	"LEN" => LEN,

	"LS" => LS,

	"MG" => MG,

	"MR" => MR,

	"M3D" => M3D,

	"NM" => NM,

	"OAM" => OAM,

	"OCT" => OCT,

	"OP" => OP,

	"OPM" => OPM,

	"OPT" => OPT,

	"OPTBSV" => OPTBSV,

	"OPTENF" => OPTENF,

	"OPV" => OPV,

	"OSS" => OSS,

	"OT" => OT,

	"PLAN" => PLAN,

	"PR" => PR,

	"PT" => PT,

	"PX" => PX,

	"REG" => REG,

	"RESP" => RESP,

	"RF" => RF,

	"RG" => RG,

	"RTDOSE" => RTDOSE,

	"RTIMAGE" => RTIMAGE,

	"RTINTENT" => RTINTENT,

	"RTPLAN" => RTPLAN,

	"RTRAD" => RTRAD,

	"RTRECORD" => RTRECORD,

	"RTSEGANN" => RTSEGANN,

	"RTSTRUCT" => RTSTRUCT,

	"RWV" => RWV,

	"SEG" => SEG,

	"SM" => SM,

	"SMR" => SMR,

	"SR" => SR,

	"SRF" => SRF,

	"STAIN" => STAIN,

	"TEXTUREMAP" => TEXTUREMAP,

	"TG" => TG,

	"US" => US,

	"VA" => VA,

	"XA" => XA,

	"XC" => XC,

	"AS" => AS,

	"CD" => CD,

	"CF" => CF,

	"CP" => CP,

	"CS" => CS,

	"DD" => DD,

	"DF" => DF,

	"DM" => DM,

	"DS" => DS,

	"EC" => EC,

	"FA" => FA,

	"FS" => FS,

	"LP" => LP,

	"MA" => MA,

	"MS" => MS,

	"OPR" => OPR,

	"ST" => ST,

	"VF" => VF,
);
