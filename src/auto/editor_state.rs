// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct EditorState(Object<webkit2_sys::WebKitEditorState, webkit2_sys::WebKitEditorStateClass, EditorStateClass>);

    match fn {
        get_type => || webkit2_sys::webkit_editor_state_get_type(),
    }
}

pub const NONE_EDITOR_STATE: Option<&EditorState> = None;

pub trait EditorStateExt: 'static {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_typing_attributes(&self) -> u32;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_copy_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_cut_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_paste_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_redo_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_undo_available(&self) -> bool;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_typing_attributes_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<EditorState>> EditorStateExt for O {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_typing_attributes(&self) -> u32 {
        unsafe {
            webkit2_sys::webkit_editor_state_get_typing_attributes(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_copy_available(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_editor_state_is_copy_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_cut_available(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_editor_state_is_cut_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_paste_available(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_editor_state_is_paste_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_redo_available(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_editor_state_is_redo_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn is_undo_available(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_editor_state_is_undo_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_typing_attributes_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_typing_attributes_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitEditorState,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<EditorState>,
        {
            let f: &F = &*(f as *const F);
            f(&EditorState::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::typing-attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_typing_attributes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EditorState")
    }
}
