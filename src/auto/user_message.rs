// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_28", feature = "dox"))]
use gio;
#[cfg(any(feature = "v2_28", feature = "dox"))]
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_28", feature = "dox"))]
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use std::fmt;
use webkit2_sys;

glib_wrapper! {
    pub struct UserMessage(Object<webkit2_sys::WebKitUserMessage, webkit2_sys::WebKitUserMessageClass, UserMessageClass>);

    match fn {
        get_type => || webkit2_sys::webkit_user_message_get_type(),
    }
}

impl UserMessage {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn new(name: &str, parameters: Option<&glib::Variant>) -> UserMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_new(
                name.to_glib_none().0,
                parameters.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn with_fd_list<P: IsA<gio::UnixFDList>>(
        name: &str,
        parameters: Option<&glib::Variant>,
        fd_list: Option<&P>,
    ) -> UserMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_new_with_fd_list(
                name.to_glib_none().0,
                parameters.to_glib_none().0,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct UserMessageBuilder {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fd_list: Option<gio::UnixFDList>,
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    name: Option<String>,
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    parameters: Option<glib::Variant>,
}

impl UserMessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> UserMessage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_28", feature = "dox"))]
        {
            if let Some(ref fd_list) = self.fd_list {
                properties.push(("fd-list", fd_list));
            }
        }
        #[cfg(any(feature = "v2_28", feature = "dox"))]
        {
            if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
        }
        #[cfg(any(feature = "v2_28", feature = "dox"))]
        {
            if let Some(ref parameters) = self.parameters {
                properties.push(("parameters", parameters));
            }
        }
        let ret = glib::Object::new(UserMessage::static_type(), &properties)
            .expect("object new")
            .downcast::<UserMessage>()
            .expect("downcast");
        ret
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn fd_list<P: IsA<gio::UnixFDList>>(mut self, fd_list: &P) -> Self {
        self.fd_list = Some(fd_list.clone().upcast());
        self
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn parameters(mut self, parameters: &glib::Variant) -> Self {
        self.parameters = Some(parameters.clone());
        self
    }
}

pub const NONE_USER_MESSAGE: Option<&UserMessage> = None;

pub trait UserMessageExt: 'static {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_fd_list(&self) -> Option<gio::UnixFDList>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_name(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_parameters(&self) -> Option<glib::Variant>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn send_reply<P: IsA<UserMessage>>(&self, reply: &P);
}

impl<O: IsA<UserMessage>> UserMessageExt for O {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_fd_list(&self) -> Option<gio::UnixFDList> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_get_fd_list(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_parameters(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_get_parameters(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn send_reply<P: IsA<UserMessage>>(&self, reply: &P) {
        unsafe {
            webkit2_sys::webkit_user_message_send_reply(
                self.as_ref().to_glib_none().0,
                reply.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for UserMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserMessage")
    }
}
