use std::str::FromStr;

use crate::{
    modules::authentication::{
        data::{
            authenticate_post_code::AuthenticatePostCode, login_client_dto::LoginCLientDto,
            register_user_client_dto::RegisterUserClientDto,
        },
        models::{
            email_sended::EmailSended, get_token_result::GetTokenResult, renew_result::RenewResult,
            user_id::UserId,
        },
    },
    utils::{
        country_repository::CountryRepository, jwt::generate::generate_jwt,
        region_repository::RegionRepository, reset_token_repository::ResetTokenRepository,
        user_config_repository::UserConfigRepository, user_repository::UserRepository,
    },
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
                    user_config_without_password::UserConfigWithoutPassword,
                },
                user_config_attributes::UserConfigAttributes,
                user_config_errors::UserConfigError,
            },
        },
        mst::user::{
            models::{identification::Identification, user_with_id::UserWithId},
            user_attributes::UserAttributes,
            user_errors::UserError,
        },
    },
    shared::jwt::claims::DefaultClaims,
};
use bson::oid::ObjectId;
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
use mongodb::bson::doc;
use ntex::{
    util::Either,
    web::{
        self,
        types::{Json, Path, State},
    },
};

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
    println!("paso session");
    //si encuentra el email verifica si esta authenticado, si lo esta regresa error, si no continuea con el codigo
    let country = country_repository
        .find_one(doc! {"code":country_code.clone()}, Some(&mut session))
        .await
        .map_err(|err| {
            println!("{}", err);
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("country not correct"))
        })?
        .ok_or_else(|| Either::Right(UserError::CreateUserError("")))?;
    println!("paso country");
    let find_region_document = doc! {"code":region_code.clone(), "countryId":country_code};
    println!("{:?}", find_region_document);
    let region = region_repository
        .find_one(find_region_document, Some(&mut session))
        .await
        .map_err(|err| {
            println!("{}", err);
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("internal error"))
        })?
        .ok_or_else(|| {
            println!("result is none");
            Either::Right(UserError::CreateUserError("incorrect region"))
        })?;
    let user_config = user_config_repository
        .find_one(doc! {"email":email.clone()}, None)
        .await
        .map_err(|err| {
            println!("{}", err);
            Either::Left(UserConfigError::CreateUserError("error finding email"))
        })?;
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
    println!("{}", update_doc);
    let user_config_inserted = user_config_repository
        .find_one_and_update_with_upsert(
            doc! {"email":email.clone()},
            update_doc,
            Some(&mut session),
        )
        .await
        .map_err(|err| {
            let _ = session.abort_transaction();
            println!("{}", err);
            Either::Left(UserConfigError::CreateUserError(
                "error on create user, aborting process",
            ))
        })?
        .unwrap();
    //*sin asignar empresa,ni permisos, permisos basicos (tabla)
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
        is_delete: user_config_to_insert.is_delete,
        created_at: user_config_inserted.created_at,
        updated_at: user_config_inserted.updated_at,
    };

    let user_config_with_id: ShortUserConfig = user_config_with_id.into();

    //4. Crear usuario principal
    let user = UserAttributes::new_client(
        user_config_with_id.clone(),
        Identification {
            identification_number,
            identification_type,
        },
        phone,
        address,
        country.into(),
        region.into(),
    );
    let doc_insert_user = doc! {
        "$set":bson::to_bson(&user).unwrap()
    };
    //insertamos usuario
    let user_inserted = user_repository
        .find_one_and_update_with_upsert(
            doc! {"userConfig._id":user_config_with_id.id},
            doc_insert_user,
            Some(&mut session),
        )
        .await
        .map_err(|_err| {
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("error inserting user"))
        })?
        .ok_or_else(|| {
            let _ = session.abort_transaction();
            Either::Right(UserError::CreateUserError("error inserting user"))
        })?;
    let token = generate_jwt(user_inserted.clone())
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
    println!("{:?}", reset_token_to_insert);
    let doc_insert_token = doc! {
        "$set":bson::to_bson(&reset_token_to_insert).unwrap()
    };
    println!("{:?}", doc_insert_token);
    let _reset_token_insertion = reset_token_repository
        .find_one_and_update_with_upsert(
            doc! {"userId":user_inserted.id},
            doc_insert_token,
            Some(&mut session),
        )
        .await
        .map_err(|err| {
            println!("{}", err);
            let _ = session.abort_transaction();
            Either::Left(UserConfigError::CreateUserError(
                "error updating token table",
            ))
        })?;
    session.commit_transaction().await.map_err(|err| {
        println!("{}", err);
        Either::Right(UserError::CreateUserError("error commiting transaction"))
    })?;
    let data_to_return = UserId {
        user: user_inserted.id.to_string(),
        user_config_id: user_config_inserted.id.to_string(),
    };
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <title>Código de Verificación</title>
        </head>
        <body>
            <p>Hola,</p>
            <p>Tu código de verificación es: <strong>{}</strong></p>
            <p>Gracias,</p>
            <p>El equipo de Soporte</p>
        </body>
        </html>
        "#,
        code
    );
    // Enviar el email de forma asincrónica sin bloquear la función principal
    tokio::spawn(async move {
        let _ = SmtpFunctions::send_email(email.as_str(), "Enable Account", &html_content);
    });
    Ok(JsonAdvanced(data_to_return))
}
//TODO verificar tabla de token al logear

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
        .map_err(|_| ResetTokenError::GetTokenError)?;
    let current_user = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| ResetTokenError::GetTokenError)?
        .find_one(doc! {"_id":ObjectId::parse_str(id_path).unwrap()}, None)
        .await
        .map_err(|_| ResetTokenError::GetTokenError)?
        .ok_or_else(|| ResetTokenError::GetTokenError)?;
    //generar random number
    let code = PasswordFunctions::generate_random_number();
    //actualizar la tabla de reset_token con ese random_number

    let filter = doc! {
        "userId":current_user.id
    };
    let update_doc = doc! {
        "$set":{
            "authCode":code
        }
    };
    let _reset_token_insertion = reset_token_repository
        .update_one(filter, update_doc)
        .await
        .map_err(|_| ResetTokenError::GetTokenError)?;
    //ahora genera el html
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <title>Código de Verificación</title>
        </head>
        <body>
            <p>Hola,</p>
            <p>Tu código de verificación es: <strong>{}</strong></p>
            <p>Gracias,</p>
            <p>El equipo de Soporte</p>
        </body>
        </html>
        "#,
        code
    );
    // Enviar el email de forma asincrónica sin bloquear la función principal
    let email_sended = SmtpFunctions::send_email(
        current_user.user_config.email.as_str(),
        "Enable Account",
        &html_content,
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
) -> Result<Json<UserConfigWithoutPassword>, UserConfigError> {
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
        .find_one(doc! {"userId":id}, None)
        .await
        .map_err(|_| UserConfigError::LoginUserError("User not finded"))?;
    if token_register.is_none() {
        return Err(UserConfigError::LoginUserError("No existe token generado"));
    }
    let token = token_register.unwrap();
    //pregunta por el codigo
    let AuthenticatePostCode { code } = code.into_inner();
    println!("{}", token.auth_code);
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
        .find_one_and_update_with_upsert(filter, update_doc, None)
        .await
        .map_err(|error| {
            println!("{}", error);
            UserConfigError::LoginUserError("Internal update error")
        })?;
    if actualized_register.is_none() {
        return Err(UserConfigError::LoginUserError(
            "no se actualizo ningun registro",
        ));
    }
    let actualized_register = actualized_register.unwrap();
    Ok(Json(actualized_register.into()))
}

#[web::post("login/client")]
pub async fn login_client(
    login_dto: JsonAdvanced<LoginCLientDto>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<UserWithId>, UserConfigError> {
    // No hacemos validación del tipo de cuenta, ya que todos los tipos pueden entrar como clientes
    let LoginCLientDto { email, password } = login_dto.into_inner();
    //* Crear repos */
    let user_config_repository: UserConfigRepository = repo
        .get_repository::<UserConfigRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener UserConfigRepository"))?;

    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener ResetTokenRepository"))?;

    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al obtener UserRepository"))?;

    // Buscar registro por email
    let user_config = user_config_repository
        .find_one(doc! {"email":email.clone()}, None)
        .await
        .map_err(|_err| UserConfigError::LoginUserError("Error al buscar UserConfig"))?;

    // Si no hay nada con ese email, mandar error de logeo
    if user_config.is_none() {
        return Err(UserConfigError::LoginUserError(
            "No se encontró ningún usuario con ese email",
        ));
    }

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
    let token_register = reset_token_repository
        .find_one(doc! {"userConfigId":user_config.id}, None)
        .await
        .map_err(|_| UserConfigError::LoginUserError("Error al buscar ResetToken"))?;

    let secret = ENV.get_string("SECRET_KEY").unwrap().to_string();
    if let Some(token_register) = token_register {
        let token = token_register.token;
        match jsonwebtoken::decode::<DefaultClaims>(
            &token,
            &DecodingKey::from_secret(secret.to_string().as_ref()),
            &Validation::default(),
        ) {
            Ok(_token_desencrypted) => {
                let user = user_repository
                    .find_one(doc! {"userConfig._id":user_config.id}, None)
                    .await
                    .map_err(|_| UserConfigError::LoginUserError("Error al buscar el usuario"))?
                    .ok_or(UserConfigError::LoginUserError("No se encontró el usuario"))?;

                println!("{:?}", _token_desencrypted);
                return Ok(JsonAdvanced(user));
            }
            Err(err) => {
                match err.kind() {
                    jsonwebtoken::errors::ErrorKind::InvalidToken
                    | jsonwebtoken::errors::ErrorKind::InvalidSignature
                    | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let user = user_repository
                            .find_one(doc! {"userConfig._id":user_config.id}, None)
                            .await
                            .map_err(|_| {
                                UserConfigError::LoginUserError("Error al buscar el usuario")
                            })?
                            .ok_or(UserConfigError::LoginUserError("No se encontró el usuario"))?;

                        let new_token = generate_jwt(user).map_err(|_| {
                            UserConfigError::LoginUserError("Error al generar nuevo token")
                        })?;

                        let filter = doc! {"userId":user_config.id};
                        let doc_insert_token = doc! {
                            "$set":{ "token": &new_token }
                        };

                        reset_token_repository
                            .update_one(filter, doc_insert_token)
                            .await
                            .map_err(|_| {
                                UserConfigError::LoginUserError("Error al actualizar el token")
                            })?;

                        let user = user_repository
                            .find_one(doc! {"userConfig._id":user_config.id}, None)
                            .await
                            .map_err(|_| {
                                UserConfigError::LoginUserError("Error al buscar el usuario")
                            })?
                            .ok_or(UserConfigError::LoginUserError("No se encontró el usuario"))?;

                        return Ok(JsonAdvanced(user));
                    }
                    _ => {}
                }
                return Err(UserConfigError::LoginUserError(
                    "Error desconocido al decodificar el token",
                ));
            }
        };
    } else {
        return Err(UserConfigError::LoginUserError(
            "No se encontró un registro de token",
        ));
    }
}

//funcion para renovar token
#[web::get("renew/{id}")]
pub async fn renew(
    id_path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<RenewResult>, ResetTokenError> {
    let id = ObjectId::from_str(id_path.id()).unwrap();
    println!("ejecutandose");
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError)?;
    //user repository
    let user_repository: UserRepository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError)?;
    //busca el user
    let user = user_repository
        .find_one(doc! {"_id":id}, None)
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError)?
        .ok_or(ResetTokenError::UpdateTokenError)?;

    let new_token = generate_jwt(user).map_err(|_| ResetTokenError::UpdateTokenError)?;
    //ahora actualizamos la tabla con el nuevo token
    let filter = doc! {"userId":id};
    let doc_insert_token = doc! {
        "$set":{
            "token":&new_token
        }
    };
    let _reset_token_insertion = reset_token_repository
        .update_one(filter, doc_insert_token)
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError)?;
    Ok(JsonAdvanced(RenewResult {
        success: true,
        message: "token generated".to_string(),
    }))
}
//* habra un metodo para obtener el token actual de la tabla, ese metodo es gettoken */
//*desde el front tienen que llamarlo con cuidado, solo si tienen un usuario cargado */
#[web::get("gettoken/{id}")]
pub async fn get_token(
    id_path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<GetTokenResult>, ResetTokenError> {
    let id = ObjectId::from_str(id_path.id()).unwrap();
    println!("ejecutandose");
    let reset_token_repository: ResetTokenRepository = repo
        .get_repository::<ResetTokenRepository>()
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError)?;
    let token_register = reset_token_repository
        .find_one(doc! {"userId":id}, None)
        .await
        .map_err(|_| ResetTokenError::UpdateTokenError)?;
    if token_register.is_none() {
        return Err(ResetTokenError::UpdateTokenError);
    }
    let token = token_register.unwrap().token;
    Ok(JsonAdvanced(GetTokenResult { token }))
}
