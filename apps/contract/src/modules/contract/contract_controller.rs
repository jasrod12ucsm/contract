use std::{io::BufWriter, vec};

use bod_models::schemas::mst::user::{
    models::{contract::ContractBuilder, identification::Identification},
    user::{UserBuilder, UserWithId},
    user_errors::UserError,
};
use bson::DateTime;
use chrono::{Datelike, Local, NaiveDate};
use common::{
    helpers::smtp::smtp_functions::SmtpFunctions,
    middlewares::date_contract_structure::DateContractStructure,
    utils::{
        database::{
            domain::{
                database_query::DatabaseQueryTrait, filter_query::FilterQueryTrait,
                update_definition::UpdateDefinitionTrait, update_query::UpdateQueryTrait,
            },
            infrastructure::database_library::{DatabaseQuery, UpdateDefinition},
        },
        ntex_private::{
            extractors::json::JsonAdvanced,
            repository::public_repository::{AbstractRepository, PublicRepository},
        },
    },
};
use mongodb::results::{InsertOneResult, UpdateResult};
use ntex::web::{
    self,
    types::{Path, State},
    HttpRequest,
};
use printpdf::{BuiltinFont, IndirectFontRef, Mm, PdfDocument, PdfLayerReference};
use rsa::{pkcs1::pem::Base64Decoder, traits::PaddingScheme, Pkcs1v15Encrypt, RsaPrivateKey};

use crate::{
    modules::contract::models::create_contract::{
        filter_user::{FilterContractExist, FilterUserExist, UserFilter},
        push_user::PushUpdateContractBuilder,
    },
    utils::{jwt::generate::generate_jwt, user_repository::UserRepository},
};

use super::{
    data::{create_contract::CreateContract, renew_contract::RenewContract},
    models::path::price_path::PricePath,
};

#[web::post("create")]
pub async fn create_contract(
    create_contract_dto: JsonAdvanced<CreateContract>,
    repo: State<PublicRepository>,
    key: State<RsaPrivateKey>,
) -> Result<JsonAdvanced<InsertOneResult>, UserError> {
    let CreateContract {
        email,
        price,
        name,
        surnames,
        address,
        role,
        birthdate,
        date_end,
        date_start,
        enterprise_name,
        enterprise_represent,
        enterprise_ruc,
        represent_dni,
        dni,
    } = create_contract_dto.into_inner();
    println!("{:?}", key);

    let mensaje =
        base64::decode(price).map_err(|_| UserError::CreateUserError("error decoding price"))?;
    let key = key
        .decrypt(Pkcs1v15Encrypt, &mensaje)
        .map_err(|_| UserError::CreateUserError("error decrypting price"))?;
    let price =
        String::from_utf8(key).map_err(|_| UserError::CreateUserError("error converting price"))?;
    let price = price
        .parse()
        .map_err(|_| UserError::CreateUserError("error converting price"))?;
    println!("{:?}", price);
    let date_start_parsed = NaiveDate::parse_from_str(date_start.as_str(), "%Y-%m-%d").unwrap();
    let date_end_parsed = NaiveDate::parse_from_str(date_end.as_str(), "%Y-%m-%d").unwrap();
    let start_year = date_start_parsed.year();
    let start_month = date_start_parsed.month();
    let start_day = date_start_parsed.day();

    // Extraer el año, mes y día de date_end
    let end_year = date_end_parsed.year();
    let end_month = date_end_parsed.month();
    let end_day = date_end_parsed.day();
    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| UserError::CreateUserError("error acceding to database"))?;
    let user = user_repository
        .find_one(DatabaseQuery::find().filter(FilterUserExist {
            or: vec![
                UserFilter::Name { name: name.clone() },
                UserFilter::Surnames {
                    surnames: surnames.clone(),
                },
                UserFilter::Address {
                    address: address.clone(),
                },
                UserFilter::Role { role: role.clone() },
                UserFilter::Birthdate {
                    birthdate: birthdate.clone(),
                },
                UserFilter::Email {
                    email: email.clone(),
                },
            ],
        }))
        .await
        .map_err(|err| {
            println!("{:?}", err);
            UserError::GetUserError("error getting user")
        })?;
    if user.is_some() {
        return Err(UserError::CreateUserError("user already exist"));
    }

    //crear usuaru
    let user_insert = UserBuilder::default()
        .name(name.clone())
        .surnames(surnames.clone())
        .address(address.clone())
        .role(role.clone())
        .birthdate(birthdate.clone())
        .email(email.clone())
        .contract(vec![])
        .is_active(true)
        .is_deleted(false)
        .represent_dni(represent_dni.clone())
        .enterprise_name(enterprise_name.clone())
        .enterprise_represent(enterprise_represent.clone())
        .enterprise_ruc(enterprise_ruc.clone())
        .created_at(DateTime::now())
        .updated_at(DateTime::now())
        .identification(Identification {
            identification_number: dni.to_string(),
            identification_type: "dni".to_string(),
        })
        .build()
        .map_err(|err| {
            println!("{:?}", err);
            UserError::CreateUserError("error creating user see the data sended")
        })?;
    let insert_result = user_repository
        .insert_one(user_insert)
        .await
        .map_err(|_| UserError::CreateUserError("error creating user"))?;
    let user_id = user_repository
        .find_one(DatabaseQuery::find().filter(FilterUserExist {
            or: vec![
                UserFilter::Name { name: name.clone() },
                UserFilter::Surnames {
                    surnames: surnames.clone(),
                },
                UserFilter::Address {
                    address: address.clone(),
                },
                UserFilter::Role { role: role.clone() },
                UserFilter::Birthdate {
                    birthdate: birthdate.clone(),
                },
                UserFilter::Email {
                    email: email.clone(),
                },
            ],
        }))
        .await
        .map_err(|_| UserError::GetUserError("error getting user"))?
        .ok_or_else(|| UserError::CreateUserError("user not exist"))?
        .id;
    let (document, page1_index, index) = PdfDocument::new(
        "Contrato_empresa",
        printpdf::Mm(210.0),
        printpdf::Mm(297.0),
        "Layer 1",
    );

    let layer1 = document.get_page(page1_index).get_layer(index);
    let font = document
        .add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|_| UserError::CreateUserError("error creating font for pdf"))?;

    let mut y_position = Mm(280.0);

    // Función para añadir texto y actualizar la posición y
    fn add_text(
        layer: &PdfLayerReference,
        text: &str,
        font: &IndirectFontRef,
        font_size: f32,
        x: Mm,
        y: &mut Mm,
    ) {
        layer.use_text(text, font_size, x, *y, font);
        *y -= Mm(5.0); // Ajusta el valor según el espacio necesario entre líneas
    }

    // Añadir contenido del contrato
    // Añadir contenido del contrato
    add_text(
        &layer1,
        "CONTRATO DE TRABAJO",
        &font,
        24.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Conste por el presente documento, que se suscribe por triplicado con igual tenor y valor,",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "el contrato de trabajo sujeto a modalidad que al amparo del Texto Unico Ordenado del",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Decreto Legislativo N* 728, Decreto Supremo N* 003-97-TR, Ley de Productividad y",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Competitividad Laboral y normas complementarias, que celebran de una parte",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, &format!("{} con RUC. N* {} y domicilio real en {}, debidamente representada por el señor {}, con DNI N* {}", enterprise_name, enterprise_ruc, "", enterprise_represent, represent_dni), &font, 10.0, Mm(10.0), &mut y_position);
    add_text(
        &layer1,
        &format!(
            "y de la otra parte, don(ña) {} {}, con DNI N* {}, domiciliado en {}",
            name, surnames, dni, address
        ),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "en los términos y condiciones siguientes:",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "1.- EL EMPLEADOR es una empresa constructora, cuyo objeto social es El desarrollo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "de construcciones, parcelaciones o urbanizaciones en bienes propios o de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "terceros, bien sea para planes de vivienda, locales comerciales o industriales. La",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "construcción de estructuras para edificios, puentes e infraestructura en general en",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "concreto o metálicas. Y que ha sido debidamente autorizada por la MUNICIPALIDAD",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!(
            "PROVINCIAL DE CHACHAPOYAS, de fecha {} de {} del {}, emitida por la",
            start_day, start_month, start_year
        )
        .as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "resolución N” 2543, que requiere de los servicios del TRABAJADOR en forma temporal,",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "para trabajar por un periodo determinado en la construcción de un inmueble en el periodo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "de 4 meses.",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "2.- Por el presente contrato, EL TRABAJADOR se obliga a prestar sus servicios al",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!("EMPLEADOR para realizar las siguientes actividades:{role}").as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "debiendo someterse al cumplimiento estricto de la labor, para la cual ha sido",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "contratado, bajo las directivas de sus jefes o instructores, y las que se impartan por",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, "necesidades del servicio en ejercicio de las facultades de administración y dirección de la", &font, 10.0, Mm(10.0), &mut y_position);
    add_text(
        &layer1,
        "empresa, de conformidad con el artículo 9% del Texto Unico Ordenado de la Ley de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Productividad y Competitividad Laboral, aprobado por Decreto Supremo N* 003-97-TR.",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    let months_difference = (date_end_parsed.year() - date_start_parsed.year()) * 12
        + (date_end_parsed.month() as i32 - date_start_parsed.month() as i32);
    add_text(
        &layer1,
        format!("3.- La duración del presente contrato es de {months_difference} meses, iniciándose el día {} de {} {}",start_day,start_month,start_year).as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!(
            "y concluirá el día {} de {} {}",
            end_day, end_month, end_year
        )
        .as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "4.- En contraprestación a los servicios del TRABAJADOR, el EMPLEADOR se obliga a",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!(
            "pagar una remuneración mensual de S/.{}. Igualmente se obliga a facilitar al",
            price
        )
        .as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, "trabajador los materiales necesarios para que desarrolle sus actividades, y a otorgarle los", &font, 10.0, Mm(10.0), &mut y_position);
    add_text(
        &layer1,
        "beneficios que por ley, pacto o costumbre tuvieran los trabajadores del centro de trabajo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "contratados a plazo indeterminado.",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "5. EL TRABAJADOR deberá prestar sus servicios en el siguiente horario: de lunes a",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "sábado, de 10.00 am a 5.00 pm, teniendo un refrigerio de 60 minutos, que será tomado de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "10.00 pm a 1.00 pm",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "6.- EL EMPLEADOR, se obliga a inscribir al TRABAJADOR en el Libro de Planillas de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Remuneraciones, así como poner a conocimiento de la Autoridad Administrativa de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Trabajo el presente contrato, para su conocimiento y registro, en cumplimiento de lo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "dispuesto por artículo 73% del Texto Único ordenado del Decreto Legislativo N* 728, Ley",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "de Productividad y Competitividad laboral, aprobado mediante Decreto Supremo N* 003-",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, "97-TR.", &font, 10.0, Mm(10.0), &mut y_position);
    let token = generate_jwt(date_start.clone(), date_end.clone(), user_id)
        .map_err(|_| UserError::CreateUserError("token not exist"))?;
    let email_template_html = r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Token de Contrato</title>
    </head>
    <body>
        <p>Buenos días, su token de contrato es: {}</p>
    </body>
    </html>
    "#;
    let template = email_template_html.replace("{}", &token);
    let mut buffer: Vec<_> = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        let _ = document.save(&mut writer);
    }
    let _ = SmtpFunctions::send_email(email.as_str(), "Contract", &template, buffer)
        .map_err(|_| UserError::CreateUserError("error sending email"))?;
    let contrato = ContractBuilder::default()
        .init_date(date_start)
        .finish_date(date_end)
        .price(price)
        .build()
        .map_err(|_| UserError::CreateUserError("error creating contract"))?;
    let contrato = PushUpdateContractBuilder::default()
        .contract(contrato)
        .build()
        .unwrap();
    let result = user_repository
        .update_one(
            DatabaseQuery::update()
                .update(UpdateDefinition::default().push(contrato))
                .filter(FilterUserExist {
                    or: vec![
                        UserFilter::Name { name: name.clone() },
                        UserFilter::Surnames {
                            surnames: surnames.clone(),
                        },
                        UserFilter::Address {
                            address: address.clone(),
                        },
                        UserFilter::Role { role: role.clone() },
                        UserFilter::Birthdate {
                            birthdate: birthdate.clone(),
                        },
                        UserFilter::Email {
                            email: email.clone(),
                        },
                    ],
                }),
        )
        .await
        .map_err(|_| UserError::CreateUserError("error creating contract"))?;
    println!("{:?}", result);
    Ok(JsonAdvanced(insert_result))
}

#[web::post("renew")]
pub async fn renew_contract(
    dates: JsonAdvanced<RenewContract>,
    req: HttpRequest,
    repo: State<PublicRepository>,
    key: State<RsaPrivateKey>,
) -> Result<JsonAdvanced<UpdateResult>, UserError> {
    let RenewContract {
        date_start,
        date_end,
        price,
    } = dates.into_inner();
    let date_start_parsed = NaiveDate::parse_from_str(date_start.as_str(), "%Y-%m-%d").unwrap();
    let date_end_parsed = NaiveDate::parse_from_str(date_end.as_str(), "%Y-%m-%d").unwrap();
    let start_year = date_start_parsed.year();
    let start_month = date_start_parsed.month();
    let start_day = date_start_parsed.day();

    // Extraer el año, mes y día de date_end
    let end_year = date_end_parsed.year();
    let end_month = date_end_parsed.month();
    let end_day = date_end_parsed.day();
    let data_token = req
        .extensions()
        .get::<DateContractStructure>()
        .cloned()
        .ok_or_else(|| UserError::UpdateUserError("cannot read token data"))?;
    let data_token_copy = data_token.clone();
    let data_token_ref = &data_token;

    let mensaje =
        base64::decode(price).map_err(|_| UserError::CreateUserError("error decoding price"))?;
    let key = key
        .decrypt(Pkcs1v15Encrypt, &mensaje)
        .map_err(|_| UserError::CreateUserError("error decrypting price"))?;
    let price =
        String::from_utf8(key).map_err(|_| UserError::CreateUserError("error converting price"))?;
    println!("{}", price);
    let price = price
        .parse()
        .map_err(|_| UserError::CreateUserError("error converting price"))?;
    print!("{}",price);
    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| UserError::CreateUserError("error acceding to database"))?;
    //buscar el contrato, si existe renuevalo
    println!("{:?}", data_token);
    let contract_finded = user_repository
        .find_one(DatabaseQuery::find().filter(FilterContractExist {
            id: data_token.id,
            init_date: data_token_copy.start_date,
            finish_date: data_token_copy.finish_date,
        }))
        .await
        .map_err(|_| UserError::GetUserError("error getting user"))?;
    if contract_finded.is_none() {
        return Err(UserError::UpdateUserError("usuario de contrato no existe"));
    }
    let UserWithId {
        id: _,
        identification,
        name,
        surnames,
        address,
        role,
        created_at: _,
        updated_at: _,
        birthdate,
        is_active: _,
        is_deleted: _,
        email,
        contract: _,
        enterprise_name,
        enterprise_represent,
        enterprise_ruc,
        represent_dni,
    } = contract_finded.clone().unwrap();
    let contract = contract_finded
        .unwrap()
        .contract
        .into_iter()
        .find(|item| {
            item.finish_date == data_token_ref.finish_date
                && item.init_date == data_token_ref.start_date.to_string()
        })
        .unwrap();

    let start_date = NaiveDate::parse_from_str(&contract.init_date, "%Y-%m-%d").unwrap();
    let finish_date = NaiveDate::parse_from_str(&contract.finish_date, "%Y-%m-%d").unwrap();
    let current_date = Local::now().naive_local().date();

    if current_date >= start_date && current_date <= finish_date {
        return Err(UserError::UpdateUserError("contrato ya activo"));
    }
    let (document, page1_index, index) = PdfDocument::new(
        "Contrato_empresa",
        printpdf::Mm(210.0),
        printpdf::Mm(297.0),
        "Layer 1",
    );

    let layer1 = document.get_page(page1_index).get_layer(index);
    let font = document
        .add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|_| UserError::CreateUserError("error creating font for pdf"))?;

    let mut y_position = Mm(280.0);

    // Función para añadir texto y actualizar la posición y
    fn add_text(
        layer: &PdfLayerReference,
        text: &str,
        font: &IndirectFontRef,
        font_size: f32,
        x: Mm,
        y: &mut Mm,
    ) {
        layer.use_text(text, font_size, x, *y, font);
        *y -= Mm(5.0); // Ajusta el valor según el espacio necesario entre líneas
    }

    // Añadir contenido del contrato
    add_text(
        &layer1,
        "CONTRATO DE TRABAJO",
        &font,
        24.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Conste por el presente documento, que se suscribe por triplicado con igual tenor y valor,",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "el contrato de trabajo sujeto a modalidad que al amparo del Texto Unico Ordenado del",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Decreto Legislativo N* 728, Decreto Supremo N* 003-97-TR, Ley de Productividad y",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Competitividad Laboral y normas complementarias, que celebran de una parte",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, &format!("{} con RUC. N* {} y domicilio real en {}, debidamente representada por el señor {}, con DNI N* {}", enterprise_name, enterprise_ruc, "", enterprise_represent, represent_dni), &font, 10.0, Mm(10.0), &mut y_position);
    add_text(
        &layer1,
        &format!(
            "y de la otra parte, don(ña) {} {}, con DNI N* {}, domiciliado en {}",
            name, surnames, identification.identification_number, address
        ),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "en los términos y condiciones siguientes:",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "1.- EL EMPLEADOR es una empresa constructora, cuyo objeto social es El desarrollo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "de construcciones, parcelaciones o urbanizaciones en bienes propios o de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "terceros, bien sea para planes de vivienda, locales comerciales o industriales. La",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "construcción de estructuras para edificios, puentes e infraestructura en general en",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "concreto o metálicas. Y que ha sido debidamente autorizada por la MUNICIPALIDAD",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!(
            "PROVINCIAL DE CHACHAPOYAS, de fecha {} de {} del {}, emitida por la",
            start_day, start_month, start_year
        )
        .as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "resolución N” 2543, que requiere de los servicios del TRABAJADOR en forma temporal,",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "para trabajar por un periodo determinado en la construcción de un inmueble en el periodo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "de 4 meses.",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "2.- Por el presente contrato, EL TRABAJADOR se obliga a prestar sus servicios al",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!("EMPLEADOR para realizar las siguientes actividades:{role}").as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "debiendo someterse al cumplimiento estricto de la labor, para la cual ha sido",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "contratado, bajo las directivas de sus jefes o instructores, y las que se impartan por",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, "necesidades del servicio en ejercicio de las facultades de administración y dirección de la", &font, 10.0, Mm(10.0), &mut y_position);
    add_text(
        &layer1,
        "empresa, de conformidad con el artículo 9% del Texto Unico Ordenado de la Ley de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Productividad y Competitividad Laboral, aprobado por Decreto Supremo N* 003-97-TR.",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    let months_difference = (date_end_parsed.year() - date_start_parsed.year()) * 12
        + (date_end_parsed.month() as i32 - date_start_parsed.month() as i32);
    add_text(
        &layer1,
        format!("3.- La duración del presente contrato es de {months_difference} meses, iniciándose el día {} de {} {}",start_day,start_month,start_year).as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!(
            "y concluirá el día {} de {} {}",
            end_day, end_month, end_year
        )
        .as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "4.- En contraprestación a los servicios del TRABAJADOR, el EMPLEADOR se obliga a",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        format!(
            "pagar una remuneración mensual de S/.{}. Igualmente se obliga a facilitar al",
            price
        )
        .as_str(),
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, "trabajador los materiales necesarios para que desarrolle sus actividades, y a otorgarle los", &font, 10.0, Mm(10.0), &mut y_position);
    add_text(
        &layer1,
        "beneficios que por ley, pacto o costumbre tuvieran los trabajadores del centro de trabajo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "contratados a plazo indeterminado.",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "5. EL TRABAJADOR deberá prestar sus servicios en el siguiente horario: de lunes a",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "sábado, de 10.00 am a 5.00 pm, teniendo un refrigerio de 60 minutos, que será tomado de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "10.00 pm a 1.00 pm",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );

    add_text(
        &layer1,
        "6.- EL EMPLEADOR, se obliga a inscribir al TRABAJADOR en el Libro de Planillas de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Remuneraciones, así como poner a conocimiento de la Autoridad Administrativa de",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "Trabajo el presente contrato, para su conocimiento y registro, en cumplimiento de lo",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "dispuesto por artículo 73% del Texto Único ordenado del Decreto Legislativo N* 728, Ley",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(
        &layer1,
        "de Productividad y Competitividad laboral, aprobado mediante Decreto Supremo N* 003-",
        &font,
        10.0,
        Mm(10.0),
        &mut y_position,
    );
    add_text(&layer1, "97-TR.", &font, 10.0, Mm(10.0), &mut y_position);

    let token = generate_jwt(date_start.clone(), date_end.clone(), (&data_token).id)
        .map_err(|_| UserError::CreateUserError("token not exist"))?;
    let email_template_html = r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Token de Contrato</title>
    </head>
    <body>
        <p>Buenos días, su token de contrato es: {}</p>
    </body>
    </html>
    "#;
    let template = email_template_html.replace("{}", &token);
    let mut buffer: Vec<_> = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        let _ = document.save(&mut writer);
    }
    let _ = SmtpFunctions::send_email(email.as_str(), "Contract", &template, buffer)
        .map_err(|_| UserError::CreateUserError("error sending email"))?;

    let contrato = ContractBuilder::default()
        .init_date(date_start)
        .finish_date(date_end)
        .price(price)
        .build()
        .map_err(|_| UserError::CreateUserError("error creating contract"))?;
    let contrato = PushUpdateContractBuilder::default()
        .contract(contrato)
        .build()
        .unwrap();
    let update_result = user_repository
        .update_one(
            DatabaseQuery::update()
                .update(UpdateDefinition::default().push(contrato))
                .filter(FilterUserExist {
                    or: vec![
                        UserFilter::Name { name: name.clone() },
                        UserFilter::Surnames {
                            surnames: surnames.clone(),
                        },
                        UserFilter::Address {
                            address: address.clone(),
                        },
                        UserFilter::Role { role: role.clone() },
                        UserFilter::Birthdate {
                            birthdate: birthdate.clone(),
                        },
                        UserFilter::Email {
                            email: email.clone(),
                        },
                    ],
                }),
        )
        .await
        .map_err(|_| UserError::CreateUserError("error creating contract"))?;
    Ok(JsonAdvanced(update_result))
}
