use anyhow::Result;
use tracing::{debug, error, info};

use neo_config::NetworkType;
use neo_ledger::Blockchain;

pub async fn debug_blockchain_init() -> Result<()> {
    info!("🔍 Starting blockchain initialization debug/* implementation */;");

    // Step 1: Test data directory creation
    debug!("📁 Creating data directory/* implementation */;");
    if let Err(e) = std::fs::create_dir_all("./data") {
        error!("❌ Failed to create data directory: {}", e);
        return Err(anyhow::anyhow!("Data directory creation failed: {}", e));
    }
    info!("✅ Data directory created");

    // Step 2: Skip direct RocksDB test since it's internal to storage

    // Step 3: Test storage creation
    debug!("💾 Testing storage creation/* implementation */;");
    let storage = match neo_ledger::Storage::new_rocksdb("./data/debug-storage") {
        Ok(storage) => {
            info!("✅ Storage creation successful");
            storage
        }
        Err(e) => {
            error!("❌ Storage creation failed: {}", e);
            return Err(anyhow::anyhow!("Storage creation failed: {}", e));
        }
    };

    // Step 4: Test basic storage operations
    debug!("🔧 Testing storage operations/* implementation */;");
    let key = neo_ledger::StorageKey::new(b"debug".to_vec(), b"test".to_vec());
    let item = neo_ledger::StorageItem::new(b"value".to_vec());

    if let Err(e) = storage.put(&key, &item).await {
        error!("❌ Storage put operation failed: {}", e);
        return Err(anyhow::anyhow!("Storage put failed: {}", e));
    }

    match storage.get(&key).await {
        Ok(retrieved) => {
            if retrieved.value == item.value {
                info!("✅ Storage operations successful");
            } else {
                error!("❌ Storage get returned wrong value");
                return Err(anyhow::anyhow!("Storage get returned wrong value"));
            }
        }
        Err(e) => {
            error!("❌ Storage get operation failed: {}", e);
            return Err(anyhow::anyhow!("Storage get failed: {}", e));
        }
    }

    // Step 5: Try to create blockchain with detailed error logging
    info!("⛓️ Testing blockchain creation/* implementation */;");
    debug!("Using NetworkType::TestNet");

    match Blockchain::new(NetworkType::TestNet).await {
        Ok(blockchain) => {
            info!("✅ Blockchain creation successful!");
            let height = blockchain.get_height().await;
            info!("📊 Current blockchain height: {}", height);

            // Test basic blockchain operations
            if let Ok(Some(genesis)) = blockchain.get_block(0).await {
                info!("✅ Genesis block retrieved successfully");
                info!("📊 Genesis block hash: {}", genesis.hash());
            } else {
                error!("❌ Could not retrieve genesis block");
                return Err(anyhow::anyhow!("Genesis block not found"));
            }
        }
        Err(e) => {
            error!("❌ Blockchain creation failed: {}", e);
            error!("❌ Error details: {:?}", e);
            error!("❌ Error source chain: (not available)");
            return Err(anyhow::anyhow!("Blockchain creation failed: {}", e));
        }
    }

    info!("🏁 All blockchain initialization tests passed!");
    Ok(())
}
