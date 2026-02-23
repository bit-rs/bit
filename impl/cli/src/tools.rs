// TODO: Move it into `common` crate?

use clap::ValueEnum;

#[derive(Copy, Clone, ValueEnum)]
pub enum Platform {
    Windows,
    Linux,
    Macos,

    // TODO/FIXME: Combile all of *BSD into `BSD` field?
    Freebsd,
    Openbsd,
    Netbsd,
}

impl Platform {
    pub const fn native() -> Self {
        #[cfg(target_os = "windows")]
        { Platform::Windows }

        #[cfg(target_os = "linux")]
        { Platform::Linux }

        #[cfg(target_os = "macos")]
        { Platform::Macos }

        #[cfg(target_os = "freebsd")]
        { Platform::Freebsd }

        #[cfg(target_os = "openbsd")]
        { Platform::Openbsd }

        #[cfg(target_os = "netbsd")]
        { Platform::Netbsd }
    }
}

pub fn platform_to_exe_extension(platform: Platform) -> &'static str {
    match platform {
        Platform::Windows => ".exe",
        Platform::Linux | Platform::Macos | Platform::Freebsd | Platform::Openbsd | Platform::Netbsd => "",
    }
}

pub fn source_file_path_to_binary_path(filepath: &str, platform: Platform) -> String {
    let pieces = filepath.rsplit_once('.');

    let raw_binary_path = pieces.map(|(filename, _extension)| filename).unwrap_or(&filepath).to_owned();

    raw_binary_path + platform_to_exe_extension(platform)
}