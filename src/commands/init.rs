use anyhow::Result;

pub async fn execute(shell: &str) -> Result<()> {
    match shell {
        "bash" | "zsh" => {
            println!(
                r#"# Add this to your ~/.bashrc or ~/.zshrc:

proj() {{
    if [ "$1" = "cd" ]; then
        shift
        local dir=$(command proj cd "$@")
        if [ $? -eq 0 ] && [ -n "$dir" ]; then
            builtin cd "$dir"
        fi
    else
        command proj "$@"
    fi
}}
"#
            );
        }
        "fish" => {
            println!(
                r#"# Add this to your ~/.config/fish/config.fish:

function proj
    if test "$argv[1]" = "cd"
        set -e argv[1]
        set dir (command proj cd $argv)
        if test $status -eq 0 -a -n "$dir"
            builtin cd $dir
        end
    else
        command proj $argv
    end
end
"#
            );
        }
        _ => {
            anyhow::bail!("Unsupported shell: {}. Use bash, zsh, or fish.", shell);
        }
    }

    Ok(())
}
