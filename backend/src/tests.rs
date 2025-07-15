use anyhow::Result;
use crate::fastembed_service::FastEmbedService;

#[tokio::test]
async fn test_fastembed_service() -> Result<()> {
    // Test FastEmbed service initialization
    let service = FastEmbedService::new()?;
    
    // Test embedding generation
    let text = "This is a test review about a great product";
    let embedding = service.embed_text(text)?;
    
    // Check embedding dimensions (BGE-small-en-v1.5 should produce 384-dimensional embeddings)
    assert_eq!(embedding.len(), 384);
    
    // Check that embedding values are not all zeros
    assert!(embedding.iter().any(|&x| x != 0.0));
    
    println!("FastEmbed test passed! Embedding dimension: {}", embedding.len());
    println!("First 5 values: {:?}", &embedding[0..5]);
    
    Ok(())
}

#[tokio::test]
async fn test_fastembed_batch() -> Result<()> {
    let service = FastEmbedService::new()?;
    
    let texts = vec![
        "This is a positive review",
        "This is a negative review",
        "This is a neutral review"
    ];
    
    let embeddings = service.embed_batch(texts)?;
    
    // Check that we got 3 embeddings
    assert_eq!(embeddings.len(), 3);
    
    // Check dimensions
    for embedding in &embeddings {
        assert_eq!(embedding.len(), 384);
    }
    
    println!("Batch embedding test passed! Generated {} embeddings", embeddings.len());
    
    Ok(())
}
