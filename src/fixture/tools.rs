//! Initialize the filesystem to use as test fixtures.

use std::fs;
use std::io::Write;
use std::path;

use globwalk;

use super::errors::*;
use super::ChildPath;
use super::NamedTempFile;
use super::TempDir;

/// Create empty directories at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait PathCreateDir {
    /// Create an empty file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp.child("subdir").create_dir_all().unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn create_dir_all(&self) -> Result<(), FixtureError>;
}

impl PathCreateDir for ChildPath {
    fn create_dir_all(&self) -> Result<(), FixtureError> {
        create_dir_all(self.path())
    }
}

/// Create empty files at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileTouch {
    /// Create an empty file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp.child("foo.txt").touch().unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn touch(&self) -> Result<(), FixtureError>;
}

impl FileTouch for ChildPath {
    fn touch(&self) -> Result<(), FixtureError> {
        touch(self.path())
    }
}

impl FileTouch for NamedTempFile {
    fn touch(&self) -> Result<(), FixtureError> {
        touch(self.path())
    }
}

/// Write a binary file at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileWriteBin {
    /// Write a binary file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp
    ///     .child("foo.txt")
    ///     .write_binary(b"To be or not to be...")
    ///     .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn write_binary(&self, data: &[u8]) -> Result<(), FixtureError>;
}

impl FileWriteBin for ChildPath {
    fn write_binary(&self, data: &[u8]) -> Result<(), FixtureError> {
        write_binary(self.path(), data)
    }
}

impl FileWriteBin for NamedTempFile {
    fn write_binary(&self, data: &[u8]) -> Result<(), FixtureError> {
        write_binary(self.path(), data)
    }
}

/// Write a text file at [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileWriteStr {
    /// Write a text file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp
    ///    .child("foo.txt")
    ///    .write_str("To be or not to be...")
    ///    .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn write_str(&self, data: &str) -> Result<(), FixtureError>;
}

impl FileWriteStr for ChildPath {
    fn write_str(&self, data: &str) -> Result<(), FixtureError> {
        write_str(self.path(), data)
    }
}

impl FileWriteStr for NamedTempFile {
    fn write_str(&self, data: &str) -> Result<(), FixtureError> {
        write_str(self.path(), data)
    }
}

/// Write (copy) a file to [`ChildPath`].
///
/// [`ChildPath`]: struct.ChildPath.html
pub trait FileWriteFile {
    /// Write (copy) a file to [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::Path;
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp
    ///    .child("foo.txt")
    ///    .write_file(Path::new("Cargo.toml"))
    ///    .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    /// [`ChildPath`]: struct.ChildPath.html
    fn write_file(&self, data: &path::Path) -> Result<(), FixtureError>;
}

impl FileWriteFile for ChildPath {
    fn write_file(&self, data: &path::Path) -> Result<(), FixtureError> {
        write_file(self.path(), data)
    }
}

impl FileWriteFile for NamedTempFile {
    fn write_file(&self, data: &path::Path) -> Result<(), FixtureError> {
        write_file(self.path(), data)
    }
}

/// Copy files into [`TempDir`].
///
/// [`TempDir`]: struct.TempDir.html
pub trait PathCopy {
    /// Copy files and directories into the current path from the `source` according to the glob
    /// `patterns`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_fs::prelude::*;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// temp.copy_from(".", &["*.rs"]).unwrap();
    /// temp.close().unwrap();
    /// ```
    fn copy_from<P, S>(&self, source: P, patterns: &[S]) -> Result<(), FixtureError>
    where
        P: AsRef<path::Path>,
        S: AsRef<str>;
}

impl PathCopy for TempDir {
    fn copy_from<P, S>(&self, source: P, patterns: &[S]) -> Result<(), FixtureError>
    where
        P: AsRef<path::Path>,
        S: AsRef<str>,
    {
        copy_files(self.path(), source.as_ref(), patterns)
    }
}

impl PathCopy for ChildPath {
    fn copy_from<P, S>(&self, source: P, patterns: &[S]) -> Result<(), FixtureError>
    where
        P: AsRef<path::Path>,
        S: AsRef<str>,
    {
        copy_files(self.path(), source.as_ref(), patterns)
    }
}

fn ensure_parent_dir(path: &path::Path) -> Result<(), FixtureError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).chain(FixtureError::new(FixtureKind::CreateDir))?;
    }
    Ok(())
}

fn create_dir_all(path: &path::Path) -> Result<(), FixtureError> {
    fs::create_dir_all(path).chain(FixtureError::new(FixtureKind::CreateDir))?;
    Ok(())
}

fn touch(path: &path::Path) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    fs::File::create(path).chain(FixtureError::new(FixtureKind::WriteFile))?;
    Ok(())
}

fn write_binary(path: &path::Path, data: &[u8]) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    let mut file = fs::File::create(path).chain(FixtureError::new(FixtureKind::WriteFile))?;
    file.write_all(data)
        .chain(FixtureError::new(FixtureKind::WriteFile))?;
    Ok(())
}

fn write_str(path: &path::Path, data: &str) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    write_binary(path, data.as_bytes()).chain(FixtureError::new(FixtureKind::WriteFile))
}

fn write_file(path: &path::Path, data: &path::Path) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    fs::copy(data, path).chain(FixtureError::new(FixtureKind::CopyFile))?;
    Ok(())
}

fn copy_files<S>(
    target: &path::Path,
    source: &path::Path,
    patterns: &[S],
) -> Result<(), FixtureError>
where
    S: AsRef<str>,
{
    // `walkdir`, on Windows, seems to convert "." into "" which then fails.
    let source = source
        .canonicalize()
        .chain(FixtureError::new(FixtureKind::Walk))?;
    for entry in globwalk::GlobWalkerBuilder::from_patterns(&source, patterns)
        .follow_links(true)
        .build()
        .chain(FixtureError::new(FixtureKind::Walk))?
    {
        let entry = entry.chain(FixtureError::new(FixtureKind::Walk))?;
        let rel = entry
            .path()
            .strip_prefix(&source)
            .expect("entries to be under `source`");
        let target_path = target.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(target_path).chain(FixtureError::new(FixtureKind::CreateDir))?;
        } else if entry.file_type().is_file() {
            fs::create_dir_all(target_path.parent().expect("at least `target` exists"))
                .chain(FixtureError::new(FixtureKind::CreateDir))?;
            fs::copy(entry.path(), target_path).chain(FixtureError::new(FixtureKind::CopyFile))?;
        }
    }
    Ok(())
}
