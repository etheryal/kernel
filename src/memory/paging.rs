use bootloader::memory_region::{MemoryRegion, MemoryRegionKind};
use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, FrameDeallocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB}};

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub(crate) fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    unsafe { OffsetPageTable::new(level_4_table, physical_memory_offset) }
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub(crate) fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}


/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct KernelFrameAllocator {
    memory_regions: &'static mut [MemoryRegion],
    next: usize,
}

impl KernelFrameAllocator {
    pub fn init(memory_regions: &'static mut [MemoryRegion]) -> Self {
        KernelFrameAllocator {
            memory_regions,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        // get usable regions from memory map
        let regions = self.memory_regions.iter();
        let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.start..r.end);
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for KernelFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

unsafe impl FrameDeallocator<Size4KiB> for KernelFrameAllocator {
    fn deallocate_frame(&mut self, frame: PhysFrame<S>) {

    }
}