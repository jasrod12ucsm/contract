use crate::{
    modules::authentication::{
        data::{
            authenticate_post_code::AuthenticatePostCode, login_by_token_dto::LoginByTokenDto,
            login_client_dto::LoginCLientDto, register_user_client_dto::RegisterUserClientDto,
            renew_token_dto::RenewTokenDto,
        },
        models::{
            email_sended::EmailSended, login_result::LoginResult, renew_result::RenewResult,
            user_id::UserId,
        },
    },
    utils::infrastructure::{
        email_functions::EmailFunctions,
        jwt::generate::{generate_jwt, generate_refresh_jwt},
        repositories::{
            app_variables_repository::AppVariablesRepository,
            card_plan_repository::CardPlanRepository, company_repository::CompanyRepository,
            country_repository::CountryRepository,
            email_template_repository::EmailTemplateRepository,
            region_repository::RegionRepository, reset_token_repository::ResetTokenRepository,
            restaurant_repository::RestaurantRepository,
            user_config_repository::UserConfigRepository, user_repository::UserRepository,
        },
    },
    FINDER,
};
use bod_models::{
    schemas::{
        config::{
            company::{
                company::{Sensible, SocialNetworks},
                company_attributes::CompanyAttributesBuilder,
            },
            reset_token::{
                reset_token_attributes::ResetTokenAttributes, reset_token_errors::ResetTokenError,
            },
            user_config::{
                models::short_user_config::ShortUserConfig,
                user_config_attributes::UserConfigAttributes, user_config_errors::UserConfigError,
            },
        },
        location::{
            country::models::short_country::ShortCountry, region::models::short_region::ShortRegion,
        },
        mst::{
            restaurant::restaurant_attributes::RestaurantAttributesBuilder,
            user::{
                models::{atention_hour::AtentionHourBuilder, identification::Identification},
                user_attributes::UserAttributes,
                user_errors::UserError,
            },
        },
    },
    shared::{bson::to_bson::ToBson, geo_point::GeoPoint, jwt::claims::RenewClaims},
};
use bson::{oid::ObjectId, DateTime};
use common::{
    helpers::{
        env::env::ENV, password::password_functions::PasswordFunctions,
        smtp::smtp_functions::SmtpFunctions,
    },
    public::models::path::IdPath,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};

use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::{bson::doc, options::SelectionCriteria};
use ntex::{
    util::Either,
    web::{
        self,
        types::{Json, Path, State},
        HttpRequest,
    },
};
use std::str::FromStr;

#[web::post("/singup/client")]
pub async fn singup_client(
    register_dto: JsonAdvanced<RegisterUserClientDto>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<UserId>, Either<UserConfigError, UserError>> {
    let RegisterUserClientDto {
        names,
        surnames,
        email,
        password,
        identification_number,
        phone,
        address,
        identification_type,
        country_code,
        region_code,
        birthdate,
        longitude,
        latitude,
        card_plan,
    } = register_dto.into_inner();
    //iniciamos repositorios
    let user_config_repository: UserConfigRepository = repo
        .get_repository::<UserConfigRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;

    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    let company_repository: CompanyRepository = repo
        .get_repository::<CompanyRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    let country_repository: CountryRepository = repo
        .get_repository::<CountryRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    let region_repository: RegionRepository = repo
        .get_repository::<RegionRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    let card_ṕlan_repository: CardPlanRepository = repo
        .get_repository::<CardPlanRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    //trae email template repository
    let email_template_repository = repo
        .get_repository::<EmailTemplateRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    //app_v ariables repository
    let app_variables_repository: AppVariablesRepository = repo
        .get_repository::<AppVariablesRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    let restaurant_repository: RestaurantRepository = repo
        .get_repository::<RestaurantRepository>()
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("internal code error")))?;
    //creamos session para la transaccion
    let mut session = repo
        .get_client()
        .unwrap()
        .start_session()
        .await
        .map_err(|_| {
            Either::Left(UserConfigError::CreateUserError(
                "secure transactions don't start",
            ))
        })?;
    session.start_transaction().await.unwrap();

    //si encuentra el email verifica si esta authenticado, si lo esta regresa error, si no continuea con el codigo
    let country = country_repository
        .find_one(doc! {"code":country_code.clone()})
        .session(&mut session)
        .selection_criteria(SelectionCriteria::ReadPreference(
            mongodb::options::ReadPreference::Primary,
        ))
        .await
        .map_err(|err| {
            println!("{:?}", err);
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("country not correct"))
        })?
        .ok_or_else(|| Either::Right(UserError::CreateUserError("")))?;

    let find_region_document =
        doc! {"code":region_code.clone(), "countryId":country_code,"noDeleted":true};
    let region = region_repository
        .find_one(find_region_document)
        .session(&mut session)
        .selection_criteria(SelectionCriteria::ReadPreference(
            mongodb::options::ReadPreference::Primary,
        ))
        .await
        .map_err(|_| {
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("internal error"))
        })?
        .ok_or_else(|| Either::Right(UserError::CreateUserError("incorrect region")))?;
    let user_config = user_config_repository
        .find_one(doc! {"email":email.clone()})
        .selection_criteria(SelectionCriteria::ReadPreference(
            mongodb::options::ReadPreference::Primary,
        ))
        .session(&mut session)
        .await
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("error finding email")))?;
    if user_config.is_some() {
        if user_config.unwrap().is_authenticated {
            return Err(Either::Left(UserConfigError::UserAlreadyExists(
                "Usuario ya existe",
            )));
        }
    }
    //traigamos el country con su codigo
    let encrypted_password = PasswordFunctions::hash_password(password.as_str())
        .map_err(|_| Either::Left(UserConfigError::CreateUserError("encryptation error")))?;
    let user_config_to_insert =
        UserConfigAttributes::new_client(names, surnames, email.clone(), encrypted_password);

    let update_doc = doc! {
        "$set":bson::to_bson(&user_config_to_insert).unwrap()
    };
    println!("{:?}", update_doc);
    let user_config_inserted = user_config_repository
        .find_one_and_update(doc! {"email":email.clone()}, update_doc)
        .session(&mut session)
        .upsert(true)
        .await
        .map_err(|err| {
            let _ = session.abort_transaction();
            println!("{:?}", err);
            return Either::Left(UserConfigError::CreateUserError("error inserting user"));
        })?
        .ok_or_else(|| Either::Left(UserConfigError::CreateUserError("error inserting user")))?;
    println!("paso");
    //creamos token, pero usamos una tabla para validar el token, si existe y lo guardamos
    //1. crear token

    //generar un numero de 6 digitos aleatorio
    let code = PasswordFunctions::generate_random_number();
    //enviar el codigo al email

    //3. devolver token
    let user_config_with_id = user_config_inserted.clone();

    let user_config_with_id: ShortUserConfig = user_config_with_id.into();

    //4. Crear usuario principal
    let user = UserAttributes::new_client(
        user_config_with_id.clone().id,
        Identification {
            identification_number,
            identification_type,
        },
        phone,
        address.clone(),
        country.clone().into(),
        region.clone().into(),
        birthdate,
        "C".to_string(),
    );
    let doc_insert_user = doc! {
        "$set":bson::to_bson(&user).unwrap()
    };
    println!("{:?}", doc_insert_user);
    //insertamos usuario
    let user_inserted = user_repository
        .find_one_and_update(doc! {"_id":user_config_with_id.id}, doc_insert_user)
        .session(&mut session)
        .upsert(true)
        .await
        .map_err(|_err| {
            println!("{:?}", _err);
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("error inserting user"))
        })?
        .ok_or_else(|| {
            println!("{:?}", "error inserting user");
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("error inserting user"))
        })?;
    //2. guardar token en la tabla token
    let reset_token_to_insert = ResetTokenAttributes::new(user_inserted.id, code);
    let doc_insert_token = doc! {
        "$set":bson::to_bson(&reset_token_to_insert).unwrap()
    };

    let _reset_token_insertion = reset_token_repository
        .find_one_and_update(
            doc! {"userId":user_inserted.id,"noActive":true},
            doc_insert_token,
        )
        .session(&mut session)
        .upsert(true)
        .await
        .map_err(|_| {
            let _ = session.abort_transaction();
            Either::Left(UserConfigError::CreateUserError(
                "error updating token table",
            ))
        })?;
    //creamos una insersion de restaurant
    let name = format!("{}-{}-res", longitude.clone(), latitude.clone());
    let timezone = FINDER.get_tz_name(longitude, latitude);
    let restaurant_insertion_data = RestaurantAttributesBuilder::default()
        .address(address)
        .close_hour(AtentionHourBuilder::default().build().unwrap())
        .open_hour(AtentionHourBuilder::default().build().unwrap())
        .country::<ShortCountry>(country.clone().into())
        .region::<ShortRegion>(region.clone().into())
        .name(name.clone())
        .location(GeoPoint::new(longitude, latitude))
        .time_zone(timezone)
        .is_active(true)
        .is_deleted(false)
        .company_id(user_inserted.id)
        .employee_count(0)
        .content_type_ids(vec![])
        .build()
        .map_err(|_| {
            Either::Left(UserConfigError::CreateUserError(
                "error updating token table",
            ))
        })?
        .to_bson()
        .map_err(|_| {
            Either::Left::<UserConfigError, UserError>(UserConfigError::CreateUserError(
                "error updating token table",
            ))
        })?;
    //TODO generar subscription culqi
    //buscamos el id del cardplan
    let card_plan = card_ṕlan_repository
        .find_one(doc! {"_id":card_plan,"noDeleted":true})
        .session(&mut session)
        .selection_criteria(SelectionCriteria::ReadPreference(
            mongodb::options::ReadPreference::Primary,
        ))
        .await
        .map_err(|err| {
            println!("{:?}", err);
            let _ = session.abort_transaction();
            Either::Left(UserConfigError::CreateUserError("error finding card plan"))
        })?
        .ok_or_else(|| Either::Left(UserConfigError::CreateUserError("card plan not found")))?;
    //TODO actualizacion de logo y todo con null
    let company_insertion_data = CompanyAttributesBuilder::default()
        .id(user_inserted.id)
        .country::<ShortCountry>(country.into())
        .region::<ShortRegion>(region.into())
        .employee_count(0)
        .sensible(Sensible::default())
        .logo(None)
        .large_logo(None)
        .small_logo(None)
        .name(None)
        .website(None)
        .display_name(None)
        .mission(None)
        .vision(None)
        .categories(None)
        .social(SocialNetworks::default())
        .card_plan(card_plan.id)
        .emails(vec![])
        .quantity_restaurant(1)
        .is_active(true)
        .is_deleted(false)
        .build()
        .map_err(|err| {
            println!("{}", err);
            Either::Left(UserConfigError::CreateUserError("error creating company"))
        })?
        .to_bson()
        .map_err(|err| {
            println!("{}", err);
            Either::Left(UserConfigError::CreateUserError("error creating company"))
        })?;
    let _ = company_repository
        .find_one_and_update(doc! {"_id":user_config_inserted.id}, company_insertion_data)
        .session(&mut session)
        .upsert(true)
        .await
        .map_err(|err| {
            let _ = session.abort_transaction();
            println!("{}", err);
            Either::Left(UserConfigError::CreateUserError(
                "error on company insertion",
            ))
        })?;

    let _ = restaurant_repository
        .find_one_and_update(doc! {"name":name}, restaurant_insertion_data)
        .session(&mut session)
        .upsert(true)
        .await
        .map_err(|_| {
            let _ = session.abort_transaction();
            Either::Left::<UserConfigError, UserError>(UserConfigError::CreateUserError(
                "error on restaurant insertion",
            ))
        });
    session
        .commit_transaction()
        .await
        .map_err(|_err| Either::Right(UserError::CreateUserError("error commiting transaction")))?;
    let data_to_return = UserId {
        user: user_inserted.id.to_string(),
        email: user_config_inserted.email,
    };

    //usa el email template repository para buscar
    let email_template = email_template_repository
        .find_one(doc! {"templateName":"register","noActive":true})
        .await
        .map_err(|_| {
            Either::Left(UserConfigError::CreateUserError(
                "error finding email template",
            ))
        })?;
    if email_template.is_none() {
        return Err(Either::Left(UserConfigError::CreateUserError(
            "no existe template",
        )));
    }
    let app_variables = match app_variables_repository
        .find_one(doc! {"_id":{"$exists":true},"noDeleted":true,"noActive":true})
        .session(&mut session)
        .await
    {
        Ok(Some(variables)) => variables,
        Ok(None) => {
            let _ = session.abort_transaction();
            return Err(Either::Left(UserConfigError::CreateUserError(
                "app variables not found",
            )));
        }
        Err(err) => {
            println!("{:?}", err);
            let _ = session.abort_transaction();
            return Err(Either::Left(UserConfigError::CreateUserError(
                "error finding app variables",
            )));
        }
    };
    let mut complete_name = user_config_inserted.names;
    complete_name.push_str(" ");
    complete_name.push_str(&user_config_inserted.surnames);
    let email_template_html = email_template.unwrap().html;
    let code_str = &code.to_string();
    let render_html = EmailFunctions::replace_placeholders(
        email_template_html,
        vec![
            app_variables.app_name.as_str(),
            complete_name.as_str(),
            format!(
                "{}-{}",
                &code_str[..3], // Primer parte del código (los primeros 3 dígitos)
                &code_str[3..]
            )
            .as_str(),
            app_variables.facebook_link.as_str(),
            app_variables.instagram_link.as_str(),
            app_variables.phone.as_str(),
            app_variables.whatsapp_link.as_str(),
        ],
    )
    .replace("\\n", "")
    .replace("\\\"", "\"");
    println!("{}", render_html);
    // Enviar el email de forma asincrónica sin bloquear la función principal
    tokio::spawn(async move {
        let _ = SmtpFunctions::send_email(email.as_str(), "Enable Account", &render_html);
    });
    Ok(JsonAdvanced(data_to_return))
}

//resend email
#[web::get("resend/email/{id}")]
pub async fn resend_email(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<EmailSended>, ResetTokenError> {
    //aqui enviamos el email
    let id_path = path.id();
    let object_id_path =
        ObjectId::from_str(id_path).map_err(|_| ResetTokenError::GetTokenError("error in id"))?;
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| ResetTokenError::GetTokenError("internal server erro"))?;
    //email template repository+
    let email_template_repository = repo
        .get_repository::<EmailTemplateRepository>()
        .await
        .map_err(|_| ResetTokenError::GetTokenError("internal server error"))?;
    let user_config_repository: UserConfigRepository = repo
        .get_repository::<UserConfigRepository>()
        .await
        .map_err(|_| ResetTokenError::GetTokenError("internal server error"))?;
    let current_user_config = user_config_repository
        .find_one(doc! {"_id":object_id_path})
        .await
        .map_err(|_| ResetTokenError::GetTokenError("cannot get that user"))?
        .ok_or_else(|| ResetTokenError::GetTokenError("user configuration not exist"))?;
    //verifica autentificacion
    if current_user_config.is_authenticated {
        return Err(ResetTokenError::GetTokenError("user is authenticated"));
    }

    //generar random number
    let code = PasswordFunctions::generate_random_number();
    //actualizar la tabla de reset_token con ese random_number

    let filter = doc! {
        "userId":object_id_path,"noActive":true
    };
    let update_doc = doc! {
        "$set":{
            "authCode":code
        }
    };
    println!("anets de transaccion");
    let _reset_token_insertion = reset_token_repository
        .update_one(filter, update_doc)
        .await
        .map_err(|_| ResetTokenError::GetTokenError("critical error is passing"))?;
    //ahora genera el html
    println!("paso");
    let email_template = email_template_repository
        .find_one(doc! {"templateName":"register","noActive":true})
        .await
        .map_err(|_| ResetTokenError::GetTokenError("cannot render template"))?;
    if email_template.is_none() {
        return Err(ResetTokenError::GetTokenError("template not exist"));
    }
    println!("{:?}", email_template);
    let mut complete_name = current_user_config.names;
    complete_name.push_str(" ");
    complete_name.push_str(&current_user_config.surnames);
    let email_template_html = email_template.unwrap().html;
    let code_str = &code.to_string();
    let render_html = EmailFunctions::replace_placeholders(
        email_template_html,
        vec![
            "Instant",
            complete_name.as_str(),
            format!(
                "{}-{}",
                &code_str[..3], // Primer parte del código (los primeros 3 dígitos)
                &code_str[3..]
            )
            .as_str(),
            "https://www.facebook.com",
            "https://www.instagram.com",
            "https://www.google.com",
            "https://www.google.com",
        ],
    )
    .replace("\\n", "")
    .replace("\\\"", "\"");
    // Enviar el email de forma asincrónica sin bloquear la función principal
    let email_sended = SmtpFunctions::send_email(
        current_user_config.email.as_str(),
        "Enable Account",
        &render_html,
    );
    if email_sended.is_err() {
        return Ok(JsonAdvanced(EmailSended { ok: false }));
    }

    Ok(JsonAdvanced(EmailSended { ok: true }))
}

//authenticate
#[web::post("authenticate/{id}")]
pub async fn authenticate(
    id_path: Path<IdPath>,
    code: JsonAdvanced<AuthenticatePostCode>,
    repo: State<PublicRepository>,
) -> Result<Json<EmailSended>, UserConfigError> {
    //
    let id = ObjectId::from_str(id_path.id()).unwrap();
    //traermos el repo de los token
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Internal code error"))?;
    //traemos el repo de user_config
    let user_config_repository: UserConfigRepository = repo
        .get_repository::<UserConfigRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Internal code error"))?;
    //ahora preguntamos por el token
    let token_register = reset_token_repository
        .find_one(doc! {"userId":id,"noActive":true})
        .await
        .map_err(|_| UserConfigError::LoginUserError("User not finded"))?;
    if token_register.is_none() {
        return Err(UserConfigError::LoginUserError("No existe token generado"));
    }
    let token = token_register.unwrap();
    //pregunta por el codigo
    let AuthenticatePostCode { code } = code.into_inner();
    //quiero validar que el code del token no se haya actualizado hace mas de dos minutos usando DATETIME DE MONGO
    let current_time = DateTime::now();
    let diff = current_time.checked_duration_since(token.updated_at);
    if diff.is_none() {
        return Err(UserConfigError::LoginUserError("Codigo expirado"));
    }
    //valida que la diferencia de tiempo no sea de ams de dos minutos para saguir
    let diff = diff.unwrap();
    if diff.as_secs() > 120 {
        return Err(UserConfigError::LoginUserError("Codigo expirado"));
    }
    if token.auth_code != code {
        return Err(UserConfigError::LoginUserError("Codigo incorrecto"));
    }
    //actualizamos tabla userConfig con isAuthenticated 1
    //aqui el fiiltro
    let filter = doc! {
        "_id": token.user_id
    };
    let update_doc = doc! {
        "$set":{
            "isAuthenticated":true
        }
    };
    let actualized_register = user_config_repository
        .find_one_and_update(filter, update_doc)
        .upsert(true)
        .await
        .map_err(|_| UserConfigError::LoginUserError("Internal update error"))?;
    if actualized_register.is_none() {
        return Err(UserConfigError::LoginUserError(
            "no se actualizo ningun registro",
        ));
    }
    Ok(Json(EmailSended { ok: true }))
}

#[web::post("login/client")]
pub async fn login_client(
    login_dto: JsonAdvanced<LoginCLientDto>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<LoginResult>, UserConfigError> {
    // No hacemos validación del tipo de cuenta, ya que todos los tipos pueden entrar como clientes
    let LoginCLientDto {
        os,
        email,
        password,
        mac,
    } = login_dto.into_inner();
    // Crear repos
    let user_config_repository: UserConfigRepository = repo
        .get_repository::<UserConfigRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener UserConfigRepository"))?;

    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener UserRepository"))?;

    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener ResetTokenRepository"))?;

    // Buscar registro por email
    let user_config = user_config_repository
        .find_one(doc! {"email": email.clone()})
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al buscar UserConfig"))?
        .ok_or(UserConfigError::LoginUserError(
            "No se encontró ningún usuario con ese email",
        ))?;

    // Verificar si está autenticado
    if !user_config.is_authenticated {
        return Err(UserConfigError::AuthenticateError("Usuario no autenticado"));
    }

    // Validar datos desencriptados
    let is_equal_passwords =
        PasswordFunctions::verify_password(&user_config.password, &password)
            .map_err(|_| UserConfigError::LoginUserError("Error al verificar la contraseña"))?;

    if !is_equal_passwords {
        return Err(UserConfigError::LoginUserError("Contraseña incorrecta"));
    }

    // Buscar usuario
    let user = user_repository
        .find_one(doc! {"_id": user_config.id})
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al buscar el usuario"))?
        .ok_or(UserConfigError::LoginUserError("No se encontró el usuario"))?;

    // Crear token
    let new_token = generate_jwt(user.id)
        .map_err(|_| UserConfigError::LoginUserError("Error al generar nuevo token"))?;

    let token = reset_token_repository
        .find_one(doc! {"userId": user.id,"noActive":true})
        .await
        .map_err(|_| UserConfigError::LoginUserError("internal error"))?
        .ok_or_else(|| UserConfigError::LoginUserError("no token found"))?;

    // Verificar token
    let num_devices = token.devices.len();
    let mac_in_devices = token.devices.iter().find(|device| device.mac == mac);
    let reset_token = generate_refresh_jwt(os.as_str(), user.id)
        .map_err(|_| UserConfigError::LoginUserError("Error al generar nuevo token"))?;
    if mac_in_devices.is_none() {
        if num_devices < 2 {
            let update_doc = doc! {
                "$push": {
                    "devices": {
                        "os": os.as_str(),
                        "mac": mac.as_str(),
                        "token": reset_token.as_str()
                    }
                }
            };
            let result = reset_token_repository
                .update_one(doc! {"userId": user.id,"noActive":true}, update_doc)
                .await
                .map_err(|_| UserConfigError::LoginUserError("internal error"))?;
            if result.matched_count < 1 {
                return Err(UserConfigError::LoginUserError("no token found"));
            }
            let short_user_config: ShortUserConfig = user_config.into();
            let login_result = LoginResult::from(&user, short_user_config, new_token, reset_token);

            Ok(JsonAdvanced(login_result))
        } else {
            return Err(UserConfigError::LoginUserError("no more devices allowed"));
        }
    } else {
        let mac_by_device = mac_in_devices.unwrap().mac.as_str();
        //actualizar refresh token
        let filter = doc! {
            "userId": user.id,"noActive":true
        };
        let reset_token = generate_refresh_jwt(os.as_str(), user.id)
            .map_err(|_| UserConfigError::LoginUserError("Error al generar nuevo token"))?;
        // Documento de actualización con array filter
        let update_doc = doc! {
            "$set": {
                "devices.$[d].token": reset_token.clone(), // Actualiza el token del dispositivo que coincide
                "devices.$[d].os": os,
            }
        };
        let array_filters = vec![doc! { "d.mac": { "$eq": mac_by_device } }];
        let result = reset_token_repository
            .update_one(filter, update_doc)
            .array_filters(array_filters)
            .await
            .map_err(|_| UserConfigError::LoginUserError("internal error"))?;
        if result.matched_count < 1 {
            return Err(UserConfigError::LoginUserError("no token found"));
        }
        let short_user_config: ShortUserConfig = user_config.into();
        let login_result =
            LoginResult::from(&user, short_user_config, new_token, reset_token.to_string());

        return Ok(JsonAdvanced(login_result));
    }
}
#[web::post("login/client/token")]
pub async fn login_by_token(
    req: HttpRequest,
    login_dto: JsonAdvanced<LoginByTokenDto>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<LoginResult>, UserConfigError> {
    // Obtener el ID del usuario desde req.extension().get()
    let LoginByTokenDto { mac, os } = login_dto.into_inner();
    let user_id = req
        .extensions()
        .get::<ObjectId>()
        .ok_or(UserConfigError::LoginUserError("User ID not found"))?
        .clone();
    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Internal server error"))?;
    let user_config_repository = repo
        .get_repository::<UserConfigRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Internal server error"))?;
    let user_config = user_config_repository
        .find_one(doc! {"_id":user_id})
        .await
        .map_err(|_| UserConfigError::LoginUserError("User not found"))?
        .ok_or(UserConfigError::LoginUserError("User not found"))?;
    // Buscar el usuario en la base de datos usando el ID
    let user = user_repository
        .find_one(doc! {"_id": user_id})
        .await
        .map_err(|_| UserConfigError::LoginUserError("User not found"))?
        .ok_or(UserConfigError::LoginUserError("User not found"))?;

    // Obtener el repositorio de reset tokens
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Internal server error"))?;

    // Generar un nuevo token JWT
    // Crear token
    let new_token = generate_jwt(user.id)
        .map_err(|_| UserConfigError::LoginUserError("Error al generar nuevo token"))?;

    let token = reset_token_repository
        .find_one(doc! {"userId": user.id})
        .await
        .map_err(|_| UserConfigError::LoginUserError("internal error"))?
        .ok_or_else(|| UserConfigError::LoginUserError("no token found"))?;

    // Verificar token
    let num_devices = token.devices.len();
    let mac_in_devices = token.devices.iter().find(|device| device.mac == mac);

    if mac_in_devices.is_none() {
        let reset_token = generate_refresh_jwt(os.as_str(), user.id)
            .map_err(|_| UserConfigError::LoginUserError("Error al generar nuevo token"))?;
        if num_devices < 2 {
            let update_doc = doc! {
                "$push": {
                    "devices": {
                        "os": os.as_str(),
                        "mac": mac.as_str(),
                        "token": reset_token.as_str()
                    }
                }
            };
            reset_token_repository
                .update_one(doc! {"userId": user.id}, update_doc)
                .await
                .map_err(|_| UserConfigError::LoginUserError("internal error"))?;
            let short_user_config: ShortUserConfig = user_config.into();
            let login_result = LoginResult::from(&user, short_user_config, new_token, reset_token);

            Ok(JsonAdvanced(login_result))
        } else {
            return Err(UserConfigError::LoginUserError("no more devices allowed"));
        }
    } else {
        let mac_by_device = mac_in_devices.unwrap().mac.as_str();
        //actualizar refresh token
        let filter = doc! {
            "userId": user.id,"noActive":true
        };
        let reset_token = generate_refresh_jwt(os.as_str(), user.id)
            .map_err(|_| UserConfigError::LoginUserError("Error al generar nuevo token"))?;
        // Documento de actualización con array filter
        let update_doc = doc! {
            "$set": {
                "devices.$[d].token": reset_token.clone(), // Actualiza el token del dispositivo que coincide
                "devices.$[d].os": os,
            }
        };
        let array_filters = vec![doc! { "d.mac": { "$eq": mac_by_device } }];
        let result = reset_token_repository
            .update_one(filter, update_doc)
            .array_filters(array_filters)
            .await
            .map_err(|_| UserConfigError::LoginUserError("internal error"))?;
        if result.matched_count < 1 {
            return Err(UserConfigError::LoginUserError("no token found"));
        }
        let short_user_config: ShortUserConfig = user_config.into();
        let login_result =
            LoginResult::from(&user, short_user_config, new_token, reset_token.to_string());

        return Ok(JsonAdvanced(login_result));
    }
}

#[web::post("renew/{id}")]
pub async fn renew(
    req: HttpRequest,
    os: JsonAdvanced<RenewTokenDto>,
    id_path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<RenewResult>, ResetTokenError> {
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError("internal server error"))?;
    let RenewTokenDto { os } = os.into_inner();
    let is_active_refresh: bool = req.extensions().get::<bool>().cloned().unwrap_or(true);
    let id = req
        .extensions()
        .get::<ObjectId>()
        .cloned()
        .ok_or_else(|| ResetTokenError::CreateTokenError("Error creating token"))?;
    let os_in_token = req.extensions().get::<String>().cloned();
    let id_path = ObjectId::parse_str(id_path.id())
        .map_err(|_| ResetTokenError::UpdateTokenError("error parsing id"))?;
    let refresh_token: String;
    if id != id_path {
        return Err(ResetTokenError::UpdateTokenError("error parsing id"));
    }

    if !is_active_refresh {
        //genera un nuevo refresh token
        refresh_token = generate_refresh_jwt(os.to_owned().as_str(), id)
            .map_err(|_| ResetTokenError::UpdateTokenError("error generating token"))?;
        //actualizar en base de datos
        let filter = doc! {"userId":id};
        let update = doc! {"$set":{
            "devices.$[device].token":refresh_token.clone()
        }};
        let array_filter = doc! {"device.os":os};
        let _ = reset_token_repository
            .update_one(filter, update)
            .array_filters(vec![array_filter])
            .await
            .map_err(|_| ResetTokenError::UpdateTokenError("sistema operativo no aceptado"))?;
    } else {
        if os != os_in_token.unwrap() {
            return Err(ResetTokenError::UpdateTokenError("error os"));
        }
        refresh_token = reset_token_repository
            .find_one(doc! {"userId":id})
            .await
            .map_err(|_| ResetTokenError::UpdateTokenError("error finding token"))?
            .unwrap()
            .devices
            .iter()
            .find(|device| device.os == os)
            .ok_or_else(|| ResetTokenError::UpdateTokenError("error finding that active device"))?
            .token
            .clone();
    }

    //user repository
    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError("internal server error"))?;
    //busca el user
    let user = user_repository
        .find_one(doc! {"_id":id})
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError("cannot find user"))?
        .ok_or(ResetTokenError::UpdateTokenError("cannot find user"))?;

    let new_token = generate_jwt(user.id)
        .map_err(|_| ResetTokenError::UpdateTokenError("cannot interact with jwt"))?;

    Ok(JsonAdvanced(RenewResult {
        success: true,
        message: new_token.to_string(),
        refresh_token: Some(refresh_token),
    }))
}
