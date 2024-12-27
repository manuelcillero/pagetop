use pagetop_build::StaticFilesBundle;

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const BUILD_STATE_FILE: &str = "./static/build_state.txt";

fn main() -> io::Result<()> {
    // Reconstruye la documentación.
    rebuild_doc()?;

    // Crea la colección de archivos estáticos.
    StaticFilesBundle::from_dir(
        "./static/doc",
        Some(pagetop_mdbook::build::except_common_resources),
    )
    .with_name("doc")
    .build()
}

/// Reconstruye la documentación que ha cambiado para cada versión y sus idiomas.
fn rebuild_doc() -> io::Result<()> {
    // Lee el estado de la última preparación.
    let mut build_state = read_build_state()?;

    // Define la ruta al directorio `doc`.
    let project_dir = env::var("CARGO_MANIFEST_DIR").map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Environment variable error: {}", e),
        )
    })?;
    let doc_dir = Path::new(&project_dir).join("doc");

    // Itera sobre las versiones de la documentación en `doc`.
    for entry in fs::read_dir(&doc_dir)? {
        let entry = entry?;
        let version_dir = entry.path();

        if version_dir.is_dir() && version_dir.file_name().unwrap_or_default() != "theme" {
            let version_name = version_dir
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            // Itera sobre los idiomas dentro de la versión.
            for lang_entry in fs::read_dir(&version_dir)? {
                let lang_entry = lang_entry?;
                let lang_dir = lang_entry.path();

                if lang_dir.is_dir() {
                    let lang_name = lang_dir.file_name().unwrap().to_string_lossy().to_string();

                    // Obtiene la última marca de tiempo de cambios en archivos para el idioma.
                    let latest_change_time = get_latest_change_time(&lang_dir)?;

                    // Obtiene la última marca de tiempo registrada para esta versión e idioma.
                    let state_key = format!("{}/{}", version_name, lang_name);
                    let last_build_time =
                        build_state.get(&state_key).copied().unwrap_or(UNIX_EPOCH);

                    // Si hay cambios, construye el libro para este idioma.
                    if latest_change_time > last_build_time {
                        build_mdbook(&lang_dir)?;
                        build_state.insert(state_key, SystemTime::now());
                    }
                }
            }
        }
    }

    // Guarda el estado actualizado.
    write_build_state(&build_state)?;

    Ok(())
}

/// Construye el libro mdBook para una versión específica.
fn build_mdbook(version_dir: &Path) -> io::Result<()> {
    if !Command::new("mdbook")
        .arg("build")
        .current_dir(version_dir)
        .status()?
        .success()
    {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to build mdBook in {:?}", version_dir),
        ));
    }
    Ok(())
}

/// Lee el estado de construcción desde un archivo de texto plano.
fn read_build_state() -> io::Result<HashMap<String, SystemTime>> {
    let mut build_state = HashMap::new();
    let path = Path::new(BUILD_STATE_FILE);

    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(timestamp) = parts[1].parse::<u64>() {
                    build_state.insert(
                        parts[0].to_string(),
                        UNIX_EPOCH + std::time::Duration::from_secs(timestamp),
                    );
                }
            }
        }
    }

    Ok(build_state)
}

/// Escribe el estado de construcción en un archivo de texto plano.
fn write_build_state(build_state: &HashMap<String, SystemTime>) -> io::Result<()> {
    let path = Path::new(BUILD_STATE_FILE);

    // Crea el directorio `static` si no existe.
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = File::create(path)?;

    for (version, &time) in build_state {
        let timestamp = time
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Invalid system time: {}", e))
            })?
            .as_secs();
        writeln!(file, "{} {}", version, timestamp)?;
    }

    Ok(())
}

/// Obtiene la marca de tiempo más reciente de los archivos de un directorio usando recursión.
fn get_latest_change_time(dir: &Path) -> io::Result<SystemTime> {
    let mut latest = UNIX_EPOCH;

    /// Función recursiva para recorrer directorios.
    fn visit_dir(dir: &Path, latest: &mut SystemTime) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                // Recurse en los subdirectorios.
                visit_dir(&entry.path(), latest)?;
            } else if let Ok(modified) = metadata.modified() {
                // Actualiza la última marca de tiempo.
                if modified > *latest {
                    *latest = modified;
                }
            }
        }
        Ok(())
    }

    visit_dir(dir, &mut latest)?;
    Ok(latest)
}
