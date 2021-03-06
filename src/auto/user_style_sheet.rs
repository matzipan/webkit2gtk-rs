// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_6", feature = "dox"))]
use glib::translate::*;
use webkit2_sys;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserContentInjectedFrames;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserStyleLevel;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserStyleSheet(Shared<webkit2_sys::WebKitUserStyleSheet>);

    match fn {
        ref => |ptr| webkit2_sys::webkit_user_style_sheet_ref(ptr),
        unref => |ptr| webkit2_sys::webkit_user_style_sheet_unref(ptr),
        get_type => || webkit2_sys::webkit_user_style_sheet_get_type(),
    }
}

impl UserStyleSheet {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn new(
        source: &str,
        injected_frames: UserContentInjectedFrames,
        level: UserStyleLevel,
        allow_list: &[&str],
        block_list: &[&str],
    ) -> UserStyleSheet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_user_style_sheet_new(
                source.to_glib_none().0,
                injected_frames.to_glib(),
                level.to_glib(),
                allow_list.to_glib_none().0,
                block_list.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    pub fn new_for_world(
        source: &str,
        injected_frames: UserContentInjectedFrames,
        level: UserStyleLevel,
        world_name: &str,
        allow_list: &[&str],
        block_list: &[&str],
    ) -> UserStyleSheet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_user_style_sheet_new_for_world(
                source.to_glib_none().0,
                injected_frames.to_glib(),
                level.to_glib(),
                world_name.to_glib_none().0,
                allow_list.to_glib_none().0,
                block_list.to_glib_none().0,
            ))
        }
    }
}
