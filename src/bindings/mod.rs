mod cauldron;
pub use cauldron::*;

mod witch;
pub use witch::*;

mod pairflash;
pub use pairflash::*;

pub type VaultIdType = [u8; 12];
pub type ArtIdType = [u8; 6];
pub type InkIdType = [u8; 6];
pub type SeriesIdType = [u8; 6];