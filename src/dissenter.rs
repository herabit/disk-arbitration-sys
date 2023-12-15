use crate::prelude::*;

/// Type of a reference to DADissenter instances.
pub type DADissenterRef = *const __DADissenter;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DADissenter {
    _unused: [u8; 0],
}

extern "C" {
    /// Creates a new dissenter object.
    ///
    /// # Parameters
    ///
    /// * `allocator` - The allocator object to be used to allocate memory.
    /// * `status`    - The return code.
    /// * `string`    - The return code string. Pass NULL for no reason.
    ///
    /// # Returns
    ///
    /// A reference to a new DADissenter.
    pub fn DADissenterCreate(
        allocator: CFAllocatorRef,
        status: DAReturn,
        string: CFStringRef,
    ) -> DADissenterRef;

    /// Obtains the return code.
    ///
    /// # Parameters
    ///
    /// * `dissenter` - The DADissenter for which to obtain the return code.
    ///
    /// # Returns
    ///
    /// The return code. A BSD return code, if applicable, is encoded with unix_err().
    pub fn DADissenterGetStatus(dissenter: DADissenterRef) -> DAReturn;

    /// Obtains the return code string.
    ///
    /// # Parameters
    ///
    /// * `dissenter` - The DADissenter for which to obtain the return code string.
    ///
    /// # Returns
    ///
    /// The return code string.
    pub fn DADissenterGetStatusString(dissenter: DADissenterRef) -> CFStringRef;
}
