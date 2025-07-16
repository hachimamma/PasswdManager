use rand::{rngs::OsRng, RngCore};
use rpassword::read_password;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logger() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt::Subscriber::builder().with_env_filter(filter).init();
}

pub fn set_verbosity(v: u8, quiet: bool) {
    let level = if quiet {
        "error"
    } else {
        match v {
            0 => "info",
            1 => "debug",
            _ => "trace",
        }
    };
    std::env::set_var("RUST_LOG", level);
}

pub fn prompt(msg: &str) -> String {
    print!("{msg}");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    read_password().unwrap_or_default()
}

pub fn random_pw() -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789!@#$%^&*()-_=+";
    let mut rng = OsRng;
    (0..24)
        .map(|_| CHARS[(rng.next_u32() as usize) % CHARS.len()] as char)
        .collect()
}
