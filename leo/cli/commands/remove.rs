// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use leo_retriever::{Dependency, Manifest};
use std::path::PathBuf;

/// Remove a dependency from the current package.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Aleo Team <hello@aleo.org>", version)]
pub struct Remove {
    #[clap(
        name = "NAME",
        help = "The dependency name. Ex: `credits.aleo` or `credits`.",
        required_unless_present = "all"
    )]
    pub(crate) name: Option<String>,

    #[clap(short = 'l', long, help = "Path to local dependency")]
    pub(crate) local: Option<PathBuf>,

    #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet3")]
    pub(crate) network: String,

    #[clap(long, help = "Clear all previous dependencies.", default_value = "false")]
    pub(crate) all: bool,
}

impl Command for Remove {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let path = context.dir()?;

        // TODO: Dedup with Add Command. Requires merging utils/retriever/program_context with leo/package as both involve modifying the manifest.
        // Deserialize the manifest.
        let program_data: String = std::fs::read_to_string(path.join("program.json"))
            .map_err(|err| PackageError::failed_to_read_file(path.to_str().unwrap(), err))?;
        let manifest: Manifest = serde_json::from_str(&program_data)
            .map_err(|err| PackageError::failed_to_deserialize_manifest_file(path.to_str().unwrap(), err))?;

        let dependencies: Vec<Dependency> = if !self.all {
            // Make sure the program name is valid.
            // Allow both `credits.aleo` and `credits` syntax.
            let name: String = match &self.name {
                Some(name)
                    if name.ends_with(".aleo")
                        && Package::<CurrentNetwork>::is_program_name_valid(&name[0..name.len() - 5]) =>
                {
                    name.clone()
                }
                Some(name) if Package::<CurrentNetwork>::is_program_name_valid(name) => format!("{name}.aleo"),
                name => return Err(PackageError::invalid_file_name_dependency(name.clone().unwrap()).into()),
            };

            let mut found_match = false;
            let dep = match manifest.dependencies() {
                Some(ref dependencies) => dependencies
                    .iter()
                    .filter_map(|dependency| {
                        if dependency.name() == &name {
                            found_match = true;
                            let msg = match (dependency.path(), dependency.network()) {
                                (Some(local_path), _) => format!(
                                    "local dependency to `{}` from path `{}`",
                                    name,
                                    local_path.to_str().unwrap().replace('\"', "")
                                ),
                                (_, Some(network)) => {
                                    format!("network dependency to `{}` from network `{}`", name, network)
                                }
                                _ => format!("git dependency to `{name}`"),
                            };
                            tracing::warn!("✅ Successfully removed the {msg}.");
                            None
                        } else {
                            Some(dependency.clone())
                        }
                    })
                    .collect(),
                _ => Vec::new(),
            };

            // Throw error if no match is found.
            if !found_match {
                return Err(PackageError::dependency_not_found(name).into());
            }

            dep
        } else {
            Vec::new()
        };

        // Update the manifest file.
        let new_manifest = Manifest::new(
            manifest.program(),
            manifest.version(),
            manifest.description(),
            manifest.license(),
            Some(dependencies),
        );
        let new_manifest_data = serde_json::to_string_pretty(&new_manifest)
            .map_err(|err| PackageError::failed_to_serialize_manifest_file(path.to_str().unwrap(), err))?;
        std::fs::write(path.join("program.json"), new_manifest_data).map_err(PackageError::failed_to_write_manifest)?;

        Ok(())
    }
}
