use futures::{Future, Stream};
use futures::prelude::*;

use std::fs;
use std::io::prelude::*;
use serde_derive::{Serialize, Deserialize};

use std::fs::File;
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, StreamingBody, GetObjectRequest};

#[derive(Serialize, Deserialize)]
struct Manifest {
    files: Vec<MetadataFile>
}

#[derive(Serialize, Deserialize)]
struct MetadataFile {
    key: String,
    size: u64
}

fn file_exists(filename: &str) -> bool {
    // Automatically closed once out of scope
    File::open(filename).is_ok()
}

fn download(key: String, filepath: String) -> bool {
    let bucket_name = "inventory.silktide.com";

    let request = GetObjectRequest {
        bucket: "inventory.silktide.com".to_string(),
        key: key.to_string(),
        ..Default::default()
    };

    let s3_client = S3Client::new(Region::UsEast1);

    let response = s3_client.get_object(request).sync().expect("Couldn't get object");

    let mut file = File::create(filepath);

    match file {
        Ok(mut f) => {
            let stream = response.body.unwrap();
            stream
//            let body = stream.try_concat().wait().unwrap();
//            f.write_all(&body);
        },
        Err(err) => panic!(err)
    }




//    let region = "us-east-1".parse().unwrap();
//    let credentials = Credentials::default();
//    let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
//    let (data, code) = bucket.get_object_stream(key).unwrap();
//    let mut output_file = File::create(filepath).expect("Unable to create file");
//    bucket.get_object_stream(key, &mut output_file);
    true
}

pub fn run(directory: String, mut file: File){
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let manifest : Manifest = serde_json::from_str(&data).unwrap();

//    let mut size : u64 = 0;
    let mut n = 0;

    let raw_folder = format!("{}/raw", directory);
    if !file_exists(&raw_folder) {
        fs::create_dir(&raw_folder);
    }
    for metadata_file in manifest.files.iter() {
        let filename = metadata_file.key.split("/").last().unwrap();
        let filepath = format!("{}/{}", raw_folder, filename);
        println!("{} {}", n, filename);
        if !file_exists(filename) {
            println!("File doesn't exist. Downloading...");
            download(metadata_file.key.to_string(), filepath);
        }

//
//        let full_filename = format!("{}/files/{}", filepath, filename);
//        let result =  read(full_filename, record_summary_map);
//        record_summary_map = result.unwrap();

//        process::exit(0);
        n = n + 1;
    }

//    let encoded = serde_json::to_string_pretty(&record_summary_map);
//    println!("{}", encoded.unwrap());

//    println!("{}", size);
}