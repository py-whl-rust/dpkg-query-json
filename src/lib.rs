//! # dpkg-query-json
//!
//!
//! A crate for parsing "dpkg-query" in json.
//!
//! # Examples
//!
//!
//! Minimum length of fields 2. Default fields `Package` `Version`. List available below.
//!
//! if the list of packages is empty, all are returned
//!
//!
//! #### Map<String, Value>
//! ```
//! use dpkg_query_json::QueryFieldPackage;
//! let fields = vec![String::from("Package"),
//! String::from("Version"),
//! String::from("Architecture")];
//! let packages = vec![String::from("dpkg")];
//! QueryFieldPackage::new(fields, packages).json(); //Map<String, Value>
//!
//! ```
//!
//!
//! ```{"dpkg": Object({"Architecture": String("amd64"), "Version": String("1.19.7ubuntu3")})}```
//!
//!
//!
//!------------------------------------
//!
//!
//!#### String
//! ```
//! use dpkg_query_json::QueryFieldPackage;
//! let fields = vec![String::from("Package"),
//! String::from("Version"),
//! String::from("Architecture")];
//! let packages = vec![String::from("dpkg")];
//! QueryFieldPackage::new(fields, packages).json_string(); //String
//!
//! ```
//!
//! ```"{\"dpkg\":{\"Architecture\":\"amd64\",\"Version\":\"1.19.7ubuntu3\"}}"```
//!


//! # Package information fields
//!
//! Architecture
//!
//! Bugs
//!
//! Conffiles
//!
//! Config-Version
//!
//! Conflicts
//!
//! Breaks
//!
//! Depends
//!
//! Description
//!
//! Enhances
//!
//! Essential
//!
//! Filename
//!
//! Installed-Size
//!
//! MD5sum
//!
//! MSDOS-Filename
//!
//! Maintainer
//!
//! Origin
//!
//! Package
//!
//! Pre-Depends
//!
//! Priority
//!
//! Provides
//!
//! Recommends
//!
//! Replaces
//!
//! Revision
//!
//! Section
//!
//! Size
//!
//! Source
//!
//! Status
//!
//! Suggests
//!
//! Version
//!


use std::process::{Command};
use std::io::{Error};
use serde_json::{Value, Map, json};

//#[derive(Debug)]
pub struct QueryFieldPackage{
    fields: Vec<String>,
    packages: Vec<String>
}

impl QueryFieldPackage{
    pub fn new(fields: Vec<String>, packages: Vec<String>) -> Self{
        QueryFieldPackage{
            fields,
            packages
        }
    }

    fn exec(&mut self) -> Result<String, Error> {

        if self.fields.len() <= 1{
            self.fields.clear();
            self.fields = vec![String::from("Package"), String::from("Version")];
        }

        let mut command = String::from("dpkg-query -W");
        if self.fields.len() > 0{
            let mut modified_fields = Vec::with_capacity(29);
            for str in self.fields.iter(){
                modified_fields.push("${".to_owned() + &str + "}");
            }
            command.push_str(&format!(" -f '{}\t\n'", modified_fields.join("<==>")))
        }

        if self.packages.len() > 0{
            command.push_str(&format!(" {}", self.packages.join(" ")))
        }

        match Command::new("sh")
            .args(&["-c", command.as_str()])
            .output(){
            Ok(data) => {Ok(String::from_utf8_lossy(&data.stdout).to_string())}
            Err(e) => Err(e)
        }

    }

    fn parse_to_json(&mut self) -> Result<Map<String, Value>, Error> {
        let mut data_json = Map::new();

        for line in self.exec()?.split("\t\n"){
            let mut d = Map::new();
            let split_line = line.split("<==>").collect::<Vec<&str>>();
            for (i,line) in split_line[1..].iter().enumerate(){
                d.insert((self.fields[i + 1]).to_string(), json!(line));
            }
            &data_json.insert(split_line[0].to_string(), Value::from(d));
        }

        Ok(data_json)
    }

    pub fn json(mut self) -> Map<String, Value> {
        self.parse_to_json().unwrap_or_else(|err|{
            let mut x = Map::new();
            x.insert(String::from("error"), Value::from(err.to_string()));
            x
        })
    }


    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn json_string(mut self) -> String {
        serde_json::to_string(&self.parse_to_json().unwrap_or_else(|err|{
            let mut x = Map::new();
            x.insert(String::from("error"), Value::from(err.to_string()));
            x
        })).unwrap()
    }

}
