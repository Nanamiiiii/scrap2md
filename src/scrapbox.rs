struct Scrapbox {
    session: Option<String>,
    project: SbProject,
}

impl Scrapbox {
    pub fn new() -> Self {
        Self {
            session: None,
            project: SbProject::default(),
        }
    }

    pub fn new_with_session(sid: String) -> Self {
        Self {
            session: Some(sid),
            project: SbProject::default(),
        }
    }
}

#[derive(Default)]
struct SbProject {
    name: String,
    page_count: u64,
    pages: Vec<SbPage>,
}

#[derive(Default)]
struct SbPage {
    name: String,
    line_count: u64,
    lines: Vec<String>,
}