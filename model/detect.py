from ultralytics import YOLO
from collections import defaultdict

import cv2
import csv

# ------ CONFIGURATION ------
video_name: str = "antispin"
weights_path: str = "model/weights/best.pt"


# Load the YOLOv8 model
model = YOLO(weights_path)

# Open the video file
video_path = f"model/dataset/videos/{video_name}.mp4"
cap = cv2.VideoCapture(video_path)

# Store the track history
track_history = defaultdict(lambda: [])

data_out = [["id", "type", "frame_num", "x", "y", "confidence"]]

# Loop through the video frames
while cap.isOpened():
    # Read a frame from the video
    success, frame = cap.read()

    if success:
        # Run YOLOv8 tracking on the frame, persisting tracks between frames
        results = model.track(frame, persist=True)

        # Get the boxes and track IDs
        boxes = results[0].boxes.data.cpu().tolist()
        for i, row in enumerate(boxes):
            object_id = int(row[-3])
            object_type = int(row[-1])  # 0 -> poi, 1 -> hand
            x, y = row[0], row[1]
            confidence = row[-2]

            data_out.append(
                [
                    object_id,
                    object_type,
                    int(cap.get(cv2.CAP_PROP_POS_FRAMES)),
                    int(x),
                    int(y),
                    confidence,
                ]
            )
    else:
        # Break the loop if the end of the video is reached
        break

# Write the tracking data to a CSV file
with open(
    f"processing/tracking_data/{video_name}_tracking.csv", "w", newline=""
) as file:
    writer = csv.writer(file)
    writer.writerows(data_out)

# Release the video capture object and close the display window
cap.release()
