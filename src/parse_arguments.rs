pub mod parse_arguments {
    use std::env;

    #[derive(Debug)]
    pub struct TelegramArguments {
        pub relative_path: String,
        pub token: String,
    }

    pub fn parse_args() -> Result<TelegramArguments, String> {
        let args: Vec<String> = env::args()
            .skip(1)
            .collect();

        if cfg!(debug_assertions) {
            eprintln!("Arguments found: {:?}", args)
        }

        if args.len() != 2 {
            return Err("Count of arguments is not 2".to_string())
        }

        Ok(TelegramArguments {
            relative_path: args[0].to_string(),
            token: args[1].to_string()
        })
    }
}
