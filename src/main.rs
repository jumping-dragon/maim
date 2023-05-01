#![allow(non_snake_case)]

use aws_sdk_iam as iam;

mod role;

#[tokio::main]
async fn main() {
    let config = aws_config::from_env().load().await;
    let iam_client = iam::Client::new(&config);

    // for n in 1..3 {
    //     let response = iam_client.list_users().max_items(2).send().await.unwrap();
    //     let users = response.users().unwrap();
    //     let markers = response.marker().unwrap();
    //     println!("{:#?}", &users);
    //     println!("{:#?}", &markers)
    // }

    let mut call = iam_client.list_roles().max_items(2);

    loop {
        let response = call.clone().send().await.unwrap();
        let roles = response.roles().unwrap();
        println!("{:#?}", roles);

        for role in roles.iter() {
            role::printRole(role)
        }

        let marker = response.marker();
        match marker {
            Some(marker) => {
                println!("Marker {:#?}", &marker);
                call = call.set_marker(Some(marker.to_string()));
            }
            None => break,
        }
    }
}
