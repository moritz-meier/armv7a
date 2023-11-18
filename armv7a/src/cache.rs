use core::ops::RangeBounds;

mod cp15_cache;
mod pl310_cache;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CacheOp {
    Invalidate,
    Clean,
    CleanInvalidate,
}

pub fn enable_branch_prediction() {
    cp15_cache::cp15_enable_branch_prediction()
}

pub fn disable_branch_prediction() {
    cp15_cache::cp15_disable_branch_prediction()
}

pub fn invalidate_branch_prediction() {
    cp15_cache::cp15_invalidate_branch_prediction_all();
}

pub fn enable_icache() {
    cp15_cache::cp15_enable_icache()
}

pub fn disable_icache() {
    cp15_cache::cp15_disable_icache()
}

pub fn invalidate_icache_all() {
    cp15_cache::cp15_invalidate_icache_all();
}

pub fn invalidate_icache<R>(virt_range: R)
where
    R: RangeBounds<u32> + Iterator<Item = u32>,
{
    cp15_cache::cp15_invalidate_icache_mva(virt_range)
}

pub fn enable_dcache() {
    cp15_cache::cp15_enable_dcache()
}

pub fn disable_dcache() {
    cp15_cache::cp15_disable_dcache()
}

pub fn invalidate_dcache_all() {
    cp15_cache::cp15_op_dcache_all(CacheOp::Invalidate)
}

pub fn invalidate_dcache<R>(virt_range: R)
where
    R: RangeBounds<u32> + Iterator<Item = u32>,
{
    cp15_cache::cp15_op_dcache_mva(virt_range, CacheOp::Invalidate)
}

pub fn clean_dcache_all() {
    cp15_cache::cp15_op_dcache_all(CacheOp::Clean)
}

pub fn clean_dcache<R>(virt_range: R)
where
    R: RangeBounds<u32> + Iterator<Item = u32>,
{
    cp15_cache::cp15_op_dcache_mva(virt_range, CacheOp::Clean)
}

pub fn flush_dcache_all() {
    cp15_cache::cp15_op_dcache_all(CacheOp::CleanInvalidate)
}

pub fn flush_dcache<R>(virt_range: R)
where
    R: RangeBounds<u32> + Iterator<Item = u32>,
{
    cp15_cache::cp15_op_dcache_mva(virt_range, CacheOp::CleanInvalidate)
}
