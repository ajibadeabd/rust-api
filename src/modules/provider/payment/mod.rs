pub mod paystack;


mod payment_interface {
    pub struct IPaymentProviderConfig {
        // Define the IPaymentProviderConfig struct here
    }

    pub struct IPaymentProviderInstance {
        // Define the IPaymentProviderInstance struct here
    }
}

mod paystack {
    // Define the paystack module here
    // You can define the Paystack struct and implement the IPaymentProviderInstance trait here
}

use std::collections::HashMap;

use paystack::Paystack; // Import the Paystack module
use payment_interface::{IPaymentProviderConfig, IPaymentProviderInstance};

const DEFAULT_PROVIDER: &str = "paystack";

// Define IPaymentProviderConfig and IPaymentProviderInstance implementations for paystack here

// Define IPaymentProviderConfig and IPaymentProviderInstance implementations for other providers here

pub struct PaymentProviderHashMap {
    providers: HashMap<String, IPaymentProviderConfig>,
}

impl PaymentProviderHashMap {
    fn new() -> Self {
        let mut providers = HashMap::new();
        
        // Define IPaymentProviderConfig instances for each payment provider
        let paystack_config = IPaymentProviderConfig {
            module: paystack, // Instantiate the Paystack struct
            supported_currencies: vec![
                SupportedCurrency {
                    code: "NGN".to_string(),
                    amount: AmountRange { min: 100, max: 3_000_000 },
                },
            ],
        };

        // Add each provider config to the hashmap
        providers.insert("paystack".to_string(), paystack_config);

        Self { providers }
    }

    fn get_provider_name(&self, amount: i64, currency: &str) -> Option<String> {
        for (provider_name, config) in &self.providers {
            for supported_currency in &config.supported_currencies {
                if supported_currency.code == currency
                    && amount > supported_currency.amount.min
                    && amount <= supported_currency.amount.max
                {
                    return Some(provider_name.clone());
                }
            }
        }
        None
    }

    fn get_provider_instance(&self, provider_name: Option<&str>) -> Option<IPaymentProviderInstance> {
        match provider_name.unwrap_or(DEFAULT_PROVIDER) {
            "paystack" => Some(self.providers.get("paystack")?.module.clone()),
            // Add cases for other providers here
            _ => None,
        }
    }
}

// Create an instance of PaymentProviderHashMap
lazy_static::lazy_static! {
    static ref PAYMENT_PROVIDER_HASHMAP: PaymentProviderHashMap = PaymentProviderHashMap::new();
}

// Export the get_provider_name and get_provider_instance functions
pub fn get_provider_name(amount: i64, currency: &str) -> Option<String> {
    PAYMENT_PROVIDER_HASHMAP.get_provider_name(amount, currency)
}

pub fn get_provider_instance(provider_name: Option<&str>) -> Option<IPaymentProviderInstance> {
    PAYMENT_PROVIDER_HASHMAP.get_provider_instance(provider_name)
}
