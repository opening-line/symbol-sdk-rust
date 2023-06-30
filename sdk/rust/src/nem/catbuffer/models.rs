// rust
use hex;

/// ast_model.display_type == DisplayType.INTEGER
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Timestamp {
	value: u32
}
impl Timestamp {
	const SIZE: usize = 4;
	pub fn new(timestamp: u32) -> Self {
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

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone)]
pub struct Address {
	bytes: [u8; 40]
}
impl Address {
	const SIZE: usize = 40;
	pub fn new(address: [u8; 40]) -> Self {
		Self {
			bytes: address
		}
	}
	pub fn default() -> Self {
		Self::new([0; 40])
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 40 { return None; }
		let mut bytes = [0u8; 40];
		bytes.copy_from_slice(payload);
		Some((Self::new(bytes), &payload[..40]))
	}
	pub fn serialize(&self) -> Vec<u8> {
		self.bytes.to_vec()
	}
	pub fn to_string(&self) -> String {
		"0x".to_string() + &hex::encode(self.bytes)
	}
}

/// ast_model.display_type == DisplayType.BYTE_ARRAY
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TransactionType {
	TRANSFER = 257,
	ACCOUNT_KEY_LINK = 2049,
	MULTISIG_ACCOUNT_MODIFICATION = 4097,
	MULTISIG_COSIGNATURE = 4098,
	MULTISIG_TRANSACTION = 4100,
	NAMESPACE_REGISTRATION = 8193,
	MOSAIC_DEFINITION = 16385,
	MOSAIC_SUPPLY_CHANGE = 16386,
}
impl TransactionType {
	const SIZE: usize = 4;
	pub fn default() -> Self {
		Self::TRANSFER
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		match u32::from_le_bytes(bytes) {
			257 => Some((TransactionType::TRANSFER, &payload[4..])),
			2049 => Some((TransactionType::ACCOUNT_KEY_LINK, &payload[4..])),
			4097 => Some((TransactionType::MULTISIG_ACCOUNT_MODIFICATION, &payload[4..])),
			4098 => Some((TransactionType::MULTISIG_COSIGNATURE, &payload[4..])),
			4100 => Some((TransactionType::MULTISIG_TRANSACTION, &payload[4..])),
			8193 => Some((TransactionType::NAMESPACE_REGISTRATION, &payload[4..])),
			16385 => Some((TransactionType::MOSAIC_DEFINITION, &payload[4..])),
			16386 => Some((TransactionType::MOSAIC_SUPPLY_CHANGE, &payload[4..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u32).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("TransactionType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct Transaction {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
}
impl Transaction {
	pub fn new() -> Self {
		Self {
			type_: TransactionType::default(),
			version: 0,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableTransaction {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
}
impl NonVerifiableTransaction {
	pub fn new() -> Self {
		Self {
			type_: TransactionType::default(),
			version: 0,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum LinkAction {
	LINK = 1,
	UNLINK = 2,
}
impl LinkAction {
	const SIZE: usize = 4;
	pub fn default() -> Self {
		Self::LINK
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		match u32::from_le_bytes(bytes) {
			1 => Some((LinkAction::LINK, &payload[4..])),
			2 => Some((LinkAction::UNLINK, &payload[4..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u32).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("LinkAction::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct AccountKeyLinkTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub link_action: LinkAction,
	pub remote_public_key_size: u32,
	pub remote_public_key: PublicKey,
}
impl AccountKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_KEY_LINK;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			link_action: LinkAction::default(),
			remote_public_key_size: 32,
			remote_public_key: PublicKey::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.link_action.size();
		size += 4;
		size += self.remote_public_key.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let remote_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if remote_public_key_size != 0 { return None; }
		let remote_public_key;
		(remote_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			link_action: link_action,
			remote_public_key_size: remote_public_key_size,
			remote_public_key: remote_public_key,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized.append(&mut self.remote_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.remote_public_key.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableAccountKeyLinkTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub link_action: LinkAction,
	pub remote_public_key_size: u32,
	pub remote_public_key: PublicKey,
}
impl NonVerifiableAccountKeyLinkTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::ACCOUNT_KEY_LINK;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			link_action: LinkAction::default(),
			remote_public_key_size: 32,
			remote_public_key: PublicKey::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += self.link_action.size();
		size += 4;
		size += self.remote_public_key.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let link_action;
		(link_action, payload) = LinkAction::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let remote_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if remote_public_key_size != 0 { return None; }
		let remote_public_key;
		(remote_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			link_action: link_action,
			remote_public_key_size: remote_public_key_size,
			remote_public_key: remote_public_key,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.link_action.serialize());
		serialized.append(&mut self.remote_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.remote_public_key.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NamespaceId {
	pub name: Vec<i8>,
}
impl NamespaceId {
	pub fn new() -> Self {
		Self {
			name: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 1 * self.name.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let name_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut name = Vec::new();
		for _ in 0..name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[name_size as usize..];
			name.push(element);
		}
		let self_ = Self {
			name: name,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
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
#[derive(Debug)]
pub struct MosaicId {
	pub namespace_id: NamespaceId,
	pub name: Vec<i8>,
}
impl MosaicId {
	pub fn new() -> Self {
		Self {
			namespace_id: NamespaceId::default(),
			name: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.namespace_id.size();
		size += 4;
		size += 1 * self.name.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let namespace_id;
		(namespace_id, payload) = NamespaceId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let name_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut name = Vec::new();
		for _ in 0..name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[name_size as usize..];
			name.push(element);
		}
		let self_ = Self {
			namespace_id: namespace_id,
			name: name,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.namespace_id.serialize());
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
#[derive(Debug)]
pub struct Mosaic {
	pub mosaic_id_size: u32,
	pub mosaic_id: MosaicId,
	pub amount: Amount,
}
impl Mosaic {
	pub fn new() -> Self {
		Self {
			mosaic_id_size: mosaic_id,
			mosaic_id: MosaicId::default(),
			amount: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += self.mosaic_id.size();
		size += self.amount.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_id_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mosaic_id;
		(mosaic_id, payload) = MosaicId::deserialize(payload).unwrap();
		let amount;
		(amount, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			mosaic_id_size: mosaic_id_size,
			mosaic_id: mosaic_id,
			amount: amount,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.mosaic_id_size.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.amount.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct SizePrefixedMosaic {
	pub mosaic_size: u32,
	pub mosaic: Mosaic,
}
impl SizePrefixedMosaic {
	pub fn new() -> Self {
		Self {
			mosaic_size: mosaic,
			mosaic: Mosaic::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += self.mosaic.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mosaic;
		(mosaic, payload) = Mosaic::deserialize(payload).unwrap();
		let self_ = Self {
			mosaic_size: mosaic_size,
			mosaic: mosaic,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.mosaic_size.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum MosaicTransferFeeType {
	ABSOLUTE = 1,
	PERCENTILE = 2,
}
impl MosaicTransferFeeType {
	const SIZE: usize = 4;
	pub fn default() -> Self {
		Self::ABSOLUTE
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		match u32::from_le_bytes(bytes) {
			1 => Some((MosaicTransferFeeType::ABSOLUTE, &payload[4..])),
			2 => Some((MosaicTransferFeeType::PERCENTILE, &payload[4..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u32).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("MosaicTransferFeeType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MosaicLevy {
	pub transfer_fee_type: MosaicTransferFeeType,
	pub recipient_address_size: u32,
	pub recipient_address: Address,
	pub mosaic_id_size: u32,
	pub mosaic_id: MosaicId,
	pub fee: Amount,
}
impl MosaicLevy {
	pub fn new() -> Self {
		Self {
			transfer_fee_type: MosaicTransferFeeType::default(),
			recipient_address_size: 40,
			recipient_address: Address::default(),
			mosaic_id_size: mosaic_id,
			mosaic_id: MosaicId::default(),
			fee: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.transfer_fee_type.size();
		size += 4;
		size += self.recipient_address.size();
		size += 4;
		size += self.mosaic_id.size();
		size += self.fee.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let transfer_fee_type;
		(transfer_fee_type, payload) = MosaicTransferFeeType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let recipient_address_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if recipient_address_size != 0 { return None; }
		let recipient_address;
		(recipient_address, payload) = Address::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_id_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mosaic_id;
		(mosaic_id, payload) = MosaicId::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			transfer_fee_type: transfer_fee_type,
			recipient_address_size: recipient_address_size,
			recipient_address: recipient_address,
			mosaic_id_size: mosaic_id_size,
			mosaic_id: mosaic_id,
			fee: fee,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.transfer_fee_type.serialize());
		serialized.append(&mut self.recipient_address_size.to_le_bytes().to_vec());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.mosaic_id_size.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MosaicProperty {
	pub name: Vec<i8>,
	pub value: Vec<i8>,
}
impl MosaicProperty {
	pub fn new() -> Self {
		Self {
			name: Vec::new(),
			value: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += 1 * self.name.len();
		size += 4;
		size += 1 * self.value.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let name_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut name = Vec::new();
		for _ in 0..name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[name_size as usize..];
			name.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let value_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut value = Vec::new();
		for _ in 0..value_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[value_size as usize..];
			value.push(element);
		}
		let self_ = Self {
			name: name,
			value: value,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.name.len().to_le_bytes().to_vec());
		serialized.append(&mut self.name.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
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
#[derive(Debug)]
pub struct SizePrefixedMosaicProperty {
	pub property_size: u32,
	pub property: MosaicProperty,
}
impl SizePrefixedMosaicProperty {
	pub fn new() -> Self {
		Self {
			property_size: property,
			property: MosaicProperty::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += self.property.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let property_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let property;
		(property, payload) = MosaicProperty::deserialize(payload).unwrap();
		let self_ = Self {
			property_size: property_size,
			property: property,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.property_size.to_le_bytes().to_vec());
		serialized.append(&mut self.property.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MosaicDefinition {
	pub owner_public_key_size: u32,
	pub owner_public_key: PublicKey,
	pub id_size: u32,
	pub id: MosaicId,
	pub description: Vec<i8>,
	pub properties: Vec<SizePrefixedMosaicProperty>,
	pub levy_size: u32,
	pub levy: MosaicLevy,
}
impl MosaicDefinition {
	pub fn new() -> Self {
		Self {
			owner_public_key_size: 32,
			owner_public_key: PublicKey::default(),
			id_size: id,
			id: MosaicId::default(),
			description: Vec::new(),
			properties: Vec::new(),
			levy_size: 0,
			levy: MosaicLevy::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += self.owner_public_key.size();
		size += 4;
		size += self.id.size();
		size += 4;
		size += 1 * self.description.len();
		size += 4;
		size += self.properties.iter().map(|x| x.size()).sum::<usize>();
		size += 4;
		size += self.levy.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let owner_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if owner_public_key_size != 0 { return None; }
		let owner_public_key;
		(owner_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let id_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let id;
		(id, payload) = MosaicId::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let description_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut description = Vec::new();
		for _ in 0..description_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[description_size as usize..];
			description.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let properties_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut properties = Vec::new();
		for _ in 0..properties_count {
			let element;
			(element, payload) = SizePrefixedMosaicProperty::deserialize(payload).unwrap();
			properties.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let levy_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let levy;
		(levy, payload) = MosaicLevy::deserialize(payload).unwrap();
		let self_ = Self {
			owner_public_key_size: owner_public_key_size,
			owner_public_key: owner_public_key,
			id_size: id_size,
			id: id,
			description: description,
			properties: properties,
			levy_size: levy_size,
			levy: levy,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.owner_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.owner_public_key.serialize());
		serialized.append(&mut self.id_size.to_le_bytes().to_vec());
		serialized.append(&mut self.id.serialize());
		serialized.append(&mut self.description.len().to_le_bytes().to_vec());
		serialized.append(&mut self.description.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized.append(&mut self.properties.len().to_le_bytes().to_vec());
		serialized.append(&mut self.properties.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.levy_size.to_le_bytes().to_vec());
		serialized.append(&mut self.levy.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MosaicDefinitionTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic_definition_size: u32,
	pub mosaic_definition: MosaicDefinition,
	pub rental_fee_sink_size: u32,
	pub rental_fee_sink: Address,
	pub rental_fee: Amount,
}
impl MosaicDefinitionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_DEFINITION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic_definition_size: mosaic_definition,
			mosaic_definition: MosaicDefinition::default(),
			rental_fee_sink_size: 40,
			rental_fee_sink: Address::default(),
			rental_fee: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.mosaic_definition.size();
		size += 4;
		size += self.rental_fee_sink.size();
		size += self.rental_fee.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_definition_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mosaic_definition;
		(mosaic_definition, payload) = MosaicDefinition::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let rental_fee_sink_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if rental_fee_sink_size != 0 { return None; }
		let rental_fee_sink;
		(rental_fee_sink, payload) = Address::deserialize(payload).unwrap();
		let rental_fee;
		(rental_fee, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			mosaic_definition_size: mosaic_definition_size,
			mosaic_definition: mosaic_definition,
			rental_fee_sink_size: rental_fee_sink_size,
			rental_fee_sink: rental_fee_sink,
			rental_fee: rental_fee,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic_definition_size.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic_definition.serialize());
		serialized.append(&mut self.rental_fee_sink_size.to_le_bytes().to_vec());
		serialized.append(&mut self.rental_fee_sink.serialize());
		serialized.append(&mut self.rental_fee.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableMosaicDefinitionTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic_definition_size: u32,
	pub mosaic_definition: MosaicDefinition,
	pub rental_fee_sink_size: u32,
	pub rental_fee_sink: Address,
	pub rental_fee: Amount,
}
impl NonVerifiableMosaicDefinitionTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_DEFINITION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic_definition_size: mosaic_definition,
			mosaic_definition: MosaicDefinition::default(),
			rental_fee_sink_size: 40,
			rental_fee_sink: Address::default(),
			rental_fee: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.mosaic_definition.size();
		size += 4;
		size += self.rental_fee_sink.size();
		size += self.rental_fee.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_definition_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mosaic_definition;
		(mosaic_definition, payload) = MosaicDefinition::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let rental_fee_sink_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if rental_fee_sink_size != 0 { return None; }
		let rental_fee_sink;
		(rental_fee_sink, payload) = Address::deserialize(payload).unwrap();
		let rental_fee;
		(rental_fee, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			mosaic_definition_size: mosaic_definition_size,
			mosaic_definition: mosaic_definition,
			rental_fee_sink_size: rental_fee_sink_size,
			rental_fee_sink: rental_fee_sink,
			rental_fee: rental_fee,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic_definition_size.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic_definition.serialize());
		serialized.append(&mut self.rental_fee_sink_size.to_le_bytes().to_vec());
		serialized.append(&mut self.rental_fee_sink.serialize());
		serialized.append(&mut self.rental_fee.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum MosaicSupplyChangeAction {
	INCREASE = 1,
	DECREASE = 2,
}
impl MosaicSupplyChangeAction {
	const SIZE: usize = 4;
	pub fn default() -> Self {
		Self::INCREASE
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		match u32::from_le_bytes(bytes) {
			1 => Some((MosaicSupplyChangeAction::INCREASE, &payload[4..])),
			2 => Some((MosaicSupplyChangeAction::DECREASE, &payload[4..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u32).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("MosaicSupplyChangeAction::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MosaicSupplyChangeTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic_id_size: u32,
	pub mosaic_id: MosaicId,
	pub action: MosaicSupplyChangeAction,
	pub delta: Amount,
}
impl MosaicSupplyChangeTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_SUPPLY_CHANGE;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic_id_size: mosaic_id,
			mosaic_id: MosaicId::default(),
			action: MosaicSupplyChangeAction::default(),
			delta: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.mosaic_id.size();
		size += self.action.size();
		size += self.delta.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_id_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mosaic_id;
		(mosaic_id, payload) = MosaicId::deserialize(payload).unwrap();
		let action;
		(action, payload) = MosaicSupplyChangeAction::deserialize(payload).unwrap();
		let delta;
		(delta, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			mosaic_id_size: mosaic_id_size,
			mosaic_id: mosaic_id,
			action: action,
			delta: delta,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic_id_size.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.action.serialize());
		serialized.append(&mut self.delta.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableMosaicSupplyChangeTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub mosaic_id_size: u32,
	pub mosaic_id: MosaicId,
	pub action: MosaicSupplyChangeAction,
	pub delta: Amount,
}
impl NonVerifiableMosaicSupplyChangeTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MOSAIC_SUPPLY_CHANGE;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			mosaic_id_size: mosaic_id,
			mosaic_id: MosaicId::default(),
			action: MosaicSupplyChangeAction::default(),
			delta: Amount::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.mosaic_id.size();
		size += self.action.size();
		size += self.delta.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaic_id_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mosaic_id;
		(mosaic_id, payload) = MosaicId::deserialize(payload).unwrap();
		let action;
		(action, payload) = MosaicSupplyChangeAction::deserialize(payload).unwrap();
		let delta;
		(delta, payload) = Amount::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			mosaic_id_size: mosaic_id_size,
			mosaic_id: mosaic_id,
			action: action,
			delta: delta,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.mosaic_id_size.to_le_bytes().to_vec());
		serialized.append(&mut self.mosaic_id.serialize());
		serialized.append(&mut self.action.serialize());
		serialized.append(&mut self.delta.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum MultisigAccountModificationType {
	ADD_COSIGNATORY = 1,
	DELETE_COSIGNATORY = 2,
}
impl MultisigAccountModificationType {
	const SIZE: usize = 4;
	pub fn default() -> Self {
		Self::ADD_COSIGNATORY
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		match u32::from_le_bytes(bytes) {
			1 => Some((MultisigAccountModificationType::ADD_COSIGNATORY, &payload[4..])),
			2 => Some((MultisigAccountModificationType::DELETE_COSIGNATORY, &payload[4..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u32).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("MultisigAccountModificationType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MultisigAccountModification {
	pub modification_type: MultisigAccountModificationType,
	pub cosignatory_public_key_size: u32,
	pub cosignatory_public_key: PublicKey,
}
impl MultisigAccountModification {
	pub fn new() -> Self {
		Self {
			modification_type: MultisigAccountModificationType::default(),
			cosignatory_public_key_size: 32,
			cosignatory_public_key: PublicKey::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.modification_type.size();
		size += 4;
		size += self.cosignatory_public_key.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let modification_type;
		(modification_type, payload) = MultisigAccountModificationType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let cosignatory_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if cosignatory_public_key_size != 0 { return None; }
		let cosignatory_public_key;
		(cosignatory_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let self_ = Self {
			modification_type: modification_type,
			cosignatory_public_key_size: cosignatory_public_key_size,
			cosignatory_public_key: cosignatory_public_key,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.modification_type.serialize());
		serialized.append(&mut self.cosignatory_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.cosignatory_public_key.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct SizePrefixedMultisigAccountModification {
	pub modification_size: u32,
	pub modification: MultisigAccountModification,
}
impl SizePrefixedMultisigAccountModification {
	pub fn new() -> Self {
		Self {
			modification_size: modification,
			modification: MultisigAccountModification::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += self.modification.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let modification_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let modification;
		(modification, payload) = MultisigAccountModification::deserialize(payload).unwrap();
		let self_ = Self {
			modification_size: modification_size,
			modification: modification,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.modification_size.to_le_bytes().to_vec());
		serialized.append(&mut self.modification.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MultisigAccountModificationTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub modifications: Vec<SizePrefixedMultisigAccountModification>,
}
impl MultisigAccountModificationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_ACCOUNT_MODIFICATION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			modifications: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.modifications.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let modifications_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut modifications = Vec::new();
		for _ in 0..modifications_count {
			let element;
			(element, payload) = SizePrefixedMultisigAccountModification::deserialize(payload).unwrap();
			modifications.push(element);
		}
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			modifications: modifications,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.modifications.len().to_le_bytes().to_vec());
		serialized.append(&mut self.modifications.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableMultisigAccountModificationTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub modifications: Vec<SizePrefixedMultisigAccountModification>,
}
impl NonVerifiableMultisigAccountModificationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_ACCOUNT_MODIFICATION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			modifications: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.modifications.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let modifications_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut modifications = Vec::new();
		for _ in 0..modifications_count {
			let element;
			(element, payload) = SizePrefixedMultisigAccountModification::deserialize(payload).unwrap();
			modifications.push(element);
		}
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			modifications: modifications,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.modifications.len().to_le_bytes().to_vec());
		serialized.append(&mut self.modifications.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MultisigAccountModificationTransactionV2 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub modifications: Vec<SizePrefixedMultisigAccountModification>,
	pub min_approval_delta_size: u32,
	pub min_approval_delta: i32,
}
impl MultisigAccountModificationTransactionV2 {
	const TRANSACTION_VERSION: u8 = 2;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_ACCOUNT_MODIFICATION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			modifications: Vec::new(),
			min_approval_delta_size: 4,
			min_approval_delta: 0,
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.modifications.iter().map(|x| x.size()).sum::<usize>();
		size += 4;
		size += 4;
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let modifications_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut modifications = Vec::new();
		for _ in 0..modifications_count {
			let element;
			(element, payload) = SizePrefixedMultisigAccountModification::deserialize(payload).unwrap();
			modifications.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let min_approval_delta_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if min_approval_delta_size != 0 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let min_approval_delta = i32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			modifications: modifications,
			min_approval_delta_size: min_approval_delta_size,
			min_approval_delta: min_approval_delta,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.modifications.len().to_le_bytes().to_vec());
		serialized.append(&mut self.modifications.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.min_approval_delta_size.to_le_bytes().to_vec());
		serialized.append(&mut self.min_approval_delta.to_le_bytes().to_vec());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableMultisigAccountModificationTransactionV2 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub modifications: Vec<SizePrefixedMultisigAccountModification>,
	pub min_approval_delta_size: u32,
	pub min_approval_delta: i32,
}
impl NonVerifiableMultisigAccountModificationTransactionV2 {
	const TRANSACTION_VERSION: u8 = 2;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_ACCOUNT_MODIFICATION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			modifications: Vec::new(),
			min_approval_delta_size: 4,
			min_approval_delta: 0,
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.modifications.iter().map(|x| x.size()).sum::<usize>();
		size += 4;
		size += 4;
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let modifications_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut modifications = Vec::new();
		for _ in 0..modifications_count {
			let element;
			(element, payload) = SizePrefixedMultisigAccountModification::deserialize(payload).unwrap();
			modifications.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let min_approval_delta_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if min_approval_delta_size != 0 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let min_approval_delta = i32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			modifications: modifications,
			min_approval_delta_size: min_approval_delta_size,
			min_approval_delta: min_approval_delta,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.modifications.len().to_le_bytes().to_vec());
		serialized.append(&mut self.modifications.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized.append(&mut self.min_approval_delta_size.to_le_bytes().to_vec());
		serialized.append(&mut self.min_approval_delta.to_le_bytes().to_vec());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct CosignatureV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub multisig_transaction_hash_outer_size: u32,
	pub multisig_transaction_hash_size: u32,
	pub multisig_transaction_hash: Hash256,
	pub multisig_account_address_size: u32,
	pub multisig_account_address: Address,
}
impl CosignatureV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_COSIGNATURE;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			multisig_transaction_hash_outer_size: 36,
			multisig_transaction_hash_size: 32,
			multisig_transaction_hash: Hash256::default(),
			multisig_account_address_size: 40,
			multisig_account_address: Address::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += 4;
		size += self.multisig_transaction_hash.size();
		size += 4;
		size += self.multisig_account_address.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let multisig_transaction_hash_outer_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if multisig_transaction_hash_outer_size != 0 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let multisig_transaction_hash_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if multisig_transaction_hash_size != 0 { return None; }
		let multisig_transaction_hash;
		(multisig_transaction_hash, payload) = Hash256::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let multisig_account_address_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if multisig_account_address_size != 0 { return None; }
		let multisig_account_address;
		(multisig_account_address, payload) = Address::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			multisig_transaction_hash_outer_size: multisig_transaction_hash_outer_size,
			multisig_transaction_hash_size: multisig_transaction_hash_size,
			multisig_transaction_hash: multisig_transaction_hash,
			multisig_account_address_size: multisig_account_address_size,
			multisig_account_address: multisig_account_address,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.multisig_transaction_hash_outer_size.to_le_bytes().to_vec());
		serialized.append(&mut self.multisig_transaction_hash_size.to_le_bytes().to_vec());
		serialized.append(&mut self.multisig_transaction_hash.serialize());
		serialized.append(&mut self.multisig_account_address_size.to_le_bytes().to_vec());
		serialized.append(&mut self.multisig_account_address.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct SizePrefixedCosignatureV1 {
	pub cosignature_size: u32,
	pub cosignature: CosignatureV1,
}
impl SizePrefixedCosignatureV1 {
	pub fn new() -> Self {
		Self {
			cosignature_size: cosignature,
			cosignature: CosignatureV1::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += 4;
		size += self.cosignature.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let cosignature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let cosignature;
		(cosignature, payload) = CosignatureV1::deserialize(payload).unwrap();
		let self_ = Self {
			cosignature_size: cosignature_size,
			cosignature: cosignature,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.cosignature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.cosignature.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct MultisigTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub inner_transaction_size: u32,
	pub inner_transaction: NonVerifiableTransaction,
	pub cosignatures: Vec<SizePrefixedCosignatureV1>,
}
impl MultisigTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_TRANSACTION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			inner_transaction_size: inner_transaction,
			inner_transaction: NonVerifiableTransaction::default(),
			cosignatures: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.inner_transaction.size();
		size += 4;
		size += self.cosignatures.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let inner_transaction_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let inner_transaction;
		(inner_transaction, payload) = NonVerifiableTransaction::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let cosignatures_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut cosignatures = Vec::new();
		for _ in 0..cosignatures_count {
			let element;
			(element, payload) = SizePrefixedCosignatureV1::deserialize(payload).unwrap();
			cosignatures.push(element);
		}
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			inner_transaction_size: inner_transaction_size,
			inner_transaction: inner_transaction,
			cosignatures: cosignatures,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.inner_transaction_size.to_le_bytes().to_vec());
		serialized.append(&mut self.inner_transaction.serialize());
		serialized.append(&mut self.cosignatures.len().to_le_bytes().to_vec());
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
#[derive(Debug)]
pub struct NonVerifiableMultisigTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub inner_transaction_size: u32,
	pub inner_transaction: NonVerifiableTransaction,
}
impl NonVerifiableMultisigTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::MULTISIG_TRANSACTION;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			inner_transaction_size: inner_transaction,
			inner_transaction: NonVerifiableTransaction::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.inner_transaction.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let inner_transaction_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let inner_transaction;
		(inner_transaction, payload) = NonVerifiableTransaction::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			inner_transaction_size: inner_transaction_size,
			inner_transaction: inner_transaction,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.inner_transaction_size.to_le_bytes().to_vec());
		serialized.append(&mut self.inner_transaction.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NamespaceRegistrationTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub rental_fee_sink_size: u32,
	pub rental_fee_sink: Address,
	pub rental_fee: Amount,
	pub name: Vec<i8>,
	pub parent_name: Vec<i8>,
}
impl NamespaceRegistrationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::default();
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			rental_fee_sink_size: 40,
			rental_fee_sink: Address::default(),
			rental_fee: Amount::default(),
			name: Vec::new(),
			parent_name: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.rental_fee_sink.size();
		size += self.rental_fee.size();
		size += 4;
		size += 1 * self.name.len();
		size += 4;
		size += 1 * self.parent_name.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let rental_fee_sink_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if rental_fee_sink_size != 0 { return None; }
		let rental_fee_sink;
		(rental_fee_sink, payload) = Address::deserialize(payload).unwrap();
		let rental_fee;
		(rental_fee, payload) = Amount::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let name_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut name = Vec::new();
		for _ in 0..name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[name_size as usize..];
			name.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let parent_name_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut parent_name = Vec::new();
		for _ in 0..parent_name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[parent_name_size as usize..];
			parent_name.push(element);
		}
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			rental_fee_sink_size: rental_fee_sink_size,
			rental_fee_sink: rental_fee_sink,
			rental_fee: rental_fee,
			name: name,
			parent_name: parent_name,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.rental_fee_sink_size.to_le_bytes().to_vec());
		serialized.append(&mut self.rental_fee_sink.serialize());
		serialized.append(&mut self.rental_fee.serialize());
		serialized.append(&mut self.name.len().to_le_bytes().to_vec());
		serialized.append(&mut self.name.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized.append(&mut self.parent_name.len().to_le_bytes().to_vec());
		serialized.append(&mut self.parent_name.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableNamespaceRegistrationTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub rental_fee_sink_size: u32,
	pub rental_fee_sink: Address,
	pub rental_fee: Amount,
	pub name: Vec<i8>,
	pub parent_name: Vec<i8>,
}
impl NonVerifiableNamespaceRegistrationTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::default();
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			rental_fee_sink_size: 40,
			rental_fee_sink: Address::default(),
			rental_fee: Amount::default(),
			name: Vec::new(),
			parent_name: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.rental_fee_sink.size();
		size += self.rental_fee.size();
		size += 4;
		size += 1 * self.name.len();
		size += 4;
		size += 1 * self.parent_name.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let rental_fee_sink_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if rental_fee_sink_size != 0 { return None; }
		let rental_fee_sink;
		(rental_fee_sink, payload) = Address::deserialize(payload).unwrap();
		let rental_fee;
		(rental_fee, payload) = Amount::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let name_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut name = Vec::new();
		for _ in 0..name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[name_size as usize..];
			name.push(element);
		}
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let parent_name_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut parent_name = Vec::new();
		for _ in 0..parent_name_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = i8::from_le_bytes(bytes);
			payload = &payload[parent_name_size as usize..];
			parent_name.push(element);
		}
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			rental_fee_sink_size: rental_fee_sink_size,
			rental_fee_sink: rental_fee_sink,
			rental_fee: rental_fee,
			name: name,
			parent_name: parent_name,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.rental_fee_sink_size.to_le_bytes().to_vec());
		serialized.append(&mut self.rental_fee_sink.serialize());
		serialized.append(&mut self.rental_fee.serialize());
		serialized.append(&mut self.name.len().to_le_bytes().to_vec());
		serialized.append(&mut self.name.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized.append(&mut self.parent_name.len().to_le_bytes().to_vec());
		serialized.append(&mut self.parent_name.iter().fold(
			Vec::new(), |mut a, b| {
		a.append(&mut b.to_le_bytes().to_vec());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.ENUM
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum MessageType {
	PLAIN = 1,
	ENCRYPTED = 2,
}
impl MessageType {
	const SIZE: usize = 4;
	pub fn default() -> Self {
		Self::PLAIN
	}
	pub fn size(&self) -> usize {
		Self::SIZE
	}
	pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
		if payload.len() < 4 { return None; }
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		match u32::from_le_bytes(bytes) {
			1 => Some((MessageType::PLAIN, &payload[4..])),
			2 => Some((MessageType::ENCRYPTED, &payload[4..])),
			_ => None,
		}
	}
	pub fn serialize(&self) -> Vec<u8> {
		(self.clone() as u32).to_le_bytes().to_vec()
	}
	pub fn to_string(&self) -> String {
		format!("MessageType::{:?}", self)
	}
}

/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct Message {
	pub message_type: MessageType,
	pub message: Vec<u8>,
}
impl Message {
	pub fn new() -> Self {
		Self {
			message_type: MessageType::default(),
			message: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.message_type.size();
		size += 4;
		size += 1 * self.message.len();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let message_type;
		(message_type, payload) = MessageType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let message_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut message = Vec::new();
		for _ in 0..message_size {
			let mut bytes = [0u8; 1];
			bytes.copy_from_slice(payload);
			let element = u8::from_le_bytes(bytes);
			payload = &payload[message_size as usize..];
			message.push(element);
		}
		let self_ = Self {
			message_type: message_type,
			message: message,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.message_type.serialize());
		serialized.append(&mut self.message.len().to_le_bytes().to_vec());
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
#[derive(Debug)]
pub struct TransferTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub recipient_address_size: u32,
	pub recipient_address: Address,
	pub amount: Amount,
	pub message_envelope_size: u32,
	pub message: Message,
}
impl TransferTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::default();
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			recipient_address_size: 40,
			recipient_address: Address::default(),
			amount: Amount::default(),
			message_envelope_size: 0,
			message: Message::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.recipient_address.size();
		size += self.amount.size();
		size += 4;
		size += self.message.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let recipient_address_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if recipient_address_size != 0 { return None; }
		let recipient_address;
		(recipient_address, payload) = Address::deserialize(payload).unwrap();
		let amount;
		(amount, payload) = Amount::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let message_envelope_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let message;
		(message, payload) = Message::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			recipient_address_size: recipient_address_size,
			recipient_address: recipient_address,
			amount: amount,
			message_envelope_size: message_envelope_size,
			message: message,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.recipient_address_size.to_le_bytes().to_vec());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.amount.serialize());
		serialized.append(&mut self.message_envelope_size.to_le_bytes().to_vec());
		serialized.append(&mut self.message.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableTransferTransactionV1 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub recipient_address_size: u32,
	pub recipient_address: Address,
	pub amount: Amount,
	pub message_envelope_size: u32,
	pub message: Message,
}
impl NonVerifiableTransferTransactionV1 {
	const TRANSACTION_VERSION: u8 = 1;
	const TRANSACTION_TYPE: TransactionType = TransactionType::default();
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			recipient_address_size: 40,
			recipient_address: Address::default(),
			amount: Amount::default(),
			message_envelope_size: 0,
			message: Message::default(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.recipient_address.size();
		size += self.amount.size();
		size += 4;
		size += self.message.size();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let recipient_address_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if recipient_address_size != 0 { return None; }
		let recipient_address;
		(recipient_address, payload) = Address::deserialize(payload).unwrap();
		let amount;
		(amount, payload) = Amount::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let message_envelope_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let message;
		(message, payload) = Message::deserialize(payload).unwrap();
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			recipient_address_size: recipient_address_size,
			recipient_address: recipient_address,
			amount: amount,
			message_envelope_size: message_envelope_size,
			message: message,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.recipient_address_size.to_le_bytes().to_vec());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.amount.serialize());
		serialized.append(&mut self.message_envelope_size.to_le_bytes().to_vec());
		serialized.append(&mut self.message.serialize());
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct TransferTransactionV2 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub signature_size: u32,
	pub signature: Signature,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub recipient_address_size: u32,
	pub recipient_address: Address,
	pub amount: Amount,
	pub message_envelope_size: u32,
	pub message: Message,
	pub mosaics: Vec<SizePrefixedMosaic>,
}
impl TransferTransactionV2 {
	const TRANSACTION_VERSION: u8 = 2;
	const TRANSACTION_TYPE: TransactionType = TransactionType::TRANSFER;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			signature_size: 64,
			signature: Signature::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			recipient_address_size: 40,
			recipient_address: Address::default(),
			amount: Amount::default(),
			message_envelope_size: 0,
			message: Message::default(),
			mosaics: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += 4;
		size += self.signature.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.recipient_address.size();
		size += self.amount.size();
		size += 4;
		size += self.message.size();
		size += 4;
		size += self.mosaics.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signature_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signature_size != 0 { return None; }
		let signature;
		(signature, payload) = Signature::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let recipient_address_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if recipient_address_size != 0 { return None; }
		let recipient_address;
		(recipient_address, payload) = Address::deserialize(payload).unwrap();
		let amount;
		(amount, payload) = Amount::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let message_envelope_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let message;
		(message, payload) = Message::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaics_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut mosaics = Vec::new();
		for _ in 0..mosaics_count {
			let element;
			(element, payload) = SizePrefixedMosaic::deserialize(payload).unwrap();
			mosaics.push(element);
		}
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			signature_size: signature_size,
			signature: signature,
			fee: fee,
			deadline: deadline,
			recipient_address_size: recipient_address_size,
			recipient_address: recipient_address,
			amount: amount,
			message_envelope_size: message_envelope_size,
			message: message,
			mosaics: mosaics,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.signature_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signature.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.recipient_address_size.to_le_bytes().to_vec());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.amount.serialize());
		serialized.append(&mut self.message_envelope_size.to_le_bytes().to_vec());
		serialized.append(&mut self.message.serialize());
		serialized.append(&mut self.mosaics.len().to_le_bytes().to_vec());
		serialized.append(&mut self.mosaics.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


/// ast_model.display_type == DisplayType.STRUCT
#[derive(Debug)]
pub struct NonVerifiableTransferTransactionV2 {
	pub type_: TransactionType,
	pub version: u8,
	pub entity_body_reserved_1: u16,
	pub network: NetworkType,
	pub timestamp: Timestamp,
	pub signer_public_key_size: u32,
	pub signer_public_key: PublicKey,
	pub fee: Amount,
	pub deadline: Timestamp,
	pub recipient_address_size: u32,
	pub recipient_address: Address,
	pub amount: Amount,
	pub message_envelope_size: u32,
	pub message: Message,
	pub mosaics: Vec<SizePrefixedMosaic>,
}
impl NonVerifiableTransferTransactionV2 {
	const TRANSACTION_VERSION: u8 = 2;
	const TRANSACTION_TYPE: TransactionType = TransactionType::TRANSFER;
	pub fn new() -> Self {
		Self {
			type_: Self::TRANSACTION_TYPE,
			version: Self::TRANSACTION_VERSION,
			entity_body_reserved_1: 0,
			network: NetworkType::default(),
			timestamp: Timestamp::default(),
			signer_public_key_size: 32,
			signer_public_key: PublicKey::default(),
			fee: Amount::default(),
			deadline: Timestamp::default(),
			recipient_address_size: 40,
			recipient_address: Address::default(),
			amount: Amount::default(),
			message_envelope_size: 0,
			message: Message::default(),
			mosaics: Vec::new(),
		}
	}
	pub fn default() -> Self {
		Self::new()
	}
	pub fn size(&self) -> usize {
		let mut size = 0;
		size += self.type_.size();
		size += 1;
		size += 2;
		size += self.network.size();
		size += self.timestamp.size();
		size += 4;
		size += self.signer_public_key.size();
		size += self.fee.size();
		size += self.deadline.size();
		size += 4;
		size += self.recipient_address.size();
		size += self.amount.size();
		size += 4;
		size += self.message.size();
		size += 4;
		size += self.mosaics.iter().map(|x| x.size()).sum::<usize>();
		size
	}
	pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {
		let type_;
		(type_, payload) = TransactionType::deserialize(payload).unwrap();
		let mut bytes = [0u8; 1];
		bytes.copy_from_slice(payload);
		let version = u8::from_le_bytes(bytes);
		payload = &payload[1 as usize..];
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(payload);
		let entity_body_reserved_1 = u16::from_le_bytes(bytes);
		payload = &payload[2 as usize..];
		if entity_body_reserved_1 != 0 { return None; }
		let network;
		(network, payload) = NetworkType::deserialize(payload).unwrap();
		let timestamp;
		(timestamp, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let signer_public_key_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if signer_public_key_size != 0 { return None; }
		let signer_public_key;
		(signer_public_key, payload) = PublicKey::deserialize(payload).unwrap();
		let fee;
		(fee, payload) = Amount::deserialize(payload).unwrap();
		let deadline;
		(deadline, payload) = Timestamp::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let recipient_address_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		if recipient_address_size != 0 { return None; }
		let recipient_address;
		(recipient_address, payload) = Address::deserialize(payload).unwrap();
		let amount;
		(amount, payload) = Amount::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let message_envelope_size = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let message;
		(message, payload) = Message::deserialize(payload).unwrap();
		let mut bytes = [0u8; 4];
		bytes.copy_from_slice(payload);
		let mosaics_count = u32::from_le_bytes(bytes);
		payload = &payload[4 as usize..];
		let mut mosaics = Vec::new();
		for _ in 0..mosaics_count {
			let element;
			(element, payload) = SizePrefixedMosaic::deserialize(payload).unwrap();
			mosaics.push(element);
		}
		let self_ = Self {
			type_: type_,
			version: version,
			entity_body_reserved_1: entity_body_reserved_1,
			network: network,
			timestamp: timestamp,
			signer_public_key_size: signer_public_key_size,
			signer_public_key: signer_public_key,
			fee: fee,
			deadline: deadline,
			recipient_address_size: recipient_address_size,
			recipient_address: recipient_address,
			amount: amount,
			message_envelope_size: message_envelope_size,
			message: message,
			mosaics: mosaics,
		};
		Some((self_, payload))
	}
	pub fn serialize(&self) -> Vec<u8> {
		let mut serialized = Vec::new();
		serialized.append(&mut self.type_.serialize());
		serialized.append(&mut self.version.to_le_bytes().to_vec());
		serialized.append(&mut self.entity_body_reserved_1.to_le_bytes().to_vec());
		serialized.append(&mut self.network.serialize());
		serialized.append(&mut self.timestamp.serialize());
		serialized.append(&mut self.signer_public_key_size.to_le_bytes().to_vec());
		serialized.append(&mut self.signer_public_key.serialize());
		serialized.append(&mut self.fee.serialize());
		serialized.append(&mut self.deadline.serialize());
		serialized.append(&mut self.recipient_address_size.to_le_bytes().to_vec());
		serialized.append(&mut self.recipient_address.serialize());
		serialized.append(&mut self.amount.serialize());
		serialized.append(&mut self.message_envelope_size.to_le_bytes().to_vec());
		serialized.append(&mut self.message.serialize());
		serialized.append(&mut self.mosaics.len().to_le_bytes().to_vec());
		serialized.append(&mut self.mosaics.iter().fold(
			Vec::new(), |mut a, b| {
				a.append(&mut b.serialize());
				a
			}
		));
		serialized
	}
}


