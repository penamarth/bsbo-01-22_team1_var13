use crate::{Delivery, Error, Payment};
use chrono::Utc;
use strum::EnumString;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct PaymentAdapter<P: PaymentSystem> {
    pub payment_system: P,
}

impl<P: PaymentSystem + std::fmt::Debug> PaymentAdapter<P> {
    #[instrument(name = "new_payment_adapter")]
    pub fn for_payment_system(payment_system: P) -> Self {
        tracing::info!(message = "creating new PaymentAdapter");
        Self { payment_system }
    }
}

pub trait PaymentSystem {
    fn request_payment(&self, delivery: Delivery) -> (Delivery, Payment);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ExternalPaymentSystem {
    Default,
}

impl ExternalPaymentSystem {
    pub const ENV_KEY: &str = "EXTERNAL_PAYMENT_SYSTEM";
    pub fn from_env() -> Result<Self, Error> {
        let string = std::env::var(Self::ENV_KEY)?;
        Self::try_from(string.as_str()).map_err(|_| Error::UnknownPaymentSystem)
    }
}

impl PaymentSystem for ExternalPaymentSystem {
    #[instrument(skip_all)]
    fn request_payment(&self, delivery: Delivery) -> (Delivery, Payment) {
        match self {
            Self::Default => {
                let id = Uuid::new_v4();

                tracing::info!(message = "processing payment via PaymentAdapter", ?id);
                tracing::info!(message = "awaiting confirmation from the external payment service");

                // NOTE: Тут объект "оплаты" создаётся сразу, с реальным внешним сервисом
                // тут была бы логика создания запроса, ожидания подтверждения и так далее.
                let payment = Payment {
                    id,
                    issue_date: Utc::now(),
                    payload: delivery.total_price(),
                };

                tracing::info!(message = "payment confirmed, yielding control back to the board");

                (delivery, payment)
            }
        }
    }
}
