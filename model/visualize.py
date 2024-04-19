from ultralytics import YOLO

# ------ CONFIGURATION ------
video_name: str = "wu"
weights_path: str = "model/weights/best.pt"


model = YOLO(weights_path)
results = model.track(
    source=f"model/dataset/videos/{video_name}.mp4", show=True, tracker="bytetrack.yaml"
)
