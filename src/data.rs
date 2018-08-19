use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Database<'a>  {
    data: HashMap<&'a str, &'a str>
}

pub struct Report {

}