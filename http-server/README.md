# 🦀 Rust Mini Web Server

A lightweight TCP-based web server written in **pure Rust** — no frameworks, no external crates, just the standard library.

This project demonstrates how a basic HTTP server works under the hood:

* Listening for TCP connections
* Reading raw HTTP requests
* Serving `.html` files
* Writing correct HTTP responses
* Understanding how browsers and servers communicate

---

## 📌 Features

* ⚡ Pure Rust implementation (no external dependencies)
* 📄 Serves static HTML files
* 🌐 Handles routes (`/` and fallback 404)
* 🛠 Teaches core Rust concepts (ownership, reading streams, etc.)
* 🔌 Works with any browser
* 📦 Super easy to run

---

## 📁 Project Structure

```
rust-mini-server/
│
├── public/
│   ├── index.html
│   └── 404.html
│
├── src/
│   └── main.rs
│
├── README.md
└── Cargo.toml
```

---

# 🚀 Getting Started

Below are step-by-step instructions to install Rust, clone this project, and run the server.

---

# 1️⃣ Install Rust

Rust provides an official installer via **rustup**.

### **For Linux & macOS**

Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then reload your shell:

```bash
source $HOME/.cargo/env
```

### **For Windows**

Download Rust installer from:
➡️ [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

---

# 2️⃣ Verify Installation

Run:

```bash
rustc --version
cargo --version
```

You should see version numbers like:

```
rustc 1.xx.x
cargo 1.xx.x
```

---

# 3️⃣ Clone This Project

```bash
git clone https://github.com/yourusername/rust-mini-server.git
cd rust-mini-server
```

(If you want, tell me your GitHub username and I’ll customize the URL.)

---

# 4️⃣ Project Files

### **src/main.rs**

This file contains the complete server implementation:

* Uses `TcpListener` to accept connections
* Reads HTTP requests via `TcpStream`
* Differentiates between `/` and other routes
* Serves HTML files
* Sends proper HTTP headers

### **public/index.html**

The main webpage displayed on visiting `/`.

### **public/404.html**

Fallback page for unknown routes.

---

# 5️⃣ Run the Server

To start the server:

```bash
cargo run
```

You should see:

```
🚀 Server running at http://127.0.0.1:7878
```

Now open your browser and enter:

👉 [http://127.0.0.1:7878/](http://127.0.0.1:7878/)

Your Rust server should load your `index.html`.

---

# 6️⃣ How It Works (Technical Breakdown)

### **1. Listening for Incoming Connections**

```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
```

This binds the server to port `7878` and waits for client connections.

---

### **2. Reading the HTTP Request**

```rust
stream.read(&mut buffer).unwrap();
```

Reads raw TCP bytes and converts them to text.

---

### **3. Routing Logic**

```rust
let get_root = request.starts_with("GET / ");
```

If the user visits `/`, we serve `index.html`.
All other routes → `404.html`.

---

### **4. Serving HTML Files**

```rust
let contents = fs::read_to_string(filename).unwrap();
```

Loads HTML content from the `public/` directory.

---

### **5. Sending an HTTP Response**

```rust
let response = format!(
    "{status_line}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
    contents.len(),
    contents
);
```

The server responds with:

* Status line (200 or 404)
* Content length
* Content type
* Body (HTML)

---

# 7️⃣ Stopping the Server

Just press:

```
CTRL + C
```

---

# 8️⃣ Example Output

When someone connects, the server prints:

```
Received Request:
GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/...
```

---

# 9️⃣ Customize the Server

You can easily extend this project:

* Add support for CSS, JS, images
* Add routing based on file paths
* Implement multi-threading
* Build your own mini framework
* Add async Rust with `tokio`

If you want, I can help you build any of those.

---

# 🧠 Why This Project Matters

Building a web server from scratch teaches you:

* How TCP works
* How HTTP requests look internally
* How to send proper HTTP headers
* Low-level Rust I/O operations
* Ownership + borrowing + lifetimes in practice

This is one of the BEST beginner Rust projects because it teaches fundamentals extremely clearly.

---

# 🙌 Contributing

Pull requests are welcome!

---

# 📜 License

MIT License — free to use, modify, and distribute.

