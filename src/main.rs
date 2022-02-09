fn main() {
    let img_sample_path_vec = load_image_path_vec("img_sample");
    println!("{:?}", img_sample_path_vec);

    img_sample_path_vec.iter().for_each(|img_sample_path| {
        let img_source_path_vec = load_image_path_vec("img_source");
        img_source_path_vec.iter().for_each(|img_source_path| {
            print!(
                "img_sample_path {:?}, img_source_path {:?} ... ",
                img_sample_path, img_source_path
            );
            let get_image_distance_result = similars_lib::get_image_distance_by_path(
                img_sample_path,
                img_source_path,
                8,
                8,
                false,
            );
            match get_image_distance_result {
                Err(e) => println!("{:?}", e),
                Ok(distance) => {
                    println!("distance {:?}", distance);
                    if distance <= 10 {
                        let filename = img_source_path.split('/').last().unwrap();
                        std::fs::rename(
                            img_source_path.clone(),
                            format!("img_result/{}", filename),
                        )
                        .unwrap();
                    }
                }
            }
        });
    });
}

fn load_image_path_vec(path: &str) -> Vec<String> {
    std::fs::read_dir(path)
        .unwrap()
        .map(|d| d.unwrap())
        .filter(|d| d.file_type().unwrap().is_file())
        .map(|d| d.path().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>()
}
