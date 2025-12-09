// test/src/admin/initializer.rs
use mongodb::Database;
use adminx::{
    adminx_initialize, 
    get_adminx_config, 
    setup_adminx_logging, 
    get_adminx_session_middleware,
    register_all_admix_routes,
    registry::register_resource,
    AdmixResource,
    AdminxConfig,
};
use actix_session::SessionMiddleware;

// Import your resources
use crate::admin::resources::user_resource::UserResource;
use crate::admin::resources::notification_resource::NotificationResource;
use crate::admin::resources::config_resource::ConfigResource;
use crate::admin::resources::image_resource::ImageResource;

pub struct AdminxInitializer;

impl AdminxInitializer {
    /// Initialize all AdminX components and return the configuration
    pub async fn initialize(db: Database) -> AdminxConfig {
        println!("Initializing AdminX components...");
        
        // Get AdminX configuration
        let adminx_config = get_adminx_config();
        
        // Setup logging
        setup_adminx_logging(&adminx_config);
        
        // Initialize AdminX with database
        let _adminx_instance = adminx_initialize(db.clone()).await;
        
        // Register resources
        Self::register_resources();
        
        // Print debug information
        Self::print_debug_info();
        
        adminx_config
    }
    
    /// Register all AdminX resources
    fn register_resources() {
        println!("📝 Registering AdminX resources...");
        // Register your resources with AdminX
        register_resource(Box::new(UserResource::new()));
        register_resource(Box::new(NotificationResource::new()));
        register_resource(Box::new(ConfigResource::new()));
        register_resource(Box::new(ImageResource::new()));
        println!("All resources registered successfully!");
    }
    
    /// Print debug information about registered resources
    fn print_debug_info() {
        // Debug: Check if resources were registered
        let resources = adminx::registry::all_resources();
        println!("📋 Total resources registered: {}", resources.len());
        
        for resource in &resources {
            println!("   - Resource: '{}' at path: '{}'", 
                     resource.resource_name(), 
                     resource.base_path());
        }
    }
    
    /// Get the AdminX session middleware
    pub fn get_session_middleware(config: &AdminxConfig) -> SessionMiddleware<impl actix_session::storage::SessionStore> {
        get_adminx_session_middleware(config)
    }
    
    /// Get the AdminX routes service
    pub fn get_routes_service() -> actix_web::Scope {
        register_all_admix_routes()
    }
}