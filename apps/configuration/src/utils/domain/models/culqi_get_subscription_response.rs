use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CulqiGetSubscriptionResponse {
    pub id: String, // ID de la suscripción.
    pub status: i32, // Estado de la suscripción.
    pub creation_date: i64, // Fecha de creación de la suscripción en formato UNIX Timestamp.
    pub next_billing_date: i64, // Próxima fecha de facturación en formato UNIX Timestamp.
    pub current_period: i32, // Número del periodo de facturación actual.
    pub trial_start: i64, // Fecha de inicio del período de prueba en formato UNIX Timestamp.
    pub trial_end: i64, // Fecha de finalización del período de prueba en formato UNIX Timestamp.
    pub active_card: String, // ID de la tarjeta activa en la suscripción.
    pub plan: Plan, // Detalles del plan de suscripción.
    pub periods: Vec<Period>, // Array de períodos de facturación.
    pub charges: Vec<Charge>, // Array de cargos.
}

#[derive(Serialize, Deserialize)]
pub struct Plan {
    pub plan_id: String, // ID del plan de suscripción.
    pub name: String, // Nombre del plan de suscripción.
    pub amount: i32, // Monto del plan de suscripción.
    pub currency: String, // Moneda del plan de suscripción.
    pub interval_unit_time: i32, // Unidad de tiempo del intervalo del plan de suscripción.
}

#[derive(Serialize, Deserialize)]
pub struct Period {
    pub period: i32, // Número del período de facturación.
    pub status: i32, // Estado del período de facturación.
}

#[derive(Serialize, Deserialize)]
pub struct Charge {
    pub card_id: String, // ID de la tarjeta utilizada para el cargo.
    pub card_number: String, // Número de tarjeta enmascarado.
    pub card_brand: String, // Marca de la tarjeta.
    pub charge_id: String, // ID del cargo.
    pub charger_status: i32, // Estado del cargo.
    pub charge_day: i64, // Marca de tiempo Unix de la fecha del cargo.
    pub error: Option<String>, // Mensaje de error (si lo hay).
    pub amount: i32, // Monto del cargo.
    pub currency: String, // Moneda del cargo.
}