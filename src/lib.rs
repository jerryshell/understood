pub fn run(
    img_sample_path: std::path::PathBuf,
    img_source_path: std::path::PathBuf,
    img_result_path: std::path::PathBuf,
    hamming_threshold: usize,
    clean_flag: bool,
) {
    let img_sample_path_vec = load_image_path_vec(img_sample_path).unwrap();
    let img_source_path_vec = load_image_path_vec(img_source_path).unwrap();
    let progress_max = img_sample_path_vec.len() * img_source_path_vec.len();
    let mut progress_current = 0usize;
    img_sample_path_vec.into_iter().for_each(|img_sample_path| {
        let img_sample_path_clone = img_sample_path.clone();
        let img_source_path_vec = img_source_path_vec.clone();
        let img_result_path = img_result_path.clone();
        handle_img_sample_path(
            img_sample_path_clone,
            img_source_path_vec,
            img_result_path,
            hamming_threshold,
            clean_flag,
        );
        progress_current += 1;
        let progress = progress_current as f32 / progress_max as f32 * 100f32;
        tracing::info!(
            "{}/{}\t{:.4}%\t{:?}",
            progress_current,
            progress_max,
            progress,
            img_sample_path
        );
    });
}

pub fn load_image_path_vec(
    path: std::path::PathBuf,
) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let image_path_vec = std::fs::read_dir(path)?
        .filter_map(|entry| match entry {
            Ok(entry) if entry.file_type().ok()?.is_file() => Some(entry.path()),
            _ => None,
        })
        .collect::<Vec<std::path::PathBuf>>();
    Ok(image_path_vec)
}

pub fn handle_img_sample_path(
    img_sample_path: std::path::PathBuf,
    img_source_path_vec: Vec<std::path::PathBuf>,
    img_result_path: std::path::PathBuf,
    hamming_threshold: usize,
    clean_flag: bool,
) {
    img_source_path_vec.iter().for_each(|img_source_path| {
        match similars_lib::image_distance(img_source_path, &img_sample_path, 8, 8) {
            Err(e) => {
                if clean_flag {
                    _ = std::fs::remove_file(img_source_path)
                }
                tracing::error!("get_image_distance_by_path()::error:{:?}", e);
            }
            Ok(distance) => {
                if distance <= hamming_threshold {
                    if let Some(filename) = img_source_path.file_name() {
                        let new_path = img_result_path.join(filename);
                        if let Err(e) = std::fs::rename(img_source_path, new_path) {
                            tracing::error!(
                                "rename()::error:{:?},img_result_path:{:?}",
                                e,
                                img_result_path
                            );
                        }
                    }
                }
            }
        }
    })
}
