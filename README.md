# advpro-modul6

# Refleksi Milestone 1: Single Threaded Web Server

## Pengalaman Membangun Server Sederhana

Pada milestone ini, kita berhasil membuat server web dasar menggunakan Rust. Server ini mampu menerima koneksi dari browser, meskipun belum mengirimkan respons yang bisa ditampilkan di layar. Saat pertama kali dijalankan, kita melihat pesan `Connection established!`, yang menunjukkan bahwa ada koneksi yang masuk ke server kita.

## Hal yang Dipelajari

### 1. Cara Server Menerima Koneksi
Menggunakan `TcpListener`, kita bisa membuat server yang mendengarkan permintaan dari klien di alamat `127.0.0.1:7878`. Ketika browser mencoba mengaksesnya, koneksi terbentuk, meskipun belum ada halaman yang ditampilkan.

### 2. Perilaku Browser Saat Tidak Mendapat Respons
Jika server tidak memberikan balasan, browser bisa secara otomatis mengirim ulang permintaan. Itulah sebabnya kita melihat beberapa pesan `Connection established!` muncul di terminal.

### 3. Membaca Permintaan dari Browser
Setelah menambahkan fungsi `handle_connection`, kita bisa membaca permintaan HTTP yang dikirim oleh browser. Permintaan ini mencakup informasi seperti:
- **Metode HTTP** (misalnya, `GET / HTTP/1.1`)
- **Header** seperti `Host`, `User-Agent`, `Accept`, dll.

Ini memberi wawasan tentang bagaimana browser dan server berkomunikasi satu sama lain.

### 4. Pentingnya Menghentikan Server dengan Benar
Jika server tidak dihentikan dengan baik, port `7878` tetap digunakan, yang menyebabkan error saat kita mencoba menjalankannya kembali. Untuk menghindari masalah ini, kita bisa menekan `Ctrl+C` di terminal atau menggunakan opsi `stop` di IDE.

## Kesimpulan
Pada tahap awal ini, kita belajar tentang dasar-dasar server berbasis TCP di Rust. Kita memahami bagaimana koneksi dibuat, bagaimana browser mengirim permintaan, dan bagaimana server dapat membaca permintaan tersebut. Langkah berikutnya adalah mengembangkan server ini agar bisa memberikan respons kepada klien.

Pengalaman ini sangat menarik dan menjadi dasar yang bagus untuk memahami cara kerja server web sebelum masuk ke fitur yang lebih kompleks seperti multi-threading dan pemrosesan request yang lebih canggih.