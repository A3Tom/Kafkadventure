from pymongo import MongoClient
from kafka import KafkaConsumer
from settings import MongoDBSettings, KafkaSettings


def build_database_client() -> MongoClient:
    settings = MongoDBSettings()
    connection_string = f"mongodb://{settings.User}:{settings.Password}@{settings.Server}:{settings.Port}/?authSource=admin&readPreference=primary&ssl=false&directConnection=true"

    return MongoClient(connection_string)


def build_kafka_consumer() -> KafkaConsumer:
    settings = KafkaSettings()

    return KafkaConsumer(
        bootstrap_servers=[f"{settings.Server}:{settings.Port}"], 
        auto_offset_reset="earliest"
    )