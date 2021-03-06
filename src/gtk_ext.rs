/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! This module contains methods to set the child properties of a `gtk::Box`.

// TODO: remove this file when the next gtk-rs version release.

use glib::translate::{ToGlib, ToGlibPtr};
use gtk;
use gtk::{IsA, PackType, Value, Widget};
use gtk_sys;

/// A trait providing methods to set the child properties of a `gtk::Box`.
pub trait BoxExtManual {
    /// Set whether the child should receive extra space when the parent grows.
    fn set_child_expand<T: IsA<Widget>>(&self, child: &T, expand: bool);

    /// Set whether the child should receive extra space when the parent grows.
    fn set_child_fill<T: IsA<Widget>>(&self, child: &T, fill: bool);

    /// Set whether the child is packed with reference to the start or end of the parent.
    fn set_child_pack_type<T: IsA<Widget>>(&self, child: &T, pack_type: PackType);

    /// Set the extra space to put between the child and its neighbors, in pixels.
    fn set_child_padding<T: IsA<Widget>>(&self, child: &T, padding: u32);

    /// Set the index of the child in the parent.
    fn set_child_position<T: IsA<Widget>>(&self, child: &T, position: i32);
}

impl BoxExtManual for gtk::Box {
    fn set_child_expand<T: IsA<Widget>>(&self, child: &T, expand: bool) {
        let expand = Value::from(&expand);
        unsafe { gtk_sys::gtk_container_child_set_property(self.to_glib_none().0, child.to_glib_none().0,
            "expand".to_glib_none().0, expand.to_glib_none().0) }
    }

    fn set_child_fill<T: IsA<Widget>>(&self, child: &T, fill: bool) {
        let fill = Value::from(&fill);
        unsafe { gtk_sys::gtk_container_child_set_property(self.to_glib_none().0, child.to_glib_none().0,
            "fill".to_glib_none().0, fill.to_glib_none().0) }
    }

    fn set_child_pack_type<T: IsA<Widget>>(&self, child: &T, pack_type: PackType) {
        let pack_type = Value::from(&(pack_type.to_glib() as i32));
        unsafe { gtk_sys::gtk_container_child_set_property(self.to_glib_none().0, child.to_glib_none().0,
            "pack-type".to_glib_none().0, pack_type.to_glib_none().0) }
    }

    fn set_child_padding<T: IsA<Widget>>(&self, child: &T, padding: u32) {
        let padding = Value::from(&padding);
        unsafe { gtk_sys::gtk_container_child_set_property(self.to_glib_none().0, child.to_glib_none().0,
            "padding".to_glib_none().0, padding.to_glib_none().0) }
    }

    fn set_child_position<T: IsA<Widget>>(&self, child: &T, position: i32) {
        let position = Value::from(&position);
        unsafe { gtk_sys::gtk_container_child_set_property(self.to_glib_none().0, child.to_glib_none().0,
            "position".to_glib_none().0, position.to_glib_none().0) }
    }
}
