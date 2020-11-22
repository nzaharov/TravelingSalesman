use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(short = "N", long, default_value = "100")]
    pub cities: usize,
    #[structopt(short = "P", long, default_value = "5000")]
    pub population: usize,
    #[structopt(short = "G", long, default_value = "1000")]
    pub generations: usize,
}
