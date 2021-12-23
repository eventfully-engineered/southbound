use crate::error::SouthboundError;

mod error;

pub type SouthboundResult<T> = core::result::Result<T, SouthboundError>;

pub struct Config {
    pub driver: String, // Database, DB, Engine, Backend?
    pub url: String, // URL?
    pub user: String,
    pub password: String,
    pub connect_retries: i32, // default to n?
    pub init_sql: String,
    pub default_schema: String,
    pub schemas: String, // comma-separated value. vec
    pub create_schema: bool,
    pub table: String, // schema_history
    // tablespace
    pub locations: String, // comma-separated value. vec
    pub sql_migration_prefix: String, // Migration
    pub undo_sql_migration_prefix: String, // downgrade
    pub repeatable_sql_migration_prefix: String, // RRR
    pub sql_migration_separator: String, // __
    pub sql_migration_suffixes: String, // comma-separated value. vec. default to .sql

    // flyway.stream=true
    // flyway.batch=true
    // flyway.encoding=ISO-8859-1
    // flyway.placeholderReplacement=true
    // flyway.placeholders.aplaceholder=value
    // flyway.placeholders.otherplaceholder=value123
    // flyway.placeholderPrefix=#[ // default ${
    // flyway.placeholderSuffix=] // default }
    // flyway.resolvers=com.mycomp.project.CustomResolver,com.mycomp.project.AnotherResolver
    // flyway.skipDefaultCallResolvers=false
    // flyway.callbacks=com.mycomp.project.CustomCallback,com.mycomp.project.AnotherCallback
    // flyway.skipDefaultCallbacks=false
    // flyway.target=5.1
    // flyway.outOfOrder=false
    // flyway.outputQueryResults=false
    // flyway.validateOnMigrate=true
    // flyway.cleanOnValidationError=false
    // flyway.mixed=false
    // flyway.group=false
    // flyway.ignoreMissingMigrations=false
    // flyway.ignoreIgnoredMigrations=false
    // flyway.ignoreFutureMigrations=false
    // flyway.cleanDisabled=false
    // flyway.baselineOnMigrate=false
    // flyway.installedBy=my-user
    // flyway.errorOverrides=99999:17110:E,42001:42001:W
    // flyway.dryRunOutput=/my/sql/dryrun-outputfile.sql
    // flyway.lockRetryCount=10
    // flyway.oracle.sqlplus=true
    // flyway.oracle.sqlplusWarn=true
    // flyway.workingDirectory=C:/myProject
    // flyway.jdbcProperties.myProperty=value

}