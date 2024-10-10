use clap::Parser;
#[derive(Parser,Debug)]
#[command(version,about,long_about = None)]
struct Args {
    #[arg(short,long)]
    task: Option<String>,
    #[arg(short,long)]
    list: String
}


fn main() {
    let args = Args::parse();
    let mut tasklist = Vec::<Option<String>>::with_capacity(10);
    let mut n = 0;
    tasklist.push(args.task);
    if args.list == "true" {
        for item in tasklist {
            if item.is_some() {
                n += 1;
                let unwrapped = item.unwrap();
                println!("Task #{} - {}",n,unwrapped);
            }
        }
    }

}