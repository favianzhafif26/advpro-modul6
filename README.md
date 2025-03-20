# advpro-modul6

# Refleksi Milestone 1: Single Threaded Web Server

Pada tahap ini, saya membangun server sederhana menggunakan Rust yang mampu menerima koneksi dari browser. Dengan menggunakan `TcpListener`, server dapat mendengarkan permintaan di alamat `127.0.0.1:7878`. Saat ada koneksi masuk, terminal menampilkan pesan Connection established!, yang menandakan bahwa server berfungsi dengan baik.

Saya juga memahami bahwa jika browser tidak menerima respons, ia akan secara otomatis mengulangi permintaan, menyebabkan beberapa pesan koneksi muncul di terminal. Selain itu, penting untuk menghentikan server dengan benar agar port yang digunakan tidak tetap terbuka dan menghalangi server dijalankan kembali.

# Refleksi Milestone 2: Returning HTML
Pada tahap ini, saya mengembangkan server agar dapat mengirimkan halaman HTML sebagai respons ke browser. Metode `handle_connection` diperbarui untuk membaca file `hello.html` dan mengirimkannya sebagai bagian dari respons HTTP dengan status `200 OK`.

Elemen penting dalam respons ini adalah Content-Length, yang memberi tahu browser ukuran konten yang dikirimkan. Dengan menambahkan HTML sederhana, kini saya dapat melihat tampilan di browser. Hal ini memperjelas bagaimana komunikasi HTTP bekerja, termasuk bagaimana server membaca permintaan, menentukan respons yang sesuai, dan mengirimkan data ke browser.

Commit 2 screen capture:
![Commit 2 screen capture](/assets/images/commit2.png)
