use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DADisk {
    _unused: [u8; 0],
}

/// Type of a reference to DADisk instances.
pub type DADiskRef = *mut __DADisk;

/// Options for [`DADiskMount`].
///
/// # Constants
///
/// * [`kDADiskMountOptionDefault`] - Default option.
/// * [`kDADiskMountOptionWhole`]   - Mount the volumes tied to the whole disk object.
pub type DADiskMountOptions = c_uint;
pub const kDADiskMountOptionDefault: DADiskMountOptions = 0;
pub const kDADiskMountOptionWhole: DADiskMountOptions = 1;

/// Options for [`DADiskRename`].
///
/// # Constants
///
/// * [`kDADiskRenameOptionDefault`] - Default option.
pub type DADiskRenameOptions = c_uint;
pub const kDADiskRenameOptionDefault: DADiskRenameOptions = 0;

/// Options for [`DADiskUnmount`].
///
/// # Constants
///
/// * [`kDADiskUnmountOptionDefault`] - Default option.
/// * [`kDADiskUnmountOptionForce`]   - Unmount the volume even if files are still active.
/// * [`kDADiskUnmountOptionWhole`]   - Unmount the volumes tied to the whole disk object.
pub type DADiskUnmountOptions = c_uint;
pub const kDADiskUnmountOptionDefault: DADiskUnmountOptions = 0;
pub const kDADiskUnmountOptionForce: DADiskUnmountOptions = 524288;
pub const kDADiskUnmountOptionWhole: DADiskUnmountOptions = 1;

/// Options for [`DADiskEject`].
///
/// # Constants
///
/// * [`kDADiskEjectOptionDefault`] - Default option.
pub type DADiskEjectOptions = c_uint;
pub const kDADiskEjectOptionDefault: DADiskEjectOptions = 0;

/// Options for [`DADiskClaim`].
///
/// # Constants
///
/// * [`kDADiskClaimOptionDefault`] - Default option.
pub type DADiskClaimOptions = c_uint;
pub const kDADiskClaimOptionDefault: DADiskClaimOptions = 0;

/// Options for [`DADiskGetOptions`] and [`DADiskSetOptions`].
///
/// # Constants
///
/// * [`kDADiskOptionDefault`] - Default option.
pub type DADiskOptions = c_uint;
pub const kDADiskOptionDefault: DADiskOptions = 0;

extern "C" {
    /// Returns the type identifier of all DADisk instances.
    pub fn DADiskGetTypeID() -> CFTypeID;

    /// Creates a new disk object.
    ///
    /// # Parameters
    ///
    /// * `allocator` - The allocator object to be used to allocate memory.
    /// * `session`   - The DASession in which to contact Disk Arbitration.
    /// * `name`      - The BSD device name.
    ///
    /// # Returns
    ///
    /// A reference to a new DADisk.
    ///
    /// # Discussion
    ///
    /// The caller of this function receives a reference to the returned object.
    /// The caller also implicitly retains the object and is responsible for
    /// releasing it with [`CFRelease`].
    pub fn DADiskCreateFromBSDName(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        name: *const c_char,
    ) -> DADiskRef;

    /// Creates a new disk object.
    ///
    /// # Parameters
    ///
    /// * `allocator` - The allocator object to be used to allocate memory.
    /// * `session`   - The DASession in which to contact Disk Arbitration.
    /// * `media`     - The I/O Kit media object.
    ///
    /// # Returns
    ///
    /// A reference to a new DADisk.
    ///
    /// # Discussion
    ///
    /// The caller of this function receives a reference to the returned object. The
    /// caller also implicitly retains the object and is responsible for releasing it
    /// with [`CFRelease`].
    pub fn DADiskCreateFromIOMedia(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        media: io_service_t,
    ) -> DADiskRef;

    /// Creates a new disk object.
    ///
    /// # Parameters
    ///
    /// * `allocator` - The allocator object to be used to allocate memory.
    /// * `session`   - The DASession in which to contact Disk Arbitration.
    /// * `path`      - The BSD mount point.
    ///
    /// # Returns
    ///
    /// A reference to a new DADisk.
    ///
    /// # Discussion
    ///
    /// The caller of this function receives a reference to the returned object. The
    /// caller also implicitly retains the object and is responsible for releasing it
    /// with [`CFRelease`].
    pub fn DADiskCreateFromVolumePath(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        path: CFURLRef,
    ) -> DADiskRef;

    /// Obtains the BSD device name for the specified disk.
    ///
    /// # Parameters
    ///
    /// * `disk` - The DADisk for which to obtain the BSD device name.
    ///
    /// # Returns
    ///
    /// The disk's BSD device name.
    ///
    /// # Discussion
    ///
    /// The BSD device name can be used with opendev() to open the BSD device.
    pub fn DADiskGetBSDName(disk: DADiskRef) -> *const ::std::os::raw::c_char;

    /// Obtains the I/O Kit media object for the specified disk.
    ///
    /// # Parameters
    ///
    /// * `disk` - The DADisk for which to obtain the I/O Kit media object.
    ///
    /// # Returns
    ///
    /// The disk's I/O Kit media object.
    ///
    /// # Discussion
    ///
    /// The caller of this function receives a reference to the returned object. The
    /// caller also implicitly retains the object and is responsible for releasing it
    /// with [`IOObjectRelease`].
    pub fn DADiskCopyIOMedia(disk: DADiskRef) -> io_service_t;

    /// Obtains the Disk Arbitration description of the specified disk.
    ///
    /// # Parameters
    ///
    /// * `disk` - The DADisk for which to obtain the Disk Arbitration description.
    ///
    /// # Returns
    ///
    /// The disk's Disk Arbitration description.
    ///
    /// # Discussion
    ///
    /// This function will contact Disk Arbitration to acquire the latest description
    /// of the specified disk, unless this function is called on a disk object passed
    /// within the context of a registered callback, in which case the description is
    /// current as of that callback event.
    ///
    /// The caller of this function receives a reference to the returned object. The
    /// caller also implicitly retains the object and is responsible for releasing it
    /// with [`CFRelease`].
    pub fn DADiskCopyDescription(disk: DADiskRef) -> CFDictionaryRef;

    /// Obtain the associated whole disk object for the specified disk.
    ///
    /// # Parameters
    ///
    /// * `disk` - The disk object.
    ///
    /// # Returns
    ///
    /// The disk's associated whole disk object.
    ///
    /// # Discussion
    ///
    /// The caller of this function receives a reference to the returned object. The
    /// caller also implicitly retains the object and is responsible for releasing it
    /// with [`CFRelease`].
    pub fn DADiskCopyWholeDisk(disk: DADiskRef) -> DADiskRef;
}

extern "C" {
    /// Mounts the volume at the specified disk object.
    ///
    /// # Parameters
    ///
    /// * `disk`     - The disk object.
    /// * `path`     - The mount path. Pass NULL for a "standard" mount path.
    /// * `options`  - The mount options.
    /// * `callback` - The callback function to call once the mount completes.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DADiskMount(
        disk: DADiskRef,
        path: CFURLRef,
        options: DADiskMountOptions,
        callback: DADiskMountCallback,
        context: *mut c_void,
    );

    /// Mounts the volume at the specified disk object, with the specified mount options.
    ///
    /// # Parameters
    ///
    /// * `disk`      - The disk object.
    /// * `path`      - The mount path. Pass NULL for a "standard" mount path.
    /// * `options`   - The mount options.
    /// * `callback`  - The callback function to call once the mount completes.
    /// * `context`   - The user-defined context parameter to pass to the callback function.
    /// * `arguments` - The null-terminated list of mount options to pass to `/sbin/mount -o`.
    pub fn DADiskMountWithArguments(
        disk: DADiskRef,
        path: CFURLRef,
        options: DADiskMountOptions,
        callback: DADiskMountCallback,
        context: *mut c_void,
        arguments: *mut CFStringRef,
    );

    /// Renames the volume at the specified disk object.
    ///
    /// # Parameters
    ///
    /// * `disk`     - The disk object.
    /// * `options`  - The rename options.
    /// * `callback` - The callback function to call once the rename completes.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DADiskRename(
        disk: DADiskRef,
        name: CFStringRef,
        options: DADiskRenameOptions,
        callback: DADiskRenameCallback,
        context: *mut c_void,
    );

    /// Unmounts the volume at the specified disk object.
    ///
    /// # Parameters
    ///
    /// * `disk`     - The disk object.
    /// * `options`  - The unmount options.
    /// * `callback` - The callback function to call once the unmount completes.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DADiskUnmount(
        disk: DADiskRef,
        options: DADiskUnmountOptions,
        callback: DADiskUnmountCallback,
        context: *mut c_void,
    );

    /// Claims the specified disk object for exclusive use.
    ///
    /// # Parameters
    ///
    /// * `disk`            - The disk object.
    /// * `options`         - The claim options.
    /// * `release`         - The callback function to call when the claim is to be released.
    /// * `releaseContext`  - The user-defined context parameter to pass to the callback function.
    /// * `callback`        - The callback function to call once the claim completes.
    /// * `callbackContext` - The user-defined context parameter to pass to the callback function.
    pub fn DADiskClaim(
        disk: DADiskRef,
        options: DADiskClaimOptions,
        release: DADiskClaimReleaseCallback,
        releaseContext: *mut c_void,
        callback: DADiskClaimCallback,
        callbackContext: *mut c_void,
    );

    /// Reports whether or not the disk is claimed.
    ///
    /// # Parameters
    ///
    /// * `disk` - The disk object.
    ///
    /// # Returns
    ///
    /// TRUE if the disk is claimed, otherwise FALSE.
    pub fn DADiskIsClaimed(disk: DADiskRef) -> Boolean;

    /// Unclaims the specified disk object.
    ///
    /// # Parameters
    ///
    /// * `disk` - The disk object.
    pub fn DADiskUnclaim(disk: DADiskRef);

    /// Obtains the options for the specified disk.
    ///
    /// # Parameters
    ///
    /// * `disk` - The disk object for which to obtain the options.
    ///
    /// # Returns
    ///
    /// The options.
    pub fn DADiskGetOptions(disk: DADiskRef) -> DADiskOptions;

    /// Sets the options for the specified disk.
    ///
    /// # Parameters
    ///
    /// * `disk`    - The disk object for which to set the options.
    /// * `options` - The options to set or clear.
    /// * `value`   - Pass TRUE to set options; otherwise pass FALSE to clear options.
    ///
    /// # Returns
    ///
    /// A result code.
    pub fn DADiskSetOptions(disk: DADiskRef, options: DADiskOptions, value: Boolean) -> DAReturn;

    /// Ejects the specified disk object.
    ///
    /// # Parameters
    ///
    /// * `disk`     - The disk object.
    /// * `options`  - The eject options.
    /// * `callback` - The callback function to call once the ejection completes.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DADiskEject(
        disk: DADiskRef,
        options: DADiskEjectOptions,
        callback: DADiskEjectCallback,
        context: *mut c_void,
    );
}

extern "C" {
    pub static kDADiskDescriptionVolumeKindKey: CFStringRef;
    pub static kDADiskDescriptionVolumeMountableKey: CFStringRef;
    pub static kDADiskDescriptionVolumeNameKey: CFStringRef;
    pub static kDADiskDescriptionVolumeNetworkKey: CFStringRef;
    pub static kDADiskDescriptionVolumePathKey: CFStringRef;
    pub static kDADiskDescriptionVolumeTypeKey: CFStringRef;
    pub static kDADiskDescriptionVolumeUUIDKey: CFStringRef;

    pub static kDADiskDescriptionMediaBSDMajorKey: CFStringRef;
    pub static kDADiskDescriptionMediaBSDMinorKey: CFStringRef;
    pub static kDADiskDescriptionMediaBSDNameKey: CFStringRef;
    pub static kDADiskDescriptionMediaBSDUnitKey: CFStringRef;
    pub static kDADiskDescriptionMediaBlockSizeKey: CFStringRef;
    pub static kDADiskDescriptionMediaContentKey: CFStringRef;
    pub static kDADiskDescriptionMediaEjectableKey: CFStringRef;
    pub static kDADiskDescriptionMediaIconKey: CFStringRef;
    pub static kDADiskDescriptionMediaKindKey: CFStringRef;
    pub static kDADiskDescriptionMediaLeafKey: CFStringRef;
    pub static kDADiskDescriptionMediaNameKey: CFStringRef;
    pub static kDADiskDescriptionMediaPathKey: CFStringRef;
    pub static kDADiskDescriptionMediaRemovableKey: CFStringRef;
    pub static kDADiskDescriptionMediaSizeKey: CFStringRef;
    pub static kDADiskDescriptionMediaTypeKey: CFStringRef;
    pub static kDADiskDescriptionMediaUUIDKey: CFStringRef;
    pub static kDADiskDescriptionMediaWholeKey: CFStringRef;
    pub static kDADiskDescriptionMediaWritableKey: CFStringRef;

    pub static kDADiskDescriptionDeviceGUIDKey: CFStringRef;
    pub static kDADiskDescriptionDeviceInternalKey: CFStringRef;
    pub static kDADiskDescriptionDeviceModelKey: CFStringRef;
    pub static kDADiskDescriptionDevicePathKey: CFStringRef;
    pub static kDADiskDescriptionDeviceProtocolKey: CFStringRef;
    pub static kDADiskDescriptionDeviceRevisionKey: CFStringRef;
    pub static kDADiskDescriptionDeviceUnitKey: CFStringRef;
    pub static kDADiskDescriptionDeviceVendorKey: CFStringRef;

    pub static kDADiskDescriptionBusNameKey: CFStringRef;
    pub static kDADiskDescriptionBusPathKey: CFStringRef;
}

#[cfg(feature = "macos_10_14_4_features")]
extern "C" {
    pub static kDADiskDescriptionDeviceTDMLockedKey: CFStringRef;
    pub static kDADiskDescriptionMediaEncryptedKey: CFStringRef;
    pub static kDADiskDescriptionMediaEncryptionDetailKey: CFStringRef;
}

extern "C" {
    /// Predefined CFDictionary object containing a set of disk description keys and values
    /// appropriate for matching unformatted media using DARegister*Callback.
    pub static mut kDADiskDescriptionMatchMediaUnformatted: CFDictionaryRef;

    /// Predefined CFDictionary object containing a set of disk description keys and values
    /// appropriate for matching whole media using DARegister*Callback.
    pub static mut kDADiskDescriptionMatchMediaWhole: CFDictionaryRef;

    /// Predefined CFDictionary object containing a set of disk description keys and values
    /// appropriate for matching mountable volumes using DARegister*Callback.
    pub static mut kDADiskDescriptionMatchVolumeMountable: CFDictionaryRef;

    /// Predefined CFDictionary object containing a set of disk description keys and values
    /// appropriate for matching unrecognized volumes using DARegister*Callback.
    pub static mut kDADiskDescriptionMatchVolumeUnrecognized: CFDictionaryRef;

    /// Predefined CFArray object containing a set of disk description keys appropriate for
    /// watching volume name changes using DARegisterDiskDescriptionChangedCallback.
    pub static mut kDADiskDescriptionWatchVolumeName: CFArrayRef;

    /// Predefined CFArray object containing a set of disk description keys appropriate for
    /// watching volume mount changes using DARegisterDiskDescriptionChangedCallback.
    pub static mut kDADiskDescriptionWatchVolumePath: CFArrayRef;
}
