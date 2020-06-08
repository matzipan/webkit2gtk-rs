// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
#[cfg(any(feature = "v2_20", feature = "dox"))]
use std::ptr;
use webkit2_sys;

glib_wrapper! {
    pub struct FormSubmissionRequest(Object<webkit2_sys::WebKitFormSubmissionRequest, webkit2_sys::WebKitFormSubmissionRequestClass, FormSubmissionRequestClass>);

    match fn {
        get_type => || webkit2_sys::webkit_form_submission_request_get_type(),
    }
}

pub const NONE_FORM_SUBMISSION_REQUEST: Option<&FormSubmissionRequest> = None;

pub trait FormSubmissionRequestExt: 'static {
    //#[cfg_attr(feature = "v2_20", deprecated)]
    //fn get_text_fields(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 };

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn list_text_fields(&self) -> Option<(Vec<GString>, Vec<GString>)>;

    fn submit(&self);
}

impl<O: IsA<FormSubmissionRequest>> FormSubmissionRequestExt for O {
    //fn get_text_fields(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 } {
    //    unsafe { TODO: call webkit2_sys:webkit_form_submission_request_get_text_fields() }
    //}

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    fn list_text_fields(&self) -> Option<(Vec<GString>, Vec<GString>)> {
        unsafe {
            let mut field_names = ptr::null_mut();
            let mut field_values = ptr::null_mut();
            let ret = from_glib(
                webkit2_sys::webkit_form_submission_request_list_text_fields(
                    self.as_ref().to_glib_none().0,
                    &mut field_names,
                    &mut field_values,
                ),
            );
            if ret {
                Some((
                    FromGlibPtrContainer::from_glib_none(field_names),
                    FromGlibPtrContainer::from_glib_none(field_values),
                ))
            } else {
                None
            }
        }
    }

    fn submit(&self) {
        unsafe {
            webkit2_sys::webkit_form_submission_request_submit(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for FormSubmissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FormSubmissionRequest")
    }
}
