from kafka import KafkaConsumer
from settings import KafkaSettings
import client_builder

kafka_settings: KafkaSettings = KafkaSettings()

consumer: KafkaConsumer = client_builder.build_kafka_consumer()


def subscribe_to_sightings_topic() -> KafkaConsumer:
    consumer.subscribe([kafka_settings.Topic])
    return consumer
