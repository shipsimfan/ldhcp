use std::num::NonZeroUsize;

/// The size of a requested page
pub struct PageSize<const MAX: usize>(NonZeroUsize);

impl<const MAX: usize> PageSize<MAX> {
    /// Creates a new [`PageSize`], verifying it is less than the maximum page size
    pub const fn new(page_size: NonZeroUsize) -> Option<Self> {
        if page_size.get() > MAX {
            return None;
        }

        Some(unsafe { PageSize::new_unchecked(page_size) })
    }

    /// Creates a new [`PageSize`] without verifying it is less than the maximum
    pub const unsafe fn new_unchecked(page_size: NonZeroUsize) -> Self {
        PageSize(page_size)
    }
}
