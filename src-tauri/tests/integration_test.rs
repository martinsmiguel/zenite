//! # Integration Tests
//!
//! Tests for Tauri commands and SQLite integration.
//!
//! This module focuses on validating the integration between:
//! - Control layer (Tauri Commands)
//! - Business logic
//! - Data persistence (SQLite)

use rusqlite::Connection;
use zenite_lib::db::create_schema_in_memory;
use zenite_lib::models::TransactionBuilder;

/// Default setup for integration tests with initialized schema
pub fn setup_integration_test() -> Connection {
    let conn = create_schema_in_memory().expect("Failed to setup test database");
    conn
}

/// Test: Should create schema with 3 correct tables
#[test]
fn should_create_schema_with_3_tables() {
    // Arrange
    let conn = setup_integration_test();

    // Act
    // Schema was already created in setup_integration_test()

    // Assert
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
        .expect("Failed to prepare query")
        .query_map([], |row| row.get::<_, String>(0))
        .expect("Failed to query")
        .filter_map(|r| r.ok())
        .collect();

    assert_eq!(
        tables.len(),
        3,
        "Should have exactly 3 tables: participants, transactions, audit_log"
    );
    assert!(tables.contains(&String::from("participants")));
    assert!(tables.contains(&String::from("transactions")));
    assert!(tables.contains(&String::from("audit_log")));
}

/// Test: Should have empty tables initially
#[test]
fn should_have_empty_tables_initially() {
    // Arrange
    let conn = setup_integration_test();

    // Act
    // Schema created on setup, tables empty by default

    // Assert
    let participants_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM participants", [], |row| row.get(0))
        .expect("Failed to count participants");

    assert_eq!(
        participants_count, 0,
        "participants table should be empty initially"
    );

    let transactions_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM transactions", [], |row| row.get(0))
        .expect("Failed to count transactions");

    assert_eq!(
        transactions_count, 0,
        "transactions table should be empty initially"
    );

    let audit_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))
        .expect("Failed to count audit logs");

    assert_eq!(
        audit_count, 0,
        "audit_log table should be empty initially"
    );
}

/// Test: Should have indexes created
#[test]
fn should_have_indexes_created() {
    // Arrange
    let conn = setup_integration_test();

    // Act
    // Indexes created in create_schema_in_memory()

    // Assert
    let indices: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%'")
        .expect("Failed to prepare query")
        .query_map([], |row| row.get::<_, String>(0))
        .expect("Failed to query")
        .filter_map(|r| r.ok())
        .collect();

    // Verify expected indexes
    let expected_indices = vec![
        "idx_transactions_participant_id",
        "idx_transactions_date_due",
        "idx_transactions_status",
        "idx_transactions_date_emission",
        "idx_audit_log_transaction_id",
        "idx_audit_log_timestamp",
        "idx_audit_log_participant_id",
    ];

    for index in expected_indices {
        assert!(
            indices.contains(&String::from(index)),
            "Should have index: {}",
            index
        );
    }
}

/// Test: Should accept default TransactionBuilder
#[test]
fn should_use_default_transaction_builder() {
    // Arrange
    let _conn = setup_integration_test();

    // Act
    let transaction = TransactionBuilder::default().build();

    // Assert
    assert_eq!(transaction.participant_id, 1);
    assert_eq!(transaction.value_agreed, 1000);
    assert_eq!(transaction.value_paid, 1000);
    assert_eq!(transaction.installment_current, 1);
    assert_eq!(transaction.installment_total, 1);
    assert_eq!(transaction.status, String::from("PAGO"));
    assert!(transaction.date_emission.len() == 10); // YYYY-MM-DD
    assert!(transaction.date_due.len() == 10);
}

/// Test: Should accept TransactionBuilder with custom values
#[test]
fn should_use_transaction_builder_with_custom_values() {
    // Arrange
    let _conn = setup_integration_test();

    // Act
    let transaction = TransactionBuilder::default()
        .with_participant_id(5)
        .with_value(2500)
        .with_status(String::from("PENDENTE"))
        .build();

    // Assert
    assert_eq!(transaction.participant_id, 5);
    assert_eq!(transaction.value_agreed, 2500);
    assert_eq!(transaction.value_paid, 2500);
    assert_eq!(transaction.status, String::from("PENDENTE"));
}

/// Test: Should have foreign keys configured
#[test]
fn should_have_foreign_keys_configured() {
    // Arrange
    let conn = setup_integration_test();

    // Act
    // Schema created with foreign_keys = ON

    // Assert
    let foreign_keys: i64 = conn
        .query_row("PRAGMA foreign_keys;", [], |row| row.get(0))
        .expect("Failed to query foreign_keys");

    assert_eq!(foreign_keys, 1, "Foreign keys should be enabled");
}
