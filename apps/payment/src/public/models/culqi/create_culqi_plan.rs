use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCulqiPlan {
    name: String,
    short_name: String,
    description: String,
    amount: i32,
    currency: Currency,
    interval_unit_time: IntervalUnitTime,
    interval_count: i32,
    initial_cycles: InitialCycles,
}

impl CreateCulqiPlan {
    pub fn builder() -> CreateCulqiPlanBuilder {
        CreateCulqiPlanBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateCulqiPlanBuilder {
    name: Option<String>,
    short_name: Option<String>,
    description: Option<String>,
    amount: Option<i32>,
    currency: Option<Currency>,
    interval_unit_time: Option<IntervalUnitTime>,
    interval_count: Option<i32>,
    initial_cycles: Option<InitialCycles>,
}

impl CreateCulqiPlanBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn short_name(mut self, short_name: String) -> Self {
        self.short_name = Some(short_name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn amount(mut self, amount: i32) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn interval_unit_time(mut self, interval_unit_time: IntervalUnitTime) -> Self {
        self.interval_unit_time = Some(interval_unit_time);
        self
    }

    pub fn interval_count(mut self, interval_count: i32) -> Self {
        self.interval_count = Some(interval_count);
        self
    }

    pub fn initial_cycles(mut self, initial_cycles: InitialCycles) -> Self {
        self.initial_cycles = Some(initial_cycles);
        self
    }

    pub fn build(self) -> Result<CreateCulqiPlan, &'static str> {
        Ok(CreateCulqiPlan {
            name: self.name.ok_or("name is required")?,
            short_name: self.short_name.ok_or("short_name is required")?,
            description: self.description.ok_or("description is required")?,
            amount: self.amount.ok_or("amount is required")?,
            currency: self.currency.ok_or("currency is required")?,
            interval_unit_time: self.interval_unit_time.ok_or("interval_unit_time is required")?,
            interval_count: self.interval_count.ok_or("interval_count is required")?,
            initial_cycles: self.initial_cycles.ok_or("initial_cycles is required")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialCycles {
    count: i32,
    has_initial_charge: bool,
    amount: i32,
    interval_unit_time: IntervalUnitTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    PEN,
    USD,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IntervalUnitTime {
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
    Yearly = 4,
    Quarterly = 5,
    SemiAnnual = 6,
}