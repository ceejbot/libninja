use crate::{Doc, Function, Ident, Import, Item, Visibility};
use proc_macro2::TokenStream;
use std::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct Class<T> {
    pub vis: Visibility,
    pub name: Ident,
    pub doc: Option<Doc>,
    pub fields: Vec<Field<T>>,
    pub methods: Vec<Function<T>>,
    pub attributes: Vec<T>,
    pub lifetimes: Vec<String>,
    pub items: Vec<Item<TokenStream>>,
    pub imports: Vec<Import>,
}

#[derive(Debug, Default)]
pub struct Field<T> {
    pub name: Ident,
    pub ty: T,
    pub default: Option<T>,
    pub vis: Visibility,
    pub doc: Option<Doc>,
    pub optional: bool,
    pub attributes: Vec<T>,
}

impl Debug for Class<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name)
            .field("doc", &self.doc)
            .field("fields", &self.fields)
            .field("vis", &self.vis)
            .finish()
    }
}
