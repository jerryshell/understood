use rand::prelude::SliceRandom;
use std::{fs, sync::mpsc::channel};
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
        pool.execute(move || {
            handle_img_sample_path(
                &img_sample_path,
                &img_source_path_vec,
                &img_result_path,
                hamming_threshold,
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
) {
    img_source_path_vec.iter().for_each(|img_source_path| {
        print!(
            "img_sample_path {:?}, img_source_path {:?} ... ",
            img_sample_path, img_source_path
        );
        match similars_lib::get_image_distance_by_path(
            img_sample_path,
            img_source_path,
            8,
            8,
            false,
        ) {
            Err(e) => println!("{:?}", e),
            Ok(distance) => {
                println!("distance {:?}", distance);
                if distance <= hamming_threshold {
                    let filename = img_source_path.split('/').last().unwrap();
                    fs::rename(img_source_path, format!("{}/{}", img_result_path, filename))
                        .unwrap();
                }
            }
        }
    })
}
