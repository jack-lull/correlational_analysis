use clap::Parser;
use log::{error, info};
use std::{process::exit, str::FromStr};

#[derive(Parser)]
#[clap(version, author, about, long_about = None, arg_required_else_help(true))]
struct Args {
    #[clap(long, short)]
    file: String,
}

#[derive(Debug, Clone)]
struct Coordinates {
    value_x: f64,
    value_y: f64,
}

impl Coordinates {
    fn new(value_x: f64, value_y: f64) -> Self {
        Self { value_x, value_y }
    }
}

#[derive(Debug)]
struct ProcessedData {
    rz: f64,
}

fn main() {
    // Configure logger to use intead of println
    env_logger::init();
    // Parse CLI argumens
    let args = Args::parse();

    let input_file = args.file;

    // Read the file
    // If it can't be read, exit with status 1 (error) and log the error
    let data: Vec<Coordinates> = match read_file(input_file) {
        Ok(data) => data,
        Err(err) => {
            error!("{}", err);
            // If file can't be read, exit with status 1 (error)
            exit(1)
        }
    };

    // Process the data
    let processed_data = logic(data.clone());

    println!(
        "\nPearson correlation coefficient = {}\n",
        processed_data.rz,
    );
}


// Read and parse the input file into the vector of Data struct
fn read_file(input_file: String) -> Result<Vec<Coordinates>, Box<dyn std::error::Error>> {
    let mut data: Vec<Coordinates> = vec![];

    info!(
        "trying to poppulate data from the input csv file: {}",
        input_file
    );
    let mut file = match csv::Reader::from_path(input_file) {
        Ok(rdr) => rdr,
        // Error handling
        Err(err) => {
            error!("{}", err);
            return Err(Box::new(err));
        }
    };

    // Parsing data into the struct
    info!("trying to parse poppulated data into a vector of structs");
    for (i, el) in file.records().enumerate() {
        let record = match el {
            Ok(rec) => rec,
            // Error handling
            Err(err) => {
                error!("{}", err);
                return Err(Box::new(err));
            }
        };
        // I wouldn't add error hanling here, because it will become a huge chunk of code,
        //  unwraps will do their job anyway, error message is not there important here
        let value_x: f64 = FromStr::from_str(record.get(0).unwrap()).unwrap();
        let value_y: f64 = FromStr::from_str(record.get(1).unwrap()).unwrap();
        // Insted of intialazing Data here as a struct, I'm using a method
        //  that is defined in the `impl Data {...} part. It's a function that
        //  that basically does the same, but it's not that ugly, I think
        data.push(Coordinates::new(value_x, value_y));
    }
    // Return the poppulated data
    Ok(data)
}


fn logic(data: Vec<Coordinates>) -> ProcessedData {
    let data_len = data.len() as i64;

    let mut value_x_sum: f64 = 0.0;
    let mut value_y_sum: f64 = 0.0;
    // Расчет суммы x и y
    for data_el in data.clone() {
        value_x_sum += data_el.value_x;
        value_y_sum += data_el.value_y;
    }
    // Расчет среднего значения x и y
    let avg_value_x: f64 = value_x_sum / data_len as f64;
    let avg_value_y: f64 = value_y_sum / data_len as f64;

    let mut x_min_avg_t_y_min_avg_sum: f64 = 0.0;
    let mut x_min_avg_sqr_sum: f64 = 0.0;
    let mut y_min_avg_sqr_sum: f64 = 0.0;


    for data_el in data.clone() {
        x_min_avg_t_y_min_avg_sum += (data_el.value_x - avg_value_x)*(data_el.value_y - avg_value_y);
        x_min_avg_sqr_sum += (data_el.value_x - avg_value_x)*(data_el.value_x - avg_value_x);
        y_min_avg_sqr_sum += (data_el.value_y - avg_value_y)*(data_el.value_y - avg_value_y);
    }

    let rz: f64 = x_min_avg_t_y_min_avg_sum / (x_min_avg_sqr_sum * y_min_avg_sqr_sum).sqrt();

    ProcessedData {
        rz,
    }
}
