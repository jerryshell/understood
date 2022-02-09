use rand::prelude::*;
use std::{
    fs,
    thread::{self, JoinHandle},
};

fn main() {
    let img_sample_path_vec = load_image_path_vec("img_sample");
    let img_source_path_vec = load_image_path_vec("img_source");

    let join_handle_vec = img_sample_path_vec
        .into_iter()
        .map(|img_sample_path| {
            let mut img_source_path_vec = img_source_path_vec.clone();
            img_source_path_vec.shuffle(&mut rand::thread_rng());
            thread::spawn(move || handle_img_sample_path(&img_sample_path, &img_source_path_vec))
        })
        .collect::<Vec<JoinHandle<()>>>();

    for join_handle in join_handle_vec {
        join_handle.join().unwrap()
    }
}

fn load_image_path_vec(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .unwrap_or_else(|e| panic!("read_dir() :: error : {} :: path : {}", e, path))
        .map(|r| r.unwrap())
        .filter(|d| d.file_type().unwrap().is_file())
        .map(|d| d.path().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>()
}

fn handle_img_sample_path(img_sample_path: &str, img_source_path_vec: &[String]) {
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
                if distance <= 16 {
                    let filename = img_source_path.split('/').last().unwrap();
                    fs::rename(img_source_path, format!("img_result/{}", filename)).unwrap();
                }
            }
        }
    })
}
