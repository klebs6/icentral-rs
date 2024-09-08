.PHONY: build vendor json active test test_one test_all

RUSTFLAGS  := -Awarnings RUST_BACKTRACE=1

CARGO := MAKEFLAGS= env CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo 
BUILD := build #--verbose
RUN   := run
TEST  := test
BENCH := bench
CLEAN := clean

#FEATURES := --features "xxx"

EXAMPLE := basic

#default: run_active_example
#default: test_one
#default: test
#default: bench
#default: build
default: active

INDIVIDUAL_TEST := test_edge_count_self_loop
INDIVIDUAL_TEST := test_edges
INDIVIDUAL_TEST := test_all_edges

#ACTIVE_PACKAGE := icentral-all-pairs-distance
#ACTIVE_PACKAGE := icentral-all-pairs-shortest-path-counts
#ACTIVE_PACKAGE := icentral-articulation-point
#ACTIVE_PACKAGE := icentral-articulation-point-map
#ACTIVE_PACKAGE := icentral-bcc
#ACTIVE_PACKAGE := icentral-brandes
#ACTIVE_PACKAGE := icentral-bridge-edges
#ACTIVE_PACKAGE := icentral-color-map
#ACTIVE_PACKAGE := icentral-component
#ACTIVE_PACKAGE := icentral-conn-vertex-map
#ACTIVE_PACKAGE := icentral-count-bcc
#ACTIVE_PACKAGE := icentral-cycle
#ACTIVE_PACKAGE := icentral-delta
#ACTIVE_PACKAGE := icentral-deltas
#ACTIVE_PACKAGE := icentral-distances
#ACTIVE_PACKAGE := icentral-edge
#ACTIVE_PACKAGE := icentral-edges
#ACTIVE_PACKAGE := icentral-errors
#ACTIVE_PACKAGE := icentral-fast-brandes
#ACTIVE_PACKAGE := icentral-gen-rand-edges
#ACTIVE_PACKAGE := icentral-graph-interface
#ACTIVE_PACKAGE := icentral-label-map
#ACTIVE_PACKAGE := icentral-mindexed-map
#ACTIVE_PACKAGE := icentral-mock
#ACTIVE_PACKAGE := icentral-muc
#ACTIVE_PACKAGE := icentral-muc-speedup-stats
#ACTIVE_PACKAGE := icentral-mucid
#ACTIVE_PACKAGE := icentral-mucid-map
#ACTIVE_PACKAGE := icentral-neighbors
#ACTIVE_PACKAGE := icentral-node-queue
#ACTIVE_PACKAGE := icentral-node-stack
#ACTIVE_PACKAGE := icentral-nodeid
#ACTIVE_PACKAGE := icentral-pair-dependencies
#ACTIVE_PACKAGE := icentral-parallel-brandes
#ACTIVE_PACKAGE := icentral-parents-map
#ACTIVE_PACKAGE := icentral-partial-bbfs
#ACTIVE_PACKAGE := icentral-path-counts
#ACTIVE_PACKAGE := icentral-predecessor-map
#ACTIVE_PACKAGE := icentral-qube
#ACTIVE_PACKAGE := icentral-rbfs
#ACTIVE_PACKAGE := icentral-scores
#ACTIVE_PACKAGE := icentral-scratch
#ACTIVE_PACKAGE := icentral-sigmas
#ACTIVE_PACKAGE := icentral-stats
#ACTIVE_PACKAGE := icentral-subgraph
#ACTIVE_PACKAGE := icentral-subgraph-map
#ACTIVE_PACKAGE := icentral-timer
#ACTIVE_PACKAGE := icentral-visit-markers
#ACTIVE_PACKAGE := icentral-workspace-map
#ACTIVE_PACKAGE := icentral-graph
#ACTIVE_PACKAGE := icentral-graph-hash
#ACTIVE_PACKAGE := icentral-delta
#ACTIVE_PACKAGE := icentral-bbfs
#ACTIVE_PACKAGE := icentral-speedup-calculation
#ACTIVE_PACKAGE := icentral-largest-bcc
#ACTIVE_PACKAGE := icentral-calculate-bc-mem
#ACTIVE_PACKAGE := icentral-workspace
#ACTIVE_PACKAGE := icentral-test-incremental-qube
#ACTIVE_PACKAGE := icentral-test-incremental-brandes
#ACTIVE_PACKAGE := icentral-test-brandes
#ACTIVE_PACKAGE := icentral-test-qube
#ACTIVE_PACKAGE := icentral-timing-update
#ACTIVE_PACKAGE := icentral-test-fuad
#ACTIVE_PACKAGE := icentral-operation-update
#ACTIVE_PACKAGE := icentral-alg
#ACTIVE_PACKAGE := icentral-operation
#ACTIVE_PACKAGE := icentral-test-update
#ACTIVE_PACKAGE := icentral-test-speedup
#ACTIVE_PACKAGE := icentral-test-rustworkx
#ACTIVE_PACKAGE := icentral-test-largest-bcc-speedup
#ACTIVE_PACKAGE := icentral-compare-runtime
#ACTIVE_PACKAGE := icentral-naive
#ACTIVE_PACKAGE := icentral-match
#ACTIVE_PACKAGE := icentral-test-fast-brandes
#ACTIVE_PACKAGE := icentral-json
#ACTIVE_PACKAGE := icentral-test-basic-icentral
#ACTIVE_PACKAGE := icentral
#ACTIVE_PACKAGE := icentral-test-insertion
ACTIVE_PACKAGE := icentral

build:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) $(FEATURES)

active:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) -p $(ACTIVE_PACKAGE) $(FEATURES)

run_active_example:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(RUN) -p $(ACTIVE_PACKAGE) $(FEATURES) --example $(EXAMPLE)

clean:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(CLEAN)

test:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) -p $(ACTIVE_PACKAGE) $(FEATURES) -- --nocapture

bench:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BENCH) -p $(ACTIVE_PACKAGE) $(FEATURES) -- --nocapture

test_one:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) -p $(ACTIVE_PACKAGE) $(INDIVIDUAL_TEST) $(FEATURES) -- --nocapture

vendor:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) vendor

json:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) $(FEATURES) --quiet --message-format=json 2> /dev/null | jq --slurp
