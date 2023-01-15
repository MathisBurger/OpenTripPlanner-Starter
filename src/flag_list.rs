use rusty_cli::flags::flag::Flag;

pub(crate) fn get_flags() -> Vec<Flag> {

    let build_on_startup = Flag::new(
        "buildOnStartup".to_string(),
        vec!["bos".to_string()],
        false
    );

    return vec![build_on_startup];
}