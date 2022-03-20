use std::sync::Arc;

use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct Binder {
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub visibility: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub private: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub public: Option<Arc<bool>>,
    pub notes: Arc<Notes>,
    pub binder: Arc<Vec<Arc<SlipBox>>>,
    #[builder(default = "None", setter(into))]
    pub identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub ownership: Option<Arc<Ownership>>,
}
/// !Default
/// @!dez/lulz/*programming*
/// #math=hard,tutorial,programming
/// $DSP

#[derive(Debug, Clone, Builder)]
pub struct Notes {
    pub hash: Arc<String>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub visibility: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub private: Option<Arc<bool>>,
    #[builder(default = "None", setter(into))]
    pub public: Option<Arc<bool>>,
    #[builder(default = "None", setter(into))]
    pub file: Option<Arc<String>>,
    pub note: Arc<Note>,
    pub notes: Arc<Vec<Arc<Note>>>,
    #[builder(default = "None", setter(into))]
    pub text: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub line: Option<Arc<Line>>,
    #[builder(default = "None", setter(into))]
    pub lines: Option<Arc<Lines>>,
    #[builder(default = "None", setter(into))]
    pub verifications: Option<Arc<Verifications>>,
    #[builder(default = "None", setter(into))]
    pub groups: Option<Arc<Groups>>,
    #[builder(default = "None", setter(into))]
    pub local_identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub remote_identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub author_data: Option<Arc<Vec<Note>>>,
    #[builder(default = "None", setter(into))]
    pub backlinks: Option<Arc<Citations>>,
    #[builder(default = "None", setter(into))]
    pub backlinks_identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub cite: Option<Arc<Citation>>,
    #[builder(default = "None", setter(into))]
    pub cite_identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub citation: Option<Arc<Citation>>,
    #[builder(default = "None", setter(into))]
    pub citation_identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub citations: Option<Arc<Citations>>,
    #[builder(default = "None", setter(into))]
    pub citations_identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
}
/// all repeating whitespace is replaced with equivalent newline, tab, or space
/// certain contexts consume whitespace as a delimiter
/// the only circumvention is to escape each repeating character with '\'
/// or by using rust-style r#"..."# raw strings, which will never be modified
#[derive(Debug, Clone, Builder)]
pub struct SlipBox {
    pub hash: Arc<String>,
    pub notes: Arc<Notes>,
    pub slipbox: Arc<Vec<Arc<Notes>>>,
    #[builder(default = "None", setter(into))]
    pub identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub visibility: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub private: Option<Arc<bool>>,
    #[builder(default = "None", setter(into))]
    pub public: Option<Arc<bool>>,
    #[builder(default = "None", setter(into))]
    pub ownership: Option<Arc<Ownership>>,
}

#[derive(Debug, Clone, Builder)]
/// I was promised a web and I will get one even if I have to make it myself.
pub struct Identifier {
    pub name: Arc<String>,
    /// by convention, the wiki/true name hashed alone is the canonical hash for meta representation of figures who do not have a hash (Such as Michaelangelo, Prince, Michelangelo (Teenage Mutant Ninja Turtles), Ged, Aragorn son of Arathorn, and so on.)
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub aliases: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub hash_aliases: Option<Arc<Categories>>,
    #[builder(default = "None", setter(into))]
    pub file: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub prefix: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub prefix_aliases: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub prefix_hash: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub prefix_hash_aliases: Option<Arc<Categories>>,
    #[builder(default = "None", setter(into))]
    pub root: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub remote_root: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub remote_tap_root: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub path: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub url: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub tags: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub categories: Option<Arc<Categories>>,
    #[builder(default = "None", setter(into))]
    pub comments: Option<Arc<Comments>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
    #[builder(default = "None", setter(into))]
    pub backlinks: Option<Arc<Citations>>,
    #[builder(default = "None", setter(into))]
    pub references: Option<Arc<Citations>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Ownership {
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub author: Option<Arc<Author>>,
    #[builder(default = "None", setter(into))]
    pub authors: Option<Arc<Authors>>,
    #[builder(default = "None", setter(into))]
    pub group: Option<Arc<Group>>,
    #[builder(default = "None", setter(into))]
    pub groups: Option<Arc<Groups>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Note {
    pub lines: Lines,
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub verifications: Option<Arc<Verifications>>,
}
#[derive(Debug, Clone, Builder)]
pub struct Group {
    pub authors: Arc<Authors>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
    #[builder(default = "None", setter(into))]
    pub bio: Option<Arc<Vec<Notes>>>,
    #[builder(default = "None", setter(into))]
    pub notes: Option<Arc<Vec<Notes>>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Authors {
    pub hash: Arc<String>,
    pub authors: Arc<Vec<Arc<Author>>>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Author {
    pub identifier: Arc<Identifier>,
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
    #[builder(default = "None", setter(into))]
    pub bio: Option<Arc<Notes>>,
    #[builder(default = "None", setter(into))]
    pub notes: Option<Arc<Notes>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Description {
    pub identifier: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub identifier_aliases: Option<Arc<Vec<Tags>>>,
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub hash_aliases: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub about: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub contact: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub username: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub handle: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub aliases: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub links: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub avatar: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub location: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub website: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub social: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub tags: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub categories: Option<Arc<Categories>>,
    #[builder(default = "None", setter(into))]
    pub references: Option<Arc<Citations>>,
    pub references_identifier: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub references_identifier_aliases: Option<Arc<Vec<Tags>>>,
    pub references_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub references_hash_aliases: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub backlinks: Option<Arc<Citations>>,
    pub backlinks_identifier: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub backlinks_identifier_aliases: Option<Arc<Vec<Tags>>>,
    pub backlinks_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub backlinks_hash_aliases: Option<Arc<Vec<Tags>>>,
    pub verification_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub verification_hash_aliases: Option<Arc<Vec<Tags>>>,
    pub verifications_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub verifications_hash_aliases: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub created_at: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub updated_at: Option<Arc<String>>,
    pub updated_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub updated_by: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub updated_note: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub updated_reason: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub updated_code: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub deleted_at: Option<Arc<String>>,
    pub deleted_by_verification_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub deleted_by_verification_hash_aliases: Option<Arc<Vec<Tags>>>,
    pub deleted_by_verifications_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub deleted_by_verifications_hash_aliases: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub deleted_note: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub deleted_reason: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub deleted_code: Option<Arc<String>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Line {
    pub zk: Arc<ZK>,
    #[builder(default = "None", setter(into))]
    pub comments: Option<Arc<Comments>>,
    pub citation_identifier: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub citation_identifier_aliases: Option<Arc<Vec<Tags>>>,
    pub citation_hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub citation_hash_aliases: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub citations: Option<Arc<Citations>>,
    #[builder(default = "None", setter(into))]
    pub citations_identifier: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub citations_identifier_aliases: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub citations_hash: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub citations_hash_aliases: Option<Arc<Vec<Tags>>>,
    #[builder(default = "None", setter(into))]
    pub signatures: Option<Arc<Signatures>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Lines {
    pub hash: Arc<String>,
    pub lines: Arc<Vec<Arc<Line>>>,
    #[builder(default = "None", setter(into))]
    pub line: Option<Arc<Line>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Verifications {
    pub hash: Arc<String>,
    pub lines: Arc<Lines>,
    pub verifications: Arc<Vec<Arc<Verification>>>,
    #[builder(default = "None", setter(into))]
    pub identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
    #[builder(default = "None", setter(into))]
    pub verification: Option<Arc<Verification>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Verification {
    pub verified: bool,
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub kind: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub status: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub by: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub at: Option<Arc<String>>,
    pub lines: Arc<Lines>,
    #[builder(default = "None", setter(into))]
    pub categories: Option<Arc<Categories>>,
    #[builder(default = "None", setter(into))]
    pub tags: Option<Arc<Tags>>,
    pub identities: Arc<Identities>,
    #[builder(default = "None", setter(into))]
    pub identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Identities {
    pub hash: Arc<String>,
    pub identities: Arc<Vec<Arc<Identity>>>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Groups {
    pub hash: Arc<String>,
    pub groups: Arc<Vec<Arc<Group>>>,
    #[builder(default = "None", setter(into))]
    pub identifier: Option<Arc<Identifier>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Citations {
    pub citations: Arc<Vec<Arc<Citation>>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Citation {
    #[builder(default = "None", setter(into))]
    pub zk: Option<Arc<ZK>>,
    #[builder(default = "None", setter(into))]
    pub format: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub value: Option<Arc<String>>,
    pub hash: Arc<String>,
    // underscore where user input stopped and auto resolution started
    #[builder(default = "None", setter(into))]
    pub identity: Option<Arc<Identity>>,
    #[builder(default = "None", setter(into))]
    pub resoived_identity: Option<Arc<Identity>>,
    #[builder(default = "None", setter(into))]
    pub archived_identity: Option<Arc<Identity>>,
    #[builder(default = "None", setter(into))]
    pub original_identity: Option<Arc<Identity>>,
    #[builder(default = "None", setter(into))]
    pub tags: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub categories: Option<Arc<Categories>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Tags {
    pub tags: Arc<Vec<Arc<Tag>>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Categories {
    pub categories: Arc<Vec<Arc<Category>>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Identifiers {
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub lines: Option<Arc<Lines>>,
    #[builder(default = "None", setter(into))]
    pub line: Option<Arc<Line>>,
    pub identifiers: Arc<Vec<Arc<Identifier>>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Tag {
    pub key: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub value: Option<Arc<String>>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Category {
    #[builder(default = "None", setter(into))]
    pub value: Option<Arc<String>>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Comments {
    pub comments: Arc<Vec<Arc<Comment>>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Comment {
    pub zk: ZK,
    #[builder(default = "None", setter(into))]
    pub value: Option<Arc<String>>,
}

#[derive(Debug, Clone, Builder)]
pub struct ZK {
    #[builder(default = "None", setter(into))]
    pub text: Option<Arc<String>>,
    pub time: Arc<String>,
    /// TODO: join opt<time, accuracy, zone>
    #[builder(default = "None", setter(into))]
    pub time_accuracy: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub time_zone: Option<Arc<String>>,
    pub year: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub locale: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub location: Option<Arc<String>>,
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub tags: Option<Arc<Tags>>,
    #[builder(default = "None", setter(into))]
    pub categories: Option<Arc<Categories>>,
    #[builder(default = "None", setter(into))]
    pub signatures: Option<Arc<Signatures>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Signatures {
    pub hash: Arc<String>,
    pub signatures: Arc<Vec<Arc<Signature>>>,
    #[builder(default = "None", setter(into))]
    pub text: Option<Arc<String>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Signature {
    pub hash: Arc<String>,
    pub signature: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub text: Option<Arc<String>>,
    pub line: Arc<Line>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Key {
    pub hash: Arc<String>,
    #[builder(default = "None", setter(into))]
    pub public_key: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub signature: Option<Arc<Signature>>,
    #[builder(default = "None", setter(into))]
    pub id: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub name: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub kind: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub version: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub note: Option<Arc<String>>,
    #[builder(default = "None", setter(into))]
    pub created_at: Option<Arc<String>>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}

#[derive(Debug, Clone, Builder)]
pub struct Identity {
    pub key: Arc<Key>,
    pub signature: Arc<Signatures>,
    #[builder(default = "None", setter(into))]
    pub verification: Option<Arc<Verification>>,
    pub identifier: Arc<Identifier>,
    #[builder(default = "None", setter(into))]
    pub description: Option<Arc<Box<Description>>>,
    #[builder(default = "None", setter(into))]
    pub identifiers: Option<Arc<Identifiers>>,
}
