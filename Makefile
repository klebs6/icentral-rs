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

#----------------------------[layer-0]
#ACTIVE_PACKAGE := icentral

#----------------------------[layer-1]
#ACTIVE_PACKAGE := icentral-operation-update

#----------------------------[layer-2]
#ACTIVE_PACKAGE := icentral-alg

#----------------------------[layer-3]
#ACTIVE_PACKAGE := icentral-largest-bcc
#ACTIVE_PACKAGE := icentral-naive
#ACTIVE_PACKAGE := icentral-test-insertion
#ACTIVE_PACKAGE := icentral-test-largest-bcc-speedup
#ACTIVE_PACKAGE := icentral-test-qube
#ACTIVE_PACKAGE := icentral-test-speedup
#ACTIVE_PACKAGE := icentral-timing-update

#----------------------------[layer-4]
#ACTIVE_PACKAGE := icentral-calculate-bc-mem
#ACTIVE_PACKAGE := icentral-compare-runtime
#ACTIVE_PACKAGE := icentral-graph-hash
#ACTIVE_PACKAGE := icentral-speedup-calculation
#ACTIVE_PACKAGE := icentral-test-brandes
#ACTIVE_PACKAGE := icentral-test-fuad
#ACTIVE_PACKAGE := icentral-test-incremental-brandes
#ACTIVE_PACKAGE := icentral-test-incremental-qube

#----------------------------[layer-5]
#ACTIVE_PACKAGE := icentral-graph
#ACTIVE_PACKAGE := icentral-qube

#----------------------------[layer-6]
#ACTIVE_PACKAGE := icentral-delta
#ACTIVE_PACKAGE := icentral-fast-brandes
#ACTIVE_PACKAGE := icentral-muc

#----------------------------[layer-7]
#ACTIVE_PACKAGE := icentral-brandes
#ACTIVE_PACKAGE := icentral-workspace-map

#----------------------------[layer-8]
#ACTIVE_PACKAGE := icentral-bbfs
#ACTIVE_PACKAGE := icentral-partial-bbfs
#ACTIVE_PACKAGE := icentral-rbfs

#----------------------------[layer-9]
#ACTIVE_PACKAGE := icentral-parallel-brandes
#ACTIVE_PACKAGE := icentral-workspace

#----------------------------[layer-10]
#ACTIVE_PACKAGE := icentral-component
#ACTIVE_PACKAGE := icentral-scratch

#----------------------------[layer-11]
#ACTIVE_PACKAGE := icentral-bc-mem
#ACTIVE_PACKAGE := icentral-gen-rand-edges
#ACTIVE_PACKAGE := icentral-match
#ACTIVE_PACKAGE := icentral-subgraph

#----------------------------[layer-12]
#ACTIVE_PACKAGE := icentral-all-pairs-distance
#ACTIVE_PACKAGE := icentral-all-pairs-shortest-path-counts
#ACTIVE_PACKAGE := icentral-bridge-edges
#ACTIVE_PACKAGE := icentral-count-bcc
#ACTIVE_PACKAGE := icentral-mock
#ACTIVE_PACKAGE := icentral-mucid-map
#ACTIVE_PACKAGE := icentral-parents-map
#ACTIVE_PACKAGE := icentral-scores

#----------------------------[layer-13]
#ACTIVE_PACKAGE := icentral-graph-interface

#----------------------------[layer-14]
#ACTIVE_PACKAGE := icentral-bcc
#ACTIVE_PACKAGE := icentral-cycle

#----------------------------[layer-15]
#ACTIVE_PACKAGE := icentral-edges

#----------------------------[layer-16]
#ACTIVE_PACKAGE := icentral-articulation-point
#ACTIVE_PACKAGE := icentral-label-map

#----------------------------[layer-17]
#ACTIVE_PACKAGE := icentral-neighbors

#----------------------------[layer-18]
#ACTIVE_PACKAGE := icentral-errors

#----------------------------[layer-19]
#ACTIVE_PACKAGE := icentral-edge

#----------------------------[layer-20]
#ACTIVE_PACKAGE := icentral-articulation-point-map
ACTIVE_PACKAGE := icentral-color-map
#ACTIVE_PACKAGE := icentral-conn-vertex-map
#ACTIVE_PACKAGE := icentral-deltas
#ACTIVE_PACKAGE := icentral-distances
#ACTIVE_PACKAGE := icentral-pair-dependencies
#ACTIVE_PACKAGE := icentral-path-counts
#ACTIVE_PACKAGE := icentral-predecessor-map
#ACTIVE_PACKAGE := icentral-sigmas
#ACTIVE_PACKAGE := icentral-subgraph-map
#ACTIVE_PACKAGE := icentral-visit-markers

#----------------------------[layer-21]
#ACTIVE_PACKAGE := icentral-mindexed-map
#ACTIVE_PACKAGE := icentral-mucid
#ACTIVE_PACKAGE := icentral-node-queue
#ACTIVE_PACKAGE := icentral-node-stack

#----------------------------[layer-22]
#ACTIVE_PACKAGE := icentral-json
#ACTIVE_PACKAGE := icentral-muc-speedup-stats
#ACTIVE_PACKAGE := icentral-nodeid
#ACTIVE_PACKAGE := icentral-operation
#ACTIVE_PACKAGE := icentral-stats
#ACTIVE_PACKAGE := icentral-test-basic-icentral
#ACTIVE_PACKAGE := icentral-test-fast-brandes
#ACTIVE_PACKAGE := icentral-test-rustworkx
#ACTIVE_PACKAGE := icentral-test-update
#ACTIVE_PACKAGE := icentral-timer

#----------------------------[layer-23]
#ACTIVE_PACKAGE := icentral-3p

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

show:
	ws show crate-tree --path $(ACTIVE_PACKAGE) --for-ai-no-tests
