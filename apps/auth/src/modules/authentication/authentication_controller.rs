use crate::{
    modules::authentication::{
        data::{
            authenticate_post_code::AuthenticatePostCode, login_client_dto::LoginCLientDto,
            register_user_client_dto::RegisterUserClientDto,
        },
        models::{
            email_sended::EmailSended, login_result::LoginResult, renew_result::RenewResult,
            user_id::UserId,
        },
    },
    utils::{
        email_functions::EmailFunctions,
        jwt::generate::{generate_jwt, generate_refresh_jwt},
        repositories::{
            app_variables_repository::AppVariablesRepository,
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
            reset_token::{
                reset_token_attributes::ResetTokenAttributes, reset_token_errors::ResetTokenError,
            },
            user_config::{
                models::{
                    short_user_config::ShortUserConfig, user_config_with_id::UserConfigWithId,
                },
                user_config_attributes::UserConfigAttributes,
                user_config_errors::UserConfigError,
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
    shared::{
        bson::to_bson::ToBson,
        geo_point::GeoPoint,
        jwt::claims::{DefaultClaims, RenewClaims},
    },
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

use jsonwebtoken::{DecodingKey, Validation};
use mongodb::{bson::doc, options::SelectionCriteria};
use ntex::{
    util::Either,
    web::{
        self,
        types::{Json, Path, State},
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
        efective_area,
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
    let user_config_with_id = UserConfigWithId {
        id: user_config_inserted.id,
        names: user_config_to_insert.names,
        surnames: user_config_to_insert.surnames,
        email: user_config_to_insert.email,
        password: user_config_to_insert.password,
        account_type: user_config_to_insert.account_type,
        is_authenticated: user_config_to_insert.is_authenticated,
        is_active: user_config_to_insert.is_active,
        is_deleted: user_config_to_insert.is_deleted,
        created_at: user_config_inserted.created_at,
        updated_at: user_config_inserted.updated_at,
    };

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
    //insertamos usuario
    let user_inserted = user_repository
        .find_one_and_update(
            doc! {"userConfig._id":user_config_with_id.id},
            doc_insert_user,
        )
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
    let token = generate_refresh_jwt()
        .map_err(|_| UserConfigError::CreateUserError("error generating token"));
    if token.is_err() {
        return Err(Either::Left(UserConfigError::CreateUserError(
            "cannot generate token",
        )));
    }
    //2. guardar token en la tabla token
    let copy_token = token.as_ref().unwrap().as_str();
    let reset_token_to_insert = ResetTokenAttributes::new(
        copy_token.to_string(),
        user_inserted.id,
        code,
        user_config_inserted.id,
    );
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
        .country::<ShortCountry>(country.into())
        .region::<ShortRegion>(region.into())
        .name(name.clone())
        .location(GeoPoint::new(longitude, latitude))
        .efective_area(efective_area)
        .num_mesas(0)
        .time_zone(timezone)
        .is_active(true)
        .is_deleted(false)
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
        user_config_id: user_config_inserted.id.to_string(),
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
    let current_user = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| ResetTokenError::GetTokenError("internal server error"))?
        .find_one(doc! {"_id":ObjectId::parse_str(id_path).unwrap()})
        .await
        .map_err(|_| ResetTokenError::GetTokenError("cannot get user"))?
        .ok_or_else(|| ResetTokenError::GetTokenError("invalid user"))?;
    let current_user_config = user_config_repository
        .find_one(doc! {"_id":current_user.user_config})
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
        "userId":current_user.id,"noActive":true
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
        "_id": token.user_config_id
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
    let LoginCLientDto { email, password } = login_dto.into_inner();
    //* Crear repos */
    let user_config_repository: UserConfigRepository = repo
        .get_repository::<UserConfigRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener UserConfigRepository"))?;

    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener UserRepository"))?;

    // Buscar registro por email
    let user_config = user_config_repository
        .find_one(doc! {"email":email.clone()})
        .await
        .map_err(|_err| UserConfigError::LoginUserError("Error al buscar UserConfig"))?;
    // Si no hay nada con ese email, mandar error de logeo
    if user_config.is_none() {
        return Err(UserConfigError::LoginUserError(
            "No se encontró ningún usuario con ese email",
        ));
    }
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener ResetTokenRepository"))?;

    // Hacer unwrap de user config
    let user_config = user_config.unwrap();

    // Verificar si está autenticado
    if !user_config.is_authenticated {
        return Err(UserConfigError::AuthenticateError("Usuario no autenticado"));
    }

    // Validar datos desencriptados.
    let is_equal_passwords =
        PasswordFunctions::verify_password(&(user_config.password), &(password))
            .map_err(|_| UserConfigError::LoginUserError("Error al verificar la contraseña"))?;

    if !is_equal_passwords {
        return Err(UserConfigError::LoginUserError("Contraseña incorrecta"));
    }

    // Crear token si es necesario (búsqueda en tabla)
    let user = user_repository
        .find_one(doc! {"userConfig._id":user_config.id})
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al buscar el usuario"))?
        .ok_or(UserConfigError::LoginUserError("No se encontró el usuario"))?;

    let new_token = generate_jwt(user.id)
        .map_err(|_| UserConfigError::LoginUserError("Error al generar nuevo token"))?;

    let user = user_repository
        .find_one(doc! {"userConfig._id":user_config.id})
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al buscar el usuario"))?
        .ok_or(UserConfigError::LoginUserError("No se encontró el usuario"))?;
    let reset_token = generate_refresh_jwt();
    if reset_token.is_err() {
        return Err(UserConfigError::LoginUserError(
            "Error al generar nuevo token",
        ));
    }
    let reset_token = reset_token.unwrap();
    let filter = doc! {
        "userId":user.id
    };
    let update_doc = doc! {
        "$set":{
            "token":reset_token.as_str()
        }
    };
    let _reset_token_insertion = reset_token_repository
        .update_one(filter, update_doc)
        .await
        .map_err(|_| UserConfigError::LoginUserError("internal error"))?;
    let short_user_config: ShortUserConfig = user_config.into();
    let login_result = LoginResult::from(&user, short_user_config, new_token,reset_token);

    return Ok(JsonAdvanced(login_result));
}
//TODO renovar con refresh token no guardar token normal, cambialo en un dia
#[web::get("renew/{id}")]
pub async fn renew(
    id_path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<RenewResult>, ResetTokenError> {
    let secret = ENV.get_string("SECRET_KEY").unwrap().to_string();
    let id = ObjectId::from_str(id_path.id()).unwrap();
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError("internal server error"))?;
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
    }))
}
