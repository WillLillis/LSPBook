use std::collections::HashMap;

use assembly_lsp::{
    instruction::Instruction, lsp::get_word_from_file_params, populate::populate_instructions,
};
use log::info;
use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::{
    request::HoverRequest, Hover, HoverContents, InitializeParams, MarkupContent, MarkupKind,
    ServerCapabilities,
};

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
    let x86_instrs = populate_instructions(xml_conts_x86)?;

    main_loop(connection, initialization_params, x86_instrs)?;
    io_threads.join()?;

    // Shut down gracefully.
    info!("Shutting down assembly-lsp");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    x86_instrs: HashMap<String, Instruction>,
) -> anyhow::Result<()> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    info!("Entering main loop");
    for msg in &connection.receiver {
        info!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                info!("Got request: {req:?}");
                if let Ok((id, params)) = cast_req::<HoverRequest>(req) {
                    if let Ok(word) =
                        get_word_from_file_params(&params.text_document_position_params)
                    {
                        let resp = match x86_instrs.get(&word) {
                            Some(instr) => Some(Hover {
                                contents: HoverContents::Markup(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: "Hello, LSP!".to_string(), //format!("{}", instr),
                                }),
                                range: None,
                            }),
                            None => None,
                        };
                        let result = serde_json::to_value(&resp).unwrap();
                        let result = Response {
                            id: id.clone(),
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(result))?;
                    }
                }
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
