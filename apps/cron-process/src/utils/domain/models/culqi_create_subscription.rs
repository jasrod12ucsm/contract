use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CulqiCreateSubscription {
    card_id: String,
    plan_id: String,
    tyc: bool,
}

pub struct CulqiCreateSubscriptionBuilder {
    card_id: String,
    plan_id: String,
    tyc: bool,
}

impl CulqiCreateSubscriptionBuilder {
    pub fn new() -> Self {
        Self {
            card_id: String::new(),
            plan_id: String::new(),
            tyc: true, // tyc es verdadero por defecto
        }
    }

    pub fn card_id(mut self, card_id: String) -> Self {
        self.card_id = card_id;
        self
    }

    pub fn plan_id(mut self, plan_id: String) -> Self {
        self.plan_id = plan_id;
        self
    }

    pub fn build(self) -> CulqiCreateSubscription {
        CulqiCreateSubscription {
            card_id: self.card_id,
            plan_id: self.plan_id,
            tyc: self.tyc,
        }
    }
}