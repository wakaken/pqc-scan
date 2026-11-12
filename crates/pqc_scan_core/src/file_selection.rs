//! Declared pre-inspection policy; cryptographic containers are never lumped in
//! with media/archives. The opt-out permits explicit forensic/custom-detector use.
use std::path::Path;
#[derive(Debug, Clone, Copy, Default)]
pub struct FileSelection {
    pub include_non_scannable: bool,
}
pub(crate) const POLICY_LABEL: &str = "known non-scannable media, archives and binary data";
pub(crate) fn known_extension(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    matches!(
        ext.as_str(),
        "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "bmp"
            | "ico"
            | "webp"
            | "tif"
            | "tiff"
            | "heic"
            | "heif"
            | "avif"
            | "jxl"
            | "jp2"
            | "icns"
            | "psd"
            | "dds"
            | "cur"
            | "tga"
            | "exr"
            | "ktx"
            | "j2c"
            | "pfm"
            | "glb"
            | "mp3"
            | "mp4"
            | "wav"
            | "webm"
            | "mov"
            | "avi"
            | "ogg"
            | "oga"
            | "flac"
            | "woff"
            | "woff2"
            | "ttf"
            | "otf"
            | "eot"
            | "zip"
            | "gz"
            | "tgz"
            | "tar"
            | "xz"
            | "lzma"
            | "bz2"
            | "zst"
            | "z"
            | "br"
            | "lz4"
            | "7z"
            | "rar"
            | "jar"
            | "war"
            | "ear"
            | "egg"
            | "whl"
            | "wasm"
            | "exe"
            | "dll"
            | "so"
            | "dylib"
            | "o"
            | "obj"
            | "class"
            | "pyc"
            | "pyc-tpl"
            | "mo"
            | "pb"
            | "binpb"
            | "desc"
            | "pdf"
    )
}
pub(crate) fn protects_crypto(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    matches!(
        ext.as_str(),
        "pem"
            | "crt"
            | "cer"
            | "der"
            | "p12"
            | "pfx"
            | "pkcs12"
            | "jks"
            | "jceks"
            | "bcfks"
            | "keystore"
            | "truststore"
            | "keytab"
            | "kbx"
            | "key"
            | "pub"
            | "asc"
    )
}
pub(crate) fn non_text(path: &Path, bytes: &[u8]) -> bool {
    !protects_crypto(path)
        && !crate::text_encoding::has_wide_bom(bytes)
        && crate::text_encoding::decode(bytes).is_none()
}
