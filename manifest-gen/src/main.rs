use std::path::Path;
use base64::prelude::*;
use anyhow::{Result, anyhow};
use clap::{Parser, ValueEnum};
use clap_num::{maybe_hex};

#[derive(Debug, Clone, ValueEnum)]
enum ChipFamily {
    ESP32,
    ESP32C3,
    ESP32S3,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    name: String,
    #[arg(long)]
    version: String,
    #[arg(long, default_value = "esp32")]
    chip_family: ChipFamily,
    #[arg(long)]
    part_data: Vec<String>,
    #[arg(long, value_parser=maybe_hex::<u32>)]
    part_offset: Vec<u32>,
    #[arg(long)]
    output: Option<String>,
}

struct PartData {
    b64_data: String,
    offset: u32,
}

fn load_part_data(path: &Path, offset: u32) -> Result<PartData> {
    let data = std::fs::read(path).map_err(|e| anyhow!("Fail to read partition data {}", e))?;
    let b64_data = BASE64_STANDARD.encode(&data);
    Ok(PartData {
        b64_data,
        offset,
    })
}

fn write_manifest_json<W: std::io::Write>(writer: &mut W, parts: &Vec<PartData>, args: &Cli) -> Result<()> {
    writeln!(writer, "{{")?;
    writeln!(writer, "\"name\":\"{}\",", args.name.escape_default())?;
    writeln!(writer, "\"version\":\"{}\",",  args.version.escape_default())?;
    writeln!(writer, "\"new_install_prompt_erase\":true,")?;
    writeln!(writer, "\"builds\":[")?;
    writeln!(writer, "{{")?;
    let chip_family_str = match args.chip_family {
        ChipFamily::ESP32 => "ESP32",
        ChipFamily::ESP32C3 => "ESP32-C3",
        ChipFamily::ESP32S3 => "ESP32-S3",
    };
    writeln!(writer, "\"chipFamily\":\"{}\",", chip_family_str)?;
    writeln!(writer, "\"parts\":[")?;
    for (index, part) in parts.iter().enumerate() {
        writeln!(writer, "{{")?;
        writeln!(writer, "\"data\":\"{}\",", &part.b64_data)?;
        let delimiter = if index == parts.len() - 1 { "" } else { "," };
        writeln!(writer, "\"offset\":\"{}\"", part.offset)?;
        writeln!(writer, "}}{}", delimiter)?;

    }
    writeln!(writer, "]")?;     // parts
    writeln!(writer, "}}")?;    // builds[]
    writeln!(writer, "]")?;     // builds
    writeln!(writer, "}}")?;    // <root>
    Ok(())
}

fn main() {
    let args = Cli::parse();
    if args.part_data.len() == 0 {
        panic!("At least one partition data must be specified by `--part-data` argument.");
    }
    if args.part_data.len() != args.part_offset.len() {
        panic!("Number of --part-data arguments and --part-offset arguments are mismatch.");
    }

    let parts: Vec<PartData> = args.part_data.iter().zip(args.part_offset.iter()).map(|(data_path, offset)| load_part_data(&Path::new(data_path), *offset).unwrap()).collect();
    match &args.output {
        Some(output) => write_manifest_json(&mut std::fs::File::create(&Path::new(output.as_str())).unwrap(), &parts, &args).unwrap(),
        None => write_manifest_json(&mut std::io::stdout(), &parts, &args).unwrap(),
    }
}
