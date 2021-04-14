use syn::{Ident, Lit, Meta, MetaNameValue, NestedMeta, Path};

pub trait ExtMeta {
    fn ident(&self) -> Option<&Ident>;
    fn get_path(&self) -> Option<&Path>;
    fn single_list(&self) -> Option<&Path>;
    fn name_value(&self) -> Option<&MetaNameValue>;
}

impl ExtMeta for Meta {
    fn ident(&self) -> Option<&Ident> {
        self.get_path()?.get_ident()
    }

    fn get_path(&self) -> Option<&Path> {
        match self {
            Self::Path(path) => Some(path),
            _ => None,
        }
    }

    fn single_list(&self) -> Option<&Path> {
        match self {
            Self::List(list) => list.nested.iter().single()?.meta()?.get_path(),
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

pub trait ExtNestedMeta {
    fn meta(&self) -> Option<&Meta>;
}

impl ExtNestedMeta for NestedMeta {
    fn meta(&self) -> Option<&Meta> {
        match self {
            Self::Meta(meta) => Some(meta),
            _ => None,
        }
    }
}

trait Single {
    type Item;
    fn single(self) -> Option<Self::Item>;
}

impl<I> Single for I
where
    I: Iterator,
{
    type Item = I::Item;
    fn single(mut self) -> Option<Self::Item> {
        let item = self.next()?;
        if self.next().is_none() {
            Some(item)
        } else {
            None
        }
    }
}
