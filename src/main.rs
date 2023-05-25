use clap::Parser;
use sb2md_converter::ToMd; 

#[derive(Debug, Parser)]
#[clap(name = "scrap2md", version, author, about, arg_required_else_help = true)]
struct Args {
    /// Project to convert
    #[arg(value_name = "PROJECT NAME")]
    project: String,
    /// Page to convert
    #[arg(value_name = "PAGE NAME")]
    page: Option<String>,
    /// Convert all of the pages in the project
    #[arg(short, long)]
    all: bool,
    /// List pages in the project
    #[arg(short, long)]
    list: bool,
    /// WIP: Convert private project (require `connect.sid` in cookie)
    #[arg(long)]
    private: bool,
    /// Output location
    #[arg(short, long, default_value = "~/scrap2md", value_name = "OUTPUT DIR")]
    output: String,
}

fn convert_all_page(project_name: String) {

}

fn convert_single_page(project_name: String, page_name: String) {

}

fn list_pages(project_name: String) {

}

fn main() {
    let args = Args::parse(); 
    
    if args.private {
        unimplemented!()
    }

    if args.list {
        list_pages(args.project);
    } else if let Some(v) = args.page {
        convert_single_page(args.project, v)
    } else if args.all {
        convert_all_page(args.project)
    } else {
        eprintln!("error: specify page name or set `--all` flag.")
    } 
}
