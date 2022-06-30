pub type ClientId = u16;
pub type TransactionId = u16;

// represents a single payment engine action
#[derive(Debug, Clone, Copy)]
pub enum PaymentCommand {
    Deposit {
        client: ClientId,
        tx: TransactionId,
        amount: f64,
    },

    Withdrawal {
        client: ClientId,
        tx: TransactionId,
        amount: f64,
    },

    Dispute {
        client: ClientId,
        tx: TransactionId,
    },

    Resolve {
        client: ClientId,
        tx: TransactionId,
    },

    Chargeback {
        client: ClientId,
        tx: TransactionId,
    },
}
