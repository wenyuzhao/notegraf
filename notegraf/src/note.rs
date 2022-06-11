//! Core types of Notegraf.
use crate::notetype::NoteType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{self, Display};
use crate::notemetadata::NoteMetadata;

/// ID of notes.
///
/// In a given note store ([`crate::notestore`]),
/// [`NoteID`] should uniquely identify a note,
/// which can have different revisions ([`Revision`]).
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Hash)]
#[serde(into = "String", from = "String")]
pub struct NoteID {
    id: String,
}

impl From<NoteID> for String {
    fn from(id: NoteID) -> String {
        id.id
    }
}

impl From<String> for NoteID {
    fn from(id: String) -> NoteID {
        NoteID::new(id)
    }
}

impl From<&str> for NoteID {
    fn from(id: &str) -> NoteID {
        NoteID::new(id.to_owned())
    }
}

impl NoteID {
    pub fn new(id: String) -> Self {
        NoteID { id }
    }
}

impl Display for NoteID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl AsRef<str> for NoteID {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub struct Revision {
    revision: String,
}

impl Revision {
    pub fn new(revision: String) -> Self {
        Revision { revision }
    }
}

impl Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.revision)
    }
}

impl AsRef<str> for Revision {
    fn as_ref(&self) -> &str {
        &self.revision
    }
}

impl From<Revision> for String {
    fn from(revision: Revision) -> String {
        revision.revision
    }
}

impl From<String> for Revision {
    fn from(revision: String) -> Revision {
        Revision::new(revision)
    }
}

impl From<&str> for Revision {
    fn from(revision: &str) -> Revision {
        Revision::new(revision.to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Note<T> {
    pub note_inner: T,
    pub id: NoteID,
    pub revision: Revision,
    pub parent: Option<NoteID>,
    pub children: HashSet<NoteID>,
    pub metadata: NoteMetadata
}

impl<T> Note<T>
where
    T: NoteType,
{
    pub fn new(note_inner: T, id: NoteID, revision: Revision, parent: Option<NoteID>) -> Self {
        Note {
            note_inner,
            id,
            revision,
            parent,
            children: HashSet::new(),
            metadata: NoteMetadata::default()
        }
    }

    pub fn get_references(&self) -> Vec<&NoteID> {
        self.note_inner.get_references()
    }
}

/// A type for locating a note.
#[derive(Debug, Serialize, Deserialize)]
pub enum NoteLocator {
    Current(NoteID),
    Specific(NoteID, Revision),
}

impl NoteLocator {
    /// Get ID of the locator.
    pub fn get_id(&self) -> &NoteID {
        match self {
            NoteLocator::Current(id) => id,
            NoteLocator::Specific(id, _) => id,
        }
    }

    /// Get revision of the locator.
    ///
    /// If the locator is not specifying a revision, returns `None`.
    pub fn get_revision(&self) -> Option<&Revision> {
        if let NoteLocator::Specific(_, rev) = self {
            Some(rev)
        } else {
            None
        }
    }

    /// Return a ([`NoteID`], [`Revision`]) tuple.
    pub fn unpack(&self) -> (&NoteID, Option<&Revision>) {
        (self.get_id(), self.get_revision())
    }

    /// Get a locator to point at a specific revision of the same note.
    pub fn at_revision(&self, r: &Revision) -> Self {
        NoteLocator::Specific(self.get_id().clone(), r.clone())
    }

    /// Get a locator to point at the current revision of the same note.
    pub fn current(&self) -> Self {
        NoteLocator::Current(self.get_id().clone())
    }
}
