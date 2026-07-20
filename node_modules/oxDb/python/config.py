"""
Configuration for news aggregator and Rust backend integration.
"""

import os
from typing import Optional

# OpenAI Configuration
OPENAI_API_KEY: str = os.getenv("OPENAI_API_KEY", "")
OPENAI_MODEL: str = os.getenv("OPENAI_MODEL", "gpt-4o-mini")
OPENAI_TEMPERATURE: float = float(os.getenv("OPENAI_TEMPERATURE", "0.3"))

# Rust Backend Configuration
RUST_API_BASE: str = os.getenv("RUST_API_BASE", "http://localhost:8080")
RUST_API_TIMEOUT: int = int(os.getenv("RUST_API_TIMEOUT", "5"))
RUST_API_ENABLED: bool = os.getenv("RUST_API_ENABLED", "true").lower() == "true"

# News Aggregation Configuration
MAX_ARTICLES_PER_TOPIC: int = int(os.getenv("MAX_ARTICLES_PER_TOPIC", "5"))
SUMMARY_MAX_LENGTH: int = int(os.getenv("SUMMARY_MAX_LENGTH", "500"))
TOPICS_FILE: Optional[str] = os.getenv("TOPICS_FILE", None)

# Logging Configuration
LOG_LEVEL: str = os.getenv("LOG_LEVEL", "INFO")
LOG_FILE: Optional[str] = os.getenv("LOG_FILE", None)

# Metrics Configuration
ENABLE_METRICS: bool = os.getenv("ENABLE_METRICS", "true").lower() == "true"
METRICS_BATCH_SIZE: int = int(os.getenv("METRICS_BATCH_SIZE", "10"))


def validate_config() -> tuple[bool, str]:
    """
    Validate configuration and return status.
    
    Returns:
        Tuple of (is_valid, message)
    """
    if not OPENAI_API_KEY:
        return False, "ERROR: OPENAI_API_KEY not set"
    
    if RUST_API_ENABLED and not RUST_API_BASE:
        return False, "ERROR: RUST_API_BASE not set when RUST_API_ENABLED=true"
    
    return True, "Configuration valid"


def get_config_dict() -> dict:
    """Return current configuration as dictionary."""
    return {
        "openai_model": OPENAI_MODEL,
        "openai_temperature": OPENAI_TEMPERATURE,
        "rust_api_base": RUST_API_BASE,
        "rust_api_enabled": RUST_API_ENABLED,
        "max_articles": MAX_ARTICLES_PER_TOPIC,
        "log_level": LOG_LEVEL,
        "metrics_enabled": ENABLE_METRICS,
    }
