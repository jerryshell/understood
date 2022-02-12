use rand::prelude::SliceRandom;
use std::{
    fs,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, Sender},
        Arc, Mutex,
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
    let img_sample_path_vec = load_image_path_vec(img_sample_path).unwrap();
    let img_source_path_vec = load_image_path_vec(img_source_path).unwrap();

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
    let tx = Arc::new(Mutex::new(tx));

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
                tx,
            );
        })
    });

    loop {
        let _ = rx.recv();
        progress_current.fetch_add(1, Ordering::SeqCst);
        let progress_current = progress_current.load(Ordering::SeqCst);
        let progress = progress_current as f32 / progress_max as f32 * 100f32;
        println!("{}/{} {:.4}%", progress_current, progress_max, progress);
        if progress >= 100f32 {
            break;
        }
    }
}

pub fn load_image_path_vec(path: &str) -> Result<Vec<String>, String> {
    match fs::read_dir(path) {
        Ok(dir) => {
            let result = dir
                .map(|r| r.unwrap())
                .filter(|d| d.file_type().unwrap().is_file())
                .map(|d| d.path().into_os_string().into_string().unwrap())
                .collect::<Vec<String>>();
            Ok(result)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn handle_img_sample_path(
    img_sample_path: &str,
    img_source_path_vec: &[String],
    img_result_path: &str,
    hamming_threshold: usize,
    tx: Arc<Mutex<Sender<String>>>,
) {
    img_source_path_vec.iter().for_each(|img_source_path| {
        match similars_lib::get_image_distance_by_path(
            img_source_path,
            img_sample_path,
            8,
            8,
            false,
        ) {
            Err(e) => {
                println!("get_image_distance_by_path() :: error : {}", e);
            }
            Ok(distance) => {
                if distance <= hamming_threshold {
                    if let Some(filename) = img_source_path.split('/').last() {
                        let new_path = format!("{}/{}", img_result_path, filename);
                        if let Err(e) = fs::rename(img_source_path, new_path) {
                            println!(
                                "rename() :: error : {} :: img_result_path : {}",
                                e, img_result_path
                            );
                        }
                    }
                }
            }
        }

        if let Ok(sender) = tx.lock() {
            if let Err(e) = sender.send(img_source_path.to_string()) {
                println!("sender.send() :: error : {}", e);
            }
        }
    })
}
