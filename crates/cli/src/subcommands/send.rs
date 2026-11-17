use crate::SendArgs;

pub fn send(args: &SendArgs) -> anyhow::Result<()> {
    println!("{:?}", args);
    Ok(())
}
