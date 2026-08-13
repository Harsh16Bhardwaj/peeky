import json
import logging
import sys
from pathlib import Path

VERSION = "0.1.0"

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s — %(message)s",
)
logger = logging.getLogger("peeky")


def main():
    config_path = Path(__file__).parent / "data" / "config.json"

    logger.info("Peeky starting — version %s", VERSION)

    try:
        with open(config_path, "r", encoding="utf-8") as f:
            config = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        logger.error("Failed to load config: %s", e)
        sys.exit(1)

    logger.info("Config loaded successfully")
    logger.info("Peeky boot complete — exiting (stub mode)")
    sys.exit(0)


if __name__ == "__main__":
    main()
