use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use rustbasic_core::colored::Colorize;

pub fn remove_native_scaffolding() {
    println!("🗑️  {}", "Membersihkan scaffolding RustBasic Native...".bold());

    // 1. Hapus folder native/
    if Path::new("native").exists() {
        if let Err(e) = fs::remove_dir_all("native") {
            println!("   {} Gagal menghapus folder native/: {}", "⚠️".yellow(), e);
        } else {
            println!("   {} Menghapus folder native/", "🗑️".red().bold());
        }
    }

    // 2. Hapus native-bridge.ts
    let bridge_path = Path::new("src/resources/js/native-bridge.ts");
    if bridge_path.exists() {
        fs::remove_file(bridge_path).ok();
        println!("   {} Menghapus file {}", "🗑️".red().bold(), bridge_path.display());
    }

    // 3. Hapus Permission.tsx
    let permission_path = Path::new("src/resources/js/Pages/Permission.tsx");
    if permission_path.exists() {
        fs::remove_file(permission_path).ok();
        println!("   {} Menghapus file {}", "🗑️".red().bold(), permission_path.display());
    }

    // 3.1 Hapus reverb.ts
    let reverb_path = Path::new("src/resources/js/reverb.ts");
    if reverb_path.exists() {
        fs::remove_file(reverb_path).ok();
        println!("   {} Menghapus file {}", "🗑️".red().bold(), reverb_path.display());
    }

    // 3.2 Hapus docs/websocket.md
    let ws_doc_path = Path::new("docs/websocket.md");
    if ws_doc_path.exists() {
        fs::remove_file(ws_doc_path).ok();
        println!("   {} Menghapus file {}", "🗑️".red().bold(), ws_doc_path.display());
    }

    // 4. Hapus setup_native!() dari src/lib.rs
    let lib_path = Path::new("src/lib.rs");
    if lib_path.exists() {
        if let Ok(content) = fs::read_to_string(lib_path) {
            let search_block = "\n// Native (Mobile & Desktop) Entry Point\n#[cfg(any(target_os = \"android\", target_os = \"ios\"))]\nrustbasic_native::setup_native!();";
            let search_block_alt = "// Native (Mobile & Desktop) Entry Point\n#[cfg(any(target_os = \"android\", target_os = \"ios\"))]\nrustbasic_native::setup_native!();";
            let mut updated = content.replace(search_block, "");
            updated = updated.replace(search_block_alt, "");
            fs::write(lib_path, updated).ok();
            println!("   {} Membersihkan entry point di {}", "📝".blue(), lib_path.display());
        }
    }

    // 5. Hapus import bridge dari main.tsx
    let entry_paths = [
        "src/resources/js/main.tsx",
        "src/resources/js/main.ts",
        "src/resources/js/main.jsx",
        "src/resources/js/main.js",
    ];
    for path in &entry_paths {
        let p = Path::new(path);
        if p.exists() {
            if let Ok(content) = fs::read_to_string(p) {
                let mut updated = content.replace("import './native-bridge';\n", "");
                updated = updated.replace("import './native-bridge';\r\n", "");
                updated = updated.replace("import \"./native-bridge\";\n", "");
                updated = updated.replace("import \"./native-bridge\";\r\n", "");
                fs::write(p, updated).ok();
                println!("   {} Membersihkan import native-bridge di {}", "📝".blue(), path);
            }
        }
    }

    // 6. Hapus navigasi Izin dari AppLayout.tsx
    let layout_path = Path::new("src/resources/js/Layouts/AppLayout.tsx");
    if layout_path.exists() {
        if let Ok(content) = fs::read_to_string(layout_path) {
            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            let mut perm_idx = None;
            for (i, line) in lines.iter().enumerate() {
                if line.contains("href={route('permission')}") {
                    perm_idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = perm_idx {
                let mut start_idx = None;
                for i in (0..=idx).rev() {
                    if lines[i].contains("<Link") {
                        start_idx = Some(i);
                        break;
                    }
                }

                let mut end_idx = None;
                for i in idx..lines.len() {
                    if lines[i].contains("</Link>") {
                        end_idx = Some(i);
                        break;
                    }
                }

                if let (Some(s_idx), Some(e_idx)) = (start_idx, end_idx) {
                    println!("   {} Membersihkan navigasi Izin di {}", "📝".blue(), layout_path.display());
                    lines.drain(s_idx..=e_idx);
                    
                    let has_crlf = content.contains("\r\n");
                    let joiner = if has_crlf { "\r\n" } else { "\n" };
                    let updated = lines.join(joiner) + joiner;
                    fs::write(layout_path, updated).ok();
                }
            }
        }
    }
    
    // 7. Hapus route permission dari src/routes/web.rs
    let routes_path = Path::new("src/routes/web.rs");
    if routes_path.exists() {
        if let Ok(content) = fs::read_to_string(routes_path) {
            let search_route = "\n        .route(\"/permission\", get(welcome_controller::permission)).name(\"permission\")";
            let search_route_alt = ".route(\"/permission\", get(welcome_controller::permission)).name(\"permission\")";
            let mut updated = content.replace(search_route, "");
            updated = updated.replace(search_route_alt, "");
            fs::write(routes_path, updated).ok();
            println!("   {} Membersihkan route permission di {}", "📝".blue(), routes_path.display());
        }
    }

    // 8. Hapus controller permission dari src/app/http/controllers/welcome_controller.rs
    let controller_path = Path::new("src/app/http/controllers/welcome_controller.rs");
    if controller_path.exists() {
        if let Ok(content) = fs::read_to_string(controller_path) {
            let search_fn = "\n\npub async fn permission(req: Request) -> impl IntoResponse {\n    inertia(&req, \"Permission\", json!({}))\n}";
            let search_fn_alt = "pub async fn permission(req: Request) -> impl IntoResponse {\n    inertia(&req, \"Permission\", json!({}))\n}";
            let mut updated = content.replace(search_fn, "");
            updated = updated.replace(search_fn_alt, "");
            fs::write(controller_path, updated).ok();
            println!("   {} Membersihkan controller permission di {}", "📝".blue(), controller_path.display());
        }
    }

    // 9. Bersihkan target binary dan dependensi tambahan di Cargo.toml
    let cargo_path = Path::new("Cargo.toml");
    if cargo_path.exists() {
        if let Ok(mut content) = fs::read_to_string(cargo_path) {
            let bin_target = "\n\n[[bin]]\nname = \"rustbasic-desktop\"\npath = \"native/desktop/src/main.rs\"\nrequired-features = [\"desktop\"]\n";
            let bin_target_alt = "\n[[bin]]\nname = \"rustbasic-desktop\"\npath = \"native/desktop/src/main.rs\"\nrequired-features = [\"desktop\"]";
            let objc_dep = "\nobjc = \"0.2.7\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"";
            let objc_dep_alt = "\nobjc = \"0.2.7\"";
            
            content = content.replace(bin_target, "");
            content = content.replace(bin_target_alt, "");
            // Also clean the old version without required-features just in case
            let old_bin_target = "\n\n[[bin]]\nname = \"rustbasic-desktop\"\npath = \"native/desktop/src/main.rs\"\n";
            let old_bin_target_alt = "\n[[bin]]\nname = \"rustbasic-desktop\"\npath = \"native/desktop/src/main.rs\"";
            content = content.replace(old_bin_target, "");
            content = content.replace(old_bin_target_alt, "");
            content = content.replace(objc_dep, "");
            content = content.replace(objc_dep_alt, "");
            
            // Clean up desktop feature from rustbasic-core feature list if present
            content = content.replace("\n    \"desktop\",", "");
            content = content.replace("\"desktop\",", "");
            content = content.replace("\n    \"websocket\",", "");
            content = content.replace("\"websocket\",", "");
            
            fs::write(cargo_path, content).ok();
            println!("   {} Membersihkan konfigurasi desktop di Cargo.toml", "📝".blue());
        }
    }

    println!("✅ {}", "Pembersihan scaffolding RustBasic Native selesai!".green().bold());
}

fn write_scaffold_file(path_str: &str, content: &[u8], make_executable: bool) {
    let path = Path::new(path_str);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Gagal membuat direktori");
    }
    if !path.exists() {
        let mut file = fs::File::create(path).expect("Gagal membuat file");
        file.write_all(content).expect("Gagal menulis file");
        
        #[cfg(unix)]
        if make_executable {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(path, perms).ok();
            }
        }
        println!("   {} Membuat file {}", "🆕".green().bold(), path_str);
    } else {
        println!("   {} File {} sudah ada, dilewati.", "ℹ️".dimmed(), path_str);
    }
}

fn get_crate_name() -> String {
    let cargo_path = Path::new("Cargo.toml");
    if let Ok(content) = fs::read_to_string(cargo_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name =") || trimmed.starts_with("name=") {
                let parts: Vec<&str> = trimmed.split('=').collect();
                if parts.len() > 1 {
                    let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                    return name.replace('-', "_");
                }
            }
        }
    }
    "rustbasic".to_string()
}

pub fn make_native_scaffolding() {
    println!("🚀 {}", "Menyiapkan scaffolding RustBasic Native...".bold());

    // 1. Tulis file-file scaffolding native
    write_scaffold_file("native/README.md", TEMPLATE_README_MD.as_bytes(), false);
    write_scaffold_file("native/native-bridge.js", TEMPLATE_NATIVE_BRIDGE_JS.as_bytes(), false);

    // Desktop wrapper
    let crate_name = get_crate_name();
    let desktop_main_content = TEMPLATE_DESKTOP_MAIN_RS.replace("rustbasic::", &format!("{}::", crate_name));
    write_scaffold_file("native/desktop/src/main.rs", desktop_main_content.as_bytes(), false);
    write_scaffold_file("native/desktop/Info.plist", TEMPLATE_DESKTOP_INFO_PLIST.as_bytes(), false);

    // Android wrapper gradle files
    write_scaffold_file("native/android/build.gradle", TEMPLATE_ANDROID_BUILD_GRADLE.as_bytes(), false);
    write_scaffold_file("native/android/gradle.properties", TEMPLATE_ANDROID_GRADLE_PROPERTIES.as_bytes(), false);
    write_scaffold_file("native/android/settings.gradle", TEMPLATE_ANDROID_SETTINGS_GRADLE.as_bytes(), false);
    write_scaffold_file("native/android/gradlew", TEMPLATE_ANDROID_GRADLEW.as_bytes(), true);
    write_scaffold_file("native/android/gradlew.bat", TEMPLATE_ANDROID_GRADLEW_BAT.as_bytes(), false);
    write_scaffold_file("native/android/gradle/wrapper/gradle-wrapper.properties", TEMPLATE_ANDROID_GRADLE_WRAPPER_PROPERTIES.as_bytes(), false);
    write_scaffold_file("native/android/gradle/wrapper/gradle-wrapper.jar", TEMPLATE_ANDROID_GRADLE_WRAPPER_JAR, false);

    // Android wrapper app module files
    write_scaffold_file("native/android/app/build.gradle", TEMPLATE_ANDROID_APP_BUILD_GRADLE.as_bytes(), false);
    write_scaffold_file("native/android/app/src/main/AndroidManifest.xml", TEMPLATE_ANDROID_MANIFEST_XML.as_bytes(), false);
    write_scaffold_file("native/android/app/src/main/java/com/rustbasic/mobile/MainActivity.kt", TEMPLATE_ANDROID_MAIN_ACTIVITY_KT.as_bytes(), false);
    write_scaffold_file("native/android/app/src/main/java/com/rustbasic/mobile/RustServer.kt", TEMPLATE_ANDROID_RUST_SERVER_KT.as_bytes(), false);

    // Frontend bridge file
    write_scaffold_file("src/resources/js/native-bridge.ts", TEMPLATE_FRONTEND_NATIVE_BRIDGE_TS.as_bytes(), false);

    // ReactJS Permission diagnostic page
    write_scaffold_file("src/resources/js/Pages/Permission.tsx", TEMPLATE_REACT_PERMISSION_TSX.as_bytes(), false);

    // WebSocket helper client and documentation
    write_scaffold_file("src/resources/js/reverb.ts", TEMPLATE_REVERB_TS.as_bytes(), false);
    write_scaffold_file("docs/websocket.md", TEMPLATE_WEBSOCKET_DOC_MD.as_bytes(), false);

    // 2. Konfigurasi entry points di src/lib.rs dan javascript main entry
    let lib_path = Path::new("src/lib.rs");
    if lib_path.exists() {
        modify_user_lib_entry(lib_path);
    }
    
    modify_user_main_entry();
    modify_user_layout_entry();
    modify_user_routes_entry();
    modify_user_controller_entry();
    modify_user_cargo_toml();
    
    println!("✅ {}", "Scaffolding RustBasic Native berhasil diselesaikan!".green().bold());
}

fn modify_user_lib_entry(lib_path: &Path) {
    let Ok(mut content) = fs::read_to_string(lib_path) else { return };

    if content.contains("rustbasic_mobile::setup_mobile") {
        content = content.replace("rustbasic_mobile::setup_mobile", "rustbasic_native::setup_native");
        fs::write(lib_path, &content).ok();
        return;
    }

    if content.contains("setup_native") {
        return; // sudah terkonfigurasi
    }

    println!("   {} Menambahkan entry point native di src/lib.rs...", "📝".bold());

    let mut file = OpenOptions::new()
        .append(true)
        .open(lib_path)
        .expect("Gagal membuka src/lib.rs");

    writeln!(
        file,
        r#"
// Native (Mobile & Desktop) Entry Point
#[cfg(any(target_os = "android", target_os = "ios"))]
rustbasic_native::setup_native!();"#
    ).ok();
}

fn modify_user_main_entry() {
    let entry_paths = [
        "src/resources/js/main.tsx",
        "src/resources/js/main.ts",
        "src/resources/js/main.jsx",
        "src/resources/js/main.js",
    ];
    for path in &entry_paths {
        if Path::new(path).exists() {
            let Ok(content) = fs::read_to_string(path) else { continue };

            if content.contains("import './native-bridge'") || content.contains("import \"./native-bridge\"") {
                return; // sudah terkonfigurasi
            }

            println!("   {} Menambahkan import native-bridge di {}...", "📝".bold(), path);

            let updated = format!("import './native-bridge';\n{}", content);
            fs::write(path, updated).ok();
            return;
        }
    }
}

fn modify_user_layout_entry() {
    let layout_path = Path::new("src/resources/js/Layouts/AppLayout.tsx");
    if !layout_path.exists() { return; }

    let Ok(content) = fs::read_to_string(layout_path) else { return };

    if content.contains("route('permission')") {
        return; // sudah terkonfigurasi
    }

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut about_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains("href={route('about')}") {
            about_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = about_idx {
        let mut close_idx = None;
        for i in idx..lines.len() {
            if lines[i].contains("</Link>") {
                close_idx = Some(i);
                break;
            }
        }

        if let Some(c_idx) = close_idx {
            println!("   {} Menambahkan navigasi Izin di {}...", "📝".bold(), layout_path.display());
            let permission_link = vec![
                "            <Link".to_string(),
                "              href={route('permission')}".to_string(),
                "              style={{".to_string(),
                "                fontSize: '0.875rem', fontWeight: 600, ".to_string(),
                "                color: currentPath === '/permission' ? '#e8520e' : (isDark ? '#888' : '#555'),".to_string(),
                "                textDecoration: 'none', transition: 'color 0.2s',".to_string(),
                "              }}".to_string(),
                "            >".to_string(),
                "              Izin".to_string(),
                "            </Link>".to_string(),
            ];
            
            for (offset, line) in permission_link.into_iter().enumerate() {
                lines.insert(c_idx + 1 + offset, line);
            }

            let has_crlf = content.contains("\r\n");
            let joiner = if has_crlf { "\r\n" } else { "\n" };
            let updated = lines.join(joiner) + joiner;
            fs::write(layout_path, updated).ok();
        }
    }
}

fn modify_user_routes_entry() {
    let routes_path = Path::new("src/routes/web.rs");
    if !routes_path.exists() { return; }

    let Ok(content) = fs::read_to_string(routes_path) else { return };

    if content.contains("get(welcome_controller::permission)") || content.contains("\"/permission\"") {
        return; // sudah terkonfigurasi
    }

    let search_target = r#".route("/about", get(welcome_controller::about)).name("about")"#;

    if let Some(pos) = content.find(search_target) {
        println!("   {} Menambahkan route permission di {}...", "📝".bold(), routes_path.display());
        let insert_pos = pos + search_target.len();
        let route_to_insert = "\n        .route(\"/permission\", get(welcome_controller::permission)).name(\"permission\")";
        let mut updated_content = content;
        updated_content.insert_str(insert_pos, route_to_insert);
        fs::write(routes_path, updated_content).ok();
    }
}

fn modify_user_controller_entry() {
    let controller_path = Path::new("src/app/http/controllers/welcome_controller.rs");
    if !controller_path.exists() { return; }

    let Ok(content) = fs::read_to_string(controller_path) else { return };

    if content.contains("pub async fn permission") {
        return; // sudah terkonfigurasi
    }

    println!("   {} Menambahkan controller handler permission di {}...", "📝".bold(), controller_path.display());
    let mut file = OpenOptions::new()
        .append(true)
        .open(controller_path)
        .expect("Gagal membuka welcome_controller.rs");
    writeln!(
        file,
        r#"

pub async fn permission(req: Request) -> impl IntoResponse {{
    inertia(&req, "Permission", json!({{}}))
}}"#
    ).ok();
}

fn modify_user_cargo_toml() {
    let cargo_path = Path::new("Cargo.toml");
    let Ok(mut content) = fs::read_to_string(cargo_path) else { return };

    let mut updated = false;

    // 1. Add websocket features to rustbasic-core if they're not there
    if let Some(pos) = content.find("rustbasic-core") {
        if let Some(features_pos) = content[pos..].find("features = [") {
            let actual_features_pos = pos + features_pos + "features = [".len();
            let mut feature_insert = String::new();
            if !content[pos..].contains("websocket") {
                feature_insert.push_str("\n    \"websocket\",");
            }
            if !feature_insert.is_empty() {
                content.insert_str(actual_features_pos, &feature_insert);
                updated = true;
                println!("   {} Menambahkan fitur websocket ke rustbasic-core di Cargo.toml...", "📝".bold());
            }
        }
    }

    // 1b. Add desktop cargo feature if not present
    if !content.contains("[features]") {
        content.push_str("\n\n[features]\ndesktop = [\"rustbasic-core/desktop\"]\n");
        updated = true;
        println!("   {} Menambahkan konfigurasi fitur [features] dan desktop ke Cargo.toml...", "📝".bold());
    } else {
        if !content.contains("desktop =") && !content.contains("desktop=") {
            if let Some(pos) = content.find("[features]") {
                let insert_pos = pos + "[features]".len();
                content.insert_str(insert_pos, "\ndesktop = [\"rustbasic-core/desktop\"]");
                updated = true;
                println!("   {} Menambahkan fitur desktop ke [features] di Cargo.toml...", "📝".bold());
            }
        }
    }

    // 2. Add objc, serde, serde_json dependencies if not present
    if !content.contains("objc =") && !content.contains("objc=") {
        if let Some(pos) = content.find("[dependencies]") {
            let insert_pos = pos + "[dependencies]".len();
            content.insert_str(insert_pos, "\nobjc = \"0.2.7\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nserde_json = \"1.0\"");
            updated = true;
            println!("   {} Menambahkan dependensi desktop (objc, serde, serde_json) di Cargo.toml...", "📝".bold());
        }
    }

    // 3. Add [[bin]] target for rustbasic-desktop if not present
    if !content.contains("name = \"rustbasic-desktop\"") {
        content.push_str("\n\n[[bin]]\nname = \"rustbasic-desktop\"\npath = \"native/desktop/src/main.rs\"\nrequired-features = [\"desktop\"]\n");
        updated = true;
        println!("   {} Menambahkan target binary rustbasic-desktop dengan required-features di Cargo.toml...", "📝".bold());
    }

    if updated {
        fs::write(cargo_path, &content).ok();
        println!("   {} Cargo.toml berhasil diperbarui.", "✅".green());
    }
}


const TEMPLATE_README_MD: &str = r##"# RustBasic Native Wrapper (Mobile & Desktop)

Folder ini berisi wrapper native untuk menjalankan aplikasi RustBasic di Android dan Desktop menggunakan WebView terintegrasi dan Rust-backend di background thread.

## Fitur
1. **Zero-Latency Backend**: Server RustBasic + Database SQLite berjalan langsung di device secara local.
2. **Native JS Bridge**: Menghubungkan halaman web (React/Vue/Inertia) dengan sensor perangkat (GPS, Battery, dll) menggunakan `window.MobileBridge`.

## Persiapan & Build

### 1. Desktop (macOS, Windows, Linux)
Untuk menjalankan versi desktop secara langsung:
```bash
rustbasic serve --desktop
```

### 2. Android
Untuk membuild paket Android (APK/AAB):
```bash
rustbasic build --desktop
# atau pilih target Android
rustbasic build --android
```
Untuk mengompilasi, menginstal, dan langsung **menjalankan** aplikasi di perangkat/emulator Android yang terhubung:
```bash
rustbasic serve --android
```
*(Catatan: Pastikan adb terinstal dan perangkat/emulator terhubung sebelum menjalankan)*
"##;

const _TEMPLATE_RUN_ANDROID_SH: &str = r##"#!/bin/bash
set -e

echo "📱 Starting RustBasic Android App..."

# Detect operating system and set paths
OS_NAME="$(uname -s)"
if [ "$OS_NAME" = "Darwin" ]; then
    ANDROID_SDK_PATH="$HOME/Library/Android/sdk"
    STUDIO_JBR="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
    if [ ! -d "$STUDIO_JBR" ]; then
        STUDIO_JBR="/Applications/Android Studio.app/Contents/jre/Contents/Home"
    fi
else
    ANDROID_SDK_PATH="$HOME/Android/Sdk"
    STUDIO_JBR="/opt/android-studio/jbr"
    if [ ! -d "$STUDIO_JBR" ]; then
        STUDIO_JBR="/usr/local/android-studio/jbr"
    fi
fi

if [ -z "$ANDROID_HOME" ]; then
    export ANDROID_HOME="$ANDROID_SDK_PATH"
fi
export PATH="$ANDROID_HOME/platform-tools:$PATH"
EMULATOR_BIN="$ANDROID_HOME/emulator/emulator"

if [ -n "$JAVA_HOME" ]; then
    echo "Using existing JAVA_HOME: $JAVA_HOME"
elif [ -d "$STUDIO_JBR" ]; then
    echo "Setting JAVA_HOME to Android Studio JDK..."
    export JAVA_HOME="$STUDIO_JBR"
fi

if ! adb devices | grep -q -E "device$|emulator-"; then
    echo "📱 Perangkat Android atau emulator tidak terdeteksi aktif."
    if [ -f "$EMULATOR_BIN" ]; then
        AVD_NAME=$("$EMULATOR_BIN" -list-avds | head -n 1)
        if [ -n "$AVD_NAME" ]; then
            echo "🚀 Menyalakan emulator AVD: $AVD_NAME..."
            "$EMULATOR_BIN" -avd "$AVD_NAME" > /dev/null 2>&1 &
            
            echo "⏳ Menunggu emulator menyala dan terdeteksi adb..."
            adb wait-for-device
            echo "✅ Emulator berhasil aktif!"
            sleep 3
        else
            echo "❌ Error: Tidak ada AVD (Android Virtual Device) yang terdaftar di sistem."
            exit 1
        fi
    else
        echo "❌ Error: Tidak ada perangkat Android terhubung dan emulator tidak ditemukan."
        exit 1
    fi
fi

# Parse VITE_PORT from .env for Hot Reload port forwarding
VITE_PORT=5173
if [ -f ".env" ]; then
    ENV_VITE_PORT=$(grep -E "^VITE_PORT=" .env | cut -d= -f2 | tr -d '\r' | tr -d '"' | tr -d "'")
    if [ -n "$ENV_VITE_PORT" ]; then
        VITE_PORT=$ENV_VITE_PORT
    fi
fi

# Get list of connected devices
devices_raw=$(adb devices | grep -E "device$|emulator-" | awk '{print $1}')
device_count=$(echo "$devices_raw" | grep -c . || true)

if [ "$device_count" -eq 0 ]; then
    echo "❌ Error: Tidak ada perangkat Android terhubung."
    exit 1
elif [ "$device_count" -eq 1 ]; then
    device=$(echo "$devices_raw" | awk '{print $1}')
    device_name=$(adb -s "$device" shell getprop ro.product.model 2>/dev/null | tr -d '\r')
    if [ -z "$device_name" ]; then
        device_name="Android Device"
    fi
    echo "📱 Menggunakan perangkat tunggal: $device_name ($device)"
else
    echo "📱 Terdeteksi beberapa perangkat Android. Silakan pilih target:"
    i=0
    dev_ids=()
    dev_names=()
    while read -r line; do
        if [ -n "$line" ]; then
            dev_id=$(echo "$line" | awk '{print $1}')
            model_name=$(adb -s "$dev_id" shell getprop ro.product.model 2>/dev/null | tr -d '\r')
            if [ -z "$model_name" ]; then
                model_name="Unknown Device"
            fi
            dev_ids+=("$dev_id")
            dev_names+=("$model_name")
            i=$((i+1))
            echo "  [$i] $model_name ($dev_id)"
        fi
    done <<< "$devices_raw"
    
    while true; do
        read -p "👉 Pilih nomor perangkat (1-$i): " choice
        if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "$i" ]; then
            idx=$((choice-1))
            device="${dev_ids[$idx]}"
            device_name="${dev_names[$idx]}"
            echo "✅ Memilih: $device_name ($device)"
            break
        else
            echo "⚠️ Pilihan tidak valid, silakan coba lagi."
        fi
    done
fi

./native/build-android.sh

# Create local.properties if not present to define Android SDK location
if [ ! -f "native/android/local.properties" ]; then
    echo "sdk.dir=$ANDROID_HOME" > native/android/local.properties
fi

cd native/android

if [ ! -f "./gradlew" ] && command -v gradle >/dev/null 2>&1; then
    echo "🔨 Generating Gradle wrapper (version 8.4) using global gradle..."
    gradle wrapper --gradle-version 8.4
fi

if [ -f "./gradlew" ]; then
    echo "🔨 Membangun debug APK menggunakan Gradle wrapper..."
    ./gradlew assembleDebug
else
    echo "🔨 Gradle wrapper tidak ditemukan. Menggunakan perintah gradle global..."
    if command -v gradle >/dev/null 2>&1; then
        gradle assembleDebug
    else
        echo "❌ Error: Perintah 'gradle' not found."
        echo "💡 Silakan buka folder 'native/android' di Android Studio terlebih dahulu agar Gradle wrapper digenerate otomatis, atau instal gradle di sistem Anda (misal: brew install gradle)."
        exit 1
    fi
fi

echo "🔨 Memasang APK ke perangkat $device_name ($device)..."
adb -s "$device" install -r app/build/outputs/apk/debug/app-debug.apk

echo "🔄 Setting up ADB port forwarding for device $device (port $VITE_PORT)..."
adb -s "$device" reverse tcp:$VITE_PORT tcp:$VITE_PORT || echo "⚠️ Warning: Failed to set up ADB port forwarding for $device"

echo "🚀 Membuka aplikasi di perangkat $device_name..."
adb -s "$device" logcat -c
adb -s "$device" shell am start -n com.rustbasic.mobile/com.rustbasic.mobile.MainActivity

echo "📋 Menampilkan log realtime dari perangkat $device_name (Tekan Ctrl+C untuk keluar)..."
adb -s "$device" logcat -s RustBasicServer
"##;

const _TEMPLATE_BUILD_ANDROID_SH: &str = r##"#!/bin/bash
set -e

echo "🚀 Building Rust library for Android..."

rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

if [ -z "$ANDROID_NDK_HOME" ]; then
    if [ -d "$HOME/Library/Android/sdk/ndk" ]; then
        NDK_DIR=$(ls -d $HOME/Library/Android/sdk/ndk/* | sort -V | tail -n 1)
        export ANDROID_NDK_HOME=$NDK_DIR
    else
        echo "❌ Error: ANDROID_NDK_HOME is not set. Please set ANDROID_NDK_HOME."
        exit 1
    fi
fi

echo "Using NDK: $ANDROID_NDK_HOME"

# Setup toolchain bin path
TOOLCHAIN_BIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin"
if [ ! -d "$TOOLCHAIN_BIN" ]; then
    TOOLCHAIN_BIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
fi

# Download and extract SQLite amalgamation if not present
SQLITE_VERSION="3450100"
SQLITE_DIR="target/sqlite-amalgamation-$SQLITE_VERSION"
if [ ! -d "$SQLITE_DIR" ]; then
    echo "📥 Downloading SQLite source amalgamation..."
    mkdir -p target
    curl -sSLo target/sqlite.zip "https://www.sqlite.org/2024/sqlite-amalgamation-$SQLITE_VERSION.zip"
    unzip -q target/sqlite.zip -d target/
    rm target/sqlite.zip
fi

TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "x86_64-linux-android")
JNILIBS_DIR="native/android/app/src/main/jniLibs"

mkdir -p $JNILIBS_DIR/arm64-v8a
mkdir -p $JNILIBS_DIR/armeabi-v7a
mkdir -p $JNILIBS_DIR/x86_64

for target in "${TARGETS[@]}"; do
    echo "🔨 Preparing SQLite static library for $target..."
    
    # Identify target compiler & packaging tools
    case $target in
        "aarch64-linux-android")
            CLANG_LINKER="$TOOLCHAIN_BIN/aarch64-linux-android21-clang"
            export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$CLANG_LINKER"
            export CC_aarch64_linux_android="$CLANG_LINKER"
            export AR_aarch64_linux_android="$TOOLCHAIN_BIN/llvm-ar"
            ;;
        "armv7-linux-androideabi")
            CLANG_LINKER="$TOOLCHAIN_BIN/armv7a-linux-androideabi21-clang"
            export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$CLANG_LINKER"
            export CC_armv7_linux_androideabi="$CLANG_LINKER"
            export AR_armv7_linux_androideabi="$TOOLCHAIN_BIN/llvm-ar"
            ;;
        "x86_64-linux-android")
            CLANG_LINKER="$TOOLCHAIN_BIN/x86_64-linux-android21-clang"
            export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$CLANG_LINKER"
            export CC_x86_64_linux_android="$CLANG_LINKER"
            export AR_x86_64_linux_android="$TOOLCHAIN_BIN/llvm-ar"
            ;;
    esac

    # Compile and archive sqlite3 if not already compiled
    SQLITE_OUT="target/$target/sqlite"
    mkdir -p "$SQLITE_OUT"
    if [ ! -f "$SQLITE_OUT/libsqlite3.a" ]; then
        echo "   Compiling SQLite static lib for $target..."
        $CLANG_LINKER -O2 -c "$SQLITE_DIR/sqlite3.c" -o "$SQLITE_OUT/sqlite3.o"
        "$TOOLCHAIN_BIN/llvm-ar" rcs "$SQLITE_OUT/libsqlite3.a" "$SQLITE_OUT/sqlite3.o"
    fi

    echo "🔨 Compiling Rust library for $target..."
    cargo build --target $target --release
    
    case $target in
        "aarch64-linux-android")
            cp target/$target/release/librustbasic.so $JNILIBS_DIR/arm64-v8a/librustbasic_mobile.so
            ;;
        "armv7-linux-androideabi")
            cp target/$target/release/librustbasic.so $JNILIBS_DIR/armeabi-v7a/librustbasic_mobile.so
            ;;
        "x86_64-linux-android")
            cp target/$target/release/librustbasic.so $JNILIBS_DIR/x86_64/librustbasic_mobile.so
            ;;
    esac
done

echo "✅ Android JNI libraries built successfully!"
"##;

const _TEMPLATE_RUN_DESKTOP_SH: &str = r##"#!/bin/bash
echo "🖥️ Starting RustBasic Native Desktop application..."
cargo run --manifest-path native/desktop/Cargo.toml
"##;

const TEMPLATE_NATIVE_BRIDGE_JS: &str = r##"// RustBasic Native JavaScript Bridge
// Exposes device hardware sensors for both Android & iOS WebViews

window.MobileBridge = {
    // 1. GPS / Location Sensor
    getGPSLocation: function() {
        return new Promise((resolve, reject) => {
            window._gpsResolve = resolve;
            window._gpsReject = reject;
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "getGPSLocation" }));
            } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.getGPSLocation) {
                window.webkit.messageHandlers.getGPSLocation.postMessage(null);
            } else if (window.MobileBridgeNative) {
                let res = window.MobileBridgeNative.getGPSLocation();
                resolve(JSON.parse(res));
            } else {
                reject("No native mobile bridge found");
            }
        });
    },

    // 2. Device Sensors (Battery, Proximity, etc.)
    getDeviceSensors: function() {
        return new Promise((resolve, reject) => {
            window._sensorsResolve = resolve;
            window._sensorsReject = reject;
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "getDeviceSensors" }));
            } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.getDeviceSensors) {
                window.webkit.messageHandlers.getDeviceSensors.postMessage(null);
            } else if (window.MobileBridgeNative) {
                let res = window.MobileBridgeNative.getDeviceSensors();
                resolve(JSON.parse(res));
            } else {
                reject("No native mobile bridge found");
            }
        });
    },

    // 3. Show Toast Notification
    showToast: function(message) {
        if (window.ipc && window.ipc.postMessage) {
            window.ipc.postMessage(JSON.stringify({ action: "showToast", message: message }));
        } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.showToast) {
            window.webkit.messageHandlers.showToast.postMessage(message);
        } else if (window.MobileBridgeNative) {
            window.MobileBridgeNative.showToast(message);
        } else {
            console.log("Toast (Fallback): " + message);
        }
    },

    // 4. Show Heads-Up Notification (WhatsApp style)
    showNotification: function(title, message) {
        if (window.ipc && window.ipc.postMessage) {
            window.ipc.postMessage(JSON.stringify({ action: "showNotification", title: title, message: message }));
        } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.showNotification) {
            window.webkit.messageHandlers.showNotification.postMessage({ title: title, message: message });
        } else if (window.MobileBridgeNative && window.MobileBridgeNative.showNotification) {
            window.MobileBridgeNative.showNotification(title, message);
        } else {
            console.log("Notification (Fallback) - Title: " + title + ", Message: " + message);
        }
    }
};

// Callbacks for asynchronous iOS Swift bridges
window.onGPSLocationResult = function(data) {
    if (window._gpsResolve) {
        window._gpsResolve(data);
    }
};

window.onDeviceSensorsResult = function(data) {
    if (window._sensorsResolve) {
        window._sensorsResolve(data);
    }
};
"##;


const TEMPLATE_DESKTOP_INFO_PLIST: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.rustbasic.desktop</string>
    <key>CFBundleName</key>
    <string>RustBasic Native Desktop</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>NSCameraUsageDescription</key>
    <string>Aplikasi membutuhkan izin kamera untuk menguji fungsionalitas preview video.</string>
    <key>NSMicrophoneUsageDescription</key>
    <string>Aplikasi membutuhkan izin mikrofon untuk menguji fungsionalitas perekaman audio.</string>
</dict>
</plist>
"##;

const TEMPLATE_DESKTOP_MAIN_RS: &str = r##"use std::thread;
use rustbasic_core::tokio;
use serde_json;

fn get_desktop_battery_info() -> (u8, bool) {
    let os = std::env::consts::OS;
    match os {
        "macos" => {
            if let Ok(output) = std::process::Command::new("pmset")
                .arg("-g")
                .arg("batt")
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut battery_pct = 100;
                let mut is_charging = true;
                
                if let Some(pct_idx) = stdout.find('%') {
                    let start = stdout[..pct_idx]
                        .rfind(|c: char| !c.is_ascii_digit())
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    if let Ok(val) = stdout[start..pct_idx].parse::<u8>() {
                        battery_pct = val;
                    }
                }
                
                if stdout.contains("discharging") {
                    is_charging = false;
                }
                return (battery_pct, is_charging);
            }
        }
        "windows" => {
            let charge_output = std::process::Command::new("powershell")
                .args(&["-Command", "(Get-CimInstance Win32_Battery).EstimatedChargeRemaining"])
                .output();
            let status_output = std::process::Command::new("powershell")
                .args(&["-Command", "(Get-CimInstance Win32_Battery).BatteryStatus"])
                .output();
                
            let mut battery_pct = 100;
            let mut is_charging = true;
            
            if let Ok(out) = charge_output {
                let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if let Ok(val) = stdout.parse::<u8>() {
                    battery_pct = val;
                }
            }
            if let Ok(out) = status_output {
                let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if let Ok(status) = stdout.parse::<u8>() {
                    if status == 1 {
                        is_charging = false;
                    }
                }
            }
            return (battery_pct, is_charging);
        }
        "linux" => {
            let capacity = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
                .or_else(|_| std::fs::read_to_string("/sys/class/power_supply/BAT1/capacity"));
            let status = std::fs::read_to_string("/sys/class/power_supply/BAT0/status")
                .or_else(|_| std::fs::read_to_string("/sys/class/power_supply/BAT1/status"));
                
            let mut battery_pct = 100;
            let mut is_charging = true;
            
            if let Ok(cap_str) = capacity {
                if let Ok(val) = cap_str.trim().parse::<u8>() {
                    battery_pct = val;
                }
            }
            if let Ok(stat_str) = status {
                if stat_str.trim().to_lowercase() == "discharging" {
                    is_charging = false;
                }
            }
            return (battery_pct, is_charging);
        }
        _ => {}
    }
    (100, true)
}

fn main() {
    // Load env
    rustbasic_core::dotenvy::dotenv().ok();
    let mut cfg = rustbasic_core::Config::load();

    // Deteksi jika Vite dev server sedang aktif
    let vite_running = if let Ok(addr) = format!("127.0.0.1:{}", cfg.vite_port).parse() {
        std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(100)).is_ok()
    } else {
        false
    };

    if !vite_running {
        unsafe {
            std::env::set_var("APP_DEBUG", "false");
        }
        cfg = rustbasic_core::Config::load(); // Reload config
    }

    let app_url = format!("http://localhost:{}", cfg.app_port);

    let cfg_clone = cfg.clone();

    // 1. Jalankan server RustBasic di background thread
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let _guard = rustbasic_core::logger::init();
            let db = rustbasic_core::database::connect(&cfg_clone).await;
            rustbasic_core::session::init_sessions(&cfg_clone).await;
            let session_store = rustbasic_core::session::setup_session(&cfg_clone).await;
            let app_router = rustbasic::routes::build_router();

            rustbasic_core::view::set_embedded_templates(rustbasic::EmbeddedTemplates::get);
            rustbasic_core::server::set_embedded_public(rustbasic::EmbeddedPublic::get);

            println!("🚀 Desktop Backend Server running on port {}", cfg_clone.app_port);
            rustbasic_core::server::start_server(cfg_clone, session_store, db, app_router).await;
        });
    });

    // 2. Tunggu server menyala sebentar
    std::thread::sleep(std::time::Duration::from_millis(1500));

    // 3. Jalankan Wry WebView Window di thread utama
    use rustbasic_core::wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::<String>::with_user_event();

    #[cfg(target_os = "macos")]
    {
        use objc::{msg_send, sel, sel_impl};
        use objc::runtime::Object;
        unsafe {
            let class = objc::runtime::Class::get("NSApplication").unwrap();
            let ns_app: *mut Object = msg_send![class, sharedApplication];
            let menu_class = objc::runtime::Class::get("NSMenu").unwrap();
            let main_menu: *mut Object = msg_send![menu_class, alloc];
            let main_menu: *mut Object = msg_send![main_menu, init];
            let _: () = msg_send![ns_app, setMainMenu: main_menu];
        }
    }

    let window = WindowBuilder::new()
        .with_title(&cfg.app_name)
        .with_inner_size(rustbasic_core::wry::application::dpi::LogicalSize::new(1280.0, 800.0))
        .build(&event_loop)
        .unwrap();

    let proxy = event_loop.create_proxy();

    let webview_builder = WebViewBuilder::new(window)
        .unwrap()
        .with_devtools(true)
        .with_ipc_handler(move |_window, req| {
            println!("[IPC Handler Received]: {}", req);
            let _ = proxy.send_event(req);
        });

    let webview = webview_builder.build().unwrap();

    #[cfg(target_os = "macos")]
    {
        use rustbasic_core::wry::webview::WebviewExtMacOS;
        use objc::{msg_send, sel, sel_impl};
        use objc::runtime::Object;
        unsafe {
            let raw_webview = webview.webview() as *mut Object;
            let configuration: *mut Object = msg_send![raw_webview, configuration];
            let preferences: *mut Object = msg_send![configuration, preferences];
            
            let responds_to_private: bool = msg_send![preferences, respondsToSelector: sel!(_setMediaCaptureRequiresSecureConnection:)];
            if responds_to_private {
                println!("🔒 Bypassing secure context requirement for media capture via private WebKit API...");
                let _: () = msg_send![preferences, _setMediaCaptureRequiresSecureConnection: false];
            }
            let responds_to_public: bool = msg_send![preferences, respondsToSelector: sel!(setMediaCaptureRequiresSecureConnection:)];
            if responds_to_public {
                println!("🔒 Bypassing secure context requirement for media capture via public WebKit API...");
                let _: () = msg_send![preferences, setMediaCaptureRequiresSecureConnection: false];
            }
        }
    }

    // Load the URL after applying preferences so WebKit considers the origin secure
    webview.load_url(&app_url);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(req_str) => {
                println!("[Event Loop Received UserEvent]: {}", req_str);
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&req_str) {
                    if let Some(action) = val.get("action").and_then(|a| a.as_str()) {
                        match action {
                            "getGPSLocation" => {
                                let js = "window.onGPSLocationResult({ status: 'success', latitude: -6.200000, longitude: 106.816666, provider: 'desktop-wry' });";
                                if let Err(e) = webview.evaluate_script(js) {
                                    println!("❌ [getGPSLocation JS Error]: {:?}", e);
                                } else {
                                    println!("✅ [getGPSLocation JS Sent]");
                                }
                            }
                            "getDeviceSensors" => {
                                let (battery_pct, is_charging) = get_desktop_battery_info();
                                let js = format!(
                                    "window.onDeviceSensorsResult({{ status: 'success', battery: {}, charging: {}, proximity: false }});",
                                    battery_pct, is_charging
                                );
                                if let Err(e) = webview.evaluate_script(&js) {
                                    println!("❌ [getDeviceSensors JS Error]: {:?}", e);
                                } else {
                                    println!("✅ [getDeviceSensors JS Sent]: {}", js);
                                }
                            }
                            "showToast" => {
                                if let Some(message) = val.get("message").and_then(|m| m.as_str()) {
                                    let escaped_message = message.replace('\'', "\\'");
                                    let js = format!(
                                        r#"
                                        (function() {{
                                            let container = document.getElementById('rustbasic-native-toast-container');
                                            if (!container) {{
                                                container = document.createElement('div');
                                                container.id = 'rustbasic-native-toast-container';
                                                container.style.position = 'fixed';
                                                container.style.bottom = '24px';
                                                container.style.left = '50%';
                                                container.style.transform = 'translateX(-50%)';
                                                container.style.zIndex = '99999';
                                                container.style.display = 'flex';
                                                container.style.flexDirection = 'column';
                                                container.style.gap = '8px';
                                                document.body.appendChild(container);
                                            }}
                                            let toast = document.createElement('div');
                                            toast.style.background = 'rgba(15, 23, 42, 0.9)';
                                            toast.style.backdropFilter = 'blur(8px)';
                                            toast.style.color = '#fff';
                                            toast.style.padding = '12px 24px';
                                            toast.style.borderRadius = '30px';
                                            toast.style.boxShadow = '0 10px 15px -3px rgba(0, 0, 0, 0.3)';
                                            toast.style.fontSize = '14px';
                                            toast.style.fontFamily = 'Inter, sans-serif';
                                            toast.style.border = '1px solid rgba(255, 255, 255, 0.1)';
                                            toast.style.animation = 'rustbasicToastIn 0.3s ease-out forwards';
                                            toast.style.whiteSpace = 'nowrap';
                                            toast.textContent = '{}';
                                            
                                            if (!document.getElementById('rustbasic-native-toast-style')) {{
                                                let style = document.createElement('style');
                                                style.id = 'rustbasic-native-toast-style';
                                                style.innerHTML = `
                                                    @keyframes rustbasicToastIn {{
                                                        from {{ opacity: 0; transform: translateY(20px) scale(0.9); }}
                                                        to {{ opacity: 1; transform: translateY(0) scale(1); }}
                                                    }}
                                                    @keyframes rustbasicToastOut {{
                                                        from {{ opacity: 1; transform: translateY(0) scale(1); }}
                                                        to {{ opacity: 0; transform: translateY(-20px) scale(0.9); }}
                                                    }}
                                                `;
                                                document.head.appendChild(style);
                                            }}
                                            
                                            container.appendChild(toast);
                                            setTimeout(() => {{
                                                toast.style.animation = 'rustbasicToastOut 0.3s ease-in forwards';
                                                setTimeout(() => {{
                                                    toast.remove();
                                                    if (container.children.length === 0) {{
                                                        container.remove();
                                                    }}
                                                }}, 300);
                                            }}, 3000);
                                        }})();
                                        "#,
                                        escaped_message
                                    );
                                    let _ = webview.evaluate_script(&js);
                                }
                            }
                            "showNotification" => {
                                let title = val.get("title").and_then(|t| t.as_str()).unwrap_or("Pesan Baru");
                                let message = val.get("message").and_then(|m| m.as_str()).unwrap_or("");
                                let escaped_title = title.replace('\'', "\\'");
                                let escaped_message = message.replace('\'', "\\'");
                                let js = format!(
                                    r#"
                                    (function() {{
                                        let container = document.getElementById('rustbasic-native-notification-container');
                                        if (!container) {{
                                            container = document.createElement('div');
                                            container.id = 'rustbasic-native-notification-container';
                                            container.style.position = 'fixed';
                                            container.style.top = '24px';
                                            container.style.right = '24px';
                                            container.style.zIndex = '99999';
                                            container.style.display = 'flex';
                                            container.style.flexDirection = 'column';
                                            container.style.gap = '12px';
                                            container.style.maxWidth = '360px';
                                            container.style.width = 'calc(100vw - 48px)';
                                            document.body.appendChild(container);
                                        }}
                                        let notif = document.createElement('div');
                                        notif.style.background = 'rgba(30, 41, 59, 0.95)';
                                        notif.style.backdropFilter = 'blur(12px)';
                                        notif.style.color = '#fff';
                                        notif.style.padding = '16px';
                                        notif.style.borderRadius = '16px';
                                        notif.style.boxShadow = '0 20px 25px -5px rgba(0, 0, 0, 0.4)';
                                        notif.style.border = '1px solid rgba(255, 255, 255, 0.15)';
                                        notif.style.fontFamily = 'Inter, sans-serif';
                                        notif.style.animation = 'rustbasicNotifIn 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards';
                                        notif.style.cursor = 'pointer';
                                        notif.style.display = 'flex';
                                        notif.style.alignItems = 'flex-start';
                                        notif.style.gap = '12px';
                                        
                                        let iconContainer = document.createElement('div');
                                        iconContainer.style.background = 'linear-gradient(135deg, #3b82f6, #1d4ed8)';
                                        iconContainer.style.borderRadius = '12px';
                                        iconContainer.style.width = '40px';
                                        iconContainer.style.height = '40px';
                                        iconContainer.style.display = 'flex';
                                        iconContainer.style.alignItems = 'center';
                                        iconContainer.style.justifyContent = 'center';
                                        iconContainer.style.flexShrink = '0';
                                        iconContainer.innerHTML = `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9M13.73 21a2 2 0 0 1-3.46 0"/></svg>`;
                                        
                                        let textContainer = document.createElement('div');
                                        textContainer.style.flexGrow = '1';
                                        
                                        let titleEl = document.createElement('div');
                                        titleEl.style.fontWeight = '600';
                                        titleEl.style.fontSize = '14px';
                                        titleEl.style.color = '#f8fafc';
                                        titleEl.style.marginBottom = '4px';
                                        titleEl.textContent = '{}';
                                        
                                        let msgEl = document.createElement('div');
                                        msgEl.style.fontSize = '13px';
                                        msgEl.style.color = '#cbd5e1';
                                        msgEl.style.lineHeight = '1.4';
                                        msgEl.textContent = '{}';
                                        
                                        textContainer.appendChild(titleEl);
                                        textContainer.appendChild(msgEl);
                                        notif.appendChild(iconContainer);
                                        notif.appendChild(textContainer);
                                        
                                        let closeBtn = document.createElement('button');
                                        closeBtn.style.background = 'none';
                                        closeBtn.style.border = 'none';
                                        closeBtn.style.color = '#94a3b8';
                                        closeBtn.style.cursor = 'pointer';
                                        closeBtn.style.padding = '2px';
                                        closeBtn.style.flexShrink = '0';
                                        closeBtn.innerHTML = `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`;
                                        
                                        let closeNotif = function() {{
                                            notif.style.animation = 'rustbasicNotifOut 0.2s ease-in forwards';
                                            setTimeout(() => {{
                                                notif.remove();
                                                if (container.children.length === 0) {{
                                                    container.remove();
                                                }}
                                            }}, 200);
                                        }};
                                        
                                        closeBtn.onclick = (e) => {{
                                            e.stopPropagation();
                                            closeNotif();
                                        }};
                                        notif.appendChild(closeBtn);
                                        
                                        if (!document.getElementById('rustbasic-native-notif-style')) {{
                                            let style = document.createElement('style');
                                            style.id = 'rustbasic-native-notif-style';
                                            style.innerHTML = `
                                                @keyframes rustbasicNotifIn {{
                                                    from {{ opacity: 0; transform: translateY(-20px) scale(0.95); }}
                                                    to {{ opacity: 1; transform: translateY(0) scale(1); }}
                                                }}
                                                @keyframes rustbasicNotifOut {{
                                                    from {{ opacity: 1; transform: translateY(0) scale(1); }}
                                                    to {{ opacity: 0; transform: translateX(50px); }}
                                                }}
                                            `;
                                            document.head.appendChild(style);
                                        }}
                                        
                                        notif.onclick = () => {{
                                            closeNotif();
                                        }};
                                        
                                        container.appendChild(notif);
                                        
                                        setTimeout(() => {{
                                            if (notif.parentNode) {{
                                                closeNotif();
                                            }}
                                        }}, 5000);
                                    }})();
                                    "#,
                                    escaped_title, escaped_message
                                );
                                let _ = webview.evaluate_script(&js);
                            }
                            _ => {}
                        }
                    }
                }
            }
            Event::NewEvents(StartCause::Init) => {}
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
"##;

const TEMPLATE_ANDROID_BUILD_GRADLE: &str = r##"buildscript {
    ext.kotlin_version = '1.9.22'
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath 'com.android.tools.build:gradle:8.2.2'
        classpath "org.jetbrains.kotlin:kotlin-gradle-plugin:$kotlin_version"
    }
}

allprojects {
    repositories {
        google()
        mavenCentral()
    }
}

task clean(type: Delete) {
    delete rootProject.buildDir
}
"##;

const TEMPLATE_ANDROID_GRADLE_PROPERTIES: &str = r##"android.useAndroidX=true
"##;

const TEMPLATE_ANDROID_SETTINGS_GRADLE: &str = r##"rootProject.name = "RustBasicMobile"
include ':app'
"##;

const TEMPLATE_ANDROID_GRADLEW: &str = r##"#!/bin/sh

#
# Copyright © 2015 the original authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0
#

##############################################################################
#
#   Gradle start up script for POSIX generated by Gradle.
#
#   Important for running:
#
#   (1) You need a POSIX-compliant shell to run this script. If your /bin/sh is
#       noncompliant, but you have some other compliant shell such as ksh or
#       bash, then to run this script, type that shell name before the whole
#       command line, like:
#
#           ksh Gradle
#
#       Busybox and similar reduced shells will NOT work, because this script
#       requires all of these POSIX shell features:
#         * functions;
#         * expansions «$var», «${var}», «${var:-default}», «${var+SET}»,
#           «${var#prefix}», «${var%suffix}», and «$( cmd )»;
#         * compound commands having a testable exit status, especially «case»;
#         * various built-in commands including «command», «set», and «ulimit».
#
#   Important for patching:
#
#   (2) This script targets any POSIX shell, so it avoids extensions provided
#       by Bash, Ksh, etc; in particular arrays are avoided.
#
#       The "traditional" practice of packing multiple parameters into a
#       space-separated string is a well documented source of bugs and security
#       problems, so this is (mostly) avoided, by progressively accumulating
#       options in "$@", and eventually passing that to Java.
#
#       Where the inherited environment variables (DEFAULT_JVM_OPTS, JAVA_OPTS,
#       and GRADLE_OPTS) rely on word-splitting, this is performed explicitly;
#       see the in-line comments for details.
#
#       There are tweaks for specific operating systems such as AIX, CygWin,
#       Darwin, MinGW, and NonStop.
#
#   (3) This script is generated from the Groovy template
#       https://github.com/gradle/gradle/blob/3d91ce3b8caaf77ad09f381f43615b715b53f72c/platforms/jvm/plugins-application/src/main/resources/org/gradle/api/internal/plugins/unixStartScript.txt
#       within the Gradle project.
#
#       You can find Gradle at https://github.com/gradle/gradle/.
#
##############################################################################

# Attempt to set APP_HOME

# Resolve links: $0 may be a link
app_path=$0

# Need this for daisy-chained symlinks.
while
    APP_HOME=${app_path%"${app_path##*/}"}  # leaves a trailing /; empty if no leading path
    [ -h "$app_path" ]
do
    ls=$( ls -ld "$app_path" )
    link=${ls#*' -> '}
    case $link in             #(
      /*)   app_path=$link ;; #(
      *)    app_path=$APP_HOME$link ;;
    esac
done

# This is normally unused
# shellcheck disable=SC2034
APP_BASE_NAME=${0##*/}
# Discard cd standard output in case $CDPATH is set (https://github.com/gradle/gradle/issues/25036)
APP_HOME=$( cd -P "${APP_HOME:-./}" > /dev/null && printf '%s\n' "$PWD" ) || exit

# Use the maximum available, or set MAX_FD != -1 to use that value.
MAX_FD=maximum

warn () {
    echo "$*"
} >&2

die () {
    echo
    echo "$*"
    echo
    exit 1
} >&2

# OS specific support (must be 'true' or 'false').
cygwin=false
msys=false
darwin=false
nonstop=false
case "$( uname )" in                #(
  CYGWIN* )         cygwin=true  ;; #(
  Darwin* )         darwin=true  ;; #(
  MSYS* | MINGW* )  msys=true    ;; #(
  NONSTOP* )        nonstop=true ;;
esac



# Determine the Java command to use to start the JVM.
if [ -n "$JAVA_HOME" ] ; then
    if [ -x "$JAVA_HOME/jre/sh/java" ] ; then
        # IBM's JDK on AIX uses strange locations for the executables
        JAVACMD=$JAVA_HOME/jre/sh/java
    else
        JAVACMD=$JAVA_HOME/bin/java
    fi
    if [ ! -x "$JAVACMD" ] ; then
        die "ERROR: JAVA_HOME is set to an invalid directory: $JAVA_HOME

Please set the JAVA_HOME variable in your environment to match the
location of your Java installation."
    fi
else
    JAVACMD=java
    if ! command -v java >/dev/null 2>&1
    then
        die "ERROR: JAVA_HOME is not set and no 'java' command could be found in your PATH.

Please set the JAVA_HOME variable in your environment to match the
location of your Java installation."
    fi
fi

# Increase the maximum file descriptors if we can.
if ! "$cygwin" && ! "$darwin" && ! "$nonstop" ; then
    case $MAX_FD in #(
      max*)
        # In POSIX sh, ulimit -H is undefined. That's why the result is checked to see if it worked.
        # shellcheck disable=SC2039,SC3045
        MAX_FD=$( ulimit -H -n ) ||
            warn "Could not query maximum file descriptor limit"
    esac
    case $MAX_FD in  #(
      '' | soft) :;; #(
      *)
        # In POSIX sh, ulimit -n is undefined. That's why the result is checked to see if it worked.
        # shellcheck disable=SC2039,SC3045
        ulimit -n "$MAX_FD" ||
            warn "Could not set maximum file descriptor limit to $MAX_FD"
    esac
fi

# Collect all arguments for the java command, stacking in reverse order:
#   * args from the command line
#   * the main class name
#   * -classpath
#   * -D...appname settings
#   * --module-path (only if needed)
#   * DEFAULT_JVM_OPTS, JAVA_OPTS, and GRADLE_OPTS environment variables.

# For Cygwin or MSYS, switch paths to Windows format before running java
if "$cygwin" || "$msys" ; then
    APP_HOME=$( cygpath --path --mixed "$APP_HOME" )

    JAVACMD=$( cygpath --unix "$JAVACMD" )

    # Now convert the arguments - kludge to limit ourselves to /bin/sh
    for arg do
        if
            case $arg in                                #(
              -*)   false ;;                            # don't mess with options #(
              /?*)  t=${arg#/} t=/${t%%/*}              # looks like a POSIX filepath
                    [ -e "$t" ] ;;                      #(
              *)    false ;;
            esac
        then
            arg=$( cygpath --path --ignore --mixed "$arg" )
        fi
        # Roll the args list around exactly as many times as the number of
        # args, so each arg winds up back in the position where it started, but
        # possibly modified.
        #
        # NB: a `for` loop captures its iteration list before it begins, so
        # changing the positional parameters here affects neither the number of
        # iterations, nor the values presented in `arg`.
        shift                   # remove old arg
        set -- "$@" "$arg"      # push replacement arg
    done
fi


# Add default JVM options here. You can also use JAVA_OPTS and GRADLE_OPTS to pass JVM options to this script.
DEFAULT_JVM_OPTS='"-Xmx64m" "-Xms64m"'

# Collect all arguments for the java command:
#   * DEFAULT_JVM_OPTS, JAVA_OPTS, and optsEnvironmentVar are not allowed to contain shell fragments,
#     and any embedded shellness will be escaped.
#   * For example: A user cannot expect ${Hostname} to be expanded, as it is an environment variable and will be
#     treated as '${Hostname}' itself on the command line.

set -- \
        "-Dorg.gradle.appname=$APP_BASE_NAME" \
        -jar "$APP_HOME/gradle/wrapper/gradle-wrapper.jar" \
        "$@"

# Stop when "xargs" is not available.
if ! command -v xargs >/dev/null 2>&1
then
    die "xargs is not available"
fi

# Use "xargs" to parse quoted args.
#
# With -n1 it outputs one arg per line, with the quotes and backslashes removed.
#
# In Bash we could simply go:
#
#   readarray ARGS < <( xargs -n1 <<<"$var" ) &&
#   set -- "${ARGS[@]}" "$@"
#
# but POSIX shell has neither arrays nor command substitution, so instead we
# post-process each arg (as a line of input to sed) to backslash-escape any
# character that might be a shell metacharacter, then use eval to reverse
# that process (while maintaining the separation between arguments), and wrap
# the whole thing up as a single "set" statement.
#
# This will of course break if any of these variables contains a newline or
# an unmatched quote.
#

eval "set -- $(
        printf '%s\n' "$DEFAULT_JVM_OPTS $JAVA_OPTS $GRADLE_OPTS" |
        xargs -n1 |
        sed ' s~[^-[:alnum:]+,./:=@_]~\\&~g; ' |
        tr '\n' ' '
    )" '"$@"'

exec "$JAVACMD" "$@"
"##;

const TEMPLATE_ANDROID_GRADLEW_BAT: &str = r##"@rem
@rem Copyright 2015 the original author or authors.
@rem
@rem Licensed under the Apache License, Version 2.0 (the "License");
@rem you may not use this file except in compliance with the License.
@rem You may obtain a copy of the License at
@rem
@rem      https://www.apache.org/licenses/LICENSE-2.0
@rem
@rem Unless required by applicable law or agreed to in writing, software
@rem distributed under the License is distributed on an "AS IS" BASIS,
@rem WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
@rem See the License for the specific language governing permissions and
@rem limitations under the License.
@rem
@rem SPDX-License-Identifier: Apache-2.0
@rem

@if "%DEBUG%"=="" @echo off
@rem ##########################################################################
@rem
@rem  Gradle startup script for Windows
@rem
@rem ##########################################################################

@rem Set local scope for the variables, and ensure extensions are enabled
setlocal EnableExtensions

set DIRNAME=%~dp0
if "%DIRNAME%"=="" set DIRNAME=.
@rem This is normally unused
set APP_BASE_NAME=%~n0
set APP_HOME=%DIRNAME%

@rem Resolve any "." and ".." in APP_HOME to make it shorter.
for %%i in ("%APP_HOME%") do set APP_HOME=%%~fi

@rem Add default JVM options here. You can also use JAVA_OPTS and GRADLE_OPTS to pass JVM options to this script.
set DEFAULT_JVM_OPTS="-Xmx64m" "-Xms64m"

@rem Find java.exe
if defined JAVA_HOME goto findJavaFromJavaHome

set JAVA_EXE=java.exe
%JAVA_EXE% -version >NUL 2>&1
if %ERRORLEVEL% equ 0 goto execute

echo. 1>&2
echo ERROR: JAVA_HOME is not set and no 'java' command could be found in your PATH. 1>&2
echo. 1>&2
echo Please set the JAVA_HOME variable in your environment to match the 1>&2
echo location of your Java installation. 1>&2

"%COMSPEC%" /c exit 1

:findJavaFromJavaHome
set JAVA_HOME=%JAVA_HOME:"=%
set JAVA_EXE=%JAVA_HOME%/bin/java.exe

if exist "%JAVA_EXE%" goto execute

echo. 1>&2
echo ERROR: JAVA_HOME is set to an invalid directory: %JAVA_HOME% 1>&2
echo. 1>&2
echo Please set the JAVA_HOME variable in your environment to match the 1>&2
echo location of your Java installation. 1>&2

"%COMSPEC%" /c exit 1

:execute
@rem Setup the command line



@rem Execute Gradle
@rem endlocal doesn't take effect until after the line is parsed and variables are expanded
@rem which allows us to clear the local environment before executing the java command
endlocal & "%JAVA_EXE%" %DEFAULT_JVM_OPTS% %JAVA_OPTS% %GRADLE_OPTS% "-Dorg.gradle.appname=%APP_BASE_NAME%" -jar "%APP_HOME%\gradle\wrapper\gradle-wrapper.jar" %* & call :exitWithErrorLevel

:exitWithErrorLevel
@rem Use "%COMSPEC%" /c exit to allow operators to work properly in scripts
"%COMSPEC%" /c exit %ERRORLEVEL%
"##;

const TEMPLATE_ANDROID_GRADLE_WRAPPER_PROPERTIES: &str = r##"distributionBase=GRADLE_USER_HOME
distributionPath=wrapper/dists
distributionUrl=https\://services.gradle.org/distributions/gradle-8.4-bin.zip
networkTimeout=10000
retries=0
retryBackOffMs=500
validateDistributionUrl=true
zipStoreBase=GRADLE_USER_HOME
zipStorePath=wrapper/dists
"##;

const TEMPLATE_ANDROID_GRADLE_WRAPPER_JAR: &[u8] = &[80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 9, 0, 77, 69, 84, 65, 45, 73, 78, 70, 47, 76, 73, 67, 69, 78, 83, 69, 85, 84, 5, 0, 1, 0, 0, 0, 0, 221, 90, 91, 115, 219, 54, 22, 126, 207, 175, 192, 104, 102, 103, 236, 25, 70, 73, 187, 237, 238, 182, 125, 82, 99, 167, 85, 55, 149, 51, 146, 189, 153, 62, 66, 36, 40, 97, 67, 18, 44, 64, 90, 214, 254, 250, 61, 23, 220, 40, 201, 78, 246, 117, 61, 153, 214, 162, 137, 131, 131, 115, 249, 206, 119, 14, 244, 74, 124, 233, 103, 209, 203, 114, 175, 196, 7, 93, 170, 206, 169, 87, 47, 188, 249, 47, 101, 157, 54, 157, 248, 118, 254, 182, 16, 191, 201, 110, 148, 246, 40, 190, 125, 251, 246, 187, 103, 23, 237, 135, 161, 255, 241, 205, 155, 195, 225, 48, 151, 180, 205, 220, 216, 221, 155, 134, 183, 114, 111, 94, 225, 194, 251, 219, 245, 239, 27, 177, 88, 221, 136, 119, 119, 171, 155, 229, 253, 242, 110, 181, 17, 239, 239, 214, 226, 97, 115, 91, 136, 245, 237, 199, 245, 221, 205, 195, 59, 124, 92, 208, 91, 55, 203, 205, 253, 122, 249, 243, 3, 62, 33, 1, 223, 204, 197, 141, 170, 117, 167, 7, 80, 206, 205, 95, 121, 109, 102, 254, 68, 51, 225, 246, 178, 105, 68, 171, 100, 39, 6, 56, 233, 160, 108, 235, 132, 236, 42, 81, 154, 174, 226, 85, 162, 54, 86, 140, 78, 21, 194, 170, 222, 154, 106, 44, 241, 113, 225, 69, 225, 187, 149, 118, 131, 213, 219, 17, 159, 11, 233, 68, 133, 91, 170, 74, 108, 143, 98, 163, 74, 22, 242, 13, 200, 183, 102, 220, 237, 197, 15, 194, 212, 240, 65, 195, 123, 166, 28, 91, 213, 13, 167, 122, 25, 123, 166, 88, 105, 250, 163, 213, 187, 253, 32, 204, 161, 83, 86, 128, 74, 176, 80, 15, 71, 33, 199, 97, 111, 172, 254, 15, 237, 231, 229, 92, 90, 49, 236, 229, 32, 96, 211, 157, 149, 176, 176, 219, 209, 75, 222, 14, 153, 2, 106, 39, 27, 113, 75, 162, 207, 148, 24, 59, 60, 32, 105, 175, 132, 44, 73, 74, 208, 2, 204, 0, 239, 122, 49, 6, 94, 240, 10, 106, 229, 120, 107, 48, 232, 96, 77, 83, 8, 105, 85, 248, 208, 144, 210, 5, 158, 6, 159, 142, 93, 5, 203, 74, 211, 182, 166, 243, 146, 252, 139, 226, 160, 135, 61, 203, 225, 13, 231, 226, 189, 177, 164, 71, 63, 218, 222, 64, 196, 36, 171, 70, 135, 7, 31, 205, 188, 148, 25, 29, 197, 137, 43, 125, 205, 75, 205, 65, 217, 2, 220, 103, 193, 75, 168, 132, 238, 248, 247, 66, 12, 70, 148, 18, 156, 142, 239, 121, 41, 252, 39, 178, 128, 21, 173, 236, 228, 78, 161, 243, 112, 95, 55, 150, 123, 175, 88, 33, 14, 123, 69, 199, 7, 239, 211, 190, 146, 100, 231, 150, 57, 104, 140, 38, 144, 114, 165, 65, 19, 114, 143, 219, 235, 30, 37, 213, 186, 6, 107, 246, 202, 150, 40, 250, 234, 251, 183, 127, 185, 166, 237, 12, 152, 135, 13, 31, 4, 141, 131, 27, 192, 234, 232, 3, 112, 147, 85, 46, 72, 4, 145, 91, 213, 129, 17, 74, 13, 174, 156, 72, 207, 244, 76, 46, 255, 195, 140, 51, 113, 5, 107, 241, 55, 59, 187, 206, 189, 14, 255, 208, 38, 143, 186, 26, 81, 150, 21, 121, 124, 120, 1, 234, 9, 180, 213, 14, 21, 1, 189, 91, 237, 28, 5, 60, 197, 25, 39, 1, 185, 229, 44, 212, 54, 176, 91, 9, 41, 8, 233, 213, 158, 70, 90, 111, 85, 173, 172, 133, 229, 244, 215, 154, 44, 254, 25, 183, 104, 77, 165, 225, 104, 146, 178, 42, 56, 88, 119, 101, 51, 146, 41, 32, 9, 69, 103, 6, 209, 232, 86, 227, 238, 224, 71, 103, 234, 225, 128, 225, 229, 104, 67, 112, 74, 5, 214, 15, 185, 71, 130, 188, 24, 126, 161, 8, 249, 95, 235, 221, 104, 233, 239, 224, 150, 70, 101, 240, 113, 183, 253, 55, 132, 194, 185, 234, 178, 59, 242, 51, 112, 199, 216, 80, 126, 212, 214, 180, 240, 199, 114, 47, 59, 208, 58, 36, 8, 68, 69, 231, 240, 77, 25, 2, 138, 158, 52, 254, 99, 45, 164, 96, 243, 144, 184, 98, 122, 64, 47, 227, 228, 152, 144, 54, 189, 198, 132, 50, 164, 156, 63, 230, 14, 34, 1, 206, 0, 143, 39, 7, 206, 209, 11, 78, 250, 200, 232, 237, 80, 14, 231, 110, 171, 42, 45, 197, 112, 236, 243, 99, 127, 50, 246, 243, 25, 40, 28, 224, 33, 105, 76, 56, 132, 145, 150, 82, 64, 119, 225, 24, 49, 1, 216, 116, 254, 88, 173, 172, 0, 72, 30, 165, 110, 228, 182, 9, 249, 159, 225, 82, 129, 104, 138, 1, 88, 74, 31, 74, 50, 226, 66, 64, 55, 48, 3, 188, 28, 225, 141, 45, 5, 47, 107, 50, 171, 28, 6, 172, 45, 100, 161, 160, 173, 23, 113, 5, 7, 80, 79, 178, 237, 97, 103, 88, 8, 208, 14, 97, 206, 11, 241, 205, 69, 223, 43, 216, 249, 9, 146, 169, 49, 135, 235, 100, 133, 27, 101, 245, 35, 88, 241, 81, 9, 52, 136, 155, 157, 70, 0, 238, 113, 217, 6, 254, 244, 94, 18, 219, 32, 40, 190, 149, 14, 157, 215, 81, 42, 86, 184, 7, 70, 63, 68, 15, 99, 21, 110, 69, 238, 194, 92, 56, 236, 117, 185, 207, 192, 0, 156, 53, 64, 13, 128, 204, 180, 234, 81, 147, 43, 49, 138, 193, 52, 62, 79, 132, 2, 11, 27, 27, 62, 129, 8, 239, 230, 60, 155, 188, 48, 172, 114, 202, 65, 164, 144, 245, 37, 108, 102, 26, 74, 10, 88, 166, 119, 186, 131, 93, 206, 125, 126, 142, 199, 1, 167, 234, 73, 250, 23, 226, 212, 124, 222, 122, 24, 205, 222, 119, 36, 222, 87, 13, 171, 90, 169, 99, 126, 170, 94, 90, 138, 20, 180, 11, 29, 163, 85, 86, 53, 71, 200, 131, 238, 51, 25, 110, 11, 209, 130, 113, 210, 201, 86, 93, 7, 167, 107, 0, 34, 91, 203, 146, 138, 68, 145, 213, 200, 104, 212, 51, 165, 208, 58, 202, 212, 201, 235, 239, 16, 202, 125, 141, 191, 232, 241, 211, 28, 136, 41, 155, 237, 23, 13, 232, 19, 46, 212, 210, 168, 7, 10, 155, 248, 132, 98, 184, 242, 76, 36, 72, 50, 108, 27, 90, 5, 127, 127, 78, 249, 34, 75, 138, 1, 81, 223, 192, 214, 77, 128, 109, 55, 110, 1, 59, 60, 120, 4, 222, 65, 209, 69, 154, 147, 122, 62, 21, 104, 35, 194, 241, 51, 90, 17, 188, 76, 229, 238, 197, 106, 145, 19, 21, 68, 101, 218, 30, 227, 125, 171, 192, 152, 53, 152, 226, 121, 242, 242, 117, 213, 94, 204, 226, 153, 102, 94, 22, 215, 251, 8, 203, 176, 72, 53, 144, 128, 214, 0, 24, 23, 232, 133, 173, 108, 40, 142, 14, 22, 215, 117, 68, 62, 198, 206, 91, 95, 96, 22, 228, 70, 87, 201, 80, 104, 167, 193, 165, 100, 33, 251, 187, 226, 197, 82, 20, 177, 43, 223, 3, 254, 37, 157, 0, 17, 117, 131, 139, 27, 160, 148, 32, 45, 43, 89, 145, 10, 185, 163, 27, 84, 235, 114, 8, 135, 154, 59, 42, 44, 33, 37, 213, 72, 255, 6, 187, 31, 43, 31, 179, 149, 200, 181, 114, 163, 23, 25, 140, 76, 162, 32, 179, 54, 218, 13, 56, 110, 57, 58, 170, 242, 180, 99, 75, 120, 233, 105, 228, 39, 66, 188, 84, 154, 212, 83, 48, 194, 244, 172, 33, 30, 225, 40, 174, 215, 229, 104, 70, 7, 201, 219, 74, 251, 25, 161, 207, 38, 118, 20, 40, 151, 114, 122, 215, 17, 246, 67, 40, 162, 143, 200, 176, 23, 35, 17, 193, 106, 182, 2, 123, 75, 145, 231, 234, 124, 118, 158, 194, 39, 252, 58, 30, 59, 100, 224, 23, 41, 79, 110, 64, 196, 199, 246, 100, 83, 177, 7, 101, 182, 10, 226, 9, 40, 163, 34, 36, 7, 165, 243, 125, 82, 18, 58, 245, 231, 8, 241, 211, 224, 182, 165, 1, 123, 115, 185, 70, 194, 155, 165, 31, 3, 209, 183, 115, 241, 11, 210, 42, 220, 246, 93, 60, 126, 96, 86, 98, 51, 114, 113, 245, 177, 122, 177, 153, 201, 210, 44, 71, 101, 5, 85, 82, 100, 6, 18, 8, 33, 160, 51, 177, 56, 226, 5, 64, 14, 225, 148, 192, 240, 122, 53, 128, 101, 66, 248, 1, 244, 53, 213, 65, 35, 215, 232, 76, 247, 154, 60, 239, 224, 196, 248, 241, 53, 176, 30, 187, 195, 198, 201, 28, 101, 51, 28, 95, 215, 86, 193, 39, 13, 196, 238, 209, 148, 8, 228, 103, 213, 220, 247, 127, 184, 97, 232, 182, 96, 5, 228, 88, 143, 113, 124, 134, 116, 9, 206, 251, 113, 11, 107, 193, 138, 16, 168, 125, 35, 33, 208, 227, 19, 208, 153, 75, 173, 163, 39, 158, 88, 228, 125, 91, 78, 243, 35, 22, 19, 89, 62, 219, 241, 66, 57, 39, 108, 97, 7, 253, 53, 115, 208, 71, 137, 160, 251, 127, 224, 157, 43, 88, 166, 250, 1, 19, 12, 90, 142, 33, 80, 36, 80, 208, 113, 67, 116, 45, 122, 62, 107, 230, 61, 160, 235, 32, 108, 47, 31, 21, 177, 188, 160, 16, 245, 209, 166, 174, 145, 231, 65, 17, 80, 13, 192, 47, 255, 23, 16, 197, 216, 129, 29, 19, 113, 192, 19, 101, 207, 10, 9, 102, 194, 201, 208, 4, 236, 163, 176, 171, 236, 251, 6, 219, 77, 211, 129, 211, 201, 202, 136, 93, 94, 181, 178, 145, 26, 236, 205, 239, 102, 135, 3, 43, 146, 144, 220, 186, 17, 55, 59, 200, 94, 231, 164, 213, 148, 157, 181, 5, 244, 9, 29, 141, 210, 161, 246, 229, 137, 127, 229, 174, 161, 13, 54, 157, 242, 21, 17, 224, 15, 24, 73, 100, 245, 180, 236, 116, 65, 56, 16, 119, 184, 190, 218, 130, 250, 76, 242, 166, 202, 249, 45, 14, 232, 138, 80, 235, 230, 98, 89, 163, 255, 99, 47, 228, 0, 169, 48, 166, 163, 83, 6, 189, 99, 21, 228, 78, 226, 159, 9, 228, 124, 227, 126, 149, 10, 86, 228, 214, 214, 56, 247, 154, 12, 134, 199, 40, 205, 136, 252, 137, 63, 131, 231, 165, 104, 228, 193, 141, 122, 192, 163, 54, 106, 199, 69, 0, 44, 22, 148, 79, 156, 224, 4, 21, 95, 2, 56, 170, 9, 172, 184, 243, 173, 118, 146, 83, 38, 231, 28, 195, 177, 130, 63, 90, 98, 170, 32, 134, 169, 216, 52, 18, 3, 101, 10, 205, 168, 207, 148, 208, 104, 164, 28, 243, 37, 47, 176, 42, 174, 14, 152, 162, 232, 189, 16, 43, 210, 5, 194, 86, 193, 195, 16, 124, 209, 186, 32, 13, 251, 196, 138, 161, 224, 187, 185, 88, 171, 124, 50, 52, 167, 173, 91, 121, 76, 200, 118, 138, 66, 128, 131, 58, 112, 155, 9, 30, 189, 192, 242, 200, 37, 72, 27, 97, 179, 17, 64, 142, 226, 8, 25, 13, 252, 223, 196, 138, 60, 109, 155, 185, 132, 63, 131, 100, 69, 106, 133, 200, 32, 41, 180, 90, 165, 216, 203, 181, 105, 160, 39, 226, 250, 30, 176, 235, 199, 80, 103, 175, 228, 53, 159, 116, 132, 72, 219, 161, 190, 168, 30, 247, 27, 224, 86, 13, 71, 68, 208, 202, 169, 111, 236, 14, 241, 231, 236, 160, 146, 234, 195, 105, 39, 241, 19, 149, 209, 176, 231, 54, 219, 147, 7, 55, 137, 74, 99, 31, 133, 253, 59, 15, 117, 44, 134, 16, 180, 15, 186, 195, 56, 225, 238, 209, 101, 219, 35, 196, 197, 144, 70, 153, 216, 186, 239, 200, 24, 138, 229, 76, 119, 46, 179, 157, 173, 26, 32, 193, 138, 192, 155, 179, 22, 158, 186, 3, 208, 232, 244, 112, 217, 198, 113, 195, 20, 16, 5, 102, 88, 170, 142, 133, 143, 238, 2, 97, 177, 82, 200, 155, 138, 140, 76, 80, 136, 14, 41, 221, 252, 217, 120, 4, 113, 65, 159, 83, 72, 197, 159, 196, 220, 24, 61, 131, 12, 82, 174, 50, 68, 104, 161, 202, 224, 49, 209, 156, 156, 113, 118, 72, 133, 139, 79, 114, 94, 170, 167, 70, 171, 174, 17, 180, 162, 255, 125, 227, 135, 174, 158, 173, 238, 238, 151, 239, 110, 103, 144, 124, 79, 3, 217, 27, 211, 206, 239, 129, 148, 59, 219, 39, 207, 174, 12, 2, 46, 100, 202, 153, 101, 201, 95, 153, 168, 208, 122, 74, 240, 161, 172, 168, 199, 76, 65, 167, 46, 154, 21, 65, 73, 226, 156, 55, 19, 227, 65, 141, 144, 129, 15, 66, 71, 40, 190, 198, 174, 153, 152, 203, 22, 190, 104, 87, 10, 54, 144, 209, 40, 233, 176, 157, 202, 167, 244, 126, 73, 202, 86, 32, 70, 176, 233, 143, 65, 77, 25, 116, 76, 182, 78, 22, 154, 68, 149, 123, 81, 135, 159, 114, 48, 159, 4, 89, 158, 215, 211, 1, 148, 208, 117, 194, 25, 44, 153, 187, 84, 1, 207, 229, 27, 91, 156, 91, 89, 6, 174, 151, 77, 185, 124, 111, 112, 193, 74, 245, 73, 166, 16, 129, 128, 14, 144, 157, 5, 2, 109, 245, 26, 15, 121, 140, 190, 233, 112, 62, 7, 13, 51, 18, 11, 37, 161, 9, 189, 223, 115, 23, 134, 248, 117, 110, 230, 204, 223, 68, 30, 184, 149, 142, 67, 62, 232, 33, 82, 243, 138, 12, 101, 170, 142, 207, 45, 66, 172, 227, 100, 54, 31, 203, 134, 172, 42, 252, 221, 98, 191, 147, 71, 100, 38, 37, 168, 238, 45, 244, 53, 153, 80, 176, 245, 29, 56, 34, 63, 19, 245, 83, 56, 222, 168, 42, 213, 85, 99, 27, 104, 235, 36, 98, 2, 176, 112, 255, 23, 220, 121, 138, 105, 100, 224, 48, 196, 0, 51, 92, 76, 38, 154, 86, 65, 207, 196, 60, 192, 142, 167, 241, 199, 134, 121, 238, 222, 226, 162, 137, 82, 87, 65, 180, 149, 134, 245, 76, 0, 78, 6, 95, 153, 43, 80, 136, 63, 71, 174, 50, 142, 228, 52, 178, 214, 9, 203, 189, 192, 224, 211, 104, 239, 194, 149, 17, 139, 201, 238, 138, 76, 125, 65, 155, 34, 165, 77, 77, 205, 226, 241, 153, 86, 36, 159, 206, 197, 84, 34, 121, 184, 117, 54, 205, 75, 10, 156, 221, 86, 77, 170, 112, 100, 221, 56, 75, 38, 42, 141, 113, 52, 25, 203, 196, 78, 229, 164, 19, 152, 56, 228, 123, 106, 118, 252, 77, 0, 247, 170, 137, 5, 186, 185, 120, 232, 160, 138, 58, 114, 154, 122, 130, 141, 74, 141, 237, 47, 73, 204, 46, 72, 226, 124, 227, 120, 202, 34, 179, 97, 86, 54, 198, 122, 118, 116, 149, 152, 62, 238, 120, 58, 200, 97, 170, 183, 205, 167, 207, 255, 75, 107, 230, 105, 22, 169, 153, 5, 12, 139, 96, 234, 90, 133, 219, 71, 94, 191, 50, 3, 46, 138, 183, 55, 84, 95, 182, 134, 155, 50, 76, 219, 29, 181, 119, 88, 70, 72, 53, 55, 66, 57, 112, 170, 82, 124, 17, 132, 105, 144, 185, 196, 111, 196, 236, 130, 7, 164, 96, 197, 216, 18, 237, 160, 167, 163, 192, 63, 250, 12, 161, 142, 76, 61, 169, 50, 131, 120, 2, 222, 104, 16, 171, 118, 210, 242, 189, 210, 105, 239, 225, 239, 2, 254, 6, 80, 24, 8, 136, 67, 88, 204, 120, 116, 101, 8, 57, 7, 166, 220, 217, 141, 16, 26, 222, 95, 168, 49, 125, 9, 215, 24, 178, 197, 185, 89, 100, 52, 56, 245, 82, 246, 17, 103, 250, 254, 35, 232, 228, 99, 152, 95, 14, 65, 27, 52, 14, 145, 146, 218, 84, 171, 254, 28, 181, 191, 61, 194, 130, 238, 192, 39, 88, 210, 201, 165, 80, 248, 77, 139, 215, 211, 168, 13, 88, 25, 120, 71, 9, 7, 244, 174, 136, 77, 7, 78, 106, 207, 230, 179, 33, 155, 130, 223, 124, 53, 184, 80, 2, 216, 82, 127, 159, 139, 27, 237, 168, 117, 194, 75, 219, 90, 124, 2, 254, 9, 118, 57, 198, 36, 136, 170, 110, 143, 220, 192, 82, 231, 141, 45, 86, 130, 1, 242, 34, 53, 47, 105, 10, 86, 36, 135, 249, 220, 119, 73, 213, 43, 212, 21, 135, 6, 167, 45, 106, 254, 54, 142, 47, 39, 206, 189, 198, 185, 22, 64, 254, 108, 177, 17, 203, 205, 76, 252, 188, 216, 44, 55, 193, 184, 159, 150, 247, 191, 222, 61, 220, 139, 79, 139, 245, 122, 177, 186, 95, 222, 110, 196, 221, 58, 191, 150, 191, 123, 47, 22, 171, 63, 196, 63, 151, 171, 27, 160, 59, 154, 111, 128, 159, 112, 58, 234, 210, 73, 52, 225, 74, 149, 141, 73, 83, 6, 209, 156, 84, 6, 156, 58, 66, 147, 75, 166, 162, 134, 200, 158, 67, 44, 24, 243, 126, 121, 255, 225, 182, 0, 171, 175, 94, 47, 87, 239, 215, 203, 213, 47, 183, 191, 223, 174, 238, 11, 241, 251, 237, 250, 221, 175, 160, 229, 226, 231, 229, 135, 229, 253, 31, 20, 66, 239, 151, 247, 171, 219, 13, 127, 125, 96, 225, 101, 124, 92, 172, 193, 97, 15, 31, 22, 107, 241, 241, 97, 253, 241, 110, 115, 203, 213, 150, 111, 11, 27, 188, 89, 0, 253, 123, 216, 84, 211, 173, 3, 221, 204, 112, 87, 56, 13, 23, 240, 156, 53, 189, 213, 72, 207, 233, 192, 53, 68, 23, 190, 66, 241, 151, 16, 55, 155, 151, 242, 180, 209, 57, 224, 68, 120, 220, 0, 215, 218, 17, 178, 59, 83, 234, 216, 38, 51, 168, 251, 123, 86, 154, 198, 230, 23, 173, 231, 205, 44, 199, 222, 63, 230, 240, 57, 152, 20, 23, 125, 208, 114, 171, 27, 186, 60, 95, 98, 229, 21, 64, 127, 186, 129, 244, 96, 25, 240, 168, 161, 97, 39, 232, 8, 157, 118, 54, 106, 9, 55, 89, 16, 64, 67, 62, 50, 232, 212, 174, 209, 192, 190, 74, 117, 93, 196, 219, 238, 98, 50, 202, 141, 147, 159, 47, 198, 251, 21, 19, 5, 156, 233, 55, 122, 75, 132, 142, 148, 219, 225, 60, 34, 222, 91, 132, 45, 7, 252, 6, 130, 163, 219, 241, 203, 249, 193, 232, 57, 41, 31, 56, 148, 9, 46, 107, 52, 109, 236, 39, 2, 228, 90, 217, 202, 221, 116, 134, 143, 171, 195, 87, 2, 210, 151, 3, 92, 175, 240, 110, 61, 187, 125, 134, 132, 2, 98, 203, 87, 9, 72, 96, 120, 166, 139, 23, 114, 94, 104, 64, 104, 156, 185, 129, 222, 56, 174, 182, 124, 103, 142, 85, 60, 214, 106, 188, 53, 62, 109, 116, 201, 154, 99, 196, 152, 145, 159, 232, 206, 59, 51, 195, 213, 124, 98, 112, 245, 226, 157, 120, 208, 10, 143, 221, 24, 14, 216, 157, 49, 213, 65, 55, 249, 236, 240, 51, 20, 101, 211, 247, 18, 167, 132, 200, 9, 70, 84, 188, 150, 186, 25, 45, 87, 35, 217, 212, 99, 151, 200, 13, 21, 193, 11, 223, 4, 193, 91, 0, 12, 222, 220, 30, 188, 177, 114, 16, 56, 24, 135, 72, 208, 79, 7, 113, 94, 70, 28, 166, 203, 234, 81, 211, 37, 105, 237, 191, 190, 1, 25, 224, 141, 16, 190, 220, 224, 197, 115, 6, 252, 48, 23, 139, 18, 107, 2, 90, 33, 32, 47, 238, 188, 72, 133, 58, 75, 138, 79, 123, 164, 238, 211, 116, 61, 189, 44, 124, 241, 186, 45, 176, 208, 114, 111, 12, 79, 65, 105, 210, 57, 185, 108, 167, 153, 43, 240, 182, 90, 17, 158, 0, 212, 145, 134, 178, 43, 21, 31, 162, 231, 49, 168, 71, 191, 35, 197, 157, 106, 59, 252, 106, 73, 26, 136, 177, 89, 155, 160, 187, 48, 219, 198, 79, 161, 136, 183, 188, 65, 216, 65, 230, 203, 87, 45, 112, 30, 204, 23, 223, 95, 233, 128, 160, 177, 193, 248, 213, 28, 176, 19, 226, 86, 50, 26, 140, 236, 153, 9, 78, 231, 163, 111, 180, 116, 77, 118, 27, 18, 57, 183, 191, 22, 161, 33, 174, 127, 140, 64, 154, 96, 148, 244, 37, 166, 147, 110, 81, 18, 162, 167, 73, 81, 22, 6, 126, 38, 140, 61, 147, 174, 25, 159, 49, 225, 57, 223, 201, 54, 117, 180, 77, 165, 106, 104, 87, 120, 5, 48, 227, 234, 194, 232, 92, 218, 150, 144, 40, 144, 235, 104, 197, 148, 206, 163, 181, 233, 182, 204, 79, 142, 1, 147, 161, 43, 199, 102, 149, 135, 168, 197, 249, 220, 120, 123, 244, 100, 35, 29, 232, 136, 22, 72, 54, 141, 100, 254, 144, 69, 99, 70, 27, 163, 46, 28, 192, 183, 171, 27, 172, 171, 151, 190, 6, 247, 234, 191, 80, 75, 7, 8, 176, 183, 163, 30, 233, 13, 0, 0, 190, 39, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 9, 0, 77, 69, 84, 65, 45, 73, 78, 70, 47, 77, 65, 78, 73, 70, 69, 83, 84, 46, 77, 70, 85, 84, 5, 0, 1, 0, 0, 0, 0, 45, 205, 205, 10, 131, 48, 16, 4, 224, 123, 32, 239, 144, 23, 216, 208, 246, 152, 91, 168, 82, 4, 149, 66, 127, 175, 219, 184, 106, 32, 198, 144, 132, 246, 245, 171, 181, 215, 97, 230, 155, 6, 189, 237, 41, 101, 184, 83, 76, 118, 246, 74, 236, 229, 142, 179, 106, 10, 142, 38, 242, 25, 243, 18, 194, 213, 102, 71, 74, 156, 34, 118, 142, 196, 35, 98, 8, 20, 57, 187, 156, 139, 39, 212, 214, 144, 79, 4, 85, 183, 212, 109, 111, 41, 42, 161, 3, 154, 145, 224, 176, 82, 13, 90, 15, 71, 135, 41, 41, 49, 199, 65, 14, 63, 68, 126, 54, 68, 110, 230, 159, 92, 187, 156, 149, 30, 95, 142, 160, 93, 190, 223, 4, 218, 24, 90, 183, 186, 174, 225, 214, 182, 186, 41, 11, 206, 56, 251, 2, 80, 75, 7, 8, 106, 207, 203, 90, 149, 0, 0, 0, 185, 0, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 65, 114, 103, 117, 109, 101, 110, 116, 69, 120, 99, 101, 112, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 77, 79, 205, 74, 3, 49, 16, 158, 180, 181, 173, 181, 30, 20, 188, 120, 204, 73, 109, 183, 139, 214, 194, 82, 69, 144, 162, 167, 158, 90, 240, 30, 179, 211, 52, 54, 201, 46, 201, 110, 17, 196, 62, 136, 111, 225, 73, 240, 224, 3, 248, 80, 226, 44, 40, 58, 3, 3, 223, 207, 252, 125, 126, 189, 127, 0, 192, 57, 236, 50, 120, 217, 108, 102, 201, 19, 191, 23, 114, 133, 46, 229, 99, 46, 23, 188, 207, 101, 102, 115, 109, 68, 161, 51, 23, 217, 44, 69, 226, 61, 26, 20, 1, 73, 92, 138, 16, 201, 37, 202, 85, 40, 109, 224, 227, 133, 48, 1, 251, 60, 87, 145, 21, 121, 164, 171, 25, 136, 103, 163, 161, 72, 200, 235, 147, 223, 254, 69, 105, 12, 17, 97, 41, 162, 211, 202, 226, 148, 118, 136, 94, 59, 69, 236, 26, 125, 160, 93, 196, 39, 131, 225, 32, 137, 82, 92, 243, 231, 54, 48, 6, 157, 121, 86, 122, 137, 183, 218, 32, 131, 94, 230, 85, 172, 188, 72, 13, 198, 210, 232, 120, 146, 89, 43, 92, 58, 165, 73, 215, 94, 149, 22, 93, 113, 243, 40, 49, 175, 238, 110, 65, 131, 193, 225, 131, 88, 139, 216, 8, 167, 226, 89, 233, 10, 109, 241, 159, 222, 100, 208, 188, 212, 78, 23, 87, 12, 14, 142, 166, 127, 214, 121, 81, 157, 117, 113, 124, 215, 133, 54, 108, 119, 160, 5, 29, 6, 141, 9, 253, 1, 123, 176, 69, 176, 10, 70, 73, 42, 213, 46, 161, 125, 168, 81, 2, 52, 79, 122, 111, 176, 243, 250, 227, 168, 83, 173, 65, 253, 27, 80, 75, 7, 8, 227, 7, 235, 72, 33, 1, 0, 0, 112, 1, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 79, 112, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 101, 83, 91, 79, 19, 81, 16, 254, 14, 109, 89, 218, 46, 151, 22, 138, 224, 21, 215, 11, 109, 105, 169, 220, 180, 82, 226, 11, 241, 66, 82, 196, 88, 2, 193, 248, 114, 216, 61, 108, 15, 108, 119, 155, 221, 45, 209, 24, 249, 31, 250, 7, 124, 85, 35, 18, 52, 49, 62, 251, 59, 252, 19, 190, 136, 179, 11, 149, 18, 94, 206, 204, 153, 253, 102, 190, 57, 223, 204, 254, 250, 251, 237, 7, 128, 89, 44, 48, 188, 223, 219, 123, 94, 126, 163, 109, 114, 125, 71, 216, 134, 54, 175, 233, 91, 90, 65, 211, 157, 70, 83, 90, 220, 151, 142, 93, 108, 56, 134, 160, 184, 43, 44, 193, 61, 65, 31, 235, 220, 43, 234, 117, 161, 239, 120, 173, 134, 167, 205, 111, 113, 203, 19, 5, 173, 105, 22, 27, 188, 89, 148, 65, 13, 33, 166, 231, 102, 120, 153, 176, 110, 185, 157, 191, 213, 178, 44, 10, 120, 117, 94, 156, 10, 32, 182, 41, 109, 33, 92, 105, 155, 20, 221, 21, 174, 71, 92, 20, 47, 79, 206, 76, 150, 139, 134, 216, 213, 222, 246, 128, 49, 36, 106, 78, 203, 213, 197, 35, 105, 9, 134, 49, 199, 53, 75, 166, 203, 13, 75, 148, 116, 75, 150, 22, 157, 70, 131, 219, 70, 149, 42, 173, 52, 131, 102, 21, 68, 25, 6, 182, 249, 46, 47, 89, 220, 54, 75, 43, 155, 219, 66, 247, 21, 116, 51, 40, 78, 136, 240, 24, 6, 171, 33, 160, 229, 75, 171, 244, 132, 123, 245, 154, 240, 43, 12, 42, 119, 205, 86, 67, 216, 254, 234, 235, 38, 81, 165, 170, 167, 85, 22, 45, 238, 121, 4, 73, 26, 194, 211, 93, 25, 214, 97, 72, 119, 32, 106, 126, 240, 16, 130, 196, 77, 215, 105, 53, 215, 165, 95, 103, 232, 94, 144, 182, 244, 31, 16, 97, 182, 131, 177, 42, 61, 191, 146, 91, 99, 136, 100, 115, 107, 42, 250, 145, 74, 64, 65, 154, 24, 207, 117, 165, 96, 40, 129, 12, 210, 42, 122, 16, 143, 35, 134, 11, 12, 125, 167, 164, 107, 142, 52, 20, 140, 50, 68, 87, 55, 158, 61, 84, 113, 9, 201, 56, 46, 226, 178, 138, 68, 224, 197, 112, 85, 69, 223, 113, 226, 24, 181, 123, 154, 184, 228, 11, 151, 111, 90, 66, 129, 198, 208, 35, 131, 155, 239, 184, 12, 195, 217, 92, 71, 163, 75, 39, 241, 138, 138, 155, 184, 149, 196, 13, 220, 110, 87, 57, 243, 93, 65, 150, 212, 165, 165, 120, 42, 94, 249, 225, 179, 94, 168, 200, 99, 34, 137, 28, 10, 212, 156, 29, 134, 135, 218, 181, 59, 230, 66, 149, 39, 81, 10, 112, 119, 206, 76, 237, 88, 77, 5, 211, 84, 141, 27, 6, 67, 38, 123, 62, 55, 96, 153, 197, 92, 32, 208, 93, 90, 19, 83, 248, 43, 237, 1, 103, 206, 188, 227, 116, 196, 209, 69, 90, 69, 134, 222, 154, 79, 219, 190, 204, 155, 171, 129, 8, 72, 145, 62, 10, 253, 14, 81, 242, 72, 103, 242, 88, 32, 96, 104, 85, 244, 146, 77, 5, 50, 146, 237, 162, 72, 63, 6, 232, 156, 167, 219, 14, 186, 17, 33, 251, 120, 34, 191, 241, 242, 0, 131, 223, 145, 217, 56, 192, 240, 62, 70, 62, 227, 202, 62, 174, 253, 191, 95, 63, 196, 56, 67, 117, 226, 16, 69, 134, 119, 24, 205, 147, 55, 197, 240, 19, 51, 203, 95, 48, 82, 248, 138, 123, 235, 31, 142, 126, 127, 10, 9, 43, 116, 166, 209, 117, 132, 113, 116, 41, 136, 41, 164, 13, 254, 224, 42, 181, 80, 198, 253, 19, 226, 20, 89, 70, 54, 150, 167, 244, 143, 64, 152, 24, 9, 251, 139, 252, 3, 80, 75, 7, 8, 173, 121, 130, 158, 141, 2, 0, 0, 219, 3, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 65, 102, 116, 101, 114, 79, 112, 116, 105, 111, 110, 115, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 149, 83, 109, 79, 211, 80, 20, 126, 46, 27, 148, 117, 99, 128, 8, 248, 46, 86, 144, 189, 80, 22, 69, 147, 9, 198, 4, 73, 76, 72, 22, 48, 162, 24, 248, 98, 238, 218, 187, 174, 172, 189, 93, 110, 59, 148, 24, 249, 33, 254, 6, 63, 104, 130, 146, 104, 226, 15, 240, 71, 25, 79, 183, 17, 16, 150, 44, 182, 201, 61, 167, 247, 60, 207, 57, 79, 207, 61, 247, 247, 159, 31, 191, 0, 60, 68, 129, 225, 211, 225, 225, 203, 242, 7, 163, 202, 173, 134, 144, 182, 177, 108, 88, 53, 99, 193, 176, 2, 191, 233, 122, 60, 114, 3, 105, 250, 129, 45, 104, 95, 9, 79, 240, 80, 80, 176, 206, 67, 211, 170, 11, 171, 17, 182, 252, 208, 88, 174, 113, 47, 20, 11, 70, 211, 49, 125, 222, 52, 221, 56, 135, 16, 15, 30, 45, 241, 50, 97, 85, 249, 132, 95, 107, 121, 30, 109, 132, 117, 110, 222, 143, 33, 210, 113, 165, 16, 202, 149, 14, 237, 238, 11, 21, 82, 45, 218, 47, 47, 46, 45, 150, 77, 91, 236, 27, 31, 135, 193, 24, 244, 173, 160, 165, 44, 241, 220, 245, 4, 131, 25, 40, 167, 228, 40, 110, 123, 162, 100, 121, 110, 105, 45, 240, 125, 46, 237, 10, 101, 122, 193, 85, 40, 212, 236, 106, 45, 18, 106, 179, 25, 11, 15, 53, 36, 25, 22, 250, 82, 58, 102, 43, 226, 145, 208, 48, 196, 144, 182, 78, 33, 12, 70, 229, 92, 130, 54, 220, 62, 147, 102, 133, 97, 232, 137, 43, 221, 232, 41, 195, 92, 174, 63, 60, 191, 205, 144, 204, 173, 231, 183, 51, 208, 145, 209, 161, 97, 36, 131, 97, 164, 82, 24, 196, 40, 195, 152, 207, 15, 170, 130, 228, 168, 168, 243, 31, 12, 147, 185, 202, 30, 223, 231, 37, 143, 75, 167, 180, 21, 197, 61, 91, 201, 239, 50, 140, 4, 242, 31, 220, 110, 15, 92, 15, 230, 121, 137, 23, 91, 210, 73, 120, 166, 49, 244, 143, 143, 251, 178, 94, 203, 134, 12, 222, 201, 11, 100, 13, 147, 12, 162, 151, 182, 190, 189, 234, 47, 245, 172, 200, 78, 75, 167, 117, 76, 225, 10, 157, 99, 32, 55, 2, 121, 210, 155, 103, 189, 122, 248, 127, 233, 25, 102, 250, 9, 214, 112, 131, 33, 43, 222, 71, 138, 175, 42, 167, 229, 11, 25, 133, 116, 126, 157, 210, 173, 200, 245, 74, 171, 74, 241, 131, 138, 27, 70, 43, 25, 220, 194, 237, 20, 110, 98, 134, 97, 162, 7, 64, 131, 193, 144, 224, 182, 125, 110, 0, 54, 171, 123, 194, 138, 104, 0, 50, 152, 197, 156, 142, 187, 184, 71, 3, 181, 70, 183, 140, 97, 52, 22, 177, 209, 242, 171, 66, 189, 226, 85, 79, 96, 156, 134, 74, 163, 187, 206, 200, 163, 25, 35, 47, 73, 190, 142, 52, 173, 57, 250, 154, 70, 2, 3, 100, 211, 133, 157, 196, 49, 178, 197, 111, 24, 251, 138, 248, 25, 167, 247, 82, 23, 148, 37, 27, 131, 6, 18, 159, 187, 177, 9, 92, 238, 198, 230, 169, 64, 130, 236, 232, 79, 76, 237, 20, 142, 48, 86, 124, 91, 56, 198, 213, 47, 237, 154, 121, 90, 135, 200, 166, 219, 245, 175, 225, 122, 151, 84, 232, 86, 205, 22, 118, 136, 113, 132, 59, 197, 239, 152, 127, 115, 202, 209, 41, 58, 72, 126, 138, 44, 107, 167, 31, 64, 226, 47, 80, 75, 7, 8, 3, 152, 98, 196, 90, 2, 0, 0, 182, 4, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 66, 101, 102, 111, 114, 101, 70, 105, 114, 115, 116, 83, 117, 98, 67, 111, 109, 109, 97, 110, 100, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 181, 85, 109, 83, 19, 87, 20, 126, 46, 164, 44, 172, 139, 188, 85, 75, 91, 171, 97, 11, 4, 146, 44, 41, 68, 33, 16, 181, 5, 124, 161, 35, 168, 99, 172, 51, 161, 51, 237, 220, 108, 46, 201, 202, 190, 100, 118, 55, 208, 78, 71, 254, 135, 126, 238, 15, 176, 51, 86, 28, 105, 213, 111, 206, 248, 163, 212, 179, 187, 113, 228, 77, 150, 47, 38, 179, 155, 236, 115, 159, 115, 238, 121, 206, 57, 247, 236, 155, 183, 207, 95, 0, 56, 143, 159, 25, 30, 109, 109, 221, 41, 252, 165, 86, 184, 190, 46, 236, 170, 58, 167, 234, 107, 106, 86, 213, 29, 171, 97, 152, 220, 55, 28, 91, 179, 156, 170, 32, 220, 21, 166, 224, 158, 160, 197, 58, 247, 52, 189, 46, 244, 117, 175, 105, 121, 234, 220, 26, 55, 61, 145, 85, 27, 53, 205, 226, 13, 205, 8, 124, 8, 49, 117, 33, 207, 11, 196, 117, 11, 31, 236, 215, 154, 166, 73, 128, 87, 231, 218, 100, 64, 177, 107, 134, 45, 132, 107, 216, 53, 66, 55, 132, 235, 209, 94, 132, 23, 38, 242, 19, 5, 173, 42, 54, 212, 7, 157, 96, 12, 114, 201, 105, 186, 186, 184, 102, 152, 130, 97, 218, 113, 107, 185, 154, 203, 171, 166, 200, 233, 166, 145, 91, 116, 44, 139, 219, 213, 101, 242, 116, 155, 187, 158, 112, 135, 23, 196, 154, 227, 18, 219, 245, 252, 82, 179, 210, 90, 151, 144, 96, 152, 137, 181, 189, 213, 8, 244, 206, 111, 114, 183, 133, 148, 124, 238, 11, 9, 29, 12, 29, 126, 221, 240, 134, 127, 96, 80, 151, 227, 220, 20, 137, 125, 209, 176, 13, 255, 50, 195, 245, 177, 120, 250, 126, 70, 8, 87, 119, 241, 138, 227, 247, 20, 116, 162, 171, 11, 95, 64, 81, 32, 227, 132, 12, 9, 39, 25, 186, 29, 155, 34, 116, 253, 40, 110, 134, 213, 177, 229, 251, 124, 131, 231, 76, 110, 215, 114, 37, 63, 72, 109, 241, 32, 50, 30, 27, 82, 43, 17, 187, 114, 64, 154, 180, 99, 90, 69, 187, 72, 232, 103, 72, 29, 47, 158, 123, 129, 166, 47, 101, 12, 224, 20, 67, 50, 110, 27, 9, 95, 49, 244, 56, 225, 94, 222, 194, 159, 145, 19, 134, 129, 200, 113, 211, 55, 204, 220, 18, 247, 234, 43, 188, 81, 84, 240, 53, 190, 233, 194, 32, 190, 101, 232, 59, 176, 44, 225, 59, 134, 246, 154, 240, 25, 70, 119, 7, 122, 171, 114, 95, 232, 62, 165, 233, 0, 164, 224, 28, 146, 50, 206, 98, 232, 200, 56, 163, 60, 72, 248, 158, 162, 226, 166, 233, 108, 254, 98, 175, 219, 206, 166, 29, 225, 30, 3, 91, 85, 48, 130, 209, 32, 178, 20, 195, 108, 108, 98, 247, 216, 239, 233, 204, 113, 134, 19, 250, 71, 254, 33, 237, 121, 176, 155, 20, 100, 144, 237, 162, 14, 210, 24, 196, 97, 21, 138, 245, 16, 223, 64, 187, 91, 39, 170, 111, 78, 70, 26, 116, 126, 50, 159, 182, 157, 119, 107, 77, 75, 216, 254, 213, 63, 116, 209, 74, 225, 20, 67, 239, 254, 50, 72, 56, 207, 48, 212, 202, 73, 178, 165, 94, 51, 201, 65, 50, 234, 138, 100, 106, 196, 75, 77, 116, 98, 122, 143, 241, 135, 190, 44, 208, 241, 164, 17, 97, 113, 170, 251, 236, 33, 242, 127, 61, 186, 21, 90, 44, 5, 115, 40, 202, 152, 197, 69, 134, 83, 135, 120, 137, 68, 95, 150, 145, 199, 143, 199, 25, 61, 55, 62, 81, 224, 121, 134, 191, 227, 103, 200, 158, 163, 119, 84, 121, 34, 222, 103, 42, 241, 162, 140, 5, 92, 97, 72, 44, 210, 192, 167, 233, 68, 107, 250, 58, 29, 180, 187, 188, 18, 204, 238, 158, 192, 248, 102, 211, 170, 8, 55, 68, 208, 71, 243, 76, 162, 215, 16, 235, 237, 11, 198, 27, 253, 107, 3, 11, 198, 27, 221, 175, 209, 211, 32, 218, 233, 11, 40, 233, 114, 230, 95, 116, 103, 178, 219, 232, 249, 7, 193, 167, 15, 189, 116, 69, 172, 255, 208, 129, 4, 253, 254, 150, 222, 193, 64, 249, 102, 192, 58, 253, 4, 221, 79, 112, 38, 251, 20, 234, 43, 12, 151, 87, 94, 35, 159, 14, 161, 177, 135, 232, 223, 65, 186, 76, 79, 19, 153, 223, 211, 219, 152, 124, 188, 131, 124, 57, 241, 63, 46, 148, 111, 180, 107, 165, 254, 153, 244, 51, 92, 218, 198, 79, 47, 119, 176, 16, 178, 150, 181, 108, 134, 120, 87, 31, 135, 209, 93, 167, 123, 18, 109, 239, 48, 29, 110, 41, 209, 200, 194, 59, 156, 70, 66, 10, 165, 12, 135, 0, 150, 232, 26, 37, 61, 244, 250, 34, 45, 109, 56, 73, 42, 250, 201, 96, 136, 72, 41, 138, 118, 138, 168, 211, 180, 186, 74, 28, 22, 42, 108, 67, 251, 123, 80, 75, 7, 8, 108, 71, 224, 210, 82, 3, 0, 0, 147, 7, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 75, 110, 111, 119, 110, 79, 112, 116, 105, 111, 110, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 157, 86, 235, 95, 28, 213, 25, 126, 6, 22, 6, 150, 73, 34, 228, 6, 73, 52, 155, 205, 13, 150, 93, 48, 23, 21, 129, 166, 2, 70, 141, 44, 144, 6, 2, 221, 152, 170, 195, 238, 1, 38, 204, 238, 108, 102, 102, 33, 104, 77, 122, 191, 168, 189, 217, 43, 214, 214, 94, 162, 169, 53, 109, 163, 93, 32, 145, 95, 91, 63, 245, 67, 191, 183, 255, 64, 251, 177, 127, 128, 173, 85, 250, 156, 217, 93, 216, 192, 166, 132, 126, 153, 115, 230, 61, 239, 229, 121, 175, 231, 252, 229, 227, 247, 254, 8, 224, 56, 22, 21, 188, 122, 249, 242, 153, 182, 231, 131, 163, 122, 124, 82, 164, 18, 193, 246, 96, 124, 44, 24, 14, 198, 173, 100, 218, 48, 117, 215, 176, 82, 145, 164, 149, 16, 164, 219, 194, 20, 186, 35, 120, 56, 161, 59, 145, 248, 132, 136, 79, 58, 153, 164, 19, 108, 31, 211, 77, 71, 132, 131, 233, 241, 72, 82, 79, 71, 12, 169, 67, 136, 163, 15, 28, 211, 219, 200, 107, 183, 21, 228, 199, 50, 166, 73, 130, 51, 161, 71, 142, 72, 150, 212, 184, 145, 18, 194, 54, 82, 227, 164, 78, 9, 219, 161, 45, 210, 219, 90, 142, 181, 180, 69, 18, 98, 42, 248, 66, 21, 20, 5, 254, 65, 43, 99, 199, 197, 99, 134, 41, 20, 60, 100, 217, 227, 173, 227, 182, 158, 48, 69, 107, 220, 52, 90, 123, 172, 100, 82, 79, 37, 162, 212, 116, 90, 183, 29, 97, 31, 232, 77, 89, 211, 169, 129, 180, 4, 158, 163, 12, 186, 186, 43, 84, 248, 20, 28, 93, 87, 184, 132, 92, 165, 2, 205, 242, 200, 131, 174, 196, 170, 160, 53, 122, 151, 122, 114, 2, 29, 10, 42, 115, 10, 20, 4, 255, 135, 104, 78, 134, 220, 53, 241, 21, 98, 9, 17, 207, 68, 162, 72, 144, 34, 21, 142, 4, 171, 160, 101, 125, 104, 69, 206, 73, 100, 83, 186, 153, 17, 142, 130, 237, 209, 11, 250, 148, 222, 154, 113, 13, 179, 181, 203, 182, 245, 153, 168, 225, 184, 146, 161, 211, 72, 25, 238, 9, 5, 87, 27, 55, 232, 246, 250, 174, 174, 239, 217, 198, 220, 105, 26, 86, 224, 107, 60, 213, 52, 172, 161, 14, 219, 252, 80, 177, 93, 193, 214, 18, 126, 169, 216, 169, 160, 188, 49, 199, 216, 224, 71, 61, 118, 105, 184, 7, 181, 213, 168, 192, 30, 13, 85, 168, 150, 187, 251, 52, 248, 81, 35, 119, 1, 13, 26, 54, 201, 93, 80, 195, 102, 108, 145, 187, 3, 172, 76, 43, 213, 101, 143, 103, 146, 34, 229, 42, 232, 110, 204, 69, 208, 212, 83, 227, 173, 249, 16, 52, 109, 52, 29, 129, 245, 98, 166, 162, 145, 245, 168, 231, 173, 14, 205, 164, 153, 244, 218, 34, 195, 61, 166, 238, 56, 29, 26, 66, 104, 174, 70, 19, 194, 10, 54, 175, 28, 14, 91, 70, 66, 69, 11, 131, 52, 20, 59, 125, 82, 195, 253, 146, 169, 21, 71, 20, 220, 179, 26, 186, 138, 99, 204, 189, 201, 30, 117, 39, 188, 80, 157, 210, 240, 0, 30, 244, 115, 100, 60, 196, 127, 61, 145, 96, 201, 20, 123, 60, 48, 122, 65, 196, 221, 142, 166, 115, 26, 30, 70, 187, 12, 105, 135, 23, 32, 122, 144, 54, 133, 172, 205, 251, 27, 55, 24, 14, 13, 159, 192, 9, 63, 67, 253, 73, 5, 205, 119, 150, 44, 164, 224, 228, 165, 184, 200, 199, 168, 235, 54, 143, 114, 208, 84, 244, 40, 232, 236, 74, 5, 68, 50, 237, 206, 4, 10, 33, 12, 76, 235, 78, 32, 109, 91, 83, 70, 66, 36, 2, 99, 150, 29, 200, 119, 95, 196, 164, 238, 64, 174, 113, 3, 135, 15, 58, 135, 91, 170, 112, 146, 49, 33, 75, 82, 103, 190, 31, 46, 145, 239, 167, 74, 4, 100, 45, 151, 134, 199, 241, 132, 140, 228, 169, 85, 49, 44, 84, 141, 87, 150, 189, 126, 116, 35, 170, 224, 88, 207, 29, 240, 4, 18, 150, 112, 2, 41, 203, 13, 184, 250, 164, 8, 232, 169, 101, 159, 136, 180, 159, 137, 151, 157, 168, 219, 110, 191, 184, 228, 50, 70, 10, 84, 195, 57, 41, 125, 247, 242, 201, 60, 125, 10, 103, 100, 158, 6, 21, 60, 184, 110, 94, 250, 12, 199, 33, 182, 92, 17, 82, 91, 126, 56, 158, 229, 68, 190, 235, 185, 112, 123, 171, 74, 31, 71, 252, 24, 198, 167, 9, 117, 92, 184, 79, 232, 206, 74, 51, 69, 251, 173, 255, 35, 67, 129, 105, 195, 157, 8, 36, 132, 19, 183, 13, 143, 218, 238, 145, 171, 240, 148, 130, 157, 171, 195, 220, 157, 49, 204, 132, 176, 85, 124, 198, 143, 167, 177, 139, 99, 183, 72, 80, 65, 93, 169, 188, 61, 11, 93, 118, 213, 40, 203, 64, 79, 167, 121, 91, 42, 136, 148, 108, 251, 59, 24, 163, 138, 4, 132, 180, 55, 198, 193, 84, 162, 90, 84, 176, 219, 170, 92, 171, 112, 213, 108, 107, 44, 89, 63, 23, 48, 41, 149, 152, 52, 191, 161, 153, 172, 34, 37, 103, 26, 157, 176, 144, 94, 59, 109, 214, 204, 95, 21, 182, 130, 45, 185, 8, 59, 221, 51, 5, 84, 91, 139, 238, 10, 166, 109, 162, 79, 79, 19, 148, 139, 76, 53, 28, 76, 113, 32, 173, 57, 86, 113, 137, 85, 199, 44, 43, 56, 84, 106, 106, 172, 37, 105, 120, 14, 207, 251, 49, 131, 207, 82, 100, 61, 156, 133, 217, 120, 249, 54, 219, 57, 77, 142, 138, 207, 177, 194, 108, 113, 49, 99, 216, 162, 223, 74, 245, 243, 25, 162, 225, 11, 82, 251, 231, 241, 69, 30, 165, 109, 225, 176, 206, 114, 74, 156, 181, 238, 13, 10, 9, 231, 203, 248, 138, 116, 239, 171, 236, 163, 124, 64, 52, 124, 93, 210, 154, 240, 162, 130, 77, 43, 34, 100, 87, 241, 178, 44, 145, 68, 162, 203, 100, 142, 26, 26, 139, 20, 246, 88, 166, 73, 84, 242, 250, 147, 61, 248, 77, 124, 171, 6, 223, 192, 183, 57, 147, 29, 227, 57, 161, 225, 21, 57, 100, 235, 241, 221, 106, 92, 193, 158, 194, 244, 246, 36, 115, 23, 215, 15, 20, 156, 232, 203, 152, 174, 193, 169, 186, 220, 34, 78, 96, 90, 216, 226, 174, 199, 216, 143, 88, 99, 134, 43, 108, 221, 181, 152, 223, 29, 133, 26, 243, 172, 156, 202, 211, 233, 241, 171, 248, 177, 132, 242, 26, 155, 97, 237, 185, 138, 159, 50, 18, 124, 8, 202, 241, 162, 225, 103, 56, 83, 131, 215, 241, 115, 250, 145, 34, 97, 117, 229, 46, 39, 245, 151, 184, 42, 249, 222, 80, 80, 61, 110, 91, 153, 244, 8, 59, 86, 195, 181, 92, 28, 127, 181, 166, 116, 188, 88, 254, 218, 143, 183, 37, 138, 42, 214, 143, 119, 191, 49, 69, 183, 105, 47, 92, 122, 191, 193, 111, 253, 120, 20, 191, 99, 236, 109, 145, 180, 166, 24, 206, 119, 228, 93, 244, 54, 222, 245, 42, 64, 146, 18, 3, 133, 228, 101, 115, 9, 157, 147, 231, 188, 171, 124, 61, 124, 165, 50, 145, 156, 79, 241, 73, 150, 236, 144, 62, 42, 31, 156, 91, 100, 125, 245, 103, 146, 163, 194, 246, 40, 168, 229, 133, 164, 242, 237, 92, 193, 29, 31, 10, 220, 213, 202, 103, 130, 183, 242, 145, 224, 173, 124, 34, 120, 43, 159, 20, 30, 39, 35, 136, 173, 252, 222, 228, 223, 81, 74, 87, 112, 61, 20, 138, 157, 63, 95, 190, 128, 29, 139, 168, 143, 245, 46, 96, 119, 104, 14, 247, 54, 207, 97, 111, 120, 14, 251, 34, 115, 216, 223, 224, 155, 195, 193, 119, 0, 79, 211, 33, 28, 206, 203, 191, 72, 233, 50, 174, 231, 66, 89, 236, 203, 34, 242, 46, 142, 190, 129, 227, 205, 243, 104, 155, 69, 109, 40, 150, 165, 146, 121, 116, 142, 204, 227, 145, 27, 139, 232, 142, 145, 107, 111, 175, 239, 15, 120, 52, 22, 45, 15, 13, 214, 61, 214, 124, 19, 79, 46, 160, 239, 79, 37, 206, 6, 10, 103, 212, 94, 134, 91, 252, 86, 162, 236, 223, 8, 238, 198, 123, 220, 215, 193, 71, 4, 140, 19, 207, 84, 148, 211, 71, 31, 113, 157, 198, 137, 60, 174, 97, 210, 20, 174, 7, 139, 113, 237, 224, 207, 189, 243, 24, 154, 133, 182, 136, 225, 88, 104, 1, 177, 27, 33, 9, 109, 217, 4, 133, 2, 158, 254, 29, 244, 75, 90, 80, 121, 176, 153, 228, 61, 252, 219, 231, 69, 249, 28, 206, 228, 109, 116, 242, 76, 218, 216, 90, 108, 67, 245, 93, 131, 175, 252, 250, 178, 70, 85, 130, 174, 125, 68, 241, 180, 250, 73, 44, 104, 173, 149, 239, 137, 156, 166, 178, 22, 82, 42, 1, 37, 86, 172, 105, 104, 25, 237, 147, 33, 198, 39, 90, 119, 190, 175, 66, 70, 167, 95, 6, 170, 252, 153, 65, 95, 7, 153, 23, 241, 116, 172, 221, 183, 128, 103, 178, 136, 199, 218, 43, 254, 140, 154, 6, 95, 67, 197, 60, 198, 71, 154, 99, 225, 72, 108, 87, 131, 111, 30, 201, 193, 124, 36, 67, 49, 38, 100, 127, 148, 226, 89, 92, 236, 163, 116, 127, 115, 22, 211, 225, 121, 188, 240, 62, 174, 220, 196, 151, 248, 141, 245, 133, 72, 251, 90, 36, 139, 151, 110, 225, 59, 101, 24, 241, 48, 124, 175, 51, 139, 239, 115, 217, 253, 172, 239, 42, 90, 242, 32, 235, 126, 248, 38, 42, 175, 97, 111, 137, 220, 205, 22, 114, 151, 115, 225, 39, 209, 230, 91, 248, 133, 130, 89, 52, 132, 185, 123, 83, 193, 251, 56, 222, 79, 149, 17, 89, 27, 215, 150, 254, 145, 211, 248, 214, 60, 174, 47, 115, 70, 67, 5, 206, 38, 34, 221, 223, 31, 38, 239, 13, 162, 121, 137, 76, 125, 225, 28, 211, 210, 95, 35, 225, 130, 186, 118, 31, 97, 75, 111, 127, 63, 187, 244, 119, 194, 159, 151, 251, 5, 42, 255, 27, 149, 31, 92, 201, 240, 0, 170, 151, 208, 139, 10, 165, 10, 245, 42, 142, 171, 48, 84, 222, 93, 88, 98, 69, 41, 94, 43, 124, 136, 179, 31, 97, 55, 19, 114, 101, 73, 86, 22, 105, 42, 94, 231, 31, 240, 47, 220, 247, 31, 248, 249, 247, 17, 14, 120, 52, 231, 3, 132, 189, 188, 166, 40, 176, 186, 26, 143, 80, 89, 59, 179, 250, 56, 41, 195, 236, 205, 9, 118, 167, 201, 10, 184, 200, 238, 204, 64, 78, 235, 77, 120, 153, 156, 175, 176, 59, 103, 217, 155, 175, 177, 34, 222, 34, 138, 235, 236, 205, 44, 182, 225, 159, 216, 142, 15, 88, 135, 31, 98, 39, 62, 70, 61, 235, 178, 65, 217, 207, 151, 193, 89, 207, 86, 185, 231, 80, 249, 127, 1, 80, 75, 7, 8, 64, 122, 198, 191, 94, 7, 0, 0, 58, 15, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 77, 105, 115, 115, 105, 110, 103, 79, 112, 116, 105, 111, 110, 65, 114, 103, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 157, 147, 109, 79, 211, 80, 20, 199, 255, 151, 193, 186, 149, 34, 48, 69, 196, 39, 180, 130, 118, 15, 101, 10, 168, 11, 24, 141, 18, 77, 76, 16, 140, 51, 24, 121, 119, 215, 222, 117, 149, 246, 118, 105, 59, 18, 99, 228, 131, 248, 25, 124, 161, 137, 206, 196, 23, 126, 0, 63, 148, 241, 180, 116, 6, 13, 9, 66, 155, 244, 220, 158, 254, 207, 239, 156, 158, 123, 238, 207, 95, 223, 127, 0, 88, 134, 201, 240, 97, 111, 239, 69, 227, 157, 222, 226, 214, 142, 144, 182, 190, 162, 91, 109, 189, 166, 91, 129, 223, 117, 61, 30, 187, 129, 52, 253, 192, 22, 228, 15, 133, 39, 120, 36, 232, 99, 135, 71, 166, 213, 17, 214, 78, 212, 243, 35, 125, 165, 205, 189, 72, 212, 244, 174, 99, 250, 188, 107, 186, 9, 67, 136, 197, 219, 75, 188, 65, 218, 176, 49, 136, 111, 247, 60, 143, 28, 81, 135, 155, 183, 18, 137, 116, 92, 41, 68, 232, 74, 135, 188, 187, 34, 140, 40, 23, 249, 27, 11, 75, 11, 13, 211, 22, 187, 250, 251, 2, 24, 131, 218, 12, 122, 161, 37, 158, 184, 158, 96, 184, 19, 132, 78, 221, 9, 185, 237, 137, 186, 229, 185, 245, 181, 192, 247, 185, 180, 215, 137, 244, 156, 135, 145, 8, 231, 158, 185, 81, 68, 200, 205, 110, 82, 250, 195, 208, 105, 198, 60, 22, 10, 134, 25, 106, 71, 198, 238, 155, 44, 34, 207, 144, 15, 82, 10, 195, 242, 250, 145, 177, 251, 9, 15, 16, 86, 41, 254, 158, 43, 221, 248, 62, 195, 93, 227, 36, 128, 242, 22, 195, 176, 241, 180, 188, 165, 65, 133, 166, 66, 193, 152, 134, 2, 138, 69, 140, 96, 156, 97, 194, 231, 111, 91, 130, 164, 97, 188, 153, 213, 57, 101, 172, 191, 225, 187, 188, 238, 113, 233, 212, 155, 113, 210, 219, 213, 242, 54, 131, 98, 60, 136, 202, 230, 66, 181, 128, 211, 20, 247, 175, 68, 193, 20, 73, 124, 30, 211, 158, 70, 26, 166, 81, 82, 113, 22, 231, 24, 198, 2, 249, 23, 126, 251, 16, 252, 33, 9, 79, 214, 172, 197, 227, 71, 41, 184, 72, 243, 17, 72, 146, 118, 61, 17, 211, 124, 220, 52, 254, 35, 251, 193, 188, 26, 46, 99, 86, 197, 37, 92, 209, 112, 30, 23, 146, 38, 235, 12, 163, 129, 220, 8, 228, 224, 183, 31, 29, 214, 213, 227, 165, 73, 203, 164, 105, 236, 249, 66, 198, 26, 174, 99, 62, 201, 121, 131, 54, 35, 173, 126, 16, 249, 88, 218, 12, 57, 35, 221, 248, 53, 58, 53, 12, 227, 137, 123, 163, 231, 183, 68, 248, 146, 183, 60, 129, 73, 218, 124, 133, 206, 46, 163, 21, 205, 2, 173, 70, 104, 173, 98, 148, 158, 85, 122, 155, 70, 14, 67, 100, 71, 43, 175, 115, 223, 112, 170, 250, 21, 19, 159, 145, 92, 147, 116, 151, 50, 209, 44, 73, 18, 145, 82, 45, 157, 233, 99, 230, 99, 202, 171, 209, 51, 79, 150, 165, 108, 106, 70, 38, 158, 33, 98, 142, 236, 88, 229, 11, 38, 250, 184, 90, 173, 245, 113, 237, 83, 198, 156, 195, 124, 38, 155, 202, 152, 197, 68, 86, 237, 195, 24, 72, 202, 168, 252, 145, 36, 236, 76, 66, 164, 87, 251, 149, 177, 20, 63, 132, 220, 111, 80, 75, 7, 8, 106, 33, 173, 75, 75, 2, 0, 0, 151, 4, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 79, 112, 116, 105, 111, 110, 65, 119, 97, 114, 101, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 133, 83, 235, 78, 19, 65, 20, 254, 134, 2, 75, 75, 17, 202, 77, 64, 84, 92, 65, 91, 218, 165, 92, 165, 92, 132, 84, 130, 134, 4, 75, 3, 68, 130, 127, 200, 116, 59, 108, 23, 118, 183, 100, 118, 139, 16, 35, 15, 226, 51, 248, 67, 13, 151, 68, 19, 31, 192, 135, 50, 158, 109, 65, 203, 37, 233, 110, 178, 51, 251, 157, 239, 124, 231, 50, 103, 126, 255, 249, 241, 11, 192, 36, 22, 25, 62, 159, 156, 172, 167, 62, 170, 57, 174, 239, 11, 39, 175, 206, 170, 250, 174, 154, 80, 245, 162, 125, 96, 90, 220, 51, 139, 142, 102, 23, 243, 130, 112, 41, 44, 193, 93, 65, 198, 2, 119, 53, 189, 32, 244, 125, 183, 100, 187, 234, 236, 46, 183, 92, 145, 80, 15, 12, 205, 230, 7, 154, 233, 107, 8, 49, 62, 53, 193, 83, 196, 149, 169, 43, 255, 221, 146, 101, 17, 224, 22, 184, 54, 230, 83, 28, 195, 116, 132, 144, 166, 99, 16, 122, 40, 164, 75, 177, 8, 79, 141, 76, 140, 164, 180, 188, 56, 84, 63, 53, 129, 49, 132, 54, 138, 37, 169, 139, 215, 166, 37, 24, 166, 139, 210, 72, 26, 146, 231, 45, 145, 212, 45, 51, 185, 84, 180, 109, 238, 228, 87, 73, 41, 203, 165, 43, 228, 224, 218, 129, 159, 115, 250, 3, 151, 151, 200, 134, 199, 61, 161, 160, 158, 33, 81, 211, 249, 154, 71, 35, 67, 179, 254, 159, 194, 160, 174, 222, 16, 40, 211, 243, 85, 50, 115, 12, 141, 94, 193, 116, 7, 71, 239, 96, 223, 10, 231, 179, 231, 77, 199, 244, 22, 24, 222, 68, 107, 211, 107, 135, 143, 189, 11, 35, 132, 230, 32, 26, 112, 143, 161, 62, 186, 226, 3, 97, 180, 133, 160, 32, 18, 70, 19, 130, 190, 169, 131, 161, 205, 230, 199, 57, 65, 133, 74, 175, 210, 48, 134, 174, 232, 234, 30, 63, 228, 73, 139, 59, 70, 114, 195, 243, 207, 101, 46, 246, 158, 65, 137, 46, 186, 49, 109, 36, 222, 132, 251, 228, 119, 147, 162, 160, 151, 40, 54, 247, 104, 30, 220, 48, 30, 160, 59, 132, 62, 244, 83, 235, 138, 78, 166, 232, 92, 137, 191, 186, 75, 188, 102, 197, 213, 231, 65, 205, 26, 168, 85, 191, 130, 1, 134, 123, 226, 200, 147, 60, 45, 141, 146, 45, 28, 207, 165, 194, 42, 161, 75, 158, 105, 37, 211, 82, 242, 227, 85, 211, 245, 230, 194, 80, 241, 52, 136, 39, 24, 100, 232, 184, 131, 160, 224, 25, 67, 128, 231, 243, 55, 58, 179, 150, 219, 19, 186, 71, 157, 9, 35, 138, 88, 8, 207, 49, 124, 59, 179, 91, 149, 40, 72, 80, 152, 181, 236, 230, 202, 90, 102, 39, 147, 126, 187, 188, 147, 77, 111, 110, 46, 175, 103, 24, 122, 171, 210, 147, 194, 16, 71, 84, 151, 231, 9, 233, 80, 138, 35, 72, 6, 161, 97, 244, 90, 227, 43, 25, 40, 24, 103, 104, 50, 132, 183, 100, 113, 151, 170, 236, 136, 198, 170, 178, 44, 131, 36, 48, 137, 169, 16, 38, 240, 130, 65, 171, 217, 236, 244, 46, 69, 173, 28, 152, 171, 32, 197, 48, 116, 107, 38, 239, 158, 184, 48, 102, 67, 152, 1, 157, 80, 253, 18, 93, 117, 134, 86, 223, 148, 41, 217, 57, 33, 55, 121, 206, 18, 116, 247, 26, 104, 2, 129, 58, 68, 252, 33, 4, 218, 34, 254, 156, 18, 18, 0, 35, 255, 22, 250, 190, 164, 191, 126, 212, 19, 66, 230, 225, 237, 237, 248, 25, 90, 3, 23, 104, 79, 156, 161, 243, 27, 252, 39, 130, 46, 116, 95, 50, 31, 147, 86, 29, 173, 74, 188, 189, 231, 28, 15, 191, 208, 150, 97, 129, 190, 141, 180, 250, 111, 4, 143, 136, 84, 33, 103, 73, 212, 39, 15, 14, 111, 159, 162, 243, 20, 67, 241, 115, 196, 183, 78, 209, 250, 29, 99, 91, 231, 152, 222, 250, 137, 153, 237, 97, 50, 93, 96, 254, 235, 63, 165, 62, 210, 106, 160, 125, 144, 124, 91, 72, 161, 147, 146, 235, 33, 100, 160, 28, 35, 80, 46, 39, 240, 23, 80, 75, 7, 8, 19, 65, 22, 149, 213, 2, 0, 0, 74, 5, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 79, 112, 116, 105, 111, 110, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 149, 80, 77, 111, 19, 49, 16, 29, 55, 95, 37, 109, 211, 242, 81, 56, 245, 178, 226, 144, 160, 110, 22, 18, 144, 86, 45, 66, 130, 74, 136, 74, 81, 91, 17, 212, 3, 183, 201, 238, 100, 227, 214, 235, 93, 217, 222, 168, 82, 69, 127, 8, 255, 130, 83, 165, 30, 248, 1, 252, 168, 138, 113, 8, 106, 111, 8, 31, 252, 198, 239, 205, 60, 207, 204, 175, 219, 155, 159, 0, 240, 26, 158, 9, 248, 126, 117, 245, 57, 190, 12, 38, 152, 156, 147, 78, 131, 189, 32, 153, 6, 187, 65, 82, 228, 165, 84, 232, 100, 161, 195, 188, 72, 137, 121, 67, 138, 208, 18, 139, 51, 180, 97, 50, 163, 228, 220, 86, 185, 13, 246, 166, 168, 44, 237, 6, 101, 22, 230, 88, 134, 210, 123, 16, 13, 222, 12, 49, 230, 92, 19, 255, 173, 159, 86, 74, 49, 97, 103, 24, 190, 242, 41, 58, 147, 154, 200, 72, 157, 49, 59, 39, 99, 249, 47, 230, 227, 254, 176, 31, 135, 41, 205, 131, 111, 171, 32, 4, 180, 199, 69, 101, 18, 250, 40, 21, 9, 24, 20, 38, 139, 50, 131, 169, 162, 40, 81, 50, 58, 40, 242, 28, 117, 58, 98, 167, 19, 52, 150, 204, 243, 227, 210, 247, 252, 231, 49, 118, 232, 168, 5, 117, 1, 91, 103, 56, 199, 72, 161, 206, 162, 227, 201, 25, 37, 174, 5, 77, 1, 205, 183, 82, 75, 247, 78, 64, 173, 219, 59, 93, 135, 85, 120, 208, 134, 22, 180, 5, 212, 187, 135, 189, 211, 54, 52, 124, 220, 41, 52, 251, 24, 119, 68, 23, 238, 189, 201, 4, 188, 236, 246, 70, 255, 108, 227, 94, 3, 251, 60, 67, 161, 185, 180, 202, 73, 59, 1, 31, 186, 163, 187, 110, 198, 206, 47, 96, 255, 191, 29, 59, 25, 185, 79, 104, 239, 92, 121, 132, 175, 139, 143, 184, 178, 84, 228, 120, 89, 245, 3, 94, 188, 128, 77, 111, 114, 84, 229, 19, 50, 95, 112, 162, 136, 215, 209, 224, 49, 253, 105, 130, 240, 83, 243, 189, 205, 175, 135, 140, 130, 177, 241, 226, 26, 214, 126, 120, 125, 203, 203, 235, 75, 121, 135, 113, 101, 41, 111, 120, 89, 192, 211, 165, 7, 199, 236, 218, 129, 77, 88, 44, 155, 157, 60, 62, 130, 199, 11, 124, 226, 121, 206, 170, 241, 189, 2, 181, 223, 80, 75, 7, 8, 187, 211, 122, 19, 161, 1, 0, 0, 125, 2, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 79, 112, 116, 105, 111, 110, 83, 116, 114, 105, 110, 103, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 117, 82, 93, 111, 18, 65, 20, 61, 83, 144, 165, 184, 72, 129, 86, 180, 85, 171, 219, 106, 129, 178, 16, 173, 38, 164, 53, 62, 216, 196, 167, 106, 141, 152, 26, 124, 49, 195, 50, 44, 219, 238, 87, 102, 23, 94, 140, 253, 31, 250, 7, 124, 213, 164, 165, 137, 38, 254, 0, 127, 148, 122, 119, 161, 241, 3, 204, 36, 147, 51, 103, 238, 61, 247, 206, 153, 251, 253, 199, 151, 111, 0, 238, 163, 206, 240, 225, 248, 248, 69, 243, 173, 214, 225, 198, 145, 112, 187, 218, 182, 102, 244, 180, 154, 102, 120, 142, 111, 217, 60, 180, 60, 87, 119, 188, 174, 32, 94, 10, 91, 240, 64, 208, 101, 159, 7, 186, 209, 23, 198, 81, 48, 112, 2, 109, 187, 199, 237, 64, 212, 52, 223, 212, 29, 238, 235, 86, 164, 33, 196, 189, 7, 91, 188, 73, 177, 178, 121, 158, 223, 27, 216, 54, 17, 65, 159, 235, 119, 163, 16, 215, 180, 92, 33, 164, 229, 154, 196, 14, 133, 12, 168, 22, 241, 205, 250, 86, 189, 169, 119, 197, 80, 123, 151, 6, 99, 200, 180, 188, 129, 52, 196, 19, 203, 22, 12, 186, 39, 205, 134, 41, 121, 215, 22, 13, 195, 182, 26, 187, 158, 227, 112, 183, 187, 71, 74, 207, 185, 12, 132, 92, 223, 247, 163, 158, 91, 97, 164, 171, 32, 201, 176, 112, 200, 135, 188, 97, 115, 215, 108, 236, 119, 14, 133, 17, 42, 72, 49, 36, 184, 52, 25, 10, 123, 191, 47, 199, 41, 59, 12, 41, 47, 150, 32, 240, 208, 114, 173, 240, 17, 195, 70, 121, 58, 110, 154, 169, 28, 144, 108, 185, 114, 160, 226, 34, 178, 25, 40, 184, 164, 34, 141, 249, 121, 92, 192, 130, 138, 204, 24, 21, 24, 210, 161, 55, 206, 96, 88, 44, 87, 102, 117, 48, 167, 235, 105, 92, 254, 171, 245, 243, 7, 93, 33, 67, 130, 144, 203, 48, 120, 101, 133, 125, 134, 165, 25, 173, 85, 94, 171, 88, 198, 74, 6, 87, 113, 141, 161, 244, 239, 253, 227, 129, 101, 119, 133, 84, 112, 227, 63, 233, 241, 11, 110, 102, 176, 138, 91, 100, 2, 247, 125, 154, 11, 178, 126, 86, 232, 20, 53, 17, 223, 81, 177, 134, 245, 72, 226, 182, 138, 69, 44, 69, 104, 131, 129, 209, 187, 42, 12, 201, 93, 154, 8, 134, 108, 43, 164, 161, 123, 202, 253, 151, 188, 19, 125, 110, 46, 250, 198, 103, 3, 167, 35, 100, 204, 32, 79, 134, 41, 52, 167, 115, 132, 200, 73, 66, 249, 200, 199, 152, 97, 212, 163, 74, 251, 38, 157, 86, 144, 160, 5, 228, 170, 237, 246, 25, 114, 155, 167, 200, 215, 78, 81, 252, 12, 196, 41, 84, 127, 18, 104, 32, 73, 8, 104, 86, 79, 144, 47, 148, 70, 184, 254, 30, 203, 95, 177, 218, 174, 190, 41, 148, 206, 160, 157, 160, 56, 194, 157, 17, 202, 31, 81, 154, 208, 213, 63, 233, 79, 113, 233, 26, 237, 89, 204, 173, 253, 68, 145, 16, 83, 200, 103, 157, 64, 138, 164, 199, 43, 17, 135, 37, 126, 1, 80, 75, 7, 8, 1, 186, 27, 26, 33, 2, 0, 0, 102, 3, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 133, 145, 207, 78, 27, 49, 16, 198, 199, 132, 36, 52, 144, 18, 160, 192, 137, 203, 170, 135, 164, 202, 178, 226, 159, 180, 130, 170, 7, 16, 149, 42, 69, 128, 72, 197, 129, 219, 236, 238, 100, 99, 240, 218, 145, 119, 19, 9, 85, 229, 65, 120, 139, 158, 42, 245, 192, 3, 240, 80, 21, 227, 16, 90, 144, 34, 177, 210, 122, 198, 243, 125, 254, 217, 30, 63, 252, 253, 115, 15, 0, 187, 176, 46, 224, 238, 246, 246, 60, 252, 225, 69, 24, 95, 147, 78, 188, 125, 47, 238, 121, 109, 47, 54, 217, 64, 42, 44, 164, 209, 126, 102, 18, 226, 186, 37, 69, 152, 19, 139, 125, 204, 253, 184, 79, 241, 117, 62, 204, 114, 111, 191, 135, 42, 167, 182, 55, 72, 253, 12, 7, 190, 116, 12, 162, 237, 189, 29, 12, 217, 107, 195, 231, 245, 189, 161, 82, 92, 200, 251, 232, 111, 57, 139, 78, 165, 38, 178, 82, 167, 92, 29, 145, 205, 121, 47, 174, 135, 155, 59, 155, 161, 159, 208, 200, 251, 57, 7, 66, 64, 173, 107, 134, 54, 166, 175, 82, 145, 128, 182, 177, 105, 144, 90, 76, 20, 5, 177, 146, 193, 145, 201, 50, 212, 73, 135, 73, 103, 104, 115, 178, 31, 159, 66, 183, 192, 130, 170, 48, 43, 160, 113, 133, 35, 12, 20, 234, 52, 56, 141, 174, 40, 46, 170, 80, 17, 80, 249, 44, 181, 44, 190, 8, 40, 53, 91, 23, 11, 48, 7, 239, 106, 80, 133, 154, 128, 217, 230, 183, 214, 69, 13, 202, 46, 111, 100, 120, 19, 17, 163, 108, 113, 58, 112, 157, 16, 176, 218, 236, 252, 231, 117, 11, 119, 248, 131, 214, 165, 128, 186, 209, 175, 124, 151, 83, 124, 83, 86, 118, 222, 188, 206, 19, 240, 197, 165, 14, 4, 204, 27, 125, 98, 244, 243, 86, 135, 211, 142, 244, 54, 248, 53, 178, 97, 244, 11, 203, 177, 78, 184, 19, 71, 252, 108, 2, 22, 93, 225, 100, 152, 69, 100, 191, 99, 164, 136, 91, 90, 230, 86, 185, 175, 2, 194, 117, 142, 199, 85, 158, 45, 113, 20, 28, 203, 159, 126, 195, 252, 47, 167, 55, 156, 188, 48, 145, 55, 56, 206, 76, 228, 186, 147, 5, 172, 77, 24, 156, 51, 245, 61, 44, 194, 248, 193, 152, 228, 226, 50, 172, 140, 93, 31, 254, 237, 80, 31, 207, 249, 31, 211, 57, 45, 241, 56, 3, 165, 71, 80, 75, 7, 8, 214, 254, 75, 70, 171, 1, 0, 0, 206, 2, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 85, 110, 107, 110, 111, 119, 110, 79, 112, 116, 105, 111, 110, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 149, 83, 237, 78, 19, 81, 16, 61, 151, 182, 20, 202, 10, 150, 111, 21, 20, 87, 212, 182, 116, 187, 8, 26, 43, 24, 19, 36, 49, 26, 27, 48, 162, 24, 136, 137, 185, 221, 189, 44, 11, 251, 209, 220, 221, 162, 198, 200, 131, 248, 12, 254, 208, 164, 96, 226, 15, 31, 192, 135, 50, 206, 45, 69, 26, 36, 105, 248, 179, 51, 119, 102, 206, 153, 51, 115, 247, 254, 254, 243, 243, 23, 128, 187, 152, 101, 248, 178, 191, 255, 178, 252, 73, 175, 114, 107, 87, 4, 182, 190, 160, 91, 91, 122, 81, 183, 66, 191, 230, 122, 60, 118, 195, 192, 240, 67, 91, 80, 92, 10, 79, 240, 72, 80, 114, 155, 71, 134, 181, 45, 172, 221, 168, 238, 71, 250, 194, 22, 247, 34, 81, 212, 107, 142, 225, 243, 154, 225, 42, 14, 33, 230, 238, 205, 243, 50, 213, 202, 242, 49, 126, 171, 238, 121, 20, 136, 182, 185, 113, 71, 149, 4, 142, 27, 8, 33, 221, 192, 161, 232, 158, 144, 17, 245, 162, 120, 185, 52, 95, 42, 27, 182, 216, 211, 63, 247, 128, 49, 100, 214, 194, 186, 180, 196, 19, 215, 19, 12, 15, 66, 233, 152, 142, 228, 182, 39, 76, 203, 115, 205, 229, 208, 247, 121, 96, 87, 136, 233, 5, 151, 145, 144, 211, 175, 131, 221, 32, 124, 31, 172, 214, 148, 244, 163, 216, 90, 204, 99, 145, 70, 146, 97, 174, 35, 252, 12, 92, 55, 67, 42, 82, 46, 67, 169, 210, 145, 160, 13, 186, 200, 144, 224, 210, 97, 24, 172, 236, 240, 61, 110, 122, 60, 112, 204, 181, 88, 77, 76, 169, 62, 235, 4, 203, 160, 159, 102, 110, 242, 216, 109, 252, 4, 233, 126, 232, 6, 110, 252, 136, 65, 228, 254, 103, 236, 76, 112, 62, 241, 249, 117, 134, 100, 238, 89, 126, 93, 67, 63, 46, 102, 144, 70, 86, 67, 6, 125, 189, 72, 97, 72, 131, 134, 11, 202, 27, 209, 208, 131, 94, 229, 141, 49, 244, 59, 34, 126, 202, 163, 37, 233, 212, 125, 17, 196, 52, 126, 46, 191, 73, 225, 48, 32, 74, 25, 175, 136, 15, 241, 146, 218, 199, 108, 46, 127, 222, 69, 102, 194, 128, 74, 106, 158, 136, 133, 134, 73, 76, 100, 168, 227, 213, 102, 248, 164, 219, 227, 51, 182, 114, 238, 70, 83, 157, 214, 152, 134, 78, 35, 209, 40, 146, 31, 183, 142, 24, 70, 142, 90, 215, 99, 215, 51, 151, 164, 228, 31, 43, 110, 20, 47, 106, 152, 198, 205, 94, 220, 192, 45, 134, 161, 51, 10, 210, 200, 169, 127, 196, 182, 137, 160, 93, 252, 106, 117, 71, 88, 241, 98, 126, 83, 67, 1, 51, 25, 228, 81, 164, 203, 88, 166, 119, 196, 48, 160, 68, 172, 212, 253, 170, 144, 175, 120, 213, 19, 200, 210, 42, 210, 244, 154, 19, 228, 209, 93, 144, 151, 85, 247, 212, 180, 116, 75, 100, 83, 32, 197, 24, 160, 111, 137, 78, 83, 116, 78, 146, 29, 46, 108, 188, 77, 252, 192, 224, 204, 1, 134, 139, 7, 24, 53, 14, 48, 254, 29, 104, 226, 46, 225, 114, 171, 186, 159, 44, 35, 219, 149, 252, 218, 202, 93, 193, 68, 43, 151, 109, 229, 82, 133, 67, 92, 251, 214, 74, 79, 225, 122, 91, 186, 235, 116, 122, 242, 31, 250, 62, 41, 86, 232, 177, 194, 198, 70, 3, 163, 207, 27, 164, 168, 129, 219, 239, 14, 97, 188, 105, 96, 92, 1, 24, 204, 166, 132, 4, 141, 67, 147, 19, 219, 80, 19, 148, 80, 130, 144, 248, 11, 80, 75, 7, 8, 102, 233, 189, 109, 114, 2, 0, 0, 199, 4, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 85, 93, 87, 19, 71, 24, 126, 198, 4, 55, 196, 88, 37, 168, 52, 22, 101, 155, 74, 9, 129, 144, 42, 90, 35, 248, 21, 35, 86, 10, 36, 72, 80, 27, 209, 210, 97, 119, 72, 86, 54, 187, 113, 119, 195, 71, 173, 94, 244, 244, 186, 231, 120, 169, 151, 189, 241, 182, 173, 22, 108, 61, 181, 189, 238, 77, 127, 68, 127, 71, 75, 223, 221, 229, 171, 16, 60, 205, 57, 217, 157, 121, 231, 153, 103, 222, 121, 222, 143, 253, 227, 159, 95, 222, 0, 56, 141, 69, 134, 103, 143, 31, 79, 100, 30, 198, 103, 184, 50, 39, 12, 53, 62, 16, 87, 102, 227, 189, 113, 197, 172, 214, 52, 157, 59, 154, 105, 164, 170, 166, 42, 200, 110, 9, 93, 112, 91, 208, 98, 133, 219, 41, 165, 34, 148, 57, 187, 94, 181, 227, 3, 179, 92, 183, 69, 111, 188, 86, 78, 85, 121, 45, 165, 185, 28, 66, 156, 58, 211, 207, 51, 132, 181, 50, 235, 251, 103, 235, 186, 78, 6, 187, 194, 83, 39, 93, 136, 81, 214, 12, 33, 44, 205, 40, 147, 117, 94, 88, 54, 157, 69, 246, 76, 95, 127, 95, 38, 165, 138, 249, 248, 163, 16, 24, 67, 184, 104, 214, 45, 69, 92, 211, 116, 193, 32, 155, 86, 57, 93, 182, 184, 170, 139, 180, 162, 107, 233, 156, 89, 173, 114, 67, 29, 37, 166, 113, 110, 217, 194, 146, 16, 100, 56, 120, 159, 207, 243, 180, 206, 141, 114, 186, 48, 115, 95, 40, 142, 132, 189, 12, 173, 133, 241, 201, 225, 66, 126, 58, 159, 29, 27, 154, 30, 207, 78, 78, 14, 77, 228, 25, 98, 163, 30, 184, 238, 104, 122, 218, 18, 101, 177, 152, 30, 231, 142, 35, 44, 99, 144, 118, 156, 224, 54, 113, 186, 34, 216, 87, 53, 155, 207, 232, 66, 101, 96, 119, 24, 14, 152, 53, 207, 122, 101, 169, 232, 184, 55, 32, 236, 22, 158, 235, 220, 174, 140, 241, 154, 203, 192, 117, 221, 92, 184, 105, 204, 25, 230, 130, 81, 240, 247, 48, 236, 61, 175, 25, 154, 115, 145, 33, 144, 232, 190, 21, 193, 1, 28, 12, 67, 66, 11, 67, 203, 14, 14, 9, 173, 97, 28, 66, 75, 4, 17, 236, 111, 70, 19, 142, 48, 132, 206, 211, 213, 125, 130, 3, 155, 55, 205, 233, 228, 172, 132, 24, 195, 17, 85, 216, 154, 37, 212, 236, 186, 243, 69, 135, 59, 117, 219, 59, 238, 78, 4, 239, 161, 61, 140, 163, 56, 22, 65, 24, 251, 92, 202, 14, 134, 246, 196, 221, 75, 95, 221, 173, 61, 204, 234, 70, 189, 250, 104, 106, 99, 148, 154, 190, 151, 236, 14, 225, 125, 134, 182, 93, 100, 146, 240, 1, 131, 228, 167, 11, 5, 40, 149, 24, 221, 116, 201, 215, 102, 176, 123, 87, 137, 35, 232, 196, 135, 97, 156, 64, 87, 4, 33, 52, 187, 206, 116, 147, 60, 190, 184, 12, 103, 19, 83, 141, 216, 118, 207, 1, 95, 97, 210, 125, 159, 98, 26, 14, 215, 12, 123, 68, 44, 49, 28, 222, 234, 148, 159, 17, 131, 174, 18, 41, 244, 185, 226, 166, 41, 166, 169, 16, 78, 254, 39, 113, 252, 211, 36, 244, 83, 6, 218, 14, 183, 28, 251, 182, 230, 84, 182, 113, 173, 187, 68, 92, 103, 240, 113, 152, 10, 234, 44, 137, 81, 229, 14, 85, 135, 197, 208, 191, 21, 155, 171, 112, 171, 40, 30, 212, 133, 161, 136, 6, 146, 140, 249, 155, 72, 146, 115, 24, 112, 37, 25, 108, 160, 249, 26, 72, 194, 133, 141, 99, 236, 8, 46, 185, 1, 189, 136, 203, 12, 241, 205, 227, 134, 117, 93, 148, 185, 158, 181, 202, 245, 170, 48, 156, 161, 69, 69, 120, 226, 72, 184, 194, 48, 149, 227, 134, 97, 58, 50, 87, 85, 217, 23, 91, 238, 234, 180, 187, 100, 110, 203, 220, 88, 183, 40, 238, 208, 208, 151, 228, 53, 45, 101, 174, 215, 42, 156, 178, 130, 106, 86, 145, 21, 186, 14, 87, 40, 138, 54, 213, 164, 220, 149, 234, 242, 94, 211, 93, 125, 33, 92, 165, 16, 206, 154, 22, 249, 199, 112, 174, 129, 92, 83, 13, 162, 177, 19, 21, 193, 53, 124, 226, 74, 122, 125, 23, 209, 189, 202, 249, 52, 140, 28, 70, 24, 6, 254, 231, 141, 92, 140, 23, 78, 121, 129, 194, 233, 250, 77, 14, 143, 49, 116, 20, 182, 108, 210, 104, 147, 110, 9, 174, 46, 201, 170, 152, 165, 188, 82, 9, 84, 160, 106, 110, 144, 142, 18, 110, 188, 181, 45, 21, 214, 84, 47, 174, 103, 151, 23, 206, 172, 101, 241, 37, 170, 214, 155, 36, 21, 183, 71, 53, 155, 164, 234, 76, 236, 46, 140, 183, 201, 133, 145, 44, 183, 241, 89, 24, 183, 80, 34, 127, 18, 219, 87, 125, 81, 166, 194, 152, 196, 93, 74, 17, 115, 189, 227, 108, 239, 78, 69, 225, 50, 125, 142, 233, 102, 66, 126, 177, 163, 241, 208, 178, 132, 25, 234, 52, 26, 5, 152, 59, 38, 229, 242, 145, 196, 86, 87, 134, 215, 236, 68, 162, 66, 132, 161, 96, 150, 33, 186, 115, 93, 2, 149, 140, 68, 223, 139, 188, 88, 116, 34, 184, 143, 246, 125, 208, 48, 199, 16, 52, 200, 192, 112, 40, 209, 189, 243, 206, 17, 84, 97, 184, 56, 147, 90, 86, 173, 78, 176, 76, 131, 2, 126, 123, 18, 109, 80, 61, 128, 229, 214, 56, 105, 16, 204, 209, 119, 136, 97, 63, 117, 67, 101, 142, 90, 235, 164, 219, 207, 169, 133, 186, 81, 202, 215, 171, 51, 194, 242, 44, 104, 161, 46, 36, 209, 215, 49, 136, 152, 219, 148, 128, 131, 49, 183, 83, 146, 165, 197, 109, 194, 244, 102, 120, 199, 155, 7, 104, 68, 205, 155, 158, 117, 154, 29, 247, 230, 64, 52, 89, 90, 65, 244, 53, 14, 149, 70, 86, 112, 56, 249, 19, 218, 126, 128, 251, 11, 225, 221, 13, 108, 7, 246, 120, 216, 214, 104, 211, 50, 142, 7, 191, 126, 1, 57, 26, 127, 133, 196, 11, 36, 125, 240, 19, 244, 160, 215, 7, 179, 24, 57, 212, 68, 182, 149, 158, 55, 23, 2, 23, 143, 181, 127, 135, 47, 147, 61, 199, 78, 13, 4, 95, 162, 45, 22, 92, 198, 71, 207, 112, 35, 22, 140, 158, 90, 70, 230, 25, 210, 63, 34, 233, 26, 207, 47, 35, 251, 20, 205, 223, 4, 216, 243, 213, 63, 95, 35, 87, 10, 254, 10, 169, 52, 18, 136, 5, 139, 209, 161, 228, 43, 12, 175, 96, 244, 183, 109, 246, 252, 46, 246, 241, 77, 251, 100, 169, 52, 214, 243, 10, 119, 86, 112, 239, 37, 248, 50, 202, 163, 61, 63, 67, 103, 120, 138, 163, 73, 26, 213, 24, 126, 199, 233, 60, 57, 150, 234, 93, 134, 115, 251, 249, 234, 95, 189, 223, 147, 235, 123, 48, 79, 207, 203, 144, 254, 70, 19, 99, 171, 232, 247, 46, 68, 50, 159, 6, 218, 218, 86, 209, 134, 61, 18, 154, 36, 76, 0, 171, 104, 69, 192, 155, 104, 18, 37, 39, 205, 59, 72, 88, 120, 99, 44, 208, 63, 76, 108, 223, 146, 52, 79, 60, 13, 3, 30, 123, 224, 95, 80, 75, 7, 8, 22, 56, 3, 120, 233, 4, 0, 0, 210, 8, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 80, 97, 114, 115, 101, 100, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 85, 93, 83, 19, 103, 20, 126, 94, 18, 216, 176, 4, 52, 81, 20, 181, 148, 24, 69, 32, 31, 164, 128, 210, 200, 135, 22, 40, 22, 37, 1, 107, 172, 52, 74, 63, 150, 228, 77, 88, 216, 236, 166, 187, 27, 70, 167, 83, 167, 211, 127, 81, 255, 128, 183, 56, 67, 65, 155, 105, 235, 85, 47, 58, 253, 3, 189, 236, 207, 232, 69, 155, 158, 119, 147, 96, 210, 100, 218, 230, 98, 223, 179, 103, 207, 199, 115, 222, 243, 156, 147, 95, 254, 250, 254, 71, 0, 87, 81, 96, 120, 246, 244, 233, 189, 248, 151, 193, 45, 37, 179, 203, 245, 108, 112, 38, 152, 201, 5, 35, 193, 140, 81, 40, 170, 154, 98, 171, 134, 30, 45, 24, 89, 78, 122, 147, 107, 92, 177, 56, 125, 220, 86, 172, 104, 102, 155, 103, 118, 173, 82, 193, 10, 206, 228, 20, 205, 226, 145, 96, 49, 31, 45, 40, 197, 168, 42, 98, 112, 62, 121, 109, 74, 137, 147, 173, 25, 175, 251, 231, 74, 154, 70, 10, 107, 91, 137, 78, 8, 19, 61, 175, 234, 156, 155, 170, 158, 39, 237, 30, 55, 45, 202, 69, 250, 248, 248, 212, 120, 60, 154, 229, 123, 193, 175, 60, 96, 12, 114, 202, 40, 153, 25, 126, 75, 213, 56, 67, 192, 48, 243, 177, 188, 169, 100, 53, 30, 203, 104, 106, 236, 174, 98, 90, 60, 187, 100, 20, 10, 138, 158, 77, 80, 60, 9, 110, 134, 147, 59, 202, 158, 18, 211, 20, 61, 31, 91, 223, 218, 225, 25, 91, 66, 23, 195, 9, 163, 40, 202, 177, 22, 159, 164, 108, 145, 149, 225, 84, 194, 49, 44, 217, 170, 22, 91, 81, 172, 237, 164, 82, 156, 101, 232, 43, 154, 220, 226, 186, 189, 94, 53, 111, 53, 75, 113, 91, 152, 153, 188, 96, 236, 241, 236, 177, 89, 31, 127, 108, 155, 202, 130, 153, 47, 21, 200, 155, 20, 253, 13, 126, 11, 166, 169, 60, 73, 168, 150, 240, 236, 154, 83, 117, 213, 190, 193, 112, 102, 180, 77, 228, 177, 7, 12, 174, 209, 177, 7, 94, 156, 128, 79, 134, 4, 63, 131, 175, 5, 167, 132, 211, 50, 250, 225, 247, 194, 131, 238, 110, 116, 226, 108, 139, 21, 5, 147, 112, 78, 198, 121, 97, 37, 163, 71, 88, 189, 229, 133, 183, 42, 189, 77, 117, 181, 129, 39, 33, 32, 227, 162, 240, 232, 69, 159, 176, 187, 196, 224, 81, 109, 110, 42, 182, 97, 10, 196, 99, 13, 144, 111, 215, 244, 179, 94, 12, 227, 138, 200, 52, 194, 224, 111, 253, 46, 97, 140, 65, 34, 214, 172, 209, 21, 57, 213, 61, 244, 34, 140, 72, 15, 66, 136, 50, 184, 117, 71, 125, 186, 30, 187, 161, 113, 20, 57, 134, 119, 132, 221, 68, 107, 243, 27, 218, 94, 109, 130, 132, 41, 134, 43, 255, 69, 145, 186, 237, 53, 25, 211, 226, 114, 229, 60, 127, 211, 235, 254, 166, 2, 235, 61, 241, 34, 142, 235, 50, 205, 203, 76, 19, 185, 170, 60, 146, 48, 71, 53, 21, 75, 84, 66, 124, 180, 181, 130, 86, 77, 219, 50, 111, 224, 166, 104, 232, 123, 12, 222, 47, 74, 134, 205, 23, 244, 236, 29, 67, 213, 25, 38, 27, 73, 178, 176, 101, 17, 199, 50, 246, 146, 161, 105, 228, 71, 152, 155, 162, 85, 1, 17, 197, 206, 254, 83, 183, 88, 82, 181, 44, 167, 78, 188, 47, 99, 89, 84, 237, 127, 99, 225, 180, 105, 75, 163, 217, 249, 160, 7, 43, 162, 135, 29, 145, 128, 7, 119, 136, 169, 74, 177, 72, 75, 129, 33, 58, 218, 154, 165, 53, 113, 45, 9, 85, 147, 64, 82, 228, 89, 99, 96, 35, 30, 220, 37, 14, 217, 70, 125, 234, 154, 251, 92, 11, 230, 197, 61, 164, 132, 203, 125, 47, 22, 177, 36, 19, 243, 104, 14, 166, 106, 35, 59, 19, 24, 182, 34, 129, 230, 249, 170, 234, 154, 135, 80, 232, 60, 248, 152, 128, 231, 12, 179, 160, 80, 71, 174, 183, 1, 254, 232, 223, 91, 114, 140, 232, 33, 30, 201, 152, 199, 38, 133, 171, 226, 96, 152, 107, 123, 15, 255, 143, 113, 212, 22, 23, 81, 141, 24, 218, 134, 37, 109, 57, 241, 57, 20, 193, 137, 45, 134, 96, 67, 183, 168, 243, 121, 69, 171, 223, 195, 242, 227, 12, 175, 17, 154, 250, 52, 80, 77, 21, 24, 25, 182, 70, 2, 186, 97, 7, 178, 60, 71, 0, 178, 227, 30, 228, 4, 185, 219, 160, 119, 22, 205, 182, 12, 14, 149, 70, 113, 137, 118, 53, 67, 111, 202, 166, 191, 3, 90, 51, 247, 5, 47, 224, 163, 110, 72, 244, 127, 225, 38, 137, 214, 13, 73, 62, 177, 80, 156, 211, 91, 59, 105, 93, 56, 22, 180, 103, 113, 146, 158, 187, 244, 246, 130, 188, 58, 233, 252, 58, 28, 74, 167, 55, 143, 112, 170, 140, 254, 116, 226, 8, 103, 194, 223, 97, 160, 140, 243, 66, 190, 64, 242, 96, 131, 60, 84, 198, 69, 33, 7, 73, 190, 124, 136, 209, 68, 248, 21, 198, 25, 190, 197, 60, 9, 147, 12, 175, 113, 181, 140, 233, 116, 242, 8, 239, 30, 98, 150, 12, 214, 162, 85, 131, 202, 111, 161, 104, 205, 98, 126, 198, 125, 128, 129, 115, 238, 200, 33, 22, 54, 158, 87, 126, 127, 65, 24, 24, 52, 122, 14, 193, 85, 193, 28, 58, 8, 151, 68, 75, 5, 127, 34, 32, 97, 154, 196, 10, 45, 61, 241, 235, 22, 236, 171, 193, 255, 134, 202, 113, 209, 153, 12, 149, 177, 156, 94, 61, 194, 45, 247, 220, 43, 220, 102, 72, 70, 106, 152, 38, 46, 68, 234, 41, 19, 207, 32, 135, 252, 171, 135, 88, 223, 160, 98, 253, 31, 10, 33, 44, 30, 85, 209, 53, 247, 188, 242, 107, 232, 16, 31, 237, 31, 99, 185, 36, 176, 248, 224, 146, 176, 204, 28, 48, 21, 12, 58, 111, 18, 230, 157, 247, 63, 48, 64, 223, 105, 46, 106, 120, 102, 233, 50, 221, 116, 198, 40, 193, 1, 6, 95, 98, 99, 245, 0, 151, 233, 72, 28, 96, 136, 142, 100, 215, 15, 144, 210, 155, 107, 174, 80, 202, 29, 78, 117, 70, 82, 254, 116, 244, 37, 62, 217, 119, 10, 243, 225, 83, 124, 86, 11, 180, 66, 129, 58, 232, 28, 11, 209, 53, 17, 200, 204, 107, 186, 209, 213, 159, 209, 25, 218, 47, 131, 167, 221, 34, 204, 170, 43, 156, 242, 231, 67, 228, 127, 132, 157, 159, 142, 65, 247, 130, 85, 224, 39, 111, 2, 233, 40, 197, 5, 117, 192, 245, 55, 80, 75, 7, 8, 174, 153, 98, 199, 130, 4, 0, 0, 82, 8, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 80, 97, 114, 115, 101, 100, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 79, 112, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 109, 80, 77, 75, 195, 64, 16, 125, 107, 213, 212, 90, 63, 234, 199, 213, 67, 14, 162, 165, 49, 104, 21, 130, 138, 32, 130, 23, 11, 21, 5, 161, 199, 109, 50, 77, 183, 221, 36, 101, 55, 41, 136, 216, 31, 226, 191, 240, 32, 130, 130, 63, 192, 31, 37, 78, 11, 222, 188, 12, 111, 222, 123, 243, 102, 119, 190, 127, 62, 190, 0, 28, 99, 75, 224, 101, 50, 185, 11, 158, 220, 174, 12, 135, 148, 70, 238, 169, 27, 246, 220, 134, 27, 102, 201, 72, 105, 153, 171, 44, 245, 146, 44, 34, 230, 13, 105, 146, 150, 88, 236, 75, 235, 133, 125, 10, 135, 182, 72, 172, 123, 218, 147, 218, 82, 195, 29, 197, 94, 34, 71, 158, 154, 102, 16, 29, 157, 52, 101, 192, 94, 19, 252, 205, 247, 10, 173, 153, 176, 125, 233, 29, 78, 45, 105, 172, 82, 34, 163, 210, 152, 217, 49, 25, 203, 187, 152, 15, 14, 154, 7, 129, 23, 209, 216, 125, 46, 67, 8, 84, 238, 179, 194, 132, 116, 173, 52, 9, 236, 102, 38, 246, 99, 35, 35, 77, 126, 168, 149, 127, 43, 141, 165, 232, 42, 75, 18, 153, 70, 45, 206, 107, 143, 166, 79, 118, 48, 47, 176, 62, 144, 99, 233, 107, 153, 198, 126, 187, 59, 160, 48, 119, 176, 40, 176, 56, 150, 186, 32, 43, 176, 221, 154, 233, 69, 174, 180, 127, 105, 140, 124, 108, 41, 155, 159, 177, 225, 92, 165, 42, 191, 16, 40, 237, 237, 63, 84, 81, 193, 114, 5, 14, 170, 2, 155, 255, 248, 29, 172, 86, 176, 134, 106, 21, 101, 44, 45, 97, 1, 53, 129, 249, 43, 254, 47, 106, 220, 56, 124, 99, 193, 136, 181, 25, 18, 211, 52, 174, 155, 220, 237, 160, 196, 8, 216, 168, 119, 222, 177, 242, 137, 181, 206, 205, 59, 214, 235, 111, 216, 120, 5, 102, 238, 18, 215, 57, 148, 126, 1, 80, 75, 7, 8, 168, 212, 99, 26, 83, 1, 0, 0, 172, 1, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 102, 105, 108, 101, 47, 80, 97, 116, 104, 84, 114, 97, 118, 101, 114, 115, 97, 108, 67, 104, 101, 99, 107, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 117, 85, 203, 119, 19, 85, 24, 255, 221, 166, 48, 211, 52, 133, 18, 222, 82, 97, 8, 2, 105, 154, 135, 188, 75, 121, 40, 4, 208, 104, 90, 144, 0, 53, 80, 133, 219, 201, 77, 50, 116, 50, 19, 102, 38, 45, 21, 1, 223, 248, 194, 61, 93, 177, 99, 171, 120, 78, 10, 246, 40, 46, 61, 234, 198, 157, 75, 61, 135, 191, 66, 143, 198, 239, 78, 218, 82, 74, 92, 228, 158, 185, 223, 227, 247, 253, 238, 247, 202, 207, 255, 126, 247, 3, 128, 221, 184, 205, 48, 117, 227, 198, 233, 254, 107, 145, 81, 174, 143, 9, 171, 16, 25, 136, 232, 197, 72, 60, 162, 219, 149, 170, 97, 114, 207, 176, 173, 68, 197, 46, 8, 146, 59, 194, 20, 220, 21, 164, 44, 115, 55, 161, 151, 133, 62, 230, 214, 42, 110, 100, 160, 200, 77, 87, 196, 35, 213, 82, 162, 194, 171, 9, 67, 98, 8, 177, 115, 207, 46, 222, 79, 182, 78, 255, 156, 127, 177, 102, 154, 36, 112, 203, 60, 177, 67, 154, 88, 37, 195, 18, 194, 49, 172, 18, 73, 199, 133, 227, 82, 44, 146, 247, 39, 119, 37, 251, 19, 5, 49, 30, 185, 174, 130, 49, 4, 115, 118, 205, 209, 197, 9, 195, 20, 12, 9, 219, 41, 165, 74, 14, 47, 152, 34, 101, 88, 158, 112, 44, 110, 166, 138, 164, 74, 157, 226, 94, 249, 140, 195, 37, 14, 55, 211, 146, 156, 112, 20, 180, 51, 116, 95, 230, 227, 60, 101, 114, 171, 148, 58, 57, 122, 89, 232, 158, 130, 165, 132, 154, 201, 93, 28, 206, 12, 29, 59, 57, 156, 99, 96, 231, 25, 66, 46, 47, 10, 9, 50, 196, 43, 20, 104, 91, 52, 251, 196, 47, 231, 73, 150, 7, 122, 159, 21, 61, 5, 223, 148, 41, 8, 49, 40, 134, 123, 188, 82, 245, 38, 25, 2, 209, 222, 243, 33, 44, 195, 242, 32, 186, 208, 29, 130, 138, 142, 14, 44, 65, 152, 162, 14, 168, 88, 197, 160, 234, 182, 229, 113, 195, 114, 25, 54, 44, 12, 154, 46, 115, 39, 39, 174, 212, 132, 165, 139, 3, 18, 99, 13, 214, 74, 140, 117, 228, 153, 82, 241, 28, 189, 193, 245, 184, 227, 185, 195, 134, 87, 102, 88, 221, 138, 48, 121, 245, 224, 121, 233, 181, 145, 188, 70, 84, 104, 244, 80, 223, 204, 176, 83, 50, 163, 10, 34, 12, 93, 174, 168, 114, 135, 123, 182, 35, 99, 146, 97, 58, 132, 23, 176, 181, 3, 91, 176, 141, 158, 226, 136, 170, 201, 117, 202, 201, 154, 104, 58, 221, 34, 7, 33, 68, 209, 43, 99, 196, 24, 150, 30, 52, 44, 195, 59, 252, 63, 116, 206, 133, 16, 71, 34, 72, 184, 73, 50, 245, 108, 153, 110, 9, 59, 11, 106, 17, 169, 249, 90, 18, 236, 139, 216, 33, 109, 119, 50, 172, 244, 245, 53, 207, 48, 83, 71, 28, 135, 79, 102, 13, 151, 202, 184, 155, 33, 252, 172, 163, 130, 189, 244, 200, 146, 240, 100, 33, 211, 118, 205, 242, 252, 34, 100, 66, 232, 199, 254, 78, 236, 195, 0, 67, 123, 52, 211, 36, 115, 48, 136, 61, 56, 68, 69, 48, 168, 151, 100, 6, 22, 208, 241, 195, 101, 102, 229, 68, 231, 37, 188, 44, 221, 143, 204, 69, 125, 74, 175, 32, 77, 169, 162, 193, 24, 18, 87, 189, 16, 142, 99, 121, 39, 142, 225, 4, 133, 178, 72, 192, 176, 42, 186, 48, 115, 205, 70, 36, 204, 87, 145, 145, 118, 175, 17, 3, 207, 110, 166, 105, 177, 237, 124, 150, 179, 24, 148, 241, 135, 232, 57, 188, 80, 88, 148, 226, 89, 68, 89, 241, 83, 120, 67, 190, 234, 180, 60, 136, 108, 91, 50, 169, 226, 44, 229, 155, 122, 137, 6, 53, 132, 97, 169, 239, 194, 155, 84, 104, 210, 80, 235, 171, 52, 247, 126, 23, 133, 48, 210, 236, 150, 183, 24, 54, 61, 193, 206, 152, 166, 40, 113, 51, 231, 113, 79, 28, 191, 170, 139, 170, 220, 10, 10, 46, 50, 172, 93, 76, 243, 104, 205, 48, 11, 114, 240, 56, 195, 186, 179, 214, 152, 101, 79, 88, 154, 44, 141, 54, 223, 99, 3, 154, 10, 61, 136, 81, 191, 7, 120, 181, 74, 193, 25, 122, 162, 45, 58, 107, 22, 140, 158, 94, 68, 73, 122, 148, 229, 49, 20, 196, 37, 233, 27, 121, 134, 224, 17, 167, 84, 171, 8, 203, 91, 192, 209, 100, 72, 110, 223, 234, 110, 215, 12, 87, 179, 108, 79, 227, 154, 28, 115, 141, 59, 122, 217, 24, 23, 26, 25, 59, 147, 154, 237, 104, 85, 106, 28, 205, 162, 134, 161, 140, 88, 196, 171, 104, 59, 21, 78, 101, 219, 223, 162, 141, 47, 180, 72, 123, 171, 122, 85, 113, 69, 230, 210, 9, 162, 34, 249, 170, 7, 117, 115, 118, 56, 168, 29, 207, 81, 183, 216, 110, 82, 134, 84, 49, 241, 244, 26, 153, 116, 61, 81, 81, 64, 203, 163, 147, 218, 248, 148, 99, 87, 133, 227, 77, 134, 112, 13, 157, 65, 188, 131, 119, 231, 172, 253, 6, 204, 218, 58, 151, 115, 124, 131, 42, 125, 150, 118, 89, 56, 187, 88, 71, 84, 222, 195, 251, 29, 184, 137, 15, 8, 209, 179, 179, 246, 132, 112, 210, 180, 201, 159, 108, 185, 133, 214, 45, 223, 242, 17, 62, 150, 111, 249, 132, 88, 79, 24, 86, 193, 158, 112, 85, 124, 74, 205, 157, 166, 213, 78, 11, 132, 90, 67, 31, 27, 228, 213, 51, 124, 84, 110, 233, 229, 89, 218, 236, 67, 181, 202, 168, 112, 124, 9, 109, 225, 37, 80, 232, 15, 135, 97, 189, 92, 128, 244, 213, 70, 103, 16, 157, 36, 249, 156, 196, 183, 177, 20, 1, 146, 254, 29, 171, 99, 197, 20, 254, 184, 143, 149, 119, 16, 138, 133, 87, 215, 177, 126, 10, 191, 197, 194, 27, 234, 216, 52, 133, 159, 98, 225, 205, 254, 199, 163, 25, 108, 201, 222, 199, 246, 252, 161, 238, 145, 187, 88, 22, 235, 78, 245, 212, 209, 55, 120, 15, 43, 122, 186, 83, 119, 113, 43, 214, 61, 226, 11, 250, 242, 241, 105, 164, 234, 216, 149, 207, 206, 96, 79, 126, 176, 239, 33, 104, 101, 79, 227, 240, 67, 28, 101, 200, 210, 245, 21, 134, 59, 88, 31, 167, 175, 215, 25, 126, 196, 190, 135, 56, 201, 80, 71, 110, 248, 94, 227, 113, 188, 142, 51, 243, 54, 59, 230, 76, 186, 242, 131, 225, 115, 117, 228, 167, 176, 95, 210, 108, 252, 25, 15, 95, 144, 215, 198, 239, 254, 199, 219, 83, 216, 124, 175, 241, 107, 236, 235, 25, 92, 202, 207, 96, 52, 223, 115, 49, 92, 152, 134, 168, 195, 168, 227, 242, 52, 198, 136, 123, 37, 223, 254, 61, 148, 124, 54, 16, 203, 133, 237, 190, 7, 112, 167, 225, 61, 242, 147, 242, 5, 157, 35, 232, 88, 243, 15, 98, 42, 162, 172, 65, 57, 15, 40, 232, 162, 111, 58, 129, 6, 54, 250, 119, 5, 199, 20, 154, 238, 249, 59, 102, 111, 237, 104, 107, 106, 129, 191, 176, 183, 65, 217, 13, 200, 164, 147, 102, 29, 152, 143, 128, 47, 233, 183, 145, 236, 105, 250, 105, 111, 183, 209, 14, 12, 224, 52, 57, 126, 69, 37, 250, 133, 202, 240, 152, 116, 42, 106, 24, 111, 150, 6, 26, 217, 16, 2, 86, 133, 175, 62, 192, 245, 251, 248, 176, 142, 91, 225, 207, 168, 46, 223, 98, 229, 55, 128, 95, 212, 128, 207, 61, 240, 31, 80, 75, 7, 8, 185, 24, 191, 2, 32, 5, 0, 0, 86, 8, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 102, 105, 108, 101, 47, 108, 111, 99, 107, 105, 110, 103, 47, 69, 120, 99, 108, 117, 115, 105, 118, 101, 70, 105, 108, 101, 65, 99, 99, 101, 115, 115, 77, 97, 110, 97, 103, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 101, 81, 193, 110, 211, 64, 16, 125, 219, 36, 117, 147, 198, 180, 161, 80, 238, 62, 181, 81, 93, 11, 10, 149, 213, 34, 36, 84, 193, 137, 10, 65, 81, 57, 111, 54, 19, 103, 155, 245, 218, 218, 181, 163, 86, 136, 126, 8, 63, 192, 185, 39, 4, 7, 142, 28, 248, 40, 96, 108, 21, 113, 96, 15, 179, 59, 239, 189, 121, 51, 154, 253, 249, 235, 219, 119, 0, 143, 241, 64, 224, 211, 245, 245, 219, 244, 67, 52, 145, 106, 65, 118, 26, 29, 69, 106, 22, 237, 69, 170, 200, 75, 109, 100, 165, 11, 27, 231, 197, 148, 24, 119, 100, 72, 122, 98, 114, 46, 125, 172, 230, 164, 22, 190, 206, 125, 116, 52, 147, 198, 211, 94, 84, 102, 113, 46, 203, 88, 55, 30, 68, 143, 158, 28, 200, 148, 181, 46, 253, 91, 63, 171, 141, 97, 192, 207, 101, 252, 176, 145, 216, 76, 91, 34, 167, 109, 198, 232, 146, 156, 231, 94, 140, 167, 251, 7, 251, 105, 60, 165, 101, 244, 113, 13, 66, 96, 112, 86, 212, 78, 209, 75, 109, 72, 224, 184, 112, 89, 146, 57, 57, 53, 148, 104, 91, 145, 179, 210, 36, 51, 166, 18, 83, 168, 5, 91, 37, 47, 46, 149, 169, 189, 94, 182, 5, 207, 149, 34, 239, 79, 165, 149, 25, 185, 0, 93, 129, 205, 11, 185, 148, 137, 145, 172, 124, 61, 185, 32, 85, 5, 88, 21, 88, 125, 170, 173, 174, 158, 9, 116, 118, 118, 207, 135, 88, 67, 127, 128, 0, 3, 129, 81, 46, 175, 38, 116, 98, 10, 79, 111, 106, 77, 149, 185, 18, 216, 222, 121, 213, 154, 232, 34, 105, 9, 57, 49, 116, 188, 123, 206, 226, 255, 224, 0, 119, 4, 122, 170, 73, 135, 216, 68, 127, 29, 27, 24, 9, 108, 253, 27, 130, 199, 165, 178, 217, 114, 128, 45, 129, 238, 9, 175, 74, 32, 60, 171, 248, 55, 78, 101, 249, 174, 49, 193, 8, 61, 158, 166, 57, 43, 16, 205, 112, 28, 239, 115, 54, 226, 91, 240, 221, 27, 127, 193, 250, 77, 43, 232, 99, 136, 240, 150, 62, 188, 165, 55, 198, 63, 16, 142, 191, 226, 174, 192, 103, 116, 223, 223, 48, 216, 101, 81, 136, 123, 252, 218, 102, 65, 136, 149, 223, 28, 0, 17, 48, 198, 96, 167, 109, 213, 249, 3, 80, 75, 7, 8, 135, 217, 45, 95, 156, 1, 0, 0, 38, 2, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 117, 116, 105, 108, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 87, 114, 97, 112, 112, 101, 114, 67, 114, 101, 100, 101, 110, 116, 105, 97, 108, 115, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 85, 221, 115, 19, 85, 20, 255, 221, 38, 97, 147, 52, 208, 124, 80, 42, 95, 90, 99, 109, 211, 77, 210, 200, 151, 212, 82, 170, 180, 182, 24, 147, 54, 72, 90, 48, 69, 45, 155, 205, 77, 186, 176, 217, 13, 155, 13, 216, 97, 224, 213, 191, 65, 124, 118, 120, 225, 161, 58, 144, 130, 140, 14, 79, 117, 198, 127, 199, 39, 29, 199, 122, 238, 38, 253, 14, 14, 249, 188, 247, 220, 115, 206, 239, 156, 223, 57, 247, 236, 31, 255, 254, 242, 27, 128, 179, 176, 24, 30, 61, 124, 120, 117, 244, 126, 180, 168, 168, 183, 185, 81, 138, 142, 69, 213, 114, 52, 17, 85, 205, 106, 77, 211, 21, 91, 51, 141, 100, 213, 44, 113, 146, 91, 92, 231, 74, 157, 211, 225, 178, 82, 79, 170, 203, 92, 189, 93, 111, 84, 235, 209, 177, 178, 162, 215, 121, 34, 90, 171, 36, 171, 74, 45, 169, 9, 31, 156, 159, 62, 119, 70, 25, 37, 93, 107, 116, 211, 190, 220, 208, 117, 18, 212, 151, 149, 228, 41, 161, 98, 84, 52, 131, 115, 75, 51, 42, 36, 189, 203, 173, 58, 97, 145, 124, 116, 228, 204, 200, 104, 178, 196, 239, 70, 31, 120, 193, 24, 252, 121, 179, 97, 169, 124, 70, 211, 57, 67, 220, 180, 42, 169, 138, 165, 148, 116, 158, 106, 216, 154, 158, 210, 12, 155, 91, 134, 162, 167, 174, 91, 74, 173, 198, 173, 41, 139, 151, 184, 97, 107, 20, 146, 4, 55, 67, 240, 150, 114, 87, 73, 233, 138, 81, 73, 229, 138, 183, 184, 106, 75, 56, 192, 224, 177, 77, 74, 150, 33, 156, 221, 62, 206, 219, 34, 148, 11, 12, 7, 139, 74, 93, 83, 23, 234, 220, 74, 27, 101, 147, 225, 192, 184, 102, 104, 246, 4, 195, 80, 108, 191, 250, 126, 201, 240, 53, 6, 87, 108, 248, 90, 0, 221, 56, 232, 135, 132, 67, 1, 120, 225, 243, 193, 131, 96, 0, 254, 214, 42, 204, 112, 200, 182, 86, 46, 115, 251, 138, 101, 82, 216, 246, 10, 195, 194, 27, 185, 111, 73, 156, 220, 203, 13, 67, 21, 5, 74, 205, 180, 23, 23, 134, 59, 229, 19, 220, 43, 147, 112, 132, 65, 178, 120, 77, 87, 84, 226, 244, 72, 108, 106, 170, 131, 97, 0, 111, 225, 168, 31, 125, 56, 182, 233, 194, 1, 205, 154, 170, 162, 115, 9, 39, 24, 220, 87, 115, 185, 249, 45, 18, 119, 156, 146, 237, 219, 120, 199, 135, 147, 232, 103, 232, 182, 205, 172, 121, 143, 42, 67, 189, 195, 48, 24, 219, 175, 221, 17, 60, 138, 247, 4, 248, 0, 67, 223, 222, 211, 201, 134, 166, 151, 184, 37, 97, 144, 210, 104, 53, 195, 136, 23, 49, 134, 222, 14, 12, 182, 10, 33, 251, 49, 132, 56, 213, 82, 52, 137, 81, 98, 72, 118, 82, 221, 39, 106, 35, 81, 56, 73, 140, 8, 23, 41, 134, 19, 177, 14, 108, 237, 84, 60, 37, 20, 79, 51, 120, 109, 179, 117, 200, 112, 56, 214, 49, 199, 179, 56, 39, 116, 63, 100, 56, 254, 63, 85, 149, 48, 74, 29, 75, 129, 235, 43, 219, 252, 237, 232, 232, 93, 113, 183, 69, 1, 140, 225, 66, 55, 62, 194, 56, 229, 172, 154, 134, 170, 216, 123, 108, 95, 155, 51, 217, 78, 224, 99, 193, 253, 39, 100, 203, 239, 52, 232, 42, 237, 161, 118, 19, 119, 145, 146, 172, 112, 123, 74, 87, 234, 164, 18, 217, 149, 164, 35, 36, 95, 159, 98, 90, 92, 130, 25, 134, 208, 118, 142, 45, 7, 116, 67, 63, 219, 115, 171, 218, 158, 59, 97, 5, 48, 137, 207, 253, 72, 35, 67, 168, 52, 129, 150, 167, 104, 172, 56, 87, 45, 77, 173, 40, 4, 162, 149, 111, 116, 48, 77, 7, 144, 195, 21, 97, 250, 5, 69, 185, 127, 80, 220, 247, 34, 79, 157, 52, 62, 159, 203, 76, 207, 77, 120, 177, 64, 51, 160, 70, 209, 223, 51, 173, 82, 127, 217, 180, 250, 189, 184, 78, 231, 154, 81, 226, 223, 230, 202, 4, 22, 75, 11, 159, 5, 44, 10, 150, 110, 48, 248, 234, 141, 98, 189, 93, 234, 35, 177, 116, 186, 35, 169, 95, 227, 27, 161, 190, 36, 184, 124, 157, 134, 34, 52, 138, 12, 242, 54, 85, 151, 132, 103, 69, 181, 103, 149, 218, 64, 94, 171, 214, 116, 158, 174, 86, 27, 182, 82, 212, 249, 180, 65, 35, 68, 66, 233, 141, 57, 116, 174, 66, 217, 15, 14, 10, 52, 178, 141, 33, 124, 183, 157, 105, 84, 115, 42, 105, 134, 175, 236, 237, 218, 173, 206, 186, 13, 189, 27, 183, 80, 37, 34, 90, 37, 56, 152, 183, 233, 225, 65, 78, 230, 69, 84, 12, 61, 89, 26, 236, 115, 141, 106, 145, 91, 142, 4, 33, 154, 120, 18, 61, 111, 186, 104, 69, 163, 144, 86, 33, 49, 8, 233, 223, 3, 26, 16, 8, 208, 175, 73, 187, 227, 112, 209, 27, 232, 145, 11, 133, 53, 244, 196, 159, 33, 148, 120, 134, 200, 79, 16, 47, 31, 14, 163, 183, 173, 104, 146, 161, 80, 156, 149, 215, 49, 158, 144, 131, 35, 193, 165, 38, 142, 255, 140, 119, 155, 120, 63, 243, 18, 67, 5, 121, 41, 60, 188, 134, 68, 19, 31, 4, 71, 154, 56, 19, 167, 69, 19, 231, 95, 224, 98, 23, 94, 161, 175, 144, 89, 135, 71, 94, 77, 132, 135, 233, 224, 210, 166, 116, 149, 28, 50, 212, 232, 55, 8, 182, 129, 139, 132, 32, 161, 79, 162, 75, 36, 226, 157, 196, 84, 27, 124, 129, 242, 232, 162, 255, 177, 248, 58, 206, 147, 131, 203, 97, 207, 210, 143, 56, 240, 24, 178, 28, 127, 5, 79, 246, 41, 66, 113, 250, 62, 71, 246, 123, 244, 202, 79, 17, 161, 93, 196, 217, 73, 238, 199, 112, 187, 158, 184, 158, 108, 33, 249, 224, 14, 253, 133, 1, 186, 105, 4, 49, 139, 185, 54, 196, 32, 229, 39, 32, 142, 201, 5, 242, 148, 33, 251, 172, 231, 87, 72, 133, 130, 75, 206, 187, 227, 249, 231, 184, 250, 196, 225, 36, 36, 70, 72, 203, 134, 117, 195, 77, 86, 192, 119, 178, 32, 32, 27, 158, 167, 252, 201, 120, 29, 190, 240, 181, 204, 99, 112, 71, 60, 27, 254, 210, 17, 71, 10, 115, 191, 195, 203, 72, 126, 42, 25, 28, 107, 226, 171, 194, 196, 15, 27, 127, 10, 180, 136, 235, 100, 19, 55, 5, 228, 73, 247, 205, 38, 212, 185, 151, 224, 133, 49, 183, 156, 92, 195, 242, 81, 119, 134, 8, 15, 202, 47, 64, 207, 78, 98, 140, 204, 61, 44, 147, 144, 91, 236, 102, 226, 98, 17, 124, 64, 124, 211, 110, 213, 169, 183, 200, 113, 17, 210, 63, 130, 199, 161, 13, 132, 29, 78, 135, 232, 3, 108, 80, 188, 110, 9, 158, 214, 158, 152, 22, 162, 36, 92, 18, 248, 182, 74, 112, 151, 1, 19, 167, 125, 155, 251, 191, 169, 151, 238, 144, 255, 48, 57, 138, 210, 89, 142, 240, 138, 164, 111, 209, 158, 57, 237, 209, 5, 215, 127, 80, 75, 7, 8, 61, 118, 253, 160, 216, 4, 0, 0, 234, 8, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 117, 116, 105, 108, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 87, 114, 97, 112, 112, 101, 114, 68, 105, 115, 116, 114, 105, 98, 117, 116, 105, 111, 110, 85, 114, 108, 67, 111, 110, 118, 101, 114, 116, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 133, 146, 75, 111, 211, 64, 20, 133, 207, 208, 180, 46, 169, 91, 10, 109, 202, 251, 101, 54, 41, 196, 53, 80, 144, 172, 6, 177, 160, 8, 9, 169, 21, 168, 161, 160, 44, 39, 206, 141, 51, 237, 248, 161, 241, 36, 27, 68, 127, 8, 191, 162, 171, 84, 162, 18, 91, 36, 126, 20, 112, 157, 80, 64, 5, 9, 75, 182, 53, 199, 231, 187, 247, 158, 241, 124, 253, 246, 233, 51, 128, 71, 240, 4, 62, 30, 28, 236, 132, 239, 189, 142, 140, 246, 41, 237, 122, 27, 94, 212, 243, 26, 94, 148, 37, 185, 210, 210, 170, 44, 245, 147, 172, 75, 172, 27, 210, 36, 11, 226, 143, 125, 89, 248, 81, 159, 162, 253, 98, 144, 20, 222, 70, 79, 234, 130, 26, 94, 30, 251, 137, 204, 125, 85, 214, 32, 122, 248, 120, 93, 134, 236, 53, 225, 9, 223, 27, 104, 205, 66, 209, 151, 254, 131, 210, 146, 198, 42, 37, 50, 42, 141, 89, 29, 146, 41, 184, 23, 235, 225, 218, 250, 90, 232, 119, 105, 232, 125, 152, 133, 16, 168, 182, 178, 129, 137, 232, 133, 210, 36, 16, 102, 38, 14, 98, 35, 187, 154, 130, 129, 85, 58, 80, 169, 37, 147, 74, 29, 188, 51, 50, 207, 201, 60, 87, 133, 53, 170, 51, 40, 7, 223, 53, 122, 51, 75, 185, 50, 91, 28, 84, 4, 22, 247, 228, 80, 6, 90, 166, 113, 240, 170, 179, 71, 145, 117, 48, 35, 176, 18, 77, 76, 167, 80, 129, 251, 245, 173, 49, 160, 178, 160, 236, 222, 220, 250, 141, 183, 108, 57, 119, 115, 117, 34, 165, 100, 131, 221, 157, 151, 77, 1, 247, 207, 181, 131, 170, 192, 204, 19, 149, 42, 251, 84, 160, 86, 255, 7, 255, 214, 133, 139, 249, 42, 230, 176, 32, 112, 54, 38, 219, 226, 125, 77, 56, 232, 114, 125, 245, 111, 187, 139, 69, 156, 47, 205, 23, 78, 58, 253, 28, 205, 193, 50, 55, 248, 133, 183, 114, 138, 84, 79, 69, 175, 165, 177, 46, 86, 38, 204, 69, 129, 59, 255, 15, 52, 30, 232, 114, 21, 53, 92, 17, 152, 182, 25, 199, 224, 125, 171, 159, 10, 234, 226, 26, 174, 151, 166, 27, 2, 149, 77, 254, 189, 2, 243, 45, 203, 39, 104, 91, 230, 111, 100, 71, 19, 111, 246, 52, 28, 148, 23, 231, 194, 44, 223, 2, 183, 120, 245, 12, 21, 76, 241, 219, 59, 198, 92, 187, 189, 125, 239, 8, 231, 70, 88, 250, 130, 165, 99, 212, 218, 119, 27, 35, 92, 58, 194, 213, 17, 110, 30, 54, 14, 199, 236, 109, 126, 46, 64, 124, 103, 152, 49, 135, 115, 148, 106, 89, 225, 12, 166, 126, 0, 80, 75, 7, 8, 112, 242, 101, 125, 218, 1, 0, 0, 199, 2, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 66, 111, 111, 116, 115, 116, 114, 97, 112, 77, 97, 105, 110, 83, 116, 97, 114, 116, 101, 114, 36, 49, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 109, 81, 203, 110, 19, 65, 16, 172, 33, 143, 53, 198, 64, 156, 144, 4, 174, 11, 7, 59, 242, 122, 21, 2, 210, 42, 65, 57, 4, 137, 83, 16, 18, 150, 56, 32, 46, 157, 113, 123, 61, 206, 238, 236, 106, 102, 108, 14, 136, 124, 8, 223, 144, 75, 46, 32, 113, 224, 3, 248, 40, 68, 175, 3, 2, 36, 46, 83, 154, 234, 170, 234, 158, 158, 239, 63, 190, 126, 3, 240, 4, 15, 20, 62, 93, 92, 188, 206, 62, 196, 103, 164, 207, 217, 142, 227, 195, 88, 79, 226, 65, 172, 171, 178, 54, 5, 5, 83, 217, 164, 172, 198, 44, 188, 227, 130, 201, 179, 20, 167, 228, 19, 61, 101, 125, 238, 231, 165, 143, 15, 39, 84, 120, 30, 196, 117, 158, 148, 84, 39, 166, 201, 96, 126, 252, 244, 128, 50, 209, 186, 236, 183, 127, 50, 47, 10, 33, 252, 148, 146, 253, 70, 98, 115, 99, 153, 157, 177, 185, 176, 11, 118, 94, 122, 9, 159, 13, 15, 134, 89, 50, 230, 69, 252, 177, 5, 165, 208, 30, 85, 115, 167, 249, 133, 41, 88, 161, 95, 185, 60, 205, 29, 141, 11, 78, 223, 59, 170, 107, 118, 233, 73, 85, 5, 31, 228, 242, 146, 140, 29, 5, 114, 129, 221, 163, 253, 8, 171, 10, 27, 51, 90, 80, 90, 144, 205, 211, 87, 103, 51, 214, 33, 194, 186, 194, 206, 146, 53, 85, 218, 100, 90, 42, 155, 108, 241, 68, 104, 41, 172, 63, 51, 214, 132, 99, 133, 149, 94, 255, 77, 7, 109, 220, 106, 35, 66, 71, 10, 164, 53, 215, 65, 225, 97, 239, 244, 111, 255, 209, 233, 159, 30, 163, 208, 188, 230, 168, 255, 86, 97, 247, 122, 200, 164, 160, 185, 149, 85, 185, 100, 184, 247, 110, 56, 35, 215, 194, 198, 63, 99, 93, 91, 34, 108, 42, 68, 37, 5, 145, 122, 133, 237, 222, 255, 66, 59, 184, 135, 237, 54, 182, 176, 163, 176, 250, 92, 118, 138, 46, 214, 100, 56, 133, 155, 242, 151, 55, 4, 101, 90, 57, 239, 203, 173, 43, 168, 4, 215, 246, 190, 224, 246, 21, 176, 164, 238, 224, 238, 175, 242, 150, 200, 87, 4, 163, 193, 102, 247, 51, 118, 47, 151, 2, 181, 164, 164, 240, 19, 80, 75, 7, 8, 161, 212, 254, 253, 142, 1, 0, 0, 30, 2, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 68, 111, 119, 110, 108, 111, 97, 100, 36, 68, 101, 102, 97, 117, 108, 116, 68, 111, 119, 110, 108, 111, 97, 100, 80, 114, 111, 103, 114, 101, 115, 115, 76, 105, 115, 116, 101, 110, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 83, 207, 111, 19, 71, 20, 254, 6, 123, 179, 241, 118, 147, 56, 36, 134, 64, 4, 38, 91, 2, 182, 137, 99, 32, 16, 220, 24, 90, 104, 0, 201, 193, 16, 20, 163, 32, 75, 72, 48, 217, 29, 175, 55, 89, 239, 186, 251, 195, 65, 170, 202, 165, 55, 14, 61, 113, 161, 135, 246, 216, 51, 82, 19, 162, 86, 162, 61, 21, 137, 255, 164, 255, 65, 79, 208, 55, 203, 175, 180, 162, 168, 187, 210, 188, 55, 111, 190, 153, 239, 189, 247, 205, 188, 120, 249, 203, 51, 0, 103, 176, 200, 240, 253, 131, 7, 43, 213, 175, 141, 53, 110, 110, 8, 207, 50, 22, 12, 179, 109, 204, 24, 166, 223, 237, 57, 46, 143, 28, 223, 43, 119, 125, 75, 80, 60, 16, 174, 224, 161, 160, 197, 14, 15, 203, 102, 71, 152, 27, 97, 220, 13, 141, 133, 54, 119, 67, 49, 99, 244, 236, 114, 151, 247, 202, 142, 60, 67, 136, 211, 103, 231, 120, 149, 176, 65, 245, 237, 254, 118, 236, 186, 20, 8, 59, 188, 124, 74, 66, 60, 219, 241, 132, 8, 28, 207, 166, 104, 95, 4, 33, 113, 81, 188, 58, 59, 55, 91, 45, 91, 162, 111, 124, 51, 8, 198, 160, 53, 253, 56, 48, 197, 85, 199, 21, 12, 53, 63, 176, 43, 118, 192, 45, 87, 84, 54, 3, 222, 235, 137, 160, 114, 217, 223, 244, 92, 159, 91, 71, 47, 139, 54, 143, 221, 232, 237, 252, 102, 224, 219, 129, 8, 195, 134, 19, 70, 194, 19, 129, 138, 52, 67, 118, 157, 247, 121, 197, 229, 158, 93, 89, 94, 91, 23, 102, 164, 98, 128, 97, 192, 245, 109, 91, 4, 12, 147, 141, 15, 16, 52, 146, 197, 26, 195, 160, 69, 45, 176, 121, 68, 137, 92, 248, 16, 240, 255, 102, 66, 71, 237, 239, 5, 162, 239, 248, 113, 248, 14, 35, 168, 72, 47, 98, 96, 117, 202, 231, 188, 227, 57, 209, 231, 12, 135, 11, 31, 73, 168, 184, 202, 144, 42, 20, 87, 117, 12, 35, 171, 65, 197, 168, 142, 65, 100, 50, 80, 48, 166, 67, 195, 39, 210, 203, 233, 208, 49, 36, 189, 253, 12, 57, 235, 13, 91, 51, 226, 81, 28, 46, 118, 168, 15, 194, 98, 80, 10, 75, 75, 197, 213, 129, 139, 45, 36, 31, 195, 240, 251, 54, 93, 231, 81, 71, 197, 33, 162, 234, 242, 251, 18, 90, 175, 23, 235, 58, 242, 56, 162, 225, 48, 166, 100, 220, 241, 116, 124, 250, 122, 126, 244, 31, 45, 110, 70, 82, 94, 21, 199, 24, 212, 62, 119, 99, 177, 220, 166, 36, 10, 245, 98, 227, 223, 152, 154, 142, 2, 138, 26, 142, 163, 196, 112, 224, 63, 107, 86, 49, 67, 221, 145, 17, 143, 210, 62, 89, 216, 117, 14, 21, 19, 52, 197, 87, 177, 240, 76, 81, 219, 77, 112, 41, 65, 243, 53, 87, 16, 201, 44, 42, 26, 202, 56, 73, 36, 133, 197, 143, 160, 78, 75, 212, 28, 195, 88, 130, 112, 252, 74, 125, 249, 202, 125, 83, 244, 228, 131, 80, 113, 150, 225, 224, 251, 173, 43, 177, 23, 57, 93, 177, 107, 253, 28, 195, 196, 238, 220, 110, 117, 2, 127, 51, 57, 251, 181, 90, 159, 105, 168, 98, 129, 180, 158, 29, 196, 121, 29, 7, 112, 80, 35, 129, 72, 239, 244, 34, 189, 21, 134, 33, 210, 199, 220, 184, 206, 123, 183, 228, 38, 134, 145, 6, 61, 149, 27, 113, 119, 77, 4, 73, 4, 163, 4, 87, 73, 169, 20, 121, 36, 57, 121, 163, 82, 240, 68, 59, 146, 155, 236, 30, 169, 34, 70, 104, 188, 72, 179, 60, 210, 20, 1, 198, 74, 173, 59, 79, 177, 247, 196, 22, 198, 217, 22, 246, 165, 182, 48, 241, 36, 145, 124, 84, 38, 241, 6, 252, 39, 6, 232, 7, 190, 205, 103, 30, 253, 128, 123, 165, 159, 177, 239, 57, 90, 89, 45, 107, 77, 61, 204, 63, 244, 199, 49, 185, 241, 93, 234, 238, 14, 140, 29, 76, 103, 53, 247, 110, 167, 53, 175, 60, 70, 149, 112, 19, 57, 229, 71, 84, 74, 57, 133, 252, 241, 156, 178, 131, 19, 219, 56, 149, 157, 158, 87, 126, 71, 57, 167, 108, 227, 204, 109, 34, 252, 9, 67, 215, 126, 69, 181, 85, 122, 138, 218, 111, 249, 169, 71, 143, 49, 36, 225, 123, 47, 16, 246, 182, 36, 107, 93, 251, 3, 153, 82, 126, 106, 27, 95, 60, 161, 58, 39, 113, 12, 45, 204, 75, 81, 18, 123, 14, 87, 19, 187, 132, 149, 196, 54, 105, 148, 118, 15, 46, 81, 210, 19, 72, 191, 130, 188, 199, 76, 197, 252, 75, 100, 84, 40, 233, 244, 200, 95, 212, 139, 47, 41, 56, 76, 29, 59, 68, 101, 78, 19, 122, 157, 124, 186, 189, 73, 175, 82, 127, 3, 80, 75, 7, 8, 94, 224, 230, 114, 75, 3, 0, 0, 18, 5, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 68, 111, 119, 110, 108, 111, 97, 100, 36, 80, 114, 111, 120, 121, 65, 117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 111, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 84, 91, 79, 19, 65, 20, 254, 134, 2, 91, 150, 5, 10, 34, 120, 71, 23, 212, 109, 161, 173, 138, 151, 10, 168, 64, 17, 47, 64, 32, 220, 98, 227, 131, 25, 182, 67, 187, 178, 221, 173, 179, 91, 144, 24, 253, 33, 62, 248, 232, 51, 26, 45, 81, 19, 227, 147, 38, 254, 40, 241, 44, 23, 211, 86, 19, 105, 210, 217, 217, 115, 190, 115, 206, 119, 206, 124, 179, 63, 127, 125, 254, 10, 224, 42, 166, 25, 94, 191, 122, 53, 159, 122, 161, 175, 112, 115, 77, 56, 89, 125, 72, 55, 87, 245, 1, 221, 116, 11, 69, 203, 230, 190, 229, 58, 241, 130, 155, 21, 100, 151, 194, 22, 220, 19, 228, 204, 115, 47, 110, 230, 133, 185, 230, 149, 10, 158, 62, 180, 202, 109, 79, 12, 232, 197, 92, 188, 192, 139, 113, 43, 200, 33, 196, 149, 107, 131, 60, 69, 88, 153, 58, 136, 95, 45, 217, 54, 25, 188, 60, 143, 95, 14, 32, 78, 206, 114, 132, 144, 150, 147, 35, 235, 186, 144, 30, 213, 34, 123, 42, 49, 152, 72, 197, 179, 98, 93, 127, 25, 6, 99, 80, 23, 220, 146, 52, 197, 164, 101, 11, 134, 132, 43, 115, 201, 156, 228, 89, 91, 36, 55, 36, 47, 22, 133, 76, 78, 184, 27, 142, 237, 242, 108, 223, 156, 116, 159, 111, 142, 149, 252, 188, 112, 124, 203, 228, 190, 43, 21, 212, 51, 116, 61, 229, 235, 60, 233, 8, 63, 89, 227, 107, 100, 136, 120, 155, 158, 47, 10, 20, 73, 153, 124, 75, 120, 12, 109, 211, 187, 248, 146, 111, 217, 201, 25, 94, 28, 102, 104, 28, 177, 28, 203, 191, 205, 208, 97, 212, 248, 162, 203, 12, 33, 35, 186, 172, 65, 133, 166, 66, 65, 139, 134, 48, 154, 154, 208, 128, 54, 134, 227, 57, 225, 207, 113, 207, 219, 112, 101, 182, 162, 52, 181, 201, 208, 107, 68, 167, 255, 240, 250, 55, 136, 42, 71, 40, 195, 188, 120, 86, 18, 30, 17, 94, 220, 44, 210, 4, 140, 202, 192, 170, 134, 250, 170, 144, 195, 26, 142, 160, 51, 224, 116, 148, 161, 239, 48, 17, 10, 186, 25, 26, 230, 230, 103, 31, 101, 24, 46, 28, 182, 200, 113, 156, 104, 194, 49, 156, 172, 34, 75, 103, 186, 52, 79, 218, 138, 84, 146, 37, 11, 225, 79, 227, 76, 64, 170, 135, 65, 171, 244, 40, 56, 199, 208, 28, 12, 76, 186, 190, 107, 186, 54, 67, 231, 65, 176, 205, 157, 92, 114, 193, 15, 148, 66, 9, 122, 209, 167, 66, 199, 121, 134, 238, 90, 239, 120, 201, 178, 179, 130, 78, 246, 162, 10, 3, 45, 116, 114, 129, 66, 156, 44, 67, 220, 248, 59, 213, 223, 217, 247, 227, 169, 72, 12, 253, 65, 138, 1, 146, 95, 162, 24, 200, 106, 201, 19, 50, 140, 4, 67, 216, 119, 247, 192, 26, 46, 5, 76, 12, 92, 102, 104, 169, 146, 133, 130, 65, 146, 5, 245, 66, 99, 172, 172, 59, 187, 242, 84, 152, 126, 85, 221, 125, 147, 134, 107, 184, 222, 76, 247, 241, 6, 205, 172, 150, 149, 130, 155, 12, 173, 123, 52, 14, 148, 18, 6, 169, 131, 164, 118, 139, 161, 231, 63, 50, 82, 112, 135, 38, 235, 187, 233, 60, 151, 99, 82, 242, 77, 134, 122, 35, 250, 56, 173, 97, 12, 227, 42, 134, 144, 166, 73, 254, 99, 60, 143, 211, 123, 186, 190, 171, 98, 20, 147, 26, 218, 209, 17, 28, 220, 125, 10, 79, 211, 133, 166, 166, 23, 124, 250, 102, 80, 191, 139, 124, 197, 22, 228, 111, 32, 55, 192, 104, 71, 119, 128, 118, 117, 180, 87, 209, 76, 235, 67, 122, 235, 162, 247, 58, 122, 170, 177, 204, 54, 90, 251, 63, 34, 242, 14, 193, 175, 61, 200, 188, 143, 121, 131, 122, 132, 232, 41, 99, 101, 116, 189, 199, 169, 183, 88, 141, 101, 202, 56, 91, 38, 61, 126, 64, 228, 11, 140, 76, 255, 147, 109, 68, 203, 136, 119, 36, 105, 41, 227, 202, 39, 164, 234, 240, 13, 67, 153, 153, 239, 184, 25, 171, 5, 141, 212, 128, 166, 126, 160, 177, 227, 246, 212, 23, 140, 102, 168, 196, 196, 0, 225, 238, 109, 197, 182, 241, 96, 107, 151, 249, 20, 173, 39, 80, 183, 131, 39, 8, 41, 52, 154, 224, 143, 29, 180, 128, 41, 212, 93, 128, 8, 237, 246, 21, 250, 13, 80, 75, 7, 8, 31, 218, 45, 130, 5, 3, 0, 0, 65, 5, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 68, 111, 119, 110, 108, 111, 97, 100, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 165, 88, 11, 124, 83, 231, 117, 63, 199, 200, 190, 178, 44, 192, 182, 128, 196, 129, 16, 225, 66, 240, 75, 54, 225, 17, 140, 32, 73, 177, 195, 195, 177, 48, 4, 217, 16, 5, 26, 231, 90, 186, 182, 5, 146, 174, 114, 239, 21, 224, 180, 161, 79, 218, 166, 201, 250, 10, 164, 13, 77, 210, 188, 54, 218, 174, 91, 66, 6, 178, 137, 219, 146, 118, 41, 233, 186, 102, 239, 181, 219, 218, 109, 221, 227, 215, 61, 187, 110, 93, 187, 165, 25, 169, 247, 63, 223, 149, 108, 249, 1, 205, 58, 240, 79, 223, 119, 206, 247, 157, 243, 157, 239, 188, 191, 251, 205, 159, 191, 244, 21, 34, 90, 207, 191, 202, 116, 250, 248, 241, 189, 237, 239, 172, 31, 208, 227, 135, 141, 76, 162, 62, 92, 31, 31, 172, 111, 169, 143, 155, 233, 108, 50, 165, 59, 73, 51, 19, 74, 155, 9, 3, 120, 203, 72, 25, 186, 109, 96, 113, 88, 183, 67, 241, 97, 35, 126, 216, 206, 165, 237, 250, 240, 160, 158, 178, 141, 150, 250, 236, 80, 40, 173, 103, 67, 73, 225, 97, 24, 107, 55, 172, 211, 219, 177, 215, 106, 47, 210, 15, 230, 82, 41, 32, 236, 97, 61, 116, 147, 108, 201, 12, 37, 51, 134, 97, 37, 51, 67, 192, 30, 49, 44, 27, 103, 1, 223, 222, 186, 174, 181, 61, 148, 48, 142, 212, 63, 224, 37, 102, 242, 69, 205, 156, 21, 55, 182, 39, 83, 6, 211, 82, 211, 26, 106, 27, 178, 244, 68, 202, 104, 59, 106, 233, 217, 172, 97, 181, 221, 110, 30, 205, 164, 76, 61, 161, 145, 135, 169, 250, 144, 126, 68, 111, 75, 233, 153, 161, 182, 221, 3, 135, 140, 184, 163, 81, 5, 83, 69, 202, 28, 26, 50, 44, 208, 71, 230, 96, 16, 81, 139, 155, 65, 156, 181, 204, 33, 203, 176, 237, 72, 210, 118, 140, 140, 16, 220, 50, 23, 65, 241, 196, 149, 183, 27, 131, 122, 46, 229, 20, 225, 61, 51, 200, 133, 165, 61, 130, 121, 26, 43, 32, 116, 146, 134, 205, 180, 48, 162, 100, 204, 57, 201, 84, 219, 46, 61, 139, 77, 11, 50, 134, 115, 212, 180, 14, 247, 38, 211, 134, 153, 115, 152, 184, 139, 233, 154, 184, 153, 129, 86, 156, 232, 44, 6, 205, 13, 37, 28, 166, 22, 54, 55, 150, 160, 119, 234, 246, 176, 203, 188, 102, 22, 82, 163, 90, 168, 100, 75, 50, 147, 116, 110, 101, 154, 215, 208, 184, 207, 79, 139, 104, 177, 143, 2, 180, 132, 105, 209, 92, 188, 53, 186, 150, 201, 107, 100, 28, 107, 36, 106, 64, 192, 154, 134, 210, 195, 128, 218, 236, 167, 235, 104, 169, 143, 234, 104, 25, 211, 252, 105, 75, 26, 45, 7, 109, 210, 49, 44, 221, 49, 161, 210, 37, 211, 104, 187, 10, 120, 48, 8, 210, 138, 42, 186, 129, 234, 153, 106, 103, 175, 107, 180, 146, 73, 131, 231, 245, 24, 199, 28, 37, 245, 221, 126, 186, 145, 86, 87, 209, 42, 106, 96, 242, 100, 20, 122, 81, 145, 119, 137, 7, 128, 115, 19, 53, 203, 190, 22, 166, 192, 52, 221, 175, 220, 38, 55, 210, 168, 21, 250, 24, 50, 156, 110, 99, 196, 79, 107, 100, 111, 27, 221, 4, 153, 29, 51, 234, 136, 127, 206, 228, 235, 98, 193, 119, 29, 173, 247, 145, 70, 27, 176, 23, 228, 251, 244, 84, 206, 240, 211, 70, 151, 65, 251, 52, 103, 116, 73, 52, 10, 67, 244, 172, 152, 184, 189, 97, 182, 160, 179, 49, 115, 222, 102, 11, 221, 34, 182, 130, 237, 214, 54, 92, 197, 159, 231, 240, 134, 174, 198, 125, 34, 239, 18, 63, 121, 169, 178, 146, 202, 169, 195, 79, 126, 154, 47, 179, 219, 153, 54, 255, 63, 92, 93, 163, 237, 76, 203, 175, 38, 142, 235, 101, 59, 125, 180, 131, 186, 252, 228, 163, 42, 57, 181, 219, 79, 11, 104, 161, 204, 118, 49, 45, 131, 199, 15, 38, 135, 114, 150, 1, 238, 199, 70, 182, 230, 156, 97, 120, 92, 50, 174, 178, 144, 159, 118, 139, 139, 150, 211, 30, 184, 129, 173, 15, 26, 125, 86, 82, 157, 168, 110, 137, 0, 106, 235, 219, 219, 85, 212, 87, 17, 100, 242, 151, 194, 26, 245, 50, 85, 194, 82, 81, 100, 175, 52, 76, 181, 79, 236, 215, 71, 251, 193, 18, 216, 157, 166, 237, 248, 41, 230, 226, 238, 118, 113, 123, 76, 203, 245, 54, 200, 124, 144, 222, 33, 43, 247, 20, 86, 116, 103, 216, 79, 247, 186, 187, 117, 215, 3, 238, 204, 25, 22, 92, 40, 238, 34, 19, 76, 85, 64, 110, 183, 244, 161, 52, 46, 226, 167, 65, 23, 15, 127, 202, 54, 204, 118, 167, 183, 130, 233, 250, 229, 200, 92, 221, 39, 229, 240, 67, 72, 130, 165, 58, 137, 142, 100, 28, 253, 216, 182, 99, 113, 35, 43, 106, 214, 40, 197, 116, 221, 20, 131, 189, 57, 88, 32, 109, 148, 172, 103, 16, 68, 219, 117, 100, 227, 68, 208, 49, 131, 89, 221, 178, 141, 32, 248, 120, 41, 203, 212, 116, 245, 107, 245, 14, 91, 230, 81, 125, 32, 101, 20, 4, 178, 124, 100, 18, 18, 154, 55, 81, 240, 42, 166, 27, 102, 24, 212, 133, 146, 102, 155, 228, 127, 144, 21, 45, 90, 192, 104, 116, 20, 233, 70, 89, 195, 130, 138, 221, 34, 81, 93, 12, 214, 34, 153, 159, 70, 232, 126, 31, 29, 163, 119, 34, 206, 211, 135, 19, 73, 203, 246, 211, 3, 180, 90, 80, 199, 153, 86, 70, 13, 11, 153, 54, 104, 25, 78, 206, 202, 224, 94, 59, 123, 123, 247, 0, 178, 179, 102, 6, 151, 139, 163, 124, 133, 131, 94, 122, 143, 159, 246, 82, 84, 92, 240, 125, 76, 229, 142, 217, 183, 55, 82, 114, 150, 43, 112, 4, 103, 125, 128, 78, 136, 162, 63, 8, 135, 46, 74, 209, 145, 27, 28, 52, 44, 35, 177, 59, 231, 32, 248, 161, 24, 67, 79, 107, 244, 97, 166, 107, 75, 229, 156, 190, 250, 17, 201, 178, 51, 175, 47, 90, 123, 216, 71, 15, 209, 175, 48, 213, 77, 173, 150, 82, 22, 118, 125, 204, 71, 15, 210, 199, 167, 7, 64, 68, 163, 79, 162, 218, 32, 169, 103, 58, 205, 76, 6, 217, 4, 22, 133, 185, 103, 220, 97, 106, 13, 183, 57, 73, 167, 124, 244, 8, 61, 10, 105, 244, 68, 98, 122, 76, 50, 53, 206, 105, 174, 217, 124, 68, 166, 79, 211, 99, 162, 188, 211, 72, 239, 113, 61, 21, 207, 161, 187, 48, 250, 108, 195, 218, 58, 164, 130, 227, 113, 9, 142, 114, 122, 2, 21, 95, 176, 33, 133, 246, 210, 103, 81, 48, 230, 102, 170, 209, 211, 96, 101, 27, 206, 94, 227, 190, 156, 97, 59, 133, 106, 53, 194, 180, 250, 45, 69, 151, 200, 244, 44, 61, 231, 163, 103, 8, 109, 80, 13, 24, 21, 120, 79, 150, 97, 79, 67, 151, 108, 58, 67, 159, 147, 77, 159, 135, 234, 212, 105, 122, 162, 176, 195, 79, 191, 238, 46, 125, 177, 24, 51, 34, 228, 78, 199, 201, 206, 16, 244, 55, 81, 245, 135, 132, 212, 117, 169, 78, 120, 148, 159, 94, 144, 116, 242, 60, 157, 45, 22, 37, 216, 177, 43, 83, 226, 0, 191, 85, 186, 176, 187, 36, 2, 207, 163, 57, 152, 121, 155, 142, 92, 50, 149, 144, 52, 60, 202, 180, 120, 142, 235, 187, 78, 113, 193, 71, 99, 244, 18, 130, 64, 50, 115, 6, 225, 182, 12, 55, 156, 181, 185, 192, 11, 198, 255, 18, 125, 89, 40, 190, 2, 155, 4, 7, 77, 11, 113, 30, 145, 72, 120, 153, 41, 52, 215, 25, 87, 229, 244, 53, 225, 244, 219, 51, 40, 231, 40, 115, 179, 41, 191, 46, 148, 151, 228, 103, 131, 143, 242, 34, 127, 96, 142, 156, 162, 209, 55, 145, 62, 38, 173, 16, 53, 209, 207, 22, 77, 89, 162, 188, 111, 193, 138, 48, 69, 137, 166, 161, 205, 146, 132, 81, 178, 128, 195, 127, 143, 126, 95, 236, 251, 7, 8, 244, 33, 229, 31, 40, 118, 78, 4, 173, 171, 228, 254, 63, 18, 3, 62, 67, 127, 204, 84, 118, 160, 67, 163, 63, 133, 195, 128, 12, 90, 45, 111, 56, 208, 33, 5, 227, 59, 244, 103, 62, 58, 71, 127, 62, 173, 19, 128, 192, 134, 52, 171, 223, 69, 226, 138, 231, 44, 73, 91, 46, 106, 102, 151, 225, 98, 33, 196, 95, 210, 95, 249, 232, 123, 244, 215, 32, 72, 218, 93, 16, 193, 178, 114, 89, 199, 72, 248, 233, 111, 36, 135, 125, 143, 254, 182, 156, 228, 95, 53, 17, 140, 95, 204, 165, 81, 71, 119, 114, 118, 231, 48, 88, 25, 74, 166, 59, 238, 16, 39, 248, 1, 253, 131, 148, 224, 127, 44, 54, 122, 51, 178, 135, 70, 255, 140, 189, 71, 45, 180, 107, 40, 115, 184, 71, 151, 10, 129, 127, 165, 31, 250, 232, 95, 232, 223, 144, 117, 138, 29, 64, 240, 168, 110, 7, 147, 83, 210, 180, 122, 233, 223, 153, 208, 88, 252, 24, 155, 174, 216, 6, 104, 244, 19, 148, 83, 180, 227, 126, 250, 47, 113, 199, 159, 210, 127, 227, 188, 120, 202, 180, 17, 17, 63, 147, 10, 127, 142, 222, 144, 163, 222, 16, 123, 163, 29, 173, 41, 158, 7, 167, 8, 14, 90, 102, 26, 254, 247, 115, 196, 125, 112, 80, 213, 160, 112, 208, 113, 77, 28, 108, 240, 162, 111, 166, 121, 105, 187, 209, 203, 101, 226, 39, 40, 44, 11, 134, 17, 139, 173, 89, 233, 38, 36, 167, 120, 185, 124, 90, 115, 42, 189, 48, 107, 32, 26, 146, 158, 246, 198, 95, 224, 152, 197, 254, 139, 43, 217, 87, 197, 94, 174, 66, 80, 11, 127, 187, 244, 128, 249, 76, 173, 87, 107, 163, 102, 118, 54, 104, 108, 121, 33, 174, 83, 218, 210, 75, 167, 166, 226, 149, 107, 124, 92, 205, 181, 165, 25, 112, 6, 233, 34, 68, 38, 114, 82, 161, 57, 67, 105, 47, 201, 199, 211, 182, 10, 67, 94, 194, 215, 248, 120, 49, 95, 59, 237, 81, 224, 94, 203, 214, 248, 58, 232, 203, 66, 42, 77, 90, 70, 143, 153, 233, 193, 107, 205, 207, 203, 216, 231, 227, 165, 124, 125, 53, 251, 22, 86, 178, 127, 134, 146, 174, 24, 247, 155, 171, 121, 5, 83, 253, 20, 58, 153, 57, 98, 30, 54, 218, 34, 122, 122, 32, 161, 239, 50, 28, 125, 80, 143, 67, 170, 17, 141, 223, 134, 38, 41, 61, 133, 96, 250, 86, 233, 9, 5, 58, 80, 12, 155, 137, 157, 122, 6, 58, 181, 87, 70, 76, 243, 112, 46, 123, 245, 198, 103, 26, 97, 239, 72, 214, 248, 229, 22, 221, 35, 175, 78, 219, 56, 123, 181, 83, 79, 165, 162, 8, 33, 56, 203, 42, 190, 209, 199, 43, 121, 245, 194, 10, 198, 51, 165, 28, 238, 144, 194, 37, 91, 103, 217, 187, 4, 30, 204, 101, 84, 221, 104, 219, 94, 152, 128, 77, 19, 55, 215, 34, 188, 241, 130, 89, 152, 64, 203, 109, 37, 7, 114, 178, 210, 103, 165, 188, 220, 250, 11, 95, 19, 115, 24, 106, 210, 155, 151, 241, 26, 49, 241, 77, 238, 243, 87, 85, 80, 56, 233, 145, 100, 66, 220, 121, 29, 26, 136, 130, 15, 247, 226, 98, 25, 47, 227, 173, 211, 92, 226, 224, 74, 96, 149, 5, 50, 122, 170, 109, 191, 187, 181, 19, 173, 142, 120, 158, 158, 130, 91, 109, 132, 91, 225, 145, 181, 195, 40, 41, 208, 125, 255, 135, 246, 247, 10, 42, 153, 235, 61, 198, 155, 56, 236, 227, 118, 222, 44, 181, 238, 57, 153, 221, 82, 236, 118, 231, 230, 162, 241, 109, 112, 191, 194, 5, 221, 0, 222, 10, 13, 23, 16, 123, 116, 219, 198, 179, 60, 225, 229, 78, 169, 148, 157, 87, 173, 82, 188, 13, 9, 139, 241, 8, 154, 63, 160, 219, 201, 184, 48, 235, 202, 12, 154, 94, 222, 233, 190, 2, 138, 8, 63, 223, 161, 94, 1, 220, 13, 111, 80, 233, 195, 203, 120, 251, 84, 32, 240, 160, 174, 25, 197, 187, 152, 133, 238, 246, 243, 110, 222, 227, 163, 205, 124, 39, 138, 223, 254, 173, 123, 123, 186, 122, 118, 4, 251, 108, 201, 137, 170, 89, 245, 114, 84, 245, 164, 48, 17, 146, 201, 92, 170, 233, 227, 125, 149, 80, 8, 158, 58, 254, 14, 3, 205, 178, 21, 44, 24, 52, 6, 194, 14, 17, 217, 203, 7, 152, 210, 193, 233, 45, 94, 208, 148, 206, 88, 207, 32, 211, 219, 6, 202, 149, 180, 195, 197, 190, 70, 250, 255, 98, 181, 9, 130, 40, 184, 67, 249, 68, 176, 212, 65, 91, 131, 123, 212, 87, 35, 33, 179, 197, 167, 130, 185, 73, 169, 163, 173, 94, 126, 7, 20, 38, 39, 154, 86, 242, 126, 117, 160, 151, 251, 81, 120, 10, 34, 122, 89, 247, 67, 129, 74, 242, 56, 148, 164, 228, 4, 214, 40, 86, 84, 101, 86, 96, 141, 155, 215, 107, 140, 119, 149, 15, 154, 222, 150, 145, 142, 221, 18, 147, 149, 70, 149, 187, 107, 101, 97, 21, 26, 73, 242, 33, 31, 15, 243, 97, 166, 21, 110, 206, 68, 41, 140, 15, 203, 123, 6, 221, 131, 131, 192, 215, 173, 68, 167, 11, 195, 141, 209, 38, 148, 247, 245, 110, 239, 199, 179, 254, 186, 200, 172, 253, 133, 125, 224, 106, 114, 182, 146, 51, 124, 159, 251, 32, 236, 24, 113, 228, 139, 205, 228, 51, 117, 14, 146, 198, 3, 29, 126, 182, 217, 17, 235, 230, 138, 47, 130, 57, 36, 214, 24, 79, 157, 5, 134, 2, 122, 39, 63, 76, 44, 145, 86, 99, 46, 123, 143, 240, 253, 62, 62, 198, 120, 241, 52, 77, 113, 220, 58, 0, 203, 32, 209, 202, 167, 143, 104, 50, 157, 77, 25, 93, 233, 116, 206, 145, 246, 201, 253, 18, 194, 15, 204, 232, 158, 175, 252, 101, 66, 21, 168, 119, 251, 248, 56, 191, 7, 63, 116, 147, 252, 180, 23, 229, 87, 59, 183, 218, 182, 124, 61, 50, 51, 219, 44, 75, 138, 213, 7, 208, 252, 117, 21, 82, 69, 208, 16, 92, 56, 216, 151, 49, 142, 101, 193, 15, 175, 174, 248, 84, 214, 8, 218, 104, 94, 12, 184, 199, 7, 175, 16, 16, 234, 240, 15, 251, 248, 4, 63, 136, 240, 146, 245, 214, 35, 104, 104, 77, 4, 240, 67, 133, 55, 79, 107, 225, 147, 162, 151, 241, 86, 90, 232, 98, 210, 83, 200, 143, 193, 203, 76, 187, 53, 163, 167, 13, 47, 127, 2, 158, 3, 96, 114, 241, 17, 119, 81, 183, 226, 195, 94, 62, 37, 239, 125, 229, 217, 71, 189, 252, 41, 52, 25, 107, 188, 252, 24, 242, 202, 42, 187, 109, 149, 29, 108, 88, 101, 111, 86, 127, 141, 37, 83, 47, 127, 6, 222, 138, 142, 57, 173, 163, 40, 111, 154, 35, 219, 29, 120, 43, 77, 48, 204, 248, 4, 63, 41, 142, 129, 87, 144, 71, 94, 13, 136, 22, 184, 102, 252, 48, 12, 216, 43, 86, 147, 175, 137, 201, 140, 209, 147, 75, 15, 32, 67, 187, 152, 234, 14, 211, 116, 196, 206, 89, 183, 72, 217, 84, 131, 119, 149, 134, 6, 209, 131, 153, 151, 42, 49, 171, 145, 15, 48, 106, 244, 211, 124, 53, 46, 160, 133, 24, 53, 172, 86, 3, 98, 126, 26, 208, 35, 52, 15, 52, 68, 209, 166, 113, 10, 196, 186, 71, 233, 154, 60, 93, 127, 129, 80, 178, 35, 205, 23, 168, 145, 233, 49, 218, 132, 73, 136, 233, 101, 106, 139, 197, 118, 93, 160, 181, 140, 198, 235, 230, 158, 11, 180, 137, 233, 85, 66, 102, 59, 67, 243, 91, 20, 4, 244, 174, 166, 80, 75, 158, 110, 219, 127, 102, 226, 149, 166, 23, 192, 150, 249, 25, 252, 182, 146, 103, 130, 106, 169, 76, 163, 128, 70, 171, 136, 222, 164, 38, 141, 218, 52, 218, 60, 1, 89, 60, 46, 22, 16, 254, 136, 94, 7, 10, 141, 43, 189, 189, 32, 224, 26, 170, 80, 2, 174, 108, 138, 29, 60, 56, 74, 91, 155, 207, 83, 103, 203, 121, 218, 54, 78, 59, 98, 221, 205, 163, 116, 71, 211, 121, 138, 92, 127, 158, 122, 242, 116, 231, 89, 213, 35, 87, 202, 27, 190, 64, 125, 10, 144, 168, 229, 182, 113, 234, 139, 9, 135, 60, 221, 213, 205, 145, 60, 29, 216, 149, 167, 254, 91, 243, 52, 16, 246, 228, 201, 8, 151, 231, 105, 56, 92, 209, 212, 220, 114, 125, 157, 167, 174, 188, 174, 98, 148, 14, 191, 208, 61, 78, 102, 172, 246, 190, 166, 81, 114, 46, 42, 38, 85, 180, 142, 210, 208, 98, 141, 26, 107, 105, 177, 26, 151, 80, 157, 26, 151, 210, 114, 53, 6, 233, 109, 106, 92, 69, 107, 213, 232, 170, 192, 79, 60, 1, 16, 26, 209, 128, 171, 161, 28, 29, 113, 69, 156, 247, 53, 215, 78, 252, 64, 51, 52, 247, 174, 60, 189, 123, 127, 237, 123, 123, 56, 236, 225, 112, 249, 24, 189, 63, 79, 31, 138, 133, 43, 198, 233, 193, 88, 88, 27, 167, 135, 98, 45, 163, 244, 209, 81, 250, 68, 158, 62, 5, 99, 52, 181, 224, 127, 76, 232, 62, 147, 167, 39, 107, 159, 234, 207, 211, 175, 157, 19, 77, 124, 65, 253, 254, 198, 87, 233, 249, 199, 104, 71, 203, 203, 244, 124, 158, 94, 140, 109, 169, 161, 111, 60, 69, 21, 103, 104, 195, 56, 229, 99, 227, 52, 22, 91, 218, 31, 26, 165, 241, 60, 93, 172, 253, 106, 158, 94, 169, 171, 200, 211, 171, 121, 250, 198, 40, 253, 206, 197, 72, 157, 22, 246, 156, 161, 215, 10, 227, 169, 150, 88, 158, 254, 48, 92, 94, 211, 64, 95, 242, 66, 123, 127, 114, 75, 101, 71, 229, 70, 111, 93, 121, 115, 158, 254, 34, 118, 179, 175, 236, 179, 20, 27, 163, 239, 231, 233, 239, 78, 211, 246, 37, 222, 21, 139, 125, 39, 14, 118, 232, 27, 189, 43, 244, 131, 29, 139, 232, 239, 211, 149, 39, 159, 160, 170, 37, 222, 101, 39, 78, 158, 166, 69, 77, 43, 128, 50, 58, 206, 81, 100, 217, 137, 37, 222, 60, 253, 83, 157, 214, 60, 111, 177, 47, 79, 63, 2, 229, 70, 111, 199, 153, 137, 47, 136, 128, 181, 255, 33, 130, 212, 105, 117, 229, 77, 231, 168, 179, 246, 63, 243, 244, 122, 158, 254, 39, 79, 151, 207, 70, 206, 208, 250, 72, 241, 14, 163, 244, 102, 237, 68, 81, 250, 0, 51, 166, 77, 234, 246, 23, 3, 60, 15, 0, 46, 212, 60, 202, 158, 139, 37, 108, 46, 17, 36, 7, 175, 58, 143, 204, 96, 255, 203, 205, 23, 105, 133, 24, 144, 67, 244, 154, 26, 27, 97, 169, 21, 212, 162, 96, 25, 5, 110, 161, 54, 58, 9, 88, 198, 135, 1, 175, 165, 62, 5, 203, 40, 240, 93, 212, 175, 96, 25, 5, 62, 68, 247, 41, 88, 70, 129, 45, 252, 10, 44, 163, 192, 143, 211, 211, 10, 150, 81, 224, 103, 233, 115, 10, 150, 81, 224, 207, 211, 23, 21, 44, 163, 192, 47, 226, 181, 36, 176, 140, 2, 95, 164, 175, 43, 88, 70, 129, 191, 67, 223, 85, 176, 140, 2, 255, 136, 126, 172, 96, 25, 5, 254, 9, 151, 41, 88, 70, 192, 120, 220, 44, 22, 88, 141, 2, 183, 242, 61, 234, 158, 101, 226, 176, 124, 3, 45, 156, 128, 232, 94, 141, 240, 160, 146, 200, 212, 232, 156, 70, 143, 104, 244, 32, 209, 4, 116, 34, 11, 164, 210, 201, 57, 18, 36, 252, 250, 119, 39, 48, 43, 193, 23, 182, 99, 229, 181, 201, 21, 13, 175, 233, 233, 172, 22, 171, 5, 141, 190, 205, 158, 73, 60, 2, 254, 50, 109, 224, 234, 9, 201, 102, 179, 24, 146, 164, 141, 153, 231, 3, 25, 160, 10, 23, 9, 88, 208, 174, 72, 243, 72, 43, 197, 130, 71, 65, 34, 82, 11, 152, 150, 46, 129, 77, 155, 98, 3, 218, 73, 70, 36, 217, 75, 100, 119, 81, 42, 73, 241, 179, 96, 80, 129, 84, 37, 233, 170, 70, 190, 92, 23, 82, 78, 31, 78, 196, 35, 153, 66, 240, 183, 109, 1, 174, 184, 192, 254, 50, 36, 202, 90, 23, 92, 160, 192, 75, 84, 51, 206, 213, 49, 65, 141, 114, 96, 140, 235, 206, 22, 245, 14, 150, 101, 203, 223, 160, 249, 138, 189, 143, 202, 232, 70, 48, 107, 196, 88, 35, 31, 215, 220, 35, 202, 54, 65, 152, 10, 216, 40, 219, 44, 28, 98, 99, 188, 124, 255, 75, 112, 85, 138, 245, 52, 7, 184, 109, 140, 215, 238, 15, 240, 122, 25, 144, 231, 2, 124, 115, 104, 140, 183, 196, 194, 30, 117, 104, 123, 172, 167, 206, 195, 163, 124, 235, 25, 58, 138, 224, 61, 0, 124, 128, 59, 220, 29, 229, 117, 152, 223, 46, 243, 158, 75, 180, 37, 116, 137, 218, 37, 192, 234, 202, 251, 17, 99, 136, 164, 234, 112, 158, 119, 132, 220, 144, 234, 17, 78, 97, 79, 40, 192, 93, 56, 231, 101, 148, 175, 126, 240, 172, 243, 244, 156, 161, 96, 115, 158, 35, 177, 158, 194, 105, 200, 99, 33, 57, 109, 226, 7, 72, 105, 103, 38, 190, 31, 122, 149, 60, 103, 113, 240, 93, 1, 238, 233, 207, 243, 222, 211, 20, 22, 69, 117, 118, 203, 81, 145, 0, 247, 34, 33, 157, 227, 187, 46, 81, 85, 128, 239, 150, 212, 227, 13, 240, 193, 176, 167, 169, 89, 226, 244, 149, 0, 223, 227, 158, 143, 40, 198, 217, 247, 118, 171, 173, 203, 132, 54, 212, 31, 224, 1, 151, 216, 221, 130, 44, 209, 28, 58, 199, 137, 75, 180, 85, 214, 3, 60, 136, 213, 49, 78, 9, 238, 69, 182, 242, 124, 36, 207, 239, 42, 108, 109, 25, 231, 227, 146, 79, 145, 41, 222, 155, 231, 247, 225, 62, 45, 121, 126, 63, 6, 36, 212, 179, 227, 124, 2, 212, 31, 26, 229, 143, 92, 156, 180, 146, 77, 254, 203, 116, 171, 198, 111, 159, 160, 61, 226, 43, 248, 235, 19, 215, 214, 184, 157, 232, 103, 84, 190, 228, 50, 80, 220, 142, 178, 215, 34, 5, 238, 167, 26, 141, 97, 89, 173, 94, 22, 196, 102, 241, 40, 143, 120, 147, 139, 156, 64, 209, 240, 184, 81, 86, 68, 172, 166, 121, 110, 77, 84, 33, 243, 58, 45, 83, 46, 113, 43, 106, 120, 45, 92, 98, 13, 92, 226, 54, 120, 218, 189, 224, 114, 63, 220, 225, 81, 56, 196, 147, 160, 30, 67, 245, 255, 54, 124, 242, 135, 112, 158, 55, 169, 138, 209, 41, 113, 23, 220, 169, 151, 22, 176, 1, 218, 26, 249, 8, 91, 240, 212, 167, 201, 167, 234, 218, 41, 41, 140, 202, 59, 31, 86, 222, 137, 123, 119, 43, 240, 163, 69, 48, 162, 192, 143, 23, 193, 93, 10, 252, 100, 17, 236, 81, 224, 201, 34, 24, 246, 40, 248, 209, 73, 184, 188, 218, 251, 101, 210, 98, 56, 228, 96, 184, 98, 94, 128, 63, 29, 133, 159, 157, 142, 150, 135, 162, 21, 117, 158, 40, 18, 124, 212, 219, 20, 173, 174, 104, 142, 86, 107, 45, 209, 0, 63, 94, 87, 49, 198, 79, 189, 160, 106, 121, 25, 174, 168, 126, 249, 57, 183, 136, 34, 67, 205, 227, 27, 56, 200, 245, 255, 11, 80, 75, 7, 8, 221, 183, 102, 60, 240, 15, 0, 0, 225, 30, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 71, 114, 97, 100, 108, 101, 85, 115, 101, 114, 72, 111, 109, 101, 76, 111, 111, 107, 117, 112, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 82, 93, 79, 19, 65, 20, 61, 67, 129, 221, 126, 160, 88, 81, 80, 84, 116, 85, 40, 9, 219, 141, 162, 73, 131, 196, 4, 165, 192, 67, 13, 166, 181, 38, 62, 53, 195, 238, 237, 118, 233, 126, 101, 118, 183, 134, 24, 249, 33, 254, 11, 99, 130, 70, 19, 127, 128, 63, 202, 120, 183, 104, 140, 226, 131, 47, 51, 115, 207, 156, 115, 239, 185, 119, 230, 219, 247, 207, 95, 1, 60, 192, 178, 192, 187, 227, 227, 118, 227, 141, 113, 32, 237, 33, 133, 142, 177, 97, 216, 125, 99, 205, 176, 163, 32, 246, 124, 153, 122, 81, 104, 6, 145, 67, 140, 43, 242, 73, 38, 196, 151, 3, 153, 152, 246, 128, 236, 97, 146, 5, 137, 177, 209, 151, 126, 66, 107, 70, 236, 154, 129, 140, 77, 47, 207, 65, 116, 255, 225, 186, 108, 48, 87, 53, 126, 233, 251, 153, 239, 51, 144, 12, 164, 121, 47, 167, 132, 174, 23, 18, 41, 47, 116, 25, 29, 145, 74, 184, 22, 227, 141, 250, 122, 189, 97, 58, 52, 50, 222, 234, 16, 2, 165, 78, 148, 41, 155, 118, 60, 159, 4, 86, 34, 229, 90, 174, 146, 142, 79, 214, 107, 37, 227, 152, 148, 181, 59, 14, 187, 9, 169, 189, 40, 160, 86, 20, 13, 179, 88, 195, 164, 192, 236, 161, 28, 73, 203, 151, 161, 107, 237, 31, 28, 146, 157, 106, 152, 22, 88, 216, 110, 238, 108, 117, 91, 47, 122, 187, 237, 173, 237, 86, 179, 215, 237, 52, 219, 189, 189, 253, 103, 77, 129, 106, 235, 183, 162, 147, 230, 206, 30, 9, 232, 155, 182, 239, 133, 94, 250, 88, 160, 80, 91, 125, 41, 48, 255, 55, 233, 73, 230, 249, 14, 41, 13, 21, 129, 233, 205, 49, 183, 130, 115, 40, 151, 48, 131, 243, 2, 197, 140, 173, 213, 7, 236, 77, 199, 133, 63, 92, 117, 142, 146, 148, 2, 13, 23, 5, 202, 46, 165, 207, 85, 196, 253, 164, 71, 2, 203, 181, 179, 78, 86, 207, 66, 21, 92, 194, 229, 18, 230, 48, 207, 133, 243, 97, 132, 142, 128, 249, 95, 218, 159, 158, 57, 197, 21, 92, 205, 141, 46, 114, 167, 86, 253, 116, 180, 58, 174, 115, 148, 70, 167, 84, 129, 185, 218, 63, 139, 47, 225, 102, 174, 188, 85, 129, 142, 98, 17, 83, 184, 45, 48, 249, 148, 31, 155, 103, 63, 5, 141, 63, 152, 224, 236, 124, 55, 62, 233, 40, 161, 204, 251, 93, 142, 86, 48, 193, 39, 96, 241, 11, 102, 94, 125, 196, 108, 181, 250, 9, 11, 39, 184, 86, 189, 193, 203, 9, 140, 15, 184, 243, 30, 24, 203, 10, 188, 78, 160, 240, 3, 80, 75, 7, 8, 138, 125, 101, 29, 216, 1, 0, 0, 178, 2, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 71, 114, 97, 100, 108, 101, 87, 114, 97, 112, 112, 101, 114, 77, 97, 105, 110, 36, 65, 99, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 53, 142, 193, 78, 195, 48, 12, 134, 29, 10, 116, 236, 196, 51, 68, 28, 64, 44, 171, 96, 32, 85, 187, 113, 129, 19, 66, 2, 9, 206, 94, 234, 166, 217, 210, 180, 74, 218, 130, 132, 216, 131, 240, 40, 28, 120, 0, 30, 10, 225, 130, 240, 193, 178, 63, 255, 191, 237, 175, 239, 143, 79, 0, 184, 128, 169, 128, 247, 237, 246, 62, 127, 149, 43, 212, 27, 242, 133, 92, 74, 93, 202, 153, 212, 77, 221, 90, 135, 157, 109, 188, 170, 155, 130, 152, 7, 114, 132, 145, 120, 88, 97, 84, 186, 34, 189, 137, 125, 29, 229, 178, 68, 23, 105, 38, 91, 163, 106, 108, 149, 29, 119, 16, 157, 95, 46, 48, 103, 109, 200, 255, 253, 101, 239, 28, 131, 88, 161, 58, 27, 37, 222, 88, 79, 20, 172, 55, 76, 7, 10, 145, 111, 49, 207, 231, 139, 121, 174, 10, 26, 228, 219, 4, 132, 128, 233, 67, 211, 7, 77, 215, 214, 145, 128, 211, 38, 152, 204, 4, 44, 28, 101, 207, 1, 219, 150, 66, 118, 243, 219, 62, 253, 117, 183, 104, 253, 209, 149, 30, 255, 78, 97, 87, 192, 225, 26, 7, 204, 28, 122, 147, 221, 173, 214, 164, 187, 20, 246, 5, 164, 244, 66, 186, 239, 120, 97, 114, 124, 242, 200, 96, 15, 82, 24, 67, 176, 101, 2, 7, 99, 5, 9, 231, 29, 72, 126, 0, 80, 75, 7, 8, 43, 212, 98, 82, 247, 0, 0, 0, 44, 1, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 71, 114, 97, 100, 108, 101, 87, 114, 97, 112, 112, 101, 114, 77, 97, 105, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 205, 89, 121, 124, 84, 245, 181, 63, 103, 182, 59, 185, 25, 5, 39, 4, 24, 22, 25, 67, 148, 201, 50, 137, 162, 34, 38, 34, 6, 18, 32, 50, 9, 200, 0, 113, 216, 226, 205, 204, 77, 50, 50, 51, 55, 206, 18, 136, 11, 85, 171, 182, 246, 89, 245, 181, 182, 53, 214, 250, 44, 181, 141, 85, 186, 104, 97, 38, 152, 10, 82, 91, 170, 86, 187, 104, 55, 171, 125, 118, 179, 171, 221, 55, 173, 149, 244, 251, 187, 119, 38, 153, 73, 38, 1, 223, 95, 143, 79, 72, 238, 253, 253, 206, 57, 191, 179, 47, 191, 251, 220, 137, 39, 142, 16, 209, 5, 166, 211, 152, 238, 219, 183, 111, 211, 242, 235, 42, 186, 148, 224, 110, 53, 22, 170, 104, 168, 8, 118, 87, 212, 86, 4, 181, 104, 95, 56, 162, 36, 195, 90, 204, 27, 213, 66, 42, 214, 227, 106, 68, 85, 18, 42, 54, 123, 149, 132, 55, 216, 171, 6, 119, 39, 82, 209, 68, 69, 67, 183, 18, 73, 168, 181, 21, 125, 61, 222, 168, 210, 231, 13, 11, 26, 170, 186, 244, 194, 243, 149, 229, 128, 141, 47, 207, 225, 119, 167, 34, 17, 44, 36, 122, 21, 239, 121, 2, 36, 214, 19, 142, 169, 106, 60, 28, 235, 193, 106, 191, 26, 79, 224, 44, 172, 47, 175, 59, 191, 110, 185, 55, 164, 246, 87, 220, 96, 39, 102, 146, 253, 90, 42, 30, 84, 215, 132, 35, 42, 83, 165, 22, 239, 169, 239, 137, 43, 161, 136, 90, 191, 39, 174, 244, 245, 169, 241, 250, 181, 250, 107, 135, 241, 214, 166, 132, 99, 18, 89, 152, 102, 94, 173, 244, 43, 245, 17, 37, 214, 83, 191, 161, 235, 106, 53, 152, 148, 200, 198, 100, 137, 98, 159, 105, 182, 103, 187, 111, 124, 223, 159, 20, 76, 52, 86, 109, 101, 154, 49, 190, 186, 58, 162, 36, 18, 18, 201, 76, 101, 61, 106, 114, 99, 92, 75, 130, 8, 88, 108, 214, 12, 26, 21, 158, 42, 131, 70, 66, 13, 166, 226, 225, 228, 64, 253, 68, 152, 70, 7, 57, 232, 52, 153, 74, 233, 116, 166, 51, 167, 135, 149, 104, 38, 211, 105, 56, 104, 53, 148, 101, 72, 204, 180, 96, 210, 17, 227, 187, 32, 238, 164, 50, 153, 206, 160, 89, 76, 115, 167, 130, 146, 104, 54, 83, 41, 200, 250, 180, 160, 110, 76, 40, 38, 71, 52, 166, 38, 235, 183, 108, 242, 129, 208, 92, 114, 201, 52, 135, 230, 49, 57, 242, 119, 36, 90, 192, 100, 77, 106, 91, 54, 181, 78, 66, 107, 5, 218, 153, 180, 72, 166, 133, 228, 46, 68, 107, 149, 168, 130, 169, 4, 103, 250, 225, 35, 81, 136, 49, 43, 135, 154, 175, 109, 7, 85, 210, 217, 50, 45, 166, 115, 96, 149, 110, 24, 215, 78, 158, 2, 171, 25, 112, 18, 85, 51, 217, 212, 107, 82, 240, 49, 166, 114, 143, 111, 162, 89, 27, 171, 182, 57, 168, 150, 188, 50, 213, 80, 29, 108, 101, 112, 18, 214, 234, 5, 205, 250, 141, 74, 178, 23, 70, 60, 151, 201, 12, 134, 224, 63, 158, 66, 33, 114, 50, 229, 195, 131, 181, 165, 116, 190, 76, 231, 209, 5, 76, 206, 201, 251, 18, 45, 3, 75, 73, 205, 240, 200, 49, 189, 0, 68, 172, 0, 123, 57, 93, 92, 74, 23, 81, 3, 184, 41, 226, 102, 18, 93, 146, 243, 6, 125, 185, 93, 211, 61, 173, 89, 237, 94, 163, 165, 98, 161, 150, 120, 92, 139, 75, 116, 105, 78, 169, 89, 178, 18, 93, 198, 36, 9, 71, 4, 7, 14, 90, 101, 232, 110, 53, 24, 185, 36, 28, 11, 39, 47, 157, 160, 155, 49, 151, 118, 80, 11, 173, 145, 169, 137, 214, 50, 157, 222, 23, 87, 251, 148, 120, 46, 84, 152, 54, 22, 11, 131, 66, 105, 170, 124, 167, 18, 113, 149, 77, 186, 43, 67, 246, 86, 186, 92, 38, 43, 173, 103, 170, 121, 23, 136, 18, 181, 65, 58, 117, 47, 28, 56, 9, 157, 154, 61, 130, 241, 13, 180, 177, 148, 218, 233, 10, 166, 121, 227, 76, 110, 74, 197, 146, 225, 168, 218, 178, 55, 168, 246, 25, 152, 126, 166, 75, 87, 43, 177, 152, 150, 116, 135, 212, 164, 26, 143, 34, 171, 184, 131, 66, 165, 125, 80, 149, 187, 91, 139, 187, 179, 199, 187, 47, 87, 226, 238, 238, 184, 22, 117, 7, 17, 32, 93, 200, 101, 238, 37, 103, 39, 150, 212, 217, 105, 11, 244, 8, 192, 168, 2, 23, 185, 184, 136, 30, 183, 23, 113, 187, 98, 46, 221, 65, 87, 10, 63, 12, 200, 180, 89, 40, 124, 126, 190, 175, 249, 7, 98, 73, 101, 111, 30, 227, 59, 16, 183, 249, 103, 109, 238, 141, 107, 123, 148, 46, 161, 116, 221, 108, 187, 4, 149, 78, 35, 148, 54, 194, 108, 177, 164, 131, 20, 97, 247, 38, 234, 50, 124, 161, 93, 137, 170, 14, 10, 25, 107, 208, 155, 109, 71, 221, 213, 74, 188, 210, 78, 61, 8, 250, 186, 190, 184, 6, 161, 147, 97, 53, 97, 167, 48, 220, 9, 198, 143, 40, 34, 145, 198, 19, 16, 115, 121, 17, 49, 139, 56, 80, 49, 41, 119, 83, 68, 72, 25, 101, 90, 114, 106, 68, 116, 113, 52, 193, 100, 159, 145, 228, 12, 113, 132, 131, 57, 40, 78, 23, 139, 29, 68, 183, 59, 207, 99, 130, 145, 48, 210, 88, 52, 170, 196, 66, 62, 24, 20, 8, 9, 21, 97, 145, 18, 148, 54, 202, 212, 79, 123, 16, 93, 74, 36, 162, 237, 217, 18, 219, 29, 211, 246, 196, 54, 232, 90, 5, 21, 70, 62, 24, 160, 107, 75, 0, 115, 29, 222, 122, 236, 116, 3, 162, 212, 32, 235, 77, 129, 138, 183, 87, 139, 34, 219, 188, 7, 250, 210, 250, 140, 156, 120, 81, 209, 122, 224, 155, 154, 31, 227, 52, 232, 226, 38, 186, 89, 112, 243, 222, 105, 185, 223, 144, 181, 248, 173, 176, 130, 18, 239, 73, 69, 33, 252, 230, 129, 62, 88, 236, 140, 188, 99, 245, 52, 0, 146, 239, 163, 247, 151, 208, 109, 116, 59, 152, 191, 198, 78, 255, 133, 20, 124, 77, 42, 172, 38, 237, 244, 65, 172, 52, 219, 233, 46, 24, 55, 49, 144, 72, 170, 81, 175, 48, 177, 157, 254, 27, 161, 173, 147, 73, 37, 195, 145, 122, 95, 56, 129, 90, 247, 97, 166, 70, 191, 154, 76, 184, 21, 247, 229, 91, 219, 220, 6, 188, 59, 235, 18, 3, 110, 143, 8, 11, 117, 175, 18, 237, 139, 168, 181, 110, 111, 115, 116, 64, 108, 173, 136, 14, 244, 43, 145, 148, 90, 133, 144, 248, 8, 78, 9, 169, 137, 96, 60, 156, 213, 145, 179, 152, 43, 124, 140, 238, 21, 188, 14, 230, 50, 183, 206, 65, 83, 60, 174, 12, 32, 235, 126, 28, 26, 86, 18, 130, 31, 166, 179, 61, 83, 199, 208, 24, 219, 32, 248, 9, 122, 64, 166, 251, 233, 127, 38, 235, 83, 119, 129, 80, 158, 86, 37, 250, 36, 20, 56, 78, 97, 157, 146, 232, 133, 200, 18, 125, 10, 117, 220, 176, 108, 98, 213, 128, 193, 42, 156, 197, 87, 8, 217, 166, 244, 225, 184, 79, 211, 103, 132, 163, 12, 77, 34, 132, 109, 137, 62, 11, 1, 116, 133, 36, 68, 168, 230, 115, 187, 90, 139, 68, 212, 92, 206, 123, 148, 14, 200, 244, 8, 125, 142, 201, 229, 41, 14, 99, 68, 192, 23, 100, 122, 136, 190, 40, 186, 15, 223, 36, 174, 179, 32, 143, 203, 180, 159, 190, 196, 180, 236, 100, 161, 80, 185, 74, 133, 9, 141, 96, 246, 167, 186, 178, 251, 18, 29, 98, 90, 235, 153, 198, 113, 13, 236, 198, 137, 16, 147, 148, 155, 229, 39, 35, 83, 154, 134, 115, 101, 80, 183, 93, 43, 114, 172, 200, 82, 18, 61, 193, 100, 15, 139, 183, 164, 22, 23, 82, 229, 43, 168, 53, 187, 14, 245, 124, 153, 158, 44, 165, 17, 58, 194, 84, 123, 82, 169, 140, 63, 254, 164, 146, 4, 253, 167, 114, 231, 22, 80, 148, 232, 43, 200, 126, 104, 65, 219, 213, 189, 73, 189, 86, 32, 224, 191, 74, 95, 43, 165, 167, 233, 56, 122, 137, 152, 190, 92, 216, 115, 100, 253, 205, 65, 207, 208, 179, 2, 238, 57, 248, 107, 84, 25, 232, 82, 113, 82, 60, 185, 33, 235, 225, 69, 43, 40, 136, 63, 79, 47, 200, 116, 140, 190, 201, 100, 242, 122, 237, 244, 109, 38, 239, 73, 5, 105, 234, 6, 187, 217, 156, 36, 209, 139, 34, 0, 78, 85, 231, 223, 149, 233, 37, 250, 30, 100, 244, 122, 183, 239, 90, 177, 179, 198, 78, 63, 192, 11, 106, 19, 218, 169, 132, 131, 94, 22, 220, 212, 208, 143, 80, 24, 18, 169, 174, 68, 214, 189, 203, 61, 173, 69, 115, 245, 171, 244, 99, 1, 253, 191, 200, 187, 90, 172, 64, 218, 109, 167, 88, 0, 78, 42, 170, 65, 48, 207, 114, 56, 246, 39, 244, 83, 161, 178, 159, 49, 45, 125, 247, 248, 18, 253, 2, 25, 45, 203, 174, 176, 114, 83, 28, 18, 158, 235, 57, 5, 94, 10, 185, 248, 37, 253, 74, 166, 215, 233, 215, 160, 230, 89, 153, 168, 202, 234, 115, 69, 93, 181, 157, 126, 11, 149, 134, 99, 33, 117, 239, 134, 110, 248, 12, 148, 215, 234, 160, 55, 232, 247, 66, 87, 127, 16, 190, 220, 58, 149, 58, 255, 36, 64, 254, 140, 177, 68, 139, 53, 101, 83, 57, 211, 170, 98, 158, 243, 110, 185, 253, 43, 253, 77, 112, 251, 119, 20, 9, 157, 91, 193, 172, 206, 235, 63, 145, 189, 42, 81, 27, 68, 49, 135, 59, 53, 135, 19, 34, 254, 66, 14, 122, 203, 40, 115, 255, 66, 170, 206, 98, 120, 119, 10, 132, 127, 231, 38, 2, 157, 159, 166, 28, 102, 182, 181, 60, 33, 211, 168, 40, 159, 165, 65, 13, 29, 73, 56, 150, 88, 175, 14, 56, 48, 104, 161, 133, 126, 132, 77, 76, 115, 38, 138, 178, 42, 21, 142, 132, 80, 125, 25, 99, 21, 123, 237, 108, 147, 217, 42, 218, 27, 155, 232, 170, 98, 33, 96, 22, 21, 127, 10, 50, 141, 14, 46, 97, 25, 36, 184, 20, 249, 35, 169, 25, 155, 14, 62, 13, 109, 12, 22, 49, 42, 157, 142, 14, 1, 105, 49, 167, 94, 7, 207, 164, 175, 65, 51, 124, 134, 174, 117, 168, 17, 69, 43, 169, 58, 184, 76, 55, 47, 99, 254, 177, 69, 48, 77, 38, 123, 245, 108, 208, 234, 224, 217, 60, 7, 86, 226, 185, 144, 81, 139, 181, 107, 217, 222, 192, 193, 243, 132, 134, 143, 241, 124, 68, 191, 78, 39, 103, 142, 150, 88, 200, 193, 11, 69, 95, 113, 140, 207, 20, 169, 28, 218, 185, 228, 84, 76, 58, 41, 132, 199, 251, 2, 62, 11, 169, 156, 49, 9, 157, 115, 106, 56, 18, 87, 34, 138, 125, 19, 170, 104, 182, 42, 62, 202, 231, 148, 240, 217, 188, 36, 55, 223, 20, 2, 72, 92, 37, 115, 181, 72, 176, 188, 194, 206, 181, 83, 36, 50, 225, 224, 92, 39, 244, 82, 207, 68, 118, 62, 15, 218, 234, 75, 77, 236, 4, 179, 153, 114, 250, 126, 55, 151, 78, 249, 124, 190, 64, 56, 205, 133, 40, 159, 134, 132, 249, 77, 39, 95, 196, 180, 216, 83, 56, 78, 76, 209, 28, 178, 104, 1, 25, 3, 147, 167, 200, 208, 176, 113, 140, 164, 32, 177, 14, 90, 139, 8, 103, 188, 196, 24, 204, 253, 122, 103, 51, 14, 195, 180, 104, 194, 153, 249, 101, 73, 175, 250, 124, 41, 175, 148, 121, 5, 99, 162, 114, 122, 38, 108, 26, 236, 172, 18, 82, 137, 225, 10, 10, 106, 138, 68, 28, 156, 93, 90, 51, 214, 73, 214, 137, 78, 178, 78, 239, 36, 121, 29, 140, 92, 108, 48, 45, 86, 128, 150, 242, 229, 130, 208, 250, 194, 81, 87, 151, 65, 226, 54, 209, 217, 233, 119, 13, 122, 151, 230, 224, 13, 232, 180, 185, 157, 55, 34, 123, 23, 176, 41, 241, 38, 99, 156, 75, 32, 62, 198, 58, 223, 137, 45, 142, 104, 43, 28, 188, 153, 183, 148, 192, 19, 183, 34, 216, 114, 1, 239, 224, 43, 69, 180, 63, 196, 1, 145, 5, 19, 45, 209, 62, 113, 216, 118, 68, 25, 87, 51, 198, 18, 75, 34, 124, 45, 226, 107, 23, 194, 8, 11, 157, 147, 170, 75, 158, 52, 138, 128, 192, 56, 178, 40, 175, 65, 64, 219, 211, 163, 68, 244, 180, 54, 62, 238, 48, 242, 196, 124, 131, 85, 55, 138, 183, 59, 154, 138, 36, 195, 8, 100, 183, 209, 96, 213, 217, 185, 91, 102, 85, 164, 149, 69, 89, 168, 144, 166, 38, 220, 98, 172, 235, 85, 250, 85, 183, 18, 27, 48, 64, 1, 137, 105, 102, 201, 148, 211, 229, 22, 24, 102, 29, 236, 226, 211, 180, 221, 41, 104, 106, 55, 146, 97, 115, 203, 154, 166, 45, 190, 205, 157, 107, 55, 53, 53, 251, 90, 58, 183, 248, 91, 54, 117, 174, 219, 208, 214, 226, 224, 40, 26, 88, 142, 112, 204, 184, 37, 201, 105, 126, 130, 65, 167, 25, 134, 184, 143, 175, 17, 38, 66, 3, 52, 115, 34, 117, 59, 35, 186, 108, 32, 171, 198, 250, 29, 220, 111, 0, 34, 181, 216, 226, 106, 84, 235, 135, 130, 7, 12, 103, 184, 54, 59, 20, 229, 249, 112, 97, 183, 57, 190, 131, 3, 175, 231, 27, 4, 157, 125, 232, 113, 138, 65, 72, 124, 163, 204, 55, 9, 79, 117, 21, 81, 145, 79, 235, 233, 17, 193, 243, 94, 81, 244, 182, 25, 206, 126, 171, 204, 183, 240, 109, 76, 87, 116, 52, 109, 106, 111, 109, 95, 235, 110, 237, 137, 161, 181, 12, 185, 131, 169, 68, 18, 195, 178, 161, 86, 183, 112, 120, 183, 112, 120, 119, 36, 123, 151, 132, 49, 58, 214, 29, 238, 73, 9, 216, 112, 108, 18, 92, 131, 219, 206, 239, 71, 43, 14, 209, 154, 186, 18, 90, 4, 163, 189, 126, 121, 193, 31, 208, 7, 86, 198, 100, 99, 142, 104, 200, 252, 31, 164, 53, 130, 133, 59, 197, 69, 207, 94, 228, 51, 248, 231, 221, 34, 227, 55, 49, 38, 155, 138, 34, 82, 100, 47, 16, 90, 244, 251, 2, 148, 52, 254, 48, 68, 22, 73, 123, 73, 209, 116, 51, 81, 135, 134, 216, 31, 149, 249, 30, 254, 24, 206, 52, 164, 96, 42, 122, 213, 145, 61, 106, 117, 86, 82, 197, 72, 239, 60, 200, 247, 149, 0, 29, 19, 206, 252, 34, 72, 205, 152, 71, 35, 154, 18, 146, 248, 19, 197, 61, 181, 24, 85, 137, 49, 244, 156, 30, 83, 147, 123, 180, 248, 238, 205, 225, 168, 170, 137, 228, 204, 40, 103, 159, 228, 253, 37, 252, 32, 99, 172, 153, 3, 94, 251, 33, 197, 228, 164, 87, 227, 153, 66, 214, 98, 99, 15, 127, 154, 63, 35, 243, 3, 140, 177, 103, 169, 167, 152, 212, 134, 155, 52, 22, 193, 109, 53, 148, 247, 89, 129, 254, 200, 76, 66, 102, 154, 19, 81, 162, 93, 33, 165, 178, 240, 114, 169, 242, 92, 166, 59, 139, 210, 158, 96, 188, 198, 98, 87, 80, 211, 176, 52, 141, 186, 27, 139, 25, 127, 172, 10, 108, 117, 240, 231, 248, 243, 50, 89, 249, 11, 51, 108, 140, 49, 172, 98, 252, 220, 112, 172, 95, 219, 173, 214, 251, 116, 73, 218, 212, 164, 210, 173, 4, 193, 219, 128, 196, 143, 35, 49, 68, 199, 23, 152, 158, 207, 79, 12, 89, 60, 96, 244, 106, 33, 163, 50, 37, 42, 141, 196, 83, 164, 212, 77, 133, 40, 174, 2, 254, 111, 155, 198, 145, 211, 227, 86, 77, 222, 93, 173, 68, 34, 254, 176, 104, 58, 249, 32, 31, 146, 249, 75, 156, 134, 74, 50, 76, 79, 252, 191, 178, 216, 187, 188, 122, 220, 192, 135, 157, 68, 140, 169, 180, 54, 187, 239, 30, 239, 70, 220, 226, 250, 86, 191, 238, 27, 175, 43, 122, 178, 65, 57, 249, 50, 211, 188, 34, 71, 181, 198, 18, 73, 40, 74, 226, 35, 133, 151, 17, 99, 125, 9, 146, 153, 232, 173, 163, 93, 122, 63, 242, 212, 228, 206, 99, 146, 7, 34, 114, 190, 34, 243, 49, 126, 154, 105, 231, 180, 129, 55, 173, 206, 78, 198, 75, 246, 36, 84, 243, 163, 140, 137, 88, 14, 198, 85, 148, 226, 102, 253, 30, 230, 226, 233, 44, 92, 152, 229, 38, 222, 107, 243, 51, 252, 172, 32, 249, 156, 200, 219, 225, 46, 59, 63, 143, 216, 8, 99, 28, 137, 171, 122, 108, 56, 248, 155, 70, 214, 254, 22, 114, 105, 145, 67, 86, 105, 90, 18, 227, 170, 210, 39, 204, 166, 207, 118, 72, 19, 231, 73, 252, 29, 153, 95, 20, 201, 187, 36, 2, 14, 197, 81, 72, 103, 85, 133, 154, 140, 41, 81, 241, 1, 40, 41, 100, 219, 62, 145, 173, 239, 242, 247, 196, 169, 223, 207, 77, 46, 217, 175, 23, 250, 117, 154, 15, 10, 19, 198, 249, 161, 184, 98, 116, 103, 191, 102, 248, 28, 252, 138, 248, 226, 177, 152, 95, 69, 107, 51, 225, 2, 46, 135, 129, 137, 121, 214, 88, 123, 153, 183, 35, 46, 121, 242, 67, 42, 111, 11, 188, 252, 132, 127, 42, 243, 107, 252, 51, 7, 41, 198, 211, 47, 132, 44, 219, 11, 63, 184, 76, 129, 109, 88, 237, 151, 50, 191, 204, 191, 42, 104, 15, 55, 247, 194, 128, 40, 38, 191, 65, 159, 16, 76, 197, 197, 213, 169, 177, 52, 241, 154, 195, 88, 5, 27, 191, 227, 55, 100, 254, 45, 255, 30, 226, 37, 196, 39, 165, 88, 18, 83, 116, 129, 16, 243, 60, 83, 115, 193, 127, 228, 63, 9, 116, 76, 185, 103, 194, 142, 117, 217, 142, 55, 162, 164, 98, 193, 94, 116, 189, 70, 12, 10, 43, 218, 249, 175, 194, 110, 192, 212, 73, 232, 183, 28, 211, 182, 77, 217, 59, 78, 254, 59, 255, 67, 104, 231, 159, 118, 12, 34, 250, 5, 183, 145, 176, 152, 46, 59, 201, 237, 187, 65, 32, 159, 100, 92, 237, 22, 215, 108, 217, 148, 7, 218, 255, 226, 183, 101, 42, 229, 194, 33, 184, 16, 74, 226, 19, 40, 252, 70, 54, 156, 112, 227, 159, 237, 109, 79, 114, 227, 159, 235, 128, 77, 100, 98, 153, 71, 77, 24, 155, 173, 193, 136, 150, 80, 29, 38, 11, 230, 72, 126, 217, 100, 101, 106, 89, 173, 165, 34, 33, 61, 213, 232, 45, 147, 234, 78, 246, 170, 185, 54, 41, 167, 77, 247, 229, 77, 155, 242, 186, 167, 80, 88, 220, 232, 116, 165, 244, 254, 202, 248, 58, 97, 146, 208, 172, 137, 79, 122, 48, 63, 194, 38, 184, 27, 117, 120, 179, 184, 1, 64, 111, 37, 230, 199, 246, 84, 180, 75, 141, 103, 87, 102, 142, 71, 153, 46, 105, 130, 192, 25, 73, 36, 254, 153, 169, 132, 236, 248, 207, 166, 18, 188, 189, 128, 117, 51, 254, 6, 157, 214, 52, 205, 72, 83, 121, 154, 230, 167, 233, 172, 128, 47, 77, 75, 156, 85, 105, 170, 31, 164, 234, 154, 97, 186, 240, 48, 53, 50, 249, 134, 200, 217, 49, 66, 77, 129, 182, 154, 52, 53, 103, 104, 93, 173, 79, 108, 250, 14, 19, 6, 145, 199, 70, 104, 115, 192, 242, 36, 73, 129, 245, 230, 26, 191, 115, 107, 245, 48, 109, 203, 208, 246, 163, 235, 197, 122, 117, 134, 174, 58, 74, 38, 28, 116, 58, 5, 104, 39, 205, 167, 74, 242, 208, 74, 50, 153, 100, 241, 5, 154, 44, 163, 120, 53, 73, 180, 66, 162, 197, 196, 18, 173, 28, 5, 160, 241, 222, 68, 52, 74, 50, 112, 197, 22, 30, 103, 3, 1, 16, 59, 77, 165, 58, 53, 193, 60, 140, 135, 253, 85, 120, 150, 197, 167, 39, 67, 52, 201, 5, 33, 65, 221, 114, 160, 186, 38, 199, 115, 16, 255, 187, 157, 189, 206, 171, 211, 20, 203, 208, 53, 105, 74, 26, 63, 190, 17, 234, 15, 236, 216, 209, 158, 161, 189, 150, 67, 116, 189, 245, 73, 170, 9, 4, 204, 206, 125, 126, 139, 243, 70, 127, 154, 110, 113, 214, 28, 162, 15, 228, 86, 239, 192, 234, 157, 98, 181, 35, 183, 114, 55, 86, 62, 36, 86, 2, 206, 123, 0, 232, 252, 232, 33, 186, 111, 152, 30, 28, 161, 253, 129, 6, 203, 8, 61, 20, 240, 30, 164, 135, 211, 244, 249, 12, 61, 150, 161, 131, 35, 148, 14, 52, 88, 189, 46, 75, 134, 14, 31, 166, 163, 76, 13, 54, 151, 237, 48, 125, 157, 105, 144, 159, 117, 89, 197, 243, 55, 152, 158, 2, 233, 6, 41, 77, 223, 26, 228, 135, 93, 146, 243, 59, 186, 41, 206, 24, 161, 151, 128, 43, 80, 191, 63, 52, 250, 60, 214, 127, 152, 166, 87, 6, 201, 5, 52, 9, 6, 124, 205, 37, 117, 166, 233, 231, 105, 250, 77, 131, 117, 104, 244, 0, 246, 127, 167, 239, 215, 137, 253, 153, 43, 210, 244, 199, 101, 2, 176, 28, 160, 127, 49, 64, 203, 173, 150, 171, 244, 167, 215, 210, 244, 15, 129, 180, 7, 72, 111, 234, 72, 110, 129, 100, 25, 7, 117, 73, 182, 113, 176, 141, 143, 211, 219, 247, 209, 92, 0, 191, 163, 3, 219, 134, 168, 116, 132, 70, 3, 104, 31, 142, 122, 129, 6, 200, 6, 59, 164, 118, 217, 211, 108, 30, 164, 89, 130, 22, 158, 199, 120, 155, 233, 205, 210, 110, 40, 17, 80, 37, 58, 212, 173, 46, 235, 8, 91, 3, 174, 146, 206, 50, 150, 50, 12, 84, 71, 154, 103, 96, 147, 126, 14, 169, 211, 236, 28, 164, 178, 156, 164, 6, 31, 39, 158, 113, 97, 189, 188, 193, 106, 93, 102, 47, 183, 187, 164, 52, 187, 246, 159, 56, 232, 178, 150, 219, 45, 87, 9, 81, 203, 237, 186, 172, 13, 118, 157, 174, 189, 144, 174, 206, 14, 144, 1, 227, 178, 55, 128, 194, 208, 232, 17, 88, 234, 250, 2, 118, 5, 241, 161, 19, 202, 212, 156, 101, 33, 86, 10, 148, 52, 47, 16, 207, 231, 187, 44, 130, 173, 69, 35, 244, 72, 160, 61, 195, 110, 231, 221, 105, 94, 124, 144, 61, 105, 174, 129, 241, 172, 134, 173, 105, 173, 120, 202, 89, 218, 86, 198, 222, 52, 159, 27, 88, 38, 221, 79, 208, 13, 94, 151, 166, 121, 89, 199, 208, 232, 75, 120, 41, 151, 92, 54, 115, 185, 36, 4, 177, 89, 174, 18, 102, 182, 117, 26, 187, 135, 132, 103, 215, 148, 241, 242, 12, 55, 14, 115, 83, 131, 85, 28, 25, 240, 118, 186, 172, 25, 110, 78, 243, 218, 50, 110, 77, 179, 207, 56, 225, 56, 57, 240, 234, 178, 13, 243, 21, 29, 46, 75, 192, 185, 15, 107, 224, 171, 227, 32, 119, 184, 240, 176, 109, 144, 252, 46, 120, 107, 83, 131, 205, 185, 15, 235, 240, 62, 157, 229, 157, 150, 155, 133, 3, 25, 47, 87, 89, 62, 69, 115, 92, 54, 253, 205, 156, 230, 32, 40, 35, 13, 12, 145, 58, 194, 106, 160, 140, 123, 50, 220, 123, 212, 120, 188, 90, 60, 62, 206, 90, 7, 206, 28, 230, 132, 126, 126, 185, 224, 182, 1, 194, 0, 199, 37, 53, 192, 103, 188, 101, 156, 26, 230, 189, 83, 236, 206, 208, 87, 108, 160, 130, 165, 220, 174, 33, 172, 46, 103, 3, 44, 14, 169, 33, 170, 46, 231, 117, 199, 73, 178, 12, 145, 197, 44, 54, 27, 74, 92, 118, 87, 73, 78, 15, 250, 10, 98, 206, 234, 42, 49, 22, 134, 249, 61, 194, 68, 55, 143, 240, 45, 129, 118, 161, 141, 59, 16, 170, 57, 109, 224, 97, 91, 134, 223, 55, 72, 11, 189, 186, 213, 37, 88, 253, 118, 221, 234, 119, 24, 134, 79, 243, 93, 181, 105, 254, 208, 32, 53, 142, 240, 61, 64, 172, 70, 52, 7, 106, 71, 248, 38, 248, 255, 71, 50, 124, 239, 65, 190, 127, 253, 8, 63, 16, 104, 243, 86, 31, 228, 135, 112, 214, 48, 63, 220, 153, 225, 71, 107, 93, 182, 154, 39, 120, 132, 232, 139, 121, 201, 178, 214, 95, 198, 79, 230, 178, 37, 178, 153, 158, 19, 249, 46, 90, 48, 74, 251, 73, 50, 114, 160, 254, 211, 47, 209, 126, 137, 142, 73, 244, 52, 209, 191, 105, 153, 68, 53, 139, 46, 56, 103, 148, 156, 34, 171, 143, 34, 61, 218, 139, 194, 2, 76, 128, 87, 226, 239, 40, 173, 45, 2, 244, 122, 14, 104, 20, 9, 179, 100, 42, 26, 44, 246, 155, 73, 158, 106, 223, 248, 17, 64, 115, 166, 0, 66, 74, 54, 0, 206, 154, 142, 211, 81, 42, 37, 219, 216, 54, 25, 155, 98, 121, 126, 222, 178, 68, 143, 232, 27, 160, 249, 14, 213, 11, 238, 222, 164, 133, 66, 37, 236, 31, 165, 141, 66, 31, 178, 168, 25, 147, 16, 216, 79, 116, 22, 0, 75, 177, 244, 14, 85, 25, 27, 151, 241, 40, 5, 243, 21, 205, 183, 8, 86, 245, 186, 115, 169, 168, 47, 250, 179, 94, 110, 142, 211, 121, 40, 52, 104, 254, 240, 123, 49, 182, 234, 200, 66, 75, 81, 65, 87, 145, 141, 214, 1, 110, 27, 42, 235, 110, 104, 177, 15, 135, 199, 33, 72, 138, 28, 180, 151, 78, 163, 123, 81, 168, 62, 78, 51, 232, 37, 154, 73, 175, 208, 25, 166, 1, 114, 154, 110, 162, 50, 211, 109, 52, 203, 116, 59, 149, 155, 94, 165, 217, 166, 55, 104, 142, 233, 109, 154, 107, 158, 77, 46, 243, 124, 154, 103, 94, 68, 243, 205, 181, 180, 192, 124, 49, 45, 52, 175, 161, 51, 205, 187, 104, 145, 57, 68, 110, 243, 181, 116, 150, 121, 31, 85, 152, 143, 211, 98, 243, 159, 169, 210, 50, 139, 206, 182, 44, 160, 115, 44, 85, 180, 196, 226, 37, 143, 229, 66, 170, 178, 92, 66, 213, 150, 173, 84, 99, 137, 83, 173, 165, 159, 188, 150, 27, 168, 206, 114, 35, 213, 91, 238, 164, 115, 45, 247, 64, 6, 89, 12, 193, 70, 153, 100, 59, 248, 182, 9, 217, 70, 248, 104, 160, 186, 19, 94, 124, 44, 176, 94, 36, 176, 12, 127, 213, 91, 157, 225, 175, 195, 153, 211, 252, 141, 245, 34, 254, 2, 109, 213, 101, 252, 2, 194, 79, 119, 254, 42, 196, 192, 183, 7, 169, 2, 40, 47, 6, 218, 50, 252, 18, 222, 127, 16, 104, 59, 78, 51, 107, 143, 88, 30, 36, 185, 214, 188, 180, 109, 8, 147, 118, 91, 237, 113, 242, 143, 240, 203, 129, 29, 235, 1, 241, 163, 52, 255, 184, 13, 174, 191, 48, 0, 207, 31, 102, 228, 207, 215, 51, 252, 235, 97, 254, 67, 117, 154, 255, 82, 198, 127, 75, 243, 155, 216, 45, 69, 96, 56, 87, 32, 50, 222, 194, 242, 59, 99, 109, 5, 87, 167, 77, 230, 142, 180, 201, 54, 222, 109, 248, 204, 213, 254, 50, 147, 189, 102, 66, 0, 209, 92, 50, 143, 210, 149, 162, 137, 128, 81, 87, 136, 16, 64, 215, 208, 244, 22, 249, 117, 75, 58, 97, 185, 185, 176, 228, 124, 88, 114, 17, 44, 185, 11, 239, 38, 189, 153, 192, 111, 147, 3, 127, 29, 196, 60, 76, 102, 62, 192, 143, 241, 129, 255, 0, 80, 75, 7, 8, 175, 160, 39, 103, 152, 18, 0, 0, 43, 39, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 73, 110, 115, 116, 97, 108, 108, 36, 73, 110, 115, 116, 97, 108, 108, 67, 104, 101, 99, 107, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 101, 145, 219, 74, 3, 49, 16, 134, 255, 88, 181, 90, 87, 107, 235, 225, 198, 187, 85, 240, 212, 117, 241, 4, 139, 138, 55, 130, 40, 40, 130, 130, 224, 101, 220, 78, 183, 209, 236, 129, 100, 91, 47, 196, 62, 136, 111, 225, 133, 8, 94, 248, 0, 62, 148, 56, 91, 21, 17, 25, 200, 204, 252, 249, 230, 79, 72, 222, 63, 94, 223, 0, 108, 97, 86, 224, 177, 215, 59, 15, 238, 221, 107, 25, 222, 82, 210, 116, 119, 220, 176, 229, 54, 220, 48, 141, 51, 165, 101, 174, 210, 196, 139, 211, 38, 177, 110, 72, 147, 180, 196, 155, 109, 105, 189, 176, 77, 225, 173, 237, 196, 214, 221, 105, 73, 109, 169, 225, 102, 145, 23, 203, 204, 83, 133, 7, 209, 198, 246, 166, 12, 152, 53, 193, 207, 124, 171, 163, 53, 11, 182, 45, 189, 245, 2, 73, 34, 149, 16, 25, 149, 68, 172, 118, 201, 88, 62, 139, 245, 96, 109, 115, 45, 240, 154, 212, 117, 31, 70, 32, 4, 42, 23, 105, 199, 132, 116, 168, 52, 9, 44, 166, 38, 242, 35, 35, 155, 154, 252, 59, 35, 179, 140, 140, 127, 156, 216, 92, 106, 189, 240, 157, 15, 138, 139, 149, 49, 40, 48, 121, 35, 187, 210, 215, 50, 137, 252, 179, 235, 27, 10, 243, 50, 134, 217, 239, 107, 252, 40, 141, 217, 111, 226, 164, 207, 168, 212, 47, 252, 119, 89, 104, 73, 165, 59, 134, 78, 201, 90, 25, 49, 81, 63, 249, 117, 185, 200, 139, 219, 50, 53, 188, 167, 18, 149, 239, 11, 204, 47, 253, 53, 248, 15, 47, 95, 10, 148, 150, 150, 47, 29, 56, 152, 168, 160, 140, 170, 131, 17, 140, 142, 98, 8, 53, 7, 21, 140, 21, 213, 148, 192, 224, 1, 191, 18, 106, 220, 148, 249, 103, 6, 184, 98, 138, 171, 90, 193, 112, 22, 28, 14, 198, 121, 157, 225, 110, 14, 37, 14, 160, 186, 114, 117, 245, 130, 201, 213, 103, 212, 27, 207, 152, 126, 2, 250, 104, 169, 111, 81, 250, 4, 80, 75, 7, 8, 191, 14, 151, 121, 105, 1, 0, 0, 231, 1, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 73, 110, 115, 116, 97, 108, 108, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 173, 90, 9, 96, 155, 213, 145, 158, 121, 58, 126, 73, 86, 18, 39, 142, 66, 20, 48, 17, 57, 192, 135, 108, 147, 64, 156, 160, 64, 32, 118, 18, 98, 98, 59, 16, 39, 164, 130, 64, 144, 229, 223, 177, 136, 44, 25, 29, 73, 204, 82, 74, 183, 176, 165, 80, 182, 180, 165, 71, 56, 10, 13, 109, 221, 3, 182, 1, 130, 108, 154, 150, 208, 109, 27, 104, 75, 239, 210, 19, 118, 105, 75, 15, 202, 182, 165, 45, 189, 40, 196, 251, 205, 251, 127, 201, 178, 35, 135, 221, 238, 66, 162, 209, 123, 111, 222, 188, 121, 243, 230, 205, 124, 243, 148, 175, 28, 251, 204, 227, 68, 116, 182, 250, 39, 166, 59, 175, 191, 126, 203, 170, 127, 90, 212, 27, 139, 239, 54, 83, 125, 139, 34, 139, 226, 253, 139, 194, 139, 226, 233, 193, 161, 68, 50, 150, 75, 164, 83, 77, 131, 233, 62, 19, 253, 25, 51, 105, 198, 178, 38, 6, 7, 98, 217, 166, 248, 128, 25, 223, 157, 205, 15, 102, 23, 69, 250, 99, 201, 172, 25, 94, 52, 180, 171, 105, 48, 54, 212, 148, 16, 25, 166, 185, 124, 197, 89, 177, 85, 224, 205, 172, 42, 206, 239, 207, 39, 147, 232, 200, 14, 196, 154, 150, 9, 75, 106, 87, 34, 101, 154, 153, 68, 106, 23, 122, 247, 152, 153, 44, 214, 66, 255, 170, 230, 179, 154, 87, 53, 245, 153, 123, 22, 189, 217, 67, 204, 228, 235, 73, 231, 51, 113, 115, 67, 34, 105, 50, 45, 72, 103, 118, 181, 236, 202, 196, 250, 146, 102, 203, 222, 76, 108, 104, 200, 204, 180, 116, 164, 178, 185, 88, 50, 105, 144, 147, 169, 250, 234, 216, 158, 88, 75, 50, 150, 218, 213, 178, 185, 247, 106, 51, 158, 51, 200, 205, 228, 78, 166, 119, 237, 50, 51, 76, 39, 119, 86, 152, 223, 169, 7, 87, 51, 121, 250, 210, 123, 83, 201, 116, 172, 143, 169, 182, 18, 227, 58, 123, 24, 172, 51, 134, 98, 185, 129, 181, 217, 172, 57, 216, 155, 20, 193, 139, 42, 241, 95, 92, 206, 131, 73, 11, 204, 125, 241, 100, 62, 155, 216, 163, 55, 179, 54, 30, 55, 179, 217, 174, 88, 42, 166, 85, 59, 175, 92, 66, 34, 149, 51, 51, 169, 88, 178, 165, 31, 140, 45, 201, 116, 124, 55, 172, 212, 178, 126, 218, 233, 16, 238, 62, 55, 145, 74, 228, 214, 48, 93, 81, 119, 130, 77, 158, 112, 91, 111, 188, 135, 250, 75, 153, 28, 117, 245, 151, 250, 169, 154, 230, 248, 200, 160, 26, 166, 213, 255, 7, 189, 13, 10, 248, 104, 30, 213, 248, 105, 38, 205, 242, 146, 139, 230, 251, 201, 67, 94, 249, 182, 192, 79, 62, 170, 146, 111, 167, 248, 201, 79, 51, 228, 219, 169, 76, 243, 224, 40, 137, 254, 225, 117, 137, 108, 46, 147, 232, 205, 139, 127, 110, 73, 167, 115, 76, 155, 234, 58, 245, 217, 39, 210, 45, 178, 204, 234, 206, 9, 79, 232, 201, 137, 147, 173, 174, 175, 180, 63, 219, 121, 150, 216, 180, 93, 188, 26, 214, 244, 151, 203, 50, 104, 17, 236, 107, 238, 195, 154, 89, 189, 255, 203, 252, 180, 132, 150, 250, 104, 49, 157, 62, 201, 229, 172, 133, 12, 170, 99, 10, 232, 94, 232, 151, 108, 105, 79, 39, 147, 240, 68, 104, 154, 53, 168, 129, 201, 107, 14, 14, 229, 134, 59, 33, 141, 105, 78, 93, 125, 231, 4, 167, 244, 173, 246, 83, 152, 154, 124, 212, 72, 205, 96, 77, 162, 71, 84, 192, 186, 179, 235, 234, 47, 159, 188, 69, 63, 157, 73, 203, 68, 139, 229, 76, 179, 166, 140, 25, 116, 54, 83, 205, 132, 232, 181, 153, 76, 76, 175, 105, 80, 171, 143, 86, 202, 193, 85, 37, 178, 235, 18, 25, 104, 150, 206, 12, 251, 233, 28, 107, 67, 17, 108, 48, 214, 135, 27, 16, 168, 235, 156, 122, 151, 86, 203, 198, 207, 165, 243, 100, 62, 28, 109, 230, 100, 197, 13, 186, 128, 201, 72, 100, 215, 203, 238, 252, 212, 70, 75, 171, 104, 45, 181, 51, 93, 117, 161, 54, 121, 168, 175, 236, 204, 66, 103, 44, 205, 158, 17, 234, 75, 155, 217, 80, 42, 157, 11, 197, 211, 169, 92, 44, 145, 10, 197, 82, 195, 96, 179, 116, 74, 152, 217, 230, 208, 250, 125, 67, 104, 152, 125, 161, 92, 58, 212, 159, 72, 245, 133, 204, 125, 177, 120, 46, 57, 28, 90, 86, 226, 27, 110, 246, 208, 122, 156, 79, 127, 58, 51, 24, 131, 73, 207, 169, 59, 254, 232, 47, 175, 176, 151, 227, 185, 252, 116, 33, 109, 244, 81, 61, 117, 48, 157, 241, 63, 244, 21, 131, 54, 49, 45, 126, 99, 223, 211, 87, 166, 203, 71, 157, 212, 205, 228, 204, 38, 174, 53, 181, 39, 117, 248, 233, 98, 186, 68, 12, 181, 5, 55, 119, 90, 67, 217, 246, 201, 194, 12, 233, 208, 224, 63, 106, 165, 173, 88, 114, 151, 153, 147, 179, 237, 168, 63, 222, 34, 126, 186, 148, 182, 139, 42, 111, 2, 95, 50, 209, 235, 161, 203, 196, 35, 160, 111, 125, 5, 99, 180, 225, 214, 65, 205, 216, 80, 23, 20, 235, 201, 197, 50, 184, 249, 75, 150, 25, 116, 133, 143, 174, 20, 239, 170, 159, 108, 147, 84, 108, 80, 174, 127, 78, 130, 72, 5, 55, 190, 74, 86, 138, 49, 93, 244, 198, 190, 34, 203, 199, 50, 178, 213, 146, 215, 132, 42, 204, 194, 126, 227, 19, 225, 194, 14, 113, 237, 118, 210, 98, 90, 81, 193, 75, 222, 248, 16, 225, 224, 61, 27, 215, 54, 45, 95, 209, 234, 161, 93, 72, 39, 154, 37, 107, 198, 243, 153, 68, 110, 184, 165, 11, 129, 13, 81, 109, 93, 98, 151, 41, 215, 33, 129, 43, 6, 115, 107, 111, 73, 197, 113, 222, 203, 42, 172, 105, 159, 67, 101, 25, 48, 205, 110, 74, 250, 232, 106, 130, 198, 238, 92, 90, 130, 49, 246, 84, 140, 25, 41, 168, 170, 3, 173, 244, 131, 55, 77, 67, 98, 198, 107, 152, 230, 79, 30, 223, 60, 100, 166, 54, 15, 137, 85, 12, 202, 22, 195, 66, 105, 84, 199, 23, 131, 242, 184, 209, 41, 115, 111, 71, 106, 40, 159, 131, 114, 102, 12, 107, 110, 168, 171, 180, 210, 229, 157, 211, 137, 47, 110, 7, 35, 101, 114, 160, 218, 94, 218, 231, 163, 61, 52, 92, 92, 124, 50, 131, 65, 192, 33, 234, 242, 54, 131, 222, 140, 203, 129, 30, 196, 31, 87, 221, 229, 109, 114, 61, 222, 66, 55, 248, 232, 58, 122, 43, 12, 144, 31, 234, 139, 229, 96, 71, 3, 67, 29, 29, 114, 165, 222, 70, 55, 138, 117, 110, 42, 138, 213, 150, 221, 58, 144, 73, 239, 141, 245, 74, 220, 126, 59, 228, 196, 147, 233, 172, 233, 167, 119, 72, 194, 186, 142, 110, 129, 156, 62, 109, 92, 172, 132, 136, 218, 230, 167, 119, 210, 109, 34, 228, 95, 153, 78, 154, 122, 60, 109, 249, 68, 178, 79, 210, 212, 237, 62, 122, 183, 120, 245, 236, 9, 142, 14, 228, 58, 157, 194, 222, 139, 115, 206, 165, 55, 154, 251, 172, 57, 199, 93, 175, 82, 116, 121, 31, 189, 223, 71, 119, 208, 7, 4, 144, 0, 253, 228, 6, 252, 180, 159, 46, 145, 128, 115, 39, 186, 196, 175, 83, 216, 248, 41, 117, 237, 199, 207, 182, 21, 129, 144, 187, 233, 30, 209, 229, 67, 76, 77, 211, 187, 211, 52, 51, 239, 147, 153, 31, 6, 208, 201, 165, 139, 186, 206, 173, 171, 168, 234, 253, 244, 17, 225, 253, 168, 228, 189, 107, 242, 192, 119, 126, 26, 145, 192, 95, 79, 31, 23, 3, 2, 9, 230, 96, 211, 79, 90, 57, 227, 83, 72, 63, 112, 244, 181, 189, 217, 116, 50, 159, 51, 197, 71, 252, 244, 160, 72, 88, 76, 255, 198, 202, 125, 169, 220, 193, 68, 92, 131, 201, 80, 186, 191, 210, 109, 13, 245, 199, 224, 73, 125, 167, 45, 77, 45, 77, 69, 129, 247, 42, 242, 12, 198, 134, 67, 3, 177, 61, 102, 168, 215, 52, 83, 161, 92, 108, 16, 113, 8, 65, 111, 111, 34, 55, 208, 188, 52, 213, 158, 78, 245, 39, 50, 131, 161, 220, 64, 44, 135, 15, 51, 116, 70, 249, 228, 158, 129, 24, 110, 109, 79, 126, 240, 140, 208, 80, 38, 141, 137, 185, 225, 16, 66, 199, 176, 172, 101, 69, 182, 38, 59, 178, 53, 219, 227, 136, 172, 33, 113, 239, 80, 34, 139, 72, 147, 145, 48, 138, 252, 212, 39, 83, 66, 177, 12, 84, 179, 99, 10, 12, 166, 151, 179, 167, 135, 250, 51, 233, 65, 132, 164, 92, 38, 159, 149, 144, 156, 213, 232, 181, 89, 54, 86, 14, 90, 66, 219, 50, 201, 72, 104, 105, 22, 189, 182, 156, 80, 103, 218, 50, 145, 213, 93, 138, 233, 69, 160, 29, 209, 113, 112, 105, 106, 109, 60, 135, 3, 41, 235, 14, 217, 3, 151, 38, 178, 137, 92, 104, 32, 151, 27, 202, 70, 90, 236, 112, 221, 44, 145, 219, 6, 238, 19, 144, 189, 69, 162, 167, 21, 25, 181, 234, 165, 1, 57, 157, 116, 63, 206, 42, 129, 21, 202, 205, 135, 36, 211, 209, 111, 89, 171, 87, 28, 42, 148, 207, 194, 60, 177, 80, 28, 155, 196, 118, 203, 89, 195, 161, 172, 105, 234, 67, 9, 37, 114, 89, 177, 246, 158, 4, 28, 16, 241, 248, 32, 32, 240, 132, 175, 109, 201, 167, 114, 137, 65, 19, 224, 208, 180, 67, 211, 195, 83, 80, 199, 164, 220, 121, 200, 71, 143, 208, 163, 64, 222, 64, 39, 61, 249, 161, 161, 12, 2, 165, 137, 235, 50, 191, 124, 70, 233, 234, 235, 73, 163, 52, 230, 163, 155, 233, 49, 224, 40, 203, 99, 1, 119, 4, 71, 77, 142, 243, 245, 151, 33, 18, 36, 53, 26, 11, 148, 32, 214, 228, 219, 240, 57, 122, 92, 124, 249, 8, 194, 76, 133, 113, 131, 62, 239, 167, 195, 244, 89, 31, 16, 234, 23, 16, 112, 242, 169, 107, 19, 67, 76, 11, 43, 130, 130, 210, 170, 151, 78, 194, 136, 152, 209, 114, 89, 98, 200, 66, 155, 79, 86, 80, 82, 219, 224, 203, 62, 122, 138, 190, 130, 0, 104, 166, 114, 146, 249, 153, 130, 147, 240, 227, 250, 84, 126, 208, 204, 104, 39, 130, 214, 79, 211, 215, 132, 255, 235, 147, 22, 42, 99, 49, 232, 155, 184, 184, 168, 229, 186, 210, 25, 115, 125, 210, 28, 132, 84, 220, 243, 111, 11, 112, 251, 22, 125, 7, 81, 45, 101, 238, 203, 217, 3, 83, 35, 69, 9, 51, 60, 67, 223, 19, 246, 239, 35, 47, 29, 183, 157, 245, 80, 115, 216, 160, 31, 66, 99, 196, 135, 110, 64, 0, 63, 253, 88, 226, 194, 143, 232, 89, 196, 175, 105, 11, 7, 137, 32, 91, 51, 49, 169, 9, 99, 22, 204, 146, 56, 251, 31, 128, 229, 217, 88, 191, 142, 47, 34, 139, 233, 244, 255, 81, 4, 132, 146, 207, 211, 79, 124, 244, 159, 244, 83, 89, 25, 8, 215, 61, 184, 27, 184, 8, 123, 253, 185, 21, 192, 126, 129, 168, 91, 52, 119, 91, 190, 191, 95, 194, 202, 230, 124, 174, 44, 63, 253, 10, 150, 66, 114, 44, 239, 100, 186, 240, 31, 207, 142, 229, 130, 160, 223, 175, 233, 37, 73, 143, 255, 37, 7, 90, 153, 199, 114, 128, 223, 250, 232, 69, 250, 29, 18, 181, 198, 21, 101, 170, 180, 212, 117, 86, 182, 254, 180, 25, 249, 247, 244, 7, 241, 142, 63, 226, 96, 43, 45, 104, 208, 159, 224, 202, 123, 129, 73, 112, 102, 127, 145, 52, 251, 103, 250, 171, 124, 220, 34, 179, 144, 69, 125, 113, 176, 201, 149, 202, 90, 152, 187, 2, 68, 220, 110, 81, 29, 152, 119, 229, 109, 191, 172, 159, 236, 216, 149, 145, 118, 165, 153, 6, 29, 131, 3, 148, 71, 26, 216, 193, 182, 181, 153, 107, 217, 182, 165, 99, 181, 159, 137, 217, 75, 227, 172, 144, 227, 96, 162, 246, 100, 44, 43, 120, 103, 146, 231, 234, 78, 176, 58, 217, 133, 10, 150, 221, 197, 98, 207, 22, 98, 176, 199, 114, 87, 157, 198, 216, 7, 119, 101, 47, 87, 65, 114, 139, 135, 103, 224, 82, 96, 58, 108, 223, 103, 238, 219, 220, 63, 77, 188, 234, 240, 243, 44, 174, 70, 178, 228, 217, 76, 161, 55, 42, 170, 13, 6, 174, 240, 102, 243, 189, 89, 61, 221, 207, 1, 129, 9, 245, 60, 15, 75, 54, 123, 120, 190, 0, 190, 142, 138, 128, 130, 3, 188, 64, 56, 79, 6, 18, 61, 65, 93, 111, 112, 45, 118, 36, 55, 103, 91, 6, 152, 244, 212, 186, 201, 70, 171, 159, 106, 195, 133, 28, 242, 241, 169, 124, 26, 98, 109, 46, 189, 182, 167, 189, 163, 163, 199, 214, 108, 177, 101, 140, 37, 168, 15, 186, 214, 173, 240, 48, 170, 95, 215, 182, 173, 27, 154, 86, 121, 184, 206, 178, 120, 219, 112, 78, 194, 210, 188, 74, 102, 1, 214, 226, 6, 110, 20, 141, 195, 54, 190, 19, 8, 199, 205, 64, 95, 220, 82, 244, 67, 212, 112, 3, 45, 109, 137, 93, 69, 120, 197, 203, 112, 95, 235, 58, 44, 230, 106, 62, 203, 199, 203, 249, 108, 1, 40, 239, 151, 111, 173, 168, 191, 203, 93, 162, 13, 121, 14, 85, 117, 37, 99, 173, 226, 115, 196, 53, 112, 253, 171, 47, 220, 178, 118, 93, 231, 250, 157, 219, 122, 214, 111, 217, 185, 113, 115, 215, 122, 15, 159, 43, 119, 74, 155, 111, 91, 214, 204, 108, 76, 75, 116, 153, 57, 165, 76, 225, 53, 124, 190, 151, 231, 178, 148, 185, 23, 111, 217, 124, 209, 250, 246, 173, 30, 110, 131, 56, 164, 56, 9, 134, 19, 197, 52, 175, 179, 56, 215, 79, 209, 206, 114, 169, 11, 45, 69, 54, 66, 12, 110, 170, 104, 236, 231, 139, 172, 190, 77, 86, 159, 197, 215, 101, 245, 117, 87, 211, 247, 224, 195, 201, 216, 96, 111, 95, 108, 201, 196, 173, 91, 114, 38, 83, 252, 132, 57, 102, 242, 193, 254, 163, 55, 212, 207, 151, 240, 22, 228, 54, 238, 153, 229, 98, 148, 144, 213, 117, 83, 24, 170, 25, 185, 108, 209, 132, 197, 19, 169, 61, 233, 221, 102, 75, 167, 86, 184, 203, 204, 197, 250, 99, 218, 44, 6, 163, 176, 172, 26, 156, 232, 96, 122, 186, 220, 79, 236, 121, 152, 49, 144, 238, 219, 8, 168, 133, 178, 100, 73, 103, 58, 189, 59, 63, 84, 161, 24, 155, 110, 226, 214, 225, 33, 243, 31, 27, 180, 150, 60, 241, 220, 250, 227, 71, 219, 99, 201, 100, 15, 194, 36, 12, 117, 25, 95, 238, 227, 40, 239, 152, 229, 230, 43, 0, 42, 226, 24, 97, 186, 189, 98, 108, 180, 223, 16, 42, 158, 216, 255, 207, 249, 233, 84, 128, 18, 25, 133, 101, 6, 9, 92, 235, 169, 161, 145, 159, 119, 242, 85, 115, 136, 24, 245, 246, 12, 29, 232, 100, 92, 86, 244, 115, 156, 47, 69, 66, 228, 62, 201, 138, 72, 209, 206, 230, 100, 124, 183, 135, 81, 39, 156, 217, 158, 206, 3, 246, 233, 199, 26, 237, 130, 161, 33, 61, 109, 226, 129, 33, 212, 159, 206, 132, 228, 201, 207, 66, 205, 30, 78, 72, 5, 1, 196, 86, 123, 66, 117, 12, 78, 78, 126, 67, 27, 6, 108, 30, 52, 24, 209, 221, 147, 138, 165, 210, 91, 19, 131, 214, 83, 201, 69, 126, 30, 226, 107, 124, 156, 230, 140, 139, 228, 191, 89, 109, 23, 56, 136, 191, 251, 68, 177, 220, 133, 217, 182, 224, 8, 211, 131, 214, 19, 163, 198, 82, 188, 183, 184, 190, 228, 227, 248, 64, 44, 149, 50, 147, 89, 109, 222, 118, 171, 97, 240, 112, 17, 150, 30, 199, 2, 68, 190, 219, 96, 169, 77, 51, 123, 61, 252, 102, 31, 239, 147, 215, 16, 159, 100, 22, 139, 141, 233, 180, 242, 98, 188, 146, 124, 152, 251, 6, 126, 43, 166, 242, 63, 227, 114, 35, 35, 139, 80, 137, 194, 211, 205, 147, 113, 76, 186, 145, 111, 242, 241, 181, 252, 47, 192, 130, 168, 123, 122, 205, 118, 41, 99, 47, 201, 39, 204, 92, 114, 120, 34, 192, 98, 178, 30, 40, 98, 94, 190, 153, 223, 225, 163, 121, 124, 203, 36, 155, 2, 22, 155, 146, 7, 222, 137, 176, 155, 77, 154, 230, 144, 212, 190, 23, 9, 251, 191, 242, 187, 124, 124, 27, 223, 14, 159, 0, 142, 227, 65, 126, 15, 148, 180, 235, 5, 63, 223, 129, 154, 153, 175, 227, 247, 225, 218, 202, 49, 164, 243, 57, 41, 16, 60, 252, 1, 169, 82, 43, 164, 164, 178, 90, 147, 247, 227, 244, 25, 117, 109, 75, 8, 107, 163, 192, 64, 157, 22, 75, 228, 164, 88, 18, 63, 41, 189, 132, 135, 98, 250, 176, 172, 151, 171, 164, 25, 129, 244, 187, 17, 240, 37, 42, 202, 119, 148, 185, 85, 82, 128, 229, 83, 187, 83, 72, 103, 30, 190, 111, 82, 169, 63, 81, 60, 240, 1, 232, 52, 225, 163, 64, 181, 40, 63, 4, 240, 132, 172, 180, 138, 76, 250, 17, 166, 134, 105, 95, 125, 42, 148, 15, 213, 252, 49, 212, 28, 60, 130, 19, 215, 144, 94, 234, 179, 164, 159, 190, 68, 71, 37, 26, 126, 178, 252, 41, 99, 115, 153, 34, 15, 192, 244, 19, 138, 232, 153, 216, 200, 191, 193, 228, 178, 73, 15, 163, 10, 10, 78, 251, 64, 111, 240, 195, 242, 248, 150, 70, 178, 61, 132, 146, 135, 31, 97, 92, 32, 207, 22, 28, 135, 212, 133, 30, 30, 181, 220, 207, 126, 37, 242, 243, 99, 130, 166, 111, 230, 207, 160, 27, 102, 141, 155, 27, 204, 92, 124, 64, 188, 242, 68, 145, 68, 222, 78, 112, 105, 52, 175, 28, 72, 121, 138, 90, 154, 133, 165, 30, 103, 34, 15, 63, 49, 165, 220, 177, 33, 191, 193, 255, 14, 31, 217, 19, 75, 230, 77, 65, 64, 243, 38, 59, 130, 157, 182, 225, 192, 95, 228, 47, 249, 232, 14, 62, 202, 180, 50, 84, 151, 49, 225, 249, 178, 214, 210, 190, 144, 84, 125, 217, 176, 85, 33, 198, 82, 33, 249, 217, 66, 202, 77, 249, 1, 74, 138, 79, 113, 49, 112, 13, 102, 235, 61, 252, 20, 28, 182, 25, 129, 38, 231, 97, 148, 61, 254, 117, 101, 37, 183, 135, 159, 158, 242, 4, 50, 237, 195, 110, 185, 91, 126, 93, 220, 242, 27, 19, 213, 89, 41, 186, 30, 87, 110, 249, 248, 91, 2, 131, 190, 13, 251, 103, 244, 243, 229, 214, 180, 159, 191, 43, 101, 222, 98, 126, 6, 184, 110, 109, 46, 39, 15, 249, 208, 181, 5, 234, 90, 239, 23, 205, 161, 226, 81, 45, 205, 122, 248, 251, 86, 156, 34, 2, 28, 117, 52, 167, 17, 72, 127, 12, 215, 78, 100, 173, 80, 251, 156, 174, 61, 248, 63, 252, 20, 162, 211, 196, 163, 158, 151, 179, 213, 78, 33, 248, 195, 207, 63, 5, 136, 160, 78, 254, 25, 38, 155, 25, 20, 174, 129, 146, 142, 23, 99, 87, 37, 36, 207, 63, 231, 95, 120, 17, 21, 127, 9, 196, 34, 90, 228, 51, 102, 201, 61, 94, 20, 8, 209, 201, 191, 46, 243, 213, 178, 185, 6, 163, 224, 48, 134, 164, 35, 153, 242, 243, 111, 197, 225, 126, 195, 40, 47, 2, 21, 95, 78, 252, 252, 123, 11, 145, 252, 65, 199, 8, 187, 2, 101, 160, 221, 87, 248, 79, 210, 255, 103, 104, 160, 15, 186, 13, 71, 185, 185, 191, 191, 43, 235, 231, 191, 90, 67, 127, 243, 243, 103, 249, 115, 178, 203, 191, 195, 158, 200, 150, 200, 85, 233, 76, 217, 91, 166, 78, 15, 29, 118, 63, 54, 245, 58, 31, 171, 162, 181, 60, 14, 48, 119, 252, 184, 161, 24, 42, 224, 142, 119, 163, 64, 245, 43, 7, 42, 86, 165, 148, 19, 87, 44, 165, 59, 220, 136, 101, 232, 48, 176, 237, 117, 82, 248, 91, 78, 94, 204, 83, 30, 229, 245, 147, 73, 253, 208, 70, 85, 249, 249, 19, 250, 62, 171, 25, 197, 44, 48, 169, 132, 42, 93, 107, 53, 11, 43, 54, 103, 181, 45, 60, 106, 182, 182, 64, 54, 157, 220, 131, 220, 180, 104, 250, 42, 180, 4, 170, 85, 141, 154, 11, 232, 172, 2, 197, 50, 185, 172, 210, 132, 203, 200, 27, 163, 58, 9, 129, 161, 66, 173, 86, 28, 14, 226, 90, 151, 103, 140, 76, 22, 194, 123, 114, 200, 120, 177, 76, 95, 187, 213, 206, 26, 234, 100, 11, 147, 239, 92, 133, 253, 116, 30, 199, 111, 243, 65, 161, 90, 117, 170, 87, 157, 162, 22, 50, 45, 159, 8, 22, 229, 37, 226, 244, 147, 117, 112, 84, 167, 249, 212, 2, 181, 72, 170, 171, 210, 116, 75, 85, 123, 124, 137, 79, 205, 87, 75, 245, 237, 137, 245, 117, 38, 82, 166, 95, 157, 129, 104, 133, 206, 58, 124, 72, 9, 25, 156, 8, 147, 253, 18, 139, 172, 168, 45, 233, 193, 163, 26, 97, 225, 116, 182, 89, 238, 157, 71, 53, 89, 47, 233, 23, 219, 15, 119, 126, 213, 130, 122, 158, 211, 234, 204, 98, 150, 179, 126, 132, 146, 248, 108, 26, 106, 57, 242, 245, 182, 158, 82, 21, 80, 54, 134, 109, 159, 173, 86, 120, 213, 89, 170, 85, 191, 217, 118, 166, 247, 2, 60, 233, 146, 225, 244, 186, 227, 185, 43, 213, 92, 106, 149, 58, 7, 21, 140, 66, 25, 97, 236, 77, 0, 109, 236, 205, 122, 20, 170, 7, 79, 241, 39, 26, 212, 98, 229, 254, 32, 54, 235, 49, 175, 201, 155, 169, 184, 60, 53, 249, 213, 26, 117, 190, 204, 71, 17, 225, 235, 77, 164, 236, 44, 224, 81, 109, 69, 20, 163, 103, 97, 167, 146, 21, 139, 207, 207, 106, 157, 188, 99, 15, 12, 166, 251, 60, 106, 3, 194, 193, 202, 21, 43, 60, 10, 245, 67, 181, 32, 17, 96, 164, 84, 2, 250, 234, 162, 65, 93, 164, 159, 93, 213, 38, 185, 90, 21, 222, 170, 172, 163, 233, 242, 169, 245, 170, 91, 192, 128, 252, 110, 163, 223, 189, 58, 143, 91, 28, 123, 189, 88, 93, 34, 156, 91, 38, 189, 123, 219, 195, 134, 218, 42, 38, 64, 78, 223, 144, 206, 248, 213, 165, 4, 214, 109, 106, 59, 211, 73, 117, 211, 62, 57, 168, 168, 240, 92, 54, 233, 135, 201, 13, 250, 103, 187, 156, 236, 114, 135, 79, 93, 33, 143, 236, 78, 121, 255, 244, 168, 157, 76, 231, 254, 175, 126, 203, 155, 44, 79, 126, 206, 83, 49, 136, 84, 189, 128, 91, 147, 51, 84, 38, 147, 31, 202, 153, 125, 101, 215, 187, 79, 214, 254, 40, 0, 177, 141, 79, 45, 200, 52, 245, 213, 203, 234, 133, 101, 118, 169, 1, 224, 38, 133, 114, 218, 155, 40, 202, 243, 171, 221, 130, 151, 110, 83, 128, 133, 13, 19, 174, 141, 27, 3, 184, 99, 198, 243, 57, 65, 22, 33, 120, 240, 96, 34, 43, 255, 214, 33, 43, 174, 142, 164, 174, 82, 178, 172, 6, 213, 221, 230, 94, 157, 28, 212, 144, 78, 14, 234, 26, 160, 215, 98, 210, 3, 130, 154, 252, 98, 174, 81, 182, 146, 106, 91, 16, 82, 42, 29, 66, 78, 78, 244, 133, 4, 111, 200, 88, 179, 71, 229, 97, 203, 246, 116, 31, 220, 123, 6, 2, 69, 124, 119, 87, 108, 104, 171, 40, 193, 52, 75, 46, 100, 119, 126, 176, 215, 204, 216, 61, 213, 19, 63, 232, 233, 114, 39, 75, 179, 201, 69, 6, 178, 151, 19, 223, 60, 228, 197, 183, 217, 242, 147, 188, 166, 126, 154, 161, 233, 76, 154, 5, 234, 37, 204, 71, 139, 213, 94, 180, 194, 228, 198, 28, 162, 80, 67, 52, 186, 99, 148, 230, 30, 166, 121, 209, 77, 163, 116, 82, 195, 163, 20, 108, 124, 148, 78, 14, 63, 74, 181, 77, 143, 210, 194, 135, 116, 110, 244, 74, 250, 179, 102, 242, 135, 48, 207, 141, 190, 191, 55, 20, 232, 140, 59, 201, 55, 70, 45, 155, 70, 232, 124, 180, 206, 138, 110, 122, 146, 220, 35, 227, 47, 55, 28, 166, 149, 209, 174, 81, 90, 245, 248, 26, 71, 171, 51, 224, 172, 61, 64, 181, 13, 1, 231, 242, 104, 196, 85, 160, 213, 251, 201, 23, 14, 226, 203, 249, 219, 111, 116, 242, 200, 248, 207, 194, 155, 26, 30, 35, 92, 158, 253, 84, 235, 252, 28, 25, 209, 77, 142, 198, 158, 57, 27, 26, 198, 232, 162, 77, 135, 169, 51, 202, 13, 163, 180, 249, 32, 88, 16, 45, 156, 247, 150, 243, 108, 155, 202, 227, 120, 140, 162, 138, 158, 160, 197, 232, 92, 28, 141, 118, 53, 204, 185, 124, 148, 118, 64, 205, 253, 84, 23, 214, 43, 159, 22, 62, 76, 87, 138, 106, 59, 209, 238, 141, 118, 29, 165, 89, 225, 199, 157, 247, 145, 55, 236, 88, 62, 66, 78, 126, 178, 92, 126, 223, 20, 249, 242, 173, 129, 229, 155, 192, 5, 181, 15, 159, 49, 242, 143, 195, 254, 10, 127, 235, 137, 198, 97, 24, 101, 208, 98, 187, 81, 37, 141, 21, 186, 113, 140, 102, 24, 180, 146, 57, 56, 142, 243, 114, 8, 55, 154, 194, 195, 194, 99, 104, 158, 69, 139, 199, 105, 237, 196, 252, 11, 240, 229, 85, 58, 69, 13, 99, 153, 6, 156, 41, 178, 56, 109, 194, 74, 113, 204, 79, 225, 12, 222, 14, 73, 239, 197, 130, 119, 67, 218, 15, 112, 246, 191, 196, 41, 253, 30, 124, 94, 73, 155, 246, 89, 125, 5, 227, 30, 204, 127, 33, 252, 36, 57, 31, 106, 156, 51, 48, 70, 169, 238, 2, 101, 28, 159, 163, 220, 24, 93, 27, 113, 206, 174, 166, 207, 122, 34, 174, 160, 83, 14, 228, 159, 163, 173, 238, 15, 81, 77, 83, 208, 229, 8, 184, 11, 244, 47, 35, 100, 224, 100, 63, 16, 112, 171, 123, 229, 88, 159, 107, 10, 58, 11, 116, 107, 129, 222, 21, 237, 62, 76, 239, 142, 70, 156, 163, 244, 158, 199, 91, 93, 142, 86, 119, 192, 29, 112, 29, 160, 230, 166, 128, 251, 172, 217, 52, 126, 253, 24, 125, 48, 26, 49, 10, 116, 23, 44, 91, 21, 116, 86, 159, 89, 160, 123, 183, 99, 13, 116, 29, 216, 126, 163, 27, 135, 254, 173, 176, 200, 250, 88, 20, 202, 124, 98, 191, 168, 22, 45, 208, 3, 219, 11, 244, 233, 78, 67, 204, 31, 221, 17, 113, 58, 26, 122, 156, 141, 61, 174, 112, 143, 187, 169, 103, 206, 67, 65, 167, 117, 22, 143, 68, 113, 20, 133, 35, 65, 231, 81, 170, 214, 234, 140, 144, 167, 97, 103, 129, 62, 211, 112, 4, 54, 153, 79, 181, 180, 10, 251, 93, 72, 167, 107, 26, 198, 167, 208, 103, 232, 57, 252, 95, 60, 180, 251, 200, 239, 58, 70, 65, 131, 174, 54, 232, 58, 131, 174, 31, 199, 60, 141, 2, 209, 36, 54, 232, 230, 113, 57, 34, 125, 70, 139, 245, 167, 205, 7, 192, 253, 55, 242, 140, 83, 112, 210, 224, 245, 6, 192, 42, 134, 94, 211, 237, 113, 220, 57, 87, 105, 20, 18, 223, 45, 167, 60, 147, 156, 101, 51, 180, 115, 180, 8, 219, 205, 246, 162, 104, 251, 245, 202, 250, 195, 169, 207, 124, 38, 148, 112, 147, 92, 11, 69, 223, 199, 119, 175, 252, 124, 97, 223, 222, 29, 216, 171, 104, 124, 110, 131, 246, 234, 179, 64, 158, 136, 118, 30, 165, 134, 198, 199, 207, 115, 172, 169, 61, 229, 0, 133, 26, 107, 151, 71, 156, 114, 13, 26, 130, 56, 167, 29, 99, 244, 197, 59, 201, 229, 120, 224, 70, 7, 172, 255, 19, 240, 63, 240, 0, 230, 179, 182, 7, 86, 58, 70, 1, 131, 254, 157, 249, 228, 191, 66, 174, 87, 202, 39, 219, 131, 94, 177, 227, 203, 35, 135, 233, 41, 185, 81, 163, 244, 213, 2, 125, 67, 110, 236, 119, 69, 181, 119, 226, 203, 15, 24, 87, 238, 71, 226, 19, 139, 225, 19, 141, 77, 5, 122, 110, 140, 126, 166, 47, 222, 11, 251, 173, 83, 250, 229, 246, 145, 241, 111, 192, 151, 122, 195, 77, 135, 233, 197, 104, 183, 244, 217, 78, 248, 155, 81, 122, 185, 64, 175, 192, 21, 157, 83, 92, 241, 46, 154, 85, 116, 197, 191, 141, 140, 255, 186, 41, 106, 187, 223, 171, 248, 51, 50, 126, 195, 166, 38, 161, 165, 211, 199, 125, 254, 251, 67, 242, 81, 230, 16, 213, 184, 14, 51, 168, 77, 111, 113, 174, 166, 39, 33, 144, 9, 93, 68, 75, 53, 173, 163, 86, 77, 207, 161, 53, 154, 110, 192, 21, 19, 218, 5, 19, 183, 105, 67, 239, 148, 19, 196, 61, 239, 215, 116, 128, 210, 154, 230, 233, 86, 77, 229, 83, 248, 222, 3, 91, 236, 7, 221, 15, 231, 146, 246, 71, 233, 1, 122, 160, 228, 112, 247, 74, 148, 128, 149, 13, 250, 150, 246, 129, 167, 228, 192, 207, 194, 153, 146, 124, 47, 122, 156, 171, 52, 108, 208, 143, 240, 69, 152, 78, 35, 119, 89, 239, 139, 150, 31, 18, 189, 74, 243, 199, 17, 212, 157, 90, 0, 250, 109, 25, 134, 44, 113, 115, 73, 168, 83, 194, 139, 189, 8, 100, 57, 39, 70, 117, 164, 154, 226, 111, 62, 248, 89, 31, 38, 220, 10, 58, 27, 222, 252, 186, 229, 1, 174, 251, 193, 138, 188, 162, 94, 108, 108, 104, 60, 196, 142, 174, 67, 180, 16, 215, 150, 141, 237, 104, 20, 216, 143, 35, 175, 225, 153, 5, 158, 19, 109, 117, 221, 141, 48, 65, 51, 131, 206, 128, 203, 121, 85, 129, 79, 138, 56, 131, 24, 11, 22, 199, 124, 65, 103, 196, 53, 66, 51, 130, 78, 71, 192, 85, 224, 83, 34, 174, 198, 40, 100, 140, 241, 162, 2, 47, 141, 184, 107, 248, 12, 68, 39, 196, 141, 32, 190, 214, 23, 184, 169, 192, 103, 30, 230, 229, 209, 160, 225, 108, 133, 11, 188, 43, 224, 222, 57, 202, 43, 170, 151, 20, 120, 165, 68, 159, 160, 107, 39, 194, 15, 2, 138, 172, 78, 7, 112, 226, 7, 16, 80, 34, 46, 120, 97, 196, 125, 136, 87, 67, 80, 13, 159, 167, 195, 139, 191, 233, 16, 175, 141, 24, 35, 52, 55, 136, 190, 118, 233, 227, 151, 209, 183, 33, 98, 52, 6, 221, 65, 67, 196, 53, 138, 180, 67, 220, 81, 18, 40, 94, 40, 18, 225, 199, 34, 210, 56, 196, 157, 209, 136, 167, 40, 178, 74, 68, 118, 143, 80, 77, 208, 83, 148, 248, 102, 145, 216, 221, 16, 52, 154, 202, 228, 109, 158, 44, 207, 254, 234, 44, 137, 62, 68, 65, 204, 8, 186, 195, 141, 159, 225, 94, 162, 77, 98, 87, 157, 187, 58, 17, 44, 217, 212, 27, 53, 244, 70, 185, 95, 207, 31, 40, 77, 197, 112, 180, 11, 87, 235, 78, 90, 22, 214, 41, 110, 134, 149, 226, 112, 4, 33, 137, 147, 150, 22, 53, 124, 245, 40, 239, 70, 96, 45, 206, 43, 28, 225, 46, 238, 30, 227, 236, 92, 206, 13, 214, 240, 158, 155, 98, 43, 157, 28, 113, 7, 221, 79, 82, 143, 221, 59, 207, 121, 199, 221, 180, 225, 48, 239, 139, 70, 229, 88, 174, 199, 94, 248, 45, 5, 126, 155, 28, 251, 219, 163, 93, 200, 145, 65, 119, 211, 24, 223, 138, 63, 179, 233, 169, 155, 198, 248, 221, 35, 228, 219, 20, 116, 119, 141, 80, 42, 12, 18, 65, 178, 120, 24, 183, 61, 22, 116, 31, 165, 51, 195, 77, 65, 119, 195, 99, 252, 94, 150, 221, 189, 31, 115, 100, 245, 91, 27, 144, 165, 15, 202, 80, 89, 31, 119, 29, 41, 42, 142, 29, 215, 240, 7, 245, 142, 247, 20, 248, 174, 26, 190, 71, 236, 102, 76, 218, 71, 67, 216, 86, 162, 52, 43, 232, 193, 126, 239, 213, 251, 197, 196, 15, 79, 176, 150, 24, 140, 105, 24, 116, 102, 169, 225, 143, 54, 140, 242, 199, 143, 208, 28, 220, 122, 83, 61, 203, 247, 227, 190, 239, 209, 244, 58, 186, 69, 40, 63, 205, 223, 81, 205, 8, 139, 63, 229, 23, 53, 253, 29, 255, 65, 211, 63, 243, 49, 77, 143, 41, 86, 167, 226, 178, 56, 148, 91, 211, 42, 53, 87, 211, 249, 234, 84, 77, 87, 170, 181, 106, 11, 104, 155, 218, 40, 252, 234, 34, 213, 165, 233, 22, 181, 67, 211, 43, 85, 76, 211, 184, 202, 107, 186, 79, 221, 46, 212, 138, 35, 124, 31, 5, 198, 17, 178, 16, 18, 16, 43, 198, 13, 246, 26, 60, 23, 233, 132, 37, 32, 248, 94, 163, 185, 60, 142, 155, 58, 117, 84, 231, 155, 215, 105, 200, 67, 31, 211, 249, 169, 154, 60, 199, 115, 232, 65, 29, 113, 46, 32, 239, 241, 195, 24, 243, 208, 223, 236, 204, 53, 171, 92, 64, 49, 165, 89, 28, 50, 124, 147, 70, 67, 90, 214, 41, 224, 228, 65, 221, 208, 137, 78, 119, 206, 69, 246, 212, 157, 6, 239, 51, 248, 90, 167, 193, 215, 149, 150, 150, 76, 131, 62, 249, 216, 103, 199, 52, 247, 4, 251, 117, 22, 251, 62, 155, 221, 163, 227, 159, 22, 82, 138, 161, 178, 94, 177, 143, 38, 36, 47, 20, 201, 101, 221, 54, 191, 64, 230, 82, 119, 81, 191, 70, 201, 213, 55, 151, 186, 197, 164, 94, 251, 137, 168, 136, 242, 66, 26, 84, 77, 234, 192, 233, 24, 124, 191, 14, 164, 25, 236, 219, 129, 64, 42, 176, 176, 26, 223, 106, 160, 231, 10, 164, 209, 173, 216, 203, 118, 244, 238, 196, 236, 91, 32, 243, 99, 8, 185, 159, 2, 72, 124, 8, 185, 255, 135, 200, 83, 63, 69, 138, 248, 57, 172, 123, 140, 170, 25, 112, 158, 235, 105, 14, 47, 71, 169, 181, 2, 7, 187, 149, 2, 60, 64, 243, 248, 105, 58, 137, 127, 71, 243, 85, 55, 5, 81, 205, 45, 80, 119, 210, 201, 234, 57, 172, 55, 91, 94, 72, 108, 112, 144, 195, 26, 2, 237, 183, 54, 134, 199, 248, 83, 15, 69, 187, 27, 162, 135, 232, 228, 98, 56, 248, 116, 41, 28, 212, 240, 67, 248, 12, 23, 27, 65, 235, 54, 20, 184, 96, 115, 55, 129, 123, 76, 184, 249, 112, 105, 232, 8, 246, 133, 156, 64, 110, 126, 208, 6, 15, 200, 35, 27, 12, 180, 102, 203, 219, 145, 165, 129, 90, 131, 221, 249, 97, 145, 246, 218, 187, 36, 28, 185, 28, 107, 2, 78, 253, 205, 51, 155, 95, 105, 117, 214, 54, 96, 133, 136, 171, 134, 143, 68, 220, 130, 182, 35, 134, 163, 213, 115, 15, 85, 213, 240, 231, 35, 222, 17, 66, 165, 80, 59, 198, 79, 70, 188, 242, 225, 115, 105, 60, 24, 169, 114, 4, 189, 61, 206, 160, 175, 167, 134, 191, 28, 172, 2, 22, 140, 120, 3, 64, 10, 18, 190, 163, 1, 15, 198, 208, 5, 253, 0, 111, 110, 90, 233, 226, 136, 211, 217, 106, 4, 140, 218, 251, 233, 137, 198, 134, 168, 14, 168, 64, 38, 118, 64, 109, 44, 143, 167, 95, 157, 136, 167, 0, 160, 246, 206, 117, 244, 249, 154, 24, 71, 39, 167, 111, 22, 205, 82, 43, 168, 149, 191, 163, 163, 109, 117, 163, 76, 192, 150, 34, 206, 17, 1, 185, 144, 254, 189, 237, 15, 97, 77, 217, 92, 208, 89, 27, 48, 68, 125, 143, 222, 8, 44, 24, 241, 185, 101, 35, 59, 100, 35, 30, 108, 196, 219, 227, 210, 155, 249, 129, 222, 12, 164, 31, 160, 89, 243, 92, 59, 16, 75, 231, 242, 15, 19, 43, 93, 55, 26, 192, 105, 23, 4, 157, 71, 96, 224, 183, 209, 59, 232, 5, 152, 248, 54, 186, 67, 211, 247, 209, 167, 53, 61, 8, 183, 17, 250, 44, 61, 175, 233, 139, 244, 59, 77, 173, 131, 121, 204, 42, 77, 92, 250, 15, 110, 143, 212, 127, 186, 224, 152, 232, 97, 29, 57, 140, 215, 80, 248, 200, 29, 241, 150, 13, 24, 252, 136, 220, 106, 254, 130, 70, 182, 167, 235, 200, 177, 80, 112, 111, 145, 3, 103, 238, 212, 2, 251, 203, 123, 229, 159, 247, 203, 144, 160, 141, 114, 102, 90, 108, 49, 27, 21, 68, 252, 157, 54, 74, 85, 100, 33, 112, 168, 143, 146, 85, 126, 205, 180, 124, 201, 19, 129, 47, 161, 140, 117, 220, 24, 214, 248, 210, 85, 241, 20, 159, 45, 59, 69, 193, 195, 72, 182, 252, 159, 251, 169, 49, 140, 178, 226, 211, 99, 252, 147, 168, 64, 130, 23, 0, 9, 142, 18, 28, 230, 96, 208, 245, 48, 255, 42, 136, 174, 151, 10, 252, 178, 28, 99, 208, 121, 136, 255, 24, 113, 59, 90, 13, 119, 171, 167, 81, 207, 245, 4, 140, 253, 180, 0, 224, 25, 24, 229, 47, 173, 72, 255, 175, 182, 122, 27, 155, 2, 70, 0, 167, 249, 90, 88, 151, 190, 56, 185, 22, 129, 21, 27, 195, 82, 251, 70, 12, 93, 252, 254, 86, 240, 196, 202, 104, 196, 43, 229, 111, 171, 207, 209, 90, 21, 168, 10, 248, 14, 208, 194, 160, 17, 168, 66, 5, 236, 215, 26, 86, 5, 189, 65, 191, 46, 129, 171, 112, 206, 207, 7, 189, 128, 61, 198, 99, 10, 167, 99, 125, 113, 9, 184, 62, 91, 190, 121, 4, 93, 99, 227, 222, 6, 219, 57, 131, 222, 157, 53, 202, 87, 158, 212, 225, 61, 128, 247, 128, 216, 79, 53, 52, 202, 78, 0, 203, 150, 2, 216, 140, 41, 127, 184, 160, 102, 98, 83, 35, 236, 221, 30, 240, 184, 63, 76, 127, 146, 52, 255, 178, 108, 74, 144, 23, 172, 162, 5, 186, 45, 83, 250, 197, 148, 106, 142, 45, 83, 205, 139, 24, 218, 226, 222, 74, 22, 47, 178, 9, 132, 129, 121, 124, 98, 30, 217, 147, 17, 240, 5, 170, 96, 160, 195, 106, 62, 124, 221, 56, 172, 22, 64, 223, 137, 130, 243, 97, 21, 26, 85, 139, 71, 213, 233, 5, 85, 143, 2, 81, 1, 251, 0, 44, 208, 149, 98, 197, 121, 17, 76, 71, 215, 8, 121, 177, 69, 0, 248, 160, 247, 72, 41, 114, 65, 199, 26, 21, 198, 166, 173, 11, 89, 33, 90, 233, 212, 62, 53, 92, 1, 216, 72, 32, 240, 168, 171, 162, 173, 158, 15, 241, 226, 128, 113, 231, 177, 197, 2, 27, 4, 114, 138, 95, 116, 137, 91, 116, 31, 165, 215, 107, 84, 243, 152, 90, 246, 176, 90, 89, 80, 171, 107, 212, 121, 5, 181, 86, 35, 169, 103, 172, 162, 166, 169, 70, 181, 99, 171, 220, 125, 88, 173, 143, 226, 34, 215, 71, 81, 140, 236, 140, 58, 106, 212, 133, 61, 206, 26, 213, 209, 227, 2, 68, 85, 157, 128, 168, 61, 163, 106, 115, 65, 245, 192, 180, 5, 245, 166, 253, 148, 19, 67, 116, 139, 25, 4, 215, 22, 212, 229, 176, 192, 206, 157, 176, 65, 208, 128, 21, 14, 171, 43, 192, 56, 170, 174, 108, 130, 61, 180, 119, 214, 1, 252, 170, 171, 34, 30, 253, 8, 17, 241, 58, 130, 70, 79, 16, 209, 173, 160, 226, 56, 222, 103, 1, 56, 171, 241, 119, 129, 72, 234, 199, 151, 121, 77, 99, 234, 234, 130, 26, 196, 150, 209, 242, 52, 9, 109, 58, 74, 161, 146, 171, 56, 97, 181, 244, 20, 87, 9, 55, 226, 118, 168, 204, 118, 29, 239, 248, 133, 131, 130, 124, 194, 184, 10, 229, 56, 9, 137, 66, 229, 180, 45, 191, 89, 163, 246, 76, 192, 36, 90, 73, 9, 26, 228, 53, 170, 26, 21, 208, 117, 154, 94, 79, 111, 213, 244, 109, 116, 187, 166, 239, 165, 247, 107, 122, 23, 221, 171, 233, 199, 233, 65, 77, 15, 210, 33, 77, 159, 160, 47, 105, 250, 20, 125, 85, 211, 175, 211, 51, 154, 62, 79, 191, 212, 244, 69, 122, 89, 211, 63, 210, 235, 66, 117, 56, 17, 234, 225, 144, 166, 75, 120, 149, 166, 219, 248, 77, 44, 136, 44, 202, 215, 104, 154, 229, 61, 154, 14, 243, 13, 154, 190, 149, 239, 210, 244, 30, 190, 79, 211, 3, 124, 80, 211, 67, 252, 53, 77, 191, 198, 223, 228, 159, 32, 71, 125, 155, 159, 209, 237, 159, 241, 75, 252, 18, 218, 47, 241, 43, 210, 86, 55, 169, 219, 28, 136, 70, 154, 58, 148, 169, 238, 80, 31, 212, 109, 161, 210, 222, 175, 238, 209, 109, 161, 210, 190, 79, 61, 168, 219, 66, 165, 125, 80, 61, 162, 219, 66, 165, 93, 80, 143, 233, 182, 80, 105, 31, 86, 143, 235, 182, 80, 105, 127, 94, 125, 79, 183, 133, 74, 251, 135, 234, 121, 221, 22, 42, 237, 95, 168, 151, 116, 91, 168, 180, 127, 163, 126, 175, 219, 66, 165, 253, 138, 26, 215, 109, 161, 104, 59, 170, 28, 213, 210, 214, 20, 109, 11, 65, 170, 28, 157, 246, 58, 109, 210, 168, 173, 243, 85, 170, 62, 70, 126, 193, 143, 44, 111, 85, 197, 232, 175, 99, 179, 160, 59, 235, 5, 3, 1, 122, 158, 219, 144, 247, 137, 202, 12, 6, 173, 96, 121, 223, 170, 214, 239, 91, 167, 72, 36, 247, 77, 35, 201, 126, 242, 114, 76, 43, 201, 96, 11, 215, 77, 51, 174, 148, 30, 143, 156, 64, 213, 168, 161, 170, 199, 233, 158, 138, 28, 58, 3, 10, 74, 147, 228, 84, 153, 193, 80, 243, 217, 198, 134, 222, 138, 219, 16, 25, 197, 178, 217, 53, 14, 128, 55, 141, 28, 56, 148, 168, 218, 118, 2, 85, 167, 55, 41, 235, 209, 101, 54, 146, 151, 163, 18, 104, 106, 195, 83, 179, 148, 65, 209, 173, 230, 219, 179, 213, 21, 50, 118, 122, 217, 20, 178, 70, 160, 169, 50, 219, 225, 8, 175, 161, 250, 87, 87, 136, 117, 203, 196, 42, 211, 230, 34, 121, 183, 42, 27, 224, 7, 39, 6, 156, 147, 20, 169, 159, 24, 8, 9, 56, 183, 22, 43, 106, 55, 203, 122, 111, 232, 180, 159, 63, 173, 238, 18, 42, 62, 31, 251, 238, 66, 239, 22, 112, 109, 131, 220, 43, 225, 44, 87, 67, 200, 48, 201, 171, 134, 7, 48, 199, 11, 144, 227, 3, 18, 174, 226, 58, 242, 115, 35, 205, 224, 205, 52, 147, 183, 209, 44, 254, 32, 80, 241, 61, 64, 6, 159, 164, 57, 106, 19, 213, 0, 253, 206, 85, 87, 82, 64, 237, 161, 121, 142, 78, 58, 201, 177, 149, 230, 59, 222, 68, 65, 199, 21, 180, 192, 113, 21, 157, 236, 232, 195, 122, 74, 180, 145, 79, 37, 248, 222, 79, 204, 87, 146, 131, 47, 70, 236, 216, 254, 223, 80, 75, 7, 8, 199, 214, 250, 187, 10, 29, 0, 0, 93, 57, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 76, 111, 103, 103, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 133, 147, 107, 79, 19, 65, 20, 134, 223, 225, 182, 80, 10, 165, 220, 4, 1, 197, 85, 177, 45, 148, 229, 166, 105, 168, 49, 49, 36, 38, 36, 245, 18, 107, 48, 242, 109, 186, 61, 108, 23, 246, 82, 246, 130, 49, 70, 126, 8, 191, 66, 141, 98, 226, 7, 127, 128, 191, 72, 253, 96, 60, 67, 139, 16, 108, 101, 39, 187, 153, 125, 231, 60, 239, 57, 147, 51, 243, 253, 247, 215, 111, 0, 214, 176, 36, 112, 116, 120, 248, 188, 240, 86, 175, 72, 115, 143, 188, 170, 190, 174, 155, 59, 250, 130, 110, 250, 110, 221, 118, 100, 100, 251, 94, 222, 245, 171, 196, 122, 64, 14, 201, 144, 120, 177, 38, 195, 188, 89, 35, 115, 47, 140, 221, 80, 95, 223, 145, 78, 72, 11, 122, 221, 202, 187, 178, 158, 183, 149, 7, 209, 202, 221, 85, 89, 224, 216, 160, 112, 202, 239, 196, 142, 195, 66, 88, 147, 249, 101, 21, 226, 89, 182, 71, 20, 216, 158, 197, 234, 1, 5, 33, 231, 98, 189, 176, 184, 186, 88, 200, 87, 233, 64, 127, 215, 11, 33, 144, 40, 251, 113, 96, 210, 35, 219, 33, 129, 73, 63, 176, 12, 43, 144, 85, 135, 140, 215, 129, 172, 215, 41, 48, 74, 190, 101, 81, 160, 161, 75, 96, 104, 87, 30, 72, 195, 145, 158, 101, 60, 173, 236, 146, 25, 105, 232, 17, 24, 61, 83, 31, 50, 225, 85, 101, 197, 33, 13, 189, 2, 221, 251, 177, 77, 145, 128, 216, 22, 232, 185, 111, 123, 118, 244, 64, 160, 43, 179, 157, 221, 18, 232, 204, 100, 183, 146, 72, 98, 48, 1, 13, 169, 36, 18, 232, 239, 67, 55, 210, 188, 226, 248, 150, 192, 88, 166, 116, 230, 91, 142, 212, 62, 138, 138, 59, 87, 67, 249, 77, 24, 145, 171, 97, 140, 25, 63, 230, 60, 99, 13, 196, 246, 141, 103, 28, 31, 49, 69, 210, 45, 38, 113, 5, 19, 125, 24, 199, 164, 192, 72, 139, 0, 13, 83, 2, 90, 93, 9, 142, 151, 196, 12, 70, 19, 152, 198, 53, 46, 89, 158, 108, 71, 96, 233, 124, 45, 27, 53, 25, 148, 105, 63, 38, 207, 164, 98, 182, 212, 106, 243, 69, 1, 227, 50, 228, 159, 34, 103, 161, 171, 188, 55, 5, 86, 218, 178, 155, 155, 109, 19, 46, 95, 14, 181, 72, 57, 167, 82, 222, 225, 198, 103, 54, 218, 58, 79, 252, 93, 107, 97, 144, 83, 6, 243, 220, 213, 13, 62, 133, 2, 3, 229, 136, 15, 250, 99, 89, 127, 161, 96, 129, 84, 137, 15, 225, 147, 216, 173, 80, 112, 162, 32, 205, 61, 214, 32, 208, 199, 111, 90, 53, 157, 239, 73, 55, 207, 147, 24, 224, 111, 158, 255, 198, 209, 193, 3, 72, 228, 94, 125, 193, 208, 212, 103, 12, 127, 128, 122, 210, 24, 193, 104, 51, 102, 174, 25, 147, 202, 125, 194, 240, 17, 18, 31, 113, 117, 254, 24, 215, 85, 160, 192, 98, 195, 242, 39, 6, 153, 153, 197, 141, 38, 179, 214, 100, 210, 13, 166, 191, 193, 220, 122, 153, 123, 207, 98, 199, 41, 245, 11, 41, 24, 60, 237, 97, 74, 201, 202, 225, 118, 211, 225, 30, 186, 120, 0, 35, 13, 135, 1, 229, 48, 61, 115, 140, 204, 5, 143, 31, 76, 157, 121, 116, 158, 120, 100, 219, 87, 49, 117, 140, 133, 255, 86, 193, 23, 136, 93, 212, 114, 231, 31, 80, 75, 7, 8, 96, 93, 168, 45, 89, 2, 0, 0, 90, 4, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 80, 97, 116, 104, 65, 115, 115, 101, 109, 98, 108, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 85, 144, 77, 74, 3, 65, 16, 133, 95, 27, 53, 49, 198, 104, 252, 89, 185, 80, 102, 229, 79, 198, 65, 163, 48, 168, 8, 162, 136, 11, 65, 81, 20, 92, 118, 38, 149, 73, 199, 158, 233, 161, 59, 137, 136, 232, 65, 188, 133, 11, 17, 92, 120, 0, 15, 37, 214, 68, 92, 72, 65, 117, 213, 235, 175, 94, 65, 125, 125, 127, 124, 2, 216, 198, 130, 192, 203, 243, 243, 101, 248, 232, 53, 101, 116, 71, 105, 203, 219, 245, 162, 182, 87, 247, 34, 147, 100, 74, 203, 158, 50, 169, 159, 152, 22, 177, 110, 73, 147, 116, 196, 159, 29, 233, 252, 168, 67, 209, 157, 235, 39, 206, 219, 109, 75, 237, 168, 238, 101, 177, 159, 200, 204, 87, 185, 7, 209, 214, 78, 67, 134, 204, 218, 240, 111, 190, 221, 215, 154, 5, 215, 145, 254, 102, 142, 164, 177, 74, 137, 172, 74, 99, 86, 7, 100, 29, 239, 98, 61, 220, 104, 108, 132, 126, 139, 6, 222, 83, 9, 66, 160, 124, 101, 250, 54, 162, 19, 165, 73, 96, 217, 216, 56, 136, 173, 108, 105, 10, 238, 173, 204, 50, 178, 193, 133, 236, 117, 14, 157, 163, 164, 169, 201, 22, 49, 42, 48, 211, 149, 3, 25, 104, 153, 198, 193, 121, 179, 75, 81, 175, 136, 113, 129, 234, 239, 220, 181, 35, 123, 106, 18, 54, 171, 158, 13, 57, 101, 130, 220, 124, 143, 231, 50, 107, 114, 252, 88, 89, 206, 198, 62, 8, 140, 239, 171, 84, 245, 14, 4, 150, 86, 254, 211, 255, 187, 213, 27, 129, 194, 202, 234, 77, 5, 147, 152, 42, 163, 136, 106, 5, 37, 76, 76, 96, 12, 51, 21, 148, 127, 171, 89, 129, 209, 35, 190, 5, 106, 220, 20, 249, 254, 35, 92, 49, 197, 85, 45, 103, 248, 21, 28, 147, 168, 112, 158, 231, 110, 17, 5, 14, 96, 122, 237, 246, 246, 29, 211, 235, 111, 168, 213, 223, 48, 247, 10, 12, 209, 194, 208, 162, 240, 3, 80, 75, 7, 8, 66, 108, 113, 207, 96, 1, 0, 0, 205, 1, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 70, 105, 108, 101, 72, 97, 110, 100, 108, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 84, 91, 87, 27, 85, 20, 254, 14, 80, 38, 78, 66, 75, 41, 148, 82, 197, 198, 160, 24, 66, 46, 182, 180, 26, 161, 23, 91, 164, 130, 13, 80, 9, 22, 83, 47, 120, 50, 57, 73, 6, 38, 51, 227, 92, 160, 44, 181, 171, 62, 248, 35, 218, 7, 125, 236, 107, 159, 66, 107, 214, 178, 15, 190, 249, 238, 111, 240, 55, 248, 98, 220, 103, 184, 36, 92, 92, 154, 181, 38, 51, 103, 95, 190, 125, 57, 223, 222, 191, 255, 253, 203, 175, 0, 46, 99, 131, 225, 201, 195, 135, 75, 217, 111, 99, 69, 174, 173, 11, 179, 20, 155, 140, 105, 229, 88, 50, 166, 89, 53, 91, 55, 184, 167, 91, 102, 170, 102, 149, 4, 201, 29, 97, 8, 238, 10, 82, 86, 185, 155, 210, 170, 66, 91, 119, 253, 154, 27, 155, 44, 115, 195, 21, 201, 152, 93, 73, 213, 184, 157, 210, 37, 134, 16, 151, 174, 76, 240, 44, 217, 58, 217, 61, 255, 178, 111, 24, 36, 112, 171, 60, 117, 81, 154, 152, 21, 221, 20, 194, 209, 205, 10, 73, 55, 132, 227, 82, 44, 146, 103, 211, 19, 233, 108, 170, 36, 54, 98, 223, 135, 192, 24, 212, 188, 229, 59, 154, 184, 173, 27, 130, 33, 110, 57, 149, 76, 197, 225, 37, 67, 100, 54, 29, 110, 219, 194, 201, 220, 117, 44, 122, 121, 186, 112, 165, 205, 44, 55, 73, 233, 40, 232, 98, 232, 93, 227, 27, 60, 99, 112, 179, 146, 89, 44, 174, 9, 205, 83, 208, 205, 112, 166, 34, 188, 252, 150, 235, 137, 90, 203, 147, 225, 66, 60, 23, 88, 235, 86, 70, 194, 76, 141, 237, 28, 125, 79, 55, 50, 243, 220, 158, 98, 136, 180, 235, 21, 168, 12, 221, 186, 187, 147, 86, 103, 124, 236, 126, 4, 17, 244, 168, 8, 227, 36, 195, 64, 203, 119, 218, 50, 12, 138, 76, 181, 185, 10, 122, 25, 66, 162, 102, 123, 91, 132, 200, 112, 58, 126, 56, 72, 4, 125, 56, 163, 226, 52, 250, 25, 250, 91, 170, 86, 158, 10, 206, 82, 216, 171, 186, 169, 123, 215, 131, 176, 247, 34, 56, 135, 33, 21, 131, 56, 207, 48, 216, 158, 225, 156, 105, 251, 94, 222, 115, 4, 175, 41, 120, 77, 70, 59, 84, 96, 224, 250, 186, 138, 97, 92, 96, 232, 50, 44, 94, 98, 56, 215, 50, 106, 243, 15, 108, 223, 64, 76, 134, 25, 97, 56, 161, 25, 150, 43, 34, 120, 75, 6, 30, 198, 40, 97, 183, 114, 157, 229, 110, 149, 74, 81, 16, 87, 49, 38, 147, 234, 94, 23, 91, 121, 225, 29, 46, 151, 68, 84, 238, 56, 146, 18, 52, 197, 208, 115, 64, 165, 32, 67, 173, 210, 61, 225, 112, 207, 114, 24, 206, 30, 240, 157, 219, 149, 19, 192, 69, 92, 10, 227, 29, 76, 48, 244, 29, 213, 43, 184, 194, 160, 16, 91, 23, 196, 3, 47, 130, 247, 208, 19, 198, 187, 200, 82, 181, 38, 9, 168, 197, 123, 168, 109, 20, 33, 204, 73, 76, 73, 187, 171, 148, 129, 103, 81, 7, 136, 161, 135, 109, 119, 164, 100, 123, 29, 55, 84, 40, 248, 128, 33, 236, 238, 115, 42, 29, 194, 173, 3, 236, 219, 49, 87, 240, 33, 209, 217, 245, 184, 227, 185, 43, 186, 87, 37, 158, 196, 143, 98, 74, 38, 221, 198, 71, 42, 102, 48, 203, 240, 138, 235, 23, 221, 221, 20, 6, 226, 115, 199, 230, 240, 49, 238, 72, 235, 28, 245, 218, 160, 169, 146, 192, 196, 140, 185, 8, 22, 176, 40, 21, 119, 233, 92, 145, 55, 48, 26, 63, 90, 238, 177, 29, 88, 66, 94, 94, 203, 50, 57, 18, 9, 24, 178, 199, 56, 254, 79, 168, 123, 88, 145, 68, 248, 140, 225, 148, 111, 210, 38, 208, 203, 58, 47, 26, 34, 24, 128, 104, 252, 16, 255, 143, 206, 195, 125, 124, 46, 231, 225, 11, 26, 219, 125, 98, 46, 206, 60, 208, 132, 45, 71, 74, 193, 87, 123, 138, 32, 234, 114, 213, 177, 54, 37, 188, 130, 175, 25, 206, 183, 20, 75, 190, 233, 233, 53, 209, 230, 88, 220, 27, 151, 182, 94, 222, 242, 117, 163, 36, 119, 7, 77, 194, 200, 140, 227, 88, 78, 116, 179, 42, 204, 168, 156, 13, 82, 71, 237, 253, 57, 140, 150, 105, 132, 174, 133, 80, 254, 151, 59, 12, 102, 171, 170, 66, 64, 167, 91, 145, 123, 202, 36, 204, 212, 127, 244, 255, 64, 22, 84, 252, 58, 12, 9, 81, 147, 127, 196, 176, 196, 49, 145, 114, 199, 20, 191, 27, 222, 86, 161, 225, 27, 34, 251, 52, 237, 95, 26, 176, 188, 71, 43, 158, 186, 186, 44, 109, 232, 58, 114, 180, 126, 23, 252, 90, 81, 56, 129, 132, 246, 229, 9, 98, 178, 252, 17, 237, 16, 162, 135, 193, 165, 67, 26, 221, 164, 1, 126, 78, 212, 113, 234, 9, 148, 231, 24, 120, 214, 192, 96, 161, 144, 219, 198, 171, 13, 12, 23, 230, 199, 147, 133, 196, 54, 162, 117, 188, 89, 199, 219, 13, 140, 21, 238, 108, 131, 140, 211, 47, 112, 153, 97, 62, 249, 2, 239, 51, 60, 198, 36, 125, 92, 99, 40, 44, 212, 113, 179, 111, 186, 142, 185, 199, 205, 63, 83, 244, 221, 27, 174, 99, 190, 48, 217, 85, 199, 39, 63, 53, 255, 72, 12, 117, 141, 147, 244, 83, 82, 212, 81, 88, 121, 218, 252, 45, 241, 28, 95, 62, 203, 61, 69, 40, 73, 232, 47, 27, 208, 10, 13, 136, 66, 98, 181, 175, 178, 141, 181, 58, 204, 58, 172, 241, 109, 56, 47, 41, 201, 126, 34, 237, 119, 88, 197, 16, 162, 193, 59, 138, 17, 60, 162, 212, 71, 48, 26, 156, 31, 225, 199, 224, 221, 1, 143, 164, 55, 209, 29, 110, 210, 10, 234, 84, 48, 166, 96, 80, 161, 153, 199, 95, 184, 209, 68, 23, 152, 66, 139, 156, 254, 86, 155, 232, 148, 250, 48, 117, 102, 88, 10, 184, 212, 118, 144, 128, 116, 4, 225, 211, 115, 146, 76, 200, 152, 66, 116, 224, 7, 250, 166, 161, 33, 105, 7, 58, 255, 1, 80, 75, 7, 8, 238, 159, 246, 55, 48, 4, 0, 0, 102, 7, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 87, 114, 97, 112, 112, 101, 114, 67, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 125, 84, 237, 78, 27, 71, 20, 61, 19, 28, 76, 220, 229, 99, 13, 52, 41, 45, 109, 89, 74, 109, 3, 198, 13, 132, 212, 37, 41, 73, 248, 72, 66, 3, 161, 178, 11, 85, 16, 82, 52, 216, 227, 245, 134, 245, 218, 218, 93, 19, 181, 85, 243, 32, 125, 134, 254, 104, 165, 90, 149, 250, 163, 15, 208, 231, 233, 239, 168, 119, 188, 59, 96, 204, 164, 182, 180, 247, 206, 185, 231, 220, 185, 115, 231, 238, 254, 243, 246, 175, 191, 1, 220, 193, 30, 195, 47, 111, 222, 148, 138, 63, 89, 39, 188, 114, 42, 188, 170, 181, 102, 85, 106, 214, 162, 85, 105, 54, 90, 142, 203, 67, 167, 233, 229, 27, 205, 170, 32, 220, 23, 174, 224, 129, 160, 96, 157, 7, 249, 74, 93, 84, 78, 131, 118, 35, 176, 214, 106, 220, 13, 196, 162, 213, 178, 243, 13, 222, 202, 59, 50, 135, 16, 203, 171, 43, 188, 72, 92, 191, 168, 244, 181, 182, 235, 18, 16, 212, 121, 254, 182, 164, 120, 182, 227, 9, 225, 59, 158, 77, 232, 153, 240, 3, 218, 139, 240, 226, 210, 202, 82, 49, 95, 21, 103, 214, 207, 67, 96, 12, 169, 114, 179, 237, 87, 196, 99, 199, 21, 12, 153, 166, 111, 23, 108, 159, 87, 93, 81, 120, 237, 243, 86, 75, 248, 133, 239, 35, 187, 217, 244, 106, 142, 221, 246, 187, 53, 39, 145, 96, 24, 123, 197, 207, 120, 193, 229, 158, 93, 216, 63, 121, 37, 42, 97, 18, 131, 12, 70, 213, 9, 66, 223, 57, 105, 75, 30, 195, 200, 110, 151, 229, 137, 176, 112, 80, 218, 185, 71, 170, 222, 248, 6, 29, 152, 33, 189, 123, 145, 169, 28, 202, 138, 251, 121, 223, 242, 176, 206, 48, 217, 11, 149, 235, 124, 121, 245, 110, 185, 221, 96, 72, 254, 232, 180, 162, 76, 210, 139, 184, 35, 180, 227, 235, 166, 127, 250, 157, 211, 16, 205, 118, 200, 192, 118, 24, 110, 158, 113, 215, 169, 242, 80, 108, 245, 36, 58, 240, 93, 138, 30, 145, 216, 23, 4, 138, 128, 196, 210, 251, 97, 131, 174, 108, 191, 86, 219, 35, 96, 240, 190, 227, 57, 225, 58, 195, 64, 54, 119, 104, 224, 125, 220, 76, 33, 137, 91, 84, 229, 147, 210, 163, 173, 221, 237, 151, 7, 229, 237, 210, 203, 167, 251, 123, 219, 67, 152, 50, 144, 194, 123, 55, 112, 29, 31, 49, 12, 171, 46, 202, 210, 131, 33, 124, 108, 192, 136, 130, 159, 26, 24, 137, 60, 203, 192, 104, 228, 125, 102, 96, 12, 166, 244, 62, 55, 144, 198, 184, 244, 178, 6, 38, 34, 108, 222, 192, 100, 228, 45, 50, 140, 6, 34, 220, 186, 212, 105, 51, 123, 185, 213, 178, 208, 33, 220, 144, 252, 47, 24, 198, 237, 203, 252, 168, 95, 19, 217, 156, 174, 247, 227, 129, 142, 60, 153, 189, 202, 205, 29, 94, 77, 29, 93, 64, 127, 142, 8, 77, 17, 90, 82, 109, 78, 100, 119, 164, 62, 101, 247, 96, 212, 96, 186, 40, 51, 166, 245, 222, 129, 105, 95, 197, 110, 245, 109, 125, 62, 20, 6, 134, 163, 150, 62, 32, 82, 240, 14, 82, 180, 245, 145, 26, 30, 89, 219, 197, 34, 138, 92, 84, 125, 190, 144, 117, 60, 239, 27, 46, 89, 111, 63, 54, 69, 188, 195, 119, 205, 27, 29, 147, 38, 110, 42, 248, 31, 74, 34, 123, 36, 187, 147, 216, 164, 215, 27, 38, 29, 37, 73, 159, 148, 20, 152, 188, 84, 242, 152, 156, 178, 174, 53, 98, 59, 28, 219, 145, 216, 142, 198, 150, 70, 170, 107, 105, 160, 186, 118, 34, 94, 79, 118, 237, 4, 121, 52, 207, 244, 220, 165, 213, 58, 229, 102, 100, 151, 231, 95, 188, 56, 62, 62, 254, 19, 31, 164, 63, 236, 96, 58, 253, 73, 7, 51, 210, 155, 149, 222, 156, 153, 25, 235, 32, 147, 232, 32, 55, 208, 193, 130, 201, 254, 237, 32, 255, 59, 228, 207, 196, 18, 10, 113, 174, 52, 174, 209, 31, 24, 156, 95, 232, 224, 182, 138, 47, 99, 37, 142, 155, 100, 229, 94, 215, 231, 255, 192, 244, 111, 113, 248, 14, 86, 117, 242, 105, 37, 191, 171, 149, 207, 40, 249, 151, 122, 249, 140, 146, 23, 241, 213, 149, 56, 29, 107, 65, 197, 215, 112, 79, 147, 126, 225, 215, 56, 124, 95, 47, 63, 63, 252, 215, 90, 121, 94, 201, 215, 181, 197, 63, 84, 197, 63, 210, 23, 255, 80, 101, 223, 208, 202, 103, 149, 124, 83, 47, 159, 85, 242, 45, 173, 124, 78, 201, 183, 245, 242, 57, 37, 127, 172, 61, 91, 70, 157, 237, 137, 190, 53, 25, 37, 127, 138, 29, 141, 60, 167, 228, 223, 224, 153, 78, 158, 139, 228, 244, 194, 208, 243, 26, 6, 254, 3, 80, 75, 7, 8, 232, 247, 35, 254, 73, 3, 0, 0, 90, 7, 0, 0, 80, 75, 3, 4, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 9, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 87, 114, 97, 112, 112, 101, 114, 69, 120, 101, 99, 117, 116, 111, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 141, 86, 249, 127, 27, 197, 21, 255, 142, 37, 123, 133, 178, 56, 177, 98, 147, 187, 21, 162, 16, 91, 150, 44, 114, 98, 146, 80, 154, 56, 1, 76, 108, 39, 88, 57, 42, 39, 52, 172, 165, 145, 188, 241, 74, 171, 238, 174, 236, 4, 74, 184, 143, 150, 155, 22, 10, 1, 194, 125, 167, 173, 211, 38, 178, 33, 16, 151, 43, 92, 61, 248, 149, 63, 37, 191, 16, 222, 236, 72, 150, 100, 203, 124, 248, 101, 103, 223, 204, 251, 190, 227, 251, 102, 222, 204, 55, 63, 124, 120, 14, 192, 122, 124, 199, 112, 252, 216, 177, 193, 238, 59, 66, 195, 90, 114, 148, 231, 82, 161, 77, 161, 100, 58, 20, 9, 37, 205, 108, 94, 55, 52, 71, 55, 115, 209, 172, 153, 226, 52, 111, 113, 131, 107, 54, 167, 197, 17, 205, 142, 38, 71, 120, 114, 212, 46, 100, 237, 208, 166, 180, 102, 216, 60, 18, 202, 103, 162, 89, 45, 31, 213, 133, 13, 206, 215, 110, 88, 167, 117, 147, 174, 213, 93, 198, 167, 11, 134, 65, 19, 246, 136, 22, 93, 35, 84, 114, 25, 61, 199, 185, 165, 231, 50, 52, 59, 198, 45, 155, 124, 209, 124, 119, 215, 186, 174, 238, 104, 138, 143, 133, 238, 244, 129, 49, 248, 227, 102, 193, 74, 242, 27, 116, 131, 51, 132, 76, 43, 19, 203, 88, 90, 202, 224, 177, 113, 75, 203, 231, 185, 21, 219, 47, 199, 29, 71, 120, 178, 224, 152, 150, 2, 47, 195, 162, 195, 218, 152, 22, 51, 180, 92, 38, 182, 107, 248, 48, 79, 58, 10, 154, 200, 84, 222, 50, 73, 211, 209, 185, 205, 112, 89, 159, 171, 83, 112, 116, 35, 182, 123, 102, 126, 51, 67, 115, 69, 75, 58, 109, 150, 154, 186, 25, 19, 50, 105, 52, 37, 205, 92, 90, 207, 48, 116, 244, 205, 31, 79, 143, 171, 83, 176, 92, 14, 5, 104, 139, 158, 211, 157, 95, 51, 172, 110, 175, 181, 87, 63, 142, 142, 125, 12, 158, 246, 142, 125, 42, 154, 177, 200, 15, 5, 45, 132, 252, 153, 222, 20, 44, 246, 163, 21, 45, 42, 84, 92, 122, 9, 26, 113, 153, 10, 31, 46, 17, 127, 75, 85, 248, 177, 64, 252, 45, 103, 80, 171, 227, 80, 176, 146, 130, 228, 71, 116, 219, 177, 93, 215, 67, 42, 126, 129, 95, 250, 177, 10, 65, 226, 192, 48, 181, 84, 37, 60, 21, 33, 44, 244, 147, 149, 43, 24, 46, 181, 184, 150, 218, 78, 48, 203, 220, 107, 25, 12, 173, 237, 29, 125, 21, 250, 227, 142, 168, 240, 102, 21, 87, 226, 42, 1, 88, 77, 128, 12, 119, 118, 107, 22, 207, 57, 146, 223, 69, 101, 64, 153, 17, 21, 29, 8, 11, 199, 157, 12, 221, 85, 57, 187, 28, 233, 57, 135, 91, 57, 205, 40, 103, 238, 122, 214, 135, 11, 34, 113, 242, 79, 68, 208, 94, 34, 21, 5, 81, 42, 114, 82, 138, 179, 148, 24, 174, 174, 91, 132, 234, 136, 75, 49, 229, 184, 19, 219, 59, 216, 75, 49, 197, 112, 181, 31, 93, 88, 195, 176, 208, 230, 53, 22, 25, 90, 218, 107, 181, 69, 221, 214, 97, 189, 168, 194, 6, 74, 48, 85, 165, 188, 141, 206, 144, 15, 215, 48, 44, 206, 212, 90, 17, 11, 42, 174, 21, 52, 181, 98, 19, 195, 2, 65, 147, 100, 252, 40, 241, 208, 62, 55, 196, 121, 131, 174, 101, 126, 11, 174, 19, 204, 211, 214, 91, 108, 207, 117, 201, 208, 86, 199, 180, 72, 224, 55, 216, 42, 66, 217, 54, 43, 129, 221, 154, 51, 226, 195, 246, 185, 9, 136, 5, 21, 55, 200, 4, 110, 156, 235, 77, 174, 247, 74, 171, 55, 147, 223, 106, 171, 241, 17, 109, 237, 134, 141, 241, 66, 214, 135, 62, 134, 165, 179, 76, 207, 172, 170, 24, 144, 246, 119, 49, 92, 251, 179, 40, 25, 154, 135, 147, 91, 4, 39, 131, 228, 202, 158, 215, 213, 30, 25, 234, 94, 58, 41, 183, 235, 249, 56, 53, 23, 46, 171, 183, 159, 122, 9, 69, 56, 164, 231, 101, 209, 18, 50, 166, 33, 154, 182, 171, 166, 15, 74, 252, 173, 85, 120, 73, 222, 161, 25, 188, 228, 68, 147, 248, 225, 25, 188, 156, 78, 73, 188, 104, 65, 180, 177, 198, 77, 107, 116, 143, 158, 229, 102, 193, 241, 129, 154, 79, 11, 89, 24, 168, 153, 118, 143, 109, 175, 10, 29, 135, 5, 110, 148, 246, 127, 29, 138, 122, 133, 202, 22, 100, 69, 254, 98, 235, 218, 115, 205, 120, 219, 123, 197, 6, 200, 227, 247, 194, 142, 197, 176, 156, 124, 237, 211, 12, 61, 165, 57, 124, 214, 89, 82, 225, 136, 46, 209, 138, 130, 192, 13, 137, 182, 177, 5, 227, 194, 248, 17, 194, 217, 243, 226, 164, 54, 121, 185, 29, 119, 8, 248, 31, 24, 20, 139, 147, 6, 183, 125, 56, 38, 9, 26, 148, 178, 138, 187, 101, 70, 247, 72, 130, 102, 166, 239, 147, 1, 222, 79, 4, 9, 232, 209, 109, 116, 137, 237, 74, 167, 251, 201, 194, 131, 146, 160, 193, 154, 105, 21, 15, 75, 67, 127, 148, 121, 207, 94, 125, 68, 218, 123, 148, 246, 110, 133, 182, 29, 71, 146, 60, 47, 219, 234, 227, 148, 81, 101, 97, 176, 144, 115, 136, 177, 170, 245, 39, 25, 34, 61, 102, 193, 72, 5, 115, 166, 19, 20, 45, 51, 88, 234, 211, 193, 202, 165, 18, 76, 91, 102, 54, 184, 250, 74, 123, 117, 151, 15, 79, 215, 220, 85, 178, 66, 10, 254, 66, 157, 56, 109, 90, 89, 205, 169, 191, 203, 15, 244, 205, 190, 223, 234, 159, 252, 103, 241, 87, 63, 158, 193, 115, 12, 225, 159, 62, 43, 123, 70, 44, 115, 92, 27, 166, 70, 40, 111, 156, 227, 126, 60, 133, 23, 24, 150, 84, 55, 201, 222, 92, 190, 224, 16, 150, 107, 89, 5, 47, 85, 154, 94, 185, 135, 74, 232, 203, 126, 156, 192, 43, 116, 13, 212, 187, 214, 20, 188, 70, 117, 23, 196, 208, 193, 171, 192, 171, 44, 187, 86, 222, 192, 155, 126, 188, 142, 183, 202, 117, 168, 85, 81, 240, 14, 67, 99, 210, 48, 197, 25, 123, 79, 220, 142, 239, 226, 253, 154, 146, 205, 164, 163, 224, 111, 212, 174, 83, 181, 59, 207, 135, 127, 48, 92, 85, 175, 233, 213, 239, 20, 167, 68, 44, 255, 100, 216, 58, 96, 6, 199, 52, 163, 192, 131, 227, 186, 51, 18, 28, 229, 71, 221, 42, 6, 237, 60, 79, 234, 105, 157, 167, 130, 122, 174, 110, 189, 137, 156, 114, 189, 79, 11, 138, 182, 10, 118, 139, 180, 223, 93, 107, 187, 210, 162, 5, 247, 214, 117, 62, 133, 15, 68, 1, 63, 36, 178, 43, 171, 189, 116, 3, 102, 196, 5, 247, 17, 131, 47, 175, 89, 54, 85, 198, 153, 167, 141, 211, 81, 63, 135, 105, 63, 62, 198, 191, 169, 154, 99, 245, 143, 162, 15, 159, 10, 120, 253, 78, 57, 133, 207, 69, 8, 95, 212, 132, 176, 205, 52, 233, 49, 72, 59, 254, 75, 234, 109, 110, 8, 165, 153, 121, 194, 160, 166, 240, 53, 190, 241, 227, 43, 124, 75, 245, 239, 161, 7, 33, 61, 3, 226, 14, 157, 188, 126, 45, 191, 71, 84, 138, 202, 212, 71, 239, 193, 129, 66, 118, 152, 91, 238, 12, 90, 168, 135, 40, 244, 76, 245, 208, 31, 61, 95, 232, 175, 69, 60, 94, 220, 145, 30, 54, 52, 42, 160, 147, 143, 133, 244, 253, 47, 192, 22, 209, 156, 151, 102, 191, 239, 12, 119, 134, 35, 225, 196, 36, 2, 103, 209, 154, 24, 152, 68, 91, 244, 12, 150, 156, 193, 178, 51, 88, 81, 196, 229, 207, 227, 131, 112, 34, 113, 240, 224, 65, 210, 139, 76, 225, 87, 69, 180, 247, 23, 17, 161, 223, 181, 209, 67, 69, 108, 12, 116, 71, 139, 216, 92, 196, 245, 66, 234, 9, 236, 32, 233, 166, 146, 180, 51, 208, 79, 210, 110, 79, 17, 113, 33, 238, 11, 252, 150, 196, 3, 165, 197, 223, 5, 110, 35, 41, 89, 146, 210, 129, 17, 146, 140, 34, 76, 33, 217, 244, 63, 86, 196, 81, 241, 127, 103, 224, 46, 146, 238, 45, 173, 60, 16, 120, 136, 164, 63, 149, 164, 199, 222, 198, 202, 254, 179, 120, 42, 225, 253, 24, 74, 98, 192, 211, 25, 15, 252, 57, 58, 133, 231, 35, 147, 120, 113, 250, 20, 26, 232, 221, 181, 22, 211, 120, 130, 94, 23, 211, 238, 200, 240, 63, 74, 185, 13, 13, 23, 73, 108, 80, 208, 168, 208, 187, 137, 41, 120, 226, 2, 150, 18, 103, 161, 50, 59, 232, 33, 110, 60, 52, 46, 61, 139, 19, 137, 254, 206, 72, 120, 18, 175, 70, 138, 120, 187, 136, 147, 167, 104, 60, 57, 77, 122, 11, 72, 123, 49, 233, 72, 163, 45, 96, 23, 73, 244, 8, 158, 79, 8, 155, 127, 167, 41, 122, 198, 149, 12, 30, 64, 19, 197, 3, 108, 12, 159, 198, 178, 192, 68, 17, 255, 58, 15, 53, 28, 152, 96, 94, 98, 103, 66, 228, 64, 11, 43, 118, 54, 138, 68, 18, 125, 158, 192, 68, 220, 27, 142, 7, 206, 116, 82, 54, 147, 152, 156, 38, 100, 131, 235, 198, 3, 214, 134, 255, 211, 143, 159, 38, 200, 167, 72, 134, 70, 122, 181, 148, 28, 5, 75, 145, 251, 168, 92, 174, 109, 55, 66, 129, 104, 162, 81, 238, 15, 186, 207, 74, 218, 225, 146, 118, 115, 184, 115, 229, 20, 206, 10, 192, 20, 62, 57, 57, 131, 17, 94, 196, 190, 241, 151, 188, 140, 151, 112, 93, 132, 19, 233, 4, 194, 43, 2, 159, 237, 156, 194, 249, 240, 33, 137, 253, 79, 45, 118, 1, 237, 202, 102, 242, 44, 176, 183, 148, 176, 121, 146, 27, 105, 188, 78, 80, 209, 73, 76, 36, 54, 121, 207, 163, 105, 153, 119, 34, 114, 30, 141, 145, 137, 85, 199, 209, 200, 102, 83, 210, 79, 197, 117, 25, 137, 204, 102, 164, 13, 158, 150, 139, 100, 208, 235, 150, 243, 25, 145, 228, 5, 18, 203, 25, 175, 162, 72, 153, 155, 99, 3, 60, 63, 2, 80, 75, 7, 8, 117, 133, 226, 197, 239, 6, 0, 0, 203, 13, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 176, 183, 163, 30, 233, 13, 0, 0, 190, 39, 0, 0, 16, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77, 69, 84, 65, 45, 73, 78, 70, 47, 76, 73, 67, 69, 78, 83, 69, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 106, 207, 203, 90, 149, 0, 0, 0, 185, 0, 0, 0, 20, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 14, 0, 0, 77, 69, 84, 65, 45, 73, 78, 70, 47, 77, 65, 78, 73, 70, 69, 83, 84, 46, 77, 70, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 227, 7, 235, 72, 33, 1, 0, 0, 112, 1, 0, 0, 49, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 15, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 65, 114, 103, 117, 109, 101, 110, 116, 69, 120, 99, 101, 112, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 173, 121, 130, 158, 141, 2, 0, 0, 219, 3, 0, 0, 38, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 16, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 79, 112, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 3, 152, 98, 196, 90, 2, 0, 0, 182, 4, 0, 0, 51, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131, 19, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 65, 102, 116, 101, 114, 79, 112, 116, 105, 111, 110, 115, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 108, 71, 224, 210, 82, 3, 0, 0, 147, 7, 0, 0, 60, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 22, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 66, 101, 102, 111, 114, 101, 70, 105, 114, 115, 116, 83, 117, 98, 67, 111, 109, 109, 97, 110, 100, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 64, 122, 198, 191, 94, 7, 0, 0, 58, 15, 0, 0, 61, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 26, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 75, 110, 111, 119, 110, 79, 112, 116, 105, 111, 110, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 106, 33, 173, 75, 75, 2, 0, 0, 151, 4, 0, 0, 60, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 33, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 77, 105, 115, 115, 105, 110, 103, 79, 112, 116, 105, 111, 110, 65, 114, 103, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 19, 65, 22, 149, 213, 2, 0, 0, 74, 5, 0, 0, 61, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 156, 36, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 79, 112, 116, 105, 111, 110, 65, 119, 97, 114, 101, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 187, 211, 122, 19, 161, 1, 0, 0, 125, 2, 0, 0, 56, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 229, 39, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 79, 112, 116, 105, 111, 110, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 1, 186, 27, 26, 33, 2, 0, 0, 102, 3, 0, 0, 51, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 245, 41, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 79, 112, 116, 105, 111, 110, 83, 116, 114, 105, 110, 103, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 214, 254, 75, 70, 171, 1, 0, 0, 206, 2, 0, 0, 50, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 44, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 102, 233, 189, 109, 114, 2, 0, 0, 199, 4, 0, 0, 63, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 148, 46, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 36, 85, 110, 107, 110, 111, 119, 110, 79, 112, 116, 105, 111, 110, 80, 97, 114, 115, 101, 114, 83, 116, 97, 116, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 22, 56, 3, 120, 233, 4, 0, 0, 210, 8, 0, 0, 38, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 124, 49, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 80, 97, 114, 115, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 174, 153, 98, 199, 130, 4, 0, 0, 82, 8, 0, 0, 38, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 54, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 80, 97, 114, 115, 101, 100, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 168, 212, 99, 26, 83, 1, 0, 0, 172, 1, 0, 0, 44, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 161, 59, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 99, 108, 105, 47, 80, 97, 114, 115, 101, 100, 67, 111, 109, 109, 97, 110, 100, 76, 105, 110, 101, 79, 112, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 185, 24, 191, 2, 32, 5, 0, 0, 86, 8, 0, 0, 51, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 61, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 102, 105, 108, 101, 47, 80, 97, 116, 104, 84, 114, 97, 118, 101, 114, 115, 97, 108, 67, 104, 101, 99, 107, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 135, 217, 45, 95, 156, 1, 0, 0, 38, 2, 0, 0, 65, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 66, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 102, 105, 108, 101, 47, 108, 111, 99, 107, 105, 110, 103, 47, 69, 120, 99, 108, 117, 115, 105, 118, 101, 70, 105, 108, 101, 65, 99, 99, 101, 115, 115, 77, 97, 110, 97, 103, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 61, 118, 253, 160, 216, 4, 0, 0, 234, 8, 0, 0, 49, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 245, 68, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 117, 116, 105, 108, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 87, 114, 97, 112, 112, 101, 114, 67, 114, 101, 100, 101, 110, 116, 105, 97, 108, 115, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 112, 242, 101, 125, 218, 1, 0, 0, 199, 2, 0, 0, 62, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 74, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 117, 116, 105, 108, 47, 105, 110, 116, 101, 114, 110, 97, 108, 47, 87, 114, 97, 112, 112, 101, 114, 68, 105, 115, 116, 114, 105, 98, 117, 116, 105, 111, 110, 85, 114, 108, 67, 111, 110, 118, 101, 114, 116, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 161, 212, 254, 253, 142, 1, 0, 0, 30, 2, 0, 0, 47, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 132, 76, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 66, 111, 111, 116, 115, 116, 114, 97, 112, 77, 97, 105, 110, 83, 116, 97, 114, 116, 101, 114, 36, 49, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 94, 224, 230, 114, 75, 3, 0, 0, 18, 5, 0, 0, 65, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 78, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 68, 111, 119, 110, 108, 111, 97, 100, 36, 68, 101, 102, 97, 117, 108, 116, 68, 111, 119, 110, 108, 111, 97, 100, 80, 114, 111, 103, 114, 101, 115, 115, 76, 105, 115, 116, 101, 110, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 31, 218, 45, 130, 5, 3, 0, 0, 65, 5, 0, 0, 52, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 82, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 68, 111, 119, 110, 108, 111, 97, 100, 36, 80, 114, 111, 120, 121, 65, 117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 111, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 221, 183, 102, 60, 240, 15, 0, 0, 225, 30, 0, 0, 33, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 171, 85, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 68, 111, 119, 110, 108, 111, 97, 100, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 138, 125, 101, 29, 216, 1, 0, 0, 178, 2, 0, 0, 45, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 243, 101, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 71, 114, 97, 100, 108, 101, 85, 115, 101, 114, 72, 111, 109, 101, 76, 111, 111, 107, 117, 112, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 43, 212, 98, 82, 247, 0, 0, 0, 44, 1, 0, 0, 49, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 104, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 71, 114, 97, 100, 108, 101, 87, 114, 97, 112, 112, 101, 114, 77, 97, 105, 110, 36, 65, 99, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 175, 160, 39, 103, 152, 18, 0, 0, 43, 39, 0, 0, 42, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 142, 105, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 71, 114, 97, 100, 108, 101, 87, 114, 97, 112, 112, 101, 114, 77, 97, 105, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 191, 14, 151, 121, 105, 1, 0, 0, 231, 1, 0, 0, 45, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 135, 124, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 73, 110, 115, 116, 97, 108, 108, 36, 73, 110, 115, 116, 97, 108, 108, 67, 104, 101, 99, 107, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 199, 214, 250, 187, 10, 29, 0, 0, 93, 57, 0, 0, 32, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 126, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 73, 110, 115, 116, 97, 108, 108, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 96, 93, 168, 45, 89, 2, 0, 0, 90, 4, 0, 0, 31, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 181, 155, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 76, 111, 103, 103, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 66, 108, 113, 207, 96, 1, 0, 0, 205, 1, 0, 0, 38, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 158, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 80, 97, 116, 104, 65, 115, 115, 101, 109, 98, 108, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 238, 159, 246, 55, 48, 4, 0, 0, 102, 7, 0, 0, 46, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 160, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 70, 105, 108, 101, 72, 97, 110, 100, 108, 101, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 232, 247, 35, 254, 73, 3, 0, 0, 90, 7, 0, 0, 45, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182, 164, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 87, 114, 97, 112, 112, 101, 114, 67, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 1, 2, 20, 0, 20, 0, 8, 8, 8, 0, 0, 0, 33, 0, 117, 133, 226, 197, 239, 6, 0, 0, 203, 13, 0, 0, 40, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 168, 0, 0, 111, 114, 103, 47, 103, 114, 97, 100, 108, 101, 47, 119, 114, 97, 112, 112, 101, 114, 47, 87, 114, 97, 112, 112, 101, 114, 69, 120, 101, 99, 117, 116, 111, 114, 46, 99, 108, 97, 115, 115, 85, 84, 5, 0, 1, 0, 0, 0, 0, 80, 75, 5, 6, 0, 0, 0, 0, 34, 0, 34, 0, 135, 13, 0, 0, 177, 175, 0, 0, 0, 0];

const TEMPLATE_ANDROID_APP_BUILD_GRADLE: &str = r##"apply plugin: 'com.android.application'
apply plugin: 'kotlin-android'

def getEnvProperty(String key, String defaultValue) {
    def envFile = file("../../../.env")
    if (envFile.exists()) {
        def lines = envFile.readLines()
        for (line in lines) {
            def trimmed = line.trim()
            if (trimmed.startsWith(key + "=")) {
                def val = trimmed.substring(key.length() + 1).trim()
                if ((val.startsWith("\"") && val.endsWith("\"")) || (val.startsWith("'") && val.endsWith("'"))) {
                    val = val.substring(1, val.length() - 1)
                }
                return val
            }
        }
    }
    return defaultValue
}

android {
    namespace 'com.rustbasic.mobile'
    compileSdk 34

    defaultConfig {
        applicationId "com.rustbasic.mobile"
        minSdk 21
        targetSdk 34
        versionCode 1
        versionName "1.0"
        testInstrumentationRunner "androidx.test.runner.AndroidJUnitRunner"
        
        resValue "integer", "app_port", getEnvProperty("APP_PORT", "4000")
        resValue "integer", "vite_port", getEnvProperty("VITE_PORT", "5173")
        resValue "string", "app_key", getEnvProperty("APP_KEY", "some_random_key_of_32_characters_long_123")
        resValue "string", "app_name", getEnvProperty("APP_NAME", "RustBasic")
    }

    buildTypes {
        release {
            minifyEnabled false
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }
    }
    compileOptions {
        sourceCompatibility JavaVersion.VERSION_1_8
        targetCompatibility JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = '1.8'
    }
    sourceSets {
        main {
            jniLibs.srcDirs = ['src/main/jniLibs']
        }
    }
}

dependencies {
    implementation 'androidx.core:core-ktx:1.12.0'
    implementation 'androidx.appcompat:appcompat:1.6.1'
    implementation 'com.google.android.material:material:1.11.0'
    implementation 'androidx.constraintlayout:constraintlayout:2.1.4'
}
"##;

const TEMPLATE_ANDROID_MANIFEST_XML: &str = r##"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <!-- 1. Internet & Konektivitas -->
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
    <uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />
    <uses-permission android:name="android.permission.CHANGE_NETWORK_STATE" />
    <uses-permission android:name="android.permission.CHANGE_WIFI_STATE" />

    <!-- 2. Komunikasi Nirkabel (Bluetooth & NFC) -->
    <uses-permission android:name="android.permission.BLUETOOTH" android:maxSdkVersion="30" />
    <uses-permission android:name="android.permission.BLUETOOTH_ADMIN" android:maxSdkVersion="30" />
    <uses-permission android:name="android.permission.BLUETOOTH_SCAN" />
    <uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />
    <uses-permission android:name="android.permission.BLUETOOTH_ADVERTISE" />
    <uses-permission android:name="android.permission.NFC" />

    <!-- 3. Lokasi/GPS (Latar Depan & Latar Belakang) -->
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
    <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
    <uses-permission android:name="android.permission.ACCESS_BACKGROUND_LOCATION" />

    <!-- 4. Sensor & Media Kamera/Audio -->
    <uses-permission android:name="android.permission.CAMERA" />
    <uses-permission android:name="android.permission.RECORD_AUDIO" />
    <uses-permission android:name="android.permission.MODIFY_AUDIO_SETTINGS" />
    <uses-permission android:name="android.permission.BODY_SENSORS" />
    <uses-permission android:name="android.permission.BODY_SENSORS_BACKGROUND" />
    <uses-permission android:name="android.permission.HIGH_SAMPLING_RATE_SENSORS" />

    <!-- 5. Notifikasi & Efek Getar -->
    <uses-permission android:name="android.permission.VIBRATE" />
    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />

    <!-- 6. Penyimpanan Tradisional (Android 12 kebawah) -->
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" android:maxSdkVersion="32" />
    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" android:maxSdkVersion="29" />

    <!-- 7. Penyimpanan Media Modern (Android 13+) -->
    <uses-permission android:name="android.permission.READ_MEDIA_IMAGES" />
    <uses-permission android:name="android.permission.READ_MEDIA_VIDEO" />
    <uses-permission android:name="android.permission.READ_MEDIA_AUDIO" />

    <!-- 8. Keamanan & Biometrik -->
    <uses-permission android:name="android.permission.USE_BIOMETRIC" />
    <uses-permission android:name="android.permission.USE_FINGERPRINT" android:maxSdkVersion="28" />

    <!-- 9. Telepon & Panggilan (Telephony) -->
    <uses-permission android:name="android.permission.READ_PHONE_STATE" />
    <uses-permission android:name="android.permission.CALL_PHONE" />

    <!-- 10. Kontak & Kalender -->
    <uses-permission android:name="android.permission.READ_CONTACTS" />
    <uses-permission android:name="android.permission.WRITE_CONTACTS" />
    <uses-permission android:name="android.permission.READ_CALENDAR" />
    <uses-permission android:name="android.permission.WRITE_CALENDAR" />

    <!-- 11. Layanan Latar Belakang & Booting (Performance) -->
    <uses-permission android:name="android.permission.WAKE_LOCK" />
    <uses-permission android:name="android.permission.RECEIVE_BOOT_COMPLETED" />
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE" />
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE_LOCATION" />
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE_DATA_SYNC" />
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK" />
    <uses-permission android:name="android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS" />
    <uses-permission android:name="android.permission.SCHEDULE_EXACT_ALARM" />

    <application
        android:allowBackup="true"
        android:label="@string/app_name"
        android:supportsRtl="true"
        android:theme="@style/Theme.AppCompat.Light.NoActionBar"
        android:usesCleartextTraffic="true">
        <activity
            android:name=".MainActivity"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>

</manifest>
"##;

const TEMPLATE_ANDROID_MAIN_ACTIVITY_KT: &str = r##"package com.rustbasic.mobile
 
import android.os.Bundle
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import android.webkit.WebChromeClient
import android.webkit.JavascriptInterface
import android.webkit.ValueCallback
import android.webkit.PermissionRequest
import android.webkit.CookieManager
import android.webkit.WebResourceRequest
import android.webkit.WebResourceError
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import android.graphics.Color
import android.graphics.Typeface
import androidx.core.view.WindowCompat
import android.view.ActionMode
import android.view.View
import android.view.ViewGroup
import android.view.Gravity
import android.widget.FrameLayout
import android.widget.LinearLayout
import android.widget.ProgressBar
import android.widget.TextView
import android.widget.Button
import android.content.Intent
import android.net.Uri
import android.animation.Animator
import android.animation.AnimatorListenerAdapter
import java.io.File
import java.net.Socket
 
class MainActivity : AppCompatActivity() {
    companion object {
        // Track server status globally in the process context to prevent double binding on activity recreation
        private var isServerStarted = false
    }

    private lateinit var webView: WebView
    private lateinit var parentLayout: FrameLayout
    private lateinit var loadingOverlay: LinearLayout
    private lateinit var errorOverlay: LinearLayout
 
    private var uploadMessage: ValueCallback<Array<Uri>>? = null
    private val FILECHOOSER_RESULTCODE = 1
    private var lastBackPressTime: Long = 0
    private var hasLoadingError = false

    private var pendingPermissionRequest: PermissionRequest? = null
    private var pendingGeolocationOrigin: String? = null
    private var pendingGeolocationCallback: android.webkit.GeolocationPermissions.Callback? = null
 
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Immersive transparent system bars (WebView spans edge-to-edge behind status and navigation bars)
        WindowCompat.setDecorFitsSystemWindows(window, false)
        window.statusBarColor = Color.TRANSPARENT
        window.navigationBarColor = Color.TRANSPARENT
 
        // Setup Programmatic Layouts
        setupProgrammaticLayout()

        // Request critical runtime permissions on startup for hardware diagnostics
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M) {
            val permissions = mutableListOf(
                android.Manifest.permission.CAMERA,
                android.Manifest.permission.RECORD_AUDIO,
                android.Manifest.permission.ACCESS_FINE_LOCATION,
                android.Manifest.permission.ACCESS_COARSE_LOCATION,
                android.Manifest.permission.BODY_SENSORS
            )
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.TIRAMISU) {
                permissions.add(android.Manifest.permission.POST_NOTIFICATIONS)
            }
            val needed = permissions.filter {
                androidx.core.content.ContextCompat.checkSelfPermission(this, it) != android.content.pm.PackageManager.PERMISSION_GRANTED
            }
            if (needed.isNotEmpty()) {
                androidx.core.app.ActivityCompat.requestPermissions(this, needed.toTypedArray(), 100)
            }
        }

        // Setup high priority Notification Channel for heads-up WhatsApp-style alerts
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
            val channelId = "rustbasic_premium_notif"
            val name = "RustBasic Notification Channel"
            val descriptionText = "High priority heads-up notifications"
            val importance = android.app.NotificationManager.IMPORTANCE_HIGH
            val channel = android.app.NotificationChannel(channelId, name, importance).apply {
                description = descriptionText
                enableLights(true)
                lightColor = Color.GREEN
                enableVibration(true)
            }
            val notificationManager = getSystemService(android.content.Context.NOTIFICATION_SERVICE) as android.app.NotificationManager
            notificationManager.createNotificationChannel(channel)
        }
 
        val webSettings = webView.settings
        webSettings.javaScriptEnabled = true
        webSettings.domStorageEnabled = true
        webSettings.databaseEnabled = true
        webSettings.mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
        webSettings.useWideViewPort = true
        webSettings.loadWithOverviewMode = true
        webSettings.setSupportZoom(false)
        webSettings.builtInZoomControls = false
        webSettings.displayZoomControls = false
        
        // Performance & Cache Tuning
        webSettings.cacheMode = WebSettings.LOAD_DEFAULT
        @Suppress("DEPRECATION")
        webSettings.databasePath = filesDir.absolutePath + "/databases"
        
        // Custom User Agent Suffix for Platform Identification
        val defaultUserAgent = webSettings.userAgentString
        webSettings.userAgentString = "$defaultUserAgent RustBasicMobileApp/1.0"
 
        // Cookie Synchronization
        val cookieManager = CookieManager.getInstance()
        cookieManager.setAcceptCookie(true)
        cookieManager.setAcceptThirdPartyCookies(webView, true)
 
        // Register Native JavaScript Bridge
        webView.addJavascriptInterface(MobileBridge(this), "MobileBridgeNative")
 
        // Setup WebChromeClient for File Uploads & Permission prompts
        webView.webChromeClient = object : WebChromeClient() {
            override fun onShowFileChooser(
                webView: WebView?,
                filePathCallback: ValueCallback<Array<Uri>>?,
                fileChooserParams: FileChooserParams?
            ): Boolean {
                if (uploadMessage != null) {
                    uploadMessage?.onReceiveValue(null)
                    uploadMessage = null
                }
                uploadMessage = filePathCallback
 
                val intent = fileChooserParams?.createIntent() ?: Intent(Intent.ACTION_GET_CONTENT).apply {
                    addCategory(Intent.CATEGORY_OPENABLE)
                    type = "*/*"
                }
 
                try {
                    startActivityForResult(intent, FILECHOOSER_RESULTCODE)
                } catch (e: Exception) {
                    uploadMessage?.onReceiveValue(null)
                    uploadMessage = null
                    Toast.makeText(this@MainActivity, "Gagal membuka file chooser", Toast.LENGTH_SHORT).show()
                    return false
                }
                return true
            }
 
            override fun onPermissionRequest(request: PermissionRequest?) {
                if (request == null) return
                val requestedResources = request.resources
                val permissionsToRequest = mutableListOf<String>()

                for (res in requestedResources) {
                    if (res == PermissionRequest.RESOURCE_VIDEO_CAPTURE) {
                        if (androidx.core.content.ContextCompat.checkSelfPermission(
                                this@MainActivity,
                                android.Manifest.permission.CAMERA
                            ) != android.content.pm.PackageManager.PERMISSION_GRANTED
                        ) {
                            permissionsToRequest.add(android.Manifest.permission.CAMERA)
                        }
                    }
                    if (res == PermissionRequest.RESOURCE_AUDIO_CAPTURE) {
                        if (androidx.core.content.ContextCompat.checkSelfPermission(
                                this@MainActivity,
                                android.Manifest.permission.RECORD_AUDIO
                            ) != android.content.pm.PackageManager.PERMISSION_GRANTED
                        ) {
                            permissionsToRequest.add(android.Manifest.permission.RECORD_AUDIO)
                        }
                    }
                }

                if (permissionsToRequest.isNotEmpty()) {
                    pendingPermissionRequest = request
                    androidx.core.app.ActivityCompat.requestPermissions(
                        this@MainActivity,
                        permissionsToRequest.toTypedArray(),
                        200
                    )
                } else {
                    runOnUiThread {
                        request.grant(requestedResources)
                    }
                }
            }

            override fun onGeolocationPermissionsShowPrompt(
                origin: String?,
                callback: android.webkit.GeolocationPermissions.Callback?
            ) {
                if (androidx.core.content.ContextCompat.checkSelfPermission(
                        this@MainActivity,
                        android.Manifest.permission.ACCESS_FINE_LOCATION
                    ) != android.content.pm.PackageManager.PERMISSION_GRANTED
                ) {
                    pendingGeolocationOrigin = origin
                    pendingGeolocationCallback = callback
                    androidx.core.app.ActivityCompat.requestPermissions(
                        this@MainActivity,
                        arrayOf(
                            android.Manifest.permission.ACCESS_FINE_LOCATION,
                            android.Manifest.permission.ACCESS_COARSE_LOCATION
                        ),
                        300
                    )
                } else {
                    callback?.invoke(origin, true, false)
                }
            }
        }
 
        webView.webViewClient = object : WebViewClient() {
            override fun onPageStarted(view: WebView?, url: String?, favicon: android.graphics.Bitmap?) {
                super.onPageStarted(view, url, favicon)
                hasLoadingError = false
            }
 
            override fun onReceivedError(
                view: WebView?,
                request: WebResourceRequest?,
                error: WebResourceError?
            ) {
                super.onReceivedError(view, request, error)
                // Trigger offline overlay only for the primary frame loading issues
                if (request?.isForMainFrame == true) {
                    hasLoadingError = true
                    showErrorLayout()
                }
            }
 
            @Suppress("DEPRECATION")
            override fun onReceivedError(
                view: WebView?,
                errorCode: Int,
                description: String?,
                failingUrl: String?
            ) {
                super.onReceivedError(view, errorCode, description, failingUrl)
                hasLoadingError = true
                showErrorLayout()
            }
 
            override fun onPageFinished(view: WebView?, url: String?) {
                super.onPageFinished(view, url)
                if (!hasLoadingError) {
                    hideLoadingLayout()
                    hideErrorLayout()
                } else {
                    showErrorLayout()
                }
 
                val css = "* { -webkit-user-select: none !important; user-select: none !important; -webkit-touch-callout: none !important; -webkit-tap-highlight-color: transparent !important; } " +
                          "input, textarea, [contenteditable] { -webkit-user-select: text !important; user-select: text !important; }"
                val js = "javascript:(function() { " +
                         "  var style = document.createElement('style'); " +
                         "  style.type = 'text/css'; " +
                         "  style.innerHTML = '$css'; " +
                         "  document.head.appendChild(style); " +
                         "  document.addEventListener('selectstart', function(e) { e.preventDefault(); }); " +
                         "  document.addEventListener('copy', function(e) { e.preventDefault(); }); " +
                         "  document.addEventListener('cut', function(e) { e.preventDefault(); }); " +
                         "  var meta = document.querySelector('meta[name=viewport]'); " +
                         "  if (meta) { " +
                         "    meta.setAttribute('content', 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover'); " +
                         "  } else { " +
                         "    meta = document.createElement('meta'); " +
                         "    meta.name = 'viewport'; " +
                         "    meta.content = 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover'; " +
                         "    document.head.appendChild(meta); " +
                         "  } " +
                         "  function getThemeColor() { " +
                         "    var m = document.querySelector('meta[name=\"theme-color\"]'); " +
                         "    if (m) return m.getAttribute('content'); " +
                         "    if (document.body) { " +
                         "      return window.getComputedStyle(document.body).backgroundColor; " +
                         "    } " +
                         "    return null; " +
                         "  } " +
                         "  function update() { " +
                         "    var col = getThemeColor(); " +
                         "    if (col && window.MobileBridgeNative && window.MobileBridgeNative.updateThemeColor) { " +
                         "      window.MobileBridgeNative.updateThemeColor(col); " +
                         "    } " +
                         "  } " +
                         "  update(); " +
                         "  var obs = new MutationObserver(update); " +
                         "  var mt = document.querySelector('meta[name=\"theme-color\"]'); " +
                         "  if (mt) { " +
                         "    obs.observe(mt, { attributes: true, attributeFilter: ['content'] }); " +
                         "  } else { " +
                         "    obs.observe(document.head, { childList: true, subtree: true }); " +
                         "  } " +
                         "})()"
                view?.evaluateJavascript(js, null)
            }
        }
 
        // Setup local database directory
        val dbFile = File(filesDir, "database.sqlite")
        if (!dbFile.exists()) {
            dbFile.createNewFile()
        }
        val dbPath = dbFile.absolutePath
        val appPort = resources.getInteger(R.integer.app_port)
        val appKey = getString(R.string.app_key)
 
        // Boot RustBasic server on background thread if not already running in this process
        if (!isServerStarted) {
            isServerStarted = true
            Thread {
                RustServer.startServer(dbPath, appPort, appKey)
            }.start()
        }
 
        // Start non-blocking socket polling to load WebView only when port becomes active
        Thread {
            var connected = false
            val timeoutMs = 15000 // 15 seconds max timeout
            val startTime = System.currentTimeMillis()
            
            while (!connected && (System.currentTimeMillis() - startTime) < timeoutMs) {
                try {
                    // Try to connect to localhost port
                    val socket = Socket("127.0.0.1", appPort)
                    socket.close()
                    connected = true
                } catch (e: Exception) {
                    // Wait 200ms before retrying
                    Thread.sleep(200)
                }
            }
            
            // Once connected or timed out, update UI on Main Thread
            runOnUiThread {
                if (connected) {
                    webView.loadUrl("http://localhost:$appPort")
                } else {
                    hasLoadingError = true
                    showErrorLayout()
                }
            }
        }.start()
    }
 
    private fun setupProgrammaticLayout() {
        parentLayout = FrameLayout(this).apply {
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT
            )
        }
 
        webView = object : WebView(this) {
            override fun startActionMode(callback: ActionMode.Callback?): ActionMode? {
                return null
            }
 
            override fun startActionMode(callback: ActionMode.Callback?, type: Int): ActionMode? {
                return null
            }
        }.apply {
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
            setBackgroundColor(Color.parseColor("#0b0f19"))
            overScrollMode = View.OVER_SCROLL_NEVER
            isVerticalScrollBarEnabled = false
            isHorizontalScrollBarEnabled = false
        }
        parentLayout.addView(webView)
 
        // Centered Circular Loading Overlay (Splash screen)
        loadingOverlay = LinearLayout(this).apply {
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
            orientation = LinearLayout.VERTICAL
            gravity = Gravity.CENTER
            setBackgroundColor(Color.parseColor("#0b0f19"))
            isClickable = true
            isFocusable = true
        }
 
        val progressSpinner = ProgressBar(this, null, android.R.attr.progressBarStyleLarge).apply {
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            ).apply {
                bottomMargin = 48
            }
        }
        loadingOverlay.addView(progressSpinner)
 
        val loadingText = TextView(this).apply {
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            )
            text = getString(R.string.app_name)
            setTextColor(Color.WHITE)
            textSize = 24f
            typeface = Typeface.DEFAULT_BOLD
        }
        loadingOverlay.addView(loadingText)
        parentLayout.addView(loadingOverlay)
 
        // Centered Native Error Overlay (Offline/Timeout View)
        errorOverlay = LinearLayout(this).apply {
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
            orientation = LinearLayout.VERTICAL
            gravity = Gravity.CENTER
            setBackgroundColor(Color.parseColor("#0b0f19"))
            visibility = View.GONE
            isClickable = true
            isFocusable = true
        }
 
        val errorTitle = TextView(this).apply {
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            ).apply {
                bottomMargin = 16
            }
            text = "Koneksi Terputus"
            setTextColor(Color.WHITE)
            textSize = 20f
            typeface = Typeface.DEFAULT_BOLD
        }
        errorOverlay.addView(errorTitle)
 
        val errorText = TextView(this).apply {
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            ).apply {
                bottomMargin = 48
                leftMargin = 48
                rightMargin = 48
            }
            text = "Gagal terhubung ke server. Pastikan koneksi internet Anda aktif dan server berjalan."
            setTextColor(Color.parseColor("#8a99ad"))
            textSize = 14f
            gravity = Gravity.CENTER
        }
        errorOverlay.addView(errorText)
 
        val retryButton = Button(this).apply {
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT,
                LinearLayout.LayoutParams.WRAP_CONTENT
            )
            text = "Coba Lagi"
            setTextColor(Color.WHITE)
            setBackgroundColor(Color.parseColor("#3b82f6"))
            setPadding(48, 24, 48, 24)
            setOnClickListener {
                hideErrorLayout()
                showLoadingLayout()
                webView.reload()
            }
        }
        errorOverlay.addView(retryButton)
        parentLayout.addView(errorOverlay)
        setContentView(parentLayout)
    }
 
    private fun showLoadingLayout() {
        runOnUiThread {
            loadingOverlay.visibility = View.VISIBLE
            loadingOverlay.alpha = 1.0f
        }
    }
 
    private fun hideLoadingLayout() {
        runOnUiThread {
            if (loadingOverlay.visibility == View.VISIBLE) {
                loadingOverlay.animate()
                    .alpha(0f)
                    .setDuration(300)
                    .setListener(object : AnimatorListenerAdapter() {
                        override fun onAnimationEnd(animation: Animator) {
                            loadingOverlay.visibility = View.GONE
                        }
                    })
            }
        }
    }
 
    private fun showErrorLayout() {
        runOnUiThread {
            errorOverlay.visibility = View.VISIBLE
            loadingOverlay.visibility = View.GONE
        }
    }
 
    private fun hideErrorLayout() {
        runOnUiThread {
            errorOverlay.visibility = View.GONE
        }
    }
 
    @Suppress("DEPRECATION")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == FILECHOOSER_RESULTCODE) {
            if (uploadMessage == null) return
            val results = WebChromeClient.FileChooserParams.parseResult(resultCode, data)
            uploadMessage?.onReceiveValue(results)
            uploadMessage = null
        }
    }
 
    // Dynamically update the status bar color based on color from WebView
    fun updateStatusBarColor(colorStr: String) {
        runOnUiThread {
            try {
                var parsedColor = Color.TRANSPARENT
                val clean = colorStr.trim()
                if (clean.startsWith("#")) {
                    parsedColor = Color.parseColor(clean)
                } else if (clean.startsWith("rgb") || clean.contains("rgb")) {
                    val values = clean.replace("rgba", "").replace("rgb", "")
                        .replace("(", "").replace(")", "").split(",")
                    if (values.size >= 3) {
                        val r = values[0].trim().toInt()
                        val g = values[1].trim().toInt()
                        val b = values[2].trim().toInt()
                        parsedColor = Color.rgb(r, g, b)
                    }
                } else {
                    parsedColor = Color.parseColor(clean)
                }
                
                if (parsedColor != Color.TRANSPARENT) {
                    val red = Color.red(parsedColor)
                    val green = Color.green(parsedColor)
                    val blue = Color.blue(parsedColor)
                    val luminance = 0.299 * red + 0.587 * green + 0.114 * blue
                    
                    val insetsController = WindowCompat.getInsetsController(window, window.decorView)
                    insetsController.isAppearanceLightStatusBars = luminance > 150
                    insetsController.isAppearanceLightNavigationBars = luminance > 150
                }
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }
 
    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == 200) {
            val request = pendingPermissionRequest
            if (request != null) {
                runOnUiThread {
                    val granted = mutableListOf<String>()
                    for (res in request.resources) {
                        if (res == PermissionRequest.RESOURCE_VIDEO_CAPTURE) {
                            if (androidx.core.content.ContextCompat.checkSelfPermission(
                                    this,
                                    android.Manifest.permission.CAMERA
                                ) == android.content.pm.PackageManager.PERMISSION_GRANTED
                            ) {
                                granted.add(res)
                            }
                        } else if (res == PermissionRequest.RESOURCE_AUDIO_CAPTURE) {
                            if (androidx.core.content.ContextCompat.checkSelfPermission(
                                    this,
                                    android.Manifest.permission.RECORD_AUDIO
                                ) == android.content.pm.PackageManager.PERMISSION_GRANTED
                            ) {
                                granted.add(res)
                            }
                        } else {
                            granted.add(res)
                        }
                    }
                    if (granted.isNotEmpty()) {
                        request.grant(granted.toTypedArray())
                    } else {
                        request.deny()
                    }
                }
                pendingPermissionRequest = null
            }
        } else if (requestCode == 300) {
            val origin = pendingGeolocationOrigin
            val callback = pendingGeolocationCallback
            if (callback != null && origin != null) {
                val granted = grantResults.isNotEmpty() && grantResults[0] == android.content.pm.PackageManager.PERMISSION_GRANTED
                callback.invoke(origin, granted, false)
                pendingGeolocationOrigin = null
                pendingGeolocationCallback = null
            }
        }
    }

    @Suppress("DEPRECATION")
    override fun onBackPressed() {
        if (webView.canGoBack()) {
            webView.goBack()
        } else {
            val currentTime = System.currentTimeMillis()
            if (currentTime - lastBackPressTime < 2000) {
                super.onBackPressed()
            } else {
                Toast.makeText(this, "Tekan sekali lagi untuk keluar", Toast.LENGTH_SHORT).show()
                lastBackPressTime = currentTime
            }
        }
    }

    fun triggerHeadsUpNotification(title: String, message: String) {
        val channelId = "rustbasic_premium_notif"
        val intent = Intent(this, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK
        }
        val pendingIntent = android.app.PendingIntent.getActivity(
            this,
            0,
            intent,
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M) {
                android.app.PendingIntent.FLAG_UPDATE_CURRENT or android.app.PendingIntent.FLAG_IMMUTABLE
            } else {
                android.app.PendingIntent.FLAG_UPDATE_CURRENT
            }
        )

        val builder = androidx.core.app.NotificationCompat.Builder(this, channelId)
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .setContentTitle(title)
            .setContentText(message)
            .setPriority(androidx.core.app.NotificationCompat.PRIORITY_HIGH)
            .setDefaults(androidx.core.app.NotificationCompat.DEFAULT_ALL)
            .setContentIntent(pendingIntent)
            .setAutoCancel(true)

        val notificationManager = getSystemService(android.content.Context.NOTIFICATION_SERVICE) as android.app.NotificationManager
        notificationManager.notify(System.currentTimeMillis().toInt(), builder.build())
    }
}
 
class MobileBridge(private val activity: MainActivity) : android.hardware.SensorEventListener {
    private val sensorManager = activity.getSystemService(android.content.Context.SENSOR_SERVICE) as android.hardware.SensorManager
    private val proximitySensor = sensorManager.getDefaultSensor(android.hardware.Sensor.TYPE_PROXIMITY)
    private var proximityValue: Float = -1f

    init {
        if (proximitySensor != null) {
            sensorManager.registerListener(this, proximitySensor, android.hardware.SensorManager.SENSOR_DELAY_NORMAL)
        }
    }

    override fun onSensorChanged(event: android.hardware.SensorEvent?) {
        if (event != null) {
            proximityValue = event.values[0]
        }
    }

    override fun onAccuracyChanged(sensor: android.hardware.Sensor?, accuracy: Int) {}

    @JavascriptInterface
    fun getGPSLocation(): String {
        val locationManager = activity.getSystemService(android.content.Context.LOCATION_SERVICE) as android.location.LocationManager
        var latitude = -6.200000
        var longitude = 106.816666
        var providerName = "fallback-default"
        var statusStr = "mock-fallback"

        try {
            if (androidx.core.content.ContextCompat.checkSelfPermission(
                    activity,
                    android.Manifest.permission.ACCESS_FINE_LOCATION
                ) == android.content.pm.PackageManager.PERMISSION_GRANTED
            ) {
                val gpsLoc = locationManager.getLastKnownLocation(android.location.LocationManager.GPS_PROVIDER)
                val netLoc = locationManager.getLastKnownLocation(android.location.LocationManager.NETWORK_PROVIDER)
                
                var finalLoc: android.location.Location? = null
                if (gpsLoc != null && netLoc != null) {
                    finalLoc = if (gpsLoc.time > netLoc.time) gpsLoc else netLoc
                } else {
                    finalLoc = gpsLoc ?: netLoc
                }

                if (finalLoc != null) {
                    latitude = finalLoc.latitude
                    longitude = finalLoc.longitude
                    providerName = finalLoc.provider ?: "gps"
                    statusStr = "success"
                } else {
                    activity.runOnUiThread {
                        try {
                            locationManager.requestSingleUpdate(
                                android.location.LocationManager.NETWORK_PROVIDER,
                                object : android.location.LocationListener {
                                    override fun onLocationChanged(location: android.location.Location) {}
                                    override fun onStatusChanged(provider: String?, status: Int, extras: android.os.Bundle?) {}
                                    override fun onProviderEnabled(provider: String) {}
                                    override fun onProviderDisabled(provider: String) {}
                                },
                                null
                            )
                        } catch (ex: Exception) {
                            ex.printStackTrace()
                        }
                    }
                    statusStr = "no-last-location"
                }
            } else {
                statusStr = "permission-denied"
            }
        } catch (e: Exception) {
            e.printStackTrace()
            statusStr = "error: ${e.message}"
        }

        return "{\"status\": \"$statusStr\", \"latitude\": $latitude, \"longitude\": $longitude, \"provider\": \"$providerName\"}"
    }

    @JavascriptInterface
    fun getDeviceSensors(): String {
        var batteryPct = 100
        var isCharging = false

        try {
            val batteryStatusIntent = activity.registerReceiver(null, android.content.IntentFilter(android.content.Intent.ACTION_BATTERY_CHANGED))
            val level = batteryStatusIntent?.getIntExtra(android.os.BatteryManager.EXTRA_LEVEL, -1) ?: -1
            val scale = batteryStatusIntent?.getIntExtra(android.os.BatteryManager.EXTRA_SCALE, -1) ?: -1
            if (level >= 0 && scale > 0) {
                batteryPct = (level * 100 / scale.toFloat()).toInt()
            }
            
            val status = batteryStatusIntent?.getIntExtra(android.os.BatteryManager.EXTRA_STATUS, -1) ?: -1
            isCharging = status == android.os.BatteryManager.BATTERY_STATUS_CHARGING || status == android.os.BatteryManager.BATTERY_STATUS_FULL
        } catch (e: Exception) {
            e.printStackTrace()
        }

        val isNear = if (proximitySensor != null && proximityValue >= 0f) {
            proximityValue < proximitySensor.maximumRange
        } else {
            false
        }

        return "{\"status\": \"success\", \"battery\": $batteryPct, \"charging\": $isCharging, \"proximity\": $isNear}"
    }

    @JavascriptInterface
    fun showToast(message: String) {
        activity.runOnUiThread {
            Toast.makeText(activity, message, Toast.LENGTH_SHORT).show()
        }
    }

    @JavascriptInterface
    fun updateThemeColor(color: String) {
        activity.updateStatusBarColor(color)
    }

    @JavascriptInterface
    fun showNotification(title: String, message: String) {
        activity.runOnUiThread {
            activity.triggerHeadsUpNotification(title, message)
        }
    }
}
"##;

const TEMPLATE_ANDROID_RUST_SERVER_KT: &str = r##"package com.rustbasic.mobile

object RustServer {
    init {
        System.loadLibrary("rustbasic_mobile")
    }

    external fun startServer(dbPath: String, port: Int, appKey: String): Int
}
"##;

const TEMPLATE_FRONTEND_NATIVE_BRIDGE_TS: &str = r##"// RustBasic Native JavaScript Bridge
// Exposes device hardware sensors for both Android & iOS WebViews with safe web fallbacks

declare global {
  interface Window {
    MobileBridge: {
      getGPSLocation(): Promise<{
        status: string;
        latitude: number;
        longitude: number;
        provider: string;
      }>;
      getDeviceSensors(): Promise<{
        status: string;
        battery: number;
        charging: boolean;
        proximity: boolean;
      }>;
      showToast(message: string): void;
      showNotification(title: string, message: string): void;
    };
    MobileBridgeNative?: {
      getGPSLocation(): string;
      getDeviceSensors(): string;
      showToast(message: string): void;
      showNotification(title: string, message: string): void;
    };
    webkit?: {
      messageHandlers: {
        getGPSLocation?: { postMessage(message: any): void };
        getDeviceSensors?: { postMessage(message: any): void };
        showToast?: { postMessage(message: any): void };
        showNotification?: { postMessage(message: any): void };
      };
    };
    ipc?: {
      postMessage(message: string): void;
    };
    _gpsResolve?: (value: any) => void;
    _gpsReject?: (reason?: any) => void;
    _sensorsResolve?: (value: any) => void;
    _sensorsReject?: (reason?: any) => void;
    onGPSLocationResult?: (data: any) => void;
    onDeviceSensorsResult?: (data: any) => void;
  }
}

window.MobileBridge = {
    // 1. GPS / Location Sensor
    getGPSLocation: function(): Promise<{ status: string; latitude: number; longitude: number; provider: string }> {
        return new Promise((resolve, reject) => {
            window._gpsResolve = resolve;
            window._gpsReject = reject;
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "getGPSLocation" }));
            } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.getGPSLocation) {
                window.webkit.messageHandlers.getGPSLocation.postMessage(null);
            } else if (window.MobileBridgeNative) {
                try {
                    let res = window.MobileBridgeNative.getGPSLocation();
                    resolve(JSON.parse(res));
                } catch(e) {
                    reject("Error parsing GPS result: " + e);
                }
            } else {
                // Fallback untuk browser web standar
                resolve({
                    status: "fallback",
                    latitude: -6.200000,
                    longitude: 106.816666,
                    provider: "web-mock"
                });
            }
        });
    },

    // 2. Device Sensors (Battery, Proximity, etc.)
    getDeviceSensors: function(): Promise<{ status: string; battery: number; charging: boolean; proximity: boolean }> {
        return new Promise((resolve, reject) => {
            window._sensorsResolve = resolve;
            window._sensorsReject = reject;
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(JSON.stringify({ action: "getDeviceSensors" }));
            } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.getDeviceSensors) {
                window.webkit.messageHandlers.getDeviceSensors.postMessage(null);
            } else if (window.MobileBridgeNative) {
                try {
                    let res = window.MobileBridgeNative.getDeviceSensors();
                    resolve(JSON.parse(res));
                } catch(e) {
                    reject("Error parsing sensors result: " + e);
                }
            } else {
                // Fallback untuk browser web standar
                resolve({
                    status: "fallback",
                    battery: 100,
                    charging: true,
                    proximity: false
                });
            }
        });
    },

    // 3. Show Toast Notification
    showToast: function(message: string): void {
        if (window.ipc && window.ipc.postMessage) {
            window.ipc.postMessage(JSON.stringify({ action: "showToast", message: message }));
        } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.showToast) {
            window.webkit.messageHandlers.showToast.postMessage(message);
        } else if (window.MobileBridgeNative) {
            window.MobileBridgeNative.showToast(message);
        } else {
            // Fallback untuk browser web standar
            console.log("Toast (Web Fallback): " + message);
        }
    },

    // 4. Show Heads-Up Notification (WhatsApp style)
    showNotification: function(title: string, message: string): void {
        if (window.ipc && window.ipc.postMessage) {
            window.ipc.postMessage(JSON.stringify({ action: "showNotification", title, message }));
        } else if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.showNotification) {
            window.webkit.messageHandlers.showNotification.postMessage({ title, message });
        } else if (window.MobileBridgeNative && window.MobileBridgeNative.showNotification) {
            window.MobileBridgeNative.showNotification(title, message);
        } else {
            // Fallback untuk browser web standar
            console.log(`Notification (Web Fallback) - Title: ${title}, Message: ${message}`);
            if ('Notification' in window) {
                if (Notification.permission === 'granted') {
                    new Notification(title, { body: message });
                } else if (Notification.permission !== 'denied') {
                    Notification.requestPermission().then(permission => {
                        if (permission === 'granted') {
                            new Notification(title, { body: message });
                        }
                    });
                }
            }
        }
    }
};

// Callbacks untuk asynchronous iOS Swift bridges
window.onGPSLocationResult = function(data: any): void {
    if (window._gpsResolve) {
        window._gpsResolve(data);
    }
};

window.onDeviceSensorsResult = function(data: any): void {
    if (window._sensorsResolve) {
        window._sensorsResolve(data);
    }
};

export {};
"##;

const TEMPLATE_REVERB_TS: &str = r##"type ListenerCallback = (data: any) => void;

class Channel {
  private name: string;
  private client: ReverbClient;
  private listeners: Record<string, ListenerCallback[]> = {};

  constructor(name: string, client: ReverbClient) {
    this.name = name;
    this.client = client;
  }

  public getName(): string {
    return this.name;
  }

  public listen(event: string, callback: ListenerCallback): this {
    if (!this.listeners[event]) {
      this.listeners[event] = [];
    }
    this.listeners[event].push(callback);
    return this;
  }

  public trigger(event: string, data: any) {
    if (this.listeners[event]) {
      this.listeners[event].forEach(cb => cb(data));
    }
  }

  /**
   * Broadcast/whisper an event to other subscribers on this channel.
   */
  public broadcast(event: string, data: any) {
    this.client.send({
      action: 'broadcast',
      channel: this.name,
      event: event,
      data: data
    });
  }
}

export class ReverbClient {
  private socket: WebSocket | null = null;
  private url: string;
  private channels: Record<string, Channel> = {};
  private pendingSubscriptions: string[] = [];
  private isConnected = false;
  private reconnectInterval = 5000;

  public onConnect: (() => void) | null = null;
  public onDisconnect: (() => void) | null = null;

  constructor(url?: string) {
    if (url) {
      this.url = url;
    } else {
      const loc = window.location;
      const proto = loc.protocol === 'https:' ? 'wss:' : 'ws:';
      this.url = `${proto}//${loc.host}/ws`;
    }
    this.connect();
  }

  private connect() {
    try {
      this.socket = new WebSocket(this.url);

      this.socket.onopen = () => {
        this.isConnected = true;
        if (this.onConnect) this.onConnect();
        this.pendingSubscriptions.forEach(channel => {
          this.sendSubscribe(channel);
        });
        this.pendingSubscriptions = [];
      };

      this.socket.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (data && data.channel && data.event) {
            const channel = this.channels[data.channel];
            if (channel) {
              channel.trigger(data.event, data.data);
            }
          }
        } catch (e) {
          // Non-JSON or malformed messages ignored
        }
      };

      this.socket.onclose = () => {
        this.isConnected = false;
        if (this.onDisconnect) this.onDisconnect();
        setTimeout(() => this.connect(), this.reconnectInterval);
      };

      this.socket.onerror = () => {
        if (this.socket) {
          this.socket.close();
        }
      };
    } catch (e) {
      setTimeout(() => this.connect(), this.reconnectInterval);
    }
  }

  private sendSubscribe(channelName: string) {
    this.send({
      action: 'subscribe',
      channel: channelName
    });
  }

  public send(payload: any) {
    if (this.socket && this.isConnected) {
      this.socket.send(JSON.stringify(payload));
    }
  }

  public subscribe(channelName: string): Channel {
    if (!this.channels[channelName]) {
      this.channels[channelName] = new Channel(channelName, this);
      if (this.isConnected) {
        this.sendSubscribe(channelName);
      } else {
        this.pendingSubscriptions.push(channelName);
      }
    }
    return this.channels[channelName];
  }

  public unsubscribe(channelName: string) {
    if (this.channels[channelName]) {
      delete this.channels[channelName];
      this.send({
        action: 'unsubscribe',
        channel: channelName
      });
    }
  }
}
"##;

const TEMPLATE_WEBSOCKET_DOC_MD: &str = r##"# WebSocket Event Broadcaster

RustBasic menyediakan sistem penyiaran event secara real-time (*WebSocket Event Broadcaster*) terpadu yang ringan dan berkinerja tinggi. Sistem ini memungkinkan backend Rust mengirimkan data secara instan (*server-to-client push*) ke halaman frontend ReactJS maupun klien eksternal secara asinkron.

---

## 🛠️ Konfigurasi (`.env`)

Fitur Broadcaster bersifat opsional dan dikonfigurasi melalui file `.env`. Untuk mengaktifkan fitur ini, tambahkan atau ubah baris berikut:

```env
# Aktifkan untuk menyalakan engine WebSocket di port aplikasi Anda pada route /ws
WEBSOCKET_ENABLED=true
```

Jika dinonaktifkan (`WEBSOCKET_ENABLED=false`), route `/ws` akan mengembalikan respons `404 Not Found` dan tidak ada koneksi WebSocket yang diterima.

---

## 📡 Penggunaan di Sisi Backend (Rust)

Untuk menyiarkan data atau pesan dari bagian manapun di backend Rust (seperti dari Controller atau Service), Anda dapat menggunakan helper statis `Broadcaster`:

```rust
use rustbasic_core::Broadcaster;
use rustbasic_core::serde_json::json;

// Menyiarkan pesan dengan payload JSON ke channel "demo-channel" dengan nama event "test-event"
Broadcaster::to("demo-channel")
    .emit("test-event", json!({
        "message": "Halo dari backend RustBasic!",
        "status": "success"
    }))
    .await;
```

---

## 💻 Penggunaan di Sisi Frontend (ReactJS / TypeScript)

Kami menyediakan klien WebSocket bawaan yang siap pakai di dalam berkas `src/resources/js/reverb.ts` (menggunakan nama modul `ReverbClient` untuk client-side helper).

### Cara Berlangganan Event:

```typescript
import { ReverbClient } from './reverb';

// 1. Inisialisasi Klien (Secara otomatis akan mendeteksi protokol ws/wss dan host saat ini)
const reverb = new ReverbClient();

// 2. Berlangganan ke Channel Tertentu
const channel = reverb.subscribe("demo-channel");

// 3. Mendengarkan Event Tertentu
channel.listen("test-event", (data) => {
    console.log("Pesan diterima:", data.message);
    
    // Contoh memicu notifikasi visual ke user
    alert(`Notifikasi Baru: ${data.message}`);
});
```

---

## 🔌 Integrasi API & Klien Eksternal (Decoupled)

Sistem WebSocket Broadcaster ini dirancang secara terpisah (*decoupled*). Klien apa pun yang mendukung protokol WebSocket standar (Aplikasi Mobile Native, Flutter, external script, dll.) dapat terhubung langsung ke server.

### 1. Endpoint Koneksi
* **URL:** `ws://<host>:<port>/ws` atau `wss://<host>:<port>/ws`

### 2. Protokol Komunikasi (JSON Frames)

#### A. Mendaftarkan Langganan (Subscribe)
Kirim payload berikut setelah koneksi WebSocket terbuka untuk mulai mendengarkan event di channel tertentu:
```json
{
  "action": "subscribe",
  "channel": "nama-channel-anda"
}
```

#### B. Membatalkan Langganan (Unsubscribe)
Kirim payload berikut untuk berhenti mendengarkan pesan dari channel tertentu:
```json
{
  "action": "unsubscribe",
  "channel": "nama-channel-anda"
}
```

#### C. Struktur Event yang Diterima Klien
Klien akan menerima pesan teks JSON dari server dengan format berikut setiap kali event dipicu di backend:
```json
{
  "event": "nama-event",
  "channel": "nama-channel",
  "data": {
    "key": "value"
  }
}
```

#### D. Mengirim Pesan dari Klien ke Klien Lain (2-Arah / Whisper)
Klien dapat menyebarkan pesan langsung via koneksi WebSocket (2-arah). Server akan otomatis mendistribusikan pesan tersebut ke seluruh pelanggan channel tersebut (kecuali si pengirim pesan itu sendiri).

Kirim payload teks JSON berikut lewat koneksi WebSocket Anda:
```json
{
  "action": "broadcast",
  "channel": "nama-channel-anda",
  "event": "nama-event",
  "data": {
    "message": "Halo dari browser client!"
  }
}
```

Or dengan memanfaatkan helper client-side JavaScript secara terintegrasi:
```typescript
channel.broadcast("test-event", { message: "Halo dari browser client!" });
```
"##;

const TEMPLATE_REACT_PERMISSION_TSX: &str = r##"import { useEffect, useState, useRef } from 'react';
import { Link, Head } from '@inertiajs/react';
import { useTheme } from '../Layouts/AppLayout';
import { useRoute } from '../route';

export default function Permission() {
  const [mounted, setMounted] = useState(false);
  const { colors, isDark } = useTheme();
  const route = useRoute();

  // Test states
  const [cameraStatus, setCameraStatus] = useState('Idle');
  const [micStatus, setMicStatus] = useState('Idle');
  const [vibrateStatus, setVibrateStatus] = useState('Idle');
  const [geoHtml5Status, setGeoHtml5Status] = useState('Idle');
  const [geoNativeStatus, setGeoNativeStatus] = useState('Idle');
  const [sensorsStatus, setSensorsStatus] = useState('Idle');
  const [notifTitle, setNotifTitle] = useState('Pesan Baru (WhatsApp Style)');
  const [notifMessage, setNotifMessage] = useState('Halo! Ini adalah notifikasi banner melayang.');

  // WebSocket test states
  const [wsStatus, setWsStatus] = useState('Disconnected');
  const [wsUrl, setWsUrl] = useState(() => {
    if (typeof window !== 'undefined') {
      const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      return `${proto}//${window.location.host}/ws`;
    }
    return 'ws://localhost:4000/ws';
  });
  const [wsChannel, setWsChannel] = useState('permission-channel');
  const [wsEvent, setWsEvent] = useState('test-event');
  const [wsPayload, setWsPayload] = useState('{"message": "Hello from React via WS!"}');
  const [wsLogs, setWsLogs] = useState<string[]>([]);
  const wsRef = useRef<WebSocket | null>(null);

  // Video reference for camera preview
  const videoRef = useRef<HTMLVideoElement>(null);
  const [stream, setStream] = useState<MediaStream | null>(null);
  const [facingMode, setFacingMode] = useState<'user' | 'environment'>('user');
 
  useEffect(() => {
    setMounted(true);
    return () => {
      // Clean up camera stream on unmount
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
      }
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [stream]);

  useEffect(() => {
    if (stream && videoRef.current) {
      videoRef.current.srcObject = stream;
      videoRef.current.play().catch(err => {
        console.error("Video play failed:", err);
      });
    }
  }, [stream]);

  // 1. Camera Test
  const testCamera = async (mode: 'user' | 'environment' = facingMode) => {
    setCameraStatus('Requesting...');
    try {
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
        setStream(null);
      }
      const media = await navigator.mediaDevices.getUserMedia({ video: { facingMode: mode } });
      setStream(media);
      setCameraStatus(`Granted (Live Stream Active - Kamera ${mode === 'user' ? 'Depan' : 'Belakang'})`);
    } catch (err: any) {
      setCameraStatus(`Denied/Error: ${err.message}`);
    }
  };

  const switchCamera = () => {
    const nextMode = facingMode === 'user' ? 'environment' : 'user';
    setFacingMode(nextMode);
    if (stream) {
      testCamera(nextMode);
    }
  };

  // 2. Microphone Test
  const testMicrophone = async () => {
    setMicStatus('Requesting...');
    try {
      const media = await navigator.mediaDevices.getUserMedia({ audio: true });
      media.getTracks().forEach(track => track.stop()); // Stop immediately after testing
      setMicStatus('Granted (Access Success)');
    } catch (err: any) {
      setMicStatus(`Denied/Error: ${err.message}`);
    }
  };

  // 3. Vibration Test
  const testVibration = () => {
    if ('vibrate' in navigator) {
      navigator.vibrate([100, 50, 100]);
      setVibrateStatus('Triggered successfully');
    } else {
      setVibrateStatus('Not supported by this browser/device');
    }
  };

  // 4. Geolocation HTML5 Test
  const testGeoHtml5 = () => {
    setGeoHtml5Status('Requesting...');
    if (!navigator.geolocation) {
      setGeoHtml5Status('Not supported by browser');
      return;
    }
    navigator.geolocation.getCurrentPosition(
      (pos) => {
        setGeoHtml5Status(`Granted: Lat ${pos.coords.latitude.toFixed(6)}, Lng ${pos.coords.longitude.toFixed(6)}`);
      },
      (err) => {
        setGeoHtml5Status(`Denied/Error: ${err.message}`);
      },
      { enableHighAccuracy: true, timeout: 5000 }
    );
  };

  // 5. Geolocation Native (Bridge) Test
  const testGeoNative = async () => {
    setGeoNativeStatus('Requesting via Bridge...');
    if (window.MobileBridge && window.MobileBridge.getGPSLocation) {
      try {
        const res = await window.MobileBridge.getGPSLocation();
        setGeoNativeStatus(`Success: Lat ${res.latitude.toFixed(6)}, Lng ${res.longitude.toFixed(6)} (Provider: ${res.provider})`);
      } catch (err: any) {
        setGeoNativeStatus(`Error: ${err}`);
      }
    } else {
      setGeoNativeStatus('Native Bridge not available (running in pure Web context)');
    }
  };

  // 6. Device Sensors (Bridge) Test
  const testSensors = async () => {
    setSensorsStatus('Reading sensors...');
    if (window.MobileBridge && window.MobileBridge.getDeviceSensors) {
      try {
        const res = await window.MobileBridge.getDeviceSensors();
        setSensorsStatus(`Success: Baterai ${res.battery}%, Charging: ${res.charging ? 'Ya' : 'Tidak'}, Proximity: ${res.proximity ? 'Dekat' : 'Jauh'}`);
      } catch (err: any) {
        setSensorsStatus(`Error: ${err}`);
      }
    } else {
      setSensorsStatus('Native Bridge not available');
    }
  };

  // 7. Show Native Toast
  const showNativeToast = () => {
    if (window.MobileBridge && window.MobileBridge.showToast) {
      window.MobileBridge.showToast('Pesan Toast dari halaman ReactJS!');
    } else {
      alert('Native Toast: Toast ini dikirim dari React (Fallback Web)!');
    }
  };

  // 8. Show Native Heads-up Notification
  const triggerNativeNotification = () => {
    if (window.MobileBridge && window.MobileBridge.showNotification) {
      window.MobileBridge.showNotification(notifTitle, notifMessage);
    } else {
      // Standard browser Notification trigger fallback
      if ('Notification' in window) {
        if (Notification.permission === 'granted') {
          new Notification(notifTitle, { body: notifMessage });
        } else {
          Notification.requestPermission().then(perm => {
            if (perm === 'granted') {
              new Notification(notifTitle, { body: notifMessage });
            } else {
              alert(`Fallback Browser Notification:\nTitle: ${notifTitle}\nMessage: ${notifMessage}`);
            }
          });
        }
      } else {
        alert(`Fallback Web Notification:\nTitle: ${notifTitle}\nBody: ${notifMessage}`);
      }
    }
  };

  // 9. WebSocket Broadcaster & Whisper helpers
  const connectWS = () => {
    if (wsRef.current) {
      wsRef.current.close();
    }
    
    setWsStatus('Connecting...');
    addWsLog(`Connecting to ${wsUrl}...`);
    
    try {
      const socket = new WebSocket(wsUrl);
      wsRef.current = socket;
      
      socket.onopen = () => {
        setWsStatus('Connected');
        addWsLog('Connected successfully!');
        // Subscribe to channel
        const subMsg = {
          action: "subscribe",
          channel: wsChannel
        };
        socket.send(JSON.stringify(subMsg));
        addWsLog(`Subscribed to channel: ${wsChannel}`);
      };
      
      socket.onmessage = (event) => {
        addWsLog(`Received: ${event.data}`);
      };
      
      socket.onclose = (e) => {
        setWsStatus('Disconnected');
        addWsLog(`Disconnected: ${e.reason || 'No reason provided'} (code: ${e.code})`);
      };
      
      socket.onerror = () => {
        addWsLog(`Error: Connection failed`);
      };
    } catch (err: any) {
      setWsStatus('Error');
      addWsLog(`Exception: ${err.message}`);
    }
  };
  
  const disconnectWS = () => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
  };
  
  const sendWhisper = () => {
    if (!wsRef.current || wsRef.current.readyState !== WebSocket.OPEN) {
      addWsLog('Error: WebSocket tidak terhubung!');
      return;
    }
    
    let parsedPayload;
    try {
      parsedPayload = JSON.parse(wsPayload);
    } catch (err) {
      parsedPayload = { message: wsPayload };
    }
    
    try {
      const whisperMsg = {
        action: "broadcast",
        event: wsEvent,
        channel: wsChannel,
        data: parsedPayload
      };
      wsRef.current.send(JSON.stringify(whisperMsg));
      addWsLog(`Whispered on ${wsChannel} [${wsEvent}]: ${JSON.stringify(parsedPayload)}`);
    } catch (err: any) {
      addWsLog(`Error sending whisper: ${err.message}`);
    }
  };
  
  const addWsLog = (msg: string) => {
    const timestamp = new Date().toLocaleTimeString();
    setWsLogs(prev => [`[${timestamp}] ${msg}`, ...prev]);
  };

  const getStatusColor = (status: string) => {
    if (status === 'Idle') return colors.textMuted;
    if (status.includes('Requesting') || status.includes('Reading')) return '#f59e0b'; // Amber
    if (status.includes('Granted') || status.includes('Success') || status.includes('Triggered')) return '#10b981'; // Emerald
    return '#ef4444'; // Red
  };

  return (
    <>
      <Head>
        <title>Uji Izin Perangkat - RustBasic Native</title>
        <meta name="description" content="Halaman khusus untuk menguji semua izin perangkat keras Android dan JNI Bridge komunikasi." />
      </Head>

      <main style={{
        maxWidth: '1280px',
        margin: '0 auto',
        padding: '60px 24px 80px',
        flexGrow: 1,
        width: '100%',
        boxSizing: 'border-box',
      }}>
        <div style={{
          display: 'flex',
          flexDirection: 'column',
          gap: '32px',
        }}>
          {/* Header */}
          <div style={{
            opacity: mounted ? 1 : 0,
            transform: mounted ? 'translateY(0)' : 'translateY(20px)',
            transition: 'all 0.6s ease',
          }}>
            <div style={{
              display: 'inline-flex',
              padding: '8px 20px',
              borderRadius: '999px',
              fontSize: '0.8rem',
              fontWeight: 700,
              background: isDark ? 'rgba(59,130,246,0.08)' : 'rgba(59,130,246,0.05)',
              border: `1px solid ${isDark ? 'rgba(59,130,246,0.15)' : 'rgba(59,130,246,0.1)'}`,
              color: '#3b82f6',
              marginBottom: '16px',
            }}>
              📱 Hardware & Permission Diagnostics
            </div>
            <h1 style={{
              fontSize: 'clamp(2rem, 4vw, 3rem)',
              fontWeight: 900,
              letterSpacing: '-0.03em',
              lineHeight: 1.1,
              margin: '0 0 16px 0',
              color: colors.textMain,
            }}>
              Uji Izin Perangkat
            </h1>
            <p style={{
              fontSize: '1.1rem',
              color: colors.textMuted,
              lineHeight: 1.7,
              margin: 0,
            }}>
              Gunakan halaman ini untuk memverifikasi fungsionalitas izin perangkat keras Android (Kamera, Audio, GPS, Sensor) baik menggunakan standar HTML5 maupun JNI Bridge.
            </p>
          </div>

          {/* Test Grid */}
          <div style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))',
            gap: '20px',
            opacity: mounted ? 1 : 0,
            transform: mounted ? 'translateY(0)' : 'translateY(20px)',
            transition: 'all 0.6s ease 0.15s',
          }}>
            
            {/* 1. Kamera */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>📷 Kamera (Camera)</h3>
              <p style={cardDescStyle(colors)}>Meminta izin akses modul kamera depan/belakang dan menampilkan preview streaming secara dinamis.</p>
              
              {stream && (
                <div style={{ width: '100%', borderRadius: '12px', overflow: 'hidden', marginBottom: '16px', background: '#000', border: '1px solid rgba(255,255,255,0.1)' }}>
                  <video ref={videoRef} autoPlay playsInline muted style={{ width: '100%', height: '180px', objectFit: 'cover' }} />
                </div>
              )}
              
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={() => testCamera()} style={btnStyle('#3b82f6')}>Uji Kamera</button>
                {stream && (
                  <button onClick={switchCamera} style={btnStyle('#8b5cf6')}>
                    Putar ke {facingMode === 'user' ? 'Belakang' : 'Depan'}
                  </button>
                )}
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ fontWeight: 700, color: getStatusColor(cameraStatus) }}>{cameraStatus}</span>
              </div>
            </div>

            {/* 2. Mikrofon */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>🎙️ Mikrofon (Audio)</h3>
              <p style={cardDescStyle(colors)}>Meminta izin merekam audio dan mengakses hardware input suara perangkat.</p>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={testMicrophone} style={btnStyle('#10b981')}>Uji Mikrofon</button>
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ fontWeight: 700, color: getStatusColor(micStatus) }}>{micStatus}</span>
              </div>
            </div>

            {/* 3. Getar */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>📳 Efek Getar (Vibration)</h3>
              <p style={cardDescStyle(colors)}>Menguji akses motor getaran internal ponsel (haptic feedback) menggunakan API standar HTML5 browser.</p>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={testVibration} style={btnStyle('#8b5cf6')}>Vibrate Ponsel</button>
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ fontWeight: 700, color: getStatusColor(vibrateStatus) }}>{vibrateStatus}</span>
              </div>
            </div>

            {/* 4. Lokasi HTML5 */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>📍 Geolocation (HTML5)</h3>
              <p style={cardDescStyle(colors)}>Mendapatkan koordinat lintang/bujur melalui API lokasi browser standar.</p>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={testGeoHtml5} style={btnStyle('#e25b12')}>Uji GPS (Web)</button>
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ fontWeight: 700, color: getStatusColor(geoHtml5Status) }}>{geoHtml5Status}</span>
              </div>
            </div>

            {/* 5. Lokasi Native (Bridge) */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>🛰️ Geolocation (Native JNI)</h3>
              <p style={cardDescStyle(colors)}>Mendapatkan data GPS secara native langsung dengan memanggil Android Location Service JNI bridge.</p>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={testGeoNative} style={btnStyle('#ec4899')}>Uji GPS (Native)</button>
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ fontWeight: 700, color: getStatusColor(geoNativeStatus) }}>{geoNativeStatus}</span>
              </div>
            </div>

            {/* 6. Sensor Perangkat */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>📊 Sensor Baterai & Proximity</h3>
              <p style={cardDescStyle(colors)}>Membaca status baterai, pengisian daya, dan sensor jarak fisik wajah (proximity) via JNI.</p>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={testSensors} style={btnStyle('#f59e0b')}>Baca Sensor</button>
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ fontWeight: 700, color: getStatusColor(sensorsStatus) }}>{sensorsStatus}</span>
              </div>
            </div>

            {/* 7. Notifikasi Toast */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>💬 Notification Toast</h3>
              <p style={cardDescStyle(colors)}>Mengirim perintah native dialog toast dari lapisan web React untuk memicu alert popup Android.</p>
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={showNativeToast} style={btnStyle('#06b6d4')}>Tampilkan Toast</button>
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ color: colors.textMuted }}>Tekan tombol untuk menguji popup</span>
              </div>
            </div>

            {/* 8. WhatsApp-style Heads-Up Notification */}
            <div style={cardStyle(colors)}>
              <h3 style={cardTitleStyle(colors)}>🔔 Notifikasi Banner (WhatsApp Style)</h3>
              <p style={cardDescStyle(colors)}>Menguji notifikasi melayang di atas layar (Heads-up banner) yang meniru gaya pesan WhatsApp masuk.</p>
              
              <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginBottom: '12px' }}>
                <input
                  type="text"
                  value={notifTitle}
                  onChange={(e) => setNotifTitle(e.target.value)}
                  placeholder="Judul Notifikasi"
                  style={{
                    padding: '10px 12px',
                    borderRadius: '10px',
                    border: `1px solid ${colors.cardBorder}`,
                    background: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.02)',
                    color: colors.textMain,
                    fontSize: '0.85rem',
                    outline: 'none',
                    transition: 'border-color 0.2s',
                  }}
                />
                <input
                  type="text"
                  value={notifMessage}
                  onChange={(e) => setNotifMessage(e.target.value)}
                  placeholder="Isi Pesan Notifikasi"
                  style={{
                    padding: '10px 12px',
                    borderRadius: '10px',
                    border: `1px solid ${colors.cardBorder}`,
                    background: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.02)',
                    color: colors.textMain,
                    fontSize: '0.85rem',
                    outline: 'none',
                    transition: 'border-color 0.2s',
                  }}
                />
              </div>

              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginTop: 'auto' }}>
                <button onClick={triggerNativeNotification} style={btnStyle('#ef4444')}>Kirim Notifikasi</button>
              </div>
              <div style={statusContainerStyle(colors)}>
                Status: <span style={{ color: colors.textMuted }}>Masukkan teks dan tekan Kirim</span>
              </div>
            </div>

            {/* 9. WebSocket Event Broadcaster */}
            <div style={{ ...cardStyle(colors), gridColumn: 'span 2' }}>
              <h3 style={cardTitleStyle(colors)}>📡 WebSocket Broadcaster & Whisper (2-Arah)</h3>
              <p style={cardDescStyle(colors)}>Menguji hubungan real-time WebSocket Event Broadcaster & Whisper 2-Arah secara lokal.</p>
              
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px', marginBottom: '16px' }}>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                  <label style={{ fontSize: '0.8rem', fontWeight: 600, color: colors.textMuted }}>Server URL</label>
                  <input
                    type="text"
                    value={wsUrl}
                    onChange={(e) => setWsUrl(e.target.value)}
                    style={{
                      padding: '10px 12px',
                      borderRadius: '10px',
                      border: `1px solid ${colors.cardBorder}`,
                      background: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.02)',
                      color: colors.textMain,
                      fontSize: '0.85rem',
                      outline: 'none',
                    }}
                  />
                  
                  <label style={{ fontSize: '0.8rem', fontWeight: 600, color: colors.textMuted }}>Channel</label>
                  <input
                    type="text"
                    value={wsChannel}
                    onChange={(e) => setWsChannel(e.target.value)}
                    style={{
                      padding: '10px 12px',
                      borderRadius: '10px',
                      border: `1px solid ${colors.cardBorder}`,
                      background: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.02)',
                      color: colors.textMain,
                      fontSize: '0.85rem',
                      outline: 'none',
                    }}
                  />
                  
                  <label style={{ fontSize: '0.8rem', fontWeight: 600, color: colors.textMuted }}>Event Name</label>
                  <input
                    type="text"
                    value={wsEvent}
                    onChange={(e) => setWsEvent(e.target.value)}
                    style={{
                      padding: '10px 12px',
                      borderRadius: '10px',
                      border: `1px solid ${colors.cardBorder}`,
                      background: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.02)',
                      color: colors.textMain,
                      fontSize: '0.85rem',
                      outline: 'none',
                    }}
                  />
                </div>
                
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                  <label style={{ fontSize: '0.8rem', fontWeight: 600, color: colors.textMuted }}>JSON Message Payload</label>
                  <textarea
                    value={wsPayload}
                    onChange={(e) => setWsPayload(e.target.value)}
                    style={{
                      padding: '10px 12px',
                      borderRadius: '10px',
                      border: `1px solid ${colors.cardBorder}`,
                      background: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.02)',
                      color: colors.textMain,
                      fontSize: '0.85rem',
                      outline: 'none',
                      height: '130px',
                      fontFamily: 'monospace',
                    }}
                  />
                </div>
              </div>
              
              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginBottom: '16px' }}>
                {wsStatus !== 'Connected' ? (
                  <button onClick={connectWS} style={btnStyle('#3b82f6')}>Hubungkan (Connect)</button>
                ) : (
                  <button onClick={disconnectWS} style={btnStyle('#ef4444')}>Putus Koneksi (Disconnect)</button>
                )}
                <button onClick={sendWhisper} disabled={wsStatus !== 'Connected'} style={btnStyle(wsStatus === 'Connected' ? '#10b981' : '#6b7280')}>Kirim Bisikan (Send Whisper)</button>
                <button onClick={() => setWsLogs([])} style={btnStyle('rgba(156, 163, 175, 0.2)')}>Bersihkan Log</button>
              </div>
              
              <div style={statusContainerStyle(colors)}>
                Status Koneksi: <span style={{ fontWeight: 700, color: wsStatus === 'Connected' ? '#10b981' : '#ef4444' }}>{wsStatus}</span>
              </div>
              
              <div style={{ marginTop: '12px' }}>
                <div style={{ fontSize: '0.8rem', fontWeight: 700, marginBottom: '6px', color: colors.textMain }}>Logs Kontrol:</div>
                <div style={{
                  background: isDark ? '#0f172a' : '#f1f5f9',
                  borderRadius: '10px',
                  padding: '12px',
                  fontSize: '0.75rem',
                  fontFamily: 'monospace',
                  color: isDark ? '#38bdf8' : '#0369a1',
                  height: '120px',
                  overflowY: 'auto',
                  border: `1px solid ${colors.cardBorder}`,
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '4px',
                }}>
                  {wsLogs.length === 0 ? (
                    <span style={{ color: colors.textMuted }}>Belum ada log pesan...</span>
                  ) : (
                    wsLogs.map((log, index) => <div key={index}>{log}</div>)
                  )}
                </div>
              </div>
            </div>

          </div>

          {/* Action Footer */}
          <div style={{
            display: 'flex',
            gap: '16px',
            opacity: mounted ? 1 : 0,
            transition: 'all 0.6s ease 0.3s',
          }}>
            <Link
              href={route('home')}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                padding: '14px 28px',
                borderRadius: '14px',
                background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                fontWeight: 700,
                fontSize: '0.9rem',
                color: isDark ? '#ccc' : '#444',
                textDecoration: 'none',
                border: `1px solid ${colors.cardBorder}`,
                transition: 'all 0.3s ease',
              }}
            >
              ← Kembali ke Beranda
            </Link>
          </div>
        </div>
      </main>
    </>
  );
}

// Styling Helper functions for clean look
const cardStyle = (colors: any) => ({
  padding: '24px',
  borderRadius: '20px',
  border: `1px solid ${colors.cardBorder}`,
  background: colors.cardBg,
  display: 'flex',
  flexDirection: 'column' as const,
  gap: '12px',
  transition: 'all 0.3s ease, border-color 0.3s, background-color 0.3s',
});

const cardTitleStyle = (colors: any) => ({
  fontSize: '1.1rem',
  fontWeight: 800,
  margin: '0',
  color: colors.textMain,
});

const cardDescStyle = (colors: any) => ({
  fontSize: '0.85rem',
  color: colors.textMuted,
  lineHeight: 1.5,
  margin: '0 0 12px 0',
});

const statusContainerStyle = (colors: any) => ({
  marginTop: '16px',
  paddingTop: '12px',
  borderTop: `1px dashed ${colors.cardBorder}`,
  fontSize: '0.8rem',
  color: colors.textMuted,
});

const btnStyle = (bgColor: string) => ({
  padding: '10px 20px',
  borderRadius: '10px',
  border: 'none',
  background: bgColor,
  color: '#fff',
  fontWeight: 700,
  fontSize: '0.8rem',
  cursor: 'pointer',
  transition: 'opacity 0.2s',
  outline: 'none',
});
"##;
