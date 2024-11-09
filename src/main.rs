use anyhow::{Context, Result};
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
            let entry = entry?;
            let file_name = entry.file_name();
            let metadata = entry.metadata()?;

            if metadata.modified()? < last_checked {
                println!("file hasn't been changed since {last_checked}");
                continue;
            }

            for t in &targets {
                let target_file = t.join(file_name.clone());

                println!("Found {:?}; will copy to {:?}", file_name, target_file);

                fs::copy(entry.path(), target_file)
                    .with_context(|| format!("Failed to copy to {:?}", t))?;
            }
        }

        last_checked = OffsetDateTime::now_local()?;
        sleep(Duration::from_secs(2));
    }
}
