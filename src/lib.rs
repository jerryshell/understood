pub fn run<P>(
    img_sample_path: P,
    img_source_path: P,
    img_result_path: P,
    hamming_threshold: usize,
    clean_flag: bool,
) -> Result<(), std::io::Error>
where
    P: AsRef<std::path::Path>,
{
    let img_sample_path_vec = load_image_path_vec(img_sample_path)?;
    let img_source_path_vec = load_image_path_vec(img_source_path)?;
    let img_result_path = img_result_path.as_ref();
    img_sample_path_vec.into_iter().for_each(|img_sample_path| {
        let img_source_path_vec = img_source_path_vec.clone();
        handle_img_sample_path(
            img_sample_path,
            img_source_path_vec,
            img_result_path.to_path_buf(),
            hamming_threshold,
            clean_flag,
        );
    });
    Ok(())
}

pub fn load_image_path_vec<P>(path: P) -> Result<Vec<std::path::PathBuf>, std::io::Error>
where
    P: AsRef<std::path::Path>,
{
    let image_path_vec = std::fs::read_dir(path)?
        .filter_map(|entry| match entry {
            Ok(entry) if entry.file_type().ok()?.is_file() => Some(entry.path()),
            _ => None,
        })
        .collect::<Vec<_>>();
    Ok(image_path_vec)
}

pub fn handle_img_sample_path<P>(
    img_sample_path: P,
    img_source_path_vec: Vec<P>,
    img_result_path: P,
    hamming_threshold: usize,
    clean_flag: bool,
) where
    P: AsRef<std::path::Path>,
{
    let img_sample_path = img_sample_path.as_ref();
    let img_result_path = img_result_path.as_ref();
    img_source_path_vec.iter().for_each(|img_source_path| {
        let img_source_path = img_source_path.as_ref();
        match similars_lib::image_distance(img_source_path, img_sample_path, 8, 8) {
            Err(e) => {
                if clean_flag {
                    _ = std::fs::remove_file(img_source_path)
                }
                tracing::error!(
                    "{:?}, sample:{:?}, source:{:?}",
                    e,
                    img_sample_path,
                    img_source_path
                );
            }
            Ok(distance) => {
                if distance <= hamming_threshold {
                    if let Some(filename) = img_source_path.file_name() {
                        let new_path = img_result_path.join(filename);
                        if let Err(e) = std::fs::rename(img_source_path, new_path) {
                            tracing::error!("{:?}, {:?}", e, img_result_path);
                        }
                    }
                }
            }
        }
    });
}
