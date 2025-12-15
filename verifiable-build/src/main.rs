use std::{fs::File, io::Read};

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use cmd_lib::{run_cmd, run_fun};
use solana_cli_config::{Config, CONFIG_FILE};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    pubkey::Pubkey,
};
use uuid::Uuid;

pub fn get_network(network_str: &str) -> &str {
    match network_str {
        "devnet" | "dev" | "d" => "https://api.devnet.solana.com",
        "mainnet" | "main" | "m" | "mainnet-beta" => "https://api.mainnet-beta.solana.com",
        "localnet" | "localhost" | "l" | "local" => "http://localhost:8899",
        _ => network_str,
    }
}

#[derive(Parser, Debug)]
#[clap(author = "Ellipsis", version, about)]
struct Arguments {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Build and verify a program built from a specific commit hash on a github repository
    VerifyFromRepo {
        /// Address of the program to verify
        #[clap(short, long)]
        program_id: Pubkey,
        /// Github Repo url, e.g. https://github.com/project-name/program-name
        #[clap(short = 'r', long)]
        repo_url: Option<String>,
        /// Path to program folder (e.g. programs/my-program if Anchor workspace). If not specified, will assume the repo root is the program folder.
        #[clap(short, long)]
        mount_path: Option<String>,
        /// Base image used to build the program. Default is projectserum/build:v0.26.0.
        #[clap(short, long)]
        base_image: Option<String>,
        /// Name of the binary
        #[clap(short, long)]
        name_of_program: String,
        /// Connection URL to Solana network to verify the on-chain program. Defaults to user global config
        #[clap(short, long)]
        connection_url: Option<String>,
        /// Commit hash that you want to verify against
        #[clap(short = 't', long)]
        commit_hash: Option<String>,
        /// Whether to use bpf or not
        #[clap(long)]
        bpf: bool,
    },
    /// Verify a program built from a provided Docker image
    VerifyFromImage {
        /// Path within the Docker image containing the binary to verify
        #[clap(short, long)]
        executable_path: String,
        /// Image that contains the source code to be verified
        #[clap(short, long)]
        image: String,
        /// Connection URL to Solana network to verify the on-chain program. Defaults to user global config
        #[clap(short, long)]
        url: Option<String>,
        /// The Program ID of the program to verify
        #[clap(short, long)]
        program_id: Pubkey,
    },
    /// Get the executable hash of a program binary
    GetHash {
        /// Path to program binary
        filepath: String,
    },
    /// Get the hash of a program binary from the deployed on-chain program
    GetProgramHash {
        /// Connection URL to Solana network to verify the on-chain program. Defaults to user global config
        #[clap(short, long)]
        url: Option<String>,
        /// The Program ID of the program to verify
        program_id: Pubkey,
    },
    /// Get the hash of a program binary from the deployed buffer address
    GetBufferHash {
        /// Connection URL to Solana network to verify the on-chain program. Defaults to user global config
        #[clap(short, long)]
        url: Option<String>,
        /// Address of the buffer account containing the deployed program data
        buffer_address: Pubkey,
    },
    /// Build the program from a github repository
    Build {
        /// Github Repo url, e.g. https://github.com/project-name/program-name
        #[clap(short = 'r', long)]
        repo_url: Option<String>,
        /// Path to program folder (e.g. programs/my-program if Anchor workspace). If not specified, will assume the repo root is the program folder.
        #[clap(short, long)]
        mount_path: Option<String>,
        /// Base image used to build the program. Default is projectserum/build:v0.26.0.
        #[clap(short, long)]
        base_image: Option<String>,
        /// Commit hash that you want to verify against
        #[clap(short = 't', long)]
        commit_hash: Option<String>,
        /// Whether to use bpf or not
        #[clap(long)]
        bpf: bool,
    },
}

pub fn get_client(url: Option<String>) -> RpcClient {
    let config = match CONFIG_FILE.as_ref() {
        Some(config_file) => Config::load(config_file).unwrap_or_else(|_| {
            println!("Failed to load config file: {}", config_file);
            Config::default()
        }),
        None => Config::default(),
    };
    let url = &get_network(&url.unwrap_or(config.json_rpc_url)).to_string();
    RpcClient::new(url)
}

fn get_binary_hash(program_data: Vec<u8>) -> String {
    // Hash the program data directly
    sha256::digest(&program_data[..])
}

pub fn get_file_hash(filepath: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(get_binary_hash(buffer))
}

pub fn get_buffer_hash(url: Option<String>, buffer_address: Pubkey) -> anyhow::Result<String> {
    let client = get_client(url);
    let offset = UpgradeableLoaderState::size_of_buffer_metadata();
    let account_data = client.get_account_data(&buffer_address)?[offset..].to_vec();
    let program_hash = get_binary_hash(account_data);
    Ok(program_hash)
}

pub fn get_program_hash(url: Option<String>, program_id: Pubkey) -> anyhow::Result<String> {
    let client = get_client(url);
    let program_buffer =
        Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable::id()).0;
    let offset = UpgradeableLoaderState::size_of_programdata_metadata();
    let account_data = client.get_account_data(&program_buffer)?[offset..].to_vec();
    let program_hash = get_binary_hash(account_data);
    Ok(program_hash)
}

pub fn build(
    filepath: Option<String>,
    base_image: Option<String>,
    commit_hash: Option<String>,
    bpf_flag: bool,
) -> anyhow::Result<()> {
    let base_image = base_image.unwrap_or("projectserum/build:v0.26.0".to_string());
    let mount_path = filepath.unwrap_or(".".to_string());
    let source_dir = std::env::current_dir()?.to_str().unwrap().to_string();
    let uuid = Uuid::new_v4();

    run_cmd!(
        docker run -v $source_dir:/code --name $uuid $base_image bash -c
        "cd /code && git submodule update --init --recursive";
    )?;

    if let Some(commit_hash) = commit_hash {
        run_cmd!(
            docker run -v $source_dir:/code --name $uuid $base_image bash -c
            "cd /code && git checkout $commit_hash";
        )?;
    }

    let build_cmd = if bpf_flag {
        format!("cd /code/{mount_path} && cargo build-bpf")
    } else {
        format!("cd /code/{mount_path} && cargo build-sbf")
    };

    run_cmd!(
        docker run -v $source_dir:/code --name $uuid $base_image bash -c $build_cmd;
    )?;

    println!("Build completed successfully!");
    Ok(())
}

pub fn verify_from_image(
    executable_path: String,
    image: String,
    network: Option<String>,
    program_id: Pubkey,
) -> anyhow::Result<()> {
    println!(
        "Verifying image: {} on network {:?} for program ID: {}",
        image, network, program_id
    );

    let container_id = run_fun!(docker create $image)?;
    run_cmd!(docker cp $container_id:/build/$executable_path /tmp/program.so)?;

    let executable_hash = get_file_hash("/tmp/program.so")?;
    let client = get_client(network);
    let program_buffer =
        Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable::id()).0;
    let offset = UpgradeableLoaderState::size_of_programdata_metadata();
    let account_data = client.get_account_data(&program_buffer)?[offset..].to_vec();
    let program_hash = get_binary_hash(account_data);

    if executable_hash == program_hash {
        println!("Executable matches on-chain program data ✅");
        println!("Executable hash: {}", executable_hash);
        Ok(())
    } else {
        Err(anyhow!(
            "Executable does not match on-chain program data ❌\nExecutable hash: {}\nOn-chain program hash: {}",
            executable_hash,
            program_hash
        ))
    }
}

pub fn verify_from_repo(
    program_id: Pubkey,
    filepath: Option<String>,
    base_image: Option<String>,
    bpf_flag: bool,
    name_of_program: String,
    connection_url: Option<String>,
    commit_hash: Option<String>,
) -> anyhow::Result<(String, String)> {
    // Build the code using the docker container
    build(filepath, base_image, commit_hash, bpf_flag)?;

    let source_dir = std::env::current_dir()?.to_str().unwrap().to_string();
    // Both BPF and SBF build to the same target/deploy directory in modern Solana
    let build_path = format!("{}/target/deploy/{}.so", source_dir, name_of_program);

    // Get the hash of the compiled program
    println!(
        "Getting hash of the compiled program at path: {}",
        build_path
    );
    let executable_hash = get_file_hash(&build_path)?;

    // Get the hash of the deployed program
    println!(
        "Fetching on-chain program data for program ID: {}",
        program_id,
    );
    let program_hash = get_program_hash(connection_url, program_id)?;

    Ok((executable_hash, program_hash))
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    match args.subcommand {
        SubCommand::VerifyFromRepo {
            program_id,
            repo_url: _,
            mount_path,
            base_image,
            name_of_program,
            connection_url,
            commit_hash,
            bpf,
        } => {
            let (executable_hash, program_hash) = verify_from_repo(
                program_id,
                mount_path,
                base_image,
                bpf,
                name_of_program,
                connection_url,
                commit_hash,
            )?;

            if executable_hash == program_hash {
                println!("Executable matches on-chain program data ✅");
                println!("Executable Program Hash: {}", executable_hash);
            } else {
                return Err(anyhow!(
                    "Executable does not match on-chain program data ❌\nExecutable hash: {}\nOn-chain program hash: {}",
                    executable_hash,
                    program_hash
                ));
            }
        }
        SubCommand::VerifyFromImage {
            executable_path,
            image,
            url,
            program_id,
        } => {
            verify_from_image(executable_path, image, url, program_id)?;
        }
        SubCommand::GetHash { filepath } => {
            let hash = get_file_hash(&filepath)?;
            println!("Executable Program Hash: {}", hash);
        }
        SubCommand::GetProgramHash { url, program_id } => {
            let hash = get_program_hash(url, program_id)?;
            println!("On-chain Program Hash: {}", hash);
        }
        SubCommand::GetBufferHash {
            url,
            buffer_address,
        } => {
            let hash = get_buffer_hash(url, buffer_address)?;
            println!("Buffer Hash: {}", hash);
        }
        SubCommand::Build {
            repo_url: _,
            mount_path,
            base_image,
            commit_hash,
            bpf,
        } => {
            build(mount_path, base_image, commit_hash, bpf)?;
        }
    }

    Ok(())
}
