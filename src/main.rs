use rand::{distributions::Alphanumeric, Rng};
use csv;
use std::collections::HashMap;
use std::fs;
use serde_yaml;
use std::collections::BTreeMap;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    data_file: String,
    #[arg(short, long)]
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

fn data_mask(path: &str, cols: Vec<String>, cache: &mut Box<HashMap<String, String>>) {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut path_splt: Vec<&str> = path.split(".csv").collect();
    path_splt.push("_mask");
    let output_path = format!("{}.csv", path_splt.join(""));
    let mut wtr = csv::Writer::from_path(output_path.as_str()).unwrap();

    println!("Masking data in : {}", path);
    println!("Masking data out: {}", output_path);
    
    let mut headers = rdr.headers().cloned().unwrap();
    wtr.write_record(headers.into_iter()).unwrap();

    let mut index = 0;
    let mut col_hash = Box::new(HashMap::new());
    for h in headers.iter() {
        col_hash.insert(h, index);
        index += 1;
    }

    for result in rdr.records() {
        let records = result.unwrap();
        let columns_headers = Vec::from_iter(headers.iter());
        let mut writers = Vec::new();

        for col in columns_headers.clone() {
            let col_idx = *col_hash.get(col).unwrap() as usize;

            let value = records.get(col_idx).unwrap().to_string();

            let mut writer: String;

            if cols.contains(&col.to_string()) {
                if !cache.contains_key(value.as_str()) {
                    let mask: String = match value.clone().parse::<i64>() {
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
    wtr.flush().unwrap()
}

fn get_columns<'a>(path: &'a str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    let yaml: BTreeMap<String, Vec<String>> = serde_yaml::from_str(contents.as_str()).unwrap();
    let mut cols = yaml.get("fields").unwrap().to_owned();
    cols
}

fn main() {
    let args = Args::parse();
    let cols = get_columns(args.config_file.as_str());
    let mut cache: Box<HashMap<String,String>> = Box::new(HashMap::new());
    data_mask(args.data_file.as_str(), cols.clone(), &mut cache);
}