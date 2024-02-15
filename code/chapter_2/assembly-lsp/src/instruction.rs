use std::fmt::Display;

use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone, Default)]
pub struct Instruction {
    pub name: String,
    pub summary: String,
    pub arch: Option<Arch>,
    pub forms: Vec<InstructionForm>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header: String;
        if let Some(arch) = &self.arch {
            header = format!("{} [{}]", &self.name, arch.as_ref());
        } else {
            header = self.name.clone();
        }

        let mut v: Vec<&str> = vec![&header, &self.summary, "\n", "## Forms", "\n"];

        // instruction forms
        let instruction_form_strs: Vec<String> =
            self.forms.iter().map(|f| format!("{}", f)).collect();
        for item in instruction_form_strs.iter() {
            v.push(item.as_str());
        }

        let s = v.join("\n");
        write!(f, "{}", s)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct InstructionForm {
    pub gas_name: Option<String>,
    pub go_name: Option<String>,
    pub encoding: String,
    pub operands: Vec<Operand>,
}

impl Display for InstructionForm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if let Some(val) = &self.gas_name {
            s += &format!("*GAS*: {} | ", val);
        }
        if let Some(val) = &self.go_name {
            s += &format!("*GO*: {} | ", val);
        }

        // get rid of trailing " | "
        if !s.is_empty() {
            s = format!("- {}\n\n", &s[..s.len() - 3]);
        }

        // Operands
        let operands_str: String = self
            .operands
            .iter()
            .map(|op| {
                let mut s = format!("  + {:<8}", format!("[{}]", op.op_type.as_ref()));
                if let Some(input) = op.input {
                    s += &format!(" input = {:<5} ", input)
                }
                if let Some(output) = op.output {
                    s += &format!(" output = {:<5}", output)
                }

                s
            })
            .collect::<Vec<String>>()
            .join("\n");
        s = s + &operands_str + "\n";

        write!(f, "{}", s)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Operand {
    pub op_type: OperandType,
    pub input: Option<bool>,
    pub output: Option<bool>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, EnumString, AsRefStr, Default)]
pub enum Arch {
    x86,
    #[default]
    x86_64,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, EnumString, AsRefStr)]
pub enum OperandType {
    #[strum(serialize = "1")]
    _1,
    #[strum(serialize = "3")]
    _3,
    imm4,
    imm8,
    imm16,
    imm32,
    imm64,
    al,
    cl,
    r8,
    r8l,
    ax,
    r16,
    r16l,
    eax,
    r32,
    r32l,
    rax,
    r64,
    mm,
    xmm0,
    xmm,
    #[strum(serialize = "xmm{k}")]
    xmm_k,
    #[strum(serialize = "xmm{k}{z}")]
    xmm_k_z,
    ymm,
    #[strum(serialize = "ymm{k}")]
    ymm_k,
    #[strum(serialize = "ymm{k}{z}")]
    ymm_k_z,
    zmm,
    #[strum(serialize = "zmm{k}")]
    zmm_k,
    #[strum(serialize = "zmm{k}{z}")]
    zmm_k_z,
    k,
    #[strum(serialize = "k{k}")]
    k_k,
    moffs32,
    moffs64,
    m,
    m8,
    m16,
    #[strum(serialize = "m16{k}")]
    m16_k,
    #[strum(serialize = "m16{k}{z}")]
    m16_k_z,
    m32,
    #[strum(serialize = "m32{k}")]
    m32_k,
    #[strum(serialize = "m32{k}{z}")]
    m32_k_z,
    #[strum(serialize = "m32/m16bcst")]
    m32_m16bcst,
    m64,
    #[strum(serialize = "m64{k}")]
    m64_k,
    #[strum(serialize = "m64{k}{z}")]
    m64_k_z,
    #[strum(serialize = "m64/m16bcst")]
    m64_m16bcst,
    m128,
    #[strum(serialize = "m128{k}")]
    m128_k,
    #[strum(serialize = "m128{k}{z}")]
    m128_k_z,
    m256,
    #[strum(serialize = "m256{k}")]
    m256_k,
    #[strum(serialize = "m256{k}{z}")]
    m256_k_z,
    m512,
    #[strum(serialize = "m512{k}")]
    m512_k,
    #[strum(serialize = "m512{k}{z}")]
    m512_k_z,
    #[strum(serialize = "m64/m32bcst")]
    m64_m32bcst,
    #[strum(serialize = "m128/m32bcst")]
    m128_m32bcst,
    #[strum(serialize = "m256/m32bcst")]
    m256_m32bcst,
    #[strum(serialize = "m512/m32bcst")]
    m512_m32bcst,
    #[strum(serialize = "m128/m16bcst")]
    m128_m16bcst,
    #[strum(serialize = "m128/m64bcst")]
    m128_m64bcst,
    #[strum(serialize = "m256/m16bcst")]
    m256_m16bcst,
    #[strum(serialize = "m256/m64bcst")]
    m256_m64bcst,
    #[strum(serialize = "m512/m16bcst")]
    m512_m16bcst,
    #[strum(serialize = "m512/m64bcst")]
    m512_m64bcst,
    vm32x,
    #[strum(serialize = "vm32x{k}")]
    vm32x_k,
    vm64x,
    #[strum(serialize = "vm64x{k}")]
    vm64xk,
    vm32y,
    #[strum(serialize = "vm32y{k}")]
    vm32yk_,
    vm64y,
    #[strum(serialize = "vm64y{k}")]
    vm64y_k,
    vm32z,
    #[strum(serialize = "vm32z{k}")]
    vm32z_k,
    vm64z,
    #[strum(serialize = "vm64z{k}")]
    vm64z_k,
    rel8,
    rel32,
    #[strum(serialize = "{er}")]
    er,
    #[strum(serialize = "{sae}")]
    sae,
    sibmem,
    tmm,
}
