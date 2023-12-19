use tonic::transport::Server;
use crate::db::datasource::mongo::DataSourcePromoCodeMongo;
use crate::db::repositories::PromoCodeRepository;
use crate::presentation::grpc::generated::promotion_service_server::PromotionServiceServer;
use crate::presentation::grpc::handlers::PromotionHandler;
use crate::use_case::CreatePromoCodeUseCase;

pub struct RegisterHandlers;


impl RegisterHandlers{
    pub async fn register() ->Result<(), Box<dyn std::error::Error>>{

        //Repositories
        let  promo_code_repository = PromoCodeRepository::new(DataSourcePromoCodeMongo::new());

        //Use cases
        let create_promo_code_use_case = CreatePromoCodeUseCase::new(promo_code_repository);

        let addr = "127.0.0.1:508".parse().unwrap();
        println!("Starting the server at : {}", addr);
        Server::builder()
            .add_service(PromotionServiceServer::new(PromotionHandler::new(create_promo_code_use_case)))
            .serve(addr)
            .await?;

        Ok(())
    }
}