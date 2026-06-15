# RustBasic Native (Mobile & Desktop Wrappers)

`rustbasic-native` adalah library resmi untuk framework **RustBasic** yang memungkinkan Anda menjalankan aplikasi full-stack RustBasic (Rust Backend + React/Inertia Frontend) sebagai aplikasi native di perangkat **Mobile (Android, iOS)** dan **Desktop (Windows, macOS, Linux)**.

---

## 🏗️ Arsitektur Sistem

1. **Local Server Execution**: Aplikasi RustBasic (termasuk routing, seeder, migrasi, dan file static Inertia React yang ditanamkan) dikompilasi menjadi library native (`.so` untuk Android, `.a` untuk iOS) atau binary tunggal untuk Desktop.
2. **Background Thread**: Wrapper native (Kotlin, Swift, atau Wry) memutar server RustBasic di background thread secara lokal di perangkat, lalu menghubungkan WebView ke `http://localhost:8080`.
3. **JS-Native Sensor Bridge (`native-bridge.js`)**: WebView menyuntikkan bridge Javascript Promise-based agar frontend React dapat mengakses hardware native (seperti GPS, status baterai, dan toast notifikasi) secara penuh.
4. **Core Dependencies**: Semua pustaka grafis dan platform yang berat (seperti `wry` dan `jni`) dikelola secara terpusat sebagai fitur opsional di `rustbasic-core` untuk menjaga build desktop/server utama tetap ringan.

---

## 🚀 Cara Instalasi

Jalankan perintah berikut menggunakan terminal di direktori proyek starter RustBasic Anda:

```bash
rustbasic install rustbasic-native
```

Scaffolding ini akan otomatis:
1. Menambahkan `rustbasic-native` ke dalam dependencies `Cargo.toml` Anda.
2. Mengonfigurasi `crate-type = ["rlib", "cdylib", "staticlib"]` di `Cargo.toml`.
3. Menyisipkan macro `rustbasic_native::setup_native!()` di paling bawah `src/lib.rs`.
4. Membuat folder `native/` di root proyek Anda.

---

## 📂 Struktur Direktori Scaffolding (`native/`)

Folder `native/` yang digenerate berisi struktur proyek berikut:
* **`desktop/`**: Project Rust mandiri menggunakan `wry` WebView untuk membuka aplikasi di desktop Windows/macOS/Linux.
* **`android/`**: Project Android Studio menggunakan WebView & Kotlin.
* **`ios/`**: Project Xcode menggunakan WKWebView & Swift.
* **`native-bridge.js`**: Helper JavaScript Promise-based untuk berinteraksi dengan sensor native.
* **`run-desktop.sh`**: Skrip cepat untuk menjalankan aplikasi desktop.
* **`build-android.sh`**: Skrip kompilasi library Android JNI (`.so`).
* **`build-ios.sh`**: Skrip kompilasi static library iOS (`.a`).

---

## 🛠️ Cara Mengompilasi & Menjalankan

### 1. Desktop (Windows, macOS, Linux) 🖥️
Pastikan Anda berada di direktori proyek utama, lalu jalankan perintah:
```bash
./native/run-desktop.sh
```
Perintah ini akan menyalakan server lokal di background dan membuka jendela aplikasi desktop native dengan WebView terintegrasi.

### 2. Android 🤖
Jalankan kompilasi untuk target arsitektur Android:
```bash
./native/build-android.sh
```
Setelah proses build selesai:
1. Buka folder `native/android` menggunakan **Android Studio**.
2. Hubungkan HP Android atau jalankan emulator.
3. Klik **Run** di Android Studio untuk menginstal aplikasi.

### 3. iOS (macOS Only) 🍏
Jalankan kompilasi untuk target iOS Simulator:
```bash
./native/build-ios.sh --sim
```
*(Gunakan `./native/build-ios.sh --device` jika Anda ingin mengompilasi untuk HP iPhone fisik).*

Setelah proses selesai:
1. Buka file `native/ios/RustBasicMobile.xcodeproj` menggunakan **Xcode**.
2. Pilih simulator target atau perangkat Anda.
3. Klik tombol **Play/Run** di Xcode.

---

## 🔌 Menggunakan Sensor Perangkat di React (Frontend)

Hubungkan antarmuka React Anda ke hardware native menggunakan objek `window.MobileBridge` yang disediakan oleh `native-bridge.js`.

### Contoh Memanggil Lokasi GPS perangkat:
```javascript
const ambilLokasiGPS = async () => {
    try {
        // Memanggil sensor GPS native secara asinkron
        const lokasi = await window.MobileBridge.getGPSLocation();
        
        console.log("Latitude: " + lokasi.latitude);
        console.log("Longitude: " + lokasi.longitude);
        console.log("Provider: " + lokasi.provider); // "ios", "native", dll.

        // Menampilkan pesan pop-up Toast native di layar HP
        window.MobileBridge.showToast(`Koordinat: ${lokasi.latitude}, ${lokasi.longitude}`);
    } catch (error) {
        console.error("Gagal mendapatkan GPS:", error);
    }
};
```

### Contoh Memanggil Sensor Baterai perangkat:
```javascript
const cekBaterai = async () => {
    try {
        const info = await window.MobileBridge.getDeviceSensors();
        console.log("Persentase Baterai: " + info.battery + "%");
        console.log("Sedang Di-charge: " + info.charging);
    } catch (error) {
        console.error("Gagal memanggil sensor:", error);
    }
};
```
