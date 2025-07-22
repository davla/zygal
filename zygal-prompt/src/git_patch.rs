use std::{fmt::Display, path::Path, sync::LazyLock};

#[derive(Clone, Copy)]
pub enum GitPatch {
    Merge,
    Rebase,
    CherryPick,
    Revert,
}

static GIT_DIRS: LazyLock<Vec<(&'static str, GitPatch)>> = LazyLock::new(|| {
    vec![
        ("MERGE_HEAD", GitPatch::Merge),
        ("rebase-merge", GitPatch::Rebase),
        ("CHERRY_PICK_HEAD", GitPatch::CherryPick),
        ("REVERT_HEAD", GitPatch::Revert),
    ]
});

impl GitPatch {
    pub fn detect(current_dir: &Path) -> Option<Self> {
        let git_dir = current_dir.join(".git");
        GIT_DIRS.iter().find_map(|(git_file, git_patch)| {
            if git_dir.join(git_file).exists() {
                Some(*git_patch)
            } else {
                None
            }
        })
    }
}

impl Display for GitPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Merge => "M",
            Self::Rebase => "B",
            Self::CherryPick => "H",
            Self::Revert => "V",
        };
        write!(f, "{symbol}")
    }
}
