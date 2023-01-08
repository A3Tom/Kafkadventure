import os
from dotenv import load_dotenv

load_dotenv()

class MongoDBSettings:
    def __init__(self):
        self.Server = os.environ['MONGO_SERVER']
        self.Port = os.environ['MONGO_PORT']
        self.DatabaseName = os.environ['MONGO_DATABASE']
        self.User = os.environ['MONGO_USER']
        self.Password = os.environ['MONGO_PASS']

class KafkaSettings:
    def __init__(self) -> None:
        self.Server = os.environ['KAFKA_SERVER']
        self.Port = os.environ['KAFKA_PORT']
        self.Topic = os.environ['KAFKA_TOPIC']