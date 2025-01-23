# ✨ Rust Tree Copy Tool ✨

**Effortlessly Replicate Directory Structures for Rust Projects**

This command-line tool, written in Rust, allows you to quickly create a copy of an existing file and directory structure, tailored for Rust development.  It intelligently replicates your project's layout, ensuring all directories and file names are preserved. The magic ✨ happens with TypeScript and JavaScript files – these are transformed into empty Rust files (`.rs` extension), setting the stage for your Rust-based reimplementation.

## 🚀 Getting Started

Ready to restructure your project for Rust? Here's how to use the `rust_tree_copy` tool:

### 🛠️ Compilation

First, you'll need to compile the Rust code. Make sure you have Rust installed on your system. Open your terminal, navigate to the directory where you saved the `rust_tree_copy.rs` file, and run the following command:

```bash
rustc rust_tree_copy.rs
```

This command will compile the Rust code and create an executable file named rust_tree_copy (or rust_tree_copy.exe on Windows) in the same directory.

🏃‍♂️ Execution

Now you can run the program! You need to provide the path to the source directory you want to copy as a command-line argument.

For example, if your original project structure is located in a directory called my_typescript_project, execute the tool like this:
```bash
./rust_tree_copy my_typescript_project
```

Replace my_typescript_project with the actual path to your source directory.

After running the command, the tool will:

Create a new directory: A new directory will be created in the same location as your source directory. The new directory will have the same name as your source directory, but with _rs appended (e.g., my_typescript_project_rs).

Replicate the structure: Inside this new directory, you'll find an identical directory structure mirroring your original project.

Transform .ts and .js files: All .ts and .js files from the original structure will be recreated as empty .rs files in the new structure. All other file types will be copied as empty files, preserving their names and locations.

💡 Example

Let's say you have a TypeScript project with this structure:
```bash
my_typescript_project/
├── src/
│   ├── index.ts
│   └── utils.js
├── package.json
└── README.md
```

After running ./rust_tree_copy my_typescript_project, you will get a new directory:
```bash
my_typescript_project_rs/
├── src/
│   ├── index.rs  <-- .ts becomes .rs (empty file)
│   └── utils.rs  <-- .js becomes .rs (empty file)
├── package.json <-- Copied as empty file
└── README.md    <-- Copied as empty file
```
Now you have a clean Rust project structure ready to be populated with your Rust code! 🎉

⚙️ Dependencies

Rust: You need to have the Rust programming language and Cargo (Rust's package manager) installed on your system to compile and run this tool. You can download Rust from https://www.rust-lang.org/.

Enjoy transforming your projects into Rust with ease! ✨

