
extern crate webe_args;
use webe_args::{Args, ArgOpts};

pub struct BenchArgs {
    pub total_requests: usize,
    pub concurrency: usize,
    pub url: String
}

impl BenchArgs {
    pub fn new(cli_args: Args) -> BenchArgs {
        cli_args.parse_args();
        BenchArgs {
            total_requests: cli_args.get("total").unwrap().unwrap().parse().unwrap(),
            concurrency: cli_args.get("concurrency").unwrap().unwrap().parse().unwrap(),
            url: cli_args.get("url").unwrap().unwrap(),
        }
    }
}

pub fn prepare_args() -> Args {
    let mut bench_args = Args::new();
    // TODO: user should be able to provide an alternative status code to treat as successful

    // "total" number of requests to perform
    bench_args.add(
        "total".to_owned(),
        ArgOpts {
            short: Some("t".to_owned()),
            description: Some("Total number of requests to make.".to_owned()),
            is_required: true,
            is_flag: false,
            validation: None,
        }
    );

    // "concurrency" number of requests to make in parallel
    bench_args.add(
        "concurrency".to_owned(),
        ArgOpts {
            short: Some("c".to_owned()),
            description: Some("Number of concurrent requests to make.".to_owned()),
            is_required: true,
            is_flag: false,
            validation: None,
        }
    );

    // "url" the target url to make requests against
    bench_args.add(
        "url".to_owned(),
        ArgOpts {
            short: Some("u".to_owned()),
            description: Some("The target URL to make requests against".to_owned()),
            is_required: true,
            is_flag: false,
            validation: None,
        }
    );

    return bench_args;
}

