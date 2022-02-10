use rand::prelude::SliceRandom;
use std::{
    fs,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::channel,
        Arc,
    },
};
use threadpool::ThreadPool;

pub fn run(
    img_sample_path: &str,
    img_source_path: &str,
    img_result_path: &str,
    n_workers: usize,
    hamming_threshold: usize,
) {
    let img_sample_path_vec = load_image_path_vec(img_sample_path);
    let img_source_path_vec = load_image_path_vec(img_source_path);

    let progress_max = img_sample_path_vec.len() * img_source_path_vec.len();
    let progress_current = Arc::new(AtomicUsize::new(0));

    let n_workers = if n_workers == 0 {
        num_cpus::get()
    } else {
        n_workers
    };
    println!("n_workers {}", n_workers);

    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    img_sample_path_vec.into_iter().for_each(|img_sample_path| {
        let mut img_source_path_vec = img_source_path_vec.clone();
        img_source_path_vec.shuffle(&mut rand::thread_rng());
        let tx = tx.clone();
        let img_result_path = img_result_path.to_string();
        let progress_current = progress_current.clone();
        pool.execute(move || {
            handle_img_sample_path(
                &img_sample_path,
                &img_source_path_vec,
                &img_result_path,
                hamming_threshold,
                progress_max,
                progress_current,
            );
            tx.send(img_sample_path.to_string()).unwrap();
        })
    });

    let _ = rx.iter().collect::<Vec<String>>();
}

pub fn load_image_path_vec(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .unwrap_or_else(|e| panic!("read_dir() :: error : {} :: path : {}", e, path))
        .map(|r| r.unwrap())
        .filter(|d| d.file_type().unwrap().is_file())
        .map(|d| d.path().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>()
}

pub fn handle_img_sample_path(
    img_sample_path: &str,
    img_source_path_vec: &[String],
    img_result_path: &str,
    hamming_threshold: usize,
    progress_max: usize,
    progress_current: Arc<AtomicUsize>,
) {
    img_source_path_vec.iter().for_each(|img_source_path| {
        progress_current.fetch_add(1, Ordering::SeqCst);
        let progress_current = progress_current.load(Ordering::SeqCst);
        let progress = progress_current as f32 / progress_max as f32 * 100f32;
        println!("{}/{} {:.4}%", progress_current, progress_max, progress);

        match similars_lib::get_image_distance_by_path(
            img_sample_path,
            img_source_path,
            8,
            8,
            false,
        ) {
            Err(_) => {}
            Ok(distance) => {
                if distance <= hamming_threshold {
                    let filename = img_source_path.split('/').last().unwrap();
                    fs::rename(img_source_path, format!("{}/{}", img_result_path, filename))
                        .unwrap_or_else(|e| {
                            panic!(
                                "rename() :: error : {} :: img_result_path : {}",
                                e, img_result_path
                            )
                        });
                }
            }
        }
    })
}
