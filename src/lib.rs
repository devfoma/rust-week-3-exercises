use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use std::ptr::read_unaligned;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        // TODO: Construct a CompactSize from a u64 value
        Self { value }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Encode according to Bitcoin's CompactSize format:
        // [0x00–0xFC] => 1 byte
        // [0xFDxxxx] => 0xFD + u16 (2 bytes)
        // [0xFExxxxxxxx] =>     0xFE + u32 (4 bytes)
        // [0xFFxxxxxxxxxxxxxxxx] => 0xFF + u64 (8 bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Decode CompactSize, returning value and number of bytes consumed.
        // First check if bytes is empty.
        // Check that enough bytes are available based on prefix.
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Serialize as a hex-encoded string (32 bytes => 64 hex characters)
        serializer.serialize_str(&hex::encode(self.0))
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Parse hex string into 32-byte array
        // Use `hex::decode`, validate length = 32
        let ds = String::deserialize(deserializer)?;
        let bytes = hex::decode(&ds).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("Txid must be 32 bytes"));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);  
        Ok(Txid(array))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        // TODO: Create an OutPoint from raw txid bytes and output index
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize as: txid (32 bytes) + vout (4 bytes, little-endian)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize 36 bytes: txid[0..32], vout[32..36]
        // Return error if insufficient bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self {
        // TODO: Simple constructor
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Prefix with CompactSize (length), then raw bytes
        let mut output = Vec::new();
        output.extend(&CompactSize::new(self.bytes.len() as u64).to_bytes());
        output.extend(&self.bytes);
        output
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Parse CompactSize prefix, then read that many bytes
        // Return error if not enough bytes
        
    }
}

impl Deref for Script {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        // TODO: Allow &Script to be used as &[u8]
        &self.bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self {
        // TODO: Basic constructor
        Self {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize: OutPoint + Script (with CompactSize) + sequence (4 bytes LE)
        let mut output = Vec::new();
        output.extend(&self.previous_output.to_bytes());
        output.extend(&self.script_sig.to_bytes());
        output.extend(&self.sequence.to_le_bytes());
        output
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize in order:
        // - OutPoint (36 bytes)
        // - Script (with CompactSize)
        // - Sequence (4 bytes)
        let mut offset = 0;
        let (outpoint, consumed) = OutPoint::from_bytes(&bytes[offset..])?;
        offset += consumed;     
        if (bytes.len() < offset + 4) {
            return Err(BitcoinError::InsufficientBytes);
        }
        let (script, consumed) = Script::from_bytes(&bytes[offset..])?;
        offset += consumed;
        if (bytes.len() < offset + 4) {
            return Err(BitcoinError::InsufficientBytes);
        }
        let mut sequence_bytes = [0u8; 4];
        sequence_bytes.copy_from_slice(&bytes[offset..offset + 4]);
        let sequence = u32::from_le_bytes(sequence_bytes);
        offset += 4;

        Ok((
            Self {
                previous_output: outpoint,
                script_sig: script,
                sequence,
            },
            offset,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self {
        // TODO: Construct a transaction from parts
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Format:
        // - version (4 bytes LE)
        // - CompactSize (number of inputs)
        // - each input serialized
        // - lock_time (4 bytes LE)
        let mut output = Vec::new();
        output.extend(&self.version.to_le_bytes());
        output.extend(&CompactSize::new(self.inputs.len() as u64).to_bytes());
        for input in &self.inputs {
            output.extend(&input.to_bytes());
        }
        output.extend(&self.lock_time.to_le_bytes());
        output
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Read version, CompactSize for input count
        // Parse inputs one by one
        // Read final 4 bytes for 
        let mut offset= 0;
        if bytes.len() < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let mut ver_bytes = [0u8; 4];
        ver_bytes.copy_from_slice(&bytes[offset..offset + 4]);
        let version = u32::from_le_bytes(ver_bytes);
        offset += 4;

        let (cs, consumed) = CompactSize::from_bytes(&bytes[offset..])?;
        offset += consumed;
        let input_count = cs.value as usize;

        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            let (input, consumed) = TransactionInput::from_bytes(&bytes[offset..])?;
            inputs.push(input);
            offset += consumed;
        }

        if bytes.len() < offset + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let mut lt_bytes = [0u8; 4];
        lt_bytes.copy_from_slice(&bytes[offset..offset + 4]);
        let lock_time = u32::from_le_bytes(lt_bytes);
        offset += 4;

        Ok((
            Self {
                version,
                inputs,
                lock_time,
            },
            offset,
        ))
    }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format a user-friendly string showing version, inputs, lock_time
        // Display scriptSig length and bytes, and previous output info
        writeln!(f, "Transaction: version={}, lock_time={}", self.version, self.lock_time)?;
        for (i, input) in self.inputs.iter().enumerate() {
            writeln!(f, "  Input {}: {:?}", i, input)?;
        }
        Ok(())
    }
}
