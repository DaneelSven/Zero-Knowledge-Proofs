# Getting Started with Noir and Nargo

[Noir Setup](https://noir-lang.org/docs/getting_started/hello_noir/#create-our-first-nargo-project)

### Step 1: Create Project Directory
```bash
mkdir ~/projects && cd ~/projects
```

### Step 2: Initialize Nargo Project
```bash
nargo new hello_world
```
*Replace `hello_world` with the name of your project.*

### Step 3: Build the Project
```bash
cd hello_world && nargo check
```

Two additional files would be generated in your project directory:

Prover.toml houses input values, and Verifier.toml houses public values.

### Step 4: Prove Your Program
```bash
nargo prove
```
*Make sure to fill in `Prover.toml` with your input values before running this.*

### Step 5: Verify the Proof
```bash
nargo verify
```
