use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Zuhang Xu", about = "Train a decision tree model")]

// build a cli app with clap that can do factorial calculation and prime number check of a number
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Factorial(Factorial),
    Prime(Prime),
}

#[derive(Parser)]
struct Factorial {
    #[clap(short, long)]
    number: u64,
}

#[derive(Parser)]
struct Prime {
    #[clap(short, long)]
    number: u64,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Factorial(f) => {
            let mut result = 1;
            for i in 1..=f.number {
                result *= i;
            }
            println!("Factorial of {} is {}", f.number, result);
        }
        SubCommand::Prime(p) => {
            let mut is_prime = true;
            for i in 2..p.number {
                if p.number % i == 0 {
                    is_prime = false;
                    break;
                }
            }
            println!("{} is prime? {}", p.number, is_prime);
        }
    }
}













