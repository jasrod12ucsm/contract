use async_trait::async_trait;

use crate::utils::domain::{
        datasources::culqi_datasource_trait::CulqiDataSourceTrait,
        models::{
            create_culqi_plan::CreateCulqiPlan,
            culqi_create_plan_response::CulqiCreatePlanResponse, culqi_error::CulqiError,
        },
        repositories::culqi_plan_repository_trait::CulqiPlanRepositoryTrait,
    };

pub struct CulqiPlanRepository<'a> {
    pub culqi_datasource: &'a dyn CulqiDataSourceTrait,
}

#[async_trait]
impl <'a> CulqiPlanRepositoryTrait<'a> for CulqiPlanRepository<'a> {
    fn new(datasource: &'a dyn CulqiDataSourceTrait) -> Self {
        CulqiPlanRepository {
            culqi_datasource: datasource,
        }
    }

    async fn create_plan(
        &self,
        plan: CreateCulqiPlan,
    ) -> Result<CulqiCreatePlanResponse, CulqiError> {
        let response = self.culqi_datasource.create_plan(plan).await?;
        Ok(response)
    }
}
