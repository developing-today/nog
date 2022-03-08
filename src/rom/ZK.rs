#[derive(Debug)]
pub struct Binder {
    pub hash: String,
    pub visibility: Option<String>,
    pub private: Option<String>,
    pub public: Option<bool>,
    pub notes: Notes,
    pub binder: Vec<SlipBox>,
    pub identifier: Option<Identifier>,
    pub ownership: Option<Ownership>,
}

#[derive(Debug)]
pub struct Notes {
    pub hash: String,
    pub identifier: Identifier,
    pub visibility: Option<String>,
    pub private: Option<bool>,
    pub public: Option<bool>,
    pub file: Option<String>,
    pub note: Note,
    pub notes: Vec<Note>,
    pub text: Option<String>,
    pub line: Option<Line>,
    pub lines: Option<Lines>,
    pub verifications: Option<Verifications>,
    pub groups: Option<Groups>,
    pub local_identifier: Option<Identifier>,
    pub remote_identifier: Option<Identifier>,
    pub author_data: Option<Vec<Note>>,
    pub backlinks: Option<Citations>,
    pub backlinks_identifier: Option<Identifier>,
    pub cite: Option<Citation>,
    pub cite_identifier: Option<Identifier>,
    pub citation: Option<Citation>,
    pub citation_identifier: Option<Identifier>,
    pub citations: Option<Citations>,
    pub citations_identifier: Option<Identifier>,
    pub description: Option<Description>,
}
#[derive(Debug)]
pub struct SlipBox {
    pub hash: String,
    pub notes: Notes,
    pub slipbox: Vec<Notes>,
    pub identifier: Option<Identifier>,
    pub visibility: Option<String>,
    pub private: Option<bool>,
    pub public: Option<bool>,
    pub ownership: Option<Ownership>,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    /// by convention, the wiki/true name hashed alone is the canonical hash for meta representation of figures who do not have a hash (Such as Michaelangelo, Prince, Michelangelo (Teenage Mutant Ninja Turtles), Ged, Aragorn son of Arathorn, and so on.)
    pub hash: String,
    pub aliases: Option<Tags>,
    pub hash_aliases: Option<Categories>,
    pub file: Option<String>,
    pub prefix: Option<String>,
    pub prefix_aliases: Option<Tags>,
    pub prefix_hash: Option<String>,
    pub prefix_hash_aliases: Option<Categories>,
    pub root: Option<String>,
    pub remote_root: Option<String>,
    pub remote_tap_root: Option<String>,
    pub path: Option<String>,
    pub url: Option<String>,
    pub tags: Option<Tags>,
    pub categories: Option<Categories>,
    pub comments: Option<Comments>,
    pub identifiers: Option<Identifiers>,
    pub backlinks: Option<Citations>,
    pub references: Option<Citations>,
}

#[derive(Debug)]
pub struct Ownership {
    pub identifier: Option<Identifier>,
    pub identifiers: Option<Identifiers>,
    pub description: Option<Description>,
    pub author: Option<Author>,
    pub authors: Option<Authors>,
    pub group: Option<Group>,
    pub groups: Option<Groups>,
}

#[derive(Debug)]
pub struct Note {
    pub lines: Lines,
    pub hash: String,
    pub identifier: Option<Identifier>,
    pub identifiers: Option<Identifiers>,
    pub description: Option<Description>,
    pub verifications: Option<Verifications>,
}
#[derive(Debug)]
pub struct Group {
    pub authors: Authors,
    pub identifier: Identifier,
    pub description: Option<Description>,
    pub identifiers: Option<Identifiers>,
    pub bio: Option<Notes>,
    pub notes: Option<Notes>,
}

#[derive(Debug)]
pub struct Authors {
    pub hash: String,
    pub authors: Vec<Author>,
    pub identifier: Identifier,
    pub description: Option<Description>,
    pub identifiers: Option<Identifiers>,
}

#[derive(Debug)]
pub struct Author {
    pub identifier: Identifier,
    pub hash: String,
    pub description: Option<Description>,
    pub identifiers: Option<Identifiers>,
    pub bio: Option<Notes>,
    pub notes: Option<Notes>,
}

#[derive(Debug)]
pub struct Description {
    pub identifier: Identifier,
    pub hash: String,
    pub about: Option<String>,
    pub contact: Option<String>,
    pub username: Option<String>,
    pub handle: Option<String>,
    pub aliases: Option<Tags>,
    pub links: Option<Tags>,
    pub avatar: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social: Option<Tags>,
    pub tags: Option<Tags>,
    pub categories: Option<Categories>,
    pub references: Option<Citations>,
    pub references_identifier: Option<Identifier>,
    pub backlinks: Option<Citations>,
    pub backlinks_identifier: Option<Identifier>,
    pub verifications: Option<Verifications>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub updated_hash: String,
    pub updated_by: Option<String>,
    pub updated_note: Option<String>,
    pub updated_reason: Option<String>,
    pub updated_code: Option<String>,
    pub deleted_at: Option<String>,
    pub deleted_by: Option<Verifications>,
    pub deleted_note: Option<String>,
    pub deleted_reason: Option<String>,
    pub deleted_code: Option<String>,
}

#[derive(Debug)]
pub struct Line {
    pub zk: ZK,
    pub comments: Option<Comments>,
    pub citation: Option<Citation>,
    pub citations: Option<Citations>,
    pub signatures: Option<Signatures>,
}

#[derive(Debug)]
pub struct Lines {
    pub hash: String,
    pub lines: Vec<Line>,
    pub line: Option<Line>,
}

#[derive(Debug)]
pub struct Verifications {
    pub hash: String,
    pub lines: Lines,
    pub verifications: Vec<Verification>,
    pub identifier: Option<Identifier>,
    pub identifiers: Option<Identifiers>,
    pub verification: Option<Verification>,
}

#[derive(Debug)]
pub struct Verification {
    pub verified: bool,
    pub hash: String,
    pub kind: Option<String>,
    pub status: Option<String>,
    pub by: Option<String>,
    pub at: Option<String>,
    pub lines: Lines,
    pub categories: Option<Categories>,
    pub tags: Option<Tags>,
    pub identities: Identities,
    pub identifier: Option<Identifier>,
    pub identifiers: Option<Identifiers>,
}

#[derive(Debug)]
pub struct Identities {
    pub hash: String,
    pub identities: Vec<Identity>,
    pub identifier: Identifier,
    pub identifiers: Option<Identifiers>,
}

#[derive(Debug)]
pub struct Groups {
    pub hash: String,
    pub groups: Vec<Group>,
    pub identifier: Option<Identifier>,
    pub identifiers: Option<Identifiers>,
    pub description: Option<Description>,
}

#[derive(Debug)]
pub struct Citations {
    pub citations: Vec<Citation>,
}

#[derive(Debug)]
pub struct Citation {
    pub zk: Option<ZK>,
    pub format: Option<String>,
    pub value: Option<String>,
    pub hash: String,
    // underscore where user input stopped and auto resolution started
    pub identity: Option<Identity>,
    pub resoived_identity: Option<Identity>,
    pub archived_identity: Option<Identity>,
    pub original_identity: Option<Identity>,
    pub tags: Option<Tags>,
    pub categories: Option<Categories>,
}

#[derive(Debug)]
pub struct Tags {
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct Categories {
    pub categories: Vec<Category>,
}

#[derive(Debug)]
pub struct Identifiers {
    pub hash: String,
    pub lines: Option<Lines>,
    pub line: Option<Line>,
    pub identifiers: Vec<Identifier>,
}

#[derive(Debug)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
    pub identifier: Identifier,
    pub description: Option<Description>,
    pub identifiers: Option<Identifiers>,
}

#[derive(Debug)]
pub struct Category {
    pub value: Option<String>,
    pub identifier: Identifier,
    pub description: Option<Description>,
    pub identifiers: Option<Identifiers>,
}

#[derive(Debug)]
pub struct Comments {
    pub comments: Vec<Comment>,
}

#[derive(Debug)]
pub struct Comment {
    pub zk: ZK,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct ZK {
    pub text: Option<String>,
    pub time: String,
    pub time_accuracy: Option<String>,
    pub time_zone: Option<String>,
    pub year: String,
    pub locale: Option<String>,
    pub location: Option<String>,
    pub hash: String,
    pub tags: Option<Tags>,
    pub categories: Option<Categories>,
    pub comments: Option<Comments>,
    pub signatures: Option<Signatures>,
}

#[derive(Debug)]
pub struct Signatures {
    pub hash: String,
    pub signatures: Vec<Signature>,
    pub text: Option<String>,
}

#[derive(Debug)]
pub struct Signature {
    pub hash: String,
    pub signature: String,
    pub text: Option<String>,
    pub line: Line,
    pub identifier: Identifier,
    pub description: Option<Description>,
    pub identifiers: Option<Identifiers>,
}

#[derive(Debug)]
pub struct Key {
    pub hash: String,
    pub public_key: String,
    pub signature: Option<Signature>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub kind: Option<String>,
    pub version: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<String>,
    pub description: Option<Description>,
    pub identifier: Identifier,
    pub identifiers: Option<Identifiers>,
}

#[derive(Debug)]
pub struct Identity {
    pub key: Key,
    pub signature: Signatures,
    pub verification: Option<Verification>,
    pub identifier: Identifier,
    pub description: Option<Description>,
    pub identifiers: Option<Identifiers>,
}
