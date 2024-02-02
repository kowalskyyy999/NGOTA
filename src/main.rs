use rand::{distributions::Alphanumeric, Rng};
use csv;
use std::collections::HashMap;
use std::fs;
use serde_yaml;
use std::collections::BTreeMap;
use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, action=ArgAction::SetTrue)]
    multi: bool,
    #[arg(long, value_name="")]
    files: String,
    #[arg(long, value_name="")]
    config_file: String
}

fn digit_mask(length: usize) -> i128 {
    let mut temp_vec = Vec::with_capacity(length);
    for _ in 0..length {
        let mut rng = rand::thread_rng();
        temp_vec.push(rng.gen_range(0..10).to_string())
    }
    let mask = temp_vec.join("");
    mask.parse::<i128>().unwrap()
}

fn string_mask(value: &str) -> String{
    let value_list: Vec<&str> = value.split(" ").collect();
    let mut temp_vec = Vec::new();
    for value in value_list {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(value.len())
            .map(char::from)
            .collect();

        temp_vec.push(s);
    }

    let mask = temp_vec.join(" ");
    mask
}

fn parse_config<'a>(file_path: String, config_file: String, is_multi: bool) -> BTreeMap<&'a str, Vec<String>> {
    let config_path = config_file.as_str(); 
    let mut config: BTreeMap<&str, Vec<String>> = BTreeMap::new();

    if is_multi {
        let contents = fs::read_to_string(config_path).unwrap();
        let yaml: BTreeMap<String, BTreeMap<String, Vec<String>>> = serde_yaml::from_str(&contents.as_str()).unwrap();
        let mut cols = yaml.get("config").unwrap()
                                        .get("fields").unwrap().to_owned();
        let mut files = yaml.get("config").unwrap()
                            .get("files").unwrap().to_owned();
        
        let mut full_files = Vec::new();

        for f in files {
            let f = format!("{}/{}", file_path, f);
            std::fs::read(f.clone()).expect(format!("File {} not found!", f.clone()).as_str());
            full_files.push(f.clone());
        }

        config.insert("files_path", full_files);
        config.insert("columns", cols.clone());


    } else {
        let contents = fs::read_to_string(config_path).unwrap();
        let yaml: BTreeMap<String, Vec<String>> = serde_yaml::from_str(&contents.as_str()).unwrap();
        let mut cols = yaml.get("fields").unwrap().to_owned();

        std::fs::read(file_path.clone().as_str()).expect(format!("File {} not found!", file_path.clone()).as_str());
        config.insert("files_path", vec![file_path]);
        config.insert("columns", cols.clone());
    }
    config
}

fn masking(config: BTreeMap<&str, Vec<String>>) -> Result<(), Box<dyn std::error::Error>>{

    let files_path = config.get("files_path").unwrap();
    let cols = config.get("columns").unwrap();
    let mut cache = Box::new(HashMap::new());

    for path in files_path {
        
        let mut path_split = path.split(".csv").collect::<Vec<&str>>();
        path_split.push("_mask");
        let output_path = format!("{}.csv", path_split.join(""));
        let mut wtr = csv::Writer::from_path(output_path.as_str()).unwrap();
        

        let mut rdr = csv::Reader::from_path(path).unwrap();
        let mut headers = rdr.headers().cloned().unwrap();
        wtr.write_record(headers.into_iter()).unwrap();

        let mut index = 0;
        let mut col_hash = Box::new(HashMap::new());
        for h in headers.iter() {
            col_hash.insert(h, index);
            index += 1;
        }

        for result in rdr.records() {
            let record = result.unwrap();
            let columns_headers = Vec::from_iter(headers.iter());
            let mut writers = Vec::new();

            for col in columns_headers.clone() {
                let col_idx = *col_hash.get(col).unwrap() as usize;
                let value = record.get(col_idx).unwrap().to_string();

                let mut writer: String;

                if cols.contains(&col.to_string()) {
                    if !cache.contains_key(value.as_str()) {
                        let mask = match value.clone().parse::<i64>() {
                            Ok(x) => {
                                let x_mask = digit_mask(x.to_string().len());
                                x_mask.to_string()
                            },
                            Err(_) => {
                                let x_mask = string_mask(value.as_str());
                                x_mask
                            }
                        };
                        cache.insert(value.to_string(), mask.clone());
                        writer = mask;
                    } else {
                        let mask = cache.get(value.as_str()).unwrap().to_owned();
                        writer = mask;
                    }
                } else {
                    writer = value.to_string();
                }
                writers.push(writer);
            }
            wtr.write_record(&writers).unwrap();
        }
        wtr.flush().unwrap();
    }
    Ok(())
}

fn main() {
    let args = Box::new(Args::parse());
    let config = parse_config(args.files, args.config_file, args.multi);
    masking(config).unwrap();
}