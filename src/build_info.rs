#[derive(Debug)]
pub struct BuildInfo {
    pub version: &'static str,
    pub commit_hash: &'static str,
}

impl BuildInfo {
    #[must_use]
    pub fn short_commit_hash(&self) -> &'static str {
        if self.commit_hash.len() > 10 {
            &self.commit_hash[..10]
        } else {
            self.commit_hash
        }
    }
}

pub const BUILD_INFO: BuildInfo = BuildInfo {
    version: match option_env!("CARGO_PKG_VERSION") {
        None => "dev",
        Some(env) => env,
    },
    commit_hash: match option_env!("BUILD_GIT_HASH") {
        None => "dev",
        Some(env) => env,
    },
};
