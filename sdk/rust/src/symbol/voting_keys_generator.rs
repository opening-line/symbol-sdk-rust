use crate::symbol::key::*;
use crate::symbol::models::*;

pub trait KeyGenerator {
    fn generate(&mut self) -> PrivateKey;
}

pub struct VotingKeysGenerator<KG: KeyGenerator> {
    root_private_key: PrivateKey,
    private_key_generator: KG,
}
impl<KG: KeyGenerator> VotingKeysGenerator<KG> {
    pub const HEADER_SIZE: usize = 80;
    pub const EPOCH_ENTRY_SIZE: usize = 96;

    pub fn new(root_private_key: PrivateKey, private_key_generator: KG) -> Self {
        Self {
            root_private_key,
            private_key_generator,
        }
    }
    pub fn generate(&mut self, start_epoch: u64, end_epoch: u64) -> Result<Vec<u8>, SymbolError> {
        let num_epochs = match end_epoch.checked_sub(start_epoch) {
            Some(x) => x + 1,
            None => {
                return Err(SymbolError::OverflowError(format!(
                    "end_epoch is less than start_epoch. ({} < {})",
                    end_epoch, start_epoch
                )))
            }
        };

        let mut buffer =
            Vec::with_capacity(Self::HEADER_SIZE + Self::EPOCH_ENTRY_SIZE * num_epochs as usize);

        buffer.extend_from_slice(&start_epoch.to_le_bytes()); // 0..8   // start key identifier
        buffer.extend_from_slice(&end_epoch.to_le_bytes()); // 8..16  // end key identifier
        buffer.extend_from_slice(&0xFFFFFFFFFFFFFFFFu64.to_le_bytes()); // 16..24 // reserved - last (used) key identifier
        buffer.extend_from_slice(&0xFFFFFFFFFFFFFFFFu64.to_le_bytes()); // 24..32 // reserved - last wiped key identifier
        buffer.extend_from_slice(&self.root_private_key.public_key().to_bytes()); // 32..64 // root voting public key
        buffer.extend_from_slice(&start_epoch.to_le_bytes()); // 64..72 // level 1/1 start key identifier
        buffer.extend_from_slice(&end_epoch.to_le_bytes()); // 72..80 // level 1/1 end key identifier

        for i in 0..num_epochs {
            let identifier = end_epoch - i;
            let child_private_key = self.private_key_generator.generate();

            let mut parent_signed_payload_buffer = Vec::with_capacity(40);
            parent_signed_payload_buffer
                .append(&mut child_private_key.public_key().to_bytes().to_vec()); // 0..32
            parent_signed_payload_buffer.extend_from_slice(&identifier.to_le_bytes()); // 32..40
            let signature = self.root_private_key.sign(&parent_signed_payload_buffer);

            buffer.extend_from_slice(&child_private_key.to_bytes()); // +32 bytes
            buffer.extend_from_slice(&signature.to_bytes()); // +64 bytes == 96 bytes(EPOCH_ENTRY_SIZE)
        }

        Ok(buffer)
    }
}
