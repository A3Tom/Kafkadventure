import json
from typing import Dict

import database
import kafka_consumer
from models import SightingEvent


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
    consumer = kafka_consumer.subscribe_to_sightings_topic()

    for event in consumer:
        decodedEvent: Dict[str, str] = json.loads(bytes.decode(event.value))
        entity: SightingEvent = translateEventToEntity(decodedEvent)
        database.insert_sighting(entity.__dict__)


if __name__ == '__main__':
    run()
