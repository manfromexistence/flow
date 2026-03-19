# Unused Serializer Files

These files were moved from `serializer/src/machine/` because they implement advanced features that are:
1. Not mentioned in LLM_FORMAT_SPEC.md or MACHINE_FORMAT.md
2. Commented out in mod.rs
3. Not needed for core functionality

## What Was Moved

### Advanced Optimization Features (Not in Spec)
- `quantum.rs` - Quantum layout optimization
- `arena.rs` / `arena_batch.rs` - Arena allocation
- `mmap.rs` - Memory-mapped file access
- `prefetch.rs` - CPU cache prefetching
- `simd512.rs` - AVX-512 SIMD operations
- `inline.rs` - Inline string optimization
- `intern.rs` - String interning
- `optimized_rkyv.rs` - RKYV wrapper with optimizations

### Platform-Specific Async I/O (Commented Out)
- `io_uring.rs` - Linux io_uring
- `iocp.rs` - Windows IOCP
- `kqueue.rs` - macOS kqueue
- `direct_io.rs` - Direct I/O
- `blocking.rs` - Blocking I/O wrapper

### Deserialization Helpers (Redundant)
- `deserialize.rs` - Deserialization utilities
- `safe_deserialize.rs` / `safe_deserialize_props.rs` - Safe deserialization wrappers

### Parallel Processing (Commented Out)
- `parallel.rs` - Parallel serialization

## What the Serializer Actually Needs

According to LLM_FORMAT_SPEC.md and MACHINE_FORMAT.md:

### LLM Format (Text)
- Token-efficient text format
- Wrapped dataframes `[headers](rows)`
- Quoted strings for multi-word values
- 52-73% token savings vs JSON

### Machine Format (Binary)
- **Pure RKYV** for zero-copy serialization
- **Zstd compression** (level 1, fast)
- **Automatic caching** (first access decompresses, subsequent cached)
- 41-74% size reduction vs text format

### Core Modules Kept
- `machine/api.rs` - Public API
- `machine/builder.rs` - Builder pattern
- `machine/compress.rs` - Zstd compression
- `machine/format.rs` - Format detection
- `machine/header.rs` - Binary header
- `machine/machine_types.rs` - RKYV types
- `machine/rkyv_compat.rs` - RKYV compatibility
- `machine/serde_compat.rs` - Serde compatibility
- `machine/simd.rs` - Basic SIMD (SSE/AVX2)
- `machine/slot.rs` - Slot-based storage
- `machine/traits.rs` - Traits
- `machine/types.rs` - Type definitions
- `machine/footer.rs` - Binary footer

## Why These Were Removed

The spec clearly states:
> "DX-Machine uses pure RKYV" - identical performance

The advanced features (quantum, arena, mmap, etc.) were:
1. Commented out in `mod.rs`
2. Not mentioned in the specification
3. Not tested (tests were passing without them)
4. Added complexity without clear benefit

## Verification

After removal:
- ✅ `cargo check --lib` passes
- ✅ All 472 library tests pass
- ✅ Core functionality intact
- ✅ LLM format works
- ✅ Machine format (RKYV + Zstd) works
- ✅ Conversions work

## If You Need These Back

These files are preserved in `cursed/serializer_unused/machine/` and can be restored if needed. However, they should only be restored if:
1. They're documented in the spec
2. They have tests
3. They provide measurable benefit
4. They're actually used (not commented out)

## Date Moved

2026-03-20
