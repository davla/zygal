use std::{collections::HashMap, fmt::Display, path::Path, sync::LazyLock};

use crate::config;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GitPatch {
    Merge,
    Rebase,
    CherryPick,
    Revert,
}

struct GitPatchInfo {
    detection_path: &'static str,
    symbol: &'static str,
}

static GIT_PATCH_INFOS: LazyLock<HashMap<GitPatch, GitPatchInfo>> = LazyLock::new(|| {
    let mut git_patch_infos = HashMap::new();

    let mut s = String::new();

    if let Some(symbol) = config::GIT_MERGE {
        s.push(' ');
        git_patch_infos.insert(
            GitPatch::Merge,
            GitPatchInfo {
                detection_path: "MERGE_HEAD",
                symbol,
            },
        );
    }
    if let Some(symbol) = config::GIT_REBASE {
        git_patch_infos.insert(
            GitPatch::Rebase,
            GitPatchInfo {
                detection_path: "rebase-merge",
                symbol,
            },
        );
    }
    if let Some(symbol) = config::GIT_CHERRY_PICK {
        git_patch_infos.insert(
            GitPatch::CherryPick,
            GitPatchInfo {
                detection_path: "CHERRY_PICK_HEAD",
                symbol,
            },
        );
    }
    if let Some(symbol) = config::GIT_REVERT {
        git_patch_infos.insert(
            GitPatch::Revert,
            GitPatchInfo {
                detection_path: "REVERT_HEAD",
                symbol,
            },
        );
    }

    git_patch_infos
});

impl GitPatch {
    pub fn detect(current_dir: &Path) -> Option<Self> {
        let git_dir = current_dir
            .ancestors()
            .map(|dir| dir.join(".git"))
            .find(|git_dir| git_dir.exists())?;
        GIT_PATCH_INFOS
            .iter()
            .find_map(|(&git_patch, git_patch_info)| {
                git_dir
                    .join(git_patch_info.detection_path)
                    .exists()
                    .then_some(git_patch)
            })
    }
}

impl Display for GitPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = GIT_PATCH_INFOS
            .get(self)
            .unwrap_or_else(|| panic!("Unknown GitPatch {self:?}"))
            .symbol;
        write!(f, "{symbol}")
    }
}
