use std::io;
use std::fs;
use std::ops::Range;


const RECORD_START: char = ':';
const RECORD_LEN_RANGE: Range<usize> = 1..3;
const RECORD_ADDR_RANGE: Range<usize> = 3..7;
const RECORD_TYPE_RANGE: Range<usize> = 7..9;
const BYTE_CHAR_LEN: usize = 2;
const RECORD_CHKSUM_LEN: usize = BYTE_CHAR_LEN;


#[derive(Debug, PartialEq, Eq)]
enum RecordType {
    Data,
    EndOfFile,
    ExtendedSegmentAddress,
    ExtendedLinearAddress,
    StartLinearAddress,
}

impl RecordType {
    fn parse(s: &str) -> Result<Self, io::Error> {
        match s {
            "00" => Ok(Self::Data),
            "01" => Ok(Self::EndOfFile),
            "02" => Ok(Self::ExtendedSegmentAddress),
            "04" => Ok(Self::ExtendedLinearAddress),
            "05" => Ok(Self::StartLinearAddress),
            _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }
}


#[derive(Debug)]
struct Record {
    length: u8,
    address: u16,
    rtype: RecordType,
    data: Vec<u8>,
    checksum: u8
}

impl Record {
    fn parse(line: &str) -> Result<Self, io::Error> {
        // Check for start record
        if !line.starts_with(RECORD_START) {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        // Get record length
        let length = u8::from_str_radix(&line[RECORD_LEN_RANGE], 16)
            .unwrap(); // TODO: handle Err

        // Error if record end is bigger than the record length itself
        let data_end =  RECORD_TYPE_RANGE.end + BYTE_CHAR_LEN * length as usize;
        let record_end = RECORD_CHKSUM_LEN + data_end;
        if record_end > line.len() {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        // Get record type
        let rtype = RecordType::parse(&line[RECORD_TYPE_RANGE])?;

        // Get record address
        let address = u16::from_str_radix(&line[RECORD_ADDR_RANGE], 16)
            .unwrap(); // TODO: handle Err

        // Get record data
        let mut data: Vec<u8> = Vec::new();
        if rtype == RecordType::EndOfFile {
            if length != 0 { return Err(io::Error::from(io::ErrorKind::InvalidData)); }
        } else {
            for i in (RECORD_TYPE_RANGE.end..data_end).step_by(BYTE_CHAR_LEN) {
                let byte = u8::from_str_radix(&line[i..i+BYTE_CHAR_LEN], 16)
                    .unwrap(); // TODO: handler Err
                data.push(byte);
            }
        }

        // Get checksum
        let checksum = u8::from_str_radix(&line[data_end..record_end], 16)
            .unwrap(); // TODO: handle Err

        // Return record instance
        Ok(Self {
            length,
            address,
            rtype,
            data,
            checksum,
        })
    }
}


pub struct IntelHex {
    filepath: String,
    records: Vec<Record>,
    size: usize, // TODO: implement
}

impl IntelHex {
    /// Creates empty IntelHex struct instance.
    ///
    /// # Examples
    /// ```
    /// let ih = IntelHex::new();
    /// ```
    pub fn new() -> Self {
        Self {
            filepath: String::new(),
            records: Vec::new(),
            size: 0,
        }
    }

    /// Parses the raw contents of the hex file and fills internal record vector.
    ///
    fn parse(raw_contents: &str) -> Result<Vec<Record>, io::Error> {
        let mut records: Vec<Record> = Vec::new();
        //
        for (idx, line) in raw_contents.lines().enumerate() {
            let r = match Record::parse(line) {
                Ok(rec) => rec,
                Err(e) => return Err(e)
            };
            records.push(r);
        };
        Ok(records)
    }

    /// Creates IntelHex struct instance and fills it with data from provided hex file.
    ///
    /// # Examples
    /// ```
    /// let ih = IntelHex::from_hex("src/ih_example.hex").unwrap();
    /// ```
    pub fn from_hex(filepath: &str) -> Result<Self, io::Error> {
        let mut ih = IntelHex::new();
        ih.load_hex(filepath)?;
        Ok(ih)
    }

    /// Fills the IntelHex struct instance with data from provided hex file.
    ///
    /// # Examples
    /// ```
    /// let mut ih = IntelHex::new();
    /// ih.load_hex("src/ih_example.hex").unwrap();
    /// ```
    pub fn load_hex(&mut self, filepath: &str) -> Result<(), io::Error> {
        //
        let raw_contents: String = fs::read_to_string(filepath)?;
        //
        let size = raw_contents.len();
        //
        self.filepath = String::from(filepath);
        self.records = match Self::parse(&raw_contents) {
            Ok(records) => records,
            Err(e) => return Err(e)
        };
        self.size = 0;
        Ok(())
    }
}


fn main() {
    // Example 1
    let mut ih1 = IntelHex::new();
    ih1.load_hex("src/ih_example.hex").unwrap();
    for rec in &ih1.records {
        println!("{:?}", rec);
    }

    // Example 2
    let ih2 = IntelHex::from_hex("src/ih_example.hex").unwrap();
    for rec in &ih2.records {
        println!("{:?}", rec);
    }
}
