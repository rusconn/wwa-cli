use clap::Args as ClapArgs;

#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Shell to generate completion for
    pub shell: clap_complete::Shell,
}
