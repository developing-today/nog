/// Line ⇐ String.
pub struct Line {
    pub text: String
}

/// Lines ⇐ [Lines].
pub struct Lines {
    pub lines: Vec<Line>
}


/// Note ⇐ [Line].
pub struct Note {
    pub lines: Lines
}

/// Index ⇐ [Line].
pub struct Index {
    pub lines: Lines
}

/// Reference ⇐ Line.
pub struct Reference {
    pub line: Line
}
