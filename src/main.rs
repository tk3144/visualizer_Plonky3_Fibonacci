use std::fmt::Debug;
use std::marker::PhantomData;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::Matrix;
use p3_matrix::dense::RowMajorMatrix;

use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_circle::CirclePcs;
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_fri::FriConfig;
use p3_keccak::Keccak256Hash;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_mersenne_31::Mersenne31;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use p3_uni_stark::{prove, verify, StarkConfig};
use tracing_forest::util::LevelFilter;
use tracing_forest::ForestLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};



//-----------------------------------------------------------
// Importing for File Handling, Writing, and Serialization for the Visualizer
use serde::Serialize;
use std::fs::File;
use std::io::Write;

// Macro for implementing the Serialize and Clone traits
#[derive(Serialize, Clone)]

// VisData acts as the data container to serialize
struct VisData {
    num_steps: usize,           // unsigned int, number of fibonacci steps
    trace: Vec<Vec<String>>,    // vector of vectors (matrix) of type String
}
//-----------------------------------------------------------


pub struct FibonacciAir {
    pub num_steps: usize,
    pub final_value: u32,
}


impl<F: Field> BaseAir<F> for FibonacciAir {
    fn width(&self) -> usize {
        2 // For current and next Fibonacci number
    }
}

impl<AB: AirBuilder> Air<AB> for FibonacciAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0).unwrap();
        let next = main.row_slice(1).unwrap();

        // Enforce starting values
        builder.when_first_row().assert_eq(local[0], AB::Expr::ZERO);
        builder.when_first_row().assert_eq(local[1], AB::Expr::ONE);

        // Enforce state transition constraints
        builder.when_transition().assert_eq(next[0], local[1]);
        builder.when_transition().assert_eq(next[1], local[0] + local[1]);

        // Constrain the final value
        let final_value = AB::Expr::from_u32(self.final_value);
        builder.when_last_row().assert_eq(local[1], final_value);
    }
}

pub fn generate_fibonacci_trace<F: Field>(num_steps: usize) -> RowMajorMatrix<F> {
    let mut values = Vec::with_capacity(num_steps * 2);
    let mut a = F::ZERO;
    let mut b = F::ONE;
    for _ in 0..num_steps {
        values.push(a);
        values.push(b);
        let c = a + b;
        a = b;
        b = c;
    }
    RowMajorMatrix::new(values, 2)
}

fn main() -> Result<(), impl Debug> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
        .with(ForestLayer::default())
        .init();

    type Val = Mersenne31;
    type Challenge = BinomialExtensionField<Val, 3>;

    type ByteHash = Keccak256Hash;
    type FieldHash = SerializingHasher<ByteHash>;
    let byte_hash = ByteHash {};
    let field_hash = FieldHash::new(Keccak256Hash {});

    type MyCompress = CompressionFunctionFromHasher<ByteHash, 2, 32>;
    let compress = MyCompress::new(byte_hash);

    type ValMmcs = MerkleTreeMmcs<Val, u8, FieldHash, MyCompress, 32>;
    let val_mmcs = ValMmcs::new(field_hash, compress);

    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    type Challenger = SerializingChallenger32<Val, HashChallenger<u8, ByteHash, 32>>;

    let fri_config = FriConfig {
        log_blowup: 1,
        num_queries: 100,
        proof_of_work_bits: 16,
        mmcs: challenge_mmcs,
        log_final_poly_len: 1,
    };

    type Pcs = CirclePcs<Val, ValMmcs, ChallengeMmcs>;
    let pcs = Pcs {
        mmcs: val_mmcs,
        fri_config,
        _phantom: PhantomData,
    };

    type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;
    let challenger = Challenger::from_hasher(vec![], byte_hash);
    let config = MyConfig::new(pcs, challenger);

    let num_steps = 8; // Choose the number of Fibonacci steps in powers of 2^n
    let final_value = 21; // Expected result of final Fibonacci value
    let air = FibonacciAir { num_steps, final_value };
    let trace = generate_fibonacci_trace::<Val>(num_steps);
    

    //-----------------------------------------------------------
    // Create a mutable trace matrix (Vec<Vec<String>>). We populate it by iterating through Seong's trace variable (line 141) with 
    // the p3_matrix::Matrix method signature for height (returns number of rows).

    let mut trace_matrix = Vec::new();

    for i in 0..trace.height() {
        let row = trace
                        .row_slice(i)           // Accesses row i of the trace matrix, returning Some(&[F]) if exists, else None. 
                        .unwrap()               // Returns the slice. Will panic if i is out of bounds.
                        .iter()                 // Creates an iterator (pointer) over the elements of a specific row.
                        .map(|v| v.to_string()) // For every element v produced/pointed by the iterator, convert it from Mersenne31 to String.
                        .collect();             // Collects iterator, allocated memory on the heap, and pushes the strings into a Vec<String>.
        trace_matrix.push(row); // Appends row (Vec<String>) to the end of trace_matrix (Vec<Vec<String>>).
    }
    
    // Create an immutable instance of the VisData struct to Export
    let vis_data = VisData{num_steps, trace: trace_matrix};

    // Export trace
    let json_valid = serde_json::to_string_pretty(&vis_data).unwrap(); // Convert vis_data into a JSON formatted string. to_string_pretty() provides indentation and newlines.
    std::fs::create_dir_all("web").expect("Failed to create web directory"); // Create the web/ directory if it does not already exist. 
    let mut file_valid = File::create("web/trace_data.json").expect("Failed to create web/trace_data.json");
    file_valid.write_all(json_valid.as_bytes()).unwrap();
    println!("Valid trace exported to web/trace_data.json");

    //-----------------------------------------------------------


    let proof = prove(&config, &air, trace, &vec![]);
    verify(&config, &air, &proof, &vec![])
}
