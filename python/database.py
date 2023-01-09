from typing import List
import client_builder
from pymongo import MongoClient
from settings import MongoDBSettings

from models import SightingEvent

mongo_client: MongoClient = client_builder.build_database_client()
mongo_settings: MongoDBSettings = MongoDBSettings()

def getDatabaseObject():
    return mongo_client[mongo_settings.DatabaseName]
    
def insert_sighting(sighting: SightingEvent) -> None:
    table = getDatabaseObject()['sightings']
    table.insert_one(sighting)

def get_sighting_by_reporter(reporter_name: str) -> List[SightingEvent]:
    table = getDatabaseObject()['sightings']
    query = { "reported_by" : reporter_name }
    return list(table.find(query))