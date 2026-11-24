use clap::{Parser, Subcommand};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const BANNER: &str = r"
  _                        
 | |    _   _ ___  _____   _____  __      ___ __ 
 | |   | | | |\ \/ / \ \ / / _ \ \ \ /\ / / '__|
 | |___| |_| | >  <   \ V / (_) | \ V  V /| |   
 |_____|\__, |/_/\_\   \_/ \___/   \_/\_/ |_|   
        |___/   AI website & app builder
";

const SITE_URL: &str = "https://lyxvo.ai";

const ABOUT_TEXT: &str = "Lyxvo.ai is an AI website and app builder.
Describe what you want in plain language and Lyxvo Agentic builds a real,
working project that you can preview live, edit with more instructions,
and publish on your own address or domain.

Learn more and start building: https://lyxvo.ai";

const INDEX_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{NAME}</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <main>
    <h1>Welcome to {NAME}</h1>
    <p>This is a simple starter website. Edit index.html and style.css to make it yours.</p>
  </main>
  <footer>
    <a href="https://lyxvo.ai">Built with Lyxvo.ai</a>
  </footer>
</body>
</html>
"#;

const STYLE_CSS: &str = r#"body {
  margin: 0;
  font-family: system-ui, sans-serif;
  background: #0f172a;
  color: #e2e8f0;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}
main {
  flex: 1;
  display: grid;
  place-items: center;
  padding: 2rem;
  text-align: center;
}
footer {
  padding: 1rem;
  text-align: center;
  font-size: 0.9rem;
}
a {
  color: #38bdf8;
}
"#;

#[derive(Parser)]
#[command(name = "lyxvo", version, about = "Learn about and promote Lyxvo.ai")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Explain what Lyxvo.ai is
    About,
    /// Open https://lyxvo.ai in your browser
    Open,
    /// Create a starter website folder
    New {
        /// Name of the folder to create
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        None => {
            println!("{BANNER}");
            println!("Run `lyxvo --help` to see what you can do.");
            Ok(())
        }
        Some(Command::About) => {
            println!("{BANNER}");
            println!("{ABOUT_TEXT}");
            Ok(())
        }
        Some(Command::Open) => open_site(),
        Some(Command::New { name }) => create_site(Path::new(&name)).map(|path| {
            println!("Created starter website in {}", path.display());
        }),
    };

    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn open_site() -> io::Result<()> {
    open::that(SITE_URL)?;
    println!("Opening {SITE_URL} ...");
    Ok(())
}

/// Creates a starter website in `dir`. Returns the created path.
fn create_site(dir: &Path) -> io::Result<PathBuf> {
    if dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("{} already exists", dir.display()),
        ));
    }
    let name = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-site");
    fs::create_dir_all(dir)?;
    fs::write(dir.join("index.html"), render_index(name))?;
    fs::write(dir.join("style.css"), STYLE_CSS)?;
    Ok(dir.to_path_buf())
}

fn render_index(name: &str) -> String {
    INDEX_HTML.replace("{NAME}", &escape_html(name))
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_text_mentions_site_url() {
        assert!(ABOUT_TEXT.contains(SITE_URL));
    }

    #[test]
    fn escape_html_handles_special_chars() {
        assert_eq!(
            escape_html(r#"<a href="x">&'"#),
            "&lt;a href=&quot;x&quot;&gt;&amp;&#39;"
        );
    }

    #[test]
    fn render_index_inserts_name_and_link() {
        let html = render_index("demo");
        assert!(html.contains("<title>demo</title>"));
        assert!(html.contains("Built with Lyxvo.ai"));
        assert!(!html.contains("{NAME}"));
    }

    #[test]
    fn create_site_writes_files_and_refuses_overwrite() {
        let base = std::env::temp_dir().join(format!("lyxvo-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let created = create_site(&base).expect("first create should succeed");
        assert!(created.join("index.html").exists());
        assert!(created.join("style.css").exists());
        assert!(create_site(&base).is_err());
        let _ = fs::remove_dir_all(&base);
    }
}
