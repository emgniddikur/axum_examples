#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
}
