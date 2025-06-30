#![no_main]
use std::{os::raw::*,ptr,mem};
use x11::xlib::*;
use maflow::*;

macro_rules! cstr {
    ($txt:literal) => {std::ffi::CString::new($txt).unwrap().as_ptr()};
    (mut $txt:literal) => {cstr!($txt) as *mut c_char};
}

#[unsafe(no_mangle)]
pub fn main(_:i32,_:*const*const u8){unsafe{

    let display = XOpenDisplay(ptr::null());
    kill!{if display.is_null()}

    let screen = XDefaultScreen(display);
    let root = XRootWindow(display, screen);

    #[allow(invalid_value)]
    let mut attributes: XSetWindowAttributes = mem::MaybeUninit::uninit().assume_init();
    attributes.background_pixel = XBlackPixel(display, screen);

    let window = XCreateWindow(
        display, root, 
        0, 0, 100, 100, 0, 0,
        InputOutput as c_uint,
        ptr::null_mut(),
        CWBackPixel,
        &mut attributes,
    );

    XStoreName( display, window, cstr!(mut "nothing") );

    let wm_protocols = XInternAtom( display, cstr!("WM_PROTOCOLS"), False );
    let wm_delete_window = XInternAtom( display, cstr!("WM_DELETE_WINDOW"), False );
    let mut protocols = [wm_delete_window];

    XSetWMProtocols(
        display, window,
        protocols.as_mut_ptr(),
        protocols.len() as c_int,
    );

    XMapWindow(display,window);

    let mut event: XEvent = mem::MaybeUninit::uninit().assume_init();
    loop {
        XNextEvent(display, &mut event);
        next!{if ClientMessage != event.get_type()}
        let xclient = XClientMessageEvent::from(event);
        next!{if xclient.message_type != wm_protocols || xclient.format != 32}
        let protocol = xclient.data.get_long(0) as Atom;
        hold!{if protocol == wm_delete_window}
    }

    XCloseDisplay(display);
}}
