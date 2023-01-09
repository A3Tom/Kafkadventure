import json
from typing import Collection, Dict

import client_builder
from kafka import KafkaConsumer
from models import SightingEvent
from pymongo import MongoClient
from settings import KafkaSettings, MongoDBSettings

consumer: KafkaConsumer = client_builder.build_kafka_consumer()
mongo_client: MongoClient = client_builder.build_database_client()

kafka_settings: KafkaSettings = KafkaSettings()
mongo_settings: MongoDBSettings = MongoDBSettings()

def subscribeToKafkaTopic() -> None:
    consumer.subscribe([kafka_settings.Topic])

def getDatabaseObject() -> Collection:
    return mongo_client[mongo_settings.DatabaseName][kafka_settings.Topic]

def insertSightingToDatabase(sighting: Dict[str, str]) -> None:
    table: Collection = getDatabaseObject()
    table.insert_one(sighting)

def translateEventToEntity(sighting: Dict) -> SightingEvent:
    meat_n_tatties = sighting["MeatNTatties"]
    return SightingEvent(
        meat_n_tatties["Animal"],
        meat_n_tatties["Loch"],
        meat_n_tatties["Count"],
        meat_n_tatties["ReportedBy"],
        meat_n_tatties["Latitude"],
        meat_n_tatties["Longitude"],
        sighting["CreatedDate"]
    )

def run() -> None:
    subscribeToKafkaTopic()

    for event in consumer:
        decodedEvent: Dict[str, str] = json.loads(bytes.decode(event.value))
        entity: SightingEvent = translateEventToEntity(decodedEvent)
        insertSightingToDatabase(entity.__dict__)

if __name__ == '__main__':
    run()
