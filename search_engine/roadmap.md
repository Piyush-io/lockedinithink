# The Intuition-Building Roadmap: From Zero to Search Engine

You want to understand the system deeply, not just copy code. Smart. Here's the task-based learning path that builds intuition layer by layer.

---

## Phase 0: Async Rust Mental Models (Week 1-2)

### Task 1: Build a Sequential Web Fetcher (done)

**Goal:** Understand the pain of sequential I/O

- Fetch 20 URLs one-by-one, print response time for each

### Task 2: Build a Concurrent Web Fetcher (done)

**Goal:** Experience the "aha!" moment of async

- Fetch the same 20 URLs concurrently with tokio::spawn

### Task 3: Add Backpressure Control (done)

**Goal:** Learn that unbounded concurrency breaks things

- Try fetching 10,000 URLs at once (spawn all tasks immediately)

### Task 4: Handle Failures Gracefully (done)

**Goal:** Understand that networks are hostile

- Add URLs that timeout, return 404, or refuse connections
- Log errors, but don't crash the program

**Checkpoint:** Can you fetch 1000 URLs, handle failures, limit concurrency, and finish in <5 seconds? You understand async Rust.

---

## Phase 1: Persistent State & The URL Frontier (Week 3)

### Task 5: Build a Naive In-Memory Frontier (done)

**Goal:** Understand what a "frontier" actually does

- Create a HashSet<String> for seen URLs
- Create a VecDeque<String> for URLs to crawl
- When you fetch a URL, extract all links (<a href>), add unseen ones to the queue
- _Intuition: This is BFS graph traversal, but for the web_

### Task 6: Make It Persistent with RocksDB (done)

**Goal:** Learn that crashes shouldn't lose state

- Store seen set in RocksDB (key: URL, value: empty)
- Store frontier queue in RocksDB (key: priority, value: URL)
- Restart your crawler - it should resume from where it crashed
- _Intuition: HashMap vs database = volatile vs durable_

### Task 7: Add Politeness (Rate Limiting per Domain)

**Goal:** Don't get IP-banned

- Extract domain from URL (example.com from https://example.com/page)
- Maintain per-domain queues (use RocksDB Column Families)
- Wait 1 second between requests to the same domain
- _Intuition: Real crawlers respect servers - it's ethics and self-preservation_

### Task 8: Implement robots.txt Checking

**Goal:** Be a good internet citizen

- Before crawling example.com/page, fetch example.com/robots.txt
- Parse it (look for User-agent: \* and Disallow: rules)
- Skip disallowed URLs
- _Intuition: Legality matters - violating robots.txt can get you sued_

**Checkpoint:** Can you crawl 10,000 pages from 5 seed URLs, respecting politeness, surviving crashes, and never hitting the same domain twice per second? You understand distributed crawling.

---

## Phase 2: Content Extraction & Cleaning (Week 4-5)

### Task 9: Parse Raw HTML

**Goal:** See how messy the web really is

- Fetch 50 diverse pages (news, blogs, e-commerce, forums)
- Parse HTML with scraper crate
- Extract ALL text (no filtering yet)
- _Intuition: 70% of the page is navigation, ads, footers - noise_

### Task 10: Remove Script/Style/Nav Tags

**Goal:** First-pass boilerplate removal

- Strip <script>, <style>, <nav>, <footer> tags
- Compare before/after text
- Notice you still have "Subscribe to newsletter!" and "Related Articles"
- _Intuition: Tag filtering is necessary but insufficient_

### Task 11: Implement Text Density Heuristic

**Goal:** Find the "meat" of the page

- Calculate text-to-HTML ratio for each DOM node
- Keep nodes with >50% text density
- Discard nodes with high link density (>20% of text is links)
- _Intuition: Main content has paragraphs, not button clusters_

### Task 12: Build a "Golden Dataset" for Testing

**Goal:** Know when your extraction is good enough

- Manually annotate 20 pages: highlight what text should be kept
- Calculate precision/recall for your extractor
- Iterate until you hit >90% precision, >95% recall
- _Intuition: Without ground truth, you're flying blind_

### Task 13: Handle JavaScript-Rendered Pages

**Goal:** Learn that modern web ≠ static HTML

- Try parsing a React/Vue site (e.g., Twitter, Reddit)
- Notice you get nothing useful
- Integrate chromiumoxide (headless browser)
- Compare: static parsing vs rendered
- _Intuition: 30% of the web needs JS execution, but it's 10x slower_

**Checkpoint:** Can you extract clean article text from 100 diverse pages with >90% quality? You understand content extraction.

---

## Phase 3: Text Chunking & Embeddings (Week 6)

### Task 14: Naive Fixed-Size Chunking

**Goal:** Understand why chunking matters

- Split long articles into 200-word chunks (overlap 20 words)
- Try searching with very small chunks (50 words) vs large (1000 words)
- Notice small = too fragmented, large = too vague
- _Intuition: Chunk size is a trade-off: context vs precision_

### Task 15: Call a Python Embedding Model

**Goal:** Bridge Rust ↔ Python ecosystem

- Use PyO3 to call sentence-transformers from Rust
- Embed 1000 chunks, measure time per chunk
- Notice it's slow (~50ms per chunk on CPU)
- _Intuition: Embedding is the latency bottleneck_

### Task 16: Batch Embeddings for Efficiency

**Goal:** Learn that batching = 5-10x speedup

- Collect 32 chunks before calling the model
- Compare: 1-by-1 vs batched
- _Intuition: GPUs/CPUs love batched matrix ops_

### Task 17: Store Embeddings in RocksDB

**Goal:** Vectors are just data

- Serialize vectors as bytes (use bincode or postcard)
- Store: key = chunk_id, value = [f32; 384] (if using mini model)
- Load them back, verify round-trip
- _Intuition: Vectors aren't magic - they're just arrays of floats_

**Checkpoint:** Can you embed 10,000 chunks and store them durably? You understand the ML pipeline.

---

## Phase 4: Vector Search & HNSW (Week 7-8)

### Task 18: Brute-Force Vector Search

**Goal:** Feel the pain of O(n) search

- Load 10,000 vectors into memory
- Given a query vector, calculate cosine similarity to ALL vectors
- Find top-10 most similar
- Measure time (probably ~50-100ms)
- _Intuition: Linear search doesn't scale to millions_

### Task 19: Build a Simple HNSW Index

**Goal:** Understand graph-based search

- Use hnsw_rs crate (don't implement from scratch yet)
- Insert 100,000 vectors
- Query it, measure time (probably ~5-10ms)
- _Intuition: HNSW trades accuracy for 10-100x speedup_

### Task 20: Tune HNSW Parameters

**Goal:** Learn the accuracy/speed trade-off

- Vary ef_construction (16, 32, 64, 128)
- Vary ef_search (10, 50, 100, 200)
- Plot: latency vs recall@10
- _Intuition: Higher = slower but more accurate_

### Task 21: Implement Query-Time Filtering

**Goal:** Realize pure vector search isn't enough

- Add metadata to chunks (domain, date)
- Try filtering results: "only from nytimes.com"
- Notice HNSW doesn't support filtering natively - you must post-filter
- _Intuition: This is why Qdrant/Milvus exist - they solve this problem_

**Checkpoint:** Can you search 100k vectors in <20ms with >90% recall? You understand vector indexing.

---

## Phase 5: Distributed Sharding (Week 9-10)

### Task 22: Hash-Based Sharding

**Goal:** Understand how to partition data

- Split 100k vectors across 4 in-memory HNSW indexes
- Use hash(doc_id) % 4 to decide which shard
- Query ALL shards, merge top-10 from each
- _Intuition: Sharding = divide-and-conquer_

### Task 23: Run Shards as Separate Processes

**Goal:** Move from threads to distributed system

- Run each HNSW shard as a separate Rust binary
- Communicate via HTTP or gRPC
- Query aggregator fans out to all shards
- _Intuition: Network = the new function call_

### Task 24: Handle Shard Failures

**Goal:** Distributed systems fail constantly

- Kill one shard mid-query
- Watch queries fail
- Add retry logic with timeout
- Return partial results if a shard is down
- _Intuition: Availability > perfect accuracy_

### Task 25: Add Result Re-Ranking

**Goal:** Combine vector + lexical signals

- Implement simple BM25 scoring (keyword matching)
- Combine with vector scores using Reciprocal Rank Fusion
- Notice hybrid search beats pure vector
- _Intuition: Semantics + keywords = best of both worlds_

**Checkpoint:** Can you query 4 shards in parallel and merge results in <100ms? You understand distributed search.

---

## Phase 6: Observability & Production Hardening (Week 11-12)

### Task 26: Add Structured Logging

**Goal:** Make the system debuggable

- Use tracing crate instead of println!
- Add span context: request_id, shard_id, stage
- Simulate a slow query - trace it through the entire pipeline
- _Intuition: Logs are your time machine for debugging_

### Task 27: Instrument with Metrics

**Goal:** Know what's actually happening

- Add Prometheus metrics: query latency (histogram), QPS (counter), error rate
- Visualize in Grafana
- Run load test, watch dashboards
- _Intuition: You can't improve what you don't measure_

### Task 28: Implement Graceful Degradation

**Goal:** Protect query latency under load

- Simulate 10x traffic spike
- Notice indexing tasks slow down queries
- Add priority: drop indexing work when query queue > 100
- _Intuition: In production, user-facing latency > background work_

### Task 29: Write Chaos Tests

**Goal:** Validate failure resilience

- Test: Kill crawler mid-crawl, restart, verify no duplicate fetches
- Test: Corrupt 1% of RocksDB keys, verify backup restore works
- Test: Network partition between query aggregator and shard
- _Intuition: Hope is not a strategy - test failure modes_

**Checkpoint:** Can your system survive crashes, traffic spikes, and partial failures gracefully? You understand production systems.

---

## Phase 7: Scale & Optimize (Week 13-16)

### Task 30: Profile the Hot Path

**Goal:** Find the real bottlenecks

- Run cargo flamegraph on a query workload
- Notice 60-70% is embedding generation
- _Intuition: Measure, don't guess_

### Task 31: Experiment with Model Compression

**Goal:** Latency = 90% model choice

- Benchmark: e5-small vs bge-base vs bge-large
- Measure: latency, memory, recall@10
- _Intuition: Smaller models are often "good enough"_

### Task 32: Implement INT8 Quantization

**Goal:** 4x memory savings, <2% accuracy loss

- Quantize vectors from FP32 → INT8
- Rebuild HNSW with quantized vectors
- Compare recall - should be >98% of original
- _Intuition: Precision is a dial, not a binary_

### Task 33: Scale to 1M Vectors

**Goal:** Hit real-world scale

- Crawl 1M pages (will take ~1 week)
- Notice RocksDB grows to ~50GB
- Tune: enable compression, increase block cache
- _Intuition: Big data = different problems_

**Checkpoint:** Can you serve queries from 1M vectors with <300ms P95 latency? You've built a real search engine.

---

## The Final Boss Task

### Task 34: Document Your Learnings

**Goal:** Solidify mental models by teaching

- Write a blog post: "I built a 1M-vector search engine from scratch"
- Explain ONE thing you learned deeply (e.g., "Why HNSW beats brute-force")
- Include graphs: latency vs scale, recall vs ef_search
- _Intuition: Teaching = 10x learning multiplier_

---

## How to Execute This

1. Do tasks sequentially - each builds on the last
2. Don't skip "boring" tasks (logging, testing) - they save you later
3. Measure everything - intuition comes from seeing data
4. Break things on purpose - crash the crawler, corrupt the DB, see what happens
5. When stuck >4 hours - ask for help, don't spiral

**Estimated total time:** 12-16 weeks if you're full-time focused.

**You'll know you're done when:** You can explain to a junior dev why each architectural decision was made, not just what you built.
