use dpkg_query_json::{QueryFieldPackage};


fn main() {
    let fields = vec![
                        "Package".to_string(),
                      "Version".to_string(),
                      "Architecture".to_string()];

    // let fields = vec!["Package".to_string(),
    //                   "Architecture".to_string()
    // ];


    //let fields = Vec::new();
    //let packages = Vec::new();
    let packages = vec!["dpkg".to_string()];
    println!("{:?}",QueryFieldPackage::new(fields, packages).json_string());

}
