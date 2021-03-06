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

#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

use gtk::{Inhibit, WidgetExt};
use relm::Widget;
use relm_attributes::widget;

use self::Msg::*;

#[derive(Msg)]
pub enum LabelMsg {
}

#[derive(Clone)]
pub struct LabelModel {
    counter: i32,
}

#[widget]
impl Widget for Label {
    fn init_view(&self) {
        self.label.set_text("Test");
    }

    fn model() -> LabelModel {
        LabelModel {
            counter: 0,
        }
    }

    fn update(&mut self, _event: LabelMsg, _model: &mut LabelModel) {
        self.label.set_text("");
    }

    view! {
        #[name="label"]
        gtk::Label {
            text: &model.counter.to_string(),
            visible: false,
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> () {
    }

    fn update(&mut self, event: Msg, _model: &mut ()) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            Label,
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
