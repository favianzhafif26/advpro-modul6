# advpro-modul6
---
# Refleksi Milestone 1: Single Threaded Web Server

Pada tahap ini, saya membangun server sederhana menggunakan Rust yang mampu menerima koneksi dari browser. Dengan menggunakan `TcpListener`, server dapat mendengarkan permintaan di alamat `127.0.0.1:7878`. Saat ada koneksi masuk, terminal menampilkan pesan Connection established!, yang menandakan bahwa server berfungsi dengan baik.

Saya juga memahami bahwa jika browser tidak menerima respons, ia akan secara otomatis mengulangi permintaan, menyebabkan beberapa pesan koneksi muncul di terminal. Selain itu, penting untuk menghentikan server dengan benar agar port yang digunakan tidak tetap terbuka dan menghalangi server dijalankan kembali.

---
# Refleksi Milestone 2: Returning HTML
Pada tahap ini, saya mengembangkan server agar dapat mengirimkan halaman HTML sebagai respons ke browser. Metode `handle_connection` diperbarui untuk membaca file `hello.html` dan mengirimkannya sebagai bagian dari respons HTTP dengan status `200 OK`.

Elemen penting dalam respons ini adalah Content-Length, yang memberi tahu browser ukuran konten yang dikirimkan. Dengan menambahkan HTML sederhana, kini saya dapat melihat tampilan di browser. Hal ini memperjelas bagaimana komunikasi HTTP bekerja, termasuk bagaimana server membaca permintaan, menentukan respons yang sesuai, dan mengirimkan data ke browser.

Commit 2 screen capture:
![Commit 2 screen capture](/assets/images/commit2.png)

---
# Refleksi Milestone 3: Validating request and selectively responding
Pada tahap ini, server dikembangkan agar dapat merespons permintaan dengan lebih cerdas. Dengan membaca dan memeriksa path dalam permintaan HTTP, server dapat memberikan respons yang berbeda tergantung pada URL yang diakses. Jika pengguna mengakses /, server akan mengirimkan `hello.html`, sedangkan untuk path yang tidak dikenali, server akan menampilkan `404.html` dengan status `404 Not Found`.

Refactoring dilakukan untuk memisahkan logika dalam membangun respons agar kode lebih terstruktur dan mudah dipahami. Hal ini sangat penting dalam pengembangan aplikasi web yang lebih kompleks, di mana berbagai jenis permintaan harus ditangani dengan cara yang berbeda. Dengan pendekatan ini, server menjadi lebih fleksibel, mudah diperluas, dan lebih siap untuk menangani skenario yang lebih beragam.

Commit 3 screen capture:
![Commit 3 screen capture](/assets/images/commit3.png)

---
# Refleksi Milestone 4: Simulation slow response
Pada tahap ini, saya melakukan simulasi untuk memahami bagaimana server menangani permintaan yang membutuhkan waktu lama untuk diproses. Dengan menambahkan `thread::sleep(Duration::from_secs(10))` pada permintaan ke `/sleep`, server akan berhenti selama 10 detik sebelum memberikan respons.

Karena server masih berjalan dalam mode single-threaded, permintaan lain, seperti ke /, akan tertunda hingga permintaan sebelumnya selesai. Dari eksperimen ini, saya memahami bahwa server single-threaded memiliki keterbatasan dalam menangani banyak permintaan secara bersamaan. Jika ada satu permintaan yang memakan waktu lama, seluruh permintaan lainnya akan ikut tertahan. Inilah alasan utama mengapa server produksi umumnya menggunakan multi-threading untuk meningkatkan efisiensi dalam menangani banyak pengguna sekaligus.

---
# Refleksi Milestone 5: Multithreaded Server
Pada tahap terakhir ini, saya mengembangkan server agar dapat menangani banyak permintaan secara bersamaan dengan menggunakan `ThreadPool`. Sebelumnya, server berjalan dalam mode single-threaded, di mana satu permintaan yang lambat dapat menahan seluruh permintaan lainnya. Dengan menerapkan `ThreadPool`, server kini dapat memproses banyak permintaan secara paralel tanpa saling menghambat.

`ThreadPool` bekerja dengan membuat sejumlah thread worker yang siap menerima tugas. Saat ada koneksi masuk, server akan mendistribusikan tugas tersebut ke salah satu thread worker yang sedang idle. Dengan pendekatan ini, server menjadi lebih efisien dalam menangani banyak pengguna sekaligus.

Hasil uji coba menunjukkan bahwa server kini lebih responsif dalam menangani permintaan secara simultan. Tidak ada lagi blocking ketika ada permintaan yang membutuhkan waktu lebih lama untuk diproses, menjadikan server lebih siap untuk menangani skenario dunia nyata dengan lebih baik.

---