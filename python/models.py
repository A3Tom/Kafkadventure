class SightingEvent:
    def __init__(self, animal, loch, count, reported_by, latitude, longitude, event_datetime):
        self.animal = animal
        self.loch = loch
        self.count = count
        self.reported_by = reported_by
        self.latitude = latitude
        self.longitude = longitude
        self.event_datetime = event_datetime
