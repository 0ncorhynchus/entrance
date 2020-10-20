use syn::{Ident, Lit, Meta, MetaNameValue};

pub trait ExtMeta {
    fn word(&self) -> Option<&Ident>;
    fn name_value(&self) -> Option<&MetaNameValue>;
}

impl ExtMeta for Meta {
    fn word(&self) -> Option<&Ident> {
        match self {
            Self::Word(ident) => Some(ident),
            _ => None,
        }
    }

    fn name_value(&self) -> Option<&MetaNameValue> {
        match self {
            Self::NameValue(name_value) => Some(name_value),
            _ => None,
        }
    }
}

pub trait ExtLit {
    fn char(&self) -> Option<char>;
    fn str(&self) -> Option<String>;
}

impl ExtLit for Lit {
    fn char(&self) -> Option<char> {
        match self {
            Self::Char(lit) => Some(lit.value()),
            _ => None,
        }
    }

    fn str(&self) -> Option<String> {
        match self {
            Self::Str(lit) => Some(lit.value()),
            _ => None,
        }
    }
}
