// rust
use hex;

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Amount {
	value: u64
}
impl Amount {
	const SIZE: usize = 8;
	pub fn new(amount: u64) -> Self {
		Self {
			value: amount
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BlockDuration {
	value: u64
}
impl BlockDuration {
	const SIZE: usize = 8;
	pub fn new(blockduration: u64) -> Self {
		Self {
			value: blockduration
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BlockFeeMultiplier {
	value: u32
}
impl BlockFeeMultiplier {
	const SIZE: usize = 4;
	pub fn new(blockfeemultiplier: u32) -> Self {
		Self {
			value: blockfeemultiplier
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let value = u32::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..4]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:08x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Difficulty {
	value: u64
}
impl Difficulty {
	const SIZE: usize = 8;
	pub fn new(difficulty: u64) -> Self {
		Self {
			value: difficulty
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FinalizationEpoch {
	value: u32
}
impl FinalizationEpoch {
	const SIZE: usize = 4;
	pub fn new(finalizationepoch: u32) -> Self {
		Self {
			value: finalizationepoch
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let value = u32::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..4]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:08x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FinalizationPoint {
	value: u32
}
impl FinalizationPoint {
	const SIZE: usize = 4;
	pub fn new(finalizationpoint: u32) -> Self {
		Self {
			value: finalizationpoint
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let value = u32::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..4]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:08x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Height {
	value: u64
}
impl Height {
	const SIZE: usize = 8;
	pub fn new(height: u64) -> Self {
		Self {
			value: height
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Importance {
	value: u64
}
impl Importance {
	const SIZE: usize = 8;
	pub fn new(importance: u64) -> Self {
		Self {
			value: importance
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ImportanceHeight {
	value: u64
}
impl ImportanceHeight {
	const SIZE: usize = 8;
	pub fn new(importanceheight: u64) -> Self {
		Self {
			value: importanceheight
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UnresolvedMosaicId {
	value: u64
}
impl UnresolvedMosaicId {
	const SIZE: usize = 8;
	pub fn new(unresolvedmosaicid: u64) -> Self {
		Self {
			value: unresolvedmosaicid
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MosaicId {
	value: u64
}
impl MosaicId {
	const SIZE: usize = 8;
	pub fn new(mosaicid: u64) -> Self {
		Self {
			value: mosaicid
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Timestamp {
	value: u64
}
impl Timestamp {
	const SIZE: usize = 8;
	pub fn new(timestamp: u64) -> Self {
		Self {
			value: timestamp
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct UnresolvedAddress {
	bytes: [u8; 24]
}
impl UnresolvedAddress {
	const SIZE: usize = 24;
	pub fn new(unresolvedaddress: [u8; 24]) -> Self {
		Self {
			bytes: unresolvedaddress
		}
	}
	pub fn default() -> Self {
		Self::new([0; 24])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 24 { return None; }
		let mut bytes = [0u8; 24];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..24]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
	bytes: [u8; 24]
}
impl Address {
	const SIZE: usize = 24;
	pub fn new(address: [u8; 24]) -> Self {
		Self {
			bytes: address
		}
	}
	pub fn default() -> Self {
		Self::new([0; 24])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 24 { return None; }
		let mut bytes = [0u8; 24];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..24]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct Hash256 {
	bytes: [u8; 32]
}
impl Hash256 {
	const SIZE: usize = 32;
	pub fn new(hash256: [u8; 32]) -> Self {
		Self {
			bytes: hash256
		}
	}
	pub fn default() -> Self {
		Self::new([0; 32])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 32 { return None; }
		let mut bytes = [0u8; 32];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..32]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct Hash512 {
	bytes: [u8; 64]
}
impl Hash512 {
	const SIZE: usize = 64;
	pub fn new(hash512: [u8; 64]) -> Self {
		Self {
			bytes: hash512
		}
	}
	pub fn default() -> Self {
		Self::new([0; 64])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 64 { return None; }
		let mut bytes = [0u8; 64];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..64]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct PublicKey {
	bytes: [u8; 32]
}
impl PublicKey {
	const SIZE: usize = 32;
	pub fn new(publickey: [u8; 32]) -> Self {
		Self {
			bytes: publickey
		}
	}
	pub fn default() -> Self {
		Self::new([0; 32])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 32 { return None; }
		let mut bytes = [0u8; 32];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..32]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct VotingPublicKey {
	bytes: [u8; 32]
}
impl VotingPublicKey {
	const SIZE: usize = 32;
	pub fn new(votingpublickey: [u8; 32]) -> Self {
		Self {
			bytes: votingpublickey
		}
	}
	pub fn default() -> Self {
		Self::new([0; 32])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 32 { return None; }
		let mut bytes = [0u8; 32];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..32]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
	bytes: [u8; 64]
}
impl Signature {
	const SIZE: usize = 64;
	pub fn new(signature: [u8; 64]) -> Self {
		Self {
			bytes: signature
		}
	}
	pub fn default() -> Self {
		Self::new([0; 64])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 64 { return None; }
		let mut bytes = [0u8; 64];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..64]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct Mosaic {
	pub mosaic_id: MosaicId,
	pub amount: Amount,
}
impl Mosaic {
	pub fn new() -> Self {
		Self {
			mosaic_id: MosaicId::default(),
			amount: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.mosaic_id.size();
		size += self.amount.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mosaic_id;
		(mosaic_id, payload) = MosaicId::deserialize(payload).unwrap();
		let amount;
		(amount, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			mosaic_id: mosaic_id,
			amount: amount,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.amount.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct UnresolvedMosaic {
	pub mosaic_id: UnresolvedMosaicId,
	pub amount: Amount,
}
impl UnresolvedMosaic {
	pub fn new() -> Self {
		Self {
			mosaic_id: UnresolvedMosaicId::default(),
			amount: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.mosaic_id.size();
		size += self.amount.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mosaic_id;
		(mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let amount;
		(amount, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			mosaic_id: mosaic_id,
			amount: amount,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.amount.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LinkAction {
	UNLINK = 0,
	LINK = 1,
}
impl LinkAction {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::UNLINK
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			0 => Some((LinkAction::UNLINK, &payload[1..])),
			1 => Some((LinkAction::LINK, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("LinkAction::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum NetworkType {
	MAINNET = 104,
	TESTNET = 152,
}
impl NetworkType {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::MAINNET
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			104 => Some((NetworkType::MAINNET, &payload[1..])),
			152 => Some((NetworkType::TESTNET, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("NetworkType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TransactionType {
	ACCOUNT_KEY_LINK = 16716,
	NODE_KEY_LINK = 16972,
	AGGREGATE_COMPLETE = 16705,
	AGGREGATE_BONDED = 16961,
	VOTING_KEY_LINK = 16707,
	VRF_KEY_LINK = 16963,
	HASH_LOCK = 16712,
	SECRET_LOCK = 16722,
	SECRET_PROOF = 16978,
	ACCOUNT_METADATA = 16708,
	MOSAIC_METADATA = 16964,
	NAMESPACE_METADATA = 17220,
	MOSAIC_DEFINITION = 16717,
	MOSAIC_SUPPLY_CHANGE = 16973,
	MOSAIC_SUPPLY_REVOCATION = 17229,
	MULTISIG_ACCOUNT_MODIFICATION = 16725,
	ADDRESS_ALIAS = 16974,
	MOSAIC_ALIAS = 17230,
	NAMESPACE_REGISTRATION = 16718,
	ACCOUNT_ADDRESS_RESTRICTION = 16720,
	ACCOUNT_MOSAIC_RESTRICTION = 16976,
	ACCOUNT_OPERATION_RESTRICTION = 17232,
	MOSAIC_ADDRESS_RESTRICTION = 16977,
	MOSAIC_GLOBAL_RESTRICTION = 16721,
	TRANSFER = 16724,
}
impl TransactionType {
	const SIZE: usize = 2;
	pub fn default() -> Self {
		Self::ACCOUNT_KEY_LINK
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 2 { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		match u16::from_le_bytes(bytes) {
			16716 => Some((TransactionType::ACCOUNT_KEY_LINK, &payload[2..])),
			16972 => Some((TransactionType::NODE_KEY_LINK, &payload[2..])),
			16705 => Some((TransactionType::AGGREGATE_COMPLETE, &payload[2..])),
			16961 => Some((TransactionType::AGGREGATE_BONDED, &payload[2..])),
			16707 => Some((TransactionType::VOTING_KEY_LINK, &payload[2..])),
			16963 => Some((TransactionType::VRF_KEY_LINK, &payload[2..])),
			16712 => Some((TransactionType::HASH_LOCK, &payload[2..])),
			16722 => Some((TransactionType::SECRET_LOCK, &payload[2..])),
			16978 => Some((TransactionType::SECRET_PROOF, &payload[2..])),
			16708 => Some((TransactionType::ACCOUNT_METADATA, &payload[2..])),
			16964 => Some((TransactionType::MOSAIC_METADATA, &payload[2..])),
			17220 => Some((TransactionType::NAMESPACE_METADATA, &payload[2..])),
			16717 => Some((TransactionType::MOSAIC_DEFINITION, &payload[2..])),
			16973 => Some((TransactionType::MOSAIC_SUPPLY_CHANGE, &payload[2..])),
			17229 => Some((TransactionType::MOSAIC_SUPPLY_REVOCATION, &payload[2..])),
			16725 => Some((TransactionType::MULTISIG_ACCOUNT_MODIFICATION, &payload[2..])),
			16974 => Some((TransactionType::ADDRESS_ALIAS, &payload[2..])),
			17230 => Some((TransactionType::MOSAIC_ALIAS, &payload[2..])),
			16718 => Some((TransactionType::NAMESPACE_REGISTRATION, &payload[2..])),
			16720 => Some((TransactionType::ACCOUNT_ADDRESS_RESTRICTION, &payload[2..])),
			16976 => Some((TransactionType::ACCOUNT_MOSAIC_RESTRICTION, &payload[2..])),
			17232 => Some((TransactionType::ACCOUNT_OPERATION_RESTRICTION, &payload[2..])),
			16977 => Some((TransactionType::MOSAIC_ADDRESS_RESTRICTION, &payload[2..])),
			16721 => Some((TransactionType::MOSAIC_GLOBAL_RESTRICTION, &payload[2..])),
			16724 => Some((TransactionType::TRANSFER, &payload[2..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u16).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("TransactionType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct Transaction {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
}
impl Transaction {
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: 0,
			network: NetworkType::default(),
			type_: TransactionType::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedTransaction {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
}
impl EmbeddedTransaction {
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: 0,
			network: NetworkType::default(),
			type_: TransactionType::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct ProofGamma {
	bytes: [u8; 32]
}
impl ProofGamma {
	const SIZE: usize = 32;
	pub fn new(proofgamma: [u8; 32]) -> Self {
		Self {
			bytes: proofgamma
		}
	}
	pub fn default() -> Self {
		Self::new([0; 32])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 32 { return None; }
		let mut bytes = [0u8; 32];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..32]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct ProofVerificationHash {
	bytes: [u8; 16]
}
impl ProofVerificationHash {
	const SIZE: usize = 16;
	pub fn new(proofverificationhash: [u8; 16]) -> Self {
		Self {
			bytes: proofverificationhash
		}
	}
	pub fn default() -> Self {
		Self::new([0; 16])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 16 { return None; }
		let mut bytes = [0u8; 16];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..16]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone, PartialEq)]
pub struct ProofScalar {
	bytes: [u8; 32]
}
impl ProofScalar {
	const SIZE: usize = 32;
	pub fn new(proofscalar: [u8; 32]) -> Self {
		Self {
			bytes: proofscalar
		}
	}
	pub fn default() -> Self {
		Self::new([0; 32])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 32 { return None; }
		let mut bytes = [0u8; 32];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..32]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BlockType {
	NEMESIS = 32835,
	NORMAL = 33091,
	IMPORTANCE = 33347,
}
impl BlockType {
	const SIZE: usize = 2;
	pub fn default() -> Self {
		Self::NEMESIS
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 2 { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		match u16::from_le_bytes(bytes) {
			32835 => Some((BlockType::NEMESIS, &payload[2..])),
			33091 => Some((BlockType::NORMAL, &payload[2..])),
			33347 => Some((BlockType::IMPORTANCE, &payload[2..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u16).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("BlockType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct VrfProof {
	pub gamma: ProofGamma,
	pub verification_hash: ProofVerificationHash,
	pub scalar: ProofScalar,
}
impl VrfProof {
	pub fn new() -> Self {
		Self {
			gamma: ProofGamma::default(),
			verification_hash: ProofVerificationHash::default(),
			scalar: ProofScalar::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.gamma.size();
		size += self.verification_hash.size();
		size += self.scalar.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let gamma;
		(gamma, payload) = ProofGamma::deserialize(payload).unwrap();
		let verification_hash;
		(verification_hash, payload) = ProofVerificationHash::deserialize(payload).unwrap();
		let scalar;
		(scalar, payload) = ProofScalar::deserialize(payload).unwrap();
		let self_ = Self {
			gamma: gamma,
			verification_hash: verification_hash,
			scalar: scalar,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.gamma.serialize());
		serialized.append(&mut self.verification_hash.serialize());
		serialized.append(&mut self.scalar.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct Block {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: BlockType,
	pub height: Height,
	pub timestamp: Timestamp,
	pub difficulty: Difficulty,
	pub generation_hash_proof: VrfProof,
	pub previous_block_hash: Hash256,
	pub transactions_hash: Hash256,
	pub receipts_hash: Hash256,
	pub state_hash: Hash256,
	pub beneficiary_address: Address,
	pub fee_multiplier: BlockFeeMultiplier,
}
impl Block {
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: 0,
			network: NetworkType::default(),
			type_: BlockType::default(),
			height: Height::default(),
			timestamp: Timestamp::default(),
			difficulty: Difficulty::default(),
			generation_hash_proof: VrfProof::default(),
			previous_block_hash: Hash256::default(),
			transactions_hash: Hash256::default(),
			receipts_hash: Hash256::default(),
			state_hash: Hash256::default(),
			beneficiary_address: Address::default(),
			fee_multiplier: BlockFeeMultiplier::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.height.size();
		size += self.timestamp.size();
		size += self.difficulty.size();
		size += self.generation_hash_proof.size();
		size += self.previous_block_hash.size();
		size += self.transactions_hash.size();
		size += self.receipts_hash.size();
		size += self.state_hash.size();
		size += self.beneficiary_address.size();
		size += self.fee_multiplier.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = BlockType::deserialize(payload).unwrap();
		let height;
		(height, payload) = Height::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let difficulty;
		(difficulty, payload) = Difficulty::deserialize(payload).unwrap();
		let generation_hash_proof;
		(generation_hash_proof, payload) = VrfProof::deserialize(payload).unwrap();
		let previous_block_hash;
		(previous_block_hash, payload) = Hash256::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let receipts_hash;
		(receipts_hash, payload) = Hash256::deserialize(payload).unwrap();
		let state_hash;
		(state_hash, payload) = Hash256::deserialize(payload).unwrap();
		let beneficiary_address;
		(beneficiary_address, payload) = Address::deserialize(payload).unwrap();
		let fee_multiplier;
		(fee_multiplier, payload) = BlockFeeMultiplier::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			height: height,
			timestamp: timestamp,
			difficulty: difficulty,
			generation_hash_proof: generation_hash_proof,
			previous_block_hash: previous_block_hash,
			transactions_hash: transactions_hash,
			receipts_hash: receipts_hash,
			state_hash: state_hash,
			beneficiary_address: beneficiary_address,
			fee_multiplier: fee_multiplier,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.height.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.difficulty.serialize());
		serialized.append(&mut self.generation_hash_proof.serialize());
		serialized.append(&mut self.previous_block_hash.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.receipts_hash.serialize());
		serialized.append(&mut self.state_hash.serialize());
		serialized.append(&mut self.beneficiary_address.serialize());
		serialized.append(&mut self.fee_multiplier.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NemesisBlockV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: BlockType,
	pub height: Height,
	pub timestamp: Timestamp,
	pub difficulty: Difficulty,
	pub generation_hash_proof: VrfProof,
	pub previous_block_hash: Hash256,
	pub transactions_hash: Hash256,
	pub receipts_hash: Hash256,
	pub state_hash: Hash256,
	pub beneficiary_address: Address,
	pub fee_multiplier: BlockFeeMultiplier,
	pub voting_eligible_accounts_count: u32,
	pub harvesting_eligible_accounts_count: u64,
	pub total_voting_balance: Amount,
	pub previous_importance_block_hash: Hash256,
	pub transactions: Vec<Transaction>,
}
impl NemesisBlockV1 {
	const BLOCK_VERSION: u8 = 1;
	const BLOCK_TYPE: BlockType = BlockType::NEMESIS;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::BLOCK_VERSION,
			network: NetworkType::default(),
			type_: Self::BLOCK_TYPE,
			height: Height::default(),
			timestamp: Timestamp::default(),
			difficulty: Difficulty::default(),
			generation_hash_proof: VrfProof::default(),
			previous_block_hash: Hash256::default(),
			transactions_hash: Hash256::default(),
			receipts_hash: Hash256::default(),
			state_hash: Hash256::default(),
			beneficiary_address: Address::default(),
			fee_multiplier: BlockFeeMultiplier::default(),
			voting_eligible_accounts_count: 0,
			harvesting_eligible_accounts_count: 0,
			total_voting_balance: Amount::default(),
			previous_importance_block_hash: Hash256::default(),
			transactions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.height.size();
		size += self.timestamp.size();
		size += self.difficulty.size();
		size += self.generation_hash_proof.size();
		size += self.previous_block_hash.size();
		size += self.transactions_hash.size();
		size += self.receipts_hash.size();
		size += self.state_hash.size();
		size += self.beneficiary_address.size();
		size += self.fee_multiplier.size();
		size += 4;
		size += 8;
		size += self.total_voting_balance.size();
		size += self.previous_importance_block_hash.size();
		size += self.transactions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = BlockType::deserialize(payload).unwrap();
		let height;
		(height, payload) = Height::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let difficulty;
		(difficulty, payload) = Difficulty::deserialize(payload).unwrap();
		let generation_hash_proof;
		(generation_hash_proof, payload) = VrfProof::deserialize(payload).unwrap();
		let previous_block_hash;
		(previous_block_hash, payload) = Hash256::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let receipts_hash;
		(receipts_hash, payload) = Hash256::deserialize(payload).unwrap();
		let state_hash;
		(state_hash, payload) = Hash256::deserialize(payload).unwrap();
		let beneficiary_address;
		(beneficiary_address, payload) = Address::deserialize(payload).unwrap();
		let fee_multiplier;
		(fee_multiplier, payload) = BlockFeeMultiplier::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let voting_eligible_accounts_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let harvesting_eligible_accounts_count = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let total_voting_balance;
		(total_voting_balance, payload) = Amount::deserialize(payload).unwrap();
		let previous_importance_block_hash;
		(previous_importance_block_hash, payload) = Hash256::deserialize(payload).unwrap();
		let mut transactions = Vec::new();
		for _ in 0..0 {
			let element;
			(element, payload) = Transaction::deserialize(payload).unwrap();
			transactions.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			height: height,
			timestamp: timestamp,
			difficulty: difficulty,
			generation_hash_proof: generation_hash_proof,
			previous_block_hash: previous_block_hash,
			transactions_hash: transactions_hash,
			receipts_hash: receipts_hash,
			state_hash: state_hash,
			beneficiary_address: beneficiary_address,
			fee_multiplier: fee_multiplier,
			voting_eligible_accounts_count: voting_eligible_accounts_count,
			harvesting_eligible_accounts_count: harvesting_eligible_accounts_count,
			total_voting_balance: total_voting_balance,
			previous_importance_block_hash: previous_importance_block_hash,
			transactions: transactions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.height.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.difficulty.serialize());
		serialized.append(&mut self.generation_hash_proof.serialize());
		serialized.append(&mut self.previous_block_hash.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.receipts_hash.serialize());
		serialized.append(&mut self.state_hash.serialize());
		serialized.append(&mut self.beneficiary_address.serialize());
		serialized.append(&mut self.fee_multiplier.serialize());
		serialized.append(&mut self.voting_eligible_accounts_count.to_le_bytes().to_vec());
		serialized.append(&mut self.harvesting_eligible_accounts_count.to_le_bytes().to_vec());
		serialized.append(&mut self.total_voting_balance.serialize());
		serialized.append(&mut self.previous_importance_block_hash.serialize());
		serialized.append(&mut self.transactions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NormalBlockV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: BlockType,
	pub height: Height,
	pub timestamp: Timestamp,
	pub difficulty: Difficulty,
	pub generation_hash_proof: VrfProof,
	pub previous_block_hash: Hash256,
	pub transactions_hash: Hash256,
	pub receipts_hash: Hash256,
	pub state_hash: Hash256,
	pub beneficiary_address: Address,
	pub fee_multiplier: BlockFeeMultiplier,
	pub block_header_reserved_1: u32,
	pub transactions: Vec<Transaction>,
}
impl NormalBlockV1 {
	const BLOCK_VERSION: u8 = 1;
	const BLOCK_TYPE: BlockType = BlockType::NORMAL;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::BLOCK_VERSION,
			network: NetworkType::default(),
			type_: Self::BLOCK_TYPE,
			height: Height::default(),
			timestamp: Timestamp::default(),
			difficulty: Difficulty::default(),
			generation_hash_proof: VrfProof::default(),
			previous_block_hash: Hash256::default(),
			transactions_hash: Hash256::default(),
			receipts_hash: Hash256::default(),
			state_hash: Hash256::default(),
			beneficiary_address: Address::default(),
			fee_multiplier: BlockFeeMultiplier::default(),
			block_header_reserved_1: 0,
			transactions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.height.size();
		size += self.timestamp.size();
		size += self.difficulty.size();
		size += self.generation_hash_proof.size();
		size += self.previous_block_hash.size();
		size += self.transactions_hash.size();
		size += self.receipts_hash.size();
		size += self.state_hash.size();
		size += self.beneficiary_address.size();
		size += self.fee_multiplier.size();
		size += 4;
		size += self.transactions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = BlockType::deserialize(payload).unwrap();
		let height;
		(height, payload) = Height::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let difficulty;
		(difficulty, payload) = Difficulty::deserialize(payload).unwrap();
		let generation_hash_proof;
		(generation_hash_proof, payload) = VrfProof::deserialize(payload).unwrap();
		let previous_block_hash;
		(previous_block_hash, payload) = Hash256::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let receipts_hash;
		(receipts_hash, payload) = Hash256::deserialize(payload).unwrap();
		let state_hash;
		(state_hash, payload) = Hash256::deserialize(payload).unwrap();
		let beneficiary_address;
		(beneficiary_address, payload) = Address::deserialize(payload).unwrap();
		let fee_multiplier;
		(fee_multiplier, payload) = BlockFeeMultiplier::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let block_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if block_header_reserved_1 != 0 { return None; }
		let mut transactions = Vec::new();
		for _ in 0..0 {
			let element;
			(element, payload) = Transaction::deserialize(payload).unwrap();
			transactions.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			height: height,
			timestamp: timestamp,
			difficulty: difficulty,
			generation_hash_proof: generation_hash_proof,
			previous_block_hash: previous_block_hash,
			transactions_hash: transactions_hash,
			receipts_hash: receipts_hash,
			state_hash: state_hash,
			beneficiary_address: beneficiary_address,
			fee_multiplier: fee_multiplier,
			block_header_reserved_1: block_header_reserved_1,
			transactions: transactions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.height.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.difficulty.serialize());
		serialized.append(&mut self.generation_hash_proof.serialize());
		serialized.append(&mut self.previous_block_hash.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.receipts_hash.serialize());
		serialized.append(&mut self.state_hash.serialize());
		serialized.append(&mut self.beneficiary_address.serialize());
		serialized.append(&mut self.fee_multiplier.serialize());
		serialized.append(&mut self.block_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.transactions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct ImportanceBlockV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: BlockType,
	pub height: Height,
	pub timestamp: Timestamp,
	pub difficulty: Difficulty,
	pub generation_hash_proof: VrfProof,
	pub previous_block_hash: Hash256,
	pub transactions_hash: Hash256,
	pub receipts_hash: Hash256,
	pub state_hash: Hash256,
	pub beneficiary_address: Address,
	pub fee_multiplier: BlockFeeMultiplier,
	pub voting_eligible_accounts_count: u32,
	pub harvesting_eligible_accounts_count: u64,
	pub total_voting_balance: Amount,
	pub previous_importance_block_hash: Hash256,
	pub transactions: Vec<Transaction>,
}
impl ImportanceBlockV1 {
	const BLOCK_VERSION: u8 = 1;
	const BLOCK_TYPE: BlockType = BlockType::IMPORTANCE;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::BLOCK_VERSION,
			network: NetworkType::default(),
			type_: Self::BLOCK_TYPE,
			height: Height::default(),
			timestamp: Timestamp::default(),
			difficulty: Difficulty::default(),
			generation_hash_proof: VrfProof::default(),
			previous_block_hash: Hash256::default(),
			transactions_hash: Hash256::default(),
			receipts_hash: Hash256::default(),
			state_hash: Hash256::default(),
			beneficiary_address: Address::default(),
			fee_multiplier: BlockFeeMultiplier::default(),
			voting_eligible_accounts_count: 0,
			harvesting_eligible_accounts_count: 0,
			total_voting_balance: Amount::default(),
			previous_importance_block_hash: Hash256::default(),
			transactions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.height.size();
		size += self.timestamp.size();
		size += self.difficulty.size();
		size += self.generation_hash_proof.size();
		size += self.previous_block_hash.size();
		size += self.transactions_hash.size();
		size += self.receipts_hash.size();
		size += self.state_hash.size();
		size += self.beneficiary_address.size();
		size += self.fee_multiplier.size();
		size += 4;
		size += 8;
		size += self.total_voting_balance.size();
		size += self.previous_importance_block_hash.size();
		size += self.transactions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = BlockType::deserialize(payload).unwrap();
		let height;
		(height, payload) = Height::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let difficulty;
		(difficulty, payload) = Difficulty::deserialize(payload).unwrap();
		let generation_hash_proof;
		(generation_hash_proof, payload) = VrfProof::deserialize(payload).unwrap();
		let previous_block_hash;
		(previous_block_hash, payload) = Hash256::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let receipts_hash;
		(receipts_hash, payload) = Hash256::deserialize(payload).unwrap();
		let state_hash;
		(state_hash, payload) = Hash256::deserialize(payload).unwrap();
		let beneficiary_address;
		(beneficiary_address, payload) = Address::deserialize(payload).unwrap();
		let fee_multiplier;
		(fee_multiplier, payload) = BlockFeeMultiplier::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let voting_eligible_accounts_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let harvesting_eligible_accounts_count = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let total_voting_balance;
		(total_voting_balance, payload) = Amount::deserialize(payload).unwrap();
		let previous_importance_block_hash;
		(previous_importance_block_hash, payload) = Hash256::deserialize(payload).unwrap();
		let mut transactions = Vec::new();
		for _ in 0..0 {
			let element;
			(element, payload) = Transaction::deserialize(payload).unwrap();
			transactions.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			height: height,
			timestamp: timestamp,
			difficulty: difficulty,
			generation_hash_proof: generation_hash_proof,
			previous_block_hash: previous_block_hash,
			transactions_hash: transactions_hash,
			receipts_hash: receipts_hash,
			state_hash: state_hash,
			beneficiary_address: beneficiary_address,
			fee_multiplier: fee_multiplier,
			voting_eligible_accounts_count: voting_eligible_accounts_count,
			harvesting_eligible_accounts_count: harvesting_eligible_accounts_count,
			total_voting_balance: total_voting_balance,
			previous_importance_block_hash: previous_importance_block_hash,
			transactions: transactions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.height.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.difficulty.serialize());
		serialized.append(&mut self.generation_hash_proof.serialize());
		serialized.append(&mut self.previous_block_hash.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.receipts_hash.serialize());
		serialized.append(&mut self.state_hash.serialize());
		serialized.append(&mut self.beneficiary_address.serialize());
		serialized.append(&mut self.fee_multiplier.serialize());
		serialized.append(&mut self.voting_eligible_accounts_count.to_le_bytes().to_vec());
		serialized.append(&mut self.harvesting_eligible_accounts_count.to_le_bytes().to_vec());
		serialized.append(&mut self.total_voting_balance.serialize());
		serialized.append(&mut self.previous_importance_block_hash.serialize());
		serialized.append(&mut self.transactions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct FinalizationRound {
	pub epoch: FinalizationEpoch,
	pub point: FinalizationPoint,
}
impl FinalizationRound {
	pub fn new() -> Self {
		Self {
			epoch: FinalizationEpoch::default(),
			point: FinalizationPoint::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.epoch.size();
		size += self.point.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let epoch;
		(epoch, payload) = FinalizationEpoch::deserialize(payload).unwrap();
		let point;
		(point, payload) = FinalizationPoint::deserialize(payload).unwrap();
		let self_ = Self {
			epoch: epoch,
			point: point,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.epoch.serialize());
		serialized.append(&mut self.point.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct FinalizedBlockHeader {
	pub round: FinalizationRound,
	pub height: Height,
	pub hash: Hash256,
}
impl FinalizedBlockHeader {
	pub fn new() -> Self {
		Self {
			round: FinalizationRound::default(),
			height: Height::default(),
			hash: Hash256::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.round.size();
		size += self.height.size();
		size += self.hash.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let round;
		(round, payload) = FinalizationRound::deserialize(payload).unwrap();
		let height;
		(height, payload) = Height::deserialize(payload).unwrap();
		let hash;
		(hash, payload) = Hash256::deserialize(payload).unwrap();
		let self_ = Self {
			round: round,
			height: height,
			hash: hash,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.round.serialize());
		serialized.append(&mut self.height.serialize());
		serialized.append(&mut self.hash.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ReceiptType {
	MOSAIC_RENTAL_FEE = 4685,
	NAMESPACE_RENTAL_FEE = 4942,
	HARVEST_FEE = 8515,
	LOCK_HASH_COMPLETED = 8776,
	LOCK_HASH_EXPIRED = 9032,
	LOCK_SECRET_COMPLETED = 8786,
	LOCK_SECRET_EXPIRED = 9042,
	LOCK_HASH_CREATED = 12616,
	LOCK_SECRET_CREATED = 12626,
	MOSAIC_EXPIRED = 16717,
	NAMESPACE_EXPIRED = 16718,
	NAMESPACE_DELETED = 16974,
	INFLATION = 20803,
	TRANSACTION_GROUP = 57667,
	ADDRESS_ALIAS_RESOLUTION = 61763,
	MOSAIC_ALIAS_RESOLUTION = 62019,
}
impl ReceiptType {
	const SIZE: usize = 2;
	pub fn default() -> Self {
		Self::MOSAIC_RENTAL_FEE
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 2 { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		match u16::from_le_bytes(bytes) {
			4685 => Some((ReceiptType::MOSAIC_RENTAL_FEE, &payload[2..])),
			4942 => Some((ReceiptType::NAMESPACE_RENTAL_FEE, &payload[2..])),
			8515 => Some((ReceiptType::HARVEST_FEE, &payload[2..])),
			8776 => Some((ReceiptType::LOCK_HASH_COMPLETED, &payload[2..])),
			9032 => Some((ReceiptType::LOCK_HASH_EXPIRED, &payload[2..])),
			8786 => Some((ReceiptType::LOCK_SECRET_COMPLETED, &payload[2..])),
			9042 => Some((ReceiptType::LOCK_SECRET_EXPIRED, &payload[2..])),
			12616 => Some((ReceiptType::LOCK_HASH_CREATED, &payload[2..])),
			12626 => Some((ReceiptType::LOCK_SECRET_CREATED, &payload[2..])),
			16717 => Some((ReceiptType::MOSAIC_EXPIRED, &payload[2..])),
			16718 => Some((ReceiptType::NAMESPACE_EXPIRED, &payload[2..])),
			16974 => Some((ReceiptType::NAMESPACE_DELETED, &payload[2..])),
			20803 => Some((ReceiptType::INFLATION, &payload[2..])),
			57667 => Some((ReceiptType::TRANSACTION_GROUP, &payload[2..])),
			61763 => Some((ReceiptType::ADDRESS_ALIAS_RESOLUTION, &payload[2..])),
			62019 => Some((ReceiptType::MOSAIC_ALIAS_RESOLUTION, &payload[2..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u16).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("ReceiptType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct Receipt {
	pub version: u16,
	pub type_: ReceiptType,
}
impl Receipt {
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: ReceiptType::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct HarvestFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub target_address: Address,
}
impl HarvestFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::HARVEST_FEE;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			target_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct InflationReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
}
impl InflationReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::INFLATION;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct LockHashCreatedFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub target_address: Address,
}
impl LockHashCreatedFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::LOCK_HASH_CREATED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			target_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct LockHashCompletedFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub target_address: Address,
}
impl LockHashCompletedFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::LOCK_HASH_COMPLETED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			target_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct LockHashExpiredFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub target_address: Address,
}
impl LockHashExpiredFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::LOCK_HASH_EXPIRED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			target_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct LockSecretCreatedFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub target_address: Address,
}
impl LockSecretCreatedFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::LOCK_SECRET_CREATED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			target_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct LockSecretCompletedFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub target_address: Address,
}
impl LockSecretCompletedFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::LOCK_SECRET_COMPLETED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			target_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct LockSecretExpiredFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub target_address: Address,
}
impl LockSecretExpiredFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::LOCK_SECRET_EXPIRED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			target_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicExpiredReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub artifact_id: MosaicId,
}
impl MosaicExpiredReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::MOSAIC_EXPIRED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			artifact_id: MosaicId::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.artifact_id.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let artifact_id;
		(artifact_id, payload) = MosaicId::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			artifact_id: artifact_id,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.artifact_id.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicRentalFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub sender_address: Address,
	pub recipient_address: Address,
}
impl MosaicRentalFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::MOSAIC_RENTAL_FEE;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			sender_address: Address::default(),
			recipient_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.sender_address.size();
		size += self.recipient_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let sender_address;
		(sender_address, payload) = Address::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			sender_address: sender_address,
			recipient_address: recipient_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.sender_address.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NamespaceId {
	value: u64
}
impl NamespaceId {
	const SIZE: usize = 8;
	pub fn new(namespaceid: u64) -> Self {
		Self {
			value: namespaceid
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum NamespaceRegistrationType {
	ROOT = 0,
	CHILD = 1,
}
impl NamespaceRegistrationType {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::ROOT
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			0 => Some((NamespaceRegistrationType::ROOT, &payload[1..])),
			1 => Some((NamespaceRegistrationType::CHILD, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("NamespaceRegistrationType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AliasAction {
	UNLINK = 0,
	LINK = 1,
}
impl AliasAction {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::UNLINK
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			0 => Some((AliasAction::UNLINK, &payload[1..])),
			1 => Some((AliasAction::LINK, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("AliasAction::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NamespaceExpiredReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub artifact_id: NamespaceId,
}
impl NamespaceExpiredReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::NAMESPACE_EXPIRED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			artifact_id: NamespaceId::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.artifact_id.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let artifact_id;
		(artifact_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			artifact_id: artifact_id,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.artifact_id.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NamespaceDeletedReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub artifact_id: NamespaceId,
}
impl NamespaceDeletedReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::NAMESPACE_DELETED;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			artifact_id: NamespaceId::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.artifact_id.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let artifact_id;
		(artifact_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			artifact_id: artifact_id,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.artifact_id.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NamespaceRentalFeeReceipt {
	pub version: u16,
	pub type_: ReceiptType,
	pub mosaic: Mosaic,
	pub sender_address: Address,
	pub recipient_address: Address,
}
impl NamespaceRentalFeeReceipt {
	const RECEIPT_TYPE: ReceiptType = ReceiptType::NAMESPACE_RENTAL_FEE;
	pub fn new() -> Self {
		Self {
			version: 0,
			type_: Self::RECEIPT_TYPE,
			mosaic: Mosaic::default(),
			sender_address: Address::default(),
			recipient_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 2;
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.sender_address.size();
		size += self.recipient_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let version = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let type_;
		(type_, payload) = ReceiptType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let sender_address;
		(sender_address, payload) = Address::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			type_: type_,
			mosaic: mosaic,
			sender_address: sender_address,
			recipient_address: recipient_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.sender_address.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct ReceiptSource {
	pub primary_id: u32,
	pub secondary_id: u32,
}
impl ReceiptSource {
	pub fn new() -> Self {
		Self {
			primary_id: 0,
			secondary_id: 0,
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let primary_id = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let secondary_id = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let self_ = Self {
			primary_id: primary_id,
			secondary_id: secondary_id,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.primary_id.to_le_bytes().to_vec());
		serialized.append(&mut self.secondary_id.to_le_bytes().to_vec());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AddressResolutionEntry {
	pub source: ReceiptSource,
	pub resolved_value: Address,
}
impl AddressResolutionEntry {
	pub fn new() -> Self {
		Self {
			source: ReceiptSource::default(),
			resolved_value: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.source.size();
		size += self.resolved_value.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let source;
		(source, payload) = ReceiptSource::deserialize(payload).unwrap();
		let resolved_value;
		(resolved_value, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			source: source,
			resolved_value: resolved_value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.source.serialize());
		serialized.append(&mut self.resolved_value.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AddressResolutionStatement {
	pub unresolved: UnresolvedAddress,
	pub resolution_entries: Vec<AddressResolutionEntry>,
}
impl AddressResolutionStatement {
	pub fn new() -> Self {
		Self {
			unresolved: UnresolvedAddress::default(),
			resolution_entries: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.unresolved.size();
		size += 4;
		size += self.resolution_entries.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let unresolved;
		(unresolved, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let resolution_entries_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut resolution_entries = Vec::new();
		for _ in 0..resolution_entries_count {
			let element;
			(element, payload) = AddressResolutionEntry::deserialize(payload).unwrap();
			resolution_entries.push(element);
		}
		let self_ = Self {
			unresolved: unresolved,
			resolution_entries: resolution_entries,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.unresolved.serialize());
		serialized.append(&mut self.resolution_entries.len().to_le_bytes().to_vec());
		serialized.append(&mut self.resolution_entries.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicResolutionEntry {
	pub source: ReceiptSource,
	pub resolved_value: MosaicId,
}
impl MosaicResolutionEntry {
	pub fn new() -> Self {
		Self {
			source: ReceiptSource::default(),
			resolved_value: MosaicId::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.source.size();
		size += self.resolved_value.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let source;
		(source, payload) = ReceiptSource::deserialize(payload).unwrap();
		let resolved_value;
		(resolved_value, payload) = MosaicId::deserialize(payload).unwrap();
		let self_ = Self {
			source: source,
			resolved_value: resolved_value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.source.serialize());
		serialized.append(&mut self.resolved_value.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicResolutionStatement {
	pub unresolved: UnresolvedMosaicId,
	pub resolution_entries: Vec<MosaicResolutionEntry>,
}
impl MosaicResolutionStatement {
	pub fn new() -> Self {
		Self {
			unresolved: UnresolvedMosaicId::default(),
			resolution_entries: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.unresolved.size();
		size += 4;
		size += self.resolution_entries.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let unresolved;
		(unresolved, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let resolution_entries_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut resolution_entries = Vec::new();
		for _ in 0..resolution_entries_count {
			let element;
			(element, payload) = MosaicResolutionEntry::deserialize(payload).unwrap();
			resolution_entries.push(element);
		}
		let self_ = Self {
			unresolved: unresolved,
			resolution_entries: resolution_entries,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.unresolved.serialize());
		serialized.append(&mut self.resolution_entries.len().to_le_bytes().to_vec());
		serialized.append(&mut self.resolution_entries.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct TransactionStatement {
	pub primary_id: u32,
	pub secondary_id: u32,
	pub receipts: Vec<Receipt>,
}
impl TransactionStatement {
	pub fn new() -> Self {
		Self {
			primary_id: 0,
			secondary_id: 0,
			receipts: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += 4;
		size += self.receipts.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let primary_id = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let secondary_id = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let receipt_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut receipts = Vec::new();
		for _ in 0..receipt_count {
			let element;
			(element, payload) = Receipt::deserialize(payload).unwrap();
			receipts.push(element);
		}
		let self_ = Self {
			primary_id: primary_id,
			secondary_id: secondary_id,
			receipts: receipts,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.primary_id.to_le_bytes().to_vec());
		serialized.append(&mut self.secondary_id.to_le_bytes().to_vec());
		serialized.append(&mut self.receipts.len().to_le_bytes().to_vec());
		serialized.append(&mut self.receipts.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct BlockStatement {
	pub transaction_statements: Vec<TransactionStatement>,
	pub address_resolution_statements: Vec<AddressResolutionStatement>,
	pub mosaic_resolution_statements: Vec<MosaicResolutionStatement>,
}
impl BlockStatement {
	pub fn new() -> Self {
		Self {
			transaction_statements: Vec::new(),
			address_resolution_statements: Vec::new(),
			mosaic_resolution_statements: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += self.transaction_statements.iter().map(|x| x.size()).sum::<usize>();
		size += 4;
		size += self.address_resolution_statements.iter().map(|x| x.size()).sum::<usize>();
		size += 4;
		size += self.mosaic_resolution_statements.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let transaction_statement_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut transaction_statements = Vec::new();
		for _ in 0..transaction_statement_count {
			let element;
			(element, payload) = TransactionStatement::deserialize(payload).unwrap();
			transaction_statements.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let address_resolution_statement_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut address_resolution_statements = Vec::new();
		for _ in 0..address_resolution_statement_count {
			let element;
			(element, payload) = AddressResolutionStatement::deserialize(payload).unwrap();
			address_resolution_statements.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_resolution_statement_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut mosaic_resolution_statements = Vec::new();
		for _ in 0..mosaic_resolution_statement_count {
			let element;
			(element, payload) = MosaicResolutionStatement::deserialize(payload).unwrap();
			mosaic_resolution_statements.push(element);
		}
		let self_ = Self {
			transaction_statements: transaction_statements,
			address_resolution_statements: address_resolution_statements,
			mosaic_resolution_statements: mosaic_resolution_statements,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.transaction_statements.len().to_le_bytes().to_vec());
		serialized.append(&mut self.transaction_statements.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.address_resolution_statements.len().to_le_bytes().to_vec());
		serialized.append(&mut self.address_resolution_statements.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.mosaic_resolution_statements.len().to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic_resolution_statements.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AccountKeyLinkTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub linked_public_key: PublicKey,
	pub link_action: LinkAction,
}
impl AccountKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_KEY_LINK;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			linked_public_key: PublicKey::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.linked_public_key.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			linked_public_key: linked_public_key,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedAccountKeyLinkTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub linked_public_key: PublicKey,
	pub link_action: LinkAction,
}
impl EmbeddedAccountKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_KEY_LINK;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			linked_public_key: PublicKey::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.linked_public_key.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			linked_public_key: linked_public_key,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NodeKeyLinkTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub linked_public_key: PublicKey,
	pub link_action: LinkAction,
}
impl NodeKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::NODE_KEY_LINK;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			linked_public_key: PublicKey::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.linked_public_key.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			linked_public_key: linked_public_key,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedNodeKeyLinkTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub linked_public_key: PublicKey,
	pub link_action: LinkAction,
}
impl EmbeddedNodeKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::NODE_KEY_LINK;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			linked_public_key: PublicKey::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.linked_public_key.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			linked_public_key: linked_public_key,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct Cosignature {
	pub version: u64,
	pub signer_public_key: PublicKey,
	pub signature: Signature,
}
impl Cosignature {
	pub fn new() -> Self {
		Self {
			version: 0,
			signer_public_key: PublicKey::default(),
			signature: Signature::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 8;
		size += self.signer_public_key.size();
		size += self.signature.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let version = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			signer_public_key: signer_public_key,
			signature: signature,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct DetachedCosignature {
	pub version: u64,
	pub signer_public_key: PublicKey,
	pub signature: Signature,
	pub parent_hash: Hash256,
}
impl DetachedCosignature {
	pub fn new() -> Self {
		Self {
			version: 0,
			signer_public_key: PublicKey::default(),
			signature: Signature::default(),
			parent_hash: Hash256::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 8;
		size += self.signer_public_key.size();
		size += self.signature.size();
		size += self.parent_hash.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let version = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let parent_hash;
		(parent_hash, payload) = Hash256::deserialize(payload).unwrap();
		let self_ = Self {
			version: version,
			signer_public_key: signer_public_key,
			signature: signature,
			parent_hash: parent_hash,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.parent_hash.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AggregateCompleteTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub transactions_hash: Hash256,
	pub aggregate_transaction_header_reserved_1: u32,
	pub transactions: Vec<EmbeddedTransaction>,
	pub cosignatures: Vec<Cosignature>,
}
impl AggregateCompleteTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::AGGREGATE_COMPLETE;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			transactions_hash: Hash256::default(),
			aggregate_transaction_header_reserved_1: 0,
			transactions: Vec::new(),
			cosignatures: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.transactions_hash.size();
		size += 4;
		size += 4;
		size += self.transactions.iter().map(|x| x.size()).sum::<usize>();
		size += self.cosignatures.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let payload_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let aggregate_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if aggregate_transaction_header_reserved_1 != 0 { return None; }
		let mut transactions = Vec::new();
		for _ in 0..payload_size {
			let element;
			(element, payload) = EmbeddedTransaction::deserialize(payload).unwrap();
			transactions.push(element);
		}
		let mut cosignatures = Vec::new();
		for _ in 0..0 {
			let element;
			(element, payload) = Cosignature::deserialize(payload).unwrap();
			cosignatures.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			transactions_hash: transactions_hash,
			aggregate_transaction_header_reserved_1: aggregate_transaction_header_reserved_1,
			transactions: transactions,
			cosignatures: cosignatures,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.transactions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.aggregate_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.transactions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.cosignatures.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AggregateCompleteTransactionV2 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub transactions_hash: Hash256,
	pub aggregate_transaction_header_reserved_1: u32,
	pub transactions: Vec<EmbeddedTransaction>,
	pub cosignatures: Vec<Cosignature>,
}
impl AggregateCompleteTransactionV2 {
	const TRANSACTION_VERSION: u8 = 2;
	const TRANSACTION_TYPE: TransactionType = TransactionType::AGGREGATE_COMPLETE;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			transactions_hash: Hash256::default(),
			aggregate_transaction_header_reserved_1: 0,
			transactions: Vec::new(),
			cosignatures: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.transactions_hash.size();
		size += 4;
		size += 4;
		size += self.transactions.iter().map(|x| x.size()).sum::<usize>();
		size += self.cosignatures.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let payload_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let aggregate_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if aggregate_transaction_header_reserved_1 != 0 { return None; }
		let mut transactions = Vec::new();
		for _ in 0..payload_size {
			let element;
			(element, payload) = EmbeddedTransaction::deserialize(payload).unwrap();
			transactions.push(element);
		}
		let mut cosignatures = Vec::new();
		for _ in 0..0 {
			let element;
			(element, payload) = Cosignature::deserialize(payload).unwrap();
			cosignatures.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			transactions_hash: transactions_hash,
			aggregate_transaction_header_reserved_1: aggregate_transaction_header_reserved_1,
			transactions: transactions,
			cosignatures: cosignatures,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.transactions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.aggregate_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.transactions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.cosignatures.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AggregateBondedTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub transactions_hash: Hash256,
	pub aggregate_transaction_header_reserved_1: u32,
	pub transactions: Vec<EmbeddedTransaction>,
	pub cosignatures: Vec<Cosignature>,
}
impl AggregateBondedTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::AGGREGATE_BONDED;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			transactions_hash: Hash256::default(),
			aggregate_transaction_header_reserved_1: 0,
			transactions: Vec::new(),
			cosignatures: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.transactions_hash.size();
		size += 4;
		size += 4;
		size += self.transactions.iter().map(|x| x.size()).sum::<usize>();
		size += self.cosignatures.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let payload_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let aggregate_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if aggregate_transaction_header_reserved_1 != 0 { return None; }
		let mut transactions = Vec::new();
		for _ in 0..payload_size {
			let element;
			(element, payload) = EmbeddedTransaction::deserialize(payload).unwrap();
			transactions.push(element);
		}
		let mut cosignatures = Vec::new();
		for _ in 0..0 {
			let element;
			(element, payload) = Cosignature::deserialize(payload).unwrap();
			cosignatures.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			transactions_hash: transactions_hash,
			aggregate_transaction_header_reserved_1: aggregate_transaction_header_reserved_1,
			transactions: transactions,
			cosignatures: cosignatures,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.transactions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.aggregate_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.transactions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.cosignatures.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AggregateBondedTransactionV2 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub transactions_hash: Hash256,
	pub aggregate_transaction_header_reserved_1: u32,
	pub transactions: Vec<EmbeddedTransaction>,
	pub cosignatures: Vec<Cosignature>,
}
impl AggregateBondedTransactionV2 {
	const TRANSACTION_VERSION: u8 = 2;
	const TRANSACTION_TYPE: TransactionType = TransactionType::AGGREGATE_BONDED;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			transactions_hash: Hash256::default(),
			aggregate_transaction_header_reserved_1: 0,
			transactions: Vec::new(),
			cosignatures: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.transactions_hash.size();
		size += 4;
		size += 4;
		size += self.transactions.iter().map(|x| x.size()).sum::<usize>();
		size += self.cosignatures.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let transactions_hash;
		(transactions_hash, payload) = Hash256::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let payload_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let aggregate_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if aggregate_transaction_header_reserved_1 != 0 { return None; }
		let mut transactions = Vec::new();
		for _ in 0..payload_size {
			let element;
			(element, payload) = EmbeddedTransaction::deserialize(payload).unwrap();
			transactions.push(element);
		}
		let mut cosignatures = Vec::new();
		for _ in 0..0 {
			let element;
			(element, payload) = Cosignature::deserialize(payload).unwrap();
			cosignatures.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			transactions_hash: transactions_hash,
			aggregate_transaction_header_reserved_1: aggregate_transaction_header_reserved_1,
			transactions: transactions,
			cosignatures: cosignatures,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.transactions_hash.serialize());
		serialized.append(&mut self.transactions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.aggregate_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.transactions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.cosignatures.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct VotingKeyLinkTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub linked_public_key: VotingPublicKey,
	pub start_epoch: FinalizationEpoch,
	pub end_epoch: FinalizationEpoch,
	pub link_action: LinkAction,
}
impl VotingKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::VOTING_KEY_LINK;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			linked_public_key: VotingPublicKey::default(),
			start_epoch: FinalizationEpoch::default(),
			end_epoch: FinalizationEpoch::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.linked_public_key.size();
		size += self.start_epoch.size();
		size += self.end_epoch.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = VotingPublicKey::deserialize(payload).unwrap();
		let start_epoch;
		(start_epoch, payload) = FinalizationEpoch::deserialize(payload).unwrap();
		let end_epoch;
		(end_epoch, payload) = FinalizationEpoch::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			linked_public_key: linked_public_key,
			start_epoch: start_epoch,
			end_epoch: end_epoch,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.start_epoch.serialize());
		serialized.append(&mut self.end_epoch.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedVotingKeyLinkTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub linked_public_key: VotingPublicKey,
	pub start_epoch: FinalizationEpoch,
	pub end_epoch: FinalizationEpoch,
	pub link_action: LinkAction,
}
impl EmbeddedVotingKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::VOTING_KEY_LINK;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			linked_public_key: VotingPublicKey::default(),
			start_epoch: FinalizationEpoch::default(),
			end_epoch: FinalizationEpoch::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.linked_public_key.size();
		size += self.start_epoch.size();
		size += self.end_epoch.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = VotingPublicKey::deserialize(payload).unwrap();
		let start_epoch;
		(start_epoch, payload) = FinalizationEpoch::deserialize(payload).unwrap();
		let end_epoch;
		(end_epoch, payload) = FinalizationEpoch::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			linked_public_key: linked_public_key,
			start_epoch: start_epoch,
			end_epoch: end_epoch,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.start_epoch.serialize());
		serialized.append(&mut self.end_epoch.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct VrfKeyLinkTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub linked_public_key: PublicKey,
	pub link_action: LinkAction,
}
impl VrfKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::VRF_KEY_LINK;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			linked_public_key: PublicKey::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.linked_public_key.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			linked_public_key: linked_public_key,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedVrfKeyLinkTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub linked_public_key: PublicKey,
	pub link_action: LinkAction,
}
impl EmbeddedVrfKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::VRF_KEY_LINK;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			linked_public_key: PublicKey::default(),
			link_action: LinkAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.linked_public_key.size();
		size += self.link_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let linked_public_key;
		(linked_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			linked_public_key: linked_public_key,
			link_action: link_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.linked_public_key.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct HashLockTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic: UnresolvedMosaic,
	pub duration: BlockDuration,
	pub hash: Hash256,
}
impl HashLockTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::HASH_LOCK;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic: UnresolvedMosaic::default(),
			duration: BlockDuration::default(),
			hash: Hash256::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.mosaic.size();
		size += self.duration.size();
		size += self.hash.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let hash;
		(hash, payload) = Hash256::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			mosaic: mosaic,
			duration: duration,
			hash: hash,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.hash.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedHashLockTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub mosaic: UnresolvedMosaic,
	pub duration: BlockDuration,
	pub hash: Hash256,
}
impl EmbeddedHashLockTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::HASH_LOCK;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			mosaic: UnresolvedMosaic::default(),
			duration: BlockDuration::default(),
			hash: Hash256::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.mosaic.size();
		size += self.duration.size();
		size += self.hash.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let hash;
		(hash, payload) = Hash256::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			mosaic: mosaic,
			duration: duration,
			hash: hash,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.hash.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LockHashAlgorithm {
	SHA3_256 = 0,
	HASH_160 = 1,
	HASH_256 = 2,
}
impl LockHashAlgorithm {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::SHA3_256
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			0 => Some((LockHashAlgorithm::SHA3_256, &payload[1..])),
			1 => Some((LockHashAlgorithm::HASH_160, &payload[1..])),
			2 => Some((LockHashAlgorithm::HASH_256, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("LockHashAlgorithm::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct SecretLockTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub recipient_address: UnresolvedAddress,
	pub secret: Hash256,
	pub mosaic: UnresolvedMosaic,
	pub duration: BlockDuration,
	pub hash_algorithm: LockHashAlgorithm,
}
impl SecretLockTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::SECRET_LOCK;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			recipient_address: UnresolvedAddress::default(),
			secret: Hash256::default(),
			mosaic: UnresolvedMosaic::default(),
			duration: BlockDuration::default(),
			hash_algorithm: LockHashAlgorithm::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.recipient_address.size();
		size += self.secret.size();
		size += self.mosaic.size();
		size += self.duration.size();
		size += self.hash_algorithm.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let secret;
		(secret, payload) = Hash256::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let hash_algorithm;
		(hash_algorithm, payload) = LockHashAlgorithm::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			recipient_address: recipient_address,
			secret: secret,
			mosaic: mosaic,
			duration: duration,
			hash_algorithm: hash_algorithm,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.secret.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.hash_algorithm.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedSecretLockTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub recipient_address: UnresolvedAddress,
	pub secret: Hash256,
	pub mosaic: UnresolvedMosaic,
	pub duration: BlockDuration,
	pub hash_algorithm: LockHashAlgorithm,
}
impl EmbeddedSecretLockTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::SECRET_LOCK;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			recipient_address: UnresolvedAddress::default(),
			secret: Hash256::default(),
			mosaic: UnresolvedMosaic::default(),
			duration: BlockDuration::default(),
			hash_algorithm: LockHashAlgorithm::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.recipient_address.size();
		size += self.secret.size();
		size += self.mosaic.size();
		size += self.duration.size();
		size += self.hash_algorithm.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let secret;
		(secret, payload) = Hash256::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let hash_algorithm;
		(hash_algorithm, payload) = LockHashAlgorithm::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			recipient_address: recipient_address,
			secret: secret,
			mosaic: mosaic,
			duration: duration,
			hash_algorithm: hash_algorithm,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.secret.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.hash_algorithm.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct SecretProofTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub recipient_address: UnresolvedAddress,
	pub secret: Hash256,
	pub hash_algorithm: LockHashAlgorithm,
	pub proof: Vec<u8>,
}
impl SecretProofTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::SECRET_PROOF;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			recipient_address: UnresolvedAddress::default(),
			secret: Hash256::default(),
			hash_algorithm: LockHashAlgorithm::default(),
			proof: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.recipient_address.size();
		size += self.secret.size();
		size += 2;
		size += self.hash_algorithm.size();
		size += 1 * self.proof.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let secret;
		(secret, payload) = Hash256::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let proof_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let hash_algorithm;
		(hash_algorithm, payload) = LockHashAlgorithm::deserialize(payload).unwrap();
		let mut proof = Vec::new();
		for _ in 0..proof_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[proof_size as usize..];
			proof.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			recipient_address: recipient_address,
			secret: secret,
			hash_algorithm: hash_algorithm,
			proof: proof,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.secret.serialize());
		serialized.append(&mut self.proof.len().to_le_bytes().to_vec());
		serialized.append(&mut self.hash_algorithm.serialize());
		serialized.append(&mut self.proof.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedSecretProofTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub recipient_address: UnresolvedAddress,
	pub secret: Hash256,
	pub hash_algorithm: LockHashAlgorithm,
	pub proof: Vec<u8>,
}
impl EmbeddedSecretProofTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::SECRET_PROOF;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			recipient_address: UnresolvedAddress::default(),
			secret: Hash256::default(),
			hash_algorithm: LockHashAlgorithm::default(),
			proof: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.recipient_address.size();
		size += self.secret.size();
		size += 2;
		size += self.hash_algorithm.size();
		size += 1 * self.proof.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let secret;
		(secret, payload) = Hash256::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let proof_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let hash_algorithm;
		(hash_algorithm, payload) = LockHashAlgorithm::deserialize(payload).unwrap();
		let mut proof = Vec::new();
		for _ in 0..proof_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[proof_size as usize..];
			proof.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			recipient_address: recipient_address,
			secret: secret,
			hash_algorithm: hash_algorithm,
			proof: proof,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.secret.serialize());
		serialized.append(&mut self.proof.len().to_le_bytes().to_vec());
		serialized.append(&mut self.hash_algorithm.serialize());
		serialized.append(&mut self.proof.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AccountMetadataTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub target_address: UnresolvedAddress,
	pub scoped_metadata_key: u64,
	pub value_size_delta: i16,
	pub value: Vec<u8>,
}
impl AccountMetadataTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_METADATA;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			target_address: UnresolvedAddress::default(),
			scoped_metadata_key: 0,
			value_size_delta: 0,
			value: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.target_address.size();
		size += 8;
		size += 2;
		size += 2;
		size += 1 * self.value.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let scoped_metadata_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size_delta = i16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut value = Vec::new();
		for _ in 0..value_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[value_size as usize..];
			value.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			target_address: target_address,
			scoped_metadata_key: scoped_metadata_key,
			value_size_delta: value_size_delta,
			value: value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized.append(&mut self.scoped_metadata_key.to_le_bytes().to_vec());
		serialized.append(&mut self.value_size_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.value.len().to_le_bytes().to_vec());
		serialized.append(&mut self.value.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedAccountMetadataTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub target_address: UnresolvedAddress,
	pub scoped_metadata_key: u64,
	pub value_size_delta: i16,
	pub value: Vec<u8>,
}
impl EmbeddedAccountMetadataTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_METADATA;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			target_address: UnresolvedAddress::default(),
			scoped_metadata_key: 0,
			value_size_delta: 0,
			value: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.target_address.size();
		size += 8;
		size += 2;
		size += 2;
		size += 1 * self.value.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let scoped_metadata_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size_delta = i16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut value = Vec::new();
		for _ in 0..value_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[value_size as usize..];
			value.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			target_address: target_address,
			scoped_metadata_key: scoped_metadata_key,
			value_size_delta: value_size_delta,
			value: value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized.append(&mut self.scoped_metadata_key.to_le_bytes().to_vec());
		serialized.append(&mut self.value_size_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.value.len().to_le_bytes().to_vec());
		serialized.append(&mut self.value.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicMetadataTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub target_address: UnresolvedAddress,
	pub scoped_metadata_key: u64,
	pub target_mosaic_id: UnresolvedMosaicId,
	pub value_size_delta: i16,
	pub value: Vec<u8>,
}
impl MosaicMetadataTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_METADATA;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			target_address: UnresolvedAddress::default(),
			scoped_metadata_key: 0,
			target_mosaic_id: UnresolvedMosaicId::default(),
			value_size_delta: 0,
			value: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.target_address.size();
		size += 8;
		size += self.target_mosaic_id.size();
		size += 2;
		size += 2;
		size += 1 * self.value.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let scoped_metadata_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let target_mosaic_id;
		(target_mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size_delta = i16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut value = Vec::new();
		for _ in 0..value_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[value_size as usize..];
			value.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			target_address: target_address,
			scoped_metadata_key: scoped_metadata_key,
			target_mosaic_id: target_mosaic_id,
			value_size_delta: value_size_delta,
			value: value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized.append(&mut self.scoped_metadata_key.to_le_bytes().to_vec());
		serialized.append(&mut self.target_mosaic_id.serialize());
		serialized.append(&mut self.value_size_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.value.len().to_le_bytes().to_vec());
		serialized.append(&mut self.value.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMosaicMetadataTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub target_address: UnresolvedAddress,
	pub scoped_metadata_key: u64,
	pub target_mosaic_id: UnresolvedMosaicId,
	pub value_size_delta: i16,
	pub value: Vec<u8>,
}
impl EmbeddedMosaicMetadataTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_METADATA;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			target_address: UnresolvedAddress::default(),
			scoped_metadata_key: 0,
			target_mosaic_id: UnresolvedMosaicId::default(),
			value_size_delta: 0,
			value: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.target_address.size();
		size += 8;
		size += self.target_mosaic_id.size();
		size += 2;
		size += 2;
		size += 1 * self.value.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let scoped_metadata_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let target_mosaic_id;
		(target_mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size_delta = i16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut value = Vec::new();
		for _ in 0..value_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[value_size as usize..];
			value.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			target_address: target_address,
			scoped_metadata_key: scoped_metadata_key,
			target_mosaic_id: target_mosaic_id,
			value_size_delta: value_size_delta,
			value: value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized.append(&mut self.scoped_metadata_key.to_le_bytes().to_vec());
		serialized.append(&mut self.target_mosaic_id.serialize());
		serialized.append(&mut self.value_size_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.value.len().to_le_bytes().to_vec());
		serialized.append(&mut self.value.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NamespaceMetadataTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub target_address: UnresolvedAddress,
	pub scoped_metadata_key: u64,
	pub target_namespace_id: NamespaceId,
	pub value_size_delta: i16,
	pub value: Vec<u8>,
}
impl NamespaceMetadataTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::NAMESPACE_METADATA;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			target_address: UnresolvedAddress::default(),
			scoped_metadata_key: 0,
			target_namespace_id: NamespaceId::default(),
			value_size_delta: 0,
			value: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.target_address.size();
		size += 8;
		size += self.target_namespace_id.size();
		size += 2;
		size += 2;
		size += 1 * self.value.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let scoped_metadata_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let target_namespace_id;
		(target_namespace_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size_delta = i16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut value = Vec::new();
		for _ in 0..value_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[value_size as usize..];
			value.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			target_address: target_address,
			scoped_metadata_key: scoped_metadata_key,
			target_namespace_id: target_namespace_id,
			value_size_delta: value_size_delta,
			value: value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized.append(&mut self.scoped_metadata_key.to_le_bytes().to_vec());
		serialized.append(&mut self.target_namespace_id.serialize());
		serialized.append(&mut self.value_size_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.value.len().to_le_bytes().to_vec());
		serialized.append(&mut self.value.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedNamespaceMetadataTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub target_address: UnresolvedAddress,
	pub scoped_metadata_key: u64,
	pub target_namespace_id: NamespaceId,
	pub value_size_delta: i16,
	pub value: Vec<u8>,
}
impl EmbeddedNamespaceMetadataTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::NAMESPACE_METADATA;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			target_address: UnresolvedAddress::default(),
			scoped_metadata_key: 0,
			target_namespace_id: NamespaceId::default(),
			value_size_delta: 0,
			value: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.target_address.size();
		size += 8;
		size += self.target_namespace_id.size();
		size += 2;
		size += 2;
		size += 1 * self.value.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let scoped_metadata_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let target_namespace_id;
		(target_namespace_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size_delta = i16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let value_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut value = Vec::new();
		for _ in 0..value_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[value_size as usize..];
			value.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			target_address: target_address,
			scoped_metadata_key: scoped_metadata_key,
			target_namespace_id: target_namespace_id,
			value_size_delta: value_size_delta,
			value: value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.target_address.serialize());
		serialized.append(&mut self.scoped_metadata_key.to_le_bytes().to_vec());
		serialized.append(&mut self.target_namespace_id.serialize());
		serialized.append(&mut self.value_size_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.value.len().to_le_bytes().to_vec());
		serialized.append(&mut self.value.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MosaicNonce {
	value: u32
}
impl MosaicNonce {
	const SIZE: usize = 4;
	pub fn new(mosaicnonce: u32) -> Self {
		Self {
			value: mosaicnonce
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let value = u32::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..4]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:08x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MosaicFlags {
	NONE = 0,
	SUPPLY_MUTABLE = 1,
	TRANSFERABLE = 2,
	RESTRICTABLE = 4,
	REVOKABLE = 8,
}
impl MosaicFlags {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::NONE
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			0 => Some((MosaicFlags::NONE, &payload[1..])),
			1 => Some((MosaicFlags::SUPPLY_MUTABLE, &payload[1..])),
			2 => Some((MosaicFlags::TRANSFERABLE, &payload[1..])),
			4 => Some((MosaicFlags::RESTRICTABLE, &payload[1..])),
			8 => Some((MosaicFlags::REVOKABLE, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("MosaicFlags::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MosaicSupplyChangeAction {
	DECREASE = 0,
	INCREASE = 1,
}
impl MosaicSupplyChangeAction {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::DECREASE
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			0 => Some((MosaicSupplyChangeAction::DECREASE, &payload[1..])),
			1 => Some((MosaicSupplyChangeAction::INCREASE, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("MosaicSupplyChangeAction::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicDefinitionTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub id: MosaicId,
	pub duration: BlockDuration,
	pub nonce: MosaicNonce,
	pub flags: MosaicFlags,
	pub divisibility: u8,
}
impl MosaicDefinitionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_DEFINITION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			id: MosaicId::default(),
			duration: BlockDuration::default(),
			nonce: MosaicNonce::default(),
			flags: MosaicFlags::default(),
			divisibility: 0,
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.id.size();
		size += self.duration.size();
		size += self.nonce.size();
		size += self.flags.size();
		size += 1;
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let id;
		(id, payload) = MosaicId::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let nonce;
		(nonce, payload) = MosaicNonce::deserialize(payload).unwrap();
		let flags;
		(flags, payload) = MosaicFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let divisibility = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			id: id,
			duration: duration,
			nonce: nonce,
			flags: flags,
			divisibility: divisibility,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.id.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.nonce.serialize());
		serialized.append(&mut self.flags.serialize());
		serialized.append(&mut self.divisibility.to_le_bytes().to_vec());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMosaicDefinitionTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub id: MosaicId,
	pub duration: BlockDuration,
	pub nonce: MosaicNonce,
	pub flags: MosaicFlags,
	pub divisibility: u8,
}
impl EmbeddedMosaicDefinitionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_DEFINITION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			id: MosaicId::default(),
			duration: BlockDuration::default(),
			nonce: MosaicNonce::default(),
			flags: MosaicFlags::default(),
			divisibility: 0,
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.id.size();
		size += self.duration.size();
		size += self.nonce.size();
		size += self.flags.size();
		size += 1;
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let id;
		(id, payload) = MosaicId::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let nonce;
		(nonce, payload) = MosaicNonce::deserialize(payload).unwrap();
		let flags;
		(flags, payload) = MosaicFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let divisibility = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			id: id,
			duration: duration,
			nonce: nonce,
			flags: flags,
			divisibility: divisibility,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.id.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.nonce.serialize());
		serialized.append(&mut self.flags.serialize());
		serialized.append(&mut self.divisibility.to_le_bytes().to_vec());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicSupplyChangeTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic_id: UnresolvedMosaicId,
	pub delta: Amount,
	pub action: MosaicSupplyChangeAction,
}
impl MosaicSupplyChangeTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_SUPPLY_CHANGE;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic_id: UnresolvedMosaicId::default(),
			delta: Amount::default(),
			action: MosaicSupplyChangeAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.mosaic_id.size();
		size += self.delta.size();
		size += self.action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let delta;
		(delta, payload) = Amount::deserialize(payload).unwrap();
		let action;
		(action, payload) = MosaicSupplyChangeAction::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			mosaic_id: mosaic_id,
			delta: delta,
			action: action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.delta.serialize());
		serialized.append(&mut self.action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMosaicSupplyChangeTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub mosaic_id: UnresolvedMosaicId,
	pub delta: Amount,
	pub action: MosaicSupplyChangeAction,
}
impl EmbeddedMosaicSupplyChangeTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_SUPPLY_CHANGE;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			mosaic_id: UnresolvedMosaicId::default(),
			delta: Amount::default(),
			action: MosaicSupplyChangeAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.mosaic_id.size();
		size += self.delta.size();
		size += self.action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let delta;
		(delta, payload) = Amount::deserialize(payload).unwrap();
		let action;
		(action, payload) = MosaicSupplyChangeAction::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			mosaic_id: mosaic_id,
			delta: delta,
			action: action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.delta.serialize());
		serialized.append(&mut self.action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicSupplyRevocationTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub source_address: UnresolvedAddress,
	pub mosaic: UnresolvedMosaic,
}
impl MosaicSupplyRevocationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_SUPPLY_REVOCATION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			source_address: UnresolvedAddress::default(),
			mosaic: UnresolvedMosaic::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.source_address.size();
		size += self.mosaic.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let source_address;
		(source_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			source_address: source_address,
			mosaic: mosaic,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.source_address.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMosaicSupplyRevocationTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub source_address: UnresolvedAddress,
	pub mosaic: UnresolvedMosaic,
}
impl EmbeddedMosaicSupplyRevocationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_SUPPLY_REVOCATION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			source_address: UnresolvedAddress::default(),
			mosaic: UnresolvedMosaic::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.source_address.size();
		size += self.mosaic.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let source_address;
		(source_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mosaic;
		(mosaic, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			source_address: source_address,
			mosaic: mosaic,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.source_address.serialize());
		serialized.append(&mut self.mosaic.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MultisigAccountModificationTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub min_removal_delta: i8,
	pub min_approval_delta: i8,
	pub multisig_account_modification_transaction_body_reserved_1: u32,
	pub address_additions: Vec<UnresolvedAddress>,
	pub address_deletions: Vec<UnresolvedAddress>,
}
impl MultisigAccountModificationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_ACCOUNT_MODIFICATION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			min_removal_delta: 0,
			min_approval_delta: 0,
			multisig_account_modification_transaction_body_reserved_1: 0,
			address_additions: Vec::new(),
			address_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 1;
		size += 1;
		size += 1;
		size += 1;
		size += 4;
		size += self.address_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.address_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let min_removal_delta = i8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let min_approval_delta = i8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let address_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let address_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let multisig_account_modification_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if multisig_account_modification_transaction_body_reserved_1 != 0 { return None; }
		let mut address_additions = Vec::new();
		for _ in 0..address_additions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			address_additions.push(element);
		}
		let mut address_deletions = Vec::new();
		for _ in 0..address_deletions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			address_deletions.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			min_removal_delta: min_removal_delta,
			min_approval_delta: min_approval_delta,
			multisig_account_modification_transaction_body_reserved_1: multisig_account_modification_transaction_body_reserved_1,
			address_additions: address_additions,
			address_deletions: address_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.min_removal_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.min_approval_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.address_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.address_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.multisig_account_modification_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.address_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.address_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMultisigAccountModificationTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub min_removal_delta: i8,
	pub min_approval_delta: i8,
	pub multisig_account_modification_transaction_body_reserved_1: u32,
	pub address_additions: Vec<UnresolvedAddress>,
	pub address_deletions: Vec<UnresolvedAddress>,
}
impl EmbeddedMultisigAccountModificationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_ACCOUNT_MODIFICATION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			min_removal_delta: 0,
			min_approval_delta: 0,
			multisig_account_modification_transaction_body_reserved_1: 0,
			address_additions: Vec::new(),
			address_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += 1;
		size += 1;
		size += 1;
		size += 1;
		size += 4;
		size += self.address_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.address_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let min_removal_delta = i8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let min_approval_delta = i8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let address_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let address_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let multisig_account_modification_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if multisig_account_modification_transaction_body_reserved_1 != 0 { return None; }
		let mut address_additions = Vec::new();
		for _ in 0..address_additions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			address_additions.push(element);
		}
		let mut address_deletions = Vec::new();
		for _ in 0..address_deletions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			address_deletions.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			min_removal_delta: min_removal_delta,
			min_approval_delta: min_approval_delta,
			multisig_account_modification_transaction_body_reserved_1: multisig_account_modification_transaction_body_reserved_1,
			address_additions: address_additions,
			address_deletions: address_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.min_removal_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.min_approval_delta.to_le_bytes().to_vec());
		serialized.append(&mut self.address_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.address_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.multisig_account_modification_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.address_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.address_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AddressAliasTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub namespace_id: NamespaceId,
	pub address: Address,
	pub alias_action: AliasAction,
}
impl AddressAliasTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ADDRESS_ALIAS;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			namespace_id: NamespaceId::default(),
			address: Address::default(),
			alias_action: AliasAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.namespace_id.size();
		size += self.address.size();
		size += self.alias_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let namespace_id;
		(namespace_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let address;
		(address, payload) = Address::deserialize(payload).unwrap();
		let alias_action;
		(alias_action, payload) = AliasAction::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			namespace_id: namespace_id,
			address: address,
			alias_action: alias_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.namespace_id.serialize());
		serialized.append(&mut self.address.serialize());
		serialized.append(&mut self.alias_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedAddressAliasTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub namespace_id: NamespaceId,
	pub address: Address,
	pub alias_action: AliasAction,
}
impl EmbeddedAddressAliasTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ADDRESS_ALIAS;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			namespace_id: NamespaceId::default(),
			address: Address::default(),
			alias_action: AliasAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.namespace_id.size();
		size += self.address.size();
		size += self.alias_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let namespace_id;
		(namespace_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let address;
		(address, payload) = Address::deserialize(payload).unwrap();
		let alias_action;
		(alias_action, payload) = AliasAction::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			namespace_id: namespace_id,
			address: address,
			alias_action: alias_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.namespace_id.serialize());
		serialized.append(&mut self.address.serialize());
		serialized.append(&mut self.alias_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicAliasTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub namespace_id: NamespaceId,
	pub mosaic_id: MosaicId,
	pub alias_action: AliasAction,
}
impl MosaicAliasTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_ALIAS;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			namespace_id: NamespaceId::default(),
			mosaic_id: MosaicId::default(),
			alias_action: AliasAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.namespace_id.size();
		size += self.mosaic_id.size();
		size += self.alias_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let namespace_id;
		(namespace_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = MosaicId::deserialize(payload).unwrap();
		let alias_action;
		(alias_action, payload) = AliasAction::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			namespace_id: namespace_id,
			mosaic_id: mosaic_id,
			alias_action: alias_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.namespace_id.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.alias_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMosaicAliasTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub namespace_id: NamespaceId,
	pub mosaic_id: MosaicId,
	pub alias_action: AliasAction,
}
impl EmbeddedMosaicAliasTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_ALIAS;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			namespace_id: NamespaceId::default(),
			mosaic_id: MosaicId::default(),
			alias_action: AliasAction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.namespace_id.size();
		size += self.mosaic_id.size();
		size += self.alias_action.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let namespace_id;
		(namespace_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = MosaicId::deserialize(payload).unwrap();
		let alias_action;
		(alias_action, payload) = AliasAction::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			namespace_id: namespace_id,
			mosaic_id: mosaic_id,
			alias_action: alias_action,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.namespace_id.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.alias_action.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct NamespaceRegistrationTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub duration: BlockDuration,
	pub parent_id: NamespaceId,
	pub id: NamespaceId,
	pub registration_type: NamespaceRegistrationType,
	pub name: Vec<u8>,
}
impl NamespaceRegistrationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::NAMESPACE_REGISTRATION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			duration: BlockDuration::default(),
			parent_id: NamespaceId::default(),
			id: NamespaceId::default(),
			registration_type: NamespaceRegistrationType::default(),
			name: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.duration.size();
		size += self.parent_id.size();
		size += self.id.size();
		size += self.registration_type.size();
		size += 1;
		size += 1 * self.name.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let parent_id;
		(parent_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let id;
		(id, payload) = NamespaceId::deserialize(payload).unwrap();
		let registration_type;
		(registration_type, payload) = NamespaceRegistrationType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let name_size = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut name = Vec::new();
		for _ in 0..name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[name_size as usize..];
			name.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			duration: duration,
			parent_id: parent_id,
			id: id,
			registration_type: registration_type,
			name: name,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.parent_id.serialize());
		serialized.append(&mut self.id.serialize());
		serialized.append(&mut self.registration_type.serialize());
		serialized.append(&mut self.name.len().to_le_bytes().to_vec());
		serialized.append(&mut self.name.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedNamespaceRegistrationTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub duration: BlockDuration,
	pub parent_id: NamespaceId,
	pub id: NamespaceId,
	pub registration_type: NamespaceRegistrationType,
	pub name: Vec<u8>,
}
impl EmbeddedNamespaceRegistrationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::NAMESPACE_REGISTRATION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			duration: BlockDuration::default(),
			parent_id: NamespaceId::default(),
			id: NamespaceId::default(),
			registration_type: NamespaceRegistrationType::default(),
			name: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.duration.size();
		size += self.parent_id.size();
		size += self.id.size();
		size += self.registration_type.size();
		size += 1;
		size += 1 * self.name.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let duration;
		(duration, payload) = BlockDuration::deserialize(payload).unwrap();
		let parent_id;
		(parent_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let id;
		(id, payload) = NamespaceId::deserialize(payload).unwrap();
		let registration_type;
		(registration_type, payload) = NamespaceRegistrationType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let name_size = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut name = Vec::new();
		for _ in 0..name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[name_size as usize..];
			name.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			duration: duration,
			parent_id: parent_id,
			id: id,
			registration_type: registration_type,
			name: name,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.duration.serialize());
		serialized.append(&mut self.parent_id.serialize());
		serialized.append(&mut self.id.serialize());
		serialized.append(&mut self.registration_type.serialize());
		serialized.append(&mut self.name.len().to_le_bytes().to_vec());
		serialized.append(&mut self.name.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AccountRestrictionFlags {
	ADDRESS = 1,
	MOSAIC_ID = 2,
	TRANSACTION_TYPE = 4,
	OUTGOING = 16384,
	BLOCK = 32768,
}
impl AccountRestrictionFlags {
	const SIZE: usize = 2;
	pub fn default() -> Self {
		Self::ADDRESS
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 2 { return None; }
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		match u16::from_le_bytes(bytes) {
			1 => Some((AccountRestrictionFlags::ADDRESS, &payload[2..])),
			2 => Some((AccountRestrictionFlags::MOSAIC_ID, &payload[2..])),
			4 => Some((AccountRestrictionFlags::TRANSACTION_TYPE, &payload[2..])),
			16384 => Some((AccountRestrictionFlags::OUTGOING, &payload[2..])),
			32768 => Some((AccountRestrictionFlags::BLOCK, &payload[2..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u16).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("AccountRestrictionFlags::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AccountAddressRestrictionTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub restriction_flags: AccountRestrictionFlags,
	pub account_restriction_transaction_body_reserved_1: u32,
	pub restriction_additions: Vec<UnresolvedAddress>,
	pub restriction_deletions: Vec<UnresolvedAddress>,
}
impl AccountAddressRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_ADDRESS_RESTRICTION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			restriction_flags: AccountRestrictionFlags::default(),
			account_restriction_transaction_body_reserved_1: 0,
			restriction_additions: Vec::new(),
			restriction_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.restriction_flags.size();
		size += 1;
		size += 1;
		size += 4;
		size += self.restriction_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.restriction_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let restriction_flags;
		(restriction_flags, payload) = AccountRestrictionFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let account_restriction_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if account_restriction_transaction_body_reserved_1 != 0 { return None; }
		let mut restriction_additions = Vec::new();
		for _ in 0..restriction_additions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			restriction_additions.push(element);
		}
		let mut restriction_deletions = Vec::new();
		for _ in 0..restriction_deletions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			restriction_deletions.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			restriction_flags: restriction_flags,
			account_restriction_transaction_body_reserved_1: account_restriction_transaction_body_reserved_1,
			restriction_additions: restriction_additions,
			restriction_deletions: restriction_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.restriction_flags.serialize());
		serialized.append(&mut self.restriction_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.account_restriction_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.restriction_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedAccountAddressRestrictionTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub restriction_flags: AccountRestrictionFlags,
	pub account_restriction_transaction_body_reserved_1: u32,
	pub restriction_additions: Vec<UnresolvedAddress>,
	pub restriction_deletions: Vec<UnresolvedAddress>,
}
impl EmbeddedAccountAddressRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_ADDRESS_RESTRICTION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			restriction_flags: AccountRestrictionFlags::default(),
			account_restriction_transaction_body_reserved_1: 0,
			restriction_additions: Vec::new(),
			restriction_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.restriction_flags.size();
		size += 1;
		size += 1;
		size += 4;
		size += self.restriction_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.restriction_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let restriction_flags;
		(restriction_flags, payload) = AccountRestrictionFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let account_restriction_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if account_restriction_transaction_body_reserved_1 != 0 { return None; }
		let mut restriction_additions = Vec::new();
		for _ in 0..restriction_additions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			restriction_additions.push(element);
		}
		let mut restriction_deletions = Vec::new();
		for _ in 0..restriction_deletions_count {
			let element;
			(element, payload) = UnresolvedAddress::deserialize(payload).unwrap();
			restriction_deletions.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			restriction_flags: restriction_flags,
			account_restriction_transaction_body_reserved_1: account_restriction_transaction_body_reserved_1,
			restriction_additions: restriction_additions,
			restriction_deletions: restriction_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.restriction_flags.serialize());
		serialized.append(&mut self.restriction_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.account_restriction_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.restriction_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AccountMosaicRestrictionTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub restriction_flags: AccountRestrictionFlags,
	pub account_restriction_transaction_body_reserved_1: u32,
	pub restriction_additions: Vec<UnresolvedMosaicId>,
	pub restriction_deletions: Vec<UnresolvedMosaicId>,
}
impl AccountMosaicRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_MOSAIC_RESTRICTION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			restriction_flags: AccountRestrictionFlags::default(),
			account_restriction_transaction_body_reserved_1: 0,
			restriction_additions: Vec::new(),
			restriction_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.restriction_flags.size();
		size += 1;
		size += 1;
		size += 4;
		size += self.restriction_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.restriction_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let restriction_flags;
		(restriction_flags, payload) = AccountRestrictionFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let account_restriction_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if account_restriction_transaction_body_reserved_1 != 0 { return None; }
		let mut restriction_additions = Vec::new();
		for _ in 0..restriction_additions_count {
			let element;
			(element, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
			restriction_additions.push(element);
		}
		let mut restriction_deletions = Vec::new();
		for _ in 0..restriction_deletions_count {
			let element;
			(element, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
			restriction_deletions.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			restriction_flags: restriction_flags,
			account_restriction_transaction_body_reserved_1: account_restriction_transaction_body_reserved_1,
			restriction_additions: restriction_additions,
			restriction_deletions: restriction_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.restriction_flags.serialize());
		serialized.append(&mut self.restriction_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.account_restriction_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.restriction_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedAccountMosaicRestrictionTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub restriction_flags: AccountRestrictionFlags,
	pub account_restriction_transaction_body_reserved_1: u32,
	pub restriction_additions: Vec<UnresolvedMosaicId>,
	pub restriction_deletions: Vec<UnresolvedMosaicId>,
}
impl EmbeddedAccountMosaicRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_MOSAIC_RESTRICTION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			restriction_flags: AccountRestrictionFlags::default(),
			account_restriction_transaction_body_reserved_1: 0,
			restriction_additions: Vec::new(),
			restriction_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.restriction_flags.size();
		size += 1;
		size += 1;
		size += 4;
		size += self.restriction_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.restriction_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let restriction_flags;
		(restriction_flags, payload) = AccountRestrictionFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let account_restriction_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if account_restriction_transaction_body_reserved_1 != 0 { return None; }
		let mut restriction_additions = Vec::new();
		for _ in 0..restriction_additions_count {
			let element;
			(element, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
			restriction_additions.push(element);
		}
		let mut restriction_deletions = Vec::new();
		for _ in 0..restriction_deletions_count {
			let element;
			(element, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
			restriction_deletions.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			restriction_flags: restriction_flags,
			account_restriction_transaction_body_reserved_1: account_restriction_transaction_body_reserved_1,
			restriction_additions: restriction_additions,
			restriction_deletions: restriction_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.restriction_flags.serialize());
		serialized.append(&mut self.restriction_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.account_restriction_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.restriction_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct AccountOperationRestrictionTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub restriction_flags: AccountRestrictionFlags,
	pub account_restriction_transaction_body_reserved_1: u32,
	pub restriction_additions: Vec<TransactionType>,
	pub restriction_deletions: Vec<TransactionType>,
}
impl AccountOperationRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_OPERATION_RESTRICTION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			restriction_flags: AccountRestrictionFlags::default(),
			account_restriction_transaction_body_reserved_1: 0,
			restriction_additions: Vec::new(),
			restriction_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.restriction_flags.size();
		size += 1;
		size += 1;
		size += 4;
		size += self.restriction_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.restriction_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let restriction_flags;
		(restriction_flags, payload) = AccountRestrictionFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let account_restriction_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if account_restriction_transaction_body_reserved_1 != 0 { return None; }
		let mut restriction_additions = Vec::new();
		for _ in 0..restriction_additions_count {
			let element;
			(element, payload) = TransactionType::deserialize(payload).unwrap();
			restriction_additions.push(element);
		}
		let mut restriction_deletions = Vec::new();
		for _ in 0..restriction_deletions_count {
			let element;
			(element, payload) = TransactionType::deserialize(payload).unwrap();
			restriction_deletions.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			restriction_flags: restriction_flags,
			account_restriction_transaction_body_reserved_1: account_restriction_transaction_body_reserved_1,
			restriction_additions: restriction_additions,
			restriction_deletions: restriction_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.restriction_flags.serialize());
		serialized.append(&mut self.restriction_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.account_restriction_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.restriction_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedAccountOperationRestrictionTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub restriction_flags: AccountRestrictionFlags,
	pub account_restriction_transaction_body_reserved_1: u32,
	pub restriction_additions: Vec<TransactionType>,
	pub restriction_deletions: Vec<TransactionType>,
}
impl EmbeddedAccountOperationRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_OPERATION_RESTRICTION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			restriction_flags: AccountRestrictionFlags::default(),
			account_restriction_transaction_body_reserved_1: 0,
			restriction_additions: Vec::new(),
			restriction_deletions: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.restriction_flags.size();
		size += 1;
		size += 1;
		size += 4;
		size += self.restriction_additions.iter().map(|x| x.size()).sum::<usize>();
		size += self.restriction_deletions.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let restriction_flags;
		(restriction_flags, payload) = AccountRestrictionFlags::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_additions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let restriction_deletions_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let account_restriction_transaction_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if account_restriction_transaction_body_reserved_1 != 0 { return None; }
		let mut restriction_additions = Vec::new();
		for _ in 0..restriction_additions_count {
			let element;
			(element, payload) = TransactionType::deserialize(payload).unwrap();
			restriction_additions.push(element);
		}
		let mut restriction_deletions = Vec::new();
		for _ in 0..restriction_deletions_count {
			let element;
			(element, payload) = TransactionType::deserialize(payload).unwrap();
			restriction_deletions.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			restriction_flags: restriction_flags,
			account_restriction_transaction_body_reserved_1: account_restriction_transaction_body_reserved_1,
			restriction_additions: restriction_additions,
			restriction_deletions: restriction_deletions,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.restriction_flags.serialize());
		serialized.append(&mut self.restriction_additions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_deletions.len().to_le_bytes().to_vec());
		serialized.append(&mut self.account_restriction_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.restriction_additions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.restriction_deletions.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicAddressRestrictionTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic_id: UnresolvedMosaicId,
	pub restriction_key: u64,
	pub previous_restriction_value: u64,
	pub new_restriction_value: u64,
	pub target_address: UnresolvedAddress,
}
impl MosaicAddressRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_ADDRESS_RESTRICTION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic_id: UnresolvedMosaicId::default(),
			restriction_key: 0,
			previous_restriction_value: 0,
			new_restriction_value: 0,
			target_address: UnresolvedAddress::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.mosaic_id.size();
		size += 8;
		size += 8;
		size += 8;
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let restriction_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let previous_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let new_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			mosaic_id: mosaic_id,
			restriction_key: restriction_key,
			previous_restriction_value: previous_restriction_value,
			new_restriction_value: new_restriction_value,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.restriction_key.to_le_bytes().to_vec());
		serialized.append(&mut self.previous_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.new_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMosaicAddressRestrictionTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub mosaic_id: UnresolvedMosaicId,
	pub restriction_key: u64,
	pub previous_restriction_value: u64,
	pub new_restriction_value: u64,
	pub target_address: UnresolvedAddress,
}
impl EmbeddedMosaicAddressRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_ADDRESS_RESTRICTION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			mosaic_id: UnresolvedMosaicId::default(),
			restriction_key: 0,
			previous_restriction_value: 0,
			new_restriction_value: 0,
			target_address: UnresolvedAddress::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.mosaic_id.size();
		size += 8;
		size += 8;
		size += 8;
		size += self.target_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let restriction_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let previous_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let new_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let target_address;
		(target_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			mosaic_id: mosaic_id,
			restriction_key: restriction_key,
			previous_restriction_value: previous_restriction_value,
			new_restriction_value: new_restriction_value,
			target_address: target_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.restriction_key.to_le_bytes().to_vec());
		serialized.append(&mut self.previous_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.new_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.target_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MosaicRestrictionKey {
	value: u64
}
impl MosaicRestrictionKey {
	const SIZE: usize = 8;
	pub fn new(mosaicrestrictionkey: u64) -> Self {
		Self {
			value: mosaicrestrictionkey
		}
	}
	pub fn default() -> Self {
		Self::new(0)
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 8 { return None; }
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let value = u64::from_le_bytes(bytes);
		Some((Self::new(value), &payload[..8]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.value.to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("0x{:016x}", self.value)
	}
}

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MosaicRestrictionType {
	NONE = 0,
	EQ = 1,
	NE = 2,
	LT = 3,
	LE = 4,
	GT = 5,
	GE = 6,
}
impl MosaicRestrictionType {
	const SIZE: usize = 1;
	pub fn default() -> Self {
		Self::NONE
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 1 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		match u8::from_le_bytes(bytes) {
			0 => Some((MosaicRestrictionType::NONE, &payload[1..])),
			1 => Some((MosaicRestrictionType::EQ, &payload[1..])),
			2 => Some((MosaicRestrictionType::NE, &payload[1..])),
			3 => Some((MosaicRestrictionType::LT, &payload[1..])),
			4 => Some((MosaicRestrictionType::LE, &payload[1..])),
			5 => Some((MosaicRestrictionType::GT, &payload[1..])),
			6 => Some((MosaicRestrictionType::GE, &payload[1..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u8).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("MosaicRestrictionType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct MosaicGlobalRestrictionTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic_id: UnresolvedMosaicId,
	pub reference_mosaic_id: UnresolvedMosaicId,
	pub restriction_key: u64,
	pub previous_restriction_value: u64,
	pub new_restriction_value: u64,
	pub previous_restriction_type: MosaicRestrictionType,
	pub new_restriction_type: MosaicRestrictionType,
}
impl MosaicGlobalRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_GLOBAL_RESTRICTION;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic_id: UnresolvedMosaicId::default(),
			reference_mosaic_id: UnresolvedMosaicId::default(),
			restriction_key: 0,
			previous_restriction_value: 0,
			new_restriction_value: 0,
			previous_restriction_type: MosaicRestrictionType::default(),
			new_restriction_type: MosaicRestrictionType::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.mosaic_id.size();
		size += self.reference_mosaic_id.size();
		size += 8;
		size += 8;
		size += 8;
		size += self.previous_restriction_type.size();
		size += self.new_restriction_type.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let reference_mosaic_id;
		(reference_mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let restriction_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let previous_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let new_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let previous_restriction_type;
		(previous_restriction_type, payload) = MosaicRestrictionType::deserialize(payload).unwrap();
		let new_restriction_type;
		(new_restriction_type, payload) = MosaicRestrictionType::deserialize(payload).unwrap();
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			mosaic_id: mosaic_id,
			reference_mosaic_id: reference_mosaic_id,
			restriction_key: restriction_key,
			previous_restriction_value: previous_restriction_value,
			new_restriction_value: new_restriction_value,
			previous_restriction_type: previous_restriction_type,
			new_restriction_type: new_restriction_type,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.reference_mosaic_id.serialize());
		serialized.append(&mut self.restriction_key.to_le_bytes().to_vec());
		serialized.append(&mut self.previous_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.new_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.previous_restriction_type.serialize());
		serialized.append(&mut self.new_restriction_type.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedMosaicGlobalRestrictionTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub mosaic_id: UnresolvedMosaicId,
	pub reference_mosaic_id: UnresolvedMosaicId,
	pub restriction_key: u64,
	pub previous_restriction_value: u64,
	pub new_restriction_value: u64,
	pub previous_restriction_type: MosaicRestrictionType,
	pub new_restriction_type: MosaicRestrictionType,
}
impl EmbeddedMosaicGlobalRestrictionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_GLOBAL_RESTRICTION;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			mosaic_id: UnresolvedMosaicId::default(),
			reference_mosaic_id: UnresolvedMosaicId::default(),
			restriction_key: 0,
			previous_restriction_value: 0,
			new_restriction_value: 0,
			previous_restriction_type: MosaicRestrictionType::default(),
			new_restriction_type: MosaicRestrictionType::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.mosaic_id.size();
		size += self.reference_mosaic_id.size();
		size += 8;
		size += 8;
		size += 8;
		size += self.previous_restriction_type.size();
		size += self.new_restriction_type.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mosaic_id;
		(mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let reference_mosaic_id;
		(reference_mosaic_id, payload) = UnresolvedMosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let restriction_key = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let previous_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(payload);
		let new_restriction_value = u64::from_le_bytes(bytes);
		payload = &payload[8 as usize..];
		let previous_restriction_type;
		(previous_restriction_type, payload) = MosaicRestrictionType::deserialize(payload).unwrap();
		let new_restriction_type;
		(new_restriction_type, payload) = MosaicRestrictionType::deserialize(payload).unwrap();
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			mosaic_id: mosaic_id,
			reference_mosaic_id: reference_mosaic_id,
			restriction_key: restriction_key,
			previous_restriction_value: previous_restriction_value,
			new_restriction_value: new_restriction_value,
			previous_restriction_type: previous_restriction_type,
			new_restriction_type: new_restriction_type,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.reference_mosaic_id.serialize());
		serialized.append(&mut self.restriction_key.to_le_bytes().to_vec());
		serialized.append(&mut self.previous_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.new_restriction_value.to_le_bytes().to_vec());
		serialized.append(&mut self.previous_restriction_type.serialize());
		serialized.append(&mut self.new_restriction_type.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct TransferTransactionV1 {
	pub verifiable_entity_header_reserved_1: u32,
	pub signature: Signature,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub recipient_address: UnresolvedAddress,
	pub transfer_transaction_body_reserved_1: u8,
	pub transfer_transaction_body_reserved_2: u32,
	pub mosaics: Vec<UnresolvedMosaic>,
	pub message: Vec<u8>,
}
impl TransferTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::TRANSFER;
	pub fn new() -> Self {
		Self {
			verifiable_entity_header_reserved_1: 0,
			signature: Signature::default(),
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			fee: Amount::default(),
			deadline: Timestamp::default(),
			recipient_address: UnresolvedAddress::default(),
			transfer_transaction_body_reserved_1: 0,
			transfer_transaction_body_reserved_2: 0,
			mosaics: Vec::new(),
			message: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signature.size();
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.recipient_address.size();
		size += 2;
		size += 1;
		size += 1;
		size += 4;
		size += self.mosaics.iter().map(|x| x.size()).sum::<usize>();
		size += 1 * self.message.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let verifiable_entity_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if verifiable_entity_header_reserved_1 != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let message_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let mosaics_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let transfer_transaction_body_reserved_1 = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		if transfer_transaction_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let transfer_transaction_body_reserved_2 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if transfer_transaction_body_reserved_2 != 0 { return None; }
		let mut mosaics = Vec::new();
		for _ in 0..mosaics_count {
			let element;
			(element, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
			mosaics.push(element);
		}
		let mut message = Vec::new();
		for _ in 0..message_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[message_size as usize..];
			message.push(element);
		}
		let self_ = Self {
			verifiable_entity_header_reserved_1: verifiable_entity_header_reserved_1,
			signature: signature,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			fee: fee,
			deadline: deadline,
			recipient_address: recipient_address,
			transfer_transaction_body_reserved_1: transfer_transaction_body_reserved_1,
			transfer_transaction_body_reserved_2: transfer_transaction_body_reserved_2,
			mosaics: mosaics,
			message: message,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.verifiable_entity_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.message.len().to_le_bytes().to_vec());
		serialized.append(&mut self.mosaics.len().to_le_bytes().to_vec());
		serialized.append(&mut self.transfer_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.transfer_transaction_body_reserved_2.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaics.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.message.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug, PartialEq)]
pub struct EmbeddedTransferTransactionV1 {
	pub embedded_transaction_header_reserved_1: u32,
	pub signer_public_key: PublicKey,
	pub entity_body_reserved_1: u32,
	pub version: u8,
	pub network: NetworkType,
	pub type_: TransactionType,
	pub recipient_address: UnresolvedAddress,
	pub transfer_transaction_body_reserved_1: u8,
	pub transfer_transaction_body_reserved_2: u32,
	pub mosaics: Vec<UnresolvedMosaic>,
	pub message: Vec<u8>,
}
impl EmbeddedTransferTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::TRANSFER;
	pub fn new() -> Self {
		Self {
			embedded_transaction_header_reserved_1: 0,
			signer_public_key: PublicKey::default(),
			entity_body_reserved_1: 0,
			version: Self::TRANSACTION_VERSION,
			network: NetworkType::default(),
			type_: Self::TRANSACTION_TYPE,
			recipient_address: UnresolvedAddress::default(),
			transfer_transaction_body_reserved_1: 0,
			transfer_transaction_body_reserved_2: 0,
			mosaics: Vec::new(),
			message: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += 1;
		size += self.network.size();
		size += self.type_.size();
		size += self.recipient_address.size();
		size += 2;
		size += 1;
		size += 1;
		size += 4;
		size += self.mosaics.iter().map(|x| x.size()).sum::<usize>();
		size += 1 * self.message.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if size as usize > payload.len() { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let embedded_transaction_header_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if embedded_transaction_header_reserved_1 != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let recipient_address;
		(recipient_address, payload) = UnresolvedAddress::deserialize(payload).unwrap();
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let message_size = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let mosaics_count = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let transfer_transaction_body_reserved_1 = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		if transfer_transaction_body_reserved_1 != 0 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let transfer_transaction_body_reserved_2 = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if transfer_transaction_body_reserved_2 != 0 { return None; }
		let mut mosaics = Vec::new();
		for _ in 0..mosaics_count {
			let element;
			(element, payload) = UnresolvedMosaic::deserialize(payload).unwrap();
			mosaics.push(element);
		}
		let mut message = Vec::new();
		for _ in 0..message_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[message_size as usize..];
			message.push(element);
		}
		let self_ = Self {
			embedded_transaction_header_reserved_1: embedded_transaction_header_reserved_1,
			signer_public_key: signer_public_key,
			entity_body_reserved_1: entity_body_reserved_1,
			version: version,
			network: network,
			type_: type_,
			recipient_address: recipient_address,
			transfer_transaction_body_reserved_1: transfer_transaction_body_reserved_1,
			transfer_transaction_body_reserved_2: transfer_transaction_body_reserved_2,
			mosaics: mosaics,
			message: message,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.size().to_le_bytes().to_vec());
		serialized.append(&mut self.embedded_transaction_header_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.message.len().to_le_bytes().to_vec());
		serialized.append(&mut self.mosaics.len().to_le_bytes().to_vec());
		serialized.append(&mut self.transfer_transaction_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.transfer_transaction_body_reserved_2.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaics.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.message.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


