// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::WebsiteDataTypes;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WebsiteData(Shared<ffi::WebKitWebsiteData>);

    match fn {
        ref => |ptr| ffi::webkit_website_data_ref(ptr),
        unref => |ptr| ffi::webkit_website_data_unref(ptr),
        get_type => || ffi::webkit_website_data_get_type(),
    }
}

impl WebsiteData {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_get_name")]
    pub fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_website_data_get_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_get_size")]
    pub fn get_size(&self, types: WebsiteDataTypes) -> u64 {
        unsafe { ffi::webkit_website_data_get_size(self.to_glib_none().0, types.to_glib()) }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_get_types")]
    pub fn get_types(&self) -> WebsiteDataTypes {
        unsafe { from_glib(ffi::webkit_website_data_get_types(self.to_glib_none().0)) }
    }
}
