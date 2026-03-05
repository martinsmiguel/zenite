use serde::{Deserialize, Serialize};

pub type Centavos = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<i64>,
    pub participant_id: i64,
    pub category: String,
    pub description: String,
    pub date_emission: String,
    pub date_due: String,
    pub date_payment: Option<String>,
    pub value_agreed: Centavos,
    pub value_paid: Centavos,
    pub installment_current: i64,
    pub installment_total: i64,
    pub payment_method: String,
    pub status: String,
    pub observations: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: Option<i64>,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub action: String,
    pub participant_id: i64,
    pub changes: String,
    pub timestamp: i64,
}

pub struct DatabaseConfig {
    pub path: String,
}

pub struct TransactionBuilder {
    transaction: Transaction,
}

impl TransactionBuilder {
    pub fn default() -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        Self {
            transaction: Transaction {
                id: None,
                participant_id: 1,
                category: String::from("Test"),
                description: String::from("Test transaction"),
                date_emission: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                date_due: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                date_payment: None,
                value_agreed: 1000,
                value_paid: 1000,
                installment_current: 1,
                installment_total: 1,
                payment_method: String::from("Test"),
                status: String::from("PAGO"),
                observations: None,
                created_at: timestamp,
                updated_at: timestamp,
            },
        }
    }

    pub fn with_participant_id(self, id: i64) -> Self {
        Self {
            transaction: Transaction {
                participant_id: id,
                ..self.transaction
            },
        }
    }

    pub fn with_value(self, value: Centavos) -> Self {
        Self {
            transaction: Transaction {
                value_agreed: value,
                value_paid: value,
                ..self.transaction
            },
        }
    }

    pub fn with_status(self, status: String) -> Self {
        Self {
            transaction: Transaction {
                status,
                ..self.transaction
            },
        }
    }

    pub fn build(self) -> Transaction {
        self.transaction
    }
}
