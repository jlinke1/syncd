use anyhow::{Context, Ok, Result};
use std::{fs, path::Path, thread::sleep, time::Duration};
use time::OffsetDateTime;

fn main() -> Result<()> {
    let source = Path::new("test/data/source");
    let target = Path::new("test/data/target");
    let target2 = Path::new("test/data/other_target");
    let targets = vec![target, target2];
    let mut last_checked = OffsetDateTime::now_local()?;

    println!("{}", last_checked);

    loop {
        for entry in fs::read_dir(&source)? {
            println!("Current entry: {:?}", entry);
            handle_entry(entry?, last_checked, &targets)?;
        }

        last_checked = OffsetDateTime::now_local()?;
        sleep(Duration::from_secs(2));
    }
}

fn handle_entry(
    entry: fs::DirEntry,
    last_checked: time::OffsetDateTime,
    targets: &[impl AsRef<Path>],
) -> Result<()> {
    if entry.path().is_dir() {
        println!("{:?} is a directory", entry);

        let targets_with_subdir: Vec<_> = targets
            .iter()
            .map(|t| t.as_ref().join(entry.file_name()))
            .collect();

        for subdir_entry in fs::read_dir(&entry.path())? {
            handle_entry(subdir_entry?, last_checked, &targets_with_subdir)?
        }

        return Ok(());
    }

    let file_name = entry.file_name();
    let metadata = entry.metadata()?;

    if metadata.modified()? < last_checked {
        return Ok(());
    }

    for target in targets {
        let target_path = target.as_ref();
        if !target_path.exists() {
            println!("creating directory {:?}", target_path);
            fs::create_dir(target_path)?;
        }
        let target_file = target_path.join(&file_name);

        println!("Found {:?}; will copy to {:?}", file_name, target_file);

        fs::copy(entry.path(), target_file)
            .with_context(|| format!("Failed to copy to {:?}", target_path))?;
    }

    Ok(())
}
