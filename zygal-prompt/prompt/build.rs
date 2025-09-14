use std::process;

fn main() -> process::ExitCode {
    match zygal_config::generate() {
        Ok(_) => process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error while generating zygal configuration. {err}");
            process::ExitCode::from(match err {
                zygal_config::Error::TomlNotFound { .. } => 80,
                zygal_config::Error::TomlParse(_) => 81,
                zygal_config::Error::ConfigRsWrite(_) => 82,
            })
        }
    }
}
