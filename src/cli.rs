use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "repo-to-gpt",
    version = "1.0",
    author = "Michael Bruno <michael.bruno1337@gmail.com>",
    about = "Scans a folder, filters files, and exports to minified JSON"
)]
pub struct Args {
    #[arg(long, value_name = "INPUT_FOLDER")]
    pub(crate) input: String,

    #[arg(long, default_value = "gpt.json", value_name = "OUTPUT_FILE")]
    pub(crate) output: String,

    #[arg(
        long,
        value_name = "EXCLUDED_DIRS",
        default_value = "node_modules,dist,.venv,extension,tmp,certs,__pycache__"
    )]
    pub(crate) excluded: String,

    #[arg(
        long,
        value_name = "IGNORED_EXTENSIONS",
        default_value = ".log,.tmp,.bak,.db,.json,.docx,.ico,.png"
    )]
    pub(crate) ignored: String,


    #[arg(long, help = "Output pretty-printed JSON instead of minified")]
    pub(crate) pretty: bool,
}
