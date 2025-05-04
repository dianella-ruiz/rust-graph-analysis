# Rust Porject Write-Up 

Goal: Analyze the structure of a large social network graph by computing the degree distributio and identifying connected components. 
What is the degree sitrivution of nodes in the graph, and how connected is the overall structure? Does the degree distribution follow a power-law distribution, as is commonly believed in social networks? 

Dataset:  

Source: Stanford Large Network Dataset 
File used: musae_facebook_edges.csv, musae_facebook_target.csv 
Size: 171,002 edges, 22,470 nodes
*File too large for GitHUb, must be downloaded manually and placed in a folder named facebook_large/ in the root directory 
https://snap.stanford.edu/data/facebook-large-page-page-network.html

Data Processing: 
how it was loaded into Rust: 
Used std::fs::File and std::io::BufReader for efficient file reading.
Used the petgraph crate to create and manipulate an undirected graph.
Parsed each line of the CSV after skipping the header.

Cleaning/Transformations: 
Each row was parsed into a pair of node IDs (source, target).
Used a HashMap<String, NodeIndex> to convert original node IDs into internal NodeIndex values without duplication.
Skipped self-loops and duplicate edges (handled automatically by petgraph unless specifically added multiple times).


Modules: 
main.rs: main program entry, loads data, and runs all analysis.  
functions and types: 
purpose: coordinates workflow, handles user visible output, manages errors
mod.rs: contains the GraphAnalyzer struct and all logic related to graph contruction, analysis, and output 
purpose: encapsulates all logic related to graph manipulation to keep main.rs clean and maintainable

GraphAnalyzer::from_csv
purpose: reads a CSV edge list and builds a petgraph undirected graph
input: path to CSV file 
output: GraphAnalyzer struct instance 
Logic: interates over the nodes and counts their neighbors 

degree_distribution
Purpose: Calculates how many nodes have each degree.
Input: Self-reference
Output: HashMap<usize, usize> where key = degree, value = count of nodes
Logic: Iterates over nodes and counts their neighbors, tallies degrees in a HashMap

save_distribution
Purpose: Outputs degree distribution to a CSV file
Input: Output file path
Output: CSV file with degree,count format
Logic: Uses File::create, iterates and writes each entry to file. 

average_degree
Purpose: Computes the average node degree
Input: none
Output: f64 
Logic: Weighted average using (count * degree)/total nodes

count_connected_components
Purpose: returns the number of connected components in the graph. 
Input: Self-reference
Output: usize number of components
Logic: Uses petgraph::algo::connected_components

Main Workflow: 
load CSV into a graph, compute and print degree distribution, save results to CSV, calculate and print average degree, count and print number of connected components

Tests: 
[1001170000@jupyter-nb-druiz43-40bu-2eedu-0 ds210_project]$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src/main.rs (target/debug/deps/ds210_project-b25bafdddc34076d)

running 4 tests
test graph::tests::test_average_degree ... ok
test graph::tests::test_connected_components ... ok
test graph::tests::test_degree_distribution ... ok
test graph::tests::test_graph_loading ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.86s

test_graph_loading: verfifies graph loads without crashing and contains nodes 
test_degree_distribution: Asserts degree distribtution is populated 
test_average_degree: ensures average degree is > 0, sanity check. 
test_connected_components: Checks components count is >= 1 


Results:
[1001170000@jupyter-nb-druiz43-40bu-2eedu-0 ds210_project]$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/ds210_project`
Graph loaded with 22470 nodes and 171002 edges
Degree Distribution:
Degree 12: 490 nodes
Degree 504: 1 nodes
Degree 199: 2 nodes
Degree 89: 10 nodes
Degree 156: 3 nodes
Degree 195: 1 nodes
Degree 216: 1 nodes
Degree 87: 13 nodes
Degree 163: 1 nodes
Degree 131: 3 nodes
Degree 33: 131 nodes
Degree 117: 5 nodes
Degree 123: 3 nodes
Degree 141: 4 nodes
Degree 105: 11 nodes
Degree 75: 16 nodes
Degree 222: 3 nodes
Degree 709: 1 nodes
Degree 338: 1 nodes
Degree 320: 1 nodes
Degree 181: 2 nodes
Degree 155: 3 nodes
Degree 112: 11 nodes
Degree 37: 89 nodes
Degree 326: 1 nodes
Degree 98: 3 nodes
Degree 39: 71 nodes
Degree 177: 3 nodes
Degree 136: 1 nodes
Degree 16: 333 nodes
Degree 205: 1 nodes
Degree 102: 5 nodes
Degree 68: 25 nodes
Degree 40: 73 nodes
Degree 34: 100 nodes
Degree 128: 1 nodes
Degree 111: 3 nodes
Degree 153: 1 nodes
Degree 142: 4 nodes
Degree 88: 12 nodes
Degree 29: 149 nodes
Degree 63: 20 nodes
Degree 27: 149 nodes
Degree 83: 17 nodes
Degree 387: 1 nodes
Degree 240: 1 nodes
Degree 224: 1 nodes
Degree 104: 10 nodes
Degree 149: 1 nodes
Degree 178: 1 nodes
Degree 333: 1 nodes
Degree 62: 41 nodes
Degree 174: 2 nodes
Degree 266: 1 nodes
Degree 127: 3 nodes
Degree 448: 1 nodes
Degree 121: 7 nodes
Degree 70: 19 nodes
Degree 241: 1 nodes
Degree 236: 2 nodes
Degree 468: 1 nodes
Degree 25: 198 nodes
Degree 108: 5 nodes
Degree 172: 1 nodes
Degree 138: 5 nodes
Degree 77: 14 nodes
Degree 296: 1 nodes
Degree 226: 2 nodes
Degree 109: 5 nodes
Degree 106: 8 nodes
Degree 7: 965 nodes
Degree 52: 36 nodes
Degree 229: 2 nodes
Degree 49: 44 nodes
Degree 8: 813 nodes
Degree 56: 36 nodes
Degree 93: 9 nodes
Degree 95: 4 nodes
Degree 46: 49 nodes
Degree 328: 1 nodes
Degree 47: 51 nodes
Degree 119: 3 nodes
Degree 96: 7 nodes
Degree 212: 1 nodes
Degree 166: 1 nodes
Degree 2: 2322 nodes
Degree 157: 1 nodes
Degree 45: 59 nodes
Degree 133: 3 nodes
Degree 26: 192 nodes
Degree 78: 12 nodes
Degree 151: 1 nodes
Degree 194: 1 nodes
Degree 170: 1 nodes
Degree 220: 1 nodes
Degree 189: 1 nodes
Degree 244: 1 nodes
Degree 76: 19 nodes
Degree 92: 9 nodes
Degree 18: 316 nodes
Degree 370: 1 nodes
Degree 80: 11 nodes
Degree 678: 1 nodes
Degree 233: 1 nodes
Degree 375: 1 nodes
Degree 64: 26 nodes
Degree 182: 2 nodes
Degree 74: 19 nodes
Degree 225: 1 nodes
Degree 150: 1 nodes
Degree 171: 1 nodes
Degree 99: 13 nodes
Degree 31: 133 nodes
Degree 114: 6 nodes
Degree 341: 1 nodes
Degree 659: 1 nodes
Degree 35: 108 nodes
Degree 380: 1 nodes
Degree 191: 1 nodes
Degree 237: 1 nodes
Degree 201: 3 nodes
Degree 124: 2 nodes
Degree 650: 1 nodes
Degree 61: 32 nodes
Degree 169: 1 nodes
Degree 38: 85 nodes
Degree 44: 67 nodes
Degree 22: 228 nodes
Degree 90: 13 nodes
Degree 408: 1 nodes
Degree 11: 552 nodes
Degree 42: 68 nodes
Degree 364: 1 nodes
Degree 115: 4 nodes
Degree 162: 1 nodes
Degree 15: 379 nodes
Degree 9: 729 nodes
Degree 24: 194 nodes
Degree 183: 2 nodes
Degree 257: 2 nodes
Degree 298: 1 nodes
Degree 73: 19 nodes
Degree 23: 214 nodes
Degree 58: 27 nodes
Degree 3: 1850 nodes
Degree 5: 1360 nodes
Degree 196: 1 nodes
Degree 72: 15 nodes
Degree 185: 1 nodes
Degree 290: 1 nodes
Degree 30: 127 nodes
Degree 280: 1 nodes
Degree 122: 5 nodes
Degree 55: 36 nodes
Degree 48: 53 nodes
Degree 28: 146 nodes
Degree 1: 2658 nodes
Degree 94: 12 nodes
Degree 85: 8 nodes
Degree 97: 8 nodes
Degree 217: 1 nodes
Degree 110: 4 nodes
Degree 214: 1 nodes
Degree 126: 2 nodes
Degree 41: 69 nodes
Degree 71: 20 nodes
Degree 275: 2 nodes
Degree 144: 3 nodes
Degree 152: 1 nodes
Degree 148: 4 nodes
Degree 43: 78 nodes
Degree 36: 106 nodes
Degree 67: 27 nodes
Degree 168: 1 nodes
Degree 14: 390 nodes
Degree 351: 1 nodes
Degree 101: 8 nodes
Degree 330: 2 nodes
Degree 140: 3 nodes
Degree 19: 264 nodes
Degree 120: 7 nodes
Degree 203: 1 nodes
Degree 17: 349 nodes
Degree 59: 38 nodes
Degree 167: 2 nodes
Degree 65: 22 nodes
Degree 129: 3 nodes
Degree 234: 1 nodes
Degree 193: 2 nodes
Degree 57: 37 nodes
Degree 69: 32 nodes
Degree 116: 7 nodes
Degree 118: 3 nodes
Degree 145: 4 nodes
Degree 260: 1 nodes
Degree 256: 1 nodes
Degree 32: 112 nodes
Degree 417: 1 nodes
Degree 60: 38 nodes
Degree 20: 263 nodes
Degree 100: 5 nodes
Degree 84: 9 nodes
Degree 132: 3 nodes
Degree 103: 3 nodes
Degree 6: 1114 nodes
Degree 107: 7 nodes
Degree 139: 2 nodes
Degree 21: 232 nodes
Degree 51: 53 nodes
Degree 147: 2 nodes
Degree 158: 1 nodes
Degree 53: 52 nodes
Degree 50: 49 nodes
Degree 13: 453 nodes
Degree 82: 9 nodes
Degree 135: 3 nodes
Degree 288: 2 nodes
Degree 54: 34 nodes
Degree 125: 4 nodes
Degree 213: 1 nodes
Degree 143: 3 nodes
Degree 218: 1 nodes
Degree 66: 23 nodes
Degree 137: 1 nodes
Degree 81: 12 nodes
Degree 4: 1509 nodes
Degree 91: 7 nodes
Degree 79: 10 nodes
Degree 134: 3 nodes
Degree 146: 3 nodes
Degree 10: 621 nodes
Degree 86: 12 nodes
Degree 113: 4 nodes
Degree 188: 2 nodes
Saved degree distribution to file.
Average node degree: 15.21
The graph has 1 connected components.

Interpretation: 
The graph contains 22,470 nodes and 171,002 edges, confirming that the dataset represents a large-scale social network. The average node degree is 15.21, meaning that each user, on average, is connected to about 15 other users. The degree distribution is right-skewed and heavy-tailed: most users have a low number of connections, while a few have very high degrees (e.g., 504 connections). Tis aligns with the power-law behavior observed in many real-world social networks. The graph has only one connected component, indicating that all users are part of a single, connected social structure, which is a common property of large online networks.

Usage Instructions: 
in terminal 
cargo build --release
cargo run --release 

Notes: 
put musae_facebook_edges.csv in facebook_large/ folder at the project root, 
expected runtime: 1-2 seconds 


Citations: 
No AI usage 
https://docs.rs/petgraph/latest/petgraph/
