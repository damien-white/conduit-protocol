use std::{env, net::SocketAddr, time::Duration};

use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<String>();
    // let cmd = parse_user_command(args);

    let addr = "0.0.0.0:59100"
        .parse::<SocketAddr>()
        .expect("socket address must be valid");

    println!("Connecting to service at: {:?}", &addr);

    let mut connection = TcpStream::connect(addr).await?;

    // tokio::spawn(async move {
    connection.write_all(args.as_bytes()).await?;

    tokio::time::sleep(Duration::from_secs(2)).await;
    // });

    // let json_payload = r#"{"tag":4,"length":8,"value":"lucy-service-daemon"}"#;

    // let mut input: &[u8] = &[
    //     0xB4, // Tag
    //     0x00, 0x13, // Length
    //     0x6c, 0x75, 0x63, 0x79, 0x2d, 0x73, 0x65, 0x72, 0x76, 0x69, // Value
    //     0x63, 0x65, 0x2d, 0x64, 0x61, 0x65, 0x6d, 0x6f, 0x6e,
    // ][..];

    Ok(())
}

// /// Converts an [`Args`] instance into an asynchronous [`Command`].
// ///
// /// The first value - arg0, or the program name itself - is skipped. This routine
// /// parses the remainder of the arguments passed in to the command line by the
// /// user. These arguments include things such as options, flags and subcommands.
// ///
// /// [`Args`]: https://doc.rust-lang.org/std/env/fn.args.html
// /// [`Command`]: https://docs.rs/tokio/latest/tokio/process/struct.Command.html
// fn parse_user_command(args: Args) -> Command {
//     let mut args = args.skip(1);
//     let namespace = args.next().expect("command should have a valid namespace");
//
//     let mut cmd = Command::new(namespace);
//     for arg in args {
//         cmd.arg(arg);
//     }
//
//     cmd
// }
