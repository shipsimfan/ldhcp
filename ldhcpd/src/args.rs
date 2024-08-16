use argparse::{config_flag, help_flag, parser, parsing_flag, simple_flag, version_flag};
use huntsman_http::{HTTPListenAddress, HTTPOptions, HTTP};
use oak::{FilterListType, LogLevel, StdLogOutput};
use std::{net::SocketAddr, num::NonZeroUsize, path::PathBuf, time::Duration};

/// Options that control how the server will run
pub struct LDHCPOptions {
    /* Database Flags */
    /// The path to the SQLite database
    pub database_path: PathBuf,

    /// The path to the migration scripts
    pub migration_path: PathBuf,

    /* Huntsman Flags */
    /// The huntsman options
    pub huntsman_options: huntsman::Options<HTTP>,

    /* HTTP Flags */
    /// The HTTP options
    pub http_options: HTTPOptions,

    /* Logging Flags */
    /// Should request headers be logged?
    pub log_headers: bool,

    /// Should request bodies be logged?
    pub log_bodies: bool,

    /// Should response codes and paths be logged?
    pub log_reponses: bool,

    /// The minimum severity to log
    pub min_log_level: LogLevel,

    /// The maximum severity to log
    pub max_log_level: Option<LogLevel>,

    /// The scope filter list type
    pub log_filter_type: FilterListType,

    /// The scope filter list
    pub log_filter: Vec<String>,

    /// The outputs for logging
    pub log_outputs: Vec<StdLogOutput>,
}

parser! {
    PARSER -> LDHCPOptions
    "LDHCPD"
    "Lance Dynamic Host Configuration Protocol Server",
    [
        // Database flags
        parsing_flag!(, "db" "PATH" "missing PATH for db"
                      ["Specify the path to the SQLite database",
                       "Defaults to \"ldhcp.db\""]
                      |options: LDHCPOptions, path: PathBuf| { options.database_path = path; }
        ).group("DATABASE FLAGS"),
        parsing_flag!(, "migrations" "PATH" "missing PATH for migrations"
                      ["Specify the path to the database migration scripts",
                       "Defaults to \"migrations\""]
                      |options: LDHCPOptions, path: PathBuf| { options.migration_path = path; }
        ).group("DATABASE FLAGS"),

        // Huntsman flags
        parsing_flag!(, "workers" "COUNT" "missing COUNT for workers"
                      ["Specify the number of worker threads",
                       "Defaults to a system provided value, usually the number of CPUs"]
                      |options: LDHCPOptions, count: NonZeroUsize| { options.huntsman_options.set_workers(count); }
        ).group("HUNTSMAN FLAGS"),
        parsing_flag!(, "worker-connections" "COUNT" "missing COUNT for worker-connections"
                      ["Specify the maximum number of connections per worker",
                       "Defaults to 64"]
                      |options: LDHCPOptions, count: NonZeroUsize| { options.huntsman_options.set_connections_per_worker(count); }
        ).group("HUNTSMAN FLAGS"),
        parsing_flag!(, "http" "ADDRESS:PORT" "missing ADDRESS for http"
                      "Specify an address to listen for insecure HTTP/1.1 connections on"
                      |options: LDHCPOptions, address: SocketAddr| { options.huntsman_options.add_address(HTTPListenAddress::HTTP(address)); }
        ).group("HUNTSMAN FLAGS"),

        // HTTP Flags
        parsing_flag!(, "max-header-size" "SIZE" "missing size for max-header-size"
                      ["Specify the maximum size of request headers in bytes to accpet",
                       "Defaults to 8,192 bytes (8 Kb)"]
                      |options: LDHCPOptions, size: usize| { options.http_options.max_header_size = size; }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "max-body-size" "SIZE" "missing SIZE for max-body-size"
                      ["Specify the maximum size of request bodies in bytes to accept",
                       "Defaults to 1,048,576 bytes (1 Mb)"]
                      |options: LDHCPOptions, size: usize| { options.http_options.max_body_size = size; }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "timeout" "TIMEOUT" "missing TIMEOUT for timeout"
                      ["Specify all timeouts to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: LDHCPOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.header_read_timeout = timeout;
                          options.http_options.body_read_timeout = timeout;
                          options.http_options.write_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "read-timeout" "TIMEOUT" "missing TIMEOUT for read-timeout"
                      ["Specify request read timeouts to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: LDHCPOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.header_read_timeout = timeout;
                          options.http_options.body_read_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "header-read-timeout" "TIMEOUT" "missing TIMEOUT for header-read-timeout"
                      ["Specify request header read timeout to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: LDHCPOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.header_read_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "body-read-timeout" "TIMEOUT" "missing TIMEOUT for body-read-timeout"
                      ["Specify request body read timeout to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: LDHCPOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.body_read_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),
        parsing_flag!(, "write-timeout" "TIMEOUT" "missing TIMEOUT for write-timeout"
                      ["Specify response write timeout to wait TIMEOUT milliseconds",
                       "Defaults to 60,000 milliseconds (1 minute)",
                       "If any conflicting flags are specified, the latest one specified will take precedence"]
                      |options: LDHCPOptions, timeout: u64| {
                          let timeout = Duration::from_millis(timeout);
                          options.http_options.write_timeout = timeout;
                      }
        ).group("HTTP FLAGS"),

        // Logging Flags
        simple_flag!(, "log-headers"
                     "Enable logging request headers"
                     |options: LDHCPOptions, _| { options.log_headers = true; }
        ).group("LOGGING FLAGS"),
        simple_flag!(, "log-bodies"
                     "Enable logging request bodies"
                     |options: LDHCPOptions, _| { options.log_bodies = true; }
        ).group("LOGGING FLAGS"),
        simple_flag!(, "log-responses"
                     "Enable logging responses statuses and paths"
                     |options: LDHCPOptions, _| { options.log_reponses = true; }
        ).group("LOGGING FLAGS"),
        parsing_flag!(, "min-log-level" "LEVEL" "missing LEVEL for min-log-level"
                      ["Sets the minimum severity of messages to log",
                       "LEVEL can be \"trace\", \"debug\", \"info\", \"warn\", \"err\", or \"fatal\"",
                       "Defaults to \"info\""]
                      |options: LDHCPOptions, level: LogLevel| { options.min_log_level = level; }
        ).group("LOGGING FLAGS"),
        parsing_flag!(, "max-log-level" "LEVEL" "missing LEVEL for max-log-level"
                      ["Sets the maximum severity of messages to log",
                       "LEVEL can be \"trace\", \"debug\", \"info\", \"warn\", \"err\", or \"fatal\"",
                       "Defaults to allowing all log messages"]
                      |options: LDHCPOptions, level: LogLevel| { options.max_log_level = Some(level); }
        ).group("LOGGING FLAGS"),
        parsing_flag!(, "log-filter-type" "TYPE" "missing TYPE for log-filter-type"
                      ["Sets the log filter type",
                       "TYPE can be \"blacklist\" or \"whitelist\"",
                       "Defaults to \"blacklist\""]
                      |options: LDHCPOptions, filter_type: FilterListType| { options.log_filter_type = filter_type; }
        ).group("LOGGING FLAGS"),
        parsing_flag!(, "log-filter" "SCOPE" "missing SCOPE for log-filter"
                     "Add SCOPE to the log filter list"
                     |options: LDHCPOptions, scope: String| { options.log_filter.push(scope); }
        ).group("LOGGING FLAGS").repeatable(true),
        parsing_flag!(, "log-output" "OUTPUT" "missing OUTPUT for log-output"
                      ["Add OUTPUT as a log output",
                       "Can be set to \"stdout\", \"stderr\", or a path"]
                      |options: LDHCPOptions, output: StdLogOutput| { options.log_outputs.push(output); }
        ).group("LOGGING FLAGS").repeatable(true),

        // Other Flags
        config_flag!(, "config").group("OTHER FLAGS"),
        help_flag!("h", "help").group("OTHER FLAGS"),
        version_flag!(, "version" concat!("LDHCP v", env!("CARGO_PKG_VERSION"))).group("OTHER FLAGS"),
    ]
}

/// Parse the command line arguments into options
pub fn parse<'a>() -> Result<Option<LDHCPOptions>, argparse::Error<'a>> {
    PARSER
        .usage("USAGE:\n    %0 [OPTIONS]...")
        .parse_env(LDHCPOptions::default())
}

impl Default for LDHCPOptions {
    fn default() -> Self {
        LDHCPOptions {
            database_path: "ldhcp.db".into(),
            migration_path: "migrations".into(),
            huntsman_options: huntsman::Options::default(),
            http_options: HTTPOptions::default(),
            log_headers: false,
            log_bodies: false,
            log_reponses: false,
            min_log_level: LogLevel::Info,
            max_log_level: None,
            log_filter_type: FilterListType::Blacklist,
            log_filter: Vec::new(),
            log_outputs: Vec::new(),
        }
    }
}
