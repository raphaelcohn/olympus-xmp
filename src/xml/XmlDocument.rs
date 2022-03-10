// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// XML document.
#[derive(Clone, PartialEq, Eq)]
pub struct XmlDocument
{
	version: XmlVersion,
	
	encoding: String,
	
	standalone: Option<bool>,

	processing_instructions_before_root: XmlProcessingInstructions,

	root: XmlElement,
	
	processing_instructions_after_root: XmlProcessingInstructions,
}

impl XmlDocumentOrXmlElement for XmlDocument
{
	#[inline(always)]
	fn get_attribute<'a, 'namespace, 'local_name>(&'a self, attribute_name: &'a XmlName<'namespace, 'local_name>) -> Option<&'a str>
	where 'a: 'namespace, 'a: 'local_name
	{
		self.root.get_attribute(attribute_name)
	}
	
	#[inline(always)]
	fn get_only_element<'a>(&'a self, path: &[XmlName]) -> Result<&'a XmlElement, NotExactlyOneElementError>
	{
		self.root.get_only_element(path)
	}
	
	#[inline(always)]
	fn get_elements<'a>(&'a self, path: &[XmlName]) -> Vec<&'a XmlElement>
	{
		self.root.get_elements(path)
	}
}

impl XmlDocument
{
	/// Root element.
	#[inline(always)]
	pub fn root(&self) -> &XmlElement
	{
		&self.root
	}
	
	/// Write (to a path).
	#[inline(always)]
	pub fn write_path<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error>
	{
		let file = OpenOptions::new().write(true).open(path)?;
		self.write_file(file)
	}
	
	/// Write (to a file).
	#[inline(always)]
	pub fn write_file(&self, file: File) -> Result<(), io::Error>
	{
		self.write(BufWriter::new(file))
	}
	
	/// Write (to a writer).
	pub fn write(&self, sink: impl Write) -> Result<(), io::Error>
	{
		const EmitterConfiguration: EmitterConfig = EmitterConfig
		{
			line_separator: Borrowed(""),
			indent_string: Borrowed(""),
			perform_indent: false,
			perform_escaping: true,
			write_document_declaration: true,
			normalize_empty_elements: true,
			cdata_to_characters: false,
			keep_element_names_stack: true,
			autopad_comments: false,
			pad_self_closing: false
		};
		
		let mut event_writer = EmitterConfiguration.create_writer(sink);
		
		event_writer.write_robustly(XmlWriteEvent::StartDocument { version: self.version, encoding: None, standalone: self.standalone })?;
		self.processing_instructions_before_root.write(&mut event_writer)?;
		self.root.write(&mut event_writer)?;
		self.processing_instructions_after_root.write(&mut event_writer)
	}
	
	/// Parse (from a path).
	#[inline(always)]
	pub fn parse_path<P: AsRef<Path>>(path: P, writable: bool) -> Result<(Self, File), ParseError>
	{
		match OpenOptions::new().read(true).write(writable).open(path.as_ref())
		{
			Err(io_error) => Err(ParseError::Read(XmlReadError::from(io_error))),
			
			Ok(file) => Self::parse_file(file)
		}
	}
	
	/// Parse (from a file).
	#[inline(always)]
	pub fn parse_file(file: File) -> Result<(Self, File), ParseError>
	{
		let (this, buf_reader) = Self::parse(BufReader::new(file))?;
		Ok((this, buf_reader.into_inner()))
	}
	
	/// Parse (from a reader).
	pub fn parse<R: Read>(source: R) -> Result<(Self, R), ParseError>
	{
		let parse_configuration = ParserConfig
		{
			trim_whitespace: true,
			whitespace_to_characters: true,
			cdata_to_characters: true,
			ignore_comments: true,
			coalesce_characters: true,
			extra_entities: HashMap::new(),
			ignore_end_of_stream: false,
			replace_unknown_entity_references: false,
			ignore_root_level_whitespace: true,
		};
		let mut event_reader = parse_configuration.create_reader(source);
		
		let (version, encoding, standalone) = match event_reader.next()?
		{
			StartDocument { version, encoding, standalone } => (version, encoding, standalone),
			
			CData(_) | Comment(_) | Whitespace(_) => unreachable_cdata_comments_or_whitespace(),
			
			_ => unreachable_only_start_document(),
		};
		
		let mut processing_instructions_before_root = XmlProcessingInstructions::new();
		let root = loop
		{
			match event_reader.next()?
			{
				ProcessingInstruction { name, data } => processing_instructions_before_root.push(name, data),
				
				StartElement { name, attributes, namespace } => break XmlElement::parse(&mut event_reader, namespace, name, attributes)?,
				
				EndDocument => return Err(ParseError::EndDocumentWithoutRootNode(event_reader.position())),
				
				Characters(text) => return Err(ParseError::TextBeforeRoot(event_reader.position(), text)),
				
				StartDocument { .. } => unreachable_start_document(),
				
				EndElement { .. } => unreachable_end_element(),
				
				CData(_) | Comment(_) | Whitespace(_) => unreachable_cdata_comments_or_whitespace(),
			}
		};
		
		let mut processing_instructions_after_root = XmlProcessingInstructions::new();
		loop
		{
			match event_reader.next()?
			{
				ProcessingInstruction { name, data } => processing_instructions_after_root.push(name, data),
				
				EndDocument => break,
				
				StartElement { name, attributes, namespace } => return Err(ParseError::StartElementAfterRoot(event_reader.position(), XmlElementCommon::new(namespace, name, attributes, event_reader.position())?)),
				
				Characters(text) => return Err(ParseError::TextAfterRoot(event_reader.position(), text)),
				
				StartDocument { .. } => unreachable_start_document(),
				
				EndElement { .. } => unreachable_end_element(),
				
				CData(_) | Comment(_) | Whitespace(_) => unreachable_cdata_comments_or_whitespace(),
			}
		};
		
		Ok
		(
			(
				Self
				{
					version,
					encoding,
					standalone,
					processing_instructions_before_root,
					root,
					processing_instructions_after_root,
				},
				event_reader.into_inner()
			)
		)
	}
}
