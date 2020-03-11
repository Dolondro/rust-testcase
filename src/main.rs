use std::fs::File;
use std::io::Write;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, GetObjectRequest};

// This https://github.com/rusoto/rusoto/issues/1268 seems to imply that this should solve the issue
// I'm having, but it's flagging them as unused imports, which I suspect means I'm barking up the
// wrong tree
use futures::{Future, Stream};

fn main() {
    let bucket = "my_example_bucket";
    let key = "my_key.json";

    let request = GetObjectRequest {
        bucket: bucket.to_string(),
        key: key.to_string(),
        ..Default::default()
    };

    let mut output_file = File::create("/tmp/blah.json").unwrap();

    let s3_client = S3Client::new(Region::UsEast1);
    let response = s3_client.get_object(request).sync().expect("Couldn't get object");


    let stream = response.body.unwrap();
    // Based on https://stackoverflow.com/questions/51287360/how-to-save-a-file-downloaded-from-s3-with-rusoto-to-my-hard-drive
    // This looks like it should work. The docs state that concat2 was renamed back to concat for futures 0.2
    let body = stream.concat().wait().unwrap();
    output_file.write_all(&body).expect("failed to write body");

}