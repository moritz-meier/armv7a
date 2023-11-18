use core::{mem::size_of, ops::RangeBounds};

use tock_registers::{
    fields::{FieldValue, TryFromValue},
    interfaces::{ReadWriteable, Readable},
};

use super::CacheOp;
use crate::{asm::*, registers::*};

pub(super) fn cp15_invalidate_branch_prediction_all() {
    cp15_invalidate_branch_predictor()
}

pub(super) fn cp15_invalidate_icache_all() {
    cp15_invalidate_icache_to_unification()
}

pub(super) fn cp15_invalidate_icache_mva<R>(virt_range: R)
where
    R: RangeBounds<u32> + Iterator<Item = u32>,
{
    cp15_select_cache(
        CSSELR::Level::Value::Level1,
        CSSELR::InD::Value::Instruction,
    );

    let CacheInfo {
        linewidth_bytes, ..
    } = CacheInfo::get_selected();

    dsb();

    virt_range
        .step_by(linewidth_bytes as usize)
        .for_each(|vaddr| {
            let vaddr = vaddr & !(linewidth_bytes - 1);
            cp15_invalidate_icache_to_unification_bymva(vaddr);
        })
}

pub(super) fn cp15_op_dcache_all(op: CacheOp) {
    let clidr = CLIDR.extract();

    for level in LEVELS {
        let ctype = (clidr.get() >> (level as u32 * 3)) & 0b111;
        let ctype = CLIDR::Ctype1::Value::try_from_value(ctype);

        match ctype {
            Some(
                CLIDR::Ctype1::Value::Data
                | CLIDR::Ctype1::Value::InstructionAndData
                | CLIDR::Ctype1::Value::Unified,
            ) => cp15_op_dcache_level(level, op),
            _ => break,
        }
    }
}

pub(super) fn cp15_op_dcache_mva<R>(virt_range: R, op: CacheOp)
where
    R: RangeBounds<u32> + Iterator<Item = u32>,
{
    cp15_select_cache(
        CSSELR::Level::Value::Level1,
        CSSELR::InD::Value::DataOrUnified,
    );

    let CacheInfo {
        linewidth_bytes, ..
    } = CacheInfo::get_selected();

    let start = match virt_range.start_bound() {
        core::ops::Bound::Excluded(start) => start.saturating_add(1),
        core::ops::Bound::Included(start) => *start,
        core::ops::Bound::Unbounded => u32::MIN,
    };

    let end = match virt_range.end_bound() {
        core::ops::Bound::Excluded(end) => end.saturating_sub(1),
        core::ops::Bound::Included(end) => *end,
        core::ops::Bound::Unbounded => u32::MAX,
    };

    dsb();

    virt_range
        .step_by(linewidth_bytes as usize)
        .for_each(|vaddr| {
            let vaddr = vaddr & !(linewidth_bytes - 1);
            let mut op = op;

            if vaddr < start && op == CacheOp::Invalidate {
                op = CacheOp::CleanInvalidate;
            }

            if vaddr.saturating_add(linewidth_bytes) > end && op == CacheOp::Invalidate {
                op = CacheOp::CleanInvalidate;
            }

            match op {
                CacheOp::Invalidate => cp15_invalidate_dcacheline_to_coherence_bymva(vaddr),
                CacheOp::Clean => cp15_clean_dcacheline_to_coherence_bymva(vaddr),
                CacheOp::CleanInvalidate => {
                    cp15_clean_invalidate_dcacheline_to_coherence_bymva(vaddr)
                }
            }
        });

    isb();
}

fn cp15_op_dcache_level(level: CSSELR::Level::Value, op: CacheOp) {
    cp15_select_cache(level, CSSELR::InD::Value::DataOrUnified);
    let cache = CacheInfo::get_selected();
    let way_shift = size_of::<u32>() as u32 - num_bits(cache.num_ways);
    let set_shift = num_bits(cache.linewidth_bytes);

    dsb();

    for way in 0..cache.num_ways {
        for set in 0..cache.num_sets {
            let wayset = way << way_shift | set << set_shift | (level as u32) << 1;

            match op {
                CacheOp::Invalidate => cp15_invalidate_dcacheline_bywayset(wayset),
                CacheOp::Clean => cp15_clean_dcacheline_bywayset(wayset),
                CacheOp::CleanInvalidate => cp15_clean_invalidate_dcacheline_bywayset(wayset),
            }
        }
    }

    isb();
}

#[derive(Clone, Copy, Debug)]
struct CacheInfo {
    num_sets: u32,
    num_ways: u32,
    linewidth_bytes: u32,
}

impl CacheInfo {
    fn get_selected() -> Self {
        let ccsidr = CCSIDR.extract();

        Self {
            num_sets: ccsidr.read(CCSIDR::NumSets) + 1,
            num_ways: ccsidr.read(CCSIDR::Associativity) + 1,
            linewidth_bytes: (1 << (ccsidr.read(CCSIDR::LineSize) + 2)) * size_of::<u32>() as u32,
        }
    }
}

const LEVELS: [CSSELR::Level::Value; 7] = [
    CSSELR::Level::Value::Level1,
    CSSELR::Level::Value::Level2,
    CSSELR::Level::Value::Level3,
    CSSELR::Level::Value::Level4,
    CSSELR::Level::Value::Level5,
    CSSELR::Level::Value::Level6,
    CSSELR::Level::Value::Level7,
];

#[inline(always)]
fn num_bits(val: u32) -> u32 {
    val.next_power_of_two().ilog2()
}

#[inline(always)]
fn cp15_select_cache(level: CSSELR::Level::Value, typ: CSSELR::InD::Value) {
    CSSELR.modify(FieldValue::from(level) + FieldValue::from(typ));
}

/// Enable DCache
#[inline(always)]
pub(super) fn cp15_enable_dcache() {
    SCTLR.modify(SCTLR::C::Enable);
    isb()
}

/// Disable DCache
#[inline(always)]
pub(super) fn cp15_disable_dcache() {
    SCTLR.modify(SCTLR::C::Disable);
    isb()
}

/// Enable ICache
#[inline(always)]
pub(super) fn cp15_enable_icache() {
    SCTLR.modify(SCTLR::I::Enable);
    isb()
}

/// Disable ICache
#[inline(always)]
pub(super) fn cp15_disable_icache() {
    SCTLR.modify(SCTLR::I::Disable);
    isb()
}

/// Enable Branch Prediction
#[inline(always)]
pub(super) fn cp15_enable_branch_prediction() {
    SCTLR.modify(SCTLR::Z::Enable);
    isb()
}

/// Disable Branch Prediction
#[inline(always)]
pub(super) fn cp15_disable_branch_prediction() {
    SCTLR.modify(SCTLR::Z::Disable);
    isb()
}

// ARM Architecture Reference Manual ARMv7-A and ARMv7-R edition; ARM DDI 0406C.d
// B4 System Control Registers in a VMSA implementation
// B4.2 VMSA system control operations described by function

/// Invalidate all instruction caches to PoU Inner Shareable.<br/>
/// If branch predictors are architecturally-visible, also flushes branch predictors
#[inline(always)]
fn cp15_invalidate_icache_to_unification_inner_shareable() {
    ICIALLUIS.set(0);
    dsb();
    isb()
}

/// Invalidate all entries from branch predictors Inner Shareable.
#[inline(always)]
fn cp15_invalidate_branch_predictors_inner_shareable() {
    BPIALLIS.set(0);
    dsb();
    isb()
}

/// Invalidate all instruction caches to PoU.<br/>
/// If branch predictors are architecturally-visible, also flushes branch predictors.
#[inline(always)]
fn cp15_invalidate_icache_to_unification() {
    ICIALLU.set(0);
    dsb();
    isb()
}

/// Invalidate instruction cache line by MVA to PoU.
#[inline(always)]
fn cp15_invalidate_icache_to_unification_bymva(va: u32) {
    ICIMVAU.set(va);
    dsb();
    isb()
}

/// Invalidate all entries from branch predictors.
#[inline(always)]
fn cp15_invalidate_branch_predictor() {
    BPIALL.set(0);
    dsb();
    isb()
}

/// Invalidate MVA from branch predictors.
#[inline(always)]
fn cp15_invalidate_branch_predictor_bymva(va: u32) {
    BPIMVA.set(va);
    dsb();
    isb()
}

/// Invalidate data or unified cache line by MVA to PoC.
#[inline(always)]
fn cp15_invalidate_dcacheline_to_coherence_bymva(va: u32) {
    DCIMVAC.set(va);
    dmb()
}

/// Invalidate data or unified cache line by set/way.
#[inline(always)]
fn cp15_invalidate_dcacheline_bywayset(setway: u32) {
    DCISW.set(setway);
    dmb()
}

/// Clean data or unified cache line by MVA to PoC.
#[inline(always)]
fn cp15_clean_dcacheline_to_coherence_bymva(va: u32) {
    DCCMVAC.set(va);
    dmb()
}

/// Clean data or unified cache line by set/way.
#[inline(always)]
fn cp15_clean_dcacheline_bywayset(wayset: u32) {
    DCCSW.set(wayset);
    dmb()
}

/// Clean data or unified cache line by MVA to PoU.
#[inline(always)]
fn cp15_clean_dcacheline_to_unification_bymva(va: u32) {
    DCCMVAU.set(va);
    dmb()
}

/// Clean and Invalidate data or unified cache line by MVA to PoC.
#[inline(always)]
fn cp15_clean_invalidate_dcacheline_to_coherence_bymva(va: u32) {
    DCCIMVAC.set(va);
    dmb()
}

/// Clean and Invalidate data or unified cache line by set/way.
#[inline(always)]
fn cp15_clean_invalidate_dcacheline_bywayset(wayset: u32) {
    DCCISW.set(wayset);
    dmb()
}
