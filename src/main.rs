mod commands;
use clap::{Parser, Subcommand};
use std::path::Path;



#[derive(Parser, Debug)]
#[command(version,
about = "A collection for small tools",
long_about = "A hub that collects some small tools that can be benefitial in the daily use")]
struct Cli{
    #[command(subcommand)]
    command:Commands
}

#[derive(Subcommand, Debug)]
enum Commands{
    #[command(about="Gives u variety of tools to deal and manipulate different files")]
    Files {
        #[command(subcommand)]
        command: FileCommands
    },
    #[command(about="A tool to download videos from Youtube either in mp3 format of in mp4 one")]
    YoutubeDownloader {
        #[command(subcommand)]
        command:DownloaderCommands
    },
    #[command(about="A tool that creates a QR code from a given link")]
    QrCodeCreator{
        link: String
    },
    #[command(about = "Download any repository you want")]
    RepositoryDownloader{
        link: String
    }
}

#[derive(Subcommand, Debug)]
enum FileCommands {
    #[command(about = "A group of tools to manipulate PDF files")]
    Pdfs {
        #[command(subcommand)]
        command: PDFCommands
    },
    #[command(about = "A group of tools to manipulate DOCX files")]
    Docx {
        #[command(subcommand)]
        command: DOCXCommands
    },
    #[command(about = "A group of tools to manipulate different types of images")]
    Img {
        #[command(subcommand)]
        command: IMGCommands
    }
}

#[derive(Subcommand, Debug)]
enum PDFCommands {
    #[command(about = "Convert to DOCX")]
    Convert{
        path:String
    },
    #[command(about = "Merge PDF files")]
    Merge{
        path1:String,
        path2:String
    },
    #[command(about = "Split PDF files")]
    Split{
        path:String
    }
}

#[derive(Subcommand, Debug)]
enum DOCXCommands {
    #[command(about = "Convert to PDF")]
    Convert{
        path:String
    },
    #[command(about = "Merge DOCX files")]
    Merge{
        path1:String,
        path2:String
    },
    #[command(about = "Split DOCX files")]
    Split{
        path:String
    }
}

#[derive(Subcommand, Debug)]
enum IMGCommands {
    #[command(about = "Convert to any other image type")]
    Convert{
        path:String,
        output: String
    },
    #[command(about = "Remove the background from any image")]
    RemoveBG{
        path:String
    },
    #[command(about = "Resize any image")]
    Resize{
        path: String,
        width: u32,
        height: u32
    }
}

#[derive(Subcommand, Debug)]
enum DownloaderCommands {
    #[command(about = "Download Videos in mp3 format")]
    Mp3 {
        link:String
    },
    #[command(about = "Download Videos in mp4 at any quality u wanat")]
    Mp4 {
        link:String,
        resolution:u32
    }
}




fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);
    
    match cli.command {
        Commands::Files { command } => {
            match command{
                FileCommands::Docx { command } => {
                    match command {
                        DOCXCommands::Convert { path } => {commands::files::convert_docx(Path::new(&path)).unwrap()},
                        DOCXCommands::Merge { path1, path2 } => {},
                        DOCXCommands::Split { path } => {}
                    }
                },
                FileCommands::Img { command } =>{
                    match command {
                        IMGCommands::Convert { path , output} => {commands::img::convert(path, output).unwrap()},
                        IMGCommands::RemoveBG { path } => {},
                        IMGCommands::Resize { path, width, height } => {}
                    }
                },
                FileCommands::Pdfs { command } => {
                    match command {
                        PDFCommands::Convert { path } => {commands::files::convert_pdf(Path::new(&path)).unwrap()},
                        PDFCommands::Merge { path1, path2 } => {commands::files::merge_pdf(path1, path2).unwrap()},
                        PDFCommands::Split { path } => {}
                    }
                }
            }
        },
        Commands::QrCodeCreator { link } => {commands::qr::qr_creator(link).unwrap()},
        Commands::RepositoryDownloader { link } => {},
        Commands::YoutubeDownloader { command } => {}
    }
}