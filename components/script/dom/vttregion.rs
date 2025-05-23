/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::VTTRegionBinding::{ScrollSetting, VTTRegionMethods};
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::num::Finite;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object_with_proto};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct VTTRegion {
    reflector_: Reflector,
    id: DomRefCell<DOMString>,
    width: Cell<f64>,
    lines: Cell<u32>,
    region_anchor_x: Cell<f64>,
    region_anchor_y: Cell<f64>,
    viewport_anchor_x: Cell<f64>,
    viewport_anchor_y: Cell<f64>,
    scroll: Cell<ScrollSetting>,
}

impl VTTRegion {
    fn new_inherited() -> Self {
        VTTRegion {
            reflector_: Reflector::new(),
            id: DomRefCell::new(DOMString::default()),
            width: Cell::new(100_f64),
            lines: Cell::new(3),
            region_anchor_x: Cell::new(0_f64),
            region_anchor_y: Cell::new(100_f64),
            viewport_anchor_x: Cell::new(0_f64),
            viewport_anchor_y: Cell::new(100_f64),
            scroll: Cell::new(Default::default()),
        }
    }

    fn new(window: &Window, proto: Option<HandleObject>, can_gc: CanGc) -> DomRoot<Self> {
        reflect_dom_object_with_proto(Box::new(Self::new_inherited()), window, proto, can_gc)
    }
}

impl VTTRegionMethods<crate::DomTypeHolder> for VTTRegion {
    // https://w3c.github.io/webvtt/#dom-vttregion-vttregion
    fn Constructor(
        window: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<Self>> {
        Ok(VTTRegion::new(window, proto, can_gc))
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-id
    fn Id(&self) -> DOMString {
        self.id.borrow().clone()
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-id
    fn SetId(&self, value: DOMString) {
        *self.id.borrow_mut() = value;
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-width
    fn Width(&self) -> Finite<f64> {
        Finite::wrap(self.width.get())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-width
    fn SetWidth(&self, value: Finite<f64>) -> ErrorResult {
        if *value < 0_f64 || *value > 100_f64 {
            return Err(Error::IndexSize);
        }

        self.width.set(*value);
        Ok(())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-lines
    fn Lines(&self) -> u32 {
        self.lines.get()
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-lines
    fn SetLines(&self, value: u32) -> ErrorResult {
        self.lines.set(value);
        Ok(())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-regionanchorx
    fn RegionAnchorX(&self) -> Finite<f64> {
        Finite::wrap(self.region_anchor_x.get())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-regionanchorx
    fn SetRegionAnchorX(&self, value: Finite<f64>) -> ErrorResult {
        if *value < 0_f64 || *value > 100_f64 {
            return Err(Error::IndexSize);
        }

        self.region_anchor_x.set(*value);
        Ok(())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-regionanchory
    fn RegionAnchorY(&self) -> Finite<f64> {
        Finite::wrap(self.region_anchor_y.get())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-regionanchory
    fn SetRegionAnchorY(&self, value: Finite<f64>) -> ErrorResult {
        if *value < 0_f64 || *value > 100_f64 {
            return Err(Error::IndexSize);
        }

        self.region_anchor_y.set(*value);
        Ok(())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-viewportanchorx
    fn ViewportAnchorX(&self) -> Finite<f64> {
        Finite::wrap(self.viewport_anchor_x.get())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-viewportanchorx
    fn SetViewportAnchorX(&self, value: Finite<f64>) -> ErrorResult {
        if *value < 0_f64 || *value > 100_f64 {
            return Err(Error::IndexSize);
        }

        self.viewport_anchor_x.set(*value);
        Ok(())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-viewportanchory
    fn ViewportAnchorY(&self) -> Finite<f64> {
        Finite::wrap(self.viewport_anchor_y.get())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-viewportanchory
    fn SetViewportAnchorY(&self, value: Finite<f64>) -> ErrorResult {
        if *value < 0_f64 || *value > 100_f64 {
            return Err(Error::IndexSize);
        }

        self.viewport_anchor_y.set(*value);
        Ok(())
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-scroll
    fn Scroll(&self) -> ScrollSetting {
        self.scroll.get()
    }

    // https://w3c.github.io/webvtt/#dom-vttregion-scroll
    fn SetScroll(&self, value: ScrollSetting) {
        self.scroll.set(value);
    }
}
