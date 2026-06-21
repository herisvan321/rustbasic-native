// Re-export rustbasic_core modules for use in the setup_native! macro
pub mod scaffolding;
pub use rustbasic_core::tokio;
pub use rustbasic_core::Config;
pub use rustbasic_core::database;
pub use rustbasic_core::session;
pub use rustbasic_core::view;
pub use rustbasic_core::server;
pub use rustbasic_core::logger;

#[cfg(target_os = "android")]
pub use rustbasic_core::jni;

#[macro_export]
macro_rules! setup_native {
    () => {
        // C-FFI for iOS and direct static linking
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn rustbasic_mobile_start_server(
            db_path: *const std::os::raw::c_char,
            port: u16,
            app_key: *const std::os::raw::c_char,
        ) -> i32 {
            static SERVER_RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            if SERVER_RUNNING.swap(true, std::sync::atomic::Ordering::SeqCst) {
                // Server is already running in this process, ignore double-start request
                return 0;
            }

            if db_path.is_null() || app_key.is_null() {
                SERVER_RUNNING.store(false, std::sync::atomic::Ordering::SeqCst);
                return -1;
            }
            let db_path_str = std::ffi::CStr::from_ptr(db_path).to_string_lossy().into_owned();
            let app_key_str = std::ffi::CStr::from_ptr(app_key).to_string_lossy().into_owned();
            
            std::thread::spawn(move || {
                // Set environment variables dynamically for mobile context
                std::env::set_var("DATABASE_URL", format!("sqlite://{}", db_path_str));
                std::env::set_var("APP_PORT", port.to_string());
                std::env::set_var("APP_KEY", app_key_str);
                std::env::set_var("APP_ENV", "production");

                // Start Tokio runtime and run the server
                let rt = $crate::tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    // Initialize Logger
                    let _guard = $crate::logger::init();
                    
                    // Load Config
                    let cfg = $crate::Config::load();
                    
                    // Setup Database
                    let db = $crate::database::connect(&cfg).await;
                    
                    // Setup Sessions
                    $crate::session::init_sessions(&cfg).await;
                    let session_store = $crate::session::setup_session(&cfg).await;
                    
                    // Build Router
                    let app_router = crate::routes::build_router();
                    
                    // Inject embedded files
                    $crate::view::set_embedded_templates(crate::config::app::EmbeddedTemplates::get);
                    $crate::server::set_embedded_public(crate::config::app::EmbeddedPublic::get);
                    
                    // Start Server
                    $crate::server::start_server(cfg, session_store, db, app_router).await;
                });
            });
            
            0
        }

        // JNI interface for Android
        #[cfg(target_os = "android")]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn Java_com_rustbasic_mobile_RustServer_startServer(
            mut env: $crate::jni::JNIEnv,
            _class: $crate::jni::objects::JClass,
            db_path: $crate::jni::objects::JString,
            port: i32,
            app_key: $crate::jni::objects::JString,
        ) -> i32 {
            // Redirect stdout/stderr to Logcat
            $crate::redirect_stdout_stderr();
            
            let db_path_str: String = match env.get_string(&db_path) {
                Ok(s) => s.into(),
                Err(_) => return -2,
            };
            
            let app_key_str: String = match env.get_string(&app_key) {
                Ok(s) => s.into(),
                Err(_) => return -3,
            };

            let db_path_c = std::ffi::CString::new(db_path_str).unwrap();
            let app_key_c = std::ffi::CString::new(app_key_str).unwrap();

            rustbasic_mobile_start_server(db_path_c.as_ptr(), port as u16, app_key_c.as_ptr())
        }
    };
}

// Redirect stdout and stderr to Android Logcat
#[cfg(target_os = "android")]
pub unsafe fn redirect_stdout_stderr() {
    use std::os::raw::c_char;
    use std::ffi::CString;
    
    unsafe extern "C" {
        fn __android_log_write(prio: std::os::raw::c_int, tag: *const c_char, text: *const c_char) -> std::os::raw::c_int;
    }

    let mut pipes = [0; 2];
    unsafe {
        if libc::pipe(pipes.as_mut_ptr()) == 0 {
            let read_fd = pipes[0];
            let write_fd = pipes[1];
            
            libc::dup2(write_fd, libc::STDOUT_FILENO);
            libc::dup2(write_fd, libc::STDERR_FILENO);
            
            std::thread::spawn(move || {
                use std::os::unix::io::FromRawFd;
                use std::fs::File;
                use std::io::{BufRead, BufReader};
                
                let file = File::from_raw_fd(read_fd);
                let reader = BufReader::new(file);
                let tag = CString::new("RustBasicServer").unwrap();
                
                for line in reader.lines() {
                    if let Ok(content) = line {
                        let text = CString::new(content).unwrap();
                        __android_log_write(4, tag.as_ptr(), text.as_ptr());
                    }
                }
            });
        }
    }
}

// Aliases for compatibility
#[macro_export]
macro_rules! setup_mobile {
    () => {
        $crate::setup_native!();
    };
}
