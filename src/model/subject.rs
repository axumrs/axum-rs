use serde::{Deserialize, Serialize};

use crate::db::Paginate;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone)]
#[repr(u8)]
pub enum SubjectStatus {
    #[default]
    Writing = 0,
    Finished = 1,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
pub struct Subject {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub is_del: bool,
    pub cover: String,
    pub status: SubjectStatus,
    pub price: u32,
    pub pin: u8,
}

impl From<&Subject> for Subject {
    fn from(s: &Subject) -> Self {
        Self {
            id: s.id,
            name: s.name.clone(),
            slug: s.slug.clone(),
            summary: s.summary.clone(),
            is_del: s.is_del,
            cover: s.cover.clone(),
            status: s.status.clone(),
            price: s.price,
            pin: s.pin,
        }
    }
}

impl From<super::UserPurchasedSubject> for Subject {
    fn from(ups: super::UserPurchasedSubject) -> Self {
        Self {
            id: ups.id,
            name: ups.name,
            slug: ups.slug,
            summary: ups.summary,
            is_del: ups.is_del,
            cover: ups.cover,
            status: ups.status,
            price: ups.price,
            pin: 0,
        }
    }
}
impl From<&super::UserPurchasedSubject> for Subject {
    fn from(ups: &super::UserPurchasedSubject) -> Self {
        Self {
            id: ups.id,
            name: ups.name.clone(),
            slug: ups.slug.clone(),
            summary: ups.summary.clone(),
            is_del: ups.is_del,
            cover: ups.cover.clone(),
            status: ups.status.clone(),
            price: ups.price,
            pin: 0,
        }
    }
}

pub enum SubjectFindBy<'a> {
    ID(u32),
    Slug(&'a str),
}

#[derive(Default)]
pub struct SubjectListWith {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub status: Option<SubjectStatus>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Serialize)]
pub struct SubjectIfPurchased {
    pub subject: Subject,
    pub is_purchased: bool,
}

impl SubjectIfPurchased {
    pub fn new(subject: Subject, is_purchased: bool) -> Self {
        Self {
            subject,
            is_purchased,
        }
    }
}

impl From<super::UserPurchasedSubject> for SubjectIfPurchased {
    fn from(value: super::UserPurchasedSubject) -> Self {
        Self {
            subject: value.into(),
            is_purchased: true,
        }
    }
}
impl From<&super::UserPurchasedSubject> for SubjectIfPurchased {
    fn from(value: &super::UserPurchasedSubject) -> Self {
        Self {
            subject: value.into(),
            is_purchased: true,
        }
    }
}

impl From<Subject> for SubjectIfPurchased {
    fn from(subject: Subject) -> Self {
        Self {
            subject,
            is_purchased: false,
        }
    }
}
impl From<&Subject> for SubjectIfPurchased {
    fn from(subject: &Subject) -> Self {
        Self {
            subject: subject.into(),
            is_purchased: false,
        }
    }
}

impl From<Paginate<Subject>> for Paginate<SubjectIfPurchased> {
    fn from(p: Paginate<Subject>) -> Self {
        let mut vs = Vec::with_capacity(p.data.len());
        for s in p.data.into_iter() {
            vs.push(s.into());
        }
        Paginate::new(p.total, p.page, p.page_size, vs)
    }
}
impl From<Paginate<super::UserPurchasedSubject>> for Paginate<SubjectIfPurchased> {
    fn from(p: Paginate<super::UserPurchasedSubject>) -> Self {
        let mut vs = Vec::with_capacity(p.data.len());
        for s in p.data.into_iter() {
            vs.push(s.into());
        }
        Paginate::new(p.total, p.page, p.page_size, vs)
    }
}
