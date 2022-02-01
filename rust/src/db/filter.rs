use std::any::Any;
use bson::{doc, Document};
use serde::Serialize;

// FILTER
pub struct DatabaseFilter<T> {
    key: String,
    filter: DatabaseFilterOpt<T>
}

pub enum DatabaseFilterOpt<T> {
    Cmp(DatabaseFilterCmp<T>),
    Ord(DatabaseFilterOrd<T>),
    OrdCmp(DatabaseFilterOrdCmp<T>)
}

pub enum DatabaseFilterCmp<T> {
    Eq(T),
    Ne(T)
}

pub enum DatabaseFilterOrd<T> {
    Lt(T),
    Gt(T)
}

pub enum DatabaseFilterOrdCmp<T> {
    Lte(T),
    Gte(T)
}

// EVAL
impl<T: PartialEq> DatabaseFilterCmp<T> {
    pub fn eval (&self, value: &T) -> bool {
        match self {
            Self::Eq(x) => value == x,
            Self::Ne(x) => value != x,
        }
    }
}

impl<T: PartialOrd> DatabaseFilterOrd<T> {
    pub fn eval (&self, value: &T) -> bool {
        match self {
            Self::Gt(x) => value > x,
            Self::Lt(x) => value < x,
        }
    }
}

impl<T: PartialEq + PartialOrd> DatabaseFilterOrdCmp<T> {
    pub fn eval (&self, value: &T) -> bool {
        match self {
            Self::Gte(x) => value >= x,
            Self::Lte(x) => value <= x,
        }
    }
}

impl<T> DatabaseFilterOpt<T>  {
    pub fn eval (&self, value: &T) -> bool {
        match self {
            Self::Cmp(cmp) => cmp.eval(value),
            Self::Ord(ord) => ord.eval(value),
            Self::OrdCmp(ord_cmp) => ord_cmp.eval(value)
        }
    }
}

impl<T> DatabaseFilter<T>  {
    pub fn eval (&self, value: &T) -> bool {
        self.filter.eval(value)
    }
}

// SERIALIZE
impl<T: Serialize> Into<Document> for DatabaseFilterCmp<T> {
    fn into (self) -> Document {
        match self {
            Self::Eq(x) => doc! { "$eq": bson::to_bson(&x).unwrap() },
            Self::Ne(x) => doc! { "$ne": bson::to_bson(&x).unwrap() },
        }
    }
}

impl<T: Serialize> Into<Document> for DatabaseFilterOrd<T> {
    fn into (self) -> Document {
        match self {
            Self::Gt(x) => doc! { "$gt": bson::to_bson(&x).unwrap() },
            Self::Lt(x) => doc! { "$lt": bson::to_bson(&x).unwrap() },
        }
    }
}

impl<T: Serialize> Into<Document> for DatabaseFilterOrdCmp<T> {
    fn into (self) -> Document {
        match self {
            Self::Gte(x) => doc! { "$gte": bson::to_bson(&x).unwrap() },
            Self::Lte(x) => doc! { "$lte": bson::to_bson(&x).unwrap() },
        }
    }
}

impl<T: Serialize> Into<Document> for DatabaseFilterOpt<T> {
    fn into(self) -> Document {
        match self {
            Self::Cmp(cmp) => cmp.into(),
            Self::Ord(ord) => ord.into(),
            Self::OrdCmp(ord_cmp) => ord_cmp.into()
        }
    }
}

impl<T: Serialize> Into<Document> for DatabaseFilter<T> {
    fn into(self) -> Document {
        doc! { self.key: Into::<Document>::into(self.filter) }
    }
}