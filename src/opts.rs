use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "murs",
    about = "a cli tool to download music",
    rename_all = "kebab-case"
)]
pub struct Opt {

    #[structopt(short, long)]
    pub input: Option<String>,

    #[structopt(short, long, default_value = ".")]
    pub output: String,

    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(short, long, help = "youtube-dl bin path", default_value = "youtube-dl")]
    pub ytdl_bin_file: String
}