pub const PHYSICAL_MEMORY_END: usize = 0x88000000;//DRAM 物理内存结束地址硬编码到内核中

//默认的 DRAM 物理内存地址范围就是 [0x80000000,0x88000000)
//我们可以用来存别的东西的物理内存的物理地址范围是：[KernelEnd, 0x88000000) 。这里的 KernelEnd​ 为内核代码结尾的物理地址

pub const KERNEL_BEGIN_PADDR: usize = 0x80200000;
pub const KERNEL_BEGIN_VADDR: usize = 0x80200000;

pub const MAX_PHYSICAL_MEMORY: usize = 0x8000000;//0x800000后是操作系统的
pub const MAX_PHYSICAL_PAGES: usize = MAX_PHYSICAL_MEMORY >> 12;//2^12=4096 one ppn location
// 内核堆大小为8MiB
pub const KERNEL_HEAP_SIZE: usize = 0x800000;
