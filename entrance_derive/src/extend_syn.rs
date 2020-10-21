use syn::{Ident, Lit, Meta, MetaNameValue};

pub trait ExtMeta {
    fn ident(&self) -> Option<&Ident>;
    fn name_value(&self) -> Option<&MetaNameValue>;
}

impl ExtMeta for Meta {
    fn ident(&self) -> Option<&Ident> {
        match self {
            Self::Path(path) => Some(path.get_ident()?),
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
