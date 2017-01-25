use smallvec::SmallVec;
use inlinable_string::{InlinableString, StringExt};

use std::slice;

type Header = (InlinableString, InlinableString);

pub struct Headers {
    list: SmallVec<[Header; 8]>
}

pub struct ResponseHeadersIter<'res> {
    headers: slice::Iter<'res, Header>
}

impl<'res> Iterator for ResponseHeadersIter<'res> {
    type Item = (&'res str, &'res str);

    fn next(&mut self) -> Option<Self::Item> {
        self.headers.next().map(|&(ref a, ref b)| {
            (a.as_ref(), b.as_ref())
        })
    }
}

impl Headers {
    #[inline]
    pub fn new() -> Self {
        Headers {
            list: SmallVec::new()
        }
    }

    pub fn iter(&self) -> ResponseHeadersIter {
        ResponseHeadersIter {
            headers: self.list.iter()
        }
    }

    #[inline]
    pub fn add<S>(&mut self, name: S, value: S) where
        S: Into<InlinableString> + AsRef<str>
    {
        self.list.push((name.into(), value.into()));
    }

    #[inline]
    pub fn set<S>(&mut self, name: S, value: S) where
        S: Into<InlinableString> + AsRef<str>
    {
        for &mut(ref k, ref mut v) in self.list.iter_mut() {
            if k == name.as_ref() {
                *v = value.into();
                return;
            }
        }

        self.add(name, value);
    }

    #[inline]
    pub fn append<S>(&mut self, name: S, value: S) where
        S: Into<InlinableString> + AsRef<str>
    {
        for entry in self.list.iter_mut() {
            if entry.0 == name.as_ref() {
                entry.1.push_str("; ");
                entry.1.push_str(value.as_ref());
                break;
            }
        }

        self.add(name, value);
    }
}
