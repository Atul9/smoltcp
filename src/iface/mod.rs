/*! Network interface logic.

The `iface` module deals with the *network interfaces*. It filters incoming frames,
provides lookup and caching of hardware addresses, and handles management packets.
*/

#[cfg(feature = "ethernet")]
mod ethernet;
#[cfg(feature = "ethernet")]
mod neighbor;
mod route;

#[cfg(feature = "ethernet")]
pub use self::ethernet::{
    Interface as EthernetInterface, InterfaceBuilder as EthernetInterfaceBuilder,
};
#[cfg(feature = "ethernet")]
pub(crate) use self::neighbor::Answer as NeighborAnswer;
#[cfg(feature = "ethernet")]
pub use self::neighbor::Cache as NeighborCache;
#[cfg(feature = "ethernet")]
pub use self::neighbor::Neighbor;
pub use self::route::{Route, Routes};
