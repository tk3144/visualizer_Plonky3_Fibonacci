# AIR Visualizer for Plonky3 Fibonacci
This project generates an execution trace using the **Plonky 3** ZKP system and provides a web-based visualizer to inspect the relevant constraints.
The project is sourced from [Plonky3_Fibonacci](https://github.com/BrianSeong99/Plonky3_Fibonacci). For further information on the respective protocol, please refer to the original repository.


### How to run the Demo:
#### 1. In `main.rs`, run the Rust program which will generate the trace data as a JSON file (`web/trace_data.json`).
```bash
cargo run
```

#### 2. Navigate to the `web/` directory and type the following:
```bash
cd web
python3 -m http.server 8000
```
#### 3. Visit [http://localhost:8000](http://localhost:8000) in your browser.
> To stop the local host server from running, use `Ctrl + Z`

#### 4. You may modify the sequence length by changing the value of the variables in 'src/main.rs' (lines 138-139):
```bash
let num_steps = 8; // Choose the number of Fibonacci steps in powers of 2^n
let final_value = 21; // Expected result of final Fibonacci value
```

### Troubleshooting
* __Cache__: If you change the value of `num_steps` in Rust, but do not see changes in the browser, please perform a Hard Refresh to clear the browser's cache:
  - Mac: `Cmd + Shift + R`
  - Windows: `Ctrl + F5`

* __Port Already in Use__: If you get an "Address already in use" error, it means a previous server used is still running. You can force kill any process (on port 8000 for instance) with the following:
  - `lsof -ti:8000 | xargs kill -9`


### Technical Resources & Citations
  
__Rust & File Handling__
* [Serde](https://serde.rs/) Serializing/Deserializing Rust data structures.
* [Serde JSON](https://docs.rs/serde_json/latest/serde_json/) Converting serialized data into JSON (String) (Note: This [video](https://www.youtube.com/watch?v=YLZtw8_aLwA) helped a lot).
* [std::fs::File](https://doc.rust-lang.org/std/fs/struct.File.html) & [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) Filesystem operations, specifically used for creating the output file and writing the JSON byte buffer.
* [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) used to dynamically construct the `trace_matrix` (`Vec<Vec<String>>`).
__Plonky3 & Matrix Logic__
* [Plonky3_Fibonacci](https://github.com/BrianSeong99/Plonky3_Fibonacci)
* ["Getting Started with Plonky3: Build, Prove, & Verify in ZK"](https://www.youtube.com/watch?v=l7v0Cr-cktg)
* [Mersenne31](https://docs.rs/p3-mersenne-31/latest/p3_mersenne_31/) & ["31 and Mersenne Primes - Numberphile"](https://www.youtube.com/watch?v=PLL0mo5rHhk) 
* [p3_matrix::Matrix](https://docs.rs/p3-matrix/latest/p3_matrix/)
__Web Technologies__
* [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)
* [JSON (JavaScript Object Notation)](https://developer.mozilla.org/en-US/docs/Glossary/JSON)
* [DOM Manipulation](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model)
