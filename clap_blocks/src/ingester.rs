//! CLI config for catalog ingest lifecycle

/// CLI config for catalog ingest lifecycle
#[derive(Debug, Clone, clap::Parser)]
#[allow(missing_copy_implementations)]
pub struct IngesterConfig {
    /// Write buffer shard index to start (inclusive) range with
    #[clap(
        long = "--shard-index-range-start",
        env = "INFLUXDB_IOX_SHARD_INDEX_RANGE_START",
        action
    )]
    pub shard_index_range_start: i32,

    /// Write buffer shard index to end (inclusive) range with
    #[clap(
        long = "--shard-index-range-end",
        env = "INFLUXDB_IOX_SHARD_INDEX_RANGE_END",
        action
    )]
    pub shard_index_range_end: i32,

    /// The ingester will continue to pull data and buffer it from the write buffer as long as the
    /// ingester buffer is below this size. If the ingester buffer hits this size, ingest from the
    /// write buffer will pause until the ingester buffer goes below this threshold.
    #[clap(
        long = "--pause-ingest-size-bytes",
        env = "INFLUXDB_IOX_PAUSE_INGEST_SIZE_BYTES",
        action
    )]
    pub pause_ingest_size_bytes: usize,

    /// Once the ingester crosses this threshold of data buffered across all shards, it will
    /// pick the largest partitions and persist them until it falls below this threshold. An
    /// ingester running in a steady state is expected to take up this much memory.
    #[clap(
        long = "--persist-memory-threshold-bytes",
        env = "INFLUXDB_IOX_PERSIST_MEMORY_THRESHOLD_BYTES",
        action
    )]
    pub persist_memory_threshold_bytes: usize,

    /// If the total bytes written to an individual partition crosses
    /// this size threshold, it will be persisted.  The default value
    /// is 300MB (in bytes).
    ///
    /// NOTE: This number is related, but *NOT* the same as the size
    /// of the memory used to keep the partition buffered.
    #[clap(
        long = "--persist-partition-size-threshold-bytes",
        env = "INFLUXDB_IOX_PERSIST_PARTITION_SIZE_THRESHOLD_BYTES",
        default_value = "314572800",
        action
    )]
    pub persist_partition_size_threshold_bytes: usize,

    /// If a partition has had data buffered for longer than this period of time, it will be
    /// persisted. This puts an upper bound on how far back the ingester may need to read from the
    /// write buffer on restart or recovery. The default value is 30 minutes (in seconds).
    #[clap(
        long = "--persist-partition-age-threshold-seconds",
        env = "INFLUXDB_IOX_PERSIST_PARTITION_AGE_THRESHOLD_SECONDS",
        default_value = "1800",
        action
    )]
    pub persist_partition_age_threshold_seconds: u64,

    /// If a partition has had data buffered and hasn't received a write for this
    /// period of time, it will be persisted. The default value is 300 seconds (5 minutes).
    #[clap(
        long = "--persist-partition-cold-threshold-seconds",
        env = "INFLUXDB_IOX_PERSIST_PARTITION_COLD_THRESHOLD_SECONDS",
        default_value = "300",
        action
    )]
    pub persist_partition_cold_threshold_seconds: u64,

    /// Trigger persistence of a partition if it contains more than this many rows.
    #[clap(
        long = "--persist-partition-max-rows",
        env = "INFLUXDB_IOX_PERSIST_PARTITION_MAX_ROWS",
        default_value = "500000",
        action
    )]
    pub persist_partition_rows_max: usize,

    /// If the catalog's max sequence number for the partition is no longer available in the write
    /// buffer due to the retention policy, by default the ingester will panic. If this flag is
    /// specified, the ingester will skip any sequence numbers that have not been retained in the
    /// write buffer and will start up successfully with the oldest available data.
    #[clap(
        long = "--skip-to-oldest-available",
        env = "INFLUXDB_IOX_SKIP_TO_OLDEST_AVAILABLE",
        action
    )]
    pub skip_to_oldest_available: bool,

    /// Sets how often `do_get` flight requests should panic for testing purposes.
    ///
    /// The first N requests will panic. Requests after this will just pass.
    #[clap(
        long = "--test-flight-do-get-panic",
        env = "INFLUXDB_IOX_FLIGHT_DO_GET_PANIC",
        default_value = "0",
        action
    )]
    pub test_flight_do_get_panic: u64,

    /// Sets how many concurrent requests the ingester will handle before rejecting
    /// incoming requests.
    #[clap(
        long = "--concurrent-request-limit",
        env = "INFLUXDB_IOX_CONCURRENT_REQEST_LIMIT",
        default_value = "20",
        action
    )]
    pub concurrent_request_limit: usize,
}
