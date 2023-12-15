use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DASession {
    _unused: [u8; 0],
}

/// Type of a reference to DASession instances.
pub type DASessionRef = *mut __DASession;
pub type DAApprovalSessionRef = *mut __DASession;

extern "C" {
    /// Returns the type identifier of all DASession instances.
    pub fn DASessionGetTypeID() -> CFTypeID;

    /// Creates a new session.
    ///
    /// # Returns
    ///
    /// A reference to a new DASession.
    ///
    /// # Discussion
    ///
    /// The caller of this function receives a reference to the returned object. The
    /// caller also implicitly retains the object and is responsible for releasing it.
    pub fn DASessionCreate(allocator: CFAllocatorRef) -> DASessionRef;

    /// Schedules the session on a run loop.
    ///
    /// # Parameters
    ///
    /// * `session`     - The session which is being scheduled.
    /// * `runLoop`     - The run loop on which the session should be scheduled.
    /// * `runLoopMode` - The run loop mode in which the session should be scheduled.
    pub fn DASessionScheduleWithRunLoop(
        session: DASessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    /// Unschedules the session from a run loop.
    ///
    /// # Parameters
    ///
    /// * `session`     - The session which is being unscheduled.
    /// * `runLoop`     - The run loop on which the session is scheduled.
    /// * `runLoopMode` - The run loop mode in which the session is scheduled.
    pub fn DASessionUnscheduleFromRunLoop(
        session: DASessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    /// Schedules the session on a dispatch queue.
    ///
    /// # Parameters
    ///
    /// * `session` - The session which is being scheduled.
    /// * `queue`   - The dispatch queue on which the session should be scheduled. Pass NULL to unschedule.
    pub fn DASessionSetDispatchQueue(session: DASessionRef, queue: dispatch_queue_t);

    /// Registers a callback function to be called whenever a volume is to be unmounted.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `match`    - The disk description keys to match. Pass NULL for all disk objects.
    /// * `callback` - The callback function to call when a volume is to be unmounted.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DARegisterDiskUnmountApprovalCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskUnmountApprovalCallback,
        context: *mut c_void,
    );

    /// Registers a callback function to be called whenever a volume is to be ejected.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `match`    - The disk description keys to match. Pass NULL for all disk objects.
    /// * `callback` - The callback function to call when a volume is to be ejected.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DARegisterDiskEjectApprovalCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskEjectApprovalCallback,
        context: *mut c_void,
    );

    /// Registers a callback function to be called whenever a disk has been probed.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `match`    - The disk description keys to match. Pass NULL for all disk objects.
    /// * `order`    - The callback order, from lowest to highest. Pass 0 for the default.
    /// * `callback` - The callback function to call when a disk has been probed.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DARegisterDiskPeekCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        order: CFIndex,
        callback: DADiskPeekCallback,
        context: *mut c_void,
    );

    /// Registers a callback function to be called whenever a volume is to be mounted.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `match`    - The disk description keys to match. Pass NULL for all disk objects.
    /// * `callback` - The callback function to call when a volume is to be mounted.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DARegisterDiskMountApprovalCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskMountApprovalCallback,
        context: *mut c_void,
    );

    /// Registers a callback function to be called whenever a disk has disappeared.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `match`    - The disk description keys to match. Pass NULL for all disk objects.
    /// * `callback` - The callback function to call when a disk has disappeared.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DARegisterDiskDisappearedCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskDisappearedCallback,
        context: *mut c_void,
    );

    /// Registers a callback function to be called whenever a disk has appeared.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `match`    - The disk description keys to match. Pass NULL for all disk objects.
    /// * `callback` - The callback function to call when a disk has appeared.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DARegisterDiskAppearedCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskAppearedCallback,
        context: *mut c_void,
    );

    /// Registers a callback function to be called whenever a disk description has changed.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `match`    - The disk description keys to match. Pass NULL for all disk objects.
    /// * `watch`    - The disk description keys to watch. Pass NULL for all keys.
    /// * `callback` - The callback function to call when a watched key changes.
    /// * `context`  - The user-defined context parameter to pass to the callback function.
    pub fn DARegisterDiskDescriptionChangedCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        watch: CFArrayRef,
        callback: DADiskDescriptionChangedCallback,
        context: *mut c_void,
    );

    /// Unregisters a registered callback function.
    ///
    /// # Parameters
    ///
    /// * `session`  - The session object.
    /// * `callback` - The registered callback function.
    /// * `context`  - The user-defined context parameter.
    pub fn DAUnregisterCallback(session: DASessionRef, callback: *mut c_void, context: *mut c_void);
}

extern "C" {
    pub fn DAApprovalSessionGetTypeID() -> CFTypeID;
    pub fn DAApprovalSessionCreate(allocator: CFAllocatorRef) -> DAApprovalSessionRef;
    pub fn DAApprovalSessionScheduleWithRunLoop(
        session: DAApprovalSessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
    pub fn DAApprovalSessionUnscheduleFromRunLoop(
        session: DAApprovalSessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
    pub fn DAUnregisterApprovalCallback(
        session: DASessionRef,
        callback: *mut c_void,
        context: *mut c_void,
    );
}
