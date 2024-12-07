use rand::prelude::SliceRandom;
use threadpool::ThreadPool;

pub fn run(
    img_sample_path: std::path::PathBuf,
    img_source_path: std::path::PathBuf,
    img_result_path: std::path::PathBuf,
    n_workers: usize,
    hamming_threshold: usize,
    clean_flag: bool,
) {
    let img_sample_path_vec = load_image_path_vec(img_sample_path).unwrap();
    let img_source_path_vec = load_image_path_vec(img_source_path).unwrap();

    let progress_max = img_sample_path_vec.len() * img_source_path_vec.len();
    let mut progress_current = 0usize;

    let n_workers = if n_workers == 0 {
        num_cpus::get()
    } else {
        n_workers
    };
    println!("n_workers {}", n_workers);

    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = std::sync::mpsc::channel::<std::path::PathBuf>();

    img_sample_path_vec.into_iter().for_each(|img_sample_path| {
        let img_sample_path = img_sample_path.clone();
        let img_result_path = img_result_path.clone();
        let mut img_source_path_vec = img_source_path_vec.clone();
        img_source_path_vec.shuffle(&mut rand::thread_rng());
        let tx = tx.clone();
        pool.execute(move || {
            handle_img_sample_path(
                img_sample_path,
                img_source_path_vec,
                img_result_path,
                hamming_threshold,
                clean_flag,
                tx,
            );
        })
    });
    drop(tx);

    for result in rx {
        progress_current += 1;
        let progress = progress_current as f32 / progress_max as f32 * 100f32;
        println!(
            "{}/{}\t{:.4}%\t{:?}",
            progress_current, progress_max, progress, result
        );
    }
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
    tx: std::sync::mpsc::Sender<std::path::PathBuf>,
) {
    img_source_path_vec.iter().for_each(|img_source_path| {
        match similars_lib::image_distance(img_source_path, &img_sample_path, 8, 8) {
            Err(e) => {
                if clean_flag {
                    _ = std::fs::remove_file(img_source_path)
                }
                eprintln!("get_image_distance_by_path()::error:{:?}", e);
            }
            Ok(distance) => {
                if distance <= hamming_threshold {
                    if let Some(filename) = img_source_path.file_name() {
                        let new_path = img_result_path.join(filename);
                        if let Err(e) = std::fs::rename(img_source_path, new_path) {
                            eprintln!(
                                "rename()::error:{:?},img_result_path:{:?}",
                                e, img_result_path
                            );
                        }
                    }
                }
            }
        }

        if let Err(e) = tx.send(img_source_path.clone()) {
            eprintln!("sender.send()::error:{:?}", e);
        }
    })
}
