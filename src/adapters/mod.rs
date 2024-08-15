pub use self::{
    btreemap::ParkingLotRwLockBTreeMapTable, btreemap::StdRwLockBTreeMapTable,
    chashmap::CHashMapTable, contrie::ContrieTable, crossbeam_skiplist::CrossbeamSkipMapTable,
    dashmap::DashMapTable, evmap::EvmapTable, flurry::FlurryTable, scc::SccMapTable,
    std::ParkingLotRwLockStdHashMapTable, std::StdRwLockStdHashMapTable,
    light_map::LightMap,
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
mod evmap;
mod flurry;
mod scc;
mod std;
mod light_map;

type Value = u32;
