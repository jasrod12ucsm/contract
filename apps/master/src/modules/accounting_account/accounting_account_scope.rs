use ntex::web::{scope, ServiceConfig};

use super::accounting_account_controller::{
    create_account, delete_account, get_account_by_id, get_account_by_name, get_account_by_user_id, get_all_accounts, update_account
};

pub fn accounting_account_scope(configure: &mut ServiceConfig) {
    configure.service(
        scope("/")
            .service(create_account)
            .service(get_account_by_id)
            .service(get_account_by_user_id)
            .service(get_all_accounts)
            .service(update_account)
            .service(delete_account)
            .service(get_account_by_name)
    );
}
