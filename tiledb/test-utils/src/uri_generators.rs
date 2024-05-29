use anyhow::{anyhow, Result};
use tempfile::TempDir;

pub trait TestArrayUri {
    fn base_dir(&self) -> Result<String>;
    fn with_paths(&self, paths: &[&str]) -> Result<String>;
    fn close(self) -> Result<()>;

    fn with_path(&self, path: &str) -> Result<String> {
        self.with_paths(&[path])
    }
}

pub fn get_uri_generator() -> Result<impl TestArrayUri> {
    // TODO: Eventually this will check an environment variable to decide
    // whether we should return a TestDirectory or a new struct called something
    // like TestRestServer to run our test suite against the cloud service.
    TestDirectory::new()
}

pub struct TestDirectory {
    base_dir: TempDir,
}

impl TestDirectory {
    pub fn new() -> Result<Self> {
        Ok(Self {
            base_dir: TempDir::new().map_err(|e| {
                anyhow!("Error creating temporary directory").context(e)
            })?,
        })
    }

    pub fn base_dir(&self) -> Result<String> {
        let path = self
            .base_dir
            .path()
            .to_str()
            .map(|s| s.to_string())
            .ok_or(anyhow!("Error creating test array URI"))?;
        Ok("file://".to_string() + &path)
    }

    pub fn with_path(&self, path: &str) -> Result<String> {
        self.with_paths(&[path])
    }

    pub fn with_paths(&self, paths: &[&str]) -> Result<String> {
        let path = self.base_dir.path().to_path_buf();
        let path = paths.iter().fold(path, |path, part| path.join(part));
        let path = path
            .to_str()
            .map(|p| p.to_string())
            .ok_or(anyhow!("Error creating temporary URI".to_string()))?;
        Ok("file://".to_string() + &path)
    }

    pub fn close(self) -> Result<()> {
        self.base_dir
            .close()
            .map_err(|e| anyhow!("Error closing temporary directory: {}", e))
    }
}

impl TestArrayUri for TestDirectory {
    fn base_dir(&self) -> Result<String> {
        self.base_dir()
    }

    fn with_paths(&self, paths: &[&str]) -> Result<String> {
        self.with_paths(paths)
    }

    fn close(self) -> Result<()> {
        self.close()
    }
}
