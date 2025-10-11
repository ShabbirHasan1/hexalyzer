use std::io;
use std::io::Write;
use std::fs;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;


mod ranges {
    use std::ops::Range;
    pub const RECORD_LEN_RANGE: Range<usize> = 1..3;
    pub const RECORD_ADDR_RANGE: Range<usize> = 3..7;
    pub const RECORD_TYPE_RANGE: Range<usize> = 7..9;
}

mod sizes {
    pub const BYTE_CHAR_LEN: usize = 2;
    pub const SMALLEST_RECORD: usize = (1 + 2 + 1 + 1) * 2; // len + addr + rtype + checksum
    pub const LARGEST_RECORD: usize = SMALLEST_RECORD + 255 * 2;
}

#[derive(Debug)]
pub enum IntelHexError {
    /// Record does not begin with a ':'
    MissingStartCode,
    /// Record contains non-hexadecimal characters
    ContainsInvalidCharacters,
    /// Record is shorter than the smallest valid
    RecordTooShort,
    /// Record is longer than the largest valid
    RecordTooLong,
    /// Record's payload length does not match the record type
    RecordLengthInvalidForType(RecordType, usize, usize),
    /// Record's address does not match the record type
    RecordAddressInvalidForType(RecordType, usize, usize),
    RecordNotSupported,
    /// Record length is odd
    RecordNotEvenLength,
    /// Record checksum mismatch
    RecordChecksumMismatch(u8, u8),
    /// Invalid checksum length
    RecordInvalidChecksumLength,
    /// Encountered address that already contains data
    RecordAddressOverlap(usize),
    /// Provided record type does not exist
    InvalidRecordType,
    ///
    InvalidAddress(usize),
    /// Encountered second start address record
    DuplicateStartAddress,
}

impl fmt::Display for IntelHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntelHexError::MissingStartCode => write!(f, "Missing start code ':'"),
            IntelHexError::ContainsInvalidCharacters => write!(f, "Record contains invalid characters"),
            IntelHexError::RecordTooShort => write!(f, "Record too short"),
            IntelHexError::RecordTooLong => write!(f, "Record too long"),
            IntelHexError::RecordLengthInvalidForType(rtype, expected, actual) => {
                write!(f, "For record type {rtype:?} expected data length is {expected} bytes, encountered {actual}")
            }
            IntelHexError::RecordAddressInvalidForType(rtype, expected, actual) => {
                write!(f, "For record type {rtype:?} expected address is {expected}, encountered {actual}")
            }
            IntelHexError::RecordAddressOverlap(address) => {
                write!(f, "Encountered data at the address {address} already used by another record")
            }
            IntelHexError::InvalidRecordType => write!(f, "Invalid record type"),
            IntelHexError::RecordChecksumMismatch(expected, actual) => {
                write!(f, "Invalid record checksum, expected: {expected}, actual: {actual}")
            },
            IntelHexError::RecordInvalidChecksumLength => {
                write!(f, "Record checksum length is invalid or additional byte(s) present between data end and checksum")
            },
            IntelHexError::RecordNotEvenLength => {
                write!(f, "Record with uneven length")
            }
            IntelHexError::RecordNotSupported => write!(f, "Record not supported"),
            IntelHexError::InvalidAddress(address) => {
                write!(f, "No data found at address {address}")
            },
            IntelHexError::DuplicateStartAddress => {
                write!(f, "Encountered second start address")
            }
        }
    }
}

impl Error for IntelHexError {}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RecordType {
    Data = 0x0,
    EndOfFile = 0x1,
    ExtendedSegmentAddress = 0x2,
    StartSegmentAddress = 0x3, // TODO: deprecate? Or allow to write with it?
    ExtendedLinearAddress = 0x4,
    StartLinearAddress = 0x5,
}

impl RecordType {
    fn parse(s: &str) -> Result<Self, IntelHexError> {
        match s {
            "00" => Ok(Self::Data),
            "01" => Ok(Self::EndOfFile),
            "02" => Ok(Self::ExtendedSegmentAddress),
            "03" => Ok(Self::StartSegmentAddress),
            "04" => Ok(Self::ExtendedLinearAddress),
            "05" => Ok(Self::StartLinearAddress),
            _ => Err(IntelHexError::InvalidRecordType),
        }
    }
}


#[derive(Debug)]
struct Record {
    length: u8,
    address: u16,
    rtype: RecordType,
    data: Vec<u8>,
    checksum: u8,
}

impl Record {
    /// Calculate checksum from the Record instance.
    ///
    fn calculate_checksum_from_self(&self) -> u8 {
        // Get length, address and record type byte data
        let length = self.length as usize;
        let addr_high_byte = (self.address >> 8) as usize;
        let addr_low_byte = (self.address & 0xFF) as usize;
        let rtype = self.rtype as usize;

        // Sum it up with data vector
        let mut sum: usize = length + addr_high_byte + addr_low_byte + rtype;

        for b in &self.data {
            sum = sum.add(*b as usize);
        }
        let checksum = (!sum as u8).wrapping_add(1); // two's complement
        checksum
    }

    /// Calculate checksum from u8 array.
    ///
    fn calculate_checksum(data: &[u8]) -> u8 {
        let mut sum: u8 = 0;
        for b in data {
            sum = sum.wrapping_add(*b);
        }
        let checksum = (!sum).wrapping_add(1); // two's complement
        checksum
    }

    /// Create the record string from address, type and data vector.
    ///
    fn create(address: u16, rtype: RecordType, data: &Vec<u8>) -> Result<String, IntelHexError> {
        // Get length of payload data
        let length = data.len();

        // Create a vector of data for checksum calculation
        let mut v = vec![length as u8, (address >> 8) as u8, (address & 0xFF) as u8];
        v.extend_from_slice(&data);

        // Checksum
        let checksum = Self::calculate_checksum(&v);

        match rtype {
            RecordType::Data => {
                // Check for data length
                if length > u8::MAX as usize {
                    return Err(IntelHexError::RecordTooLong);
                }

                // Create record string
                let record = format!(
                    ":{:02X}{:04X}00{}{:02X}",
                    length,
                    address,
                    data.iter().map(|b| format!("{:02X}", b)).collect::<String>(),
                    checksum
                );

                Ok(record)
            }
            RecordType::EndOfFile => {
                Ok(String::from(":00000001FF"))
            }
            RecordType::ExtendedLinearAddress => {
                // Check for data length (has to be 1 byte)
                if length != 2 {
                    return Err(IntelHexError::RecordLengthInvalidForType(rtype, 2, length));
                }

                // Check for address (has to be 0x0)
                if address != 0 {
                    return Err(IntelHexError::RecordAddressInvalidForType(rtype, 0, address as usize));
                }

                // Create record string
                let record = format!(
                    ":{:02X}{:04X}00{}{:02X}",
                    length,
                    address,
                    data.iter().map(|b| format!("{:02X}", b)).collect::<String>(),
                    checksum
                );

                Ok(record)
            }
            RecordType::StartLinearAddress | RecordType::StartSegmentAddress => {
                // Check for data length
                if length != 4 {
                    return Err(IntelHexError::RecordLengthInvalidForType(rtype, 4, length));
                }

                // Check for address
                if address != 0 {
                    return Err(IntelHexError::RecordAddressInvalidForType(rtype, 0, address as usize));
                }

                // Create record string
                let record = format!(
                    ":{:02X}{:04X}{}{}{:02X}",
                    length,
                    address,
                    rtype as u8,
                    data.iter().map(|b| format!("{:02X}", b)).collect::<String>(),
                    checksum
                );

                Ok(record)
            }
            RecordType::ExtendedSegmentAddress => {
                Err(IntelHexError::RecordNotSupported)
            }
        }
    }

    /// Parse the record string into Record.
    ///
    fn parse(line: &str) -> Result<Self, IntelHexError> {
        // Check for start record
        if !line.starts_with(':') {
            return Err(IntelHexError::MissingStartCode);
        }

        let hexdigit_part = &line[1..];
        let hexdigit_part_len = hexdigit_part.len();

        // Validate all characters are hexadecimal
        if !hexdigit_part.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(IntelHexError::ContainsInvalidCharacters);
        }

        // Validate record's size
        if hexdigit_part_len < sizes::SMALLEST_RECORD {
            return Err(IntelHexError::RecordTooShort);
        } else if hexdigit_part_len > sizes::LARGEST_RECORD {
            return Err(IntelHexError::RecordTooLong);
        } else if (hexdigit_part_len % 2) != 0 {
            return Err(IntelHexError::RecordNotEvenLength);
        }

        // Get record length
        let length = u8::from_str_radix(&line[ranges::RECORD_LEN_RANGE], 16)
            .unwrap(); // TODO: handle Err

        // Check if record end is bigger than the record length itself
        let data_end =  ranges::RECORD_TYPE_RANGE.end + sizes::BYTE_CHAR_LEN * length as usize;
        let record_end = sizes::BYTE_CHAR_LEN + data_end; // last byte is checksum
        if record_end > line.len() {
            return Err(IntelHexError::RecordInvalidChecksumLength);
        }

        // Get record type
        let rtype = RecordType::parse(&line[ranges::RECORD_TYPE_RANGE])?;

        // Get record address
        let address = u16::from_str_radix(&line[ranges::RECORD_ADDR_RANGE], 16)
            .unwrap(); // TODO: handle Err

        // Get record data payload
        let mut data: Vec<u8> = Vec::new();
        if rtype == RecordType::EndOfFile {
            if length != 0 {
                return Err(IntelHexError::RecordLengthInvalidForType(rtype, 0, length as usize));
            }
        } else {
            for i in (ranges::RECORD_TYPE_RANGE.end..data_end).step_by(sizes::BYTE_CHAR_LEN) {
                let byte = u8::from_str_radix(&line[i..i+sizes::BYTE_CHAR_LEN], 16)
                    .unwrap(); // TODO: handler Err
                data.push(byte);
            }
        }

        // Get checksum
        let checksum = u8::from_str_radix(&line[data_end..record_end], 16)
            .unwrap(); // TODO: handle Err

        // Validate checksum
        // TODO: ...

        Ok(Self {
            length,
            address,
            rtype,
            data,
            checksum,
        })
    }
}


#[derive(Debug, Clone)]
struct StartAddress {
    rtype: Option<RecordType>, // type of the start address
    bytes: Vec<u8>,  // data bytes (aka the address itself stored as bytes)
}

impl StartAddress {
    pub fn is_empty(&self) -> bool {
        self.rtype.is_none() && self.bytes.is_empty()
    }
}


#[derive(Debug, Clone)]
pub struct IntelHex {
    pub filepath: PathBuf,
    pub size: usize,
    offset: usize,
    start_addr: StartAddress,
    buffer: BTreeMap<usize, u8>,
}

impl Default for IntelHex {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelHex {
    /// Creates empty IntelHex struct instance.
    ///
    /// # Examples
    /// ```
    /// use intelhex::IntelHex;
    ///
    /// let ih = IntelHex::new();
    /// ```
    pub fn new() -> Self {
        Self {
            filepath: PathBuf::new(),
            size: 0,
            offset: 0,
            start_addr: StartAddress {
                rtype: None,
                bytes: Vec::new(),
            },
            buffer: BTreeMap::new(),
        }
    }

    /// Parse the raw contents of the hex file and fill internal record vector.
    ///
    fn parse(&mut self, raw_contents: &str) -> Result<(), IntelHexError> {
        // Iterate over lines of records
        for line in raw_contents.lines() {
            // Parse the record
            let record = match Record::parse(line) {
                Ok(rec) => rec,
                Err(e) => return Err(e)
            };

            // Validate checksum of the record
            let expected_checksum = Record::calculate_checksum_from_self(&record);
            if record.checksum != expected_checksum{
                return Err(IntelHexError::RecordChecksumMismatch(expected_checksum, record.checksum));
            }

            //
            match record.rtype {
                RecordType::Data => {
                    let mut addr = record.address as usize + self.offset;
                    for byte in &record.data {
                        if let Some(_) = self.buffer.insert(addr, *byte) {
                            // Address overlap
                            return Err(IntelHexError::RecordAddressOverlap(addr));
                        }
                        addr += 1;
                    }
                }
                RecordType::EndOfFile => {}
                RecordType::ExtendedSegmentAddress => {
                    self.offset = (record.data[0] as usize * 256 + record.data[1] as usize) * 16;
                }
                RecordType::ExtendedLinearAddress => {
                    self.offset = (record.data[0] as usize * 256 + record.data[1] as usize) * 65536;
                }
                RecordType::StartSegmentAddress | RecordType::StartLinearAddress => {
                    if !self.start_addr.is_empty() {
                        return Err(IntelHexError::DuplicateStartAddress);
                    }
                    self.start_addr.rtype = Some(record.rtype);
                    self.start_addr.bytes = record.data[0..4].to_owned();
                }
            }
        };
        Ok(())
    }

    /// Creates IntelHex struct instance and fills it with data from provided hex file.
    ///
    /// # Example
    /// ```
    /// use intelhex::IntelHex;
    ///
    /// let ih = IntelHex::from_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// ```
    pub fn from_hex<P: AsRef<Path>>(filepath: P) -> Result<Self, Box<dyn Error>> {
        let mut ih = IntelHex::new();
        ih.load_hex(filepath)?;
        Ok(ih)
    }

    /// Fills the IntelHex struct instance with data from provided hex file.
    ///
    /// # Example
    /// ```
    /// use intelhex::IntelHex;
    ///
    /// let mut ih = IntelHex::new();
    /// ih.load_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// ```
    pub fn load_hex<P: AsRef<Path>>(&mut self, filepath: P) -> Result<(), Box<dyn Error>> {
        // Read contents of the file
        let raw_contents: String = fs::read_to_string(&filepath)?;

        // Compute the size (in bytes)
        self.size = raw_contents.len();

        // Load filepath
        self.filepath = filepath.as_ref().to_path_buf();

        // Parse contents and return
        self.parse(&raw_contents)?;
        Ok(())
    }

    /// Creates empty IntelHex struct instance.
    ///
    /// # Example
    /// ```
    /// use intelhex::IntelHex;
    ///
    /// let mut ih = IntelHex::from_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// ih.write_hex("build/ex1/ih.hex");
    /// ```
    pub fn write_hex<P: AsRef<Path>>(&mut self, filepath: P) -> Result<(), Box<dyn Error>> {
        // Ensure the parent directory exists
        if let Some(parent) = filepath.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filepath)?;

        // Wrap in BufWriter for efficient line-by-line writing
        let mut writer = io::BufWriter::new(file);

        // Write start address record
        // TODO: place it - start or end of file?
        if !self.start_addr.is_empty() {
            // let (rtype, data) = self.start_addr.iter().next().unwrap(); // fix
            let rtype = self.start_addr.rtype.unwrap();
            let data = &self.start_addr.bytes;
            let record = Record::create(0, rtype, data)?;
            writeln!(writer, "{}", record)?;
        }

        let mut curr_high_addr = 0;
        let mut chunk_start: Option<u16> = None;
        let mut prev_addr: Option<usize> = None;
        let mut chunk_data = Vec::new();

        for (addr, byte) in &self.buffer {
            // Split address into low and high
            let high_addr = (addr >> 16) as u16;
            let low_addr = (addr & 0xFFFF) as u16;

            // If ELA segment changed -> flush current chunk and emit ELA
            if curr_high_addr != high_addr {
                if let Some(start) = chunk_start {
                    // Write data record
                    let record = Record::create(start, RecordType::Data, &chunk_data)?;
                    writeln!(writer, "{}", record)?;

                    // Write ELA record
                    let (msb, lsb) = (high_addr / 256, high_addr % 256);
                    let bin: Vec<u8> = vec![msb as u8, lsb as u8];
                    let record = Record::create(0, RecordType::ExtendedLinearAddress, &bin)?;
                    writeln!(writer, "{}", record)?;

                    // Update segment's current address
                    curr_high_addr = high_addr;

                    // Clean up
                    chunk_data.clear();
                    chunk_start = None;
                    prev_addr = None; // resets continuity check
                }
            }

            // If gap detected or chunk full -> flush
            if let Some(prev) = prev_addr {
                if (*addr != prev + 1) || chunk_data.len() >= 16 {
                    // Write data record
                    let record = Record::create(chunk_start.unwrap(), RecordType::Data, &chunk_data)?;
                    writeln!(writer, "{}", record)?;

                    // Clean up
                    chunk_data.clear();
                    chunk_start = None;
                }
            }

            // Start new chunk if empty
            if chunk_start.is_none() {
                chunk_start = Some(low_addr);
            }

            // Push byte into data chunk
            chunk_data.push(*byte);

            // Update address
            prev_addr = Some(*addr);
        }

        // Flush last data chunk
        let record = Record::create(chunk_start.unwrap(), RecordType::Data, &chunk_data)?;
        writeln!(writer, "{}", record)?;

        // Write EOL record
        let record = Record::create(0, RecordType::EndOfFile, &vec![])?;
        write!(writer, "{}", record)?; // writes a line (no newline)

        Ok(())
    }

    /// Get copy of the data buffer as BTreeMap from IntelHex.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use intelhex::IntelHex;
    ///
    /// let ih = IntelHex::from_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// let addr_byte_map: BTreeMap<usize, u8> = ih.to_btree_map();
    /// ```
    pub fn to_btree_map(&self) -> BTreeMap<usize, u8> {
        self.buffer.clone()
    }

    /// Get byte from IntelHex at provided address.
    ///
    /// # Example
    /// ```
    /// use intelhex::IntelHex;
    ///
    /// let ih = IntelHex::from_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// let b: u8 = ih.get_byte(0x0).unwrap();
    /// ```
    pub fn get_byte(&self, address: usize) -> Option<u8> {
        self.buffer.get(&address).copied()
    }

    /// Get array of bytes from IntelHex at provided addresses.
    ///
    /// # Example
    /// ```
    /// use intelhex::IntelHex;
    ///
    /// let ih = IntelHex::from_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// let b: Vec<u8> = ih.get_buffer_slice(&[0x0, 0x1, 0x2]).unwrap();
    /// ```
    pub fn get_buffer_slice(&self, addr_vec: &[usize]) -> Option<Vec<u8>> {
        let mut out = Vec::with_capacity(addr_vec.len());
        for addr in addr_vec {
            if let Some(&byte) = self.buffer.get(addr) {
                out.push(byte);
            } else {
                return None; // invalid address
            }
        }
        Some(out)
    }

    /// Update byte in IntelHex at provided address.
    ///
    /// # Example
    /// ```
    /// use intelhex::{IntelHex, IntelHexError};
    /// use std::io;
    ///
    /// let mut ih = IntelHex::from_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// let res: Result<(), IntelHexError> = ih.update_byte(0x0, 0xFF);
    /// ```
    pub fn update_byte(&mut self, address: usize, value: u8) -> Result<(), IntelHexError> {
        if let Some(v) = self.buffer.get_mut(&address) {
            *v = value;
            Ok(())
        } else {
            Err(IntelHexError::InvalidAddress(address))
        }
    }

    /// Update array of bytes in IntelHex at provided addresses.
    ///
    /// # Example
    /// ```
    /// use intelhex::{IntelHex, IntelHexError};
    /// use std::io;
    ///
    /// let mut ih = IntelHex::from_hex("tests/fixtures/ih_example_1.hex").unwrap();
    /// let res: Result<(), IntelHexError> = ih.update_buffer_slice(&[(0x0, 0xFF), (0x1, 0xFF), (0x2, 0xFF)]);
    /// ```
    pub fn update_buffer_slice(&mut self, updates_map: &[(usize, u8)]) -> Result<(), IntelHexError> {
        for &(addr, value) in updates_map {
            if let Some(byte) = self.buffer.get_mut(&addr) {
                *byte = value;
            } else {
                return Err(IntelHexError::InvalidAddress(addr));
            }
        }
        Ok(())
    }
}
