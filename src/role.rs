use aws_sdk_iam::types::Role;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urlencoding::decode;

pub fn printRole(role: &Role) {
    println!("Name: {}", &role.role_name().unwrap());
    let raw_assume_role_policy = &role.assume_role_policy_document().unwrap();
    let dec_assume_role_policy = decode(&raw_assume_role_policy).expect("UTF-8");
    let assume_role_policy = serde_json::from_str::<AssumeRolePolicy>(&dec_assume_role_policy);
    match assume_role_policy {
        Ok(assume_role_policy) => {
            println!("Assume Role Policy: {:#?}\n", assume_role_policy);
        }
        Err(error) => {
            panic!("Error; {:#?}\n", error);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AssumeRolePolicy {
    Statement: Vec<AssumeRoleStatement>,
    Version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssumeRoleStatement {
    Effect: String,
    Principal: AssumeRolePrincipal,
    Action: Quantifiable,
    Condition: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum AssumeRolePrincipal {
    Simple(String),
    Complex(HashMap<String, Quantifiable>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Quantifiable {
    Single(String),
    Multiple(Vec<String>),
}
