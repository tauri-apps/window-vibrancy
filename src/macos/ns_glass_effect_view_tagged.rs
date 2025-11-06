use objc2::{
    define_class, msg_send,
    rc::{Allocated, Retained},
    DefinedClass,
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
    #[name = "NSGlassEffectViewTagged"]
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
}
