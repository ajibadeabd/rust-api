pub mod paystack;


// mod payment_interface {
//     use super::paystack;

//     pub struct IPaymentProviderConfig {
//         // Define the IPaymentProviderConfig struct here
//     }

//     pub struct IPaymentProviderInstance {
//         module:paystack::*,
//         // Define the IPaymentProviderInstance struct here
//     }
// }

struct AmountRange {
    min: f64,
    max: f64,
}
struct SupportedCurrency {
    code:String,
    amount:AmountRange
}

struct IPaymentProviderConfig<T> {
    supported_currencies :Vec<SupportedCurrency>,
    module:T

}

// mod paystack {
//     // Define the paystack module here
//     // You can define the Paystack struct and implement the IPaymentProviderInstance trait here
// }

use std::collections::HashMap;

pub use self::paystack::PaystackApi;
// use paystack as paystack_module::;
// use paystack::Paystack; // Import the Paystack module
// use payment_interface::{IPaymentProviderConfig, IPaymentProviderInstance};

const DEFAULT_PROVIDER: &str = "paystack";

// Define IPaymentProviderConfig and IPaymentProviderInstance implementations for paystack here

// Define IPaymentProviderConfig and IPaymentProviderInstance implementations for other providers here

pub struct PaymentProviderHashMap {
    providers: HashMap<String, IPaymentProviderConfig<PaystackApi>>,
}



impl PaymentProviderHashMap{
pub   fn new() -> Self {
        let mut providers = HashMap::new();
        
        
        // Define IPaymentProviderConfig instances for each payment provider
        let paystack_config = IPaymentProviderConfig {
            module: PaystackApi, // Instantiate the Paystack struct
            supported_currencies: vec![
                SupportedCurrency {
                    code: "NGN".to_string(),

                    
                    amount: AmountRange { min: 100.00, max: 3_000_000.00 },
                },
            ],
        };

        // Add each provider config to the hashmap
        providers.insert("paystack".to_string(), paystack_config);

        Self { providers }
    }

pub  fn get_provider_name(&self, amount: f64, currency: &str) -> Option<String> {
        for (provider_name, config) in &self.providers {
            for supported_currency in &config.supported_currencies {
                if supported_currency.code == currency
                    && amount >= supported_currency.amount.min
                    && amount <= supported_currency.amount.max
                {
                    return Some(provider_name.clone());
                }
            }
        }
        None
    }
pub fn get_provider_instance(&self, provider_name: Option<&str>)->Option<PaystackApi>
 {
        match provider_name.unwrap_or(DEFAULT_PROVIDER) {
            "paystack" => Some(self.providers.get("paystack")?.module.clone()),
            _ => None,
        }
    }

//    
}

// Create an instance of PaymentProviderHashMap
// lazy_static::lazy_static! {
//     static ref PAYMENT_PROVIDER_HASHMAP: PaymentProviderHashMap = PaymentProviderHashMap::new();
// }

// Export the get_provider_name and get_provider_instance functions
// pub fn get_provider_name(amount: i64, currency: &str) -> Option<String> {
//     PAYMENT_PROVIDER_HASHMAP.get_provider_name(amount, currency)
// }

// pub fn get_provider_instance(provider_name: Option<&str>) -> Option<IPaymentProviderInstance> {
//     PAYMENT_PROVIDER_HASHMAP.get_provider_instance(provider_name)
// }
