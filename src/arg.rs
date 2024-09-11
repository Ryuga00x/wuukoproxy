use clap::{Subcommand,Parser,command};

#[derive(Parser,Debug)]
#[command(name = "wuukoproxy")]
#[command(version = "1.0.1",
about ="使用 Rust 语言编写的服务器代理工具",long_about = "可兼容的方法, 如http https socks5 ")]
pub struct Cli{  // for arg )
    /// the ip adress of our Binded server
    #[arg(short,long)]
    pub ipadress : String,

}