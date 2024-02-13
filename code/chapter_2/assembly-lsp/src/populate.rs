use crate::instruction::{Operand, OperandType, Instruction, InstructionForm};
use std::collections::HashMap;

use std::str;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

use anyhow::anyhow;
use log::debug;

/// Takes xml file contents and converts it into a Vec<Instruction>
pub fn populate_instructions(xml_contents: &str) -> anyhow::Result<HashMap<String, Instruction>> {
    let mut instructions_map = HashMap::<String, Instruction>::new();

    let mut reader = Reader::from_str(xml_contents);
    reader.trim_text(true);

    // instruction and instruction form that are currently under construction
    let mut curr_instruction = Instruction::default();
    let mut curr_instruction_form = InstructionForm::default();

    debug!("Parsing XML contents...");
    loop {
        match reader.read_event() {
            // start event ------------------------------------------------------------------------
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    QName(b"Instruction") => {
                        // start of a new instruction
                        curr_instruction = Instruction::default();

                        // iterate over the attributes
                        for attr in e.attributes() {
                            let Attribute { key, value } = attr.unwrap();
                            match str::from_utf8(key.into_inner()).unwrap() {
                                "name" => unsafe {
                                    let name = String::from(str::from_utf8_unchecked(&value).to_lowercase());
                                    curr_instruction.name = name;
                                },
                                "summary" => unsafe {
                                    curr_instruction.summary =
                                        String::from(str::from_utf8_unchecked(&value));
                                },
                                _ => {}
                            }
                        }
                    }
                    QName(b"InstructionForm") => {
                        // new instruction form 
                        curr_instruction_form = InstructionForm::default();

                        // iterate over the attributes
                        for attr in e.attributes() {
                            let Attribute { key, value } = attr.unwrap();
                            match str::from_utf8(key.into_inner()).unwrap() {
                                "gas-name" => unsafe {
                                    curr_instruction_form.gas_name =
                                        Some(String::from(str::from_utf8_unchecked(&value).to_lowercase()));
                                },
                                "go-name" => unsafe {
                                    curr_instruction_form.go_name =
                                        Some(String::from(str::from_utf8_unchecked(&value).to_lowercase()));
                                },
                                _ => {}
                            }
                        }
                    }
                    QName(b"Encoding") => {} // TODO
                    _ => (),                 // unknown event
                }
            }
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    QName(b"Operand") => {
                        let mut op_type = OperandType::k; // dummy initialisation
                        let mut input = None;
                        let mut output = None;

                        for attr in e.attributes() {
                            let Attribute { key, value } = attr.unwrap();
                            match str::from_utf8(key.into_inner()).unwrap() {
                                "type" => {
                                    op_type = match OperandType::from_str(str::from_utf8(&value)?) {
                                        Ok(op_type) => op_type,
                                        Err(_) => {
                                            return Err(anyhow!(
                                                "Unknown value for operand type -- Variant: {}",
                                                str::from_utf8(&value)?
                                            ));
                                        }
                                    }
                                }
                                "input" => match str::from_utf8(&value).unwrap() {
                                    "true" => input = Some(true),
                                    "false" => input = Some(false),
                                    _ => return Err(anyhow!("Unknown value for operand type")),
                                },
                                "output" => match str::from_utf8(&value).unwrap() {
                                    "true" => output = Some(true),
                                    "false" => output = Some(false),
                                    _ => return Err(anyhow!("Unknown value for operand type")),
                                },
                                _ => (), // unknown event
                            }
                        }

                        curr_instruction_form.operands.push(Operand {
                            op_type,
                            input,
                            output,
                        })
                    }
                    _ => (), // unknown event
                }
            }
            // end event --------------------------------------------------------------------------
            Ok(Event::End(ref e)) => {
                match e.name() {
                    QName(b"Instruction") => {
                        // finish instruction
                        instructions_map
                            .insert(curr_instruction.name.clone(), curr_instruction.clone());
                    }
                    QName(b"InstructionForm") => {
                        curr_instruction.forms.push(curr_instruction_form.clone());
                    }
                    _ => (), // unknown event
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // rest of events that we don't consider
        }
    }

    Ok(instructions_map)
}
