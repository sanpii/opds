use clap::Clap;

#[derive(Clap)]
struct Opt {
    #[clap(long)]
    /// basic auth username
    username: Option<String>,
    #[clap(long)]
    /// basic auth password
    password: Option<String>,
    /// OPDS root URL
    url: String,
}

fn main() {
    let opt = Opt::parse();
}
