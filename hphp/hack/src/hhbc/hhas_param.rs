// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
#![feature(rustc_private)]
use hhas_attribute_rust::HhasAttribute;
use hhas_type::{constraint, Info};
use label_rust::Label;
use oxidized::ast as tast;
extern crate bitflags;

#[derive(Clone)]
pub struct HhasParam {
    pub name: String,
    pub is_reference: bool,
    pub is_variadic: bool,
    pub is_inout: bool,
    pub user_attributes: Vec<HhasAttribute>,
    pub type_info: Option<Info>,
    pub default_value: Option<(Label, tast::Expr)>,
}

impl HhasParam {
    pub fn replace_default_value_label(&mut self, new_label: Label) {
        if let Some((label, _)) = self.default_value.as_mut() {
            *label = new_label;
        }
    }

    pub fn without_type(&mut self) {
        self.type_info.as_mut().map(|ti| {
            ti.type_constraint = constraint::Type::default();
        });
    }

    pub fn with_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn switch_inout_to_reference(&mut self) {
        if self.is_inout {
            self.is_inout = false;
            self.is_reference = true;
        }
    }

    pub fn switch_reference_to_inout(&mut self) {
        if self.is_reference {
            self.is_inout = true;
            self.is_reference = false;
        }
    }
}
