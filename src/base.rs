use crate::prelude::*;

pub type mach_error_t = c_int;

// TODO: Better descriptions.
/// A return code.
///
/// # Constants
///
/// * [`kDAReturnSuccess`]         - No error was encountered.
/// * [`kDAReturnError`]           - An error was encountered.
/// * [`kDAReturnBusy`]            - Busy error.
/// * [`kDAReturnBadArgument`]     - Bad argument error.
/// * [`kDAReturnExclusiveAccess`] - Exclusive access error.
/// * [`kDAReturnNoResources`]     - No resources error.
/// * [`kDAReturnNotFound`]        - Not found error.
/// * [`kDAReturnNotMounted`]      - Not mounted error.
/// * [`kDAReturnNotPermitted`]    - Not permitted error.
/// * [`kDAReturnNotPrivileged`]   - Not privileged error.
/// * [`kDAReturnNotReady`]        - Not ready error.
/// * [`kDAReturnNotWritable`]     - Not writable error.
/// * [`kDAReturnUnsupported`]     - Unsupported error.
pub type DAReturn = mach_error_t;

pub const kDAReturnSuccess: DAReturn = 0;
pub const kDAReturnError: DAReturn = -119930879;
pub const kDAReturnBusy: DAReturn = -119930878;
pub const kDAReturnBadArgument: DAReturn = -119930877;
pub const kDAReturnExclusiveAccess: DAReturn = -119930876;
pub const kDAReturnNoResources: DAReturn = -119930875;
pub const kDAReturnNotFound: DAReturn = -119930874;
pub const kDAReturnNotMounted: DAReturn = -119930873;
pub const kDAReturnNotPermitted: DAReturn = -119930872;
pub const kDAReturnNotPrivileged: DAReturn = -119930871;
pub const kDAReturnNotReady: DAReturn = -119930870;
pub const kDAReturnNotWritable: DAReturn = -119930869;
pub const kDAReturnUnsupported: DAReturn = -119930868;

pub const TRUE: Boolean = true as _;
pub const FALSE: Boolean = false as _;
