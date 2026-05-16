use clap::{Parser, Subcommand};

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
        path:String
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
        path:String
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
        path:String
    },
    #[command(about = "Remove the background from any image")]
    RemoveBG{
        path:String
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
        quality:u32
    }
}




fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli)

}