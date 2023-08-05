//! Shared RISC-V intrinsics

mod p;
mod zk;

pub use p::*;
pub use zk::*;

use crate::arch::asm;

/// Generates the `PAUSE` instruction
///
/// The PAUSE instruction is a HINT that indicates the current hart's rate of instruction retirement
/// should be temporarily reduced or paused. The duration of its effect must be bounded and may be zero.
#[inline]
pub fn pause() {
    unsafe { asm!(".insn i 0x0F, 0, x0, x0, 0x010", options(nomem, nostack)) }
}

/// Generates the `NOP` instruction
///
/// The NOP instruction does not change any architecturally visible state, except for
/// advancing the `pc` and incrementing any applicable performance counters.
#[inline]
pub fn nop() {
    unsafe { asm!("nop", options(nomem, nostack)) }
}

/// Generates the `WFI` instruction
///
/// The WFI instruction provides a hint to the implementation that the current hart can be stalled
/// until an interrupt might need servicing. This instruction is a hint,
/// and a legal implementation is to simply implement WFI as a NOP.
#[inline]
pub unsafe fn wfi() {
    asm!("wfi", options(nomem, nostack))
}

/// Generates the `FENCE.I` instruction
///
/// A FENCE.I instruction ensures that a subsequent instruction fetch on a RISC-V hart will see
/// any previous data stores already visible to the same RISC-V hart.
///
/// FENCE.I does not ensure that other RISC-V harts' instruction fetches will observe the
/// local hart's stores in a multiprocessor system.
#[inline]
pub unsafe fn fence_i() {
    asm!("fence.i", options(nostack))
}

/// Supervisor memory management fence for given virtual address and address space
///
/// The fence orders only reads and writes made to leaf page table entries corresponding to
/// the virtual address in parameter `vaddr`, for the address space identified by integer parameter
/// `asid`. Accesses to global mappings are not ordered. The fence also invalidates all
/// address-translation cache entries that contain leaf page table entries corresponding to the
/// virtual address in parameter `vaddr` and that match the address space identified by integer
/// parameter `asid`, except for entries containing global mappings.
#[inline]
pub unsafe fn sfence_vma(vaddr: usize, asid: usize) {
    asm!("sfence.vma {}, {}", in(reg) vaddr, in(reg) asid, options(nostack))
}

/// Supervisor memory management fence for given virtual address
///
/// The fence orders only reads and writes made to leaf page table entries corresponding to
/// the virtual address in parameter `vaddr`, for all address spaces.
/// The fence also invalidates all address-translation cache entries that contain leaf page
/// table entries corresponding to the virtual address in parameter `vaddr`, for all address spaces.
#[inline]
pub unsafe fn sfence_vma_vaddr(vaddr: usize) {
    asm!("sfence.vma {}, x0", in(reg) vaddr, options(nostack))
}

/// Supervisor memory management fence for given address space
///
/// The fence orders all reads and writes made to any level of the page tables,
/// but only for the address space identified by integer parameter `asid`.
///
/// Accesses to global mappings are not ordered. The fence also invalidates all
/// address-translation cache entries matching the address space identified by integer
/// parameter `asid`, except for entries containing global mappings.
#[inline]
pub unsafe fn sfence_vma_asid(asid: usize) {
    asm!("sfence.vma x0, {}", in(reg) asid, options(nostack))
}

/// Supervisor memory management fence for all address spaces and virtual addresses
///
/// The fence orders all reads and writes made to any level of the page
/// tables, for all address spaces. The fence also invalidates all address-translation cache entries,
/// for all address spaces.
#[inline]
pub unsafe fn sfence_vma_all() {
    asm!("sfence.vma", options(nostack))
}

/// Invalidate supervisor translation cache for given virtual address and address space
///
/// This instruction invalidates any address-translation cache entries that an
/// `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate.
#[inline]
pub unsafe fn sinval_vma(vaddr: usize, asid: usize) {
    // asm!("sinval.vma {}, {}", in(reg) vaddr, in(reg) asid, options(nostack))
    asm!(".insn r 0x73, 0, 0x0B, x0, {}, {}", in(reg) vaddr, in(reg) asid, options(nostack))
}

/// Invalidate supervisor translation cache for given virtual address
///
/// This instruction invalidates any address-translation cache entries that an
/// `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate.
#[inline]
pub unsafe fn sinval_vma_vaddr(vaddr: usize) {
    asm!(".insn r 0x73, 0, 0x0B, x0, {}, x0", in(reg) vaddr, options(nostack))
}

/// Invalidate supervisor translation cache for given address space
///
/// This instruction invalidates any address-translation cache entries that an
/// `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate.
#[inline]
pub unsafe fn sinval_vma_asid(asid: usize) {
    asm!(".insn r 0x73, 0, 0x0B, x0, x0, {}", in(reg) asid, options(nostack))
}

/// Invalidate supervisor translation cache for all address spaces and virtual addresses
///
/// This instruction invalidates any address-translation cache entries that an
/// `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate.
#[inline]
pub unsafe fn sinval_vma_all() {
    asm!(".insn r 0x73, 0, 0x0B, x0, x0, x0", options(nostack))
}

/// Generates the `SFENCE.W.INVAL` instruction
///
/// This instruction guarantees that any previous stores already visible to the current RISC-V hart
/// are ordered before subsequent `SINVAL.VMA` instructions executed by the same hart.
#[inline]
pub unsafe fn sfence_w_inval() {
    // asm!("sfence.w.inval", options(nostack))
    asm!(".insn i 0x73, 0, x0, x0, 0x180", options(nostack))
}

/// Generates the `SFENCE.INVAL.IR` instruction
///
/// This instruction guarantees that any previous SINVAL.VMA instructions executed by the current hart
/// are ordered before subsequent implicit references by that hart to the memory-management data structures.
#[inline]
pub unsafe fn sfence_inval_ir() {
    // asm!("sfence.inval.ir", options(nostack))
    asm!(".insn i 0x73, 0, x0, x0, 0x181", options(nostack))
}

/// Loads virtual machine memory by signed byte integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HLV.B`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hlv_b(src: *const i8) -> i8 {
    let value: i8;
    asm!(".insn i 0x73, 0x4, {}, {}, 0x600", out(reg) value, in(reg) src, options(readonly, nostack));
    value
}

/// Loads virtual machine memory by unsigned byte integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HLV.BU`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hlv_bu(src: *const u8) -> u8 {
    let value: u8;
    asm!(".insn i 0x73, 0x4, {}, {}, 0x601", out(reg) value, in(reg) src, options(readonly, nostack));
    value
}

/// Loads virtual machine memory by signed half integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HLV.H`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hlv_h(src: *const i16) -> i16 {
    let value: i16;
    asm!(".insn i 0x73, 0x4, {}, {}, 0x640", out(reg) value, in(reg) src, options(readonly, nostack));
    value
}

/// Loads virtual machine memory by unsigned half integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HLV.HU`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hlv_hu(src: *const u16) -> u16 {
    let value: u16;
    asm!(".insn i 0x73, 0x4, {}, {}, 0x641", out(reg) value, in(reg) src, options(readonly, nostack));
    value
}

/// Accesses virtual machine instruction by unsigned half integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// the memory being read must be executable in both stages of address translation,
/// but read permission is not required.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HLVX.HU`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hlvx_hu(src: *const u16) -> u16 {
    let insn: u16;
    asm!(".insn i 0x73, 0x4, {}, {}, 0x643", out(reg) insn, in(reg) src, options(readonly, nostack));
    insn
}

/// Loads virtual machine memory by signed word integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HLV.W`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hlv_w(src: *const i32) -> i32 {
    let value: i32;
    asm!(".insn i 0x73, 0x4, {}, {}, 0x680", out(reg) value, in(reg) src, options(readonly, nostack));
    value
}

/// Accesses virtual machine instruction by unsigned word integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// the memory being read must be executable in both stages of address translation,
/// but read permission is not required.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HLVX.WU`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hlvx_wu(src: *const u32) -> u32 {
    let insn: u32;
    asm!(".insn i 0x73, 0x4, {}, {}, 0x683", out(reg) insn, in(reg) src, options(readonly, nostack));
    insn
}

/// Stores virtual machine memory by byte integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HSV.B`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hsv_b(dst: *mut i8, src: i8) {
    asm!(".insn r 0x73, 0x4, 0x31, x0, {}, {}", in(reg) dst, in(reg) src, options(nostack));
}

/// Stores virtual machine memory by half integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HSV.H`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hsv_h(dst: *mut i16, src: i16) {
    asm!(".insn r 0x73, 0x4, 0x33, x0, {}, {}", in(reg) dst, in(reg) src, options(nostack));
}

/// Stores virtual machine memory by word integer
///
/// This instruction performs an explicit memory access as though `V=1`;
/// i.e., with the address translation and protection, and the endianness, that apply to memory
/// accesses in either VS-mode or VU-mode.
///
/// This function is unsafe for it accesses the virtual supervisor or user via a `HSV.W`
/// instruction which is effectively a dereference to any memory address.
#[inline]
pub unsafe fn hsv_w(dst: *mut i32, src: i32) {
    asm!(".insn r 0x73, 0x4, 0x35, x0, {}, {}", in(reg) dst, in(reg) src, options(nostack));
}

/// Hypervisor memory management fence for given guest virtual address and guest address space
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all
/// implicit reads by that hart done for VS-stage address translation for instructions that:
/// - are subsequent to the `HFENCE.VVMA`, and
/// - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed.
///
/// This fence specifies a single guest virtual address, and a single guest address-space identifier.
#[inline]
pub unsafe fn hfence_vvma(vaddr: usize, asid: usize) {
    // asm!("hfence.vvma {}, {}", in(reg) vaddr, in(reg) asid)
    asm!(".insn r 0x73, 0, 0x11, x0, {}, {}", in(reg) vaddr, in(reg) asid, options(nostack))
}

/// Hypervisor memory management fence for given guest virtual address
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all
/// implicit reads by that hart done for VS-stage address translation for instructions that:
/// - are subsequent to the `HFENCE.VVMA`, and
/// - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed.
///
/// This fence specifies a single guest virtual address.
#[inline]
pub unsafe fn hfence_vvma_vaddr(vaddr: usize) {
    asm!(".insn r 0x73, 0, 0x11, x0, {}, x0", in(reg) vaddr, options(nostack))
}

/// Hypervisor memory management fence for given guest address space
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all
/// implicit reads by that hart done for VS-stage address translation for instructions that:
/// - are subsequent to the `HFENCE.VVMA`, and
/// - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed.
///
/// This fence specifies a single guest address-space identifier.
#[inline]
pub unsafe fn hfence_vvma_asid(asid: usize) {
    asm!(".insn r 0x73, 0, 0x11, x0, x0, {}", in(reg) asid, options(nostack))
}

/// Hypervisor memory management fence for all guest address spaces and guest virtual addresses
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all
/// implicit reads by that hart done for VS-stage address translation for instructions that:
/// - are subsequent to the `HFENCE.VVMA`, and
/// - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed.
///
/// This fence applies to any guest address spaces and guest virtual addresses.
#[inline]
pub unsafe fn hfence_vvma_all() {
    asm!(".insn r 0x73, 0, 0x11, x0, x0, x0", options(nostack))
}

/// Hypervisor memory management fence for guest physical address and virtual machine
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads
/// by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA.
///
/// This fence specifies a single guest physical address, **shifted right by 2 bits**, and a single virtual machine
/// by virtual machine identifier (VMID).
#[inline]
pub unsafe fn hfence_gvma(gaddr: usize, vmid: usize) {
    // asm!("hfence.gvma {}, {}", in(reg) gaddr, in(reg) vmid, options(nostack))
    asm!(".insn r 0x73, 0, 0x31, x0, {}, {}", in(reg) gaddr, in(reg) vmid, options(nostack))
}

/// Hypervisor memory management fence for guest physical address
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads
/// by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA.
///
/// This fence specifies a single guest physical address; **the physical address should be shifted right by 2 bits**.
#[inline]
pub unsafe fn hfence_gvma_gaddr(gaddr: usize) {
    asm!(".insn r 0x73, 0, 0x31, x0, {}, x0", in(reg) gaddr, options(nostack))
}

/// Hypervisor memory management fence for given virtual machine
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads
/// by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA.
///
/// This fence specifies a single virtual machine by virtual machine identifier (VMID).
#[inline]
pub unsafe fn hfence_gvma_vmid(vmid: usize) {
    asm!(".insn r 0x73, 0, 0x31, x0, x0, {}", in(reg) vmid, options(nostack))
}

/// Hypervisor memory management fence for all virtual machines and guest physical addresses
///
/// Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads
/// by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA.
///
/// This fence specifies all guest physical addresses and all virtual machines.
#[inline]
pub unsafe fn hfence_gvma_all() {
    asm!(".insn r 0x73, 0, 0x31, x0, x0, x0", options(nostack))
}

/// Invalidate hypervisor translation cache for given guest virtual address and guest address space
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate.
///
/// This fence specifies a single guest virtual address, and a single guest address-space identifier.
#[inline]
pub unsafe fn hinval_vvma(vaddr: usize, asid: usize) {
    // asm!("hinval.vvma {}, {}", in(reg) vaddr, in(reg) asid, options(nostack))
    asm!(".insn r 0x73, 0, 0x13, x0, {}, {}", in(reg) vaddr, in(reg) asid, options(nostack))
}

/// Invalidate hypervisor translation cache for given guest virtual address
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate.
///
/// This fence specifies a single guest virtual address.
#[inline]
pub unsafe fn hinval_vvma_vaddr(vaddr: usize) {
    asm!(".insn r 0x73, 0, 0x13, x0, {}, x0", in(reg) vaddr, options(nostack))
}

/// Invalidate hypervisor translation cache for given guest address space
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate.
///
/// This fence specifies a single guest address-space identifier.
#[inline]
pub unsafe fn hinval_vvma_asid(asid: usize) {
    asm!(".insn r 0x73, 0, 0x13, x0, x0, {}", in(reg) asid, options(nostack))
}

/// Invalidate hypervisor translation cache for all guest address spaces and guest virtual addresses
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate.
///
/// This fence applies to any guest address spaces and guest virtual addresses.
#[inline]
pub unsafe fn hinval_vvma_all() {
    asm!(".insn r 0x73, 0, 0x13, x0, x0, x0", options(nostack))
}

/// Invalidate hypervisor translation cache for guest physical address and virtual machine
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate.
///
/// This fence specifies a single guest physical address, **shifted right by 2 bits**, and a single virtual machine
/// by virtual machine identifier (VMID).
#[inline]
pub unsafe fn hinval_gvma(gaddr: usize, vmid: usize) {
    // asm!("hinval.gvma {}, {}", in(reg) gaddr, in(reg) vmid, options(nostack))
    asm!(".insn r 0x73, 0, 0x33, x0, {}, {}", in(reg) gaddr, in(reg) vmid, options(nostack))
}

/// Invalidate hypervisor translation cache for guest physical address
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate.
///
/// This fence specifies a single guest physical address; **the physical address should be shifted right by 2 bits**.
#[inline]
pub unsafe fn hinval_gvma_gaddr(gaddr: usize) {
    asm!(".insn r 0x73, 0, 0x33, x0, {}, x0", in(reg) gaddr, options(nostack))
}

/// Invalidate hypervisor translation cache for given virtual machine
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate.
///
/// This fence specifies a single virtual machine by virtual machine identifier (VMID).
#[inline]
pub unsafe fn hinval_gvma_vmid(vmid: usize) {
    asm!(".insn r 0x73, 0, 0x33, x0, x0, {}", in(reg) vmid, options(nostack))
}

/// Invalidate hypervisor translation cache for all virtual machines and guest physical addresses
///
/// This instruction invalidates any address-translation cache entries that an
/// `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate.
///
/// This fence specifies all guest physical addresses and all virtual machines.
#[inline]
pub unsafe fn hinval_gvma_all() {
    asm!(".insn r 0x73, 0, 0x33, x0, x0, x0", options(nostack))
}

/// Reads the floating-point control and status register `fcsr`
///
/// Register `fcsr` is a 32-bit read/write register that selects the dynamic rounding mode
/// for floating-point arithmetic operations and holds the accrued exception flag.
///
/// According to "F" Standard Extension for Single-Precision Floating-Point, Version 2.2,
/// register `fcsr` is defined as:
///
/// | Bit index | Meaning |
/// |:----------|:--------|
/// | 0..=4 | Accrued Exceptions (`fflags`) |
/// | 5..=7 | Rounding Mode (`frm`) |
/// | 8..=31 | _Reserved_ |
///
/// For definition of each field, visit [`frrm`] and [`frflags`].
///
/// [`frrm`]: fn.frrm.html
/// [`frflags`]: fn.frflags.html
#[inline]
pub fn frcsr() -> u32 {
    let value: u32;
    unsafe { asm!("frcsr {}", out(reg) value, options(nomem, nostack)) };
    value
}

/// Swaps the floating-point control and status register `fcsr`
///
/// This function swaps the value in `fcsr` by copying the original value to be returned,
/// and then writing a new value obtained from input variable `value` into `fcsr`.
#[inline]
pub fn fscsr(value: u32) -> u32 {
    let original: u32;
    unsafe { asm!("fscsr {}, {}", out(reg) original, in(reg) value, options(nomem, nostack)) }
    original
}

/// Reads the floating-point rounding mode register `frm`
///
/// According to "F" Standard Extension for Single-Precision Floating-Point, Version 2.2,
/// the rounding mode field is defined as listed in the table below:
///
/// | Rounding Mode | Mnemonic | Meaning |
/// |:-------------|:----------|:---------|
/// | 000 | RNE | Round to Nearest, ties to Even |
/// | 001 | RTZ | Round towards Zero |
/// | 010 | RDN | Round Down (towards −∞) |
/// | 011 | RUP | Round Up (towards +∞) |
/// | 100 | RMM | Round to Nearest, ties to Max Magnitude |
/// | 101 |     | _Reserved for future use._ |
/// | 110 |     | _Reserved for future use._ |
/// | 111 | DYN | In Rounding Mode register, _reserved_. |
#[inline]
pub fn frrm() -> u32 {
    let value: u32;
    unsafe { asm!("frrm {}", out(reg) value, options(nomem, nostack)) };
    value
}

/// Swaps the floating-point rounding mode register `frm`
///
/// This function swaps the value in `frm` by copying the original value to be returned,
/// and then writing a new value obtained from the three least-significant bits of
/// input variable `value` into `frm`.
#[inline]
pub fn fsrm(value: u32) -> u32 {
    let original: u32;
    unsafe { asm!("fsrm {}, {}", out(reg) original, in(reg) value, options(nomem, nostack)) }
    original
}

/// Reads the floating-point accrued exception flags register `fflags`
///
/// The accrued exception flags indicate the exception conditions that have arisen
/// on any floating-point arithmetic instruction since the field was last reset by software.
///
/// According to "F" Standard Extension for Single-Precision Floating-Point, Version 2.2,
/// the accrued exception flags is defined as a bit vector of 5 bits.
/// The meaning of each binary bit is listed in the table below.
///
/// | Bit index | Mnemonic | Meaning |
/// |:--|:---|:-----------------|
/// | 4 | NV | Invalid Operation |
/// | 3 | DZ | Divide by Zero |
/// | 2 | OF | Overflow |
/// | 1 | UF | Underflow |
/// | 0 | NX | Inexact |
#[inline]
pub fn frflags() -> u32 {
    let value: u32;
    unsafe { asm!("frflags {}", out(reg) value, options(nomem, nostack)) };
    value
}

/// Swaps the floating-point accrued exception flags register `fflags`
///
/// This function swaps the value in `fflags` by copying the original value to be returned,
/// and then writing a new value obtained from the five least-significant bits of
/// input variable `value` into `fflags`.
#[inline]
pub fn fsflags(value: u32) -> u32 {
    let original: u32;
    unsafe { asm!("fsflags {}, {}", out(reg) original, in(reg) value, options(nomem, nostack)) }
    original
}
