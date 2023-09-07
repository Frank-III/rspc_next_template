pub mod json_rpc {
//! The JSON-RPC API definition.
//!
//! ## Methods
//!
//!
//! ### 🔌 applyMigrations

//! ➡️  [ApplyMigrationsInput](./types/struct.ApplyMigrationsInput.html)
//!
//! ↩️  [ApplyMigrationsOutput](./types/struct.ApplyMigrationsOutput.html)
//!
//! Apply the migrations from the migrations directory to the database.
//! 
//! This is the command behind `prisma migrate deploy`.
//!
//! ### 🔌 createDatabase

//! ➡️  [CreateDatabaseParams](./types/struct.CreateDatabaseParams.html)
//!
//! ↩️  [CreateDatabaseResult](./types/struct.CreateDatabaseResult.html)
//!
//! Create the logical database from the Prisma schema.
//!
//! ### 🔌 createMigration

//! ➡️  [CreateMigrationInput](./types/struct.CreateMigrationInput.html)
//!
//! ↩️  [CreateMigrationOutput](./types/struct.CreateMigrationOutput.html)
//!
//! Create the next migration in the migrations history. If `draft` is false and
//! there are no unexecutable steps, it will also apply the newly created
//! migration.
//! 
//! **Note**: This will use the shadow database on the connectors where we need
//! one.
//!
//! ### 🔌 dbExecute

//! ➡️  [DbExecuteParams](./types/struct.DbExecuteParams.html)
//!
//! ↩️  [DbExecuteResult](./types/struct.DbExecuteResult.html)
//!
//! Execute a database script directly on the specified live database.
//! 
//! Note that this may not be
//! defined on all connectors.
//!
//! ### 🔌 debugPanic

//! ➡️  [DebugPanicInput](./types/struct.DebugPanicInput.html)
//!
//! ↩️  [DebugPanicOutput](./types/struct.DebugPanicOutput.html)
//!
//! Make the migration engine panic. Only useful to test client error handling.
//!
//! ### 🔌 devDiagnostic

//! ➡️  [DevDiagnosticInput](./types/struct.DevDiagnosticInput.html)
//!
//! ↩️  [DevDiagnosticOutput](./types/struct.DevDiagnosticOutput.html)
//!
//! The method called at the beginning of `migrate dev` to decide the course of
//! action based on the current state of the workspace.
//! 
//! It acts as a wrapper around diagnoseMigrationHistory. Its role is to interpret
//! the diagnostic output, and translate it to a concrete action to be performed by
//! the CLI.
//!
//! ### 🔌 diagnoseMigrationHistory

//! ➡️  [DiagnoseMigrationHistoryInput](./types/struct.DiagnoseMigrationHistoryInput.html)
//!
//! ↩️  [DiagnoseMigrationHistoryOutput](./types/struct.DiagnoseMigrationHistoryOutput.html)
//!
//! Read the contents of the migrations directory and the migrations table,
//! and returns their relative statuses. At this stage, the migration
//! engine only reads, it does not write to the database nor the migrations
//! directory, nor does it use a shadow database.
//!
//! ### 🔌 diff

//! ➡️  [DiffParams](./types/struct.DiffParams.html)
//!
//! ↩️  [DiffResult](./types/struct.DiffResult.html)
//!
//! Compares two databases schemas from two arbitrary sources, and display the
//! difference as either a human-readable summary, or an executable script that can
//! be passed to dbExecute.
//! 
//! Connection to a shadow database is only necessary when either the `from` or the
//! `to` params is a migrations directory.
//! 
//! Diffs have a _direction_. Which source is `from` and which is `to` matters. The
//! resulting diff should be thought as a migration from the schema in `from` to
//! the schema in `to`.
//! 
//! By default, we output a human-readable diff. If you want an executable script,
//! pass the `"script": true` param.
//!
//! ### 🔌 ensureConnectionValidity

//! ➡️  [EnsureConnectionValidityParams](./types/struct.EnsureConnectionValidityParams.html)
//!
//! ↩️  [EnsureConnectionValidityResult](./types/struct.EnsureConnectionValidityResult.html)
//!
//! Make sure the migration engine can connect to the database from the Prisma schema.
//!
//! ### 🔌 evaluateDataLoss

//! ➡️  [EvaluateDataLossInput](./types/struct.EvaluateDataLossInput.html)
//!
//! ↩️  [EvaluateDataLossOutput](./types/struct.EvaluateDataLossOutput.html)
//!
//! Development command for migrations. Evaluate the data loss induced by the next
//! migration the engine would generate on the main database.
//! 
//! At this stage, the engine does not create or mutate anything in the database
//! nor in the migrations directory.
//! 
//! This is part of the `migrate dev` flow.
//! 
//! **Note**: the engine currently assumes the main database schema is up-to-date
//! with the migration history.
//!
//! ### 🔌 getDatabaseVersion

//! ➡️  [GetDatabaseVersionInput](./types/struct.GetDatabaseVersionInput.html)
//!
//! ↩️  [GetDatabaseVersionOutput](./types/struct.GetDatabaseVersionOutput.html)
//!
//! Get the database version for error reporting.
//!
//! ### 🔌 listMigrationDirectories

//! ➡️  [ListMigrationDirectoriesInput](./types/struct.ListMigrationDirectoriesInput.html)
//!
//! ↩️  [ListMigrationDirectoriesOutput](./types/struct.ListMigrationDirectoriesOutput.html)
//!
//! List the names of the migrations in the migrations directory.
//!
//! ### 🔌 markMigrationApplied

//! ➡️  [MarkMigrationAppliedInput](./types/struct.MarkMigrationAppliedInput.html)
//!
//! ↩️  [MarkMigrationAppliedOutput](./types/struct.MarkMigrationAppliedOutput.html)
//!
//! Mark a migration as applied in the migrations table.
//! 
//! There are two possible outcomes:
//! 
//! - The migration is already in the table, but in a failed state. In this case, we will mark it
//! as rolled back, then create a new entry.
//! - The migration is not in the table. We will create a new entry in the migrations table. The
//! `started_at` and `finished_at` will be the same.
//! - If it is already applied, we return a user-facing error.
//!
//! ### 🔌 markMigrationRolledBack

//! ➡️  [MarkMigrationRolledBackInput](./types/struct.MarkMigrationRolledBackInput.html)
//!
//! ↩️  [MarkMigrationRolledBackOutput](./types/struct.MarkMigrationRolledBackOutput.html)
//!
//! Mark an existing failed migration as rolled back in the migrations table. It
//! will still be there, but ignored for all purposes except as audit trail.
//!
//! ### 🔌 reset

//! ➡️  [ResetInput](./types/struct.ResetInput.html)
//!
//! ↩️  [ResetOutput](./types/struct.ResetOutput.html)
//!
//! Try to make the database empty: no data and no schema. On most connectors, this
//! is implemented by dropping and recreating the database. If that fails (most
//! likely because of insufficient permissions), the engine attemps a "best effort
//! reset" by inspecting the contents of the database and dropping them
//! individually.
//! 
//! Drop and recreate the database. The migrations will not be applied, as it would
//! overlap with `applyMigrations`.
//!
//! ### 🔌 schemaPush

//! ➡️  [SchemaPushInput](./types/struct.SchemaPushInput.html)
//!
//! ↩️  [SchemaPushOutput](./types/struct.SchemaPushOutput.html)
//!
//! The command behind `db push`.
/// String constants for method names.
pub mod method_names {
/// Exhaustive list of the names of all JSON-RPC methods.
pub const METHOD_NAMES: &[&str] = &[    "applyMigrations",
    "createDatabase",
    "createMigration",
    "dbExecute",
    "debugPanic",
    "devDiagnostic",
    "diagnoseMigrationHistory",
    "diff",
    "ensureConnectionValidity",
    "evaluateDataLoss",
    "getDatabaseVersion",
    "listMigrationDirectories",
    "markMigrationApplied",
    "markMigrationRolledBack",
    "reset",
    "schemaPush",
];
/// applyMigrations
pub const APPLY_MIGRATIONS: &str = "applyMigrations";
/// createDatabase
pub const CREATE_DATABASE: &str = "createDatabase";
/// createMigration
pub const CREATE_MIGRATION: &str = "createMigration";
/// dbExecute
pub const DB_EXECUTE: &str = "dbExecute";
/// debugPanic
pub const DEBUG_PANIC: &str = "debugPanic";
/// devDiagnostic
pub const DEV_DIAGNOSTIC: &str = "devDiagnostic";
/// diagnoseMigrationHistory
pub const DIAGNOSE_MIGRATION_HISTORY: &str = "diagnoseMigrationHistory";
/// diff
pub const DIFF: &str = "diff";
/// ensureConnectionValidity
pub const ENSURE_CONNECTION_VALIDITY: &str = "ensureConnectionValidity";
/// evaluateDataLoss
pub const EVALUATE_DATA_LOSS: &str = "evaluateDataLoss";
/// getDatabaseVersion
pub const GET_DATABASE_VERSION: &str = "getDatabaseVersion";
/// listMigrationDirectories
pub const LIST_MIGRATION_DIRECTORIES: &str = "listMigrationDirectories";
/// markMigrationApplied
pub const MARK_MIGRATION_APPLIED: &str = "markMigrationApplied";
/// markMigrationRolledBack
pub const MARK_MIGRATION_ROLLED_BACK: &str = "markMigrationRolledBack";
/// reset
pub const RESET: &str = "reset";
/// schemaPush
pub const SCHEMA_PUSH: &str = "schemaPush";
}
/// API type definitions used by the methods.
#[allow(missing_docs)] pub mod types {
use serde::{Serialize, Deserialize};

/// The output of the `evaluateDataLoss` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateDataLossOutput {
    /// The number migration steps that would be generated. If this is empty, we
    /// wouldn't generate a new migration, unless the `draft` option is
    /// passed.
    ///
    /// JSON name: migrationSteps
    #[serde(rename = "migrationSteps")]
    pub migration_steps: u32,
    /// Steps that cannot be executed on the local database in the
    /// migration that would be generated.
    ///
    /// JSON name: unexecutableSteps
    #[serde(rename = "unexecutableSteps")]
    pub unexecutable_steps: Vec<MigrationFeedback>,
    /// Destructive change warnings for the local database. These are the
    /// warnings *for the migration that would be generated*. This does not
    /// include other potentially yet unapplied migrations.
    pub warnings: Vec<MigrationFeedback>,
}

/// The result type for the `diff` method.
#[derive(Serialize, Deserialize, Debug)]
pub struct DiffResult {
    /// The exit code that the CLI should return.
    ///
    /// JSON name: exitCode
    #[serde(rename = "exitCode")]
    pub exit_code: u32,
}

/// The type of params for the `diff` method.
/// ### Example
///
/// ```ignore
/// {
///     "from": {
///         "tag": "migrations",
///         "path": "./prisma/migrations"
///     },
///     "to": {
///         "tag": "schemaDatamodel",
///         "schema": "./prisma/schema.prisma",
///     }
///     "shadowDatabaseUrl": "mysql://test/test"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct DiffParams {
    /// Whether the --exit-code param was passed.
    /// 
    /// If this is set, the engine will return exitCode = 2 in the diffResult in case the diff is
    /// non-empty. Other than this, it does not change the behaviour of the command.
    ///
    /// JSON name: exitCode
    #[serde(rename = "exitCode")]
    pub exit_code: Option<bool>,
    /// The source of the schema to consider as a _starting point_.
    pub from: DiffTarget,
    /// By default, the response will contain a human-readable diff. If you want an
    /// executable script, pass the `"script": true` param.
    pub script: bool,
    /// The URL to a live database to use as a shadow database. The schema and data on
    /// that database will be wiped during diffing.
    /// 
    /// This is only necessary when one of `from` or `to` is referencing a migrations
    /// directory as a source for the schema.
    ///
    /// JSON name: shadowDatabaseUrl
    #[serde(rename = "shadowDatabaseUrl")]
    pub shadow_database_url: Option<String>,
    /// The source of the schema to consider as a _destination_, or the desired
    /// end-state.
    pub to: DiffTarget,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseIsBehindFields {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnsureConnectionValidityParams {
    pub datasource: DatasourceParam,
}

/// The input to the `createMigration` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMigrationInput {
    /// If true, always generate a migration, but do not apply.
    pub draft: bool,
    /// The user-given name for the migration. This will be used for the migration directory.
    ///
    /// JSON name: migrationName
    #[serde(rename = "migrationName")]
    pub migration_name: String,
    /// The filesystem path of the migrations directory to use.
    ///
    /// JSON name: migrationsDirectoryPath
    #[serde(rename = "migrationsDirectoryPath")]
    pub migrations_directory_path: String,
    /// The Prisma schema to use as a target for the generated migration.
    ///
    /// JSON name: prismaSchema
    #[serde(rename = "prismaSchema")]
    pub prisma_schema: String,
}

/// The input to the `evaluateDataLoss` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateDataLossInput {
    /// The location of the migrations directory.
    ///
    /// JSON name: migrationsDirectoryPath
    #[serde(rename = "migrationsDirectoryPath")]
    pub migrations_directory_path: String,
    /// The prisma schema to migrate to.
    ///
    /// JSON name: prismaSchema
    #[serde(rename = "prismaSchema")]
    pub prisma_schema: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListMigrationDirectoriesOutput {
    /// The names of the migrations in the migration directory. Empty if no migrations are found.
    pub migrations: Vec<String>,
}

/// The names of the migrations in the migration directory. Empty if no migrations are found.
#[derive(Serialize, Deserialize, Debug)]
pub struct MarkMigrationAppliedInput {
    /// The name of the migration to mark applied.
    ///
    /// JSON name: migrationName
    #[serde(rename = "migrationName")]
    pub migration_name: String,
    /// The path to the root of the migrations directory.
    ///
    /// JSON name: migrationsDirectoryPath
    #[serde(rename = "migrationsDirectoryPath")]
    pub migrations_directory_path: String,
}

/// Request params for the `schemaPush` method.
#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaPushInput {
    /// Push the schema ignoring destructive change warnings.
    pub force: bool,
    /// The Prisma schema.
    pub schema: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDatabaseParams {
    pub datasource: DatasourceParam,
}

/// The type of params accepted by dbExecute.
#[derive(Serialize, Deserialize, Debug)]
pub struct DbExecuteParams {
    /// The location of the live database to connect to.
    ///
    /// JSON name: datasourceType
    #[serde(rename = "datasourceType")]
    pub datasource_type: DbExecuteDatasourceType,
    /// The input script.
    pub script: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DebugPanicOutput {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetOutput {
}

/// The output of the `markMigrationRolledBack` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct MarkMigrationRolledBackOutput {
}

/// The result type for `diagnoseMigrationHistory` responses.
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnoseMigrationHistoryOutput {
    /// The names of the migrations for which the checksum of the script in the
    /// migration directory does not match the checksum of the applied migration
    /// in the database.
    ///
    /// JSON name: editedMigrationNames
    #[serde(rename = "editedMigrationNames")]
    pub edited_migration_names: Vec<String>,
    /// The names of the migrations that are currently in a failed state in the migrations table.
    ///
    /// JSON name: failedMigrationNames
    #[serde(rename = "failedMigrationNames")]
    pub failed_migration_names: Vec<String>,
    /// Is the migrations table initialized/present in the database?
    ///
    /// JSON name: hasMigrationsTable
    #[serde(rename = "hasMigrationsTable")]
    pub has_migrations_table: bool,
    /// The current status of the migration history of the database
    /// relative to migrations directory. `null` if they are in sync and up
    /// to date.
    pub history: Option<HistoryDiagnostic>,
}

/// Response result for the `schemaPush` method.
#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaPushOutput {
    /// How many migration steps were executed.
    ///
    /// JSON name: executedSteps
    #[serde(rename = "executedSteps")]
    pub executed_steps: u32,
    /// Steps that cannot be executed in the current state of the database.
    pub unexecutable: Vec<String>,
    /// Destructive change warnings.
    pub warnings: Vec<String>,
}

/// An object with a `url` field.
#[derive(Serialize, Deserialize, Debug)]
pub struct UrlContainer {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnsureConnectionValidityResult {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetInput {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDatabaseResult {
    ///
    /// JSON name: databaseName
    #[serde(rename = "databaseName")]
    pub database_name: String,
}

/// The response type for `devDiagnostic`.
#[derive(Serialize, Deserialize, Debug)]
pub struct DevDiagnosticOutput {
    /// The suggested course of action for the CLI.
    pub action: DevAction,
}

/// An object with a `schema` field.
#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaContainer {
    pub schema: String,
}

/// The output of the `creatMigration` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMigrationOutput {
    /// The name of the newly generated migration directory, if any.
    /// 
    /// generatedMigrationName will be null if: 
    /// 
    /// 1. The migration we generate would be empty, **AND**
    /// 2. the `draft` param was not true, because in that case the engine would still generate an empty
    ///    migration script.
    ///
    /// JSON name: generatedMigrationName
    #[serde(rename = "generatedMigrationName")]
    pub generated_migration_name: Option<String>,
}

/// The type of results returned by dbExecute.
#[derive(Serialize, Deserialize, Debug)]
pub struct DbExecuteResult {
}

/// A data loss warning or an unexecutable migration error, associated with the step that triggered it.
#[derive(Serialize, Deserialize, Debug)]
pub struct MigrationFeedback {
    /// The human-readable message.
    pub message: String,
    /// The index of the step this pertains to.
    ///
    /// JSON name: stepIndex
    #[serde(rename = "stepIndex")]
    pub step_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathContainer {
    pub path: String,
}

/// The input to the `markMigrationRolledBack` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct MarkMigrationRolledBackInput {
    /// The name of the migration to mark rolled back.
    ///
    /// JSON name: migrationName
    #[serde(rename = "migrationName")]
    pub migration_name: String,
}

/// The input to the `applyMigrations` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyMigrationsInput {
    /// The location of the migrations directory.
    ///
    /// JSON name: migrationsDirectoryPath
    #[serde(rename = "migrationsDirectoryPath")]
    pub migrations_directory_path: String,
}

/// The input to the `listMigrationDirectories` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListMigrationDirectoriesInput {
    /// The location of the migrations directory.
    ///
    /// JSON name: migrationsDirectoryPath
    #[serde(rename = "migrationsDirectoryPath")]
    pub migrations_directory_path: String,
}

/// The output of the `markMigrationApplied` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct MarkMigrationAppliedOutput {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDatabaseVersionOutput {
    pub version: String,
}

/// The request type for `devDiagnostic`.
#[derive(Serialize, Deserialize, Debug)]
pub struct DevDiagnosticInput {
    /// The location of the migrations directory.
    ///
    /// JSON name: migrationsDirectoryPath
    #[serde(rename = "migrationsDirectoryPath")]
    pub migrations_directory_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DebugPanicInput {
}

/// The output of the `applyMigrations` command.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyMigrationsOutput {
    /// The names of the migrations that were just applied. Empty if no migration was applied.
    ///
    /// JSON name: appliedMigrationNames
    #[serde(rename = "appliedMigrationNames")]
    pub applied_migration_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DevActionReset {
    /// Why do we need to reset?
    pub reason: String,
}

/// The request params for the `diagnoseMigrationHistory` method.
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnoseMigrationHistoryInput {
    /// The path to the root of the migrations directory.
    ///
    /// JSON name: migrationsDirectoryPath
    #[serde(rename = "migrationsDirectoryPath")]
    pub migrations_directory_path: String,
    /// Whether creating a shadow database is allowed.
    ///
    /// JSON name: optInToShadowDatabase
    #[serde(rename = "optInToShadowDatabase")]
    pub opt_in_to_shadow_database: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDatabaseVersionInput {
}

/// A suggested action for the CLI `migrate dev` command.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag")]
pub enum DevAction {
/// Reset the database.
///
/// JSON name: reset
#[serde(rename = "reset")]
    Reset(DevActionReset),
/// Proceed to the next step
///
/// JSON name: createMigration
#[serde(rename = "createMigration")]
    CreateMigration,
}
/// The location of the live database to connect to.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag")]
pub enum DbExecuteDatasourceType {
/// The URL of the database to run the command on.
///
/// JSON name: url
#[serde(rename = "url")]
    Url(UrlContainer),
/// Path to the Prisma schema file to take the datasource URL from.
///
/// JSON name: schema
#[serde(rename = "schema")]
    Schema(SchemaContainer),
}
/// The path to a live database taken as input. For flexibility, this can be the path to a Prisma
/// schema file containing the datasource, or the whole Prisma schema as a string, or only the
/// connection string. See variants.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag")]
pub enum DatasourceParam {
    ConnectionString(UrlContainer),
    SchemaPath(PathContainer),
    SchemaString(SchemaContainer),
}
/// A supported source for a database schema to diff in the `diff` command.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag")]
pub enum DiffTarget {
/// The path to a migrations directory of the shape expected by Prisma Migrate. The
/// migrations will be applied to a **shadow database**, and the resulting schema
/// considered for diffing.
///
/// JSON name: migrations
#[serde(rename = "migrations")]
    Migrations(PathContainer),
/// The url to a live database. Its schema will be considered.
/// 
/// This will cause the migration engine to connect to the database and read from
/// it. It will not write.
///
/// JSON name: url
#[serde(rename = "url")]
    Url(UrlContainer),
/// The path to a Prisma schema. The contents of the schema itself will be
/// considered. This source does not need any database connection.
///
/// JSON name: schemaDatamodel
#[serde(rename = "schemaDatamodel")]
    SchemaDatamodel(SchemaContainer),
/// An empty schema.
///
/// JSON name: empty
#[serde(rename = "empty")]
    Empty,
/// The path to a Prisma schema. The _datasource url_ will be considered, and the
/// live database it points to introspected for its schema.
///
/// JSON name: schemaDatasource
#[serde(rename = "schemaDatasource")]
    SchemaDatasource(SchemaContainer),
}
/// A diagnostic returned by `diagnoseMigrationHistory` when looking at the
/// database migration history in relation to the migrations directory.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag")]
pub enum HistoryDiagnostic {
/// There are migrations in the migrations directory that have not been applied to
/// the database yet.
    DatabaseIsBehind(DatabaseIsBehindFields),
    MigrationsDirectoryIsBehind,
    HistoriesDiverge,
}
}
}
