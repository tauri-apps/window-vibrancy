use objc2::{
    define_class, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObjectProtocol,
    sel, DefinedClass,
};
use objc2_app_kit::NSGlassEffectView;
use objc2_foundation::{NSInteger, NSRect};

/// NSGlassEffectViewTagged state.
/// Forced to be public by declare_class! macro.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct NSGlassEffectViewTaggedIvars {
    /// NSView tag to identify the view
    pub tag: NSInteger,
}

define_class!(
    /// A custom NSVisualEffectView subclass
    /// that overrides the tag method to provide a custom tag, to later identify the view
    #[unsafe(super(NSGlassEffectView))]
    #[ivars = NSGlassEffectViewTaggedIvars]
    pub struct NSGlassEffectViewTagged;

    impl NSGlassEffectViewTagged {
        #[unsafe(method(tag))]
        fn tag(&self) -> NSInteger {
            self.ivars().tag
        }
    }
);

#[allow(non_snake_case)]
impl NSGlassEffectViewTagged {
    /// # Safety
    ///
    /// This method is unsafe because it calls an Objective-C method.
    pub unsafe fn initWithFrame(
        this: Allocated<Self>,
        frame_rect: NSRect,
        tag: NSInteger,
    ) -> Retained<Self> {
        let state = NSGlassEffectViewTaggedIvars { tag };
        let this = this.set_ivars(state);

        msg_send![super(this), initWithFrame: frame_rect]
    }

    /// Whether the `effectIsInteractive` property is available on this system.
    ///
    /// The property was introduced in macOS 27.0.
    pub fn supportsEffectIsInteractive(&self) -> bool {
        self.respondsToSelector(sel!(setEffectIsInteractive:))
    }

    /// Whether interactive glass behavior is enabled, which adds a visual response to user interactions.
    ///
    /// <https://developer.apple.com/documentation/appkit/nsglasseffectview/effectisinteractive>
    ///
    /// Returns `false` on systems older than macOS 27.0, where the property does not exist.
    pub fn effectIsInteractive(&self) -> bool {
        if !self.supportsEffectIsInteractive() {
            return false;
        }

        // SAFETY: `effectIsInteractive` is declared as `@property BOOL effectIsInteractive`
        // in the macOS 27 SDK, and we checked that the receiver responds to it.
        unsafe { msg_send![self, effectIsInteractive] }
    }

    /// Enables interactive glass behavior, which adds a visual response to user interactions.
    ///
    /// <https://developer.apple.com/documentation/appkit/nsglasseffectview/effectisinteractive>
    ///
    /// The property is only available on macOS 27.0 or newer.
    ///
    /// # Returns
    ///
    /// - `true` if the property was set
    /// - `false` if the property is not available on this system (macOS 26), in which case this is a no-op.
    pub fn setEffectIsInteractive(&self, interactive: bool) -> bool {
        if !self.supportsEffectIsInteractive() {
            return false;
        }

        // SAFETY: `setEffectIsInteractive:` is the setter of `@property BOOL effectIsInteractive`
        // in the macOS 27 SDK, and we checked that the receiver responds to it.
        let () = unsafe { msg_send![self, setEffectIsInteractive: interactive] };
        true
    }
}
