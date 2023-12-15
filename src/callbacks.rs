use crate::prelude::*;

/// Type of the callback function used by [`DARegisterDiskAppearedCallback`].
///
/// # Parameters
///
/// * `disk`    - A disk object.
/// * `context` - The user-defined context parameter given to the registration function.
pub type DADiskAppearedCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, context: *mut c_void)>;

/// Type of the callback function used by [`DARegisterDiskDescriptionChangedCallback`].
///
/// # Parameters
///
/// * `disk`    - A disk object.
/// * `keys`    - A list of changed keys.
/// * `context` - The user-defined context parameter given to the registration function.
pub type DADiskDescriptionChangedCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, keys: CFArrayRef, context: *mut c_void)>;

/// Type of the callback function used by [`DARegisterDiskDisappearedCallback`].
///
/// # Parameters
///
/// * `disk`    - A disk object.
/// * `context` - The user-defined context parameter given to the registration function.
pub type DADiskDisappearedCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, context: *mut c_void)>;

/// Type of the callback function used by [`DADiskMount`].
///
/// # Parameters
///
/// * `disk`      - The disk object.
/// * `dissenter` - A dissenter object on failure or NULL on success.
/// * `context`   - The user-defined context parameter given to the mount function.
///
/// # Discussion
///
/// If the disk is already mounted, then status code in the
/// dissenter object will be set to [`kDAReturnBusy`].
pub type DADiskMountCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, dissenter: DADissenterRef, context: *mut c_void)>;

/// Type of the callback function used by [`DARegisterDiskMountApprovalCallback`].
///
/// # Parameters
///
/// * `disk`    - A disk object.
/// * `context` - The user-defined context parameter given to the registration function.
///
/// # Returns
///
/// A dissenter reference. Pass NULL to approve.
///
/// # Discussion
///
/// The caller of this callback receives a reference to the returned object. The
/// caller also implicitly retains the object and is responsible for releasing it
/// with [`CFRelease`].
pub type DADiskMountApprovalCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, context: *mut c_void) -> DADissenterRef>;

/// Type of the callback function used by [`DADiskRename`].
///
/// # Parameters
///
/// * `disk`      - The disk object.
/// * `dissenter` - A dissenter object on failure or NULL on success.
/// * `context`   - The user-defined context parameter given to the rename function.
pub type DADiskRenameCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, dissenter: DADissenterRef, context: *mut c_void)>;

/// Type of the callback function used by [`DADiskUnmount`].
///
/// # Parameters
///
/// * `disk`      - The disk object.
/// * `dissenter` - A dissenter object on failure or NULL on success.
/// * `context`   - The user-defined context parameter given to the unmount function.
pub type DADiskUnmountCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, dissenter: DADissenterRef, context: *mut c_void)>;

/// Type of the callback function used by [`DARegisterDiskUnmountApprovalCallback`].
///
/// # Parameters
///
/// * `disk`    - A disk object.
/// * `context` - The user-defined context parameter given to the registration function.
///
/// # Returns
///
/// A dissenter reference. Pass NULL to approve.
///
/// # Discussion
///
/// The caller of this callback receives a reference to the returned object.  The
/// caller also implicitly retains the object and is responsible for releasing it
/// with [`CFRelease`].
pub type DADiskUnmountApprovalCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, context: *mut c_void) -> DADissenterRef>;

/// Type of the callback function used by [`DADiskEject`].
///
/// # Parameters
///
/// * `disk`      - The disk object.
/// * `dissenter` - A dissenter object on failure or NULL on success.
/// * `context`   - The user-defined context parameter given to the eject function."#]
pub type DADiskEjectCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, dissenter: DADissenterRef, context: *mut c_void)>;

/// Type of the callback function used by [`DARegisterDiskEjectApprovalCallback`].
///
/// # Parameters
///
/// * `disk`    - A disk object.
/// * `context` - The user-defined context parameter given to the registration function.
///
/// # Returns
///
/// A dissenter reference. Pass NULL to approve.
///
/// # Discussion
///
/// The caller of this callback receives a reference to the returned object. The
/// caller also implicitly retains the object and is responsible for releasing it
/// with [`CFRelease`]
pub type DADiskEjectApprovalCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, context: *mut c_void) -> DADissenterRef>;

/// Type of the callback function used by [`DADiskClaim`].
///
/// # Parameters
///
/// * `disk`      - The disk object.
/// * `dissenter` - A dissenter object on failure or NULL on success.
/// * `context`   - The user-defined context parameter given to the claim function.
pub type DADiskClaimCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, dissenter: DADissenterRef, context: *mut c_void)>;

/// Type of the callback function used by [`DADiskClaim`].
///
/// # Parameters
///
/// * `disk`    - The disk object.
/// * `context` - The user-defined context parameter given to the claim function.
///
/// # Returns
///
/// A dissenter reference. Pass NULL to release claim.
///
/// # Discussion
///
/// The caller of this callback receives a reference to the returned object. The
/// caller also implicitly retains the object and is responsible for releasing it
/// with [`CFRelease`].
pub type DADiskClaimReleaseCallback =
    Option<unsafe extern "C" fn(disk: DADiskRef, context: *mut c_void) -> DADissenterRef>;

/// Type of the callback function used by [`DARegisterDiskPeekCallback`].
///
/// # Parameters
///
/// * `disk`    - A disk object.
/// * `context` - The user-defined context parameter given to the registration function.
///
/// # Discussion
///
/// The peek callback functions are called in a specific order, from lowest order to highest
/// order. [`DADiskClaim`] could be used here to claim the disk object and
/// [`DADiskSetOptions`] could be used here to set up options on the disk object.
pub type DADiskPeekCallback = Option<unsafe extern "C" fn(disk: DADiskRef, context: *mut c_void)>;
