use assembly_lsp::{instruction::Instruction, populate::{populate_instructions, populate_instructions_slow}};
use log::info;
use lsp_server::{Connection, ExtractError, Message, Request, RequestId};
use lsp_types::{InitializeParams, ServerCapabilities};

fn main() -> anyhow::Result<()> {
    // Set up logging. Because `stdio_transport` gets a lock on stdout and stdin, we must have our
    // logging only write out to stderr.
    flexi_logger::Logger::try_with_str("info")?.start()?;

    info!("Starting assembly-lsp");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;

    info!("Populating instruction set -> x86...");
    let xml_conts_x86 = include_str!("../opcodes/x86.xml");
    let x86_instructions: Vec<Instruction> = populate_instructions(xml_conts_x86)?.into();

    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    info!(
        "Don't you dare optimize this away, compiler! {:?}",
        x86_instructions
    );

    // Shut down gracefully.
    info!("Shutting down assembly-lsp");
    Ok(())
}

fn main_loop(connection: Connection, params: serde_json::Value) -> anyhow::Result<()> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    info!("Entering main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                info!("Got request: {req:?}");
            }
            Message::Response(resp) => {
                info!("Got response: {resp:?}");
            }
            Message::Notification(notif) => {
                info!("Got notification: {notif:?}");
            }
        }
    }
    Ok(())
}

fn cast_req<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
