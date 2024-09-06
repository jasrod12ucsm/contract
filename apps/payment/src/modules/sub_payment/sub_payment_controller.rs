use ntex::web::{self, Responder};

//* datos */
//* primero que nada el id_account de stripe*/
//* 2do precio del plan */
//* 3ro metodos de pago */
//* precio del cobro porque el plan puede variar */
//* precio del plan */
//* 4to por address seran los pagos ahora mismo no importa */
//* 5to guardar pago */
//* 6to separar pago para igv tmb */
//* aqui se guardaran pagos mensuales a nuetra app*/
//* este es el primer pago que se realiza */
//* cuando hayan mas pagos se tiene que calcular por tiendas*/
//* si no hay tiendas se cobra directamente a la compañia */
//*cuando agregue tiendas se tiene que calcular por la cantidaad de tiendas  */
//*este metodo es el primero y es por compañia */
//*primero terminar el company, ahi se guardara el stripe_id */
//*antes que nada actualiza el token si es un usuario c que revuelva el token, si no, */
//* verifica la geolocation para traerte los id de las tiendas que va a mostrar*/




#[web::post("pay")]
pub async fn validateTarget() -> impl Responder {
    ""
}
