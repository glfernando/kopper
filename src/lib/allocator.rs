use crate::println;
use core::cell::UnsafeCell;
use core::mem;
use core::ptr;

const FREE: u32 = 0xA110F7EE;
const USED: u32 = 0xA10045ED;
const CHUNK_SIZE: u32 = mem::size_of::<Chunk>() as u32;
const MIN_ALIGNMENT: u32 = mem::size_of::<Chunk>() as u32;

// helper function to align size
#[inline(always)]
fn align_up(x: u32, a: u32) -> u32 {
    let r = x + a - 1;
    r & !(a - 1)
}

/// struct representing an allocator.
pub struct Allocator {
    start: *const u8,
    end: *const u8,
    current: UnsafeCell<*const u8>,
}

unsafe impl Sync for Allocator {}

impl Allocator {
    /// create new Allocator instance
    pub const fn new(start: *const u8, end: *const u8) -> Self {
        Self {
            start,
            end,
            current: UnsafeCell::new(ptr::null()),
        }
    }
    fn init(&self) {
        unsafe {
            if (*self.current.get()).is_null() {
                // validate parameters
                assert!(self.end > self.start);
                assert!(self.end.offset_from(self.start) <= u32::MAX as isize);

                // TODO: use current to cache free chunk;
                *self.current.get() = self.start as *mut u8;

                let e = Chunk::from_ptr_mut(self.start as *mut u8);

                e.state = FREE;
                e.size = self.end.offset_from(self.start) as u32 - CHUNK_SIZE;
            }
        }
    }

    /// find free memory chunk which complaints with specified size and alignment
    fn find_free(&self, size: u32, align: u32) -> Option<&mut Chunk> {
        self.iter_mut().find(|c| {
            if c.state != FREE {
                return false;
            }

            // merge all free chunks
            // create new iterator for the rest of the chunks
            let mut merge_size = 0u32;
            let start = unsafe { c.mem_ptr().offset(c.size as isize) };
            for tmp in ChunkIterMut::new(start, self.end) {
                if tmp.state == FREE {
                    merge_size += CHUNK_SIZE + tmp.size;
                //tmp.state = 0;
                } else {
                    break;
                }
            }

            let chunk = Chunk::from_ptr_mut(c.as_ptr());
            chunk.size += merge_size;

            let ptr = c.mem_ptr();
            let total_size = size + ptr.align_offset(align as usize) as u32;

            c.size >= total_size
        })
    }

    /// returns a mutable iterator
    pub fn iter_mut(&self) -> ChunkIterMut {
        ChunkIterMut::new(self.start, self.end)
    }

    /// allocate memory
    pub fn alloc(&self, size: u32, align: u32) -> *mut u8 {
        self.init();

        // update alignment
        let align = if align > MIN_ALIGNMENT {
            align
        } else {
            MIN_ALIGNMENT
        };

        // find a free chunk big enough
        let chunk = if let Some(c) = self.find_free(size, align) {
            c
        } else {
            // no memory
            return ptr::null_mut();
        };

        let ptr = chunk.mem_ptr();
        let align_offset = ptr.align_offset(align as usize) as u32;

        // check if we can create a free chunk because of the alignment
        let chunk = if align_offset != 0 {
            // use unwrap since it is guaranteed it can split it
            chunk.split_at(align_offset - CHUNK_SIZE).unwrap()
        } else {
            chunk
        };

        // align size if needed
        let size = align_up(size + CHUNK_SIZE, MIN_ALIGNMENT) - CHUNK_SIZE;
        let size = if size > chunk.size { chunk.size } else { size };

        // check if there is space for creating a free chunk after the current one
        if let Some(new) = chunk.split_at(size) {
            new.state = FREE;
        }

        chunk.state = USED;
        chunk.mem_ptr()
    }

    /// free memory
    pub fn free(&self, p: *mut u8) {
        let chunk = Chunk::from_mem_ptr(p);
        match chunk.state {
            FREE => println!("pointer {:p} already freed", p),
            USED => chunk.state = FREE,
            _ => println!("invalid flags"),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
/// struct represents a memory chunk
pub struct Chunk {
    state: u32,
    size: u32,
}

impl Chunk {
    fn from_ptr(ptr: *const u8) -> &'static mut Self {
        unsafe { &mut *(ptr as *mut Chunk) }
    }

    fn from_ptr_mut(ptr: *mut u8) -> &'static mut Self {
        unsafe { &mut *(ptr as *mut Chunk) }
    }

    fn from_mem_ptr(ptr: *mut u8) -> &'static mut Self {
        Chunk::from_ptr_mut(unsafe { ptr.offset(-(CHUNK_SIZE as isize)) })
    }

    fn next_mut(&self) -> &'static mut Self {
        unsafe {
            let ptr = self.as_ptr().offset((CHUNK_SIZE + self.size) as isize);
            Chunk::from_ptr_mut(ptr as *mut u8)
        }
    }

    fn as_ptr(&self) -> *mut u8 {
        self as *const Chunk as *mut u8
    }

    fn mem_ptr(&self) -> *mut u8 {
        unsafe { self.as_ptr().offset(CHUNK_SIZE as isize) }
    }

    // split a chunk into 2 at specified offset and return new chunk
    fn split_at(&mut self, offset: u32) -> Option<&mut Chunk> {
        if self.size <= CHUNK_SIZE {
            None
        } else if offset > self.size - CHUNK_SIZE {
            None
        } else {
            let total_size = self.size;
            self.size = offset;
            let new = self.next_mut();
            new.size = total_size - offset - CHUNK_SIZE;
            Some(new)
        }
    }
}

/// Iterator: in order to iterate all memory chunks inside the heap
pub struct ChunkIterMut {
    start: *const u8,
    end: *const u8,
}

impl ChunkIterMut {
    fn new(start: *const u8, end: *const u8) -> Self {
        Self { start, end }
    }
}

impl Iterator for ChunkIterMut {
    type Item = &'static mut Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let chunk = Chunk::from_ptr(self.start);
            // make sure it is a valid chunk
            assert!(chunk.state == FREE || chunk.state == USED);
            // update start to next element
            unsafe {
                self.start = self.start.add((CHUNK_SIZE + chunk.size) as usize);
            }
            Some(chunk)
        } else {
            None
        }
    }
}
